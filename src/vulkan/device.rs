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

    pub fn find_queue_families(instance: &Instance, device: vk::PhysicalDevice, surface: vk::SurfaceKHR, ) {
        let queue_families = unsafe {instance.get_physical_device_queue_family_properties(device)};

        let mut graphics_family = None;
        let mut present_family = None;

        

    }



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