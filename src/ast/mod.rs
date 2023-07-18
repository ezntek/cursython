mod expr;
mod ops;
mod stmt;

pub enum Stmt {}

pub struct Block {
    content: Vec<Stmt>,
}

pub enum Value {
    Ident(Ident),
    Literal(String),
}
pub enum Expr {
    Ident(Ident),
    Literal(String),
    MathExpr(expr::MathExpr),
    CondExpr(expr::CondExpr),
}

pub struct Ident {
    name: String,
}
