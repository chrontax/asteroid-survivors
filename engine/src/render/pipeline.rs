use ash::{prelude::VkResult, vk};

use super::context::Context;

pub struct PipelineCreateInfo<'a> {
    pub layout: vk::PipelineLayoutCreateInfo<'a>,
    pub vertex_shader: &'a [u32],
    pub vertex_input_state: vk::PipelineVertexInputStateCreateInfo<'a>,
    pub fragment_shader: &'a [u32],
    pub render_pass: vk::RenderPass,
}

#[derive(Default)]
pub struct Pipeline {
    pub layout: vk::PipelineLayout,
    pub pipeline: vk::Pipeline,
}

pub fn create_pipelines(
    ctx: &Context,
    ci: &[PipelineCreateInfo],
    topology: vk::PrimitiveTopology,
) -> VkResult<Vec<Pipeline>> {
    let layouts = ci
        .iter()
        .map(|ci| unsafe { ctx.device.create_pipeline_layout(&ci.layout, None) })
        .collect::<VkResult<Vec<_>>>()?;

    let vertex_shader_modules = ci
        .iter()
        .map(|ci| ctx.create_shader_module(ci.vertex_shader))
        .collect::<VkResult<Vec<_>>>()?;
    let fragment_shader_modules = ci
        .iter()
        .map(|ci| ctx.create_shader_module(ci.fragment_shader))
        .collect::<VkResult<Vec<_>>>()?;

    let dynamic_states = [vk::DynamicState::VIEWPORT, vk::DynamicState::SCISSOR];
    let dynamic_state_ci =
        vk::PipelineDynamicStateCreateInfo::default().dynamic_states(&dynamic_states);

    let input_assembly_state_ci =
        vk::PipelineInputAssemblyStateCreateInfo::default().topology(topology);
    let viewport_state_ci = vk::PipelineViewportStateCreateInfo::default()
        .viewport_count(1)
        .scissor_count(1);
    let rasterization_state_ci = vk::PipelineRasterizationStateCreateInfo::default()
        .depth_clamp_enable(false)
        .polygon_mode(vk::PolygonMode::FILL)
        .line_width(1.);
    let multisample_state_ci = vk::PipelineMultisampleStateCreateInfo::default()
        .sample_shading_enable(false)
        .rasterization_samples(vk::SampleCountFlags::TYPE_1)
        .min_sample_shading(1.);
    let color_blend_state_ci = vk::PipelineColorBlendStateCreateInfo::default().attachments(&[
        vk::PipelineColorBlendAttachmentState {
            color_write_mask: vk::ColorComponentFlags::RGBA,
            blend_enable: vk::TRUE,
            src_color_blend_factor: vk::BlendFactor::SRC_ALPHA,
            dst_color_blend_factor: vk::BlendFactor::ONE_MINUS_SRC_ALPHA,
            color_blend_op: vk::BlendOp::ADD,
            src_alpha_blend_factor: vk::BlendFactor::ONE,
            dst_alpha_blend_factor: vk::BlendFactor::ZERO,
            alpha_blend_op: vk::BlendOp::ADD,
        },
    ]);

    let stages = vertex_shader_modules
        .iter()
        .zip(fragment_shader_modules.iter())
        .map(|(&v, &f)| {
            [
                vk::PipelineShaderStageCreateInfo::default()
                    .stage(vk::ShaderStageFlags::VERTEX)
                    .module(v)
                    .name(c"main"),
                vk::PipelineShaderStageCreateInfo::default()
                    .stage(vk::ShaderStageFlags::FRAGMENT)
                    .module(f)
                    .name(c"main"),
            ]
        })
        .collect::<Vec<_>>();

    let pipelines = match unsafe {
        ctx.device.create_graphics_pipelines(
            vk::PipelineCache::null(),
            &stages
                .iter()
                .zip(
                    layouts
                        .iter()
                        .zip(ci.iter().map(|ci| (&ci.vertex_input_state, ci.render_pass))),
                )
                .map(|(s, (&l, (v, r)))| {
                    vk::GraphicsPipelineCreateInfo::default()
                        .stages(s)
                        .vertex_input_state(v)
                        .render_pass(r)
                        .input_assembly_state(&input_assembly_state_ci)
                        .viewport_state(&viewport_state_ci)
                        .rasterization_state(&rasterization_state_ci)
                        .multisample_state(&multisample_state_ci)
                        .color_blend_state(&color_blend_state_ci)
                        .dynamic_state(&dynamic_state_ci)
                        .layout(l)
                        .subpass(0)
                })
                .collect::<Vec<_>>(),
            None,
        )
    } {
        Ok(p) => p,
        Err((p, r)) => {
            r.result()?;
            p
        }
    };

    unsafe {
        for &shader in vertex_shader_modules
            .iter()
            .chain(fragment_shader_modules.iter())
        {
            ctx.device.destroy_shader_module(shader, None);
        }
    }

    Ok(pipelines
        .iter()
        .zip(layouts.iter())
        .map(|(&pipeline, &layout)| Pipeline { layout, pipeline })
        .collect())
}
