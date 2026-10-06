use crate::mesh::{Mesh, Vertex, Face};
use std::fs;

fn is_valid_file(file_path: &str) -> Result<(), String> {

    if !file_path.ends_with(".obj") {
        return Err("Error: Invalid file format".to_string())
    }
    Ok(())
}

fn read_file_to_string(file_path: &str) -> Result<String, String> {

    is_valid_file(file_path)?;
    let content_file = fs::read_to_string(file_path).map_err(|e| format!("reading file error: {}", e))?;
    Ok(content_file)
}

fn extract_vertices(file_content: &str) -> Result<Vec<Vertex>, String> {

    let default_color = Color {r: 255, g: 255, b: 255, alpha: 255};
   let vertices: Vec<Vertex>, String = file_content
       .lines()
       .filter(|line| line.contains("v "))
       .map(|line| {
           let mut parts = line.split_whitespace();
           parts.next();
           let x_str = parts.next().ok_or("Error: Missing X")?;
           let x: f32 = x_str.parse().map_err(|_| "Error: X isn't a valid number")?; 

           let y_str = parts.next().ok_or("Error: Missing Y")?;
           let y: f32 = x_str.parse().map_err(|_| "Error: Y isn't a valid number")?; 
           
           let z_str = parts.next().ok_or("Error: Missing Z")?;
           let z: f32 = x_str.parse().map_err(|_| "Error: Z isn't a valid number")?; 


           Ok(Vertex {x, y, z, color: default_color.clone()})
       })
       .collect();

    vertices
}

pub fn parse_obj_file(file_path: &str) -> Result<Mesh, String> {

    let file_content = read_file_to_string(file_path)?;
    let vertices = extract_vertices(&file_content)?;
    let faces = extract_faces(&file_content)?;

    Ok(Mesh {
        vertices,
        faces,
    })
} 

