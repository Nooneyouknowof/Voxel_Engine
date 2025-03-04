use std::{ffi::CString, fs::File, io::Read, path::Path};

use ash::vk;

pub fn create_image_views(device: &ash::Device, surface_format: vk::Format, images: &Vec<vk::Image>) -> Vec<vk::ImageView> {
    let mut swapchain_imageviews = vec![];

    for &image in images.iter() {
        let imageview_create_info = vk::ImageViewCreateInfo {
            s_type: vk::StructureType::IMAGE_VIEW_CREATE_INFO,
            flags: vk::ImageViewCreateFlags::empty(),
            view_type: vk::ImageViewType::TYPE_2D,
            format: surface_format,
            components: vk::ComponentMapping {
                r: vk::ComponentSwizzle::IDENTITY,
                g: vk::ComponentSwizzle::IDENTITY,
                b: vk::ComponentSwizzle::IDENTITY,
                a: vk::ComponentSwizzle::IDENTITY,
            },
            subresource_range: vk::ImageSubresourceRange {
                aspect_mask: vk::ImageAspectFlags::COLOR,
                base_mip_level: 0,
                level_count: 1,
                base_array_layer: 0,
                layer_count: 1,
            },
            image,
            ..Default::default()
        };

        let imageview = unsafe {
            device
                .create_image_view(&imageview_create_info, None)
                .expect("Failed to create Image View!")
        };
        swapchain_imageviews.push(imageview);
    }

    swapchain_imageviews
}

pub fn create_graphics_pipeline(device: &ash::Device) {
    let vert_shader_code = read_shader_code(Path::new("shaders/glsl.vert.spv"));
    let frag_shader_code = read_shader_code(Path::new("shaders/glsl.frag.spv"));

    let vert_shader_module = create_shader_module(device, vert_shader_code);
    let frag_shader_module = create_shader_module(device, frag_shader_code);

    let main_function_name = CString::new("main").unwrap();

    let _shader_stages = [
        vk::PipelineShaderStageCreateInfo {
            s_type: vk::StructureType::PIPELINE_SHADER_STAGE_CREATE_INFO,
            flags: vk::PipelineShaderStageCreateFlags::empty(),
            module: vert_shader_module,
            p_name: main_function_name.as_ptr(),
            stage: vk::ShaderStageFlags::VERTEX,
            ..Default::default()
        },
        vk::PipelineShaderStageCreateInfo {
            s_type: vk::StructureType::PIPELINE_SHADER_STAGE_CREATE_INFO,
            flags: vk::PipelineShaderStageCreateFlags::empty(),
            module: frag_shader_module,
            p_name: main_function_name.as_ptr(),
            stage: vk::ShaderStageFlags::FRAGMENT,
            ..Default::default()
        }
    ];

    unsafe {
        device.destroy_shader_module(vert_shader_module, None);
        device.destroy_shader_module(frag_shader_module, None);
    }
}

fn create_shader_module(device: &ash::Device, code: Vec<u8>) -> vk::ShaderModule {
    let shader_module_create_info = vk::ShaderModuleCreateInfo {
        s_type: vk::StructureType::SHADER_MODULE_CREATE_INFO,
        flags: vk::ShaderModuleCreateFlags::empty(),
        code_size: code.len(),
        p_code: code.as_ptr() as *const u32,
        ..Default::default()
    };

    unsafe {
        device
            .create_shader_module(&shader_module_create_info, None)
            .expect("Failed to create Shader Module!")
    }
}

fn read_shader_code(shader_path: &Path) -> Vec<u8> {
    let spv_file = File::open(shader_path)
    .expect(&format!("Failed to find spv file at {:?}", shader_path));
    let bytes_code: Vec<u8> = spv_file.bytes().filter_map(|byte| byte.ok()).collect();

    bytes_code
}