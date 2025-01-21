use std::io::Read;

use lng::lexer;

fn get_args() -> Vec<String> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }
    args
}

fn read_file(filename: &str) -> Vec<u8> {
    let mut file = std::fs::File::open(filename).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    buffer
}

fn main() {
    //
    // to read from stdin
    //
    // let mut stdin = io::stdin();
    // let mut buffer: Vec<u8> = Vec::new();
    // stdin.read_to_end(buffer.as_mut()).unwrap();

    //
    // to read from filename
    //
    let args = get_args();
    let buffer = read_file(&args[1]);
    let chars: Vec<char> = buffer.iter().map(|&x| x as char).collect();
    let tokens = lexer::get_tokens(&chars);

    dbg!(&tokens);
}
