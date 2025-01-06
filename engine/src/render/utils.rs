use ash::{khr::surface, prelude::VkResult, vk, Instance};
use winit::dpi::PhysicalSize;

#[derive(Default)]
pub struct QueueFamilyIndices {
    pub graphics: u32,
    pub present: u32,
}

impl QueueFamilyIndices {
    pub fn find(
        instance: &Instance,
        surface_instance: &surface::Instance,
        surface: vk::SurfaceKHR,
        device: vk::PhysicalDevice,
    ) -> Self {
        unsafe { instance.get_physical_device_queue_family_properties(device) }
            .iter()
            .enumerate()
            .fold(
                Self {
                    graphics: u32::MAX,
                    present: u32::MAX,
                },
                |mut acc, (i, family)| {
                    if family.queue_flags.contains(vk::QueueFlags::GRAPHICS) {
                        acc.graphics = i as u32
                    }
                    if unsafe {
                        surface_instance
                            .get_physical_device_surface_support(device, i as u32, surface)
                            .unwrap()
                    } {
                        acc.present = i as u32
                    }
                    acc
                },
            )
    }

    pub fn is_complete(&self) -> bool {
        self.graphics != u32::MAX && self.present != u32::MAX
    }
}

pub struct SwapchainSupportDetails {
    pub capabilities: vk::SurfaceCapabilitiesKHR,
    pub formats: Vec<vk::SurfaceFormatKHR>,
    pub present_modes: Vec<vk::PresentModeKHR>,
}

impl SwapchainSupportDetails {
    pub fn query(
        device: vk::PhysicalDevice,
        surface_instance: &surface::Instance,
        surface: vk::SurfaceKHR,
    ) -> VkResult<Self> {
        unsafe {
            Ok(Self {
                capabilities: surface_instance
                    .get_physical_device_surface_capabilities(device, surface)?,
                formats: surface_instance.get_physical_device_surface_formats(device, surface)?,
                present_modes: surface_instance
                    .get_physical_device_surface_present_modes(device, surface)?,
            })
        }
    }
}

pub fn choose_swap_surface_format(available: &[vk::SurfaceFormatKHR]) -> vk::SurfaceFormatKHR {
    *available
        .iter()
        .find(|format| {
            format.format == vk::Format::B8G8R8A8_SRGB
                && format.color_space == vk::ColorSpaceKHR::SRGB_NONLINEAR
        })
        .unwrap_or(&available[0])
}

pub fn choose_swap_present_mode(available: &[vk::PresentModeKHR]) -> vk::PresentModeKHR {
    if available.contains(&vk::PresentModeKHR::MAILBOX) {
        vk::PresentModeKHR::MAILBOX
    } else {
        vk::PresentModeKHR::FIFO
    }
}

pub fn choose_swap_extent(
    capabilites: &vk::SurfaceCapabilitiesKHR,
    window_size: PhysicalSize<u32>,
) -> vk::Extent2D {
    match capabilites.current_extent.width {
        u32::MAX => vk::Extent2D {
            width: window_size.width,
            height: window_size.height,
        },
        _ => capabilites.current_extent,
    }
}
