use std::fs;
use crate::html_generator::from_markdown_ast;
use crate::markdown::{parse};

pub fn build(folder: String) {
    println!("Building folder {}", folder);

    let home_path = format!("{}/src/index.md", folder);
    let markdown = fs::read_to_string(home_path).unwrap();

    let ast = parse(&*markdown);
    println!("AST: {:?}", ast);

    let html = from_markdown_ast(ast);
    println!("HTML: {:?}", html);
}