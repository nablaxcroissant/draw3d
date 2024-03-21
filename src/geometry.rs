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

//TODO: This is confusing to have in the geometry module and should be moved to the geometry module next commit.
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

pub struct Polygon{
    vertices: Vec<Vertex>,
}

impl Polygon{
    pub fn new(vertices: Vec<Vertex>) -> Polygon {
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

pub type Parametric = fn(a: f32, b: f32) -> [f32; 3];

pub struct ParametricSurface {
    p: Parametric,
    r1: Vec<f32>,
    r2: Vec<f32>,
}

impl ParametricSurface{
    pub fn new(p: Parametric, r1: Vec<f32>, r2: Vec<f32>) -> ParametricSurface {
        ParametricSurface {p, r1, r2}
    }
}

impl Draw for ParametricSurface{
    fn draw(&self, app: &App) -> Geometry {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        let l1 = self.r1.len() as u16;
        let l2 = self.r2.len() as u16;

        for &s1 in self.r1.iter() {
            for &s2 in self.r2.iter() {
                //println!("sphere({s1}, {s2}");
                let new_vertex = Vertex::new((self.p)(s1, s2), [0.6, 0., 0.6]);
                vertices.push(new_vertex);
            }
        }
        for i in 0..l1*l2-l2 {
            let i = i as u16;
            indices.push(i);
            indices.push(i+l2+1);
            indices.push(i+1);
            indices.push(i);
            indices.push(i+l2);
            indices.push(i+l2+1);
    }

        //print!("{:?}", vertices);
        Geometry::new_mesh(app, &vertices, &indices)
    }
}

pub struct PolyLine {
    vertices: Vec<Vertex>,
    indices: Vec<u16>,
}

impl PolyLine {
    pub fn new(vertices: Vec<Vertex>) -> PolyLine {
        let mut indices = Vec::new();
        vertices.iter()
            .enumerate()
            .for_each(|(i, _)| {
                indices.push(i as u16);
                indices.push((i+1) as u16);
            });
        PolyLine {vertices, indices}
    }

    pub fn push(&mut self, vertex: Vertex) {
        // let index = *self.indices.get(self.indices.len()-1).unwrap();
        let index = self.vertices.len() as u16;
        self.vertices.push(vertex);
        self.indices.push(index-1);
        self.indices.push(index);
    }

    pub fn len(&self) -> usize {
        self.vertices.len()
    }
}

impl Draw for PolyLine {
    fn draw(&self, app: &App) -> Geometry {
        
        Geometry::new_line(app, &self.vertices, &self.indices)
    }
}