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

pub struct BinExpr {
    op: LogOp,
    values: Box<[Expr]>,
}

pub enum CondExpr {
    Bin(BinExpr),
    Cmp(CmpExpr),
}
