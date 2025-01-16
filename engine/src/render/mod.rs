use std::mem::offset_of;

use anyhow::Result;
use ash::{prelude::VkResult, vk};
use bytemuck::{bytes_of, NoUninit};
use context::Context;
use shaders::{
    GAME_VERTEX_SHADER, SIMPLE_GAME_FRAGMENT_SHADER, SIMPLE_UI_FRAGMENT_SHADER, UI_VERTEX_SHADER,
};
use ultraviolet::Vec4;
use winit::window::Window;

mod buffer;
mod context;
mod debug;
mod pipeline;
mod shaders;
mod utils;

use buffer::Buffer;
use pipeline::{create_pipelines, Pipeline, PipelineCreateInfo};
use utils::{
    choose_swap_extent, choose_swap_present_mode, choose_swap_surface_format, QueueFamilyIndices,
    SwapchainSupportDetails,
};

pub enum RenderLiteral {
    UI {
        anchor: [f32; 2],
        shape: ShapeLiteral,
    },
    Game(ShapeLiteral),
}

pub enum ShapeLiteral {
    Polygon {
        pos: [f32; 2],
        colour: Vec4,
        angles: Vec<f32>,
        distances: Vec<f32>,
        border_thickness: f32,
    },
    // TODO: letter / text or smth
}

// TODO: better name
pub struct EverythingToDraw {
    pub camera_pos: [f32; 2],
    pub scale: f32,
    pub inverted: bool,
    pub shapes: Vec<RenderLiteral>,
}

impl EverythingToDraw {
    fn game_pc(&self, window: &Window) -> GamePushConstants {
        let size = window.inner_size();
        GamePushConstants {
            cam_pos: self.camera_pos,
            scale: self.scale,
            width: size.width,
            height: size.height,
        }
    }

    fn ui_pc(&self, window: &Window) -> UiPushConstants {
        let size = window.inner_size();
        UiPushConstants {
            height: size.height,
            width: size.width,
        }
    }

    fn frag_pc(&self) -> FragPushConstants {
        FragPushConstants {
            inverted: self.inverted,
            padding: [0; 3],
        }
    }

    fn game_vertices(&self) -> Vec<GameVertex> {
        self.shapes
            .iter()
            .filter(|s| matches!(s, RenderLiteral::Game(_)))
            .flat_map(|s| {
                let RenderLiteral::Game(shape) = s else {
                    unreachable!()
                };
                match shape {
                    ShapeLiteral::Polygon {
                        pos,
                        angles,
                        distances,
                        colour,
                        ..
                    } => angles
                        .iter()
                        .zip(distances.iter())
                        .map(|(&a, &d)| GameVertex {
                            position: [pos[0] + d * a.cos(), pos[1] + d * a.sin()],
                            colour: *colour,
                        }),
                }
            })
            .collect()
    }

    fn ui_vertices(&self) -> Vec<UiVertex> {
        self.shapes
            .iter()
            .filter(|s| matches!(s, RenderLiteral::UI { .. }))
            .flat_map(|s| {
                let RenderLiteral::UI { anchor, shape } = s else {
                    unreachable!()
                };
                match shape {
                    ShapeLiteral::Polygon {
                        pos,
                        angles,
                        distances,
                        colour,
                        ..
                    } => angles
                        .iter()
                        .zip(distances.iter())
                        .map(|(&a, &d)| UiVertex {
                            position: [pos[0] + d * a.sin(), pos[1] + d * a.cos()],
                            anchor: *anchor,
                            colour: *colour,
                        }),
                }
            })
            .collect()
    }

