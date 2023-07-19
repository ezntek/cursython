use crate::codegen::Codegen;

use super::ops::*;
use super::{Ident, Value};

#[derive(Clone)]
pub struct Kwarg {
    name: Ident,
    value: Value,
}

#[derive(Clone)]
pub struct MathExpr {
    op: MathOp,
    values: Box<[Value]>,
}

#[derive(Clone)]
pub struct CmpExpr {
    op: CmpOp,
    values: Box<[Value; 2]>,
}

#[derive(Clone)]
pub struct BitwExpr {
    op: BitwOp,
    values: Box<[Value; 2]>,
}

#[derive(Clone)]
pub struct LogExpr {
    op: LogOp,
    values: Box<[Value]>,
}

impl Codegen for Kwarg {
    fn code_gen(&self) -> String {
        format!("{}={}", self.name.code_gen(), self.value.code_gen())
    }
}

impl Codegen for MathExpr {
    fn code_gen(&self) -> String {
        let res = self
            .values
            .iter()
            .map(|expr| expr.code_gen())
            .collect::<Vec<String>>()
            .join(format!(" {} ", self.op.code_gen()).as_str());

        format!("({})", res)
    }
}

impl Codegen for CmpExpr {
    fn code_gen(&self) -> String {
        format!(
            "({} {} {})",
            self.values[0].code_gen(),
            self.op.code_gen(),
            self.values[1].code_gen(),
        )
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
}

impl Codegen for LogExpr {
    fn code_gen(&self) -> String {
        let res = self
            .values
            .iter()
            .map(|expr| expr.code_gen())
            .collect::<Vec<String>>()
            .join(format!(" {} ", self.op.code_gen()).as_str());

        format!("({})", res)
    }
}
