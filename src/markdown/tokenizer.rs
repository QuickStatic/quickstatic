#[derive(Debug)]
pub enum Token {
    Heading { level: usize, text: String },
    Text(String),
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }

        if let Some((level, text)) = heading(line) {
            tokens.push(Token::Heading { level, text });
        } else {
            tokens.push(Token::Text(line.to_string()));
        }
    }
    tokens
}

fn heading(line: &str) -> Option<(usize, String)> {
    let mut level = 0;
    let mut chars = line.chars();
    while let Some('#') = chars.next() {
        level += 1;
    }
    if level > 0 {
        Some((level, chars.collect::<String>().trim().to_string()))
    } else {
        None
    }
}
