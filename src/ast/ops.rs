use crate::codegen::Codegen;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum MathOp {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

#[derive(Clone, Serialize, Deserialize)]
//#[serde(untagged)]
pub enum CmpOp {
    Gt,
    Lt,
    Eq,
    Neq,
    Geq,
    Leq,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum BitwOp {
    And,
    Or,
    Xor,
    Neg,
    LShift,
    RShift,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum LogOp {
    And,
    Or,
    Not,
}

#[typetag::serde]
impl Codegen for MathOp {
    fn code_gen(&self) -> String {
        use MathOp::*;

        let res = match &self {
            Add => "+",
            Sub => "-",
            Mul => "*",
            Div => "/",
            Pow => "**",
        };

        res.to_owned()
    }
}

#[typetag::serde]
impl Codegen for CmpOp {
    fn code_gen(&self) -> String {
        use CmpOp::*;

        let res = match &self {
            Gt => ">",
            Lt => "<",
            Eq => "==",
            Neq => "!=",
            Geq => ">=",
            Leq => "<=",
        };

        res.to_owned()
    }
}

#[typetag::serde]
impl Codegen for BitwOp {
    fn code_gen(&self) -> String {
        use BitwOp::*;

        let res = match &self {
            And => "&",
            Or => "|",
            Xor => "^",
            Neg => "~",
            LShift => "<<",
            RShift => ">>",
        };

        res.to_owned()
    }
}

#[typetag::serde]
impl Codegen for LogOp {
    fn code_gen(&self) -> String {
        use LogOp::*;

        let res = match &self {
            And => "and",
            Or => "or",
            Not => "not",
        };

        res.to_owned()
    }
}
