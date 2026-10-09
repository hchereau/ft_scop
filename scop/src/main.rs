use crate::parser::parse_obj_file;

mod mesh;
mod parser;

fn main() {
    let my_mesh = parse_obj_file("../../maps/resources/42.obj");
}
