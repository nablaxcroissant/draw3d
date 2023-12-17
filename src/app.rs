use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{WindowBuilder, Window},
};
#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;
use std::cell::RefCell;

pub enum Target{
    Wasm,
    Local,
}

pub struct AppBuilder{
    target: Target,
}

impl AppBuilder{
    pub fn new(target: Target) -> AppBuilder{
        AppBuilder{
            target,
        }
    }

    pub fn run(self){
        let mut app = App::new();
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
        run_loop(app);
    }
}

struct App {
    event_loop: EventLoop<()>,
    window: Window,
}

impl App {
    fn new() -> App {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().build(&event_loop).unwrap();
        App {
            event_loop,
            window,
        }
    }
}

fn run_loop(app: App) {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Warn).expect("Couldn't initialize logger");
        } else {
            env_logger::init();
        }
    }

    app.event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == app.window.id() => match event {
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
            _ => {}
        },
        _ => {}
    })
}