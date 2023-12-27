use crate::vertex::Vertex;
use crate::app::App;
use wgpu::util::DeviceExt;

pub type Color = (f64, f64, f64);

pub struct DrawState {
    background_color: Color,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
}

impl DrawState {
    pub fn new(background_color: Color, vertex_buffer: wgpu::Buffer, index_buffer: wgpu::Buffer, num_indices: u32) -> DrawState {
        DrawState{
            background_color,
            vertex_buffer,
            index_buffer,
            num_indices,
        }
    }

    pub fn background_color(&self) -> Color {
        self.background_color
    }

    pub fn vertex_buffer(&self) -> &wgpu::Buffer {
        &self.vertex_buffer
    }

    pub fn index_buffer(&self) -> &wgpu::Buffer {
        &self.index_buffer
    }

    pub fn num_indices(&self) -> u32 {
        self.num_indices
    }

    pub fn update_background_color(&mut self, color: Color) {
        self.background_color = color;
    }
}