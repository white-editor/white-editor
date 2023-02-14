use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
};

use crate::window::Window;

pub struct Application {
    wgpu_instance: wgpu::Instance,
}

impl Application {
    pub fn new() -> Self {
        Application {
            wgpu_instance: wgpu::Instance::new(wgpu::InstanceDescriptor {
                backends: wgpu::Backends::all(),
                dx12_shader_compiler: Default::default(),
            }),
        }
    }

    pub async fn run(&self) {
        env_logger::init();

        let event_loop = EventLoop::new();
        let mut window = Window::new(&self.wgpu_instance, &event_loop).await;
        // let ta = TextArea::new(&window).await;

        event_loop.run(move |event, _, control_flow| match event {
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                window.update();
                match window.render() {
                    Ok(_) => {}
                    // Reconfigure the surface if lost
                    Err(wgpu::SurfaceError::Lost) => window.resize(window.size),
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    // All other errors (Outdated, Timeout) should be resolved by the next frame
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                window.update();

                if !window.input(event) {
                    match event {
                        WindowEvent::CloseRequested
                        | WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    state: ElementState::Pressed,
                                    virtual_keycode: Some(VirtualKeyCode::Escape),
                                    ..
                                },
                            ..
                        } => *control_flow = ControlFlow::Exit,
                        WindowEvent::Resized(physical_size) => {
                            window.resize(*physical_size);
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            // new_inner_size is &&mut so we have to dereference it twice
                            window.resize(**new_inner_size);
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        });
    }
}
