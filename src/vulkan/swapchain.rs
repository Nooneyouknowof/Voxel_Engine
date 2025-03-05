use std::marker::PhantomData;
use std::ptr;

use ash::vk;
use ash::Instance;
use winit::window::Window;

pub struct SwapChainSupportDetails {
    pub capabilities: vk::SurfaceCapabilitiesKHR,
    pub formats: Vec<vk::SurfaceFormatKHR>,
    pub present_modes: Vec<vk::PresentModeKHR>,
}

#[allow(unused)]
pub struct SwapChainStuff {
    pub swapchain_loader: ash::khr::swapchain::Device,
    pub swapchain: vk::SwapchainKHR,
    pub swapchain_images: Vec<vk::Image>,
    pub swapchain_format: vk::Format,
    pub swapchain_extent: vk::Extent2D,
}

fn query_swapchain_support(physical_device: vk::PhysicalDevice, surface: vk::SurfaceKHR, surface_loader: ash::khr::surface::Instance) -> SwapChainSupportDetails {
    unsafe {
        let capabilities = surface_loader
            .get_physical_device_surface_capabilities(physical_device, surface)
            .expect("Failed to query for surface capabilities.");
        let formats = surface_loader
            .get_physical_device_surface_formats(physical_device, surface)
            .expect("Failed to query for surface formats.");
        let present_modes = surface_loader
            .get_physical_device_surface_present_modes(physical_device, surface)
            .expect("Failed to query for surface present mode.");

        SwapChainSupportDetails {
            capabilities,
            formats,
            present_modes
        }
    }
}

fn choose_swapchain_format(available_formats: &Vec<vk::SurfaceFormatKHR>) -> vk::SurfaceFormatKHR {
    for available_format in available_formats {
        if available_format.format == vk::Format::B8G8R8A8_SRGB && available_format.color_space == vk::ColorSpaceKHR::SRGB_NONLINEAR {
            return available_format.clone();
        }
    }
    return available_formats.first().unwrap().clone(); // Default
}

fn choose_swapchain_present_mode(available_present_modes: &Vec<vk::PresentModeKHR>) -> vk::PresentModeKHR {
    for &available_present_mode in available_present_modes {
        if available_present_mode == vk::PresentModeKHR::MAILBOX {
            return available_present_mode;
        }
    }
    vk::PresentModeKHR::FIFO // Default
}

fn choose_swapchain_extent(capabilities: &vk::SurfaceCapabilitiesKHR, window: &Window) -> vk::Extent2D {
    // println!("Capabilities: {:?}", capabilities);
    if capabilities.current_extent.width != u32::max_value() {
        capabilities.current_extent
    } else {
        let size = window.inner_size();
        let width = size.width;
        let height = size.height;
        vk::Extent2D {
            width: width.clamp(
                capabilities.min_image_extent.width,
                capabilities.max_image_extent.width
            ),
            height: height.clamp(
                capabilities.min_image_extent.height,
                capabilities.max_image_extent.height
            )
        }
    }
}

pub fn create_swap_chain(
    instance: &Instance, 
    device: ash::Device, 
    physical_device: vk::PhysicalDevice, 
    surface: vk::SurfaceKHR, 
    surface_loader: ash::khr::surface::Instance,
    queue_family: (u32, u32),
    window: &Window
) -> SwapChainStuff {
    let swapchain_support = query_swapchain_support(physical_device, surface, surface_loader.clone());

    let surface_format = choose_swapchain_format(&swapchain_support.formats);
    let present_mode = choose_swapchain_present_mode(&swapchain_support.present_modes);
    let extent = choose_swapchain_extent(&swapchain_support.capabilities, window);

    let image_count = swapchain_support.capabilities.min_image_count + 1;
    let image_count = if swapchain_support.capabilities.max_image_count > 0 {
        image_count.min(swapchain_support.capabilities.max_image_count)
    } else {
        image_count
    };

    let (image_sharing_mode, queue_family_index_count, queue_family_indices) =
    if queue_family.0 != queue_family.1 {
        (
            vk::SharingMode::CONCURRENT,
            2,
            vec![
                queue_family.0,
                queue_family.1,
            ],
        )
    } else {
        (vk::SharingMode::EXCLUSIVE, 0, vec![])
    };

    let swapchain_create_info = vk::SwapchainCreateInfoKHR {
        s_type: vk::StructureType::SWAPCHAIN_CREATE_INFO_KHR,
        p_next: ptr::null(),
        flags: vk::SwapchainCreateFlagsKHR::empty(),
        surface,
        min_image_count: image_count,
        image_color_space: surface_format.color_space,
        image_format: surface_format.format,
        image_extent: extent,
        image_usage: vk::ImageUsageFlags::COLOR_ATTACHMENT,
        image_sharing_mode,
        p_queue_family_indices: queue_family_indices.as_ptr(),
        queue_family_index_count,
        pre_transform: swapchain_support.capabilities.current_transform,
        composite_alpha: vk::CompositeAlphaFlagsKHR::OPAQUE,
        present_mode,
        clipped: vk::TRUE,
        old_swapchain: vk::SwapchainKHR::null(),
        image_array_layers: 1,
        _marker: PhantomData
    };

    // println!("Swap Chain Create Info: {:?}", swapchain_create_info);
    let swapchain_loader = ash::khr::swapchain::Device::new(instance, &device);
    // println!("Swapchain loader initialized: {:?}", swapchain_loader.fp_v1_0().create_swapchain_khr);

    let swapchain = unsafe {
        swapchain_loader
            .create_swapchain(&swapchain_create_info, None)
            .expect("Failed to create Swapchain!")
    };

    let swapchain_images = unsafe {
        swapchain_loader
            .get_swapchain_images(swapchain)
            .expect("Failed to create Swapchain Images!")
    };

    SwapChainStuff {
        swapchain_loader,
        swapchain,
        swapchain_format: surface_format.format,
        swapchain_extent: extent,
        swapchain_images
    }
}