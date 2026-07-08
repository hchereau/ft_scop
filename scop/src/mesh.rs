pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub struct Face {
    pub v1: u32,
    pub v2: u32,
    pub v3: u32,
}

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub faces: Vec<Face>,
}

impl Mesh {
    pub fn new() -> Self {
        Self {
        vertices: Vec::new(),
        faces: Vec::new(),
        }
    }
}
