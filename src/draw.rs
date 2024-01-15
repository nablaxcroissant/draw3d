use crate::{geometry::{Geometry, Draw}, app::App};

pub type Color = (f64, f64, f64);

pub struct DrawState {
    background_color: Color,
    geometry_list: Vec<Geometry>,
    instance_count: u32,
}

impl DrawState {
    pub fn new(background_color: Color) -> DrawState {
        let geometry_list: Vec<Geometry> = Vec::new();
        let instance_count = geometry_list.len() as u32;
        DrawState{
            background_color,
            geometry_list,
            instance_count,
        }
    }

    pub fn add_geometry(&mut self, geometry: Geometry) {
        self.geometry_list.push(geometry);
        self.instance_count = self.geometry_list.len() as u32;
    }

    pub fn add(&mut self, object: &dyn Draw, app: &App) {
        self.geometry_list.push(object.draw(app));
        self.instance_count = self.geometry_list.len() as u32;
    }

    pub fn update_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    pub fn background_color(&self) -> Color {
        self.background_color
    }

    pub fn geometry_list(&self) -> &Vec<Geometry> {
        &self.geometry_list
    }

    pub fn instance_count(&self) -> u32 {
        self.instance_count
    }
}