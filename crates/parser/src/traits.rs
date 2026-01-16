use crate::expr::Expr;

pub trait Parser {
    fn parse_exprs(&mut self) -> Result<Expr, String>;
}
