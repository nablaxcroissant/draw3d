use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{WindowBuilder, Window}, dpi::PhysicalSize,
};
use crate::{draw::DrawState, vertex::Vertex, geometry::{GeometryType, Geometry}};
#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;

pub type ModelFn<Model> = fn(&App) -> Model;

pub type ViewFn<Model> = fn(&mut App, &Model);

pub type UpdateFn<Model> = fn(&App, &mut Model);

pub struct AppBuilder<M = ()> {
    model: ModelFn<M>,
    update: Option<UpdateFn<M>>,
    view: Option<ViewFn<M>>,
    window_size: Option<winit::dpi::PhysicalSize<u32>>,
}

impl AppBuilder{
    pub fn new() -> AppBuilder<()>{
        fn model(_: &App){}
        AppBuilder{
            model,
            update: None,
            view: None,
            window_size: None,
        }
    }

    pub fn window_size(&mut self, x: u32, y: u32) {
        self.window_size = Some(PhysicalSize::new(x, y));
    }

}

impl<M> AppBuilder<M> where M: 'static {
    pub fn run(self){
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_inner_size(self.window_size.unwrap_or(PhysicalSize::new(800, 800)))
            .build(&event_loop).unwrap();
        let app = App::new(window);

        let model = (self.model)(&app);
        #[cfg(target_arch = "wasm32")]
        {
            // Winit prevents sizing with CSS, so we have to set
            // the size manually when on web.
            use winit::dpi::PhysicalSize;
            app.window.set_inner_size(PhysicalSize::new(450, 400));
            
            use winit::platform::web::WindowExtWebSys;
            web_sys::window()
                .and_then(|win| win.document())
                .and_then(|doc| {
                    let dst = doc.get_element_by_id("wasm-example")?;
                    let canvas = web_sys::Element::from(app.window.canvas());
                    dst.append_child(&canvas).ok()?;
                    Some(())
                })
                .expect("Couldn't append canvas to document body.");
        }
        run_loop(app, event_loop, model, self.view, self.update);
    }

    pub fn app(model: ModelFn<M>) -> AppBuilder<M>{
        AppBuilder{
            model,
            update: None,
            view: None,
            window_size: None,
        }
    }

    pub fn update(mut self, u: UpdateFn<M>) -> AppBuilder<M>{
        self.update = Some(u);
        self
    }

    pub fn view(mut self, v: ViewFn<M>) -> AppBuilder<M>{
        self.view = Some(v);
        self
    }
}

pub struct App {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    triangle_render_pipeline: wgpu::RenderPipeline,
    line_render_pipeline: wgpu::RenderPipeline,
    window: Window,
    draw_state: DrawState,
}

impl App{
    fn new(window: Window) -> App{
        let size = window.inner_size();

        // The instance is a handle to our GPU
        // Backends::all => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        let surface = unsafe { instance.create_surface(&window) }.unwrap();

        let adapter = pollster::block_on(instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            },
        )).unwrap();

        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::empty(),
                // WebGL doesn't support all of wgpu's features, so if
                // we're building for the web, we'll have to disable some.
                limits: if cfg!(target_arch = "wasm32") {
                    wgpu::Limits::downlevel_webgl2_defaults()
                } else {
                    wgpu::Limits::default()
                },
                label: None,
            },
            None, // Trace path
        )).unwrap();
        

        let surface_caps = surface.get_capabilities(&adapter);
        // Shader code in this tutorial assumes an sRGB surface texture. Using a different
        // one will result in all the colors coming out darker. If you want to support non
        // sRGB surfaces, you'll need to account for that when drawing to the frame.
        let surface_format = surface_caps.formats.iter()
            .copied()
            .filter(|f| f.is_srgb())
            .next()
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &config);

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });  

        let render_pipeline_layout =
        device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],

        });

        let triangle_render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Triangle Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main", // 1.
                buffers: &[Vertex::desc()], // 2.
            },
            fragment: Some(wgpu::FragmentState { // 3.
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState { // 4.
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList, // 1.
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw, // 2.
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: None, // 1.
            multisample: wgpu::MultisampleState {
                count: 1, // 2.
                mask: !0, // 3.
                alpha_to_coverage_enabled: false, // 4.
            },
            multiview: None, // 5.
        });

        let line_render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Line Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main", // 1.
                buffers: &[Vertex::desc()], // 2.
            },
            fragment: Some(wgpu::FragmentState { // 3.
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState { // 4.
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState{
                topology: wgpu::PrimitiveTopology::LineList,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        
        let draw_state = DrawState::new((0.1, 0.2, 0.3));

        App {
            window,
            surface,
            device,
            queue,
            config,
            size,
            draw_state,
            triangle_render_pipeline,
            line_render_pipeline,
        }
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn device(&self) -> &wgpu::Device {
        &self.device
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    fn input(&mut self, _event: &WindowEvent) -> bool {
        false
    }

    fn update(&mut self) {}

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let (r, g, b) = self.draw_state.background_color();

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r,
                            g,
                            b,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            let geometry_list = self.draw_state.geometry_list();
            // println!("{:?}", self.draw_state.instance_count());

            for geometry in geometry_list.iter() {
                render_pass.set_pipeline(match geometry.geometry_type() {
                    GeometryType::Line => &self.line_render_pipeline,
                    GeometryType::Mesh => &self.triangle_render_pipeline,
                    
                });
                render_pass.set_vertex_buffer(0, geometry.vertex_buffer().slice(..));
                render_pass.set_index_buffer(geometry.index_buffer().slice(..), wgpu::IndexFormat::Uint16);
                render_pass.draw_indexed(0..geometry.num_indices(), 0, 0..self.draw_state.instance_count());
            }
            // render_pass.set_pipeline(&self.line_render_pipeline); // 2.
            
            
        }
    
        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    
        Ok(())
    }

    pub fn draw_state(&mut self) -> &mut DrawState{
        &mut self.draw_state
    }

    pub fn draw(&self) -> DrawState{
        let background_color = self.draw_state.background_color();
        DrawState::new(background_color)
    }

    pub fn draw_to_frame(&mut self, draw: DrawState) {
        self.draw_state = draw;
    }
}

fn run_loop<M>(
    mut app: App,
    event_loop: EventLoop<()>,
    mut model: M,
    view: Option<ViewFn<M>>,
    update: Option<UpdateFn<M>>,
) where
    M: 'static,
{
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Warn).expect("Couldn't initialize logger");
        } else {
            env_logger::init();
        }
    }

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == app.window.id() => if !app.input(event) {
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
                    app.resize(*physical_size);
                },
                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    // new_inner_size is &&mut so we have to dereference it twice
                    app.resize(**new_inner_size);
                },
                // WindowEvent::CursorMoved { device_id: _, position, modifiers: _ } => {
                //    app.draw_state.update_background_color((position.x/app.size.width as f64, 0., position.y/app.size.height as f64));
                //}
                _ => {},
            }
        },
        Event::RedrawRequested(window_id) if window_id == app.window().id() => {
            if let Some(update) = update{
                update(&app, &mut model)
            }
            if let Some(view) = view{
                view(&mut app, &model)
            };
            match app.render() {
                Ok(_) => {}
                // Reconfigure the surface if lost
                Err(wgpu::SurfaceError::Lost) => app.resize(app.size),
                // The system is out of memory, we should probably quit
                Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                // All other errors (Outdated, Timeout) should be resolved by the next frame
                Err(e) => eprintln!("{:?}", e),
            }
        },
        Event::MainEventsCleared => {
            // RedrawRequested will only trigger once unless we manually
            // request it.
            app.window().request_redraw();
        },
        _ => {}
    })
}