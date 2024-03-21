use draw3d::app::App;
use draw3d::geometry::ParametricSurface;
use std::f32::consts::PI;

fn main(){
    draw3d::app(model)
        .view(view)
        .run();
}

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