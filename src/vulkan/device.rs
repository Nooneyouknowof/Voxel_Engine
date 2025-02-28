use ash::vk;
use ash::Instance;

#[allow(unused)]
#[derive(Debug)]
pub struct VulkanPhysicalDevice {
    pub physical_device: vk::PhysicalDevice,
    pub swap_chain: vk::SwapchainKHR,
    pub swap_chain_images: Vec<vk::Image>,
    pub swap_chain_image_views: Vec<vk::ImageView>,
    pub render_pass: vk::RenderPass,
}

impl VulkanPhysicalDevice {
    pub fn pick_physical_device(instance: &Instance) -> VulkanPhysicalDevice {
        let devices = unsafe {
            instance
                .enumerate_physical_devices()
                .expect("Failed to enumerate physical devices!")
        };

        println!("Found {} physical devices.", devices.len());

        let mut selected_device = None;

        for &device in &devices {
            let properties = unsafe { instance.get_physical_device_properties(device) };
            let features = unsafe { instance.get_physical_device_features(device) };

            println!("Device: {:?} ({:?}) API_VER: {:?}", 
                unsafe { std::ffi::CStr::from_ptr(properties.device_name.as_ptr()) }, 
                properties.device_type,
                properties.api_version
            );
            // println!("Supported Features: {:?}", features); // Debug Vulkan Feature Support for device

            let is_discrete_gpu = properties.device_type == vk::PhysicalDeviceType::DISCRETE_GPU;
            let is_integrated_gpu = properties.device_type == vk::PhysicalDeviceType::INTEGRATED_GPU;
            let supports_geometry_shader = features.geometry_shader == vk::TRUE;

            if supports_geometry_shader {
                if is_discrete_gpu {
                    selected_device = Some(device);
                    break; // Immediately select discrete GPU if available
                } else if is_integrated_gpu && selected_device.is_none() {
                    selected_device = Some(device); // Fallback to integrated GPU if no discrete found
                }
            }
        }

        let physical_device = selected_device.expect("No suitable GPU found!");

        let swap_chain = Self::create_swap_chain(instance, physical_device);
        let swap_chain_images = vec![];
        let swap_chain_image_views = vec![];
        let render_pass = Self::create_render_pass(instance, physical_device);

        VulkanPhysicalDevice {
            physical_device,
            swap_chain,
            swap_chain_images,
            swap_chain_image_views,
            render_pass,
        }
    }

    // fn find_queue_families(instance: &ash::Instance, device: vk::PhysicalDevice, surface: vk::SurfaceKHR, surface_loader: &ash::extensions::khr::Surface) -> (u32, u32) {
    //     let queue_families = unsafe { instance.get_physical_device_queue_family_properties(device) };
    
    //     let mut graphics_queue_index = None;
    //     let mut present_queue_index = None;
    
    //     for (index, queue_family) in queue_families.iter().enumerate() {
    //         let index = index as u32;
    
    //         // Check for graphics support
    //         if queue_family.queue_flags.contains(vk::QueueFlags::GRAPHICS) {
    //             graphics_queue_index = Some(index);
    //         }
    
    //         // Check for presentation support
    //         let present_support = unsafe {
    //             surface_loader.get_physical_device_surface_support(device, index, surface)
    //         }.unwrap_or(false);
    
    //         if present_support {
    //             present_queue_index = Some(index);
    //         }
    
    //         if graphics_queue_index.is_some() && present_queue_index.is_some() {
    //             break;
    //         }
    //     }
    
    //     (
    //         graphics_queue_index.expect("Failed to find a graphics queue family!"),
    //         present_queue_index.expect("Failed to find a present queue family!")
    //     )
    // }    

    #[allow(unused)]
    fn create_swap_chain(instance: &Instance, device: vk::PhysicalDevice) -> vk::SwapchainKHR {
        // Placeholder for swap chain creation logic
        vk::SwapchainKHR::null()
    }

    #[allow(unused)]
    fn create_render_pass(instance: &Instance, device: vk::PhysicalDevice) -> vk::RenderPass {
        // Placeholder for render pass creation logic
        vk::RenderPass::null()
    }
}