    fn indices(&self) -> (Vec<u16>, usize) {
        let ui_start = self
            .shapes
            .iter()
            .filter(|s| matches!(s, RenderLiteral::Game(_)))
            .map(|s| {
                let RenderLiteral::Game(ShapeLiteral::Polygon { angles, .. }) = s else {
                    unreachable!()
                };
                angles.len()
            })
            .sum::<usize>()
            * 2;

        let helper = |s: &RenderLiteral| match s {
            RenderLiteral::Game(ShapeLiteral::Polygon { angles, .. }) => angles.len(),
            RenderLiteral::UI {
                shape: ShapeLiteral::Polygon { angles, .. },
                ..
            } => angles.len(),
        } as u16;

        let mut a = 0;
        let game_indices = self
            .shapes
            .iter()
            .filter(|s| matches!(s, RenderLiteral::Game(_)))
            .flat_map(|s| {
                let count = helper(s);
                let n = a;
                a += count;
                (0..count).flat_map(move |i| [i + n, (i + 1) % count + n])
            });
        let mut b = 0;
        (
            game_indices
                .chain(
                    self.shapes
                        .iter()
                        .filter(|s| matches!(s, RenderLiteral::UI { .. }))
                        .flat_map(|s| {
                            let count = helper(s);
                            let n = b;
                            b += count;
                            (0..count).flat_map(move |i| [i + n, (i + 1) % count + n])
                        }),
                )
                .collect(),
            ui_start,
        )
    }
}

const MAX_FRAMES_IN_FLIGHT: usize = 2;

#[derive(Default)]
pub struct Renderer {
    ctx: Option<Context>,

    surface: vk::SurfaceKHR,

    queue_indices: QueueFamilyIndices,
    graphics_queue: vk::Queue,
    present_queue: vk::Queue,

    swapchain: vk::SwapchainKHR,
    swapchain_images: Vec<vk::Image>,
    swapchain_image_format: vk::Format,
    swapchain_extent: vk::Extent2D,
    swapchain_image_views: Vec<vk::ImageView>,
    swapchain_framebuffers: Vec<vk::Framebuffer>,

    render_pass: vk::RenderPass,
    ui_polygon_pipeline: Pipeline,
    game_polygon_pipeline: Pipeline,

    game_vb: Buffer<GameVertex>,
    ui_vb: Buffer<UiVertex>,
    index_buffer: Buffer<u16>,
    command_pool: vk::CommandPool,
    command_buffers: Vec<vk::CommandBuffer>,

    fences: [vk::Fence; MAX_FRAMES_IN_FLIGHT],
    image_available_semaphores: [vk::Semaphore; MAX_FRAMES_IN_FLIGHT],
    render_finished_semaphores: [vk::Semaphore; MAX_FRAMES_IN_FLIGHT],

    current_frame: usize,
    pub resized: bool,
}

impl Renderer {
    pub fn init(&mut self, window: &Window) -> Result<()> {
        self.ctx = Some(Context::new(window, &mut self.surface)?);
        self.create_queues();
        self.create_swapchain(window)?;
        self.create_pipelines()?;
        self.create_framebuffers()?;
        self.create_cmd()?;
        self.game_vb = self.create_vertex_buffer::<GameVertex>(20)?;
        self.ui_vb = self.create_vertex_buffer::<UiVertex>(20)?;
        self.index_buffer = self.create_index_buffer(20)?;
        self.create_sync()?;
        Ok(())
    }

    fn create_queues(&mut self) {
        unsafe {
            let ctx = self.ctx.as_ref().unwrap();
            self.queue_indices = QueueFamilyIndices::find(
                &ctx.instance,
                &ctx.surface_instance,
                self.surface,
                ctx.physical_device,
            );
            self.graphics_queue = ctx.device.get_device_queue(self.queue_indices.graphics, 0);
            self.present_queue = ctx.device.get_device_queue(self.queue_indices.present, 0);
        }
    }

