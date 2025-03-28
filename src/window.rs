use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::{event_loop::ActiveEventLoop, window::{Window, WindowId}};


use crate::vulkan::device::*;
use crate::vulkan::swapchain::*;
use crate::vulkan::other::*;
use ash::{vk, Entry, Instance};
use ash_window;
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use std::{ffi::CString, os::raw::c_char};

#[derive(Default)]
pub struct AppEvents {
    window: Option<Window>,
    entry: Option<Entry>,
    pub instance: Option<Instance>,
    pub surface: vk::SurfaceKHR,
    pub surface_loader: Option<ash::khr::surface::Instance>,
    pub physical_device: vk::PhysicalDevice,
    pub swapchain: vk::SwapchainKHR,
    pub swapchain_loader: Option<ash::khr::swapchain::Device>,
    pub logical_device: Option<ash::Device>,
    swapchain_imageviews: Vec<vk::ImageView>,
    render_pass: vk::RenderPass,
    pipeline_layout: vk::PipelineLayout,
    graphics_pipeline: vk::Pipeline,
    command_buffers: Vec<vk::CommandBuffer>,
    queue: vk::Queue,
    pub queue_family: (u32, u32),

    image_available_semaphores: Vec<vk::Semaphore>,
    render_finished_semaphores: Vec<vk::Semaphore>,
    in_flight_fences: Vec<vk::Fence>,
    current_frame: usize,
}

impl ApplicationHandler for AppEvents {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let attributes = Window::default_attributes()
            .with_title("Sage Zinnia (Beta)");
        self.window = Some(event_loop.create_window(attributes.clone()).unwrap());
        let window = self.window.as_ref().unwrap();

        let entry = unsafe { Entry::load().expect("Failed to load Vulkan") };
        // println!("Supports Validation Layers {:?}", check_validation_layer_support(&entry));
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
        // println!("Required Extensions for winit: "); // Debug
        // for ext in &extension_names {
        //     let ext_name = unsafe { CStr::from_ptr(ext.clone()) };
        //     println!("- {}", ext_name.to_string_lossy());
        // }

