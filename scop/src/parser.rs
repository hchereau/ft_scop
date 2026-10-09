use crate::mesh::{Color, Face, Mesh, Vertex};
use std::fs;

fn is_valid_file(file_path: &str) -> Result<(), String> {
    if !file_path.ends_with(".obj") {
        return Err("Error: Invalid file format".to_string());
    }
    Ok(())
}

fn read_file_to_string(file_path: &str) -> Result<String, String> {
    is_valid_file(file_path)?;
    let content_file =
        fs::read_to_string(file_path).map_err(|e| format!("reading file error: {}", e))?;
    Ok(content_file)
}

///
fn parse_vertex(line: &str, default_color: &Color) -> Result<Vertex, String> {
    let mut parts = line.split_whitespace();
    parts.next();
    let mut parse_coord = |missing_err: &str, parse_err: &str| -> Result<f32, String> {
        parts
            .next()
            .ok_or_else(|| missing_err.to_string())?
            .parse::<f32>()
            .map_err(|_| parse_err.to_string())
    };

    Ok(Vertex {
        x: parse_coord("Error: Missing X", "Error: X isn't a valid number")?,
        y: parse_coord("Error: Missing Y", "Error: Y isn't a valid number")?,
        z: parse_coord("Error: Missing Z", "Error: Z isn't a valid number")?,
        color: default_color.clone(),
    })
}

fn extract_vertices(file_content: &str) -> Result<Vec<Vertex>, String> {
    let default_color = Color {
        r: 255,
        g: 255,
        b: 255,
        alpha: 255,
    };

    file_content
        .lines()
        .filter(|line| line.starts_with("v "))
        .map(|line| parse_vertex(line, &default_color))
        .collect()
}
///

///
fn parse_face(line: &str) -> Result<Vec<Face>, String> {
    let indices: Vec<u32> = line
        .split_whitespace()
        .skip(1)
        .map(|token| {
            let index = token
                .split('/')
                .next()
                .ok_or_else(|| "Error: Invalid number in faces".to_string())?
                .parse::<u32>()
                .map_err(|_| "Error: Invalid number in faces".to_string())?;

            if index == 0 {
                return Err("Error: face index 0 is invalid".to_string());
            }
            // L'OBJ commence à 1, Vulkan à 0
            Ok(index - 1)
        })
        .collect::<Result<Vec<u32>, String>>()?;

    if indices.len() < 3 {
        return Err("Error: Missing faces".to_string());
    }

    let faces = (1..indices.len() - 1)
        .map(|i| Face {
            v1: indices[0],
            v2: indices[i],
            v3: indices[i + 1],
        })
        .collect();

    Ok(faces)
}

fn extract_faces(file_content: &str) -> Result<Vec<Face>, String> {
    let mut final_faces = Vec::new();

    for line in file_content.lines().filter(|l| l.starts_with("f ")) {
        final_faces.extend(parse_face(line)?);
    }

    Ok(final_faces)
}
///

pub fn parse_obj_file(file_path: &str) -> Result<Mesh, String> {
    let file_content = read_file_to_string(file_path)?;
    let vertices = extract_vertices(&file_content)?;
    let faces = extract_faces(&file_content)?;

    Ok(Mesh { vertices, faces })
}

#[cfg(test)]
mod tests {
    use super::*;

    // is_valid_file TESTS

    #[test]
    fn test_valid_file_extension() {
        assert!(is_valid_file("mon_modele.obj").is_ok());
    }

    #[test]
    fn test_invalid_file_extension() {
        assert!(is_valid_file("mon_modele.txt").is_err());
    }

    // extract_vertices TESTS

    #[test]
    fn test_extract_vertices_valid() {
        let fake_file = "v 1.0 2.0 3.0\nv 0.5 0.5 0.5";
        let result = extract_vertices(fake_file);

        assert!(result.is_ok());
        let vertices = result.unwrap();
        assert_eq!(vertices.len(), 2);
        assert_eq!(vertices[0].x, 1.0);
    }

    #[test]
    fn test_extract_vertices_invalid() {
        let fake_file = "v 1.0 2.0 3.0\nv 0.5 0.5";
        let result = extract_vertices(fake_file);

        assert!(result.is_err());
    }

    // extract_faces TESTS

    #[test]
    fn test_extract_faces_valid() {
        let fake_file = "f 1/2/2 2/3/4 3/5/6\nf 1 3 4 2";
        let result = extract_faces(fake_file);

        assert!(result.is_ok());
        let faces = result.unwrap();
        assert_eq!(faces.len(), 3);
        assert_eq!(faces[0].v1, 1);
    }

    #[test]
    fn test_extract_faces_invalid() {
        let fake_file = "f 1 2 3\nf 1 1";
        let result = extract_faces(fake_file);

        assert!(result.is_err());
    }

    #[test]
    fn test_extract_faces_invalid_negative() {
        let fake_file = "f 1 2 3\nv 0 0 0";
        let result = extract_faces(fake_file);

        assert!(result.is_err());
    }
}
