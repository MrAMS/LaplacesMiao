use lexer::Token;
use std::fmt;

#[derive(Debug)]
pub enum Expr {
    // Type
    F32(f32),
    F64(f64),
    I32(i32),
    I64(i64),
    List(Vec<Expr>),

    Identifier(String),

    Let {
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
        args: Vec<Expr>,
    },
    Lambda {
        param: String,
        body: Box<Expr>,
    },
    // Control Flow
    If {
        cond: Box<Expr>,
        then_body: Box<Expr>,
        else_body: Box<Expr>,
    },
    For {
        start: Box<Expr>,
        end: Box<Expr>,
        step: Box<Expr>,
        body: Box<Expr>,
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
            Expr::F64(n) => writeln!(f, "Literal({})", n)?,
            Expr::Identifier(s) => writeln!(f, "{}", s)?,
            Expr::Unary { op, .. } => writeln!(f, "Unary({})", op)?,
            Expr::Binary { op, .. } => writeln!(f, "Binary({})", op)?,
            Expr::Call { callee, .. } => {
                if let Expr::Identifier(name) = &**callee {
                    writeln!(f, "Call({})", name)?;
                } else {
                    writeln!(f, "Call(...)")?;
                    callee.format_as_tree(f, &new_prefix, false)?;
                }
            }
            Expr::List(_) => writeln!(f, "List")?,
            Expr::Let { name, .. } => writeln!(f, "Let({})", name)?,
            Expr::Lambda { param, .. } => writeln!(f, "Lambda({})", param)?,
            _ => writeln!(f, "???")?,
        }

        match self {
            Expr::Unary { arg, .. } => {
                arg.format_as_tree(f, &new_prefix, true)?;
            }
            Expr::Binary { left, right, .. } => {
                left.format_as_tree(f, &new_prefix, false)?;
                right.format_as_tree(f, &new_prefix, true)?;
            }
            Expr::Call { args, .. } | Expr::List(args) => {
                for (i, arg) in args.iter().enumerate() {
                    let last_child = i == args.len() - 1;
                    arg.format_as_tree(f, &new_prefix, last_child)?;
                }
            }
            Expr::Let { val, .. } => {
                val.format_as_tree(f, &new_prefix, true)?;
            }
            Expr::Lambda { body, .. } => {
                body.format_as_tree(f, &new_prefix, true)?;
            }
            _ => {}
        }
        Ok(())
    }
}