    fn create_swapchain(&mut self, window: &Window) -> VkResult<()> {
        let ctx = self.ctx.as_ref().unwrap();
        let details = SwapchainSupportDetails::query(
            ctx.physical_device,
            &ctx.surface_instance,
            self.surface,
        )?;

        let surface_format = choose_swap_surface_format(&details.formats);
        let present_mode = choose_swap_present_mode(&details.present_modes);
        self.swapchain_extent = choose_swap_extent(&details.capabilities, window.inner_size());

        let mut image_count = details.capabilities.min_image_count + 1;

        if details.capabilities.max_image_count > 0
            && image_count > details.capabilities.max_image_count
        {
            image_count = details.capabilities.max_image_count
        }

        let mut swapchain_ci = vk::SwapchainCreateInfoKHR::default()
            .surface(self.surface)
            .min_image_count(image_count)
            .image_format(surface_format.format)
            .image_color_space(surface_format.color_space)
            .image_extent(self.swapchain_extent)
            .image_array_layers(1)
            .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
            .pre_transform(details.capabilities.current_transform)
            .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
            .present_mode(present_mode)
            .clipped(true);

        let indices = [self.queue_indices.graphics, self.queue_indices.present];
        if self.queue_indices.graphics != self.queue_indices.present {
            swapchain_ci = swapchain_ci
                .image_sharing_mode(vk::SharingMode::CONCURRENT)
                .queue_family_indices(&indices);
        }

        self.swapchain = unsafe { ctx.swapchain_device.create_swapchain(&swapchain_ci, None)? };
        self.swapchain_images =
            unsafe { ctx.swapchain_device.get_swapchain_images(self.swapchain)? };
        self.swapchain_image_format = surface_format.format;

        self.swapchain_image_views = self
            .swapchain_images
            .iter()
            .map(|&img| unsafe {
                ctx.device.create_image_view(
                    &vk::ImageViewCreateInfo::default()
                        .image(img)
                        .format(self.swapchain_image_format)
                        .components(vk::ComponentMapping {
                            r: vk::ComponentSwizzle::IDENTITY,
                            g: vk::ComponentSwizzle::IDENTITY,
                            b: vk::ComponentSwizzle::IDENTITY,
                            a: vk::ComponentSwizzle::IDENTITY,
                        })
                        .subresource_range(vk::ImageSubresourceRange {
                            aspect_mask: vk::ImageAspectFlags::COLOR,
                            base_mip_level: 0,
                            level_count: 1,
                            base_array_layer: 0,
                            layer_count: 1,
                        })
                        .view_type(vk::ImageViewType::TYPE_2D),
                    None,
                )
            })
            .collect::<VkResult<_>>()?;

        Ok(())
    }