        // let validation_layers = vec!["VK_LAYER_KHRONOS_validation"];
        let instance_info = vk::InstanceCreateInfo {
            s_type: vk::StructureType::INSTANCE_CREATE_INFO,
            p_application_info: &app_info,
            enabled_extension_count: extension_names.len() as u32,
            pp_enabled_extension_names: extension_names.as_ptr(),
            // enabled_layer_count: validation_layers.len() as u32,
            // pp_enabled_layer_names: validation_layers.as_ptr() as *const *const i8,
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
        self.surface = surface;
        self.surface_loader = Some(ash::khr::surface::Instance::new(&self.entry.as_ref().unwrap(), &self.instance.as_ref().unwrap()));

        println!("Vulkan surface & surface loader successfully created!");

        let instance = self.instance.as_ref().unwrap();
        let physical_device = pick_physical_device(&instance);
        self.physical_device = physical_device;

        let queue_family = find_queue_families(instance, physical_device,
            self.surface, self.surface_loader.as_ref().unwrap().clone());
        self.queue_family = queue_family;
        let logical_device = create_logical_device(instance, physical_device, queue_family);
        self.logical_device = Some(logical_device.0);
        self.queue = logical_device.1;

        println!("Logical Device properties: {:?}, {:?}", logical_device.1, logical_device.2);

        let swapchain_stuff = create_swap_chain(
            instance,
            self.logical_device.as_ref().unwrap().clone(),
            physical_device,
            surface,
            self.surface_loader.as_ref().unwrap().clone(),
            queue_family,
            window
        );

        self.swapchain = swapchain_stuff.swapchain;
        self.swapchain_loader = Some(swapchain_stuff.swapchain_loader);
        println!("Swapchain: {:?}", swapchain_stuff.swapchain);

        let swapchain_imageviews = create_image_views(
            self.logical_device.as_ref().unwrap(),
            swapchain_stuff.swapchain_format,
            &swapchain_stuff.swapchain_images
        );
        self.swapchain_imageviews = swapchain_imageviews;

        let render_pass = create_render_pass(
            self.logical_device.as_ref().unwrap(),
            swapchain_stuff.swapchain_format
        );
        println!("Render Pass: {:?}", render_pass);
        self.render_pass = render_pass;
        let (graphics_pipeline, pipeline_layout) = create_graphics_pipeline(
            self.logical_device.as_ref().unwrap(),
            self.render_pass,
            swapchain_stuff.swapchain_extent
        );
        self.graphics_pipeline = graphics_pipeline;
        self.pipeline_layout = pipeline_layout;


        println!("Graphics Pipeline: {:?}", graphics_pipeline);
        println!("Pipeline Layout: {:?}", pipeline_layout);

        let swapchain_framebuffers = create_framebuffers(
            self.logical_device.as_ref().unwrap(),
            render_pass,
            &self.swapchain_imageviews,
            &swapchain_stuff.swapchain_extent,
        );
        println!("Swapchain Framebuffers: {:?}", swapchain_framebuffers);

        println!("Queue Family: {:?}", queue_family);
        let command_pool = create_command_pool(&self.logical_device.as_ref().unwrap(), queue_family.1);
        println!("Command Pool: {:?}", command_pool);
        let command_buffers = create_command_buffers(
            &self.logical_device.as_ref().unwrap(),
            command_pool,
            graphics_pipeline,
            &swapchain_framebuffers,
            render_pass,
            swapchain_stuff.swapchain_extent,
        );
        println!("Command Buffers: {:?}", command_buffers);
        self.command_buffers = command_buffers;

        let sync_objects = create_sync_objects(&self.logical_device.as_ref().unwrap());
        self.image_available_semaphores = sync_objects.image_available_semaphores;
        self.render_finished_semaphores = sync_objects.render_finished_semaphores;
        self.in_flight_fences = sync_objects.in_flight_fences;
        self.current_frame = 0;
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::Resized(_) => {
                let window = self.window.as_ref().unwrap();
                println!("Window was resized: {:?}", window.inner_size());
            }

            WindowEvent::CloseRequested => {
                println!("\nThe close button was pressed");
                event_loop.exit();
            }

            WindowEvent::RedrawRequested => {
                let wait_fences = [self.in_flight_fences[self.current_frame]];

                unsafe {
                    // Wait for the previous frame to finish
                    self.logical_device.as_ref().unwrap()
                        .wait_for_fences(&wait_fences, true, std::u64::MAX)
                        .expect("Failed to wait for Fence!");

                    // Reset the fence for this frame
                    self.logical_device.as_ref().unwrap()
                        .reset_fences(&wait_fences)
                        .expect("Failed to reset Fence!");

                    // Acquire the next image
                    let (image_index, _is_sub_optimal) = match self.swapchain_loader.as_ref().unwrap()
                        .acquire_next_image(
                            self.swapchain,
                            std::u64::MAX,
                            self.image_available_semaphores[self.current_frame],
                            vk::Fence::null(),
                        ) {
                            Ok((index, sub_optimal)) => (index, sub_optimal),
                            Err(e) => {
                                println!("Failed to acquire next image: {:?}", e);
                                return;
                            }
                        };

                    // Submit the command buffer
                    let wait_semaphores = [self.image_available_semaphores[self.current_frame]];
                    let wait_stages = [vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];
                    let command_buffers = [self.command_buffers[image_index as usize]];
                    let signal_semaphores = [self.render_finished_semaphores[self.current_frame]];

                    let submit_info = vk::SubmitInfo {
                        s_type: vk::StructureType::SUBMIT_INFO,
                        wait_semaphore_count: 1,
                        p_wait_semaphores: wait_semaphores.as_ptr(),
                        p_wait_dst_stage_mask: wait_stages.as_ptr(),
                        command_buffer_count: 1,
                        p_command_buffers: command_buffers.as_ptr(),
                        signal_semaphore_count: 1,
                        p_signal_semaphores: signal_semaphores.as_ptr(),
                        ..Default::default()
                    };

                    self.logical_device.as_ref().unwrap()
                        .queue_submit(
                            self.queue,
                            &[submit_info],
                            self.in_flight_fences[self.current_frame],
                        )
                        .expect("Failed to submit draw command buffer");

                    // Present the image
                    let swapchains = [self.swapchain];
                    let image_indices = [image_index];
                    let present_info = vk::PresentInfoKHR {
                        s_type: vk::StructureType::PRESENT_INFO_KHR,
                        wait_semaphore_count: 1,
                        p_wait_semaphores: signal_semaphores.as_ptr(),
                        swapchain_count: 1,
                        p_swapchains: swapchains.as_ptr(),
                        p_image_indices: image_indices.as_ptr(),
                        p_results: std::ptr::null_mut(),
                        ..Default::default()
                    };

                    match self.swapchain_loader.as_ref().unwrap()
                        .queue_present(self.queue, &present_info) {
                        Ok(_) => {},
                        Err(e) => println!("Failed to present queue: {:?}", e),
                    }

                    self.current_frame = (self.current_frame + 1) % MAX_FRAMES_IN_FLIGHT;
                }
            }
            _ => ()
        }
    }

    fn exiting(&mut self, _: &ActiveEventLoop) {
        // Destroy Vulkan resources safely
        unsafe { self.logical_device.as_ref().unwrap().destroy_device(None) };
        unsafe { self.surface_loader.as_ref().unwrap().destroy_surface(self.surface, None) };
        unsafe { self.instance.as_ref().unwrap().destroy_instance(None) };
        println!("Exiting window");
    }
}

fn required_extensions(window: &Window) -> Vec<*const c_char> {
    let mut extensions = Vec::new();
    // Get required extensions from winit
    let surface_extensions = ash_window::enumerate_required_extensions(window.display_handle().unwrap().into()).unwrap();
    extensions.extend(surface_extensions.iter().copied());
    extensions
}

// fn check_validation_layer_support(entry: &ash::Entry) -> bool {
//     let available_layers = unsafe {
//         entry.enumerate_instance_layer_properties().unwrap()
//     };

//     for layer in &["VK_LAYER_KHRONOS_validation"] {
//         let found = available_layers.iter().any(|l| {
//             unsafe { CStr::from_ptr(l.layer_name.as_ptr()) }
//                 .to_str()
//                 .unwrap() == *layer
//         });

//         if !found {
//             return false;
//         }
//     }
//     true
// }