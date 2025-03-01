use ash::vk;
use ash::Instance;


pub fn pick_physical_device(instance: &Instance) -> vk::PhysicalDevice {
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
    physical_device
}

pub fn find_queue_families(instance: &Instance, device: vk::PhysicalDevice, surface: vk::SurfaceKHR, surface_loader: ash::khr::surface::Instance) -> (u32, u32) {
    let queue_families = unsafe {instance.get_physical_device_queue_family_properties(device)};

    let mut graphics_queue_index: Option<u32> = None;
    let mut present_queue_index: Option<u32> = None;

    for (index, queue_family) in queue_families.iter().enumerate() {
        let index = index as u32; // Convert index to u32
        // Check if the queue supports graphics
        let supports_graphics = queue_family.queue_flags.contains(vk::QueueFlags::GRAPHICS);

        // Check if the queue supports presenting to the window
        let supports_presentation = unsafe {
            surface_loader
                .get_physical_device_surface_support(device, index as u32, surface)
                .unwrap()
        };
        if supports_graphics {
            graphics_queue_index = Some(index as u32);
        }
        if supports_presentation {
            present_queue_index = Some(index as u32);
        }
        if graphics_queue_index.is_some() && present_queue_index.is_some() {
            break;
        }
    }

    (
        graphics_queue_index.expect("No suitable graphics queue found!"),
        present_queue_index.expect("No suitable present queue found!")
    )
}

pub fn create_logical_device(instance: &ash::Instance, physical_device: vk::PhysicalDevice, queue_family: (u32, u32)) -> (ash::Device, vk::Queue, Option<vk::Queue>) {
    let (graphics_queue_index, present_queue_index) = queue_family;
    let use_single_queue = graphics_queue_index == present_queue_index;
    let mut unique_indices = vec![graphics_queue_index];
    if !use_single_queue {unique_indices.push(present_queue_index)};

    let queue_priorities = vec![1.0_f32];
    let queue_create_infos: Vec<_> = unique_indices.iter().map(|&queue_index| {
        vk::DeviceQueueCreateInfo {
            s_type: vk::StructureType::DEVICE_QUEUE_CREATE_INFO,
            flags: vk::DeviceQueueCreateFlags::empty(),
            queue_family_index: queue_index,
            p_queue_priorities: queue_priorities.as_ptr(),
            queue_count: queue_priorities.len() as u32,
            ..Default::default()
        }
    }).collect();

    let physical_device_features = vk::PhysicalDeviceFeatures::default();

    let device_create_info = vk::DeviceCreateInfo {
        s_type: vk::StructureType::DEVICE_CREATE_INFO,
        queue_create_info_count: queue_create_infos.len() as u32,
        p_queue_create_infos: queue_create_infos.as_ptr(),
        enabled_extension_count: 0,
        p_enabled_features: &physical_device_features,
        ..Default::default()
    };

    let logical_device: ash::Device = unsafe {
        instance
            .create_device(physical_device, &device_create_info, None)
            .expect("Failed to create logical Device!")
    };

    let graphics_queue = unsafe { logical_device.get_device_queue(graphics_queue_index, 0) };
    let present_queue = if use_single_queue {
        None // Same queue handles both
    } else {
        Some(unsafe { logical_device.get_device_queue(present_queue_index, 0) })
    };

    (logical_device, graphics_queue, present_queue)
}
