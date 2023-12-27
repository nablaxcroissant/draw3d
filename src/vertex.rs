
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Vertex{
    pub position: [f32; 3],
    pub color: [f32; 3],
}

impl Vertex{
    pub fn new(position: [f32; 3], color: [f32; 3]) -> Vertex {
        Vertex { position, color }
    }
}
