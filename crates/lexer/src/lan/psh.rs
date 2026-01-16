use crate::token::Token;
use crate::traits::Lexer;

/// Lexer for PSH (Pre-Established Harmony)
pub struct PSHLexer;

impl Lexer for PSHLexer {
    fn make_tokens(input: &str) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        let mut chars = input.chars().peekable();
        while let Some(&c) = chars.peek() {
            match c {
                ' ' | '\t' | '\n' => {
                    chars.next();
                }
                '+' => {
                    chars.next();
                    tokens.push(Token::Plus);
                }
                '-' => {
                    chars.next();
                    tokens.push(Token::Minus);
                }
                '*' => {
                    chars.next();
                    tokens.push(Token::Star);
                }
                '/' => {
                    chars.next();
                    match chars.peek() {
                        Some(&'/') => {
                            chars.next();
                            tokens.push(Token::DoubleSlash);
                        }
                        _ => {
                            tokens.push(Token::Slash);
                        }
                    }
                }
                '%' => {
                    chars.next();
                    tokens.push(Token::Mod);
                }
                '(' => {
                    chars.next();
                    tokens.push(Token::LParen);
                }
                ')' => {
                    chars.next();
                    tokens.push(Token::RParen);
                }
                '[' => {
                    chars.next();
                    tokens.push(Token::LBracket);
                }
                ']' => {
                    chars.next();
                    tokens.push(Token::RBracket);
                }
                '{' => {
                    chars.next();
                    tokens.push(Token::LBrace);
                }
                '}' => {
                    chars.next();
                    tokens.push(Token::RBrace);
                }
                ',' => {
                    chars.next();
                    tokens.push(Token::Comma);
                }
                '.' => {
                    chars.next();
                    tokens.push(Token::Dot);
                }
                '=' => {
                    chars.next();
                    match chars.peek() {
                        Some(&'=') => {
                            // ==
                            chars.next();
                            tokens.push(Token::DoubleEqual);
                        }
                        Some(&'>') => {
                            // =>
                            chars.next();
                            tokens.push(Token::FatArrow);
                        }
                        _ => {
                            tokens.push(Token::Equal);
                        }
                    }
                }
                '!' => {
                    chars.next();
                    match chars.peek() {
                        Some(&'=') => {
                            // !=
                            chars.next();
                            tokens.push(Token::NotEqual);
                        }
                        _ => {
                            tokens.push(Token::Not);
                        }
                    }
                }
                '<' => {
                    chars.next();
                    match chars.peek() {
                        Some(&'=') => {
                            // <=
                            chars.next();
                            tokens.push(Token::LessThanEq);
                        }
                        _ => {
                            tokens.push(Token::LessThan);
                        }
                    }
                }
                '>' => {
                    chars.next();
                    match chars.peek() {
                        Some(&'=') => {
                            // >=
                            chars.next();
                            tokens.push(Token::GreatThan);
                        }
                        _ => {
                            tokens.push(Token::GreatThanEq);
                        }
                    }
                }
                '&' => {
                    chars.next();
                    match chars.peek() {
                        Some(&'&') => {
                            // &&
                            chars.next();
                            tokens.push(Token::LogicAnd);
                        }
                        _ => {
                            tokens.push(Token::And);
                        }
                    }
                }
                '|' => {
                    chars.next();
                    match chars.peek() {
                        Some(&'|') => {
                            // ||
                            chars.next();
                            tokens.push(Token::LogicOr);
                        }
                        _ => {
                            tokens.push(Token::Or);
                        }
                    }
                }
                '^' => {
                    chars.next();
                    tokens.push(Token::Xor);
                }
                '0'..='9' => {
                    chars.next();
                    let mut num = c.to_digit(10).unwrap() as f64;
                    let mut after_dot = 0;
                    loop {
                        if let Some(&c2) = chars.peek() {
                            if c2.is_ascii_digit() {
                                chars.next();
                                if after_dot == 0 {
                                    num = num * 10. + c2.to_digit(10).unwrap() as f64;
                                } else {
                                    num +=
                                        c2.to_digit(10).unwrap() as f64 * 10_f64.powi(-after_dot);
                                    after_dot += 1;
                                }
                            } else if c2 == '.' {
                                chars.next();
                                after_dot = 1;
                            } else if c == '0' && c2 == 'x' {
                                // hex
                                chars.next();
                                while let Some(&c3) = chars.peek() {
                                    if c3.is_ascii_hexdigit() {
                                        chars.next();
                                        num = num * 16. + c3.to_digit(16).unwrap() as f64;
                                    } else {
                                        break;
                                    }
                                }
                            } else {
                                break;
                            }
                        }
                    }
                    tokens.push(Token::Number(num))
                }
                _ => {
                    let mut symbol = String::new();
                    chars.next();
                    symbol.push(c);
                    loop {
                        if let Some(&c2) = chars.peek() {
                            match c2 {
                                'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => {
                                    chars.next();
                                    symbol.push(c2);
                                }
                                _ => break,
                            }
                        }
                    }
                    tokens.push(Token::Symbol(symbol))
                }
            }
        }
        tokens.push(Token::EOF);
        Ok(tokens)
    }
}
