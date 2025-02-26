mod window;

use ash::{vk, Entry, Instance};
use std::ffi::CString;
use winit::event_loop::{ControlFlow, EventLoop};
use window::*;

fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::default();
    event_loop.run_app(&mut app).expect("Run App failed!");
    println!("Hello, world!");
}

// Vulkan Stuff
struct VulkanApp {
    entry: Entry,
    instance: Instance,
}

impl VulkanApp {
    fn new() -> Self {
        let entry = unsafe { Entry::load().unwrap() }; // Load Vulkan
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

        let instance_info = vk::InstanceCreateInfo {
            s_type: vk::StructureType::INSTANCE_CREATE_INFO,
            p_application_info: &app_info,
            ..Default::default()
        };

        let instance = unsafe {
            entry
                .create_instance(&instance_info, None)
                .expect("Failed to create Vulkan instance")
        };

        Self { entry, instance }
    }
}
