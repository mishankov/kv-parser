mod lexer;
mod parser;
mod tokens;

use std::fs;

use crate::{lexer::Lexer, parser::Parser};

fn main() {
    let file_content = match fs::read_to_string(".env.example") {
        Ok(content) => content,
        Err(_) => todo!(),
    };

    println!("File content: {file_content}");

    let lexer = Lexer::new(file_content);
    println!("{:?}", lexer);

    let tokens = lexer.get_tokens();
    println!("{:?}", tokens);

    let parser = Parser { tokens };
    println!("{:?}", parser.parse())
}
