use crate::types::Type;
use lexer::Token;
use std::fmt;

#[derive(Debug)]
pub enum Expr {
    // Type
    Unit,
    Float {
        val: f64,
        typ: Type,
    },
    Integer {
        val: u64,
        typ: Type,
    },

    List(Vec<Expr>),
    Tuple(Vec<Expr>),
    Buffer {
        size: u64,
        anno: String, // e.g. "local", "global"
    },

    Identifier {
        name: String,
        typ: Type,
    },

    Assign {
        name: Box<Expr>,
        val: Box<Expr>,
    },
    MetaDefine {
        name: String,
        val: Box<Expr>,
    },

    // Op
    Unary {
        op: Token,
        arg: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        op: Token,
        right: Box<Expr>,
    },
    // Function
    Call {
        callee: Box<Expr>,
        args: Box<Expr>,
    },
    Lambda {
        param: Box<Expr>,
        body: Box<Expr>,
    },
    // Control Flow
    // If {
    //     cond: Box<Expr>,
    //     then_body: Box<Expr>,
    //     else_body: Box<Expr>,
    // },

    // Move
    Move {
        val: Box<Expr>,
        device: Box<Expr>,
    },

    Block(Vec<Expr>),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.format_as_tree(f, "", true)
    }
}

impl Expr {
    fn format_as_tree(
        &self,
        f: &mut fmt::Formatter<'_>,
        prefix: &str,
        is_last: bool,
    ) -> fmt::Result {
        let connector = if is_last { "└── " } else { "├── " };
        write!(f, "{}{}", prefix, connector)?;

        let new_prefix = format!("{}{}", prefix, if is_last { "    " } else { "│   " });

        match self {
            Expr::Float { val, typ } => writeln!(f, "Num({}:{})", val, typ)?,
            Expr::Integer { val, typ } => writeln!(f, "Num({}:{})", val, typ)?,
            Expr::Unit => writeln!(f, "Unit")?,
            Expr::Identifier { name, typ } => writeln!(f, "{}:{}", name, typ)?,
            Expr::Unary { op, .. } => writeln!(f, "Unary({})", op)?,
            Expr::Binary { op, .. } => writeln!(f, "Binary({})", op)?,
            Expr::Call { callee, .. } => {
                if let Expr::Identifier { name, typ } = &**callee {
                    writeln!(f, "Call({}:{})", name, typ)?;
                } else {
                    writeln!(f, "Call")?;
                    callee.format_as_tree(f, &new_prefix, false)?;
                }
            }
            Expr::List(_) => writeln!(f, "List")?,
            Expr::Tuple(_) => writeln!(f, "Tuple")?,
            Expr::Buffer { size, anno } => writeln!(f, "Buffer({}):{}", size, anno)?,
            Expr::Assign { name, .. } => {
                if let Expr::Identifier { name, typ } = &**name {
                    writeln!(f, "Assign({}:{})", name, typ)?
                } else {
                    panic!("name in Expr::Let must be an Expr::Identifier")
                }
            }
            Expr::MetaDefine { name, .. } => writeln!(f, "MetaDefine({})", name)?,
            Expr::Lambda { param, .. } => {
                if let Expr::Identifier { name, typ } = &**param {
                    writeln!(f, "Lambda({}:{})", name, typ)?
                } else if let Expr::Tuple(items) = &**param {
                    let param_names: Vec<String> = items
                        .iter()
                        .map(|item| {
                            if let Expr::Identifier { name, typ } = item {
                                format!("{}:{}", name, typ)
                            } else {
                                "???".to_string()
                            }
                        })
                        .collect();
                    writeln!(f, "Lambda({})", param_names.join(", "))?
                } else {
                    panic!("param in Expr::Lambda must be an Expr::Identifier or Expr::Tuple")
                }
            }
            Expr::Move { device, .. } => {
                if let Expr::Identifier { name, .. } = &**device {
                    writeln!(f, "Move@{}", name)?
                } else {
                    panic!("device in Expr::Move must be an Expr::Identifier")
                }
            }
            _ => panic!("Do not known how to print {}", self),
        }

        match self {
            Expr::Unary { arg, .. } => {
                arg.format_as_tree(f, &new_prefix, true)?;
            }
            Expr::Binary { left, right, .. } => {
                left.format_as_tree(f, &new_prefix, false)?;
                right.format_as_tree(f, &new_prefix, true)?;
            }
            Expr::Call { args, .. } => {
                args.format_as_tree(f, &new_prefix, true)?;
            }
            Expr::Tuple(items) | Expr::List(items) => {
                for (i, item) in items.iter().enumerate() {
                    let last_child = i == items.len() - 1;
                    item.format_as_tree(f, &new_prefix, last_child)?;
                }
            }
            Expr::Assign { val, .. } => {
                val.format_as_tree(f, &new_prefix, true)?;
            }
            Expr::MetaDefine { val, .. } => {
                val.format_as_tree(f, &new_prefix, true)?;
            }
            Expr::Lambda { body, .. } => {
                body.format_as_tree(f, &new_prefix, true)?;
            }
            Expr::Move { val, .. } => {
                val.format_as_tree(f, &new_prefix, true)?;
            }
            _ => {}
        }
        Ok(())
    }
}
