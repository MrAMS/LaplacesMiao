use lexer::LasmiaoLexer;
use lexer::Lexer;
use parser::TokenParser;
use parser::traits::Parser;
use std::io::{self, Write};

fn main() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        input.clear();
        io::stdin().read_line(&mut input).expect("input error");

        match input.trim() {
            "exit" => {
                println!("Bye");
                break;
            }
            "help" => {
                println!("exit - exit the loop");
            }
            _ => match LasmiaoLexer::make_tokens(&input) {
                Ok(v) => {
                    println!("Tokens: {:?}", v);
                    let mut parser = TokenParser::new(v);
                    match parser.parse_exprs() {
                        Ok(expr) => println!("AST:\n {}", expr),
                        Err(e) => println!("Parser Error:\n  {}", e),
                    }
                }
                Err(e) => println!("Lexer Error:\n  {}", e),
            },
        }
    }
}
