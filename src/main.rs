mod window;
mod vulkan;

use winit::event_loop::{ControlFlow, EventLoop};
use window::*;
use vulkan::surface::VulkanApp;
use vulkan::device::VulkanPhysicalDevice;


fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::default();
    event_loop.run_app(&mut app).expect("Run App failed!");
    let vulkan_app = VulkanApp::new(app.window.as_ref().unwrap());
    
    let instance = vulkan_app.instance.clone();
    let physical_device = VulkanPhysicalDevice::pick_physical_device(&instance);
    println!("Selected Physical Device: {:?}", physical_device.physical_device);
    
}