    fn create_pipelines(&mut self) -> VkResult<()> {
        let ctx = self.ctx.as_ref().unwrap();

        let color_attachment = [vk::AttachmentDescription::default()
            .format(self.swapchain_image_format)
            .samples(vk::SampleCountFlags::TYPE_1)
            .load_op(vk::AttachmentLoadOp::CLEAR)
            .store_op(vk::AttachmentStoreOp::STORE)
            .stencil_load_op(vk::AttachmentLoadOp::DONT_CARE)
            .stencil_store_op(vk::AttachmentStoreOp::DONT_CARE)
            .initial_layout(vk::ImageLayout::UNDEFINED)
            .final_layout(vk::ImageLayout::PRESENT_SRC_KHR)];

        let subpass = [vk::SubpassDescription::default()
            .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
            .color_attachments(&[vk::AttachmentReference {
                attachment: 0,
                layout: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
            }])];

        let dependency = [vk::SubpassDependency::default()
            .src_subpass(vk::SUBPASS_EXTERNAL)
            .dst_subpass(0)
            .src_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
            .src_access_mask(vk::AccessFlags::NONE)
            .dst_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
            .dst_access_mask(vk::AccessFlags::COLOR_ATTACHMENT_WRITE)];

        self.render_pass = unsafe {
            ctx.device.create_render_pass(
                &vk::RenderPassCreateInfo::default()
                    .attachments(&color_attachment)
                    .subpasses(&subpass)
                    .dependencies(&dependency),
                None,
            )?
        };

        let mut pipelines = create_pipelines(
            ctx,
            &[
                PipelineCreateInfo {
                    vertex_shader: UI_VERTEX_SHADER,
                    fragment_shader: SIMPLE_UI_FRAGMENT_SHADER,
                    layout: vk::PipelineLayoutCreateInfo::default().push_constant_ranges(&[
                        vk::PushConstantRange::default()
                            .size(size_of::<UiPushConstants>() as _)
                            .stage_flags(vk::ShaderStageFlags::VERTEX),
                        vk::PushConstantRange::default()
                            .size(size_of::<FragPushConstants>() as _)
                            .offset(16)
                            .stage_flags(vk::ShaderStageFlags::FRAGMENT),
                    ]),
                    vertex_input_state: vk::PipelineVertexInputStateCreateInfo::default()
                        .vertex_binding_descriptions(&[vk::VertexInputBindingDescription::default(
                        )
                        .stride(size_of::<UiVertex>() as _)
                        .input_rate(vk::VertexInputRate::VERTEX)])
                        .vertex_attribute_descriptions(&[
                            vk::VertexInputAttributeDescription::default()
                                .format(vk::Format::R32G32_SFLOAT)
                                .offset(offset_of!(UiVertex, position) as _),
                            vk::VertexInputAttributeDescription::default()
                                .format(vk::Format::R32G32_SFLOAT)
                                .location(1)
                                .offset(offset_of!(UiVertex, anchor) as _),
                            vk::VertexInputAttributeDescription::default()
                                .format(vk::Format::R32G32B32A32_SFLOAT)
                                .location(2)
                                .offset(offset_of!(UiVertex, colour) as _),
                        ]),
                    render_pass: self.render_pass,
                },
                PipelineCreateInfo {
                    vertex_shader: GAME_VERTEX_SHADER,
                    fragment_shader: SIMPLE_GAME_FRAGMENT_SHADER,
                    layout: vk::PipelineLayoutCreateInfo::default().push_constant_ranges(&[
                        vk::PushConstantRange::default()
                            .size(size_of::<GamePushConstants>() as _)
                            .stage_flags(vk::ShaderStageFlags::VERTEX),
                        vk::PushConstantRange::default()
                            .size(size_of::<FragPushConstants>() as _)
                            .offset(32)
                            .stage_flags(vk::ShaderStageFlags::FRAGMENT),
                    ]),
                    vertex_input_state: vk::PipelineVertexInputStateCreateInfo::default()
                        .vertex_binding_descriptions(&[vk::VertexInputBindingDescription::default(
                        )
                        .stride(size_of::<GameVertex>() as _)
                        .input_rate(vk::VertexInputRate::VERTEX)])
                        .vertex_attribute_descriptions(&[
                            vk::VertexInputAttributeDescription::default()
                                .format(vk::Format::R32G32_SFLOAT),
                            vk::VertexInputAttributeDescription::default()
                                .format(vk::Format::R32G32B32A32_SFLOAT)
                                .location(1)
                                .offset(offset_of!(GameVertex, colour) as _),
                        ]),
                    render_pass: self.render_pass,
                },
            ],
            vk::PrimitiveTopology::LINE_LIST,
        )?;

        self.game_polygon_pipeline = pipelines.pop().unwrap();
        self.ui_polygon_pipeline = pipelines.pop().unwrap();

        Ok(())
    }

    fn create_framebuffers(&mut self) -> VkResult<()> {
        let ctx = self.ctx.as_ref().unwrap();

        self.swapchain_framebuffers = self
            .swapchain_image_views
            .iter()
            .map(|&view| unsafe {
                ctx.device.create_framebuffer(
                    &vk::FramebufferCreateInfo::default()
                        .render_pass(self.render_pass)
                        .attachment_count(1)
                        .attachments(&[view])
                        .width(self.swapchain_extent.width)
                        .height(self.swapchain_extent.height)
                        .layers(1),
                    None,
                )
            })
            .collect::<VkResult<_>>()?;

        Ok(())
    }

