// #![allow(dead_code)]

pub mod app;
pub mod draw;
pub mod vertex;
pub mod geometry;

use app::AppBuilder;

#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;

pub fn app<M>(model: app::ModelFn<M>) -> app::AppBuilder<M> where M: 'static{
    AppBuilder::app(model)
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen(start))]
pub async fn run_example() {
    use app::App;
    use geometry::ParametricSurface;
    use std::f32::consts::PI;

    struct Model{
        surface: ParametricSurface,
    }

    fn model(_app: &App) -> Model{
        // let vertices= vec![
        // Vertex { position: [-0.0868241, 0.49240386, 0.0], color: [0.5, 0.0, 0.5] }, // A
        // Vertex { position: [-0.49513406, 0.06958647, 0.0], color: [0.5, 0.0, 0.5] }, // B
        // Vertex { position: [-0.21918549, -0.44939706, 0.0], color: [0.5, 0.0, 0.5] }, // C
        // Vertex { position: [0.35966998, -0.3473291, 0.0], color: [0.5, 0.0, 0.5] }, // D
        // Vertex { position: [0.44147372, 0.2347359, 0.0], color: [0.5, 0.0, 0.5] }, // E
        // ];

        fn sphere(theta: f32, phi: f32) -> [f32; 3] {
            let r = 0.5;
            let x = r*theta.cos()*phi.sin();
            let y = r*theta.sin()*phi.sin();
            let z = r*phi.cos();
            [x, y, z]
        }

        let r1 = (0..=20).map(|n| 2.*PI*n as f32/20.).collect();
        let r2 = (0..=10).map(|n| PI*n as f32/10.-PI/2.).collect();

        let sphere = ParametricSurface::new(sphere, r1, r2);

        Model { surface: sphere }
    }

    fn view(app: &mut App, model: &Model){
        let mut draw = app.draw();
        //draw.update_background_color((0.1, 0.2, 0.3));
        draw.add(&model.surface, app);

        app.draw_to_frame(draw)
    }
    

    #[cfg(not(target_arch="wasm32"))]
    let _run_example = pollster::block_on(app(model)
        .view(view)
        .run());

    #[cfg(target_arch="wasm32")]
    let _run_example = app(model)
        .view(view)
        .run();

}
