use std::ffi::{c_void, CStr};

use ash::{ext::debug_utils, prelude::VkResult, vk, Entry, Instance};
use colorize::AnsiColor;

#[cfg(not(debug_assertions))]
pub const DEBUG_ENABLED: bool = false;
#[cfg(debug_assertions)]
pub const DEBUG_ENABLED: bool = true;

pub const REQUIRED_LAYERS: [&CStr; 1] = [c"VK_LAYER_KHRONOS_validation"];
pub const RAW_REQUIRED_LAYERS: [*const i8; 1] = car::map!(REQUIRED_LAYERS, |name| name.as_ptr());

pub fn check_layer_support(entry: &Entry) {
    let props = unsafe { entry.enumerate_instance_layer_properties().unwrap() };
    for layer in REQUIRED_LAYERS {
        if !props
            .iter()
            .any(|prop| prop.layer_name_as_c_str().unwrap() == layer)
        {
            panic!("Layer not supported: {}", layer.to_str().unwrap());
        }
    }
}

pub unsafe extern "system" fn vulkan_debug_callback(
    message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    message_type: vk::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
    _user_data: *mut c_void,
) -> vk::Bool32 {
    let type_str = match message_type {
        vk::DebugUtilsMessageTypeFlagsEXT::GENERAL => "GENERAL",
        vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION => "VALIDATION",
        vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE => "PERFORMANCE",
        _ => "",
    };

    let mess_str = CStr::from_ptr((*p_callback_data).p_message)
        .to_str()
        .unwrap();

    eprintln!(
        "{}",
        match message_severity {
            vk::DebugUtilsMessageSeverityFlagsEXT::INFO =>
                format!("INFO ({}): {}", type_str, mess_str).blue(),
            vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE =>
                format!("VERBOSE ({}): {}", type_str, mess_str).green(),
            vk::DebugUtilsMessageSeverityFlagsEXT::WARNING =>
                format!("WARNING ({}): {}", type_str, mess_str).yellow(),
            vk::DebugUtilsMessageSeverityFlagsEXT::ERROR =>
                format!("ERROR ({}): {}", type_str, mess_str).red(),
            _ => unreachable!(),
        }
    );

    vk::FALSE
}

pub struct Debug(vk::DebugUtilsMessengerEXT, debug_utils::Instance);

impl Debug {
    pub fn new(
        debug_ci: &vk::DebugUtilsMessengerCreateInfoEXT,
        entry: &Entry,
        instance: &Instance,
    ) -> VkResult<Option<Self>> {
        if !DEBUG_ENABLED {
            Ok(None)
        } else {
            let instance = debug_utils::Instance::new(entry, instance);
            Ok(Some(Debug(
                unsafe { instance.create_debug_utils_messenger(debug_ci, None)? },
                instance,
            )))
        }
    }
}

impl Drop for Debug {
    fn drop(&mut self) {
        unsafe {
            self.1.destroy_debug_utils_messenger(self.0, None);
        }
    }
}
