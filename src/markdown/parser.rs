use crate::markdown::tokenizer::Token;

#[derive(Debug)]
pub enum AstNode {
    Heading { level: usize, text: String },
    Text(String),
}

pub fn parse(tokens: Vec<Token>) -> Vec<AstNode> {
    let mut ast = Vec::new();
    for token in tokens {
        match token {
            Token::Heading { level, text } => ast.push(AstNode::Heading { level, text }),
            Token::Text(text) => ast.push(AstNode::Text(text)),
        }
    }
    ast
}

