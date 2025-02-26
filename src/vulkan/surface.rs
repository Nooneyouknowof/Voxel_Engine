use ash::{vk, Entry, Instance};
use ash_window;
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use std::{ffi::CString, os::raw::c_char};
use winit::window::Window;

#[allow(dead_code)]
pub struct VulkanApp {
    entry: Entry,
    pub instance: Instance,
    surface: vk::SurfaceKHR,
}

impl VulkanApp {
    pub fn new(window: &Window) -> Self {
        let entry = unsafe { Entry::load().expect("Failed to load Vulkan") };

        // Application info
        let app_name = CString::new("Vulkan App").unwrap();
        let app_info = vk::ApplicationInfo {
            s_type: vk::StructureType::APPLICATION_INFO,
            p_application_name: app_name.as_ptr(),
            application_version: vk::make_api_version(0, 1, 0, 0),
            p_engine_name: app_name.as_ptr(),
            engine_version: vk::make_api_version(0, 1, 0, 0),
            api_version: vk::make_api_version(0, 1, 3, 0),
            ..Default::default()
        };

        // Get required extensions from winit
        let extension_names = VulkanApp::required_extensions(window);

        let instance_info = vk::InstanceCreateInfo {
            s_type: vk::StructureType::INSTANCE_CREATE_INFO,
            p_application_info: &app_info,
            enabled_extension_count: extension_names.len() as u32,
            pp_enabled_extension_names: extension_names.as_ptr(),
            ..Default::default()
        };

        let instance = unsafe {
            entry
                .create_instance(&instance_info, None)
                .expect("Failed to create Vulkan instance")
        };

        // Create Vulkan surface
        let surface = unsafe {
            ash_window::create_surface(
                &entry,
                &instance,
                window.display_handle().unwrap().into(),
                window.window_handle().unwrap().into(),
                None
            ).expect("Failed to create Vulkan surface")
        };

        Self {
            entry,
            instance,
            surface,
        }
    }

    fn required_extensions(window: &Window) -> Vec<*const c_char> {
        let mut extensions = vec![];

        // Get extensions required by winit
        let surface_extensions = ash_window::enumerate_required_extensions(window.display_handle().unwrap().into()).unwrap();
        extensions.extend(surface_extensions.iter().copied());

        extensions
    }
}