use futures::executor;
use winit::{
    application::ApplicationHandler,
    event::{ElementState, KeyEvent, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::Window,
};

#[derive(Debug)]
struct WebGpu<'a> {
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
}

#[derive(Default, Debug)]
struct App {
    window: Option<Window>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.window = Some(
            event_loop
            .create_window(Window::default_attributes())
            .unwrap(),
        );
        println!("resumed -> Created new window!!");
    }

    fn new_events(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        cause: winit::event::StartCause,
    ) {
        match cause {
            winit::event::StartCause::Init => {
                self.window = Some(
                    event_loop
                    .create_window(Window::default_attributes())
                    .unwrap(),
                );
                println!("new_events -> Created new window!!");
                executor::block_on(self.init_wgpu());
            }
            _ => (),
        }
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
impl App {
    pub async fn init_wgpu(&mut self) {
        let tmp = self.window.as_ref().unwrap();
        let size = tmp.inner_size();

        //To create a gpu instance we need to set some options for it
        //Like here we use deafult backend and no special flags ( flags are for validation )
        let instance_descriptor = wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        };
        // Lets create a new instance to the wgpu struct, mainly used Adapter (which card to use)
        // and surface (handle to the window to write to)
        let instance = wgpu::Instance::new(instance_descriptor);
        let surface = instance
            .create_surface(self.window.as_ref().unwrap())
            .unwrap();
        let adapter_descriptor = wgpu::RequestAdapterOptionsBase {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        };
        let adapter = instance.request_adapter(&adapter_descriptor)
            .await.unwrap();

        let device_descriptor = wgpu::DeviceDescriptor {
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::default(),
            memory_hints: wgpu::MemoryHints::Performance,
            label: Some("Device"),
        };
        let (device, queue) = adapter
            .request_device(&device_descriptor, None)
            .await.unwrap();


        let surface_capabilities = surface.get_capabilities(&adapter);
        let surface_format = surface_capabilities
            .formats
            .iter()
            .copied()
            .filter(|f | f.is_srgb())
            .next()
            .unwrap_or(surface_capabilities.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_capabilities.present_modes[0],
            alpha_mode: surface_capabilities.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2
        };
        surface.configure(&device, &config);
    }
}

impl<'a> WebGpu<'a> {
  async fn new(window: &'a Window) -> WebGpu<'a> {
        let size = window.inner_size();

        //To create a gpu instance we need to set some options for it
        //Like here we use deafult backend and no special flags ( flags are for validation )
        let instance_descriptor = wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        };
        // Lets create a new instance to the wgpu struct, mainly used Adapter (which card to use)
        // and surface (handle to the window to write to)
        let instance = wgpu::Instance::new(instance_descriptor);
        // Connect the surface to a window handle, the surface will be used to paint on later.
        let surface = instance
            .create_surface(window)
            .unwrap();
        // fill in a adapter descriptor connected to the surface so that we can get a gfx card to
        // use.
        let adapter_descriptor = wgpu::RequestAdapterOptionsBase {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        };
        // Get the adapter that fufills the descriptor if possible.
        let adapter = instance.request_adapter(&adapter_descriptor)
            .await.unwrap();
        // Device descriptor needs parameters filled in so it can be requested.
        let device_descriptor = wgpu::DeviceDescriptor {
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::default(),
            memory_hints: wgpu::MemoryHints::Performance,
            label: Some("Device"),
        };
        // Get a device== gpu and a command queue.
        let (device, queue) = adapter
            .request_device(&device_descriptor, None)
            .await.unwrap();


        let surface_capabilities = surface.get_capabilities(&adapter);
        // Find a surface with the capabilites we need, in this case srgb is the only req we have.
        let surface_format = surface_capabilities
            .formats
            .iter()
            .copied()
            .filter(|f | f.is_srgb())
            .next()
            .unwrap_or(surface_capabilities.formats[0]);
        // create a config for our surface.
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_capabilities.present_modes[0],
            alpha_mode: surface_capabilities.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2
        };
        surface.configure(&device, &config);

    Self {
        surface,
        device,
        queue,
        config,
        size,
    }
  }
}

fn main() {
    if cfg!(debug_assertions) {
        println!("Debugging enabled");
    } else {
        println!("Debugging disabled");
    }

    let event_loop = EventLoop::new().expect("Couldnt create event loop");
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::default();
    event_loop.run_app(&mut app).unwrap();
}
