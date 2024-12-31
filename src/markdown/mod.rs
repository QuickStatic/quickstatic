mod parser;
mod tokenizer;

pub use parser::AstNode;

pub fn parse(md: &str) -> Vec<AstNode> {
    let tokens = tokenizer::tokenize(md);
    parser::parse(tokens)
}