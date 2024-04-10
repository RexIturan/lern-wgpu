use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder
};

pub fn run() {
    //logger important! else no logs from wgpu
    env_logger::init();

    let event_loop = EventLoop::new();
}
