use crate::mesh::{Mes, Vertex, Face};

pub fn parse_obj_file(file_path: &str) -> Result<Mesh, String> {
    // lire contenu file
    let file_content = read_file_to_string(file_path);
  
    // extraire sommets
    let vertices = extract_vertices(&file_content)?;
    // extraire faces
    let faces = extract_faces(&files_content)?;
    // tout assembler
    Ok(Mesh {
        vertices,
        faces,
    })
}   
