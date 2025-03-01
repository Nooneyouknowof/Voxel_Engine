mod window;
mod vulkan;

use winit::event_loop::EventLoop;
use window::*;

fn main() {
    let event_loop = EventLoop::new().unwrap();
    
    let mut events = AppEvents::default();
    event_loop.run_app(&mut events).expect("Run App failed!");
}
