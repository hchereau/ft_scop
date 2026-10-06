use crate::mesh::{Mesh, Vertex, Face, Color};
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
    let vertices: Result<Vec<Vertex>, String> = file_content
       .lines()
       .filter(|line| line.starts_with("v "))
       .map(|line| {
           let mut parts = line.split_whitespace();
           parts.next();
           let x_str = parts.next().ok_or("Error: Missing X")?;
           let x: f32 = x_str.parse().map_err(|_| "Error: X isn't a valid number")?; 

           let y_str = parts.next().ok_or("Error: Missing Y")?;
           let y: f32 = y_str.parse().map_err(|_| "Error: Y isn't a valid number")?; 
           
           let z_str = parts.next().ok_or("Error: Missing Z")?;
           let z: f32 = z_str.parse().map_err(|_| "Error: Z isn't a valid number")?; 


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

#[cfg(test)]
mod tests {
    use super::*; 

    #[test]
    fn test_valid_file_extension() {
        assert!(is_valid_file("mon_modele.obj").is_ok());
    }

    #[test]
    fn test_invalid_file_extension() {
        assert!(is_valid_file("mon_modele.txt").is_err());
    }

    #[test]
    fn test_extract_vertices_valid() {
        let fake_file = "v 1.0 2.0 3.0\nv 0.5 0.5 0.5";
        let result = extract_vertices(fake_file);
        
        assert!(result.is_ok());
        let vertices = result.unwrap(); 
        assert_eq!(vertices.len(), 2);
        assert_eq!(vertices[0].x, 1.0);
    }
}

