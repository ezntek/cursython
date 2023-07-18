use crate::codegen::Codegen;

mod expr;
mod ops;
mod stmt;

#[derive(Clone, Copy)]
pub enum Stmt {}

pub struct Block {
    content: Box<[Value]>,
}

impl IntoIterator for Block {
    type Item = Value;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        return self.content.into_vec().into_iter();
    }
}

pub type Value = Box<dyn Codegen>;

pub enum Expr {
    Ident(Ident),
    Literal(String),
    MathExpr(expr::MathExpr),
    CondExpr(expr::CondExpr),
}

#[derive(Clone)]
pub struct Ident {
    name: String,
}

impl Codegen for Ident {
    fn codegen(&self) -> String {
        self.name.clone()
    }
}

impl Codegen for Expr {
    fn codegen(&self) -> String {
        use Expr::*;

        match self {
            Ident(val) => val.codegen(),
            Literal(val) => val.clone(),
            MathExpr(val) => todo!(),
            CondExpr(val) => todo!(), // FIXME: todo
        }
    }
}

impl Codegen for Block {
    fn codegen(&self) -> String {
        let mut lines = self
            .content
            .iter()
            .map(|val| val.codegen())
            .collect::<Vec<String>>();

        let lines = lines.join("\n\t");
        let mut res = String::from(":\n\t");
        res.push_str(lines.as_str());
        res
    }
}
