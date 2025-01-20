use std::{ffi::c_void, marker::PhantomData};

use anyhow::{bail, Context as AnyhowContext, Result};
use ash::vk;

use super::context::Context;

#[derive(Default)]
pub struct Buffer<T> {
    pub buffer: vk::Buffer,
    memory: vk::DeviceMemory,
    properties: vk::MemoryPropertyFlags,
    usage: vk::BufferUsageFlags,
    mapped: Option<*mut c_void>,
    size: vk::DeviceSize,
    pub len: usize,
    phantom: PhantomData<T>,
}

impl<T> Buffer<T> {
    pub fn new(
        ctx: &Context,
        usage: vk::BufferUsageFlags,
        properties: vk::MemoryPropertyFlags,
        mut len: usize,
    ) -> Result<Self> {
        let mut size = (len * size_of::<T>()) as _;
        size += ctx.physical_device_properties.limits.non_coherent_atom_size
            - size % ctx.physical_device_properties.limits.non_coherent_atom_size;
        len = size as usize / size_of::<T>();

        unsafe {
            let buffer = ctx.device.create_buffer(
                &vk::BufferCreateInfo::default()
                    .size(size)
                    .usage(usage)
                    .sharing_mode(vk::SharingMode::EXCLUSIVE),
                None,
            )?;

            let memory_requirements = ctx.device.get_buffer_memory_requirements(buffer);

            let memory = ctx.device.allocate_memory(
                &vk::MemoryAllocateInfo::default()
                    .allocation_size(memory_requirements.size)
                    .memory_type_index(
                        find_memory_type(
                            memory_requirements.memory_type_bits,
                            properties,
                            ctx.instance
                                .get_physical_device_memory_properties(ctx.physical_device),
                        )
                        .context("Couldn't find a suitable memory type")?,
                    ),
                None,
            )?;

            ctx.device.bind_buffer_memory(buffer, memory, 0)?;

            Ok(Self {
                buffer,
                memory,
                usage,
                properties,
                mapped: if properties.contains(vk::MemoryPropertyFlags::HOST_VISIBLE) {
                    Some(
                        ctx.device
                            .map_memory(memory, 0, size, vk::MemoryMapFlags::empty())?,
                    )
                } else {
                    None
                },
                len,
                size,
                phantom: PhantomData,
            })
        }
    }

    pub fn copy_from(&self, ptr: *const c_void, len: usize, ctx: &Context) -> Result<()> {
        let Some(mapped) = self.mapped else {
            bail!("Cannot write to buffer from host");
        };

        unsafe {
            mapped.copy_from(ptr, len);

            if !self
                .properties
                .contains(vk::MemoryPropertyFlags::HOST_COHERENT)
            {
                ctx.device
                    .flush_mapped_memory_ranges(&[vk::MappedMemoryRange::default()
                        .memory(self.memory)
                        .offset(0)
                        .size(self.size)])?;
            }
        }

        Ok(())
    }

    // currently unused, but may come in handy
    #[allow(unused)]
    pub fn copy_to(
        &self,
        other: &Self,
        ctx: &Context,
        pool: vk::CommandPool,
        queue: vk::Queue,
    ) -> Result<()> {
        if !self.usage.contains(vk::BufferUsageFlags::TRANSFER_SRC) {
            bail!("Source buffer cannot be used for transfer");
        }
        if !other.usage.contains(vk::BufferUsageFlags::TRANSFER_DST) {
            bail!("Destination buffer cannot be used for transfer");
        }
        if self.size != other.size {
            bail!("Size mismatch");
        }

        unsafe {
            let command_buffer = ctx
                .device
                .allocate_command_buffers(
                    &vk::CommandBufferAllocateInfo::default()
                        .level(vk::CommandBufferLevel::PRIMARY)
                        .command_pool(pool)
                        .command_buffer_count(1),
                )
                .unwrap()[0];

            ctx.device
                .begin_command_buffer(
                    command_buffer,
                    &vk::CommandBufferBeginInfo::default()
                        .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT),
                )
                .unwrap();

            ctx.device.cmd_copy_buffer(
                command_buffer,
                self.buffer,
                other.buffer,
                &[vk::BufferCopy::default().size(self.size)],
            );

            ctx.device.end_command_buffer(command_buffer).unwrap();

            ctx.device
                .queue_submit(
                    queue,
                    &[vk::SubmitInfo::default().command_buffers(&[command_buffer])],
                    vk::Fence::null(),
                )
                .unwrap();
            ctx.device.queue_wait_idle(queue).unwrap();

            ctx.device.free_command_buffers(pool, &[command_buffer]);
        }

        Ok(())
    }

    pub fn free(&self, ctx: &Context) {
        unsafe {
            ctx.device.destroy_buffer(self.buffer, None);
            ctx.device.free_memory(self.memory, None);
        }
    }
}

fn find_memory_type(
    filter: u32,
    properties: vk::MemoryPropertyFlags,
    mem_properties: vk::PhysicalDeviceMemoryProperties,
) -> Option<u32> {
    (0..mem_properties.memory_type_count).find(|&i| {
        (filter & (1 << i)) != 0
            && (mem_properties.memory_types[i as usize]
                .property_flags
                .contains(properties))
    })
}
