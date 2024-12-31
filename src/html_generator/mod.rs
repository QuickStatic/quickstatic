use std::fs;
use std::path::Path;
use crate::markdown::AstNode;

pub fn from_markdown_ast(ast: Vec<AstNode>) -> String {
    // Get the current file's path
    let current_file = file!();

    // Construct a relative path to the desired file
    let current_dir = Path::new(current_file).parent().unwrap();
    let layout_path = current_dir.join("./templates/layout.html");

    let layout = fs::read_to_string(layout_path).unwrap();
    layout
}