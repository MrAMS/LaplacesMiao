use crate::token::Token;

pub trait Lexer {
    fn make_tokens(input: &str) -> Result<Vec<Token>, String>;
}
