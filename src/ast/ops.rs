use crate::codegen::Codegen;

#[derive(Clone)]
pub enum MathOp {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

#[derive(Clone)]
pub enum CmpOp {
    Gt,
    Lt,
    Eq,
    Neq,
    Geq,
    Leq,
}

#[derive(Clone)]
pub enum BitwOp {
    And,
    Or,
    Xor,
    Neg,
    LShift,
    RShift,
}

#[derive(Clone)]
pub enum LogOp {
    And,
    Or,
    Not,
}
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

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

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

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

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

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

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

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
