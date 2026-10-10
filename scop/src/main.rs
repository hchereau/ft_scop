use crate::parser::parse_obj_file;

mod mesh;
mod parser;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <model.obj>", args[0]);
        std::process::exit(1);
    }
    match parse_obj_file(&args[1]) {
        Ok(mesh) => println!(
            "Mesh chargé avec succès : {} sommets, {} faces",
            mesh.vertices.len(),
            mesh.faces.len()
        ),
        Err(err) => {
            eprintln!("Error: {err}");
            std::process::exit(1);
        }
    }
}
