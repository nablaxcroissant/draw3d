use wgpu::util::DeviceExt;

use crate::vertex::Vertex;
use crate::app::App;

pub enum GeometryType {
    Line,
    Mesh,
}

pub struct Geometry{
    geometry_type: GeometryType,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
}

pub trait Draw{
    fn draw(&self, app: &App) -> Geometry;
}

impl Geometry{
    pub fn new_mesh(
        app: &App,
        vertices: &Vec<Vertex>,
        indices: &[u16],
    ) -> Geometry {
        let (vertex_buffer, index_buffer, num_indices) = Geometry::buffers_from_slice(app, vertices, indices);

        Geometry {
            geometry_type: GeometryType::Mesh,
            vertex_buffer,
            index_buffer,
            num_indices,
        }
    }

    pub fn geometry_type(&self) -> &GeometryType{
        &self.geometry_type
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

    pub fn new_line(
        app: &App,
        vertices: &[Vertex],
        indices: &[u16],
    ) -> Geometry {
        let (vertex_buffer, index_buffer, num_indices) = Geometry::buffers_from_slice(app, vertices, indices);

        Geometry {
            geometry_type: GeometryType::Line,
            vertex_buffer,
            index_buffer,
            num_indices,
        }
    }

    fn buffers_from_slice(app: &App, vertices: &[Vertex], indices: &[u16]) -> (wgpu::Buffer, wgpu::Buffer, u32) {
        let vertex_buffer = app.device().create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(vertices),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );

        let num_indices = indices.len() as u32;

        let index_buffer = app.device().create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(indices),
                usage: wgpu::BufferUsages::INDEX,
            }
        );
        return (vertex_buffer, index_buffer, num_indices);
    }
}

struct Polygon{
    vertices: Vec<Vertex>,
}

impl Polygon{
    fn new(vertices: Vec<Vertex>) -> Polygon {
       Polygon { vertices } 
    }
}

impl Draw for Polygon{
    fn draw(&self, app: &App) -> Geometry {
        let n = self.vertices.len() as u16;
        let mut indices = Vec::new();
        for i in 1..(n-1) {
            indices.push(0);
            indices.push(i);
            indices.push(i+1)
        }
        Geometry::new_mesh(app, &self.vertices, &indices)
    }
}
