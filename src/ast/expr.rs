use crate::codegen::Codegen;

use super::ops::*;
use super::{Expr, Ident, Value};

pub struct Kwarg {
    name: Ident,
    value: Value,
}

pub struct MathExpr {
    op: MathOp,
    values: Box<[Expr]>,
}

pub struct CmpExpr {
    op: CmpOp,
    values: Box<[Expr; 2]>,
}

pub struct BitwExpr {
    op: BitwOp,
    values: Box<[Expr; 2]>,
}

pub struct LogExpr {
    op: LogOp,
    values: Box<[Expr]>,
}

pub enum CondExpr {
    Log(LogExpr),
    Cmp(CmpExpr),
}

impl Codegen for Kwarg {
    fn codegen(&self) -> String {
        format!("{}={}", self.name.codegen(), self.value.codegen())
    }
}

impl Codegen for MathExpr {
    fn codegen(&self) -> String {
        self.values
            .iter()
            .map(|expr| expr.codegen())
            .collect::<Vec<String>>()
            .join(format!(" {} ", self.op.codegen()).as_str())
    }
}

impl Codegen for CmpExpr {
    fn codegen(&self) -> String {
        format!(
            "{} {} {}",
            self.values[0].codegen(),
            self.op.codegen(),
            self.values[1].codegen(),
        )
    }
}

impl Codegen for BitwExpr {
    fn codegen(&self) -> String {
        format!(
            "{} {} {}",
            self.values[0].codegen(),
            self.op.codegen(),
            self.values[1].codegen(),
        )
    }
}

impl Codegen for LogExpr {
    fn codegen(&self) -> String {
        self.values
            .iter()
            .map(|expr| expr.codegen())
            .collect::<Vec<String>>()
            .join(format!(" {} ", self.op.codegen()).as_str())
    }
}

impl Codegen for CondExpr {
    fn codegen(&self) -> String {
        use CondExpr::*;

        match self {
            Log(expr) => expr.codegen(),
            Cmp(expr) => expr.codegen(),
        }
    }
}
