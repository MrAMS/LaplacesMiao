use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(f64),
    Symbol(String),
    Plus,        // +
    Minus,       // -
    Star,        // *
    Slash,       // /
    DoubleSlash, // //
    Mod,         // %
    LParen,      // (
    RParen,      // )
    LBracket,    // [
    RBracket,    // ]
    LBrace,      // {
    RBrace,      // }
    Semicolon,   // ;
    Comma,       //,
    Dot,         // .
    Equal,       // =

    DoubleEqual, // ==
    NotEqual,    // !=
    LessThan,    // <
    LessThanEq,  // <=
    GreatThan,   // >
    GreatThanEq, // >=

    LogicAnd, // &&
    LogicOr,  // ||

    And, // &
    Or,  // |
    Not, // !
    Xor, // ^

    FatArrow, // =>

    EOF,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Token::Number(n) => n.to_string(),
                Token::Symbol(s) => s.to_string(),
                Token::Plus => "+".to_string(),
                Token::Minus => "-".to_string(),
                Token::Star => "*".to_string(),
                Token::Slash => "/".to_string(),
                Token::DoubleSlash => "//".to_string(),
                Token::Mod => "%".to_string(),
                Token::LParen => "(".to_string(),
                Token::RParen => ")".to_string(),
                Token::LBracket => "[".to_string(),
                Token::RBracket => "]".to_string(),
                Token::LBrace => "{".to_string(),
                Token::RBrace => "}".to_string(),
                Token::Semicolon => ";".to_string(),
                Token::Comma => ",".to_string(),
                Token::Dot => ".".to_string(),
                Token::Equal => "=".to_string(),

                Token::DoubleEqual => "==".to_string(),
                Token::NotEqual => "!=".to_string(),
                Token::LessThan => "<".to_string(),
                Token::LessThanEq => "<=".to_string(),
                Token::GreatThan => ">".to_string(),
                Token::GreatThanEq => ">=".to_string(),

                Token::LogicAnd => "&&".to_string(),
                Token::LogicOr => "||".to_string(),

                Token::And => "AND".to_string(),
                Token::Or => "OR".to_string(),
                Token::Not => "NOT".to_string(),
                Token::Xor => "XOR".to_string(),

                Token::FatArrow => "=>".to_string(),

                Token::EOF => "EOF".to_string(),
            }
        )
    }
}
