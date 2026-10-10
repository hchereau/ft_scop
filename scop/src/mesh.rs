#[derive(Debug)]
#[repr(C)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub color: Color,
}
#[derive(Debug)]
#[repr(C)]
pub struct Face {
    pub v1: u32,
    pub v2: u32,
    pub v3: u32,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub alpha: u8,
}

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub faces: Vec<Face>,
}

#[allow(dead_code)]
impl Mesh {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            faces: Vec::new(),
        }
    }
}
