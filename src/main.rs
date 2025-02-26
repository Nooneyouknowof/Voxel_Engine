mod window;
mod vulkan;

use winit::event_loop::{ControlFlow, EventLoop};
use window::*;
use vulkan::surface::VulkanApp;

fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::default();
    event_loop.run_app(&mut app).expect("Run App failed!");
    let _vulkan_app = VulkanApp::new(app.window.as_ref().unwrap());
}
