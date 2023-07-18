use crate::codegen::Codegen;

use super::ops::*;
use super::{Expr, Ident, Value};

#[derive(Clone)]
pub struct Kwarg {
    name: Ident,
    value: Value,
}

#[derive(Clone)]
pub struct MathExpr {
    op: MathOp,
    values: Box<[Expr]>,
}

#[derive(Clone)]
pub struct CmpExpr {
    op: CmpOp,
    values: Box<[Expr; 2]>,
}

#[derive(Clone)]
pub struct BitwExpr {
    op: BitwOp,
    values: Box<[Expr; 2]>,
}

#[derive(Clone)]
pub struct LogExpr {
    op: LogOp,
    values: Box<[Expr]>,
}

#[derive(Clone)]
pub enum CondExpr {
    Log(LogExpr),
    Cmp(CmpExpr),
}

impl Codegen for Kwarg {
    fn code_gen(&self) -> String {
        format!("{}={}", self.name.code_gen(), self.value.code_gen())
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Codegen for MathExpr {
    fn code_gen(&self) -> String {
        self.values
            .iter()
            .map(|expr| expr.code_gen())
            .collect::<Vec<String>>()
            .join(format!(" {} ", self.op.code_gen()).as_str())
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Codegen for CmpExpr {
    fn code_gen(&self) -> String {
        format!(
            "{} {} {}",
            self.values[0].code_gen(),
            self.op.code_gen(),
            self.values[1].code_gen(),
        )
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Codegen for BitwExpr {
    fn code_gen(&self) -> String {
        format!(
            "{} {} {}",
            self.values[0].code_gen(),
            self.op.code_gen(),
            self.values[1].code_gen(),
        )
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Codegen for LogExpr {
    fn code_gen(&self) -> String {
        self.values
            .iter()
            .map(|expr| expr.code_gen())
            .collect::<Vec<String>>()
            .join(format!(" {} ", self.op.code_gen()).as_str())
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Codegen for CondExpr {
    fn code_gen(&self) -> String {
        use CondExpr::*;

        match self {
            Log(expr) => expr.code_gen(),
            Cmp(expr) => expr.code_gen(),
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
