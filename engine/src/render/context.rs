use std::ffi::CStr;

use anyhow::Result;
use ash::{
    ext::debug_utils,
    khr::{surface, swapchain},
    prelude::VkResult,
    vk, Device, Entry, Instance,
};
use ash_window::enumerate_required_extensions;
use winit::{
    raw_window_handle::{HasDisplayHandle, HasWindowHandle},
    window::Window,
};

use super::{
    debug::{
        check_layer_support, vulkan_debug_callback, Debug, DEBUG_ENABLED, RAW_REQUIRED_LAYERS,
    },
    utils::{QueueFamilyIndices, SwapchainSupportDetails},
};

const REQUIRED_DEVICE_EXTENSIONS: [&CStr; 1] = [swapchain::NAME];
const RAW_REQUIRED_DEVICE_EXTENSIONS: [*const i8; 1] =
    car::map!(REQUIRED_DEVICE_EXTENSIONS, |name| name.as_ptr());

pub struct Context {
    _entry: Entry,
    debug: Option<Debug>,
    pub instance: Instance,
    pub surface_instance: surface::Instance,
    pub device: Device,
    pub physical_device: vk::PhysicalDevice,
    pub physical_device_properties: vk::PhysicalDeviceProperties,
    pub swapchain_device: swapchain::Device,
}

impl Context {
    pub fn new(window: &Window, surface: &mut vk::SurfaceKHR) -> Result<Self> {
        let entry = unsafe { Entry::load()? };

        let app_info = vk::ApplicationInfo::default()
            .application_name(c"OwO")
            .application_version(vk::make_api_version(0, 0, 1, 0))
            .engine_name(c"No Engine")
            .engine_version(vk::make_api_version(0, 0, 1, 0))
            .api_version(vk::make_api_version(0, 1, 0, 0));

        let mut extension_names =
            enumerate_required_extensions(window.display_handle()?.as_raw())?.to_vec();

        if DEBUG_ENABLED {
            extension_names.push(debug_utils::NAME.as_ptr());
        }

        let mut instance_ci = vk::InstanceCreateInfo::default()
            .application_info(&app_info)
            .enabled_extension_names(&extension_names);

        let debug_ci = vk::DebugUtilsMessengerCreateInfoEXT::default()
            .message_severity(
                vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE
                // | vk::DebugUtilsMessageSeverityFlagsEXT::INFO
                | vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
                | vk::DebugUtilsMessageSeverityFlagsEXT::ERROR,
            )
            .message_type(
                vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
                    | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION
                    | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE,
            )
            .pfn_user_callback(Some(vulkan_debug_callback));

        if DEBUG_ENABLED {
            check_layer_support(&entry);
            instance_ci = instance_ci.enabled_layer_names(&RAW_REQUIRED_LAYERS);
            instance_ci.p_next = &debug_ci as *const _ as _;
        }

        let instance = unsafe { entry.create_instance(&instance_ci, None)? };
        let debug = Debug::new(&debug_ci, &entry, &instance)?;
        let surface_instance = surface::Instance::new(&entry, &instance);

        *surface = unsafe {
            ash_window::create_surface(
                &entry,
                &instance,
                window.display_handle()?.as_raw(),
                window.window_handle()?.as_raw(),
                None,
            )?
        };

        let scored_device = rate_devices(&instance, &unsafe {
            instance.enumerate_physical_devices()?
        })
        .max_by_key(|&(_, s)| s)
        .unwrap();

        let physical_device = if scored_device.1 == 0
            || !is_device_suitable(&instance, &surface_instance, *surface, scored_device.0)?
        {
            panic!("Failed to find a suitable GPU");
        } else {
            scored_device.0
        };

        let indices =
            QueueFamilyIndices::find(&instance, &surface_instance, *surface, physical_device);
        let queue_cis = [indices.graphics, indices.present].map(|i| {
            vk::DeviceQueueCreateInfo::default()
                .queue_family_index(i)
                .queue_priorities(&[1.])
        });

        let features = vk::PhysicalDeviceFeatures::default();
        let mut device_ci = vk::DeviceCreateInfo::default()
            .queue_create_infos(&queue_cis)
            .enabled_features(&features)
            .enabled_extension_names(&RAW_REQUIRED_DEVICE_EXTENSIONS);

        #[allow(deprecated)]
        if DEBUG_ENABLED {
            device_ci = device_ci.enabled_layer_names(&RAW_REQUIRED_LAYERS);
        }

        let device = unsafe { instance.create_device(physical_device, &device_ci, None)? };

        Ok(Self {
            _entry: entry,
            debug,
            surface_instance,
            physical_device,
            physical_device_properties: unsafe {
                instance.get_physical_device_properties(physical_device)
            },
            swapchain_device: swapchain::Device::new(&instance, &device),
            instance,
            device,
        })
    }

    pub fn create_shader_module(&self, source: &[u32]) -> VkResult<vk::ShaderModule> {
        unsafe {
            self.device
                .create_shader_module(&vk::ShaderModuleCreateInfo::default().code(source), None)
        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        drop(self.debug.take());
        unsafe {
            self.device.destroy_device(None);
            self.instance.destroy_instance(None);
        }
    }
}

fn rate_devices<'a>(
    instance: &'a Instance,
    devices: &'a [vk::PhysicalDevice],
) -> impl Iterator<Item = (vk::PhysicalDevice, u32)> + 'a {
    devices.iter().map(|&device| {
        let mut score = 0;
        let props = unsafe { instance.get_physical_device_properties(device) };

        if props.device_type == vk::PhysicalDeviceType::DISCRETE_GPU {
            score += 1000;
        }

        score += props.limits.max_image_dimension2_d;

        (device, score)
    })
}

fn is_device_suitable(
    instance: &Instance,
    surface_instance: &surface::Instance,
    surface: vk::SurfaceKHR,
    device: vk::PhysicalDevice,
) -> VkResult<bool> {
    let available_extensions = unsafe { instance.enumerate_device_extension_properties(device)? };
    let available_names = available_extensions
        .iter()
        .flat_map(|ext| ext.extension_name_as_c_str())
        .collect::<Vec<_>>();

    let supports_extensions = REQUIRED_DEVICE_EXTENSIONS
        .iter()
        .all(|name| available_names.contains(name));

    let swapchain_adequate = supports_extensions && {
        let details = SwapchainSupportDetails::query(device, surface_instance, surface)?;
        !details.formats.is_empty() && !details.present_modes.is_empty()
    };

    Ok(supports_extensions
        && swapchain_adequate
        && QueueFamilyIndices::find(instance, surface_instance, surface, device).is_complete())
}
