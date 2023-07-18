use crate::codegen::Codegen;

pub enum MathOp {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

impl Codegen for MathOp {
    fn codegen(&self) -> String {
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

pub enum CmpOp {
    Gt,
    Lt,
    Eq,
    Neq,
    Geq,
    Leq,
}

impl Codegen for CmpOp {
    fn codegen(&self) -> String {
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

pub enum BitwOp {
    And,
    Or,
    Xor,
    Neg,
    LShift,
    RShift,
}

impl Codegen for BitwOp {
    fn codegen(&self) -> String {
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

pub enum LogOp {
    And,
    Or,
    Not,
}

impl Codegen for LogOp {
    fn codegen(&self) -> String {
        use LogOp::*;

        let res = match &self {
            And => "and",
            Or => "or",
            Not => "not",
        };

        res.to_owned()
    }
}
