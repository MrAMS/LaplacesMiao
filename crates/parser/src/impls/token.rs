use crate::expr::Expr;
use crate::traits::Parser;
use lexer::Token;

pub struct TokenParser {
    tokens: Vec<Token>,
    pos: usize,
}

impl TokenParser {
    pub fn new(tokens: Vec<Token>) -> Self {
        TokenParser { tokens, pos: 0 }
    }

    fn current(&self) -> &Token {
        &self.tokens[self.pos]
    }

    fn advance(&mut self) -> Token {
        let t = self.tokens[self.pos].clone();
        self.pos += 1;
        t
    }

    fn get_binding_power(&self, token: &Token) -> u8 {
        match token {
            Token::FatArrow => 1,
            Token::Equal => 2,
            Token::DoubleEqual
            | Token::NotEqual
            | Token::GreatThan
            | Token::GreatThanEq
            | Token::LessThan
            | Token::LessThanEq => 5,
            Token::Plus | Token::Minus => 10,
            Token::Star | Token::Slash | Token::DoubleSlash | Token::Mod => 20,
            Token::Not => 30,
            Token::Dot | Token::LParen | Token::LBracket => 40,
            _ => 0,
        }
    }

    fn parse_list(&mut self, rtoken: &Token) -> Result<Vec<Expr>, String> {
        let mut args: Vec<Expr> = Vec::new();
        loop {
            args.push(self.parse_expression(0)?);
            let cur = self.current();
            if cur == &Token::Comma {
                self.advance();
            } else if cur == rtoken {
                self.advance();
                break;
            } else {
                return Err(format!(
                    "Expected {}, but got token: {}",
                    rtoken,
                    self.current()
                ));
            }
        }
        Ok(args)
    }

    fn parse_expression(&mut self, rbp: u8) -> Result<Expr, String> {
        let token = self.advance();
        let mut left: Expr = match token {
            Token::Number(n) => Expr::F64(n),
            Token::Minus | Token::Star => {
                let sub_expr = self.parse_expression(128)?;
                Expr::Unary {
                    op: token,
                    arg: Box::new(sub_expr),
                }
            }
            Token::Symbol(identifier) => {
                if self.current() == &Token::LParen {
                    self.advance();
                    // Function
                    let args = self.parse_list(&Token::RParen)?;
                    Expr::Call {
                        callee: Box::new(Expr::Identifier(identifier)),
                        args: args,
                    }
                } else {
                    // Symbol
                    Expr::Identifier(identifier)
                }
            }
            Token::LParen => {
                let sub_expr = self.parse_expression(0)?;
                assert_eq!(self.advance(), Token::RParen);
                sub_expr
            }
            Token::LBracket => {
                // List
                let args = self.parse_list(&Token::RBracket)?;
                Expr::List(args)
            }
            _ => return Err(format!("Unexpected start token: {:?}", token)),
        };

        loop {
            if self.current() == &Token::EOF || self.get_binding_power(self.current()) <= rbp {
                break;
            }
            let op = self.advance();
            left = match op {
                Token::Plus
                | Token::Minus
                | Token::Star
                | Token::Slash
                | Token::DoubleSlash
                | Token::Mod
                | Token::LogicAnd
                | Token::LogicOr
                | Token::And
                | Token::Or
                | Token::Xor
                | Token::DoubleEqual
                | Token::NotEqual
                | Token::LessThan
                | Token::LessThanEq
                | Token::GreatThan
                | Token::GreatThanEq => {
                    let right_expr = self.parse_expression(self.get_binding_power(&op))?;
                    Expr::Binary {
                        left: Box::new(left),
                        op,
                        right: Box::new(right_expr),
                    }
                }
                Token::Equal => {
                    if let Expr::Identifier(name) = left {
                        let right_expr = self.parse_expression(self.get_binding_power(&op))?;
                        Expr::Let {
                            name: name,
                            val: Box::new(right_expr),
                        }
                    } else {
                        return Err(format!(
                            "Expect a string on the left of Token::Equal(=), but got {:?}",
                            op
                        ));
                    }
                }
                Token::FatArrow => {
                    if let Expr::Identifier(param) = left {
                        let right_expr = self.parse_expression(self.get_binding_power(&op))?;
                        Expr::Lambda {
                            param: param,
                            body: Box::new(right_expr),
                        }
                    } else {
                        return Err(format!(
                            "Expect an Expr::Identifier on the left of Token::FatArrow(=>), but got {:?}",
                            op
                        ));
                    }
                }
                Token::Dot => {
                    if let Token::Symbol(func) = self.current().clone() {
                        self.advance();

                        let mut args: Vec<Expr> = Vec::new();
                        args.push(left);
                        if self.current() == &Token::LParen {
                            self.advance();
                            args.extend(self.parse_list(&Token::RParen)?);
                        }
                        Expr::Call {
                            callee: Box::new(Expr::Identifier(func)),
                            args: args,
                        }
                    } else {
                        return Err(format!(
                            "Expect a Token::Symbol on the left of Token::Dot(.), but got {:?}",
                            op
                        ));
                    }
                }
                _ => return Err(format!("Unexpected infix operator: {:?}", op)),
            }
        }
        Ok(left)
    }
}

impl Parser for TokenParser {
    fn parse_exprs(&mut self) -> Result<Expr, String> {
        self.parse_expression(0)
    }
}
