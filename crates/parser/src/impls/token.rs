use crate::expr::Expr;
use crate::traits::Parser;
use crate::types::{TensorShapeType, Type};
use lexer::Token;

pub struct TokenParser {
    tokens: Vec<Token>,
    pos: usize,
}

impl TokenParser {
    pub fn new(tokens: Vec<Token>) -> Self {
        TokenParser { tokens, pos: 0 }
    }

    fn current(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn advance(&mut self) -> Token {
        let t = self.tokens[self.pos].clone();
        self.pos += 1;
        t
    }

    fn get_binding_power(&self, token: &Token) -> u8 {
        match token {
            Token::Comma => 1,
            Token::FatArrow => 2,
            Token::Equal | Token::Hash => 3,
            Token::DoubleEqual
            | Token::NotEqual
            | Token::GreatThan
            | Token::GreatThanEq
            | Token::LessThan
            | Token::LessThanEq => 5,
            Token::Plus | Token::Minus => 10,
            Token::Star | Token::Slash | Token::Mod => 20,
            Token::Not => 30,
            Token::Dot | Token::LParen | Token::LBracket => 40,
            Token::At => 49,
            Token::Colon => 50,
            _ => 0,
        }
    }

    fn parse_type_annotation(&mut self) -> Result<Type, String> {
        let token_after_colon = self.advance();
        if let Token::Symbol(annotation) = token_after_colon {
            match annotation.as_str() {
                "tensor" => {
                    if self.advance() == Token::LParen {
                        let right_expr = self.parse_sub_and_check_pair(Token::RParen)?;
                        let Expr::Tuple(args) = right_expr else {
                            return Err(format!(
                                "Expect an Expr::Tuple between `(` and `)` for a tensor type annotation, but got {}",
                                right_expr
                            ));
                        };

                        let mut shape: Vec<u64> = Vec::new();
                        let mut typ = Type::Unknown;

                        for arg in args {
                            match arg {
                                Expr::Integer { val, .. } => shape.push(val),
                                Expr::Identifier { name, .. } => {
                                    let t = Type::from_str(name.as_str());
                                    if typ != Type::Unknown {
                                        if t == Type::Any {
                                            break;
                                        }
                                        return Err(format!(
                                            "Expect single type for a tensor type annotation, but got two types: {} and {}",
                                            typ, t
                                        ));
                                    }
                                    typ = t;
                                }
                                _ => {
                                    return Err(format!(
                                        "Expect an Expr::Integer or Type for a tensor type annotation, but got {}",
                                        arg
                                    ));
                                }
                            }
                        }
                        if shape.len() != 0 {
                            Ok(Type::Tensor {
                                dtype: Box::new(typ),
                                shape: TensorShapeType::Shape(shape),
                            })
                        } else {
                            Ok(Type::Tensor {
                                dtype: Box::new(typ),
                                shape: TensorShapeType::Any,
                            })
                        }
                    } else {
                        return Err(format!(
                            "Expect a Token::LParen `(` after `tensor` for type annotation"
                        ));
                    }
                }
                "list" => {
                    if self.advance() == Token::LParen {
                        let typ = self.parse_type_annotation()?;
                        if self.advance() == Token::RParen {
                            Ok(Type::List(Box::new(typ)))
                        } else {
                            return Err(format!("Expect a `)` to match `(`"));
                        }
                    } else {
                        return Err(format!(
                            "Expect a Token::LParen `(` after `list` for type annotation"
                        ));
                    }
                }
                _ => Ok(Type::from_str(annotation.as_str())),
            }
        } else {
            return Err(format!(
                "Expect a Token::Symbol after Token::Colon `:` for type annotation, but got {}",
                token_after_colon
            ));
        }
    }

    fn parse_sub_and_check_pair(&mut self, expect: Token) -> Result<Expr, String> {
        if self.current() == Some(&expect) {
            self.advance();
            return Ok(Expr::Unit);
        }
        let sub_expr = self.parse_expression(0)?;
        if self.advance() != expect {
            Err(format!(
                "Expect {} to match a pair, but got {:?}",
                expect,
                self.current()
            ))
        } else {
            Ok(sub_expr)
        }
    }

    fn parse_expression(&mut self, rbp: u8) -> Result<Expr, String> {
        let token = self.advance();
        let mut left: Expr = match token {
            Token::F64(n) => Expr::Float {
                val: n,
                typ: Type::Unknown,
            },
            Token::U64(n) => Expr::Integer {
                val: n,
                typ: Type::Unknown,
            },
            Token::Minus | Token::Star => {
                let sub_expr = self.parse_expression(128)?;
                Expr::Unary {
                    op: token,
                    arg: Box::new(sub_expr),
                }
            }
            Token::Symbol(identifier) => Expr::Identifier {
                name: identifier,
                typ: Type::Unknown,
            },
            Token::LParen => self.parse_sub_and_check_pair(Token::RParen)?,
            Token::LBracket => {
                // List
                let sub_expr = self.parse_sub_and_check_pair(Token::RBracket)?;
                if let Expr::Tuple(args) = sub_expr {
                    Expr::List(args)
                } else {
                    return Err(format!(
                        "Expect a Expr::Tuple between `[` and `]`, but got {}",
                        sub_expr
                    ));
                }
            }
            Token::Cache => Expr::Buffer {
                size: 0,
                anno: String::new(),
            },
            _ => return Err(format!("Unexpected start token: {:?}", token)),
        };

        loop {
            if self.current() == None || self.get_binding_power(self.current().unwrap()) <= rbp {
                break;
            }
            let op = self.advance();
            left = match op {
                Token::Plus
                | Token::Minus
                | Token::Star
                | Token::Slash
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
                // <id> = <body>
                Token::Equal => {
                    if let Expr::Identifier { .. } = left {
                        let right_expr = self.parse_expression(self.get_binding_power(&op))?;
                        Expr::Assign {
                            name: Box::new(left),
                            val: Box::new(right_expr),
                        }
                    } else {
                        return Err(format!(
                            "Expect a string on the left of `=`, but got {:?}",
                            op
                        ));
                    }
                }
                // (args)=><body>
                Token::FatArrow => match left {
                    Expr::Identifier { .. } | Expr::Tuple(_) => {
                        let right_expr = self.parse_expression(self.get_binding_power(&op))?;
                        Expr::Lambda {
                            param: Box::new(left),
                            body: Box::new(right_expr),
                        }
                    }
                    _ => {
                        return Err(format!(
                            "Expect an Expr::Identifier or an Expr::Tuple on the left of `=>`, but got {:?}",
                            op
                        ));
                    }
                },
                // <var>.<func>(<args>)
                Token::Dot => {
                    if let Token::Symbol(callee) = self.advance() {
                        let mut args: Vec<Expr> = Vec::new();
                        args.push(left);
                        if op == Token::LParen {
                            self.advance();
                            args.extend(self.parse_sub_and_check_pair(Token::RParen))
                        }
                        Expr::Call {
                            callee: Box::new(Expr::Identifier {
                                name: callee,
                                typ: Type::Unknown,
                            }),
                            args: Box::new(Expr::Tuple(args)),
                        }
                    } else {
                        return Err(format!(
                            "Expect a Token::Symbol on the left of `.`, but got {:?}",
                            op
                        ));
                    }
                }
                Token::Comma => {
                    let right_expr = self.parse_expression(self.get_binding_power(&op))?;
                    if let Expr::Tuple(mut a) = left {
                        if let Expr::Tuple(t) = right_expr {
                            a.extend(t);
                        } else {
                            a.push(right_expr);
                        }
                        Expr::Tuple(a)
                    } else {
                        Expr::Tuple(vec![left, right_expr])
                    }
                }
                Token::LParen => {
                    let right_expr = self.parse_sub_and_check_pair(Token::RParen)?;
                    match left {
                        // e.g. sin(pi), (x=>x+1)(1)
                        Expr::Identifier { .. } | Expr::Lambda { .. } => Expr::Call {
                            callee: Box::new(left),
                            args: Box::new(right_expr),
                        },
                        // e.g. $(1024, local)
                        Expr::Buffer { .. } => {
                            if let Expr::Tuple(args) = right_expr {
                                let size: u64;
                                let anno: String;
                                if args.len() == 2 {
                                    if let Expr::Integer { val, .. } = &args[0] {
                                        size = val.clone();
                                    } else {
                                        return Err(format!(
                                            "Expect an Expr::Integer at arg0 for Buffer size, but got {}",
                                            args[0]
                                        ));
                                    }
                                    if let Expr::Identifier { name, .. } = &args[1] {
                                        anno = name.clone();
                                    } else {
                                        return Err(format!(
                                            "Expect an Expr::Identifier at arg1 for Buffer anno, but got {}",
                                            args[1]
                                        ));
                                    }
                                } else {
                                    return Err(format!(
                                        "Expect 2 args for Buffer: $(size:int, anno:str), but got {} args",
                                        args.len()
                                    ));
                                }
                                Expr::Buffer { size, anno }
                            } else {
                                return Err(format!(
                                    "Expect an Expr::Tuple for Buffer args, but got {}",
                                    right_expr
                                ));
                            }
                        }
                        _ => {
                            return Err(format!(
                                "Expect an Expr::Identifier or an Expr::Lambda on the left of an infix `(`, but got {}",
                                left
                            ));
                        }
                    }
                }
                // <id|float|int>:<type>
                Token::Colon => {
                    // Parse the type annotation
                    let new_typ = self.parse_type_annotation()?;

                    match &mut left {
                        Expr::Identifier { typ, .. }
                        | Expr::Float { typ, .. }
                        | Expr::Integer { typ, .. } => {
                            *typ = new_typ;
                            left
                        }
                        _ => {
                            return Err(format!(
                                "Expect an Identifier, Float, or Integer on the left of `:`, but got {}",
                                left
                            ));
                        }
                    }
                }
                // <expr>@<device>
                Token::At => {
                    let right_expr = self.parse_expression(self.get_binding_power(&op))?;
                    if !matches!(right_expr, Expr::Identifier { .. }) {
                        return Err(format!(
                            "Expect an Expr::Identifier after Token::At `@`, but got {}",
                            right_expr
                        ));
                    }
                    Expr::Move {
                        val: Box::new(left),
                        device: Box::new(right_expr),
                    }
                }
                // <name>#<param>
                Token::Hash => {
                    let right_expr = self.parse_expression(self.get_binding_power(&op))?;
                    if let Expr::Identifier { name, .. } = left {
                        Expr::MetaDefine {
                            name: name,
                            val: Box::new(right_expr),
                        }
                    } else {
                        return Err(format!(
                            "Expect an Expr::Identifier before `#` for MetaDefine name, but got {}",
                            right_expr
                        ));
                    }
                }
                _ => {
                    return Err(format!(
                        "Unexpected infix operator: {:?}, left now is :\nAST:\n{}",
                        op, left
                    ));
                }
            }
        }
        Ok(left)
    }
}

impl Parser for TokenParser {
    fn parse_exprs(&mut self) -> Result<Expr, String> {
        let res = self.parse_expression(0);
        if self.pos != self.tokens.len() {
            return Err(format!(
                "Unhandled tokens remain: {:?}",
                &self.tokens[self.pos..]
            ));
        }
        res
    }
}
