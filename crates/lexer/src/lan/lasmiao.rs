use crate::token::Token;
use crate::traits::Lexer;
use std::collections::VecDeque;

/// Lexer for PSH (Pre-Established Harmony)
pub struct LasmiaoLexer;

fn try_match_pair(pair_queue: &mut VecDeque<char>, got: char) -> Result<(), String> {
    match pair_queue.pop_back() {
        Some(expect) if expect == got => Ok(()),
        Some(expect) => Err(format!(
            "Expect {} to match a pair, but got {}",
            expect, got
        )),
        None => Err(format!("Unexpected {}, no matching pair found", got)),
    }
}

impl Lexer for LasmiaoLexer {
    fn make_tokens(input: &str) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        let mut chars = input.chars().peekable();

        let mut pair_queue: VecDeque<char> = VecDeque::new();

        while let Some(&c) = chars.peek() {
            match c {
                ' ' | '\t' => {
                    chars.next();
                }
                '\n' => {
                    chars.next();

                    if pair_queue.is_empty() && tokens.last() != Some(&Token::Semicolon) {
                        tokens.push(Token::Semicolon);
                    }
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
                            // skip comment which start with `//` and end with `\n`
                            chars.next();
                            loop {
                                if let Some(&c) = chars.peek() {
                                    if c == '\n' {
                                        break;
                                    }
                                    chars.next();
                                } else {
                                    break;
                                }
                            }
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
                    pair_queue.push_back(')');
                }
                ')' => {
                    chars.next();
                    tokens.push(Token::RParen);
                    try_match_pair(&mut pair_queue, c)?;
                }
                '[' => {
                    chars.next();
                    tokens.push(Token::LBracket);
                    pair_queue.push_back(']');
                }
                ']' => {
                    chars.next();
                    tokens.push(Token::RBracket);
                    try_match_pair(&mut pair_queue, c)?;
                }
                '{' => {
                    chars.next();
                    tokens.push(Token::LBrace);
                    pair_queue.push_back('}');
                }
                '}' => {
                    chars.next();
                    tokens.push(Token::RBrace);
                    try_match_pair(&mut pair_queue, c)?;
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
                ':' => {
                    chars.next();
                    tokens.push(Token::Colon);
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
                            tokens.push(Token::GreatThanEq);
                        }
                        _ => {
                            tokens.push(Token::GreatThan);
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
                '@' => {
                    chars.next();
                    tokens.push(Token::At);
                }
                '$' => {
                    chars.next();
                    tokens.push(Token::Cache);
                }
                '#' => {
                    chars.next();
                    tokens.push(Token::Hash);
                }
                '0'..='9' => {
                    chars.next();
                    let mut num: u64 = c.to_digit(10).unwrap() as u64;
                    let mut num_f64 = 0_f64;
                    let mut after_dot = 0;
                    loop {
                        if let Some(&c2) = chars.peek() {
                            if c2.is_ascii_digit() {
                                chars.next();
                                if after_dot == 0 {
                                    num = num * 10 + c2.to_digit(10).unwrap() as u64;
                                } else {
                                    num_f64 +=
                                        c2.to_digit(10).unwrap() as f64 * 10_f64.powi(-after_dot);
                                    after_dot += 1;
                                }
                            } else if c2 == '.' {
                                chars.next();
                                after_dot = 1;
                                num_f64 = num as f64;
                            } else if c == '0' && c2 == 'x' {
                                // hex
                                chars.next();
                                while let Some(&c3) = chars.peek() {
                                    if c3.is_ascii_hexdigit() {
                                        chars.next();
                                        num = num * 16
                                            + u64::try_from(c3.to_digit(16).unwrap()).unwrap();
                                    } else {
                                        break;
                                    }
                                }
                            } else {
                                break;
                            }
                        }
                    }
                    if after_dot == 0 {
                        tokens.push(Token::U64(num))
                    } else {
                        tokens.push(Token::F64(num_f64))
                    }
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
        if !pair_queue.is_empty() {
            return Err(format!("{:?} remain to unmatch!", pair_queue));
        }
        if tokens.last() == Some(&Token::Semicolon) {
            tokens.pop();
        }
        Ok(tokens)
    }
}
