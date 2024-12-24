use winit::{
    application::ApplicationHandler,
    event::{ElementState, KeyEvent, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::Window,
};

struct App {
    window: Option<Window>,
}

impl Default for App {
    fn default() -> Self {
        Self { window: Default::default() }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.window =
            Some(event_loop
                .create_window(Window::default_attributes())
                .unwrap());
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(KeyCode::Escape),
                        state: ElementState::Pressed,
                        repeat: false,
                        ..
                    },
                ..
            } => {
                println!("Escape key pressed, quitting application");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                // can render here instead.
                self.window.as_ref().unwrap().request_redraw();
            }
            //_ => println!("Default case, x = {event:?}"),
            _ => (),
        }
    }
}
/* impl<'a> App<'a> {
    pub fn new(window: & mut Window) -> Self {
        let size = window.inner_size();

        let instance_descriptor = wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        };
        let instance = wgpu::Instance::new(instance_descriptor);
        let surface = instance.create_surface(window).unwrap();
        Self{
            window: Some(window),
        }
    }
   
} */
fn main() {
    let event_loop = EventLoop::new().expect("Couldnt create event loop");
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::default();
    event_loop.run_app(&mut app).unwrap();
}
