use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::{event_loop::ActiveEventLoop, window::{Window, WindowId}};


use crate::vulkan::device::VulkanApp;
use ash::{vk, Entry, Instance};
use ash_window;
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use std::{ffi::CString, os::raw::c_char};

#[derive(Default)]
pub struct AppEvents {
    window: Option<Window>,
    instance: Option<Instance>,  // Store Vulkan instance
    surface: Option<vk::SurfaceKHR>,  // Store Vulkan surface
    entry: Option<Entry>,  // Store Vulkan entry
    logical_device: Option<ash::Device>
}

impl ApplicationHandler for AppEvents {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let attributes = Window::default_attributes()
            .with_title("Sage Zinnia (Beta)");
        self.window = Some(event_loop.create_window(attributes.clone()).unwrap());
        let window = self.window.as_ref().unwrap();

        let entry = unsafe { Entry::load().expect("Failed to load Vulkan") };
        // Application info
        let app_name = CString::new(attributes.title.to_string()).unwrap();
        let engine_name = CString::new("Custom Engine").unwrap();
        let app_info = vk::ApplicationInfo {
            s_type: vk::StructureType::APPLICATION_INFO,
            p_application_name: app_name.as_ptr(),
            application_version: vk::make_api_version(0, 1, 0, 0),
            p_engine_name: engine_name.as_ptr(),
            engine_version: vk::make_api_version(0, 1, 0, 0),
            api_version: vk::make_api_version(0, 1, 3, 0),
            ..Default::default()
        };

        // Get required extensions from winit
        let extension_names = required_extensions(window);

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
        
        self.entry = Some(entry);
        self.instance = Some(instance);
        self.surface = Some(surface);
        
        let surface_loader = ash::khr::surface::Instance::new(&self.entry.as_ref().unwrap(), &self.instance.as_ref().unwrap());
        println!("Vulkan surface & loader successfully created!");
        
        let instance = self.instance.as_ref().unwrap();
        let device = VulkanApp::pick_physical_device(&instance);
        println!("Physical Device: {:?}", device);

        let queue_family = VulkanApp::find_queue_families(instance, device.physical_device, *self.surface.as_ref().unwrap(), surface_loader);
        let logical_device = VulkanApp::create_logical_device(instance, device.physical_device, queue_family);
        self.logical_device = Some(logical_device.0);

        println!("Logical Device properties: {:?}, {:?}", logical_device.1, logical_device.2);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed");
                event_loop.exit();
            }

            WindowEvent::RedrawRequested => {
                // TODO: Draw frame using Vulkan
            }
            _ => ()
        }
    }

    fn exiting(&mut self, _: &ActiveEventLoop) {
        unsafe { self.logical_device.as_ref().unwrap().destroy_device(None) };
        println!("Exiting window");
        // Destroy Vulkan resources safely
        // if let Some(instance) = &self.instance {
        //     if let Some(surface) = self.surface.take() {
        //         unsafe {
        //             let surface_loader = ash::khr::Surface::new(self.entry.as_ref().unwrap(), instance);
        //             surface_loader.destroy_surface(surface, None);
        //         }
        //     }
        // }
    }
}

fn required_extensions(window: &Window) -> Vec<*const c_char> {
    let mut extensions = Vec::new();
    // Get required extensions from winit
    let surface_extensions = ash_window::enumerate_required_extensions(window.display_handle().unwrap().into()).unwrap();
    extensions.extend(surface_extensions.iter().copied());
    // Always include VK_KHR_SURFACE
    extensions.push(ash::khr::surface::NAME.as_ptr());
    extensions
}