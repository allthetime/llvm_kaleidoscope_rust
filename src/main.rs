use std::{
    io::{self, Read, Write},
    str::from_utf8,
};

use lng::lexer;

fn main() {
    let mut stdin = io::stdin();
    let mut buffer: Vec<u8> = Vec::new();
    stdin.read_to_end(buffer.as_mut()).unwrap();
    let chars: Vec<char> = buffer.iter().map(|&x| x as char).collect();

    let tokens = lexer::get_tokens(&chars);

    dbg!(&tokens);
}
