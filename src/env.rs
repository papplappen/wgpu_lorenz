use wgpu::{
    Backends, Device, Instance, InstanceDescriptor, Queue, Surface, SurfaceConfiguration,
    TextureUsages,
};
use winit::{
    dpi::PhysicalSize,
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

pub struct Environment {
    pub surface: Surface,
    pub device: Device,
    pub queue: Queue,
    pub config: SurfaceConfiguration,
    pub window: Window,
    pub cursor_grab: bool,
}

const WINDOW_SIZE: PhysicalSize<u32> = PhysicalSize {
    width: 1600,
    height: 900,
};

impl Environment {
    pub async fn new(event_loop: &EventLoop<()>) -> Self {
        // * CREATE CREATE WINDOW
        let window_builder = WindowBuilder::new().with_inner_size(WINDOW_SIZE);
        let window = window_builder.build(event_loop).unwrap();

        // * CREATE INSTANCE
        let instance = Instance::new(InstanceDescriptor {
            backends: Backends::VULKAN,
            ..Default::default()
        });

        // * CREATE SURFACE (unconfigured)
        let surface = unsafe { instance.create_surface(&window) }.unwrap();

        // * CREATE ADAPTER
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptionsBase {
                power_preference: wgpu::PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .unwrap();
        dbg!(adapter.limits());
        // * CREATE DEVICE & QUEUE
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default(), None)
            .await
            .unwrap();
        // * CONFIGURE SURFACE
        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .cloned()
            .unwrap_or(surface_caps.formats[0]);
        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: window.inner_size().width,
            height: window.inner_size().height,
            present_mode: wgpu::PresentMode::AutoNoVsync,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &config);

        Self {
            surface,
            device,
            queue,
            config,
            window,
            cursor_grab: false,
        }
    }
}