    fn create_cmd(&mut self) -> VkResult<()> {
        let ctx = self.ctx.as_ref().unwrap();

        self.command_pool = unsafe {
            ctx.device.create_command_pool(
                &vk::CommandPoolCreateInfo::default()
                    .flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER)
                    .queue_family_index(self.queue_indices.graphics),
                None,
            )?
        };

        self.command_buffers = unsafe {
            ctx.device.allocate_command_buffers(
                &vk::CommandBufferAllocateInfo::default()
                    .command_pool(self.command_pool)
                    .level(vk::CommandBufferLevel::PRIMARY)
                    .command_buffer_count(MAX_FRAMES_IN_FLIGHT as _),
            )?
        };

        Ok(())
    }

    fn create_vertex_buffer<T>(&self, len: usize) -> Result<Buffer<T>> {
        Buffer::new(
            self.ctx.as_ref().unwrap(),
            vk::BufferUsageFlags::VERTEX_BUFFER,
            vk::MemoryPropertyFlags::DEVICE_LOCAL | vk::MemoryPropertyFlags::HOST_VISIBLE,
            len,
        )
    }

    fn create_index_buffer(&self, len: usize) -> Result<Buffer<u16>> {
        Buffer::new(
            self.ctx.as_ref().unwrap(),
            vk::BufferUsageFlags::INDEX_BUFFER,
            vk::MemoryPropertyFlags::DEVICE_LOCAL | vk::MemoryPropertyFlags::HOST_VISIBLE,
            len,
        )
    }

    fn create_sync(&mut self) -> VkResult<()> {
        let ctx = self.ctx.as_ref().unwrap();

        unsafe {
            for i in 0..MAX_FRAMES_IN_FLIGHT {
                self.image_available_semaphores[i] = ctx
                    .device
                    .create_semaphore(&vk::SemaphoreCreateInfo::default(), None)?;
                self.render_finished_semaphores[i] = ctx
                    .device
                    .create_semaphore(&vk::SemaphoreCreateInfo::default(), None)?;
                self.fences[i] = ctx.device.create_fence(
                    &vk::FenceCreateInfo::default().flags(vk::FenceCreateFlags::SIGNALED),
                    None,
                )?;
            }
        }

        Ok(())
    }

    fn cleanup_swapchain(&self) {
        let ctx = self.ctx.as_ref().unwrap();

        unsafe {
            ctx.swapchain_device.destroy_swapchain(self.swapchain, None);

            for &framebuffer in &self.swapchain_framebuffers {
                ctx.device.destroy_framebuffer(framebuffer, None);
            }
            for &view in &self.swapchain_image_views {
                ctx.device.destroy_image_view(view, None);
            }
        }
    }

    fn recreate_swapchain(&mut self, window: &Window) -> VkResult<()> {
        unsafe {
            self.ctx.as_ref().unwrap().device.device_wait_idle()?;
        }

        self.cleanup_swapchain();

        self.create_swapchain(window)?;
        self.create_framebuffers()
    }

    pub fn render(&mut self, to_draw: &EverythingToDraw, window: &Window) -> Result<()> {
        let command_buffer = self.command_buffers[self.current_frame];
        let fence = self.fences[self.current_frame];
        let image_available_semaphore = self.image_available_semaphores[self.current_frame];
        let render_finished_semaphore = self.render_finished_semaphores[self.current_frame];
        let ctx = self.ctx.as_ref().unwrap();

        unsafe {
            ctx.device.wait_for_fences(&[fence], true, u64::MAX)?;
        }

        let image_index = unsafe {
            if self.resized {
                self.resized = false;
                self.recreate_swapchain(window)?;
                return self.render(to_draw, window);
            } else {
                match ctx.swapchain_device.acquire_next_image(
                    self.swapchain,
                    u64::MAX,
                    image_available_semaphore,
                    vk::Fence::null(),
                ) {
                    Ok((i, _)) => i,
                    Err(vk::Result::ERROR_OUT_OF_DATE_KHR) => {
                        self.recreate_swapchain(window)?;
                        return self.render(to_draw, window);
                    }
                    other => other?.0,
                }
            }
        };

        let game_pc = to_draw.game_pc(window);
        let ui_pc = to_draw.ui_pc(window);
        let frag_pc = to_draw.frag_pc();
        let game_vertices = to_draw.game_vertices();
        let ui_vertices = to_draw.ui_vertices();
        let (indices, ui_start) = to_draw.indices();
        let bg_colour = to_draw.inverted as u8 as f32;

        if game_vertices.len() > self.game_vb.len {
            self.game_vb = self.create_vertex_buffer::<GameVertex>(game_vertices.len())?;
        }
        self.game_vb.copy_from(
            game_vertices.as_ptr() as _,
            game_vertices.len() * size_of::<GameVertex>(),
            ctx,
        )?;
        if ui_vertices.len() > self.ui_vb.len {
            self.ui_vb = self.create_vertex_buffer::<UiVertex>(ui_vertices.len())?;
        }
        self.ui_vb.copy_from(
            ui_vertices.as_ptr() as _,
            ui_vertices.len() * size_of::<UiVertex>(),
            ctx,
        )?;
        if indices.len() > self.index_buffer.len {
            self.index_buffer = self.create_index_buffer(indices.len())?;
        }
        self.index_buffer.copy_from(
            indices.as_ptr() as _,
            indices.len() * size_of::<u16>(),
            ctx,
        )?;

        unsafe {
            ctx.device.reset_fences(&[fence]).unwrap();

            ctx.device
                .reset_command_buffer(command_buffer, vk::CommandBufferResetFlags::empty())?;

            ctx.device
                .begin_command_buffer(command_buffer, &Default::default())?;

            ctx.device.cmd_begin_render_pass(
                command_buffer,
                &vk::RenderPassBeginInfo::default()
                    .render_pass(self.render_pass)
                    .framebuffer(self.swapchain_framebuffers[image_index as usize])
                    .render_area(vk::Rect2D::default().extent(self.swapchain_extent))
                    .clear_values(&[vk::ClearValue {
                        color: vk::ClearColorValue {
                            float32: [bg_colour, bg_colour, bg_colour, 1.],
                        },
                    }]),
                vk::SubpassContents::INLINE,
            );

            ctx.device.cmd_bind_pipeline(
                command_buffer,
                vk::PipelineBindPoint::GRAPHICS,
                self.game_polygon_pipeline.pipeline,
            );

            ctx.device.cmd_set_viewport(
                command_buffer,
                0,
                &[vk::Viewport {
                    x: 0.,
                    y: 0.,
                    width: self.swapchain_extent.width as f32,
                    height: self.swapchain_extent.height as f32,
                    min_depth: 0.,
                    max_depth: 1.,
                }],
            );

            ctx.device.cmd_set_scissor(
                command_buffer,
                0,
                &[vk::Rect2D::default().extent(self.swapchain_extent)],
            );

            ctx.device
                .cmd_bind_vertex_buffers(command_buffer, 0, &[self.game_vb.buffer], &[0]);
            ctx.device.cmd_bind_index_buffer(
                command_buffer,
                self.index_buffer.buffer,
                0,
                vk::IndexType::UINT16,
            );

            ctx.device.cmd_push_constants(
                command_buffer,
                self.game_polygon_pipeline.layout,
                vk::ShaderStageFlags::VERTEX,
                0,
                bytes_of(&game_pc),
            );

            ctx.device.cmd_push_constants(
                command_buffer,
                self.game_polygon_pipeline.layout,
                vk::ShaderStageFlags::FRAGMENT,
                32,
                bytes_of(&frag_pc),
            );

            ctx.device
                .cmd_draw_indexed(command_buffer, ui_start as u32, 1, 0, 0, 0);

            ctx.device.cmd_bind_pipeline(
                command_buffer,
                vk::PipelineBindPoint::GRAPHICS,
                self.ui_polygon_pipeline.pipeline,
            );

            ctx.device
                .cmd_bind_vertex_buffers(command_buffer, 0, &[self.ui_vb.buffer], &[0]);

            ctx.device.cmd_push_constants(
                command_buffer,
                self.ui_polygon_pipeline.layout,
                vk::ShaderStageFlags::VERTEX,
                0,
                bytes_of(&ui_pc),
            );

            ctx.device.cmd_push_constants(
                command_buffer,
                self.ui_polygon_pipeline.layout,
                vk::ShaderStageFlags::FRAGMENT,
                16,
                bytes_of(&frag_pc),
            );

            ctx.device.cmd_draw_indexed(
                command_buffer,
                (indices.len() - ui_start) as _,
                1,
                ui_start as _,
                0,
                0,
            );

            ctx.device.cmd_end_render_pass(command_buffer);

            ctx.device.end_command_buffer(command_buffer).unwrap();

            let signal_semaphores = [render_finished_semaphore];

            ctx.device.queue_submit(
                self.graphics_queue,
                &[vk::SubmitInfo::default()
                    .wait_dst_stage_mask(&[vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT])
                    .wait_semaphores(&[image_available_semaphore])
                    .command_buffers(&[command_buffer])
                    .signal_semaphores(&signal_semaphores)],
                fence,
            )?;

            ctx.swapchain_device.queue_present(
                self.present_queue,
                &vk::PresentInfoKHR::default()
                    .wait_semaphores(&signal_semaphores)
                    .swapchains(&[self.swapchain])
                    .image_indices(&[image_index]),
            )?;
        }

        self.current_frame = (self.current_frame + 1) % MAX_FRAMES_IN_FLIGHT;
        Ok(())
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        let ctx = self.ctx.as_ref().unwrap();

        unsafe {
            ctx.device.device_wait_idle().unwrap();

            self.game_vb.free(ctx);
            self.ui_vb.free(ctx);
            self.index_buffer.free(ctx);

            for &semaphore in self
                .image_available_semaphores
                .iter()
                .chain(self.render_finished_semaphores.iter())
            {
                ctx.device.destroy_semaphore(semaphore, None);
            }
            for &fence in &self.fences {
                ctx.device.destroy_fence(fence, None);
            }

            self.cleanup_swapchain();

            ctx.device.destroy_command_pool(self.command_pool, None);
            for pipeline in [&self.game_polygon_pipeline, &self.ui_polygon_pipeline] {
                ctx.device.destroy_pipeline_layout(pipeline.layout, None);
                ctx.device.destroy_pipeline(pipeline.pipeline, None);
            }
            ctx.device.destroy_render_pass(self.render_pass, None);

            ctx.surface_instance.destroy_surface(self.surface, None);
        }
    }
}

#[repr(C)]
#[derive(Default, Clone, Copy, NoUninit, Debug)]
struct UiPushConstants {
    height: u32,
    width: u32,
}

#[repr(C)]
#[derive(Default, Clone, Copy, NoUninit, Debug)]
struct GamePushConstants {
    cam_pos: [f32; 2],
    scale: f32,
    width: u32,
    height: u32,
}

#[repr(C)]
#[derive(Default, Clone, Copy, NoUninit, Debug)]
struct FragPushConstants {
    inverted: bool,
    padding: [u8; 3],
}

#[repr(C)]
#[derive(Default, Debug)]
struct GameVertex {
    position: [f32; 2],
    colour: Vec4,
}

#[repr(C)]
#[derive(Default, Debug)]
struct UiVertex {
    position: [f32; 2],
    anchor: [f32; 2],
    colour: Vec4,
}
