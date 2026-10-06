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

pub fn parse_obj_file(file_path: &str) -> Result<Mesh, String> {

    let file_content = read_file_to_string(file_path)?;
    let vertices = extract_vertices(&file_content)?;
    let faces = extract_faces(&file_content)?;

    Ok(Mesh {
        vertices,
        faces,
    })
} 

