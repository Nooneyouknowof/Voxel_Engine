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
        let devices;
        unsafe {
            devices = instance
                .enumerate_physical_devices()
                .expect("Failed to enumerate physical devices!");
        };
        println!("GPU Devices: {:?}", devices);

        let physical_device = devices
            .into_iter()
            .find(|&device| Self::is_device_suitable(instance, device))
            .expect("No suitable GPU found!");

        let swap_chain = Self::create_swap_chain(instance, physical_device);
        let swap_chain_images = vec![];
        let swap_chain_image_views = vec![];
        let render_pass = Self::create_render_pass(instance, physical_device);

        VulkanPhysicalDevice {
            physical_device,
            swap_chain,
            swap_chain_images,
            swap_chain_image_views,
            render_pass
        }
    }

    fn is_device_suitable(instance: &Instance, device: vk::PhysicalDevice) -> bool {
        let properties = unsafe {instance.get_physical_device_properties(device)};
        let features = unsafe {instance.get_physical_device_features(device)};

        let is_discrete_gpu = properties.device_type == vk::PhysicalDeviceType::DISCRETE_GPU;
        let supports_geometry_shader = features.geometry_shader == vk::TRUE;

        is_discrete_gpu && supports_geometry_shader
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