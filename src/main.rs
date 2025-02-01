mod lexer;
mod parser;
mod evaluator;
mod token;
mod ast;

use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("usage: m <file-name.m>");
        return;
    }

    let filename = &args[1];
    // Dosya uzantısını kontrol et
    if Path::new(filename)
        .extension()
        .and_then(|ext| ext.to_str())
        != Some("m")
    {
        eprintln!("Error: file extension is not 'm'!");
        return;
    }

    let code = fs::read_to_string(filename).expect("file cannot be read!");

    let tokens = lexer::lex(&code);
    let ast = parser::parse(&tokens);
    let result = evaluator::evaluate(&ast);

    println!("res: {}", result);
}
