#[derive(Debug, PartialEq)]
pub enum Token {
    Eof,
    Def,
    Extern,
    Identifier(String),
    Number(f64),
    Unknown(char),
}

pub fn get_tokens(input: &[char]) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut chars = input.iter().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            // starts with alphabetic character
            'a'..='z' | 'A'..='Z' => {
                let mut identifier = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_alphanumeric() {
                        identifier.push(*c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                if identifier == "def" {
                    tokens.push(Token::Def);
                } else if identifier == "extern" {
                    tokens.push(Token::Extern);
                } else {
                    tokens.push(Token::Identifier(identifier));
                }
            }
            // starts with numeric character
            '0'..='9' | '.' => {
                let mut number = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_numeric() || *c == '.' {
                        number.push(*c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Number(number.parse().unwrap()));
            }
            // handle comments until newline
            '#' => {
                while let Some(&c) = chars.peek() {
                    if *c != '\n' && *c != '\r' {
                        chars.next();
                    } else {
                        break;
                    }
                }
            }
            // ignore whitespace and new lines
            ' ' | '\n' | '\r' => {
                chars.next();
            }
            // handle unknown characters
            _ => {
                tokens.push(Token::Unknown(*c));
                chars.next();
            }
        }
    }
    tokens.push(Token::Eof);
    tokens
}
