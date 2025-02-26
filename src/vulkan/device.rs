use ash::vk;
use ash::Instance;

pub struct VulkanPhysicalDevice {
    pub physical_device: vk::PhysicalDevice,
}

impl VulkanPhysicalDevice {
    pub fn pick_physical_device(instance: &Instance) -> VulkanPhysicalDevice {
        let devices;
        unsafe {
            devices = instance
                .enumerate_physical_devices()
                .expect("Failed to enumerate physical devices!");
        };

        let physical_device = devices
            .into_iter()
            .find(|&device| Self::is_device_suitable(instance, device))
            .expect("No suitable GPU found!");

        VulkanPhysicalDevice { physical_device }
    }

    fn is_device_suitable(instance: &Instance, device: vk::PhysicalDevice) -> bool {
        let properties = unsafe {instance.get_physical_device_properties(device)};
        let features = unsafe {instance.get_physical_device_features(device)};

        let is_discrete_gpu = properties.device_type == vk::PhysicalDeviceType::DISCRETE_GPU;
        let supports_geometry_shader = features.geometry_shader == vk::TRUE;

        is_discrete_gpu && supports_geometry_shader
    }
}