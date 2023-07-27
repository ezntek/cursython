use super::codegen::Codegen;
use serde::{Deserialize, Serialize};

use super::ops::*;
use super::{Ident, Value};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct DictKvPair {
    key: Value,
    value: Value,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct Kwarg {
    name: Ident,
    value: Value,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct MathExpr {
    op: MathOp,
    values: Box<[Value]>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct CmpExpr {
    op: CmpOp,
    values: Box<[Value; 2]>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct BitwExpr {
    op: BitwOp,
    values: Box<[Value; 2]>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct LogExpr {
    op: LogOp,
    values: Box<[Value]>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct TupleExpr {
    elems: Box<[Value]>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct ListExpr {
    elems: Box<[Value]>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct DictExpr {
    elems: Box<[DictKvPair]>,
}
#[typetag::serde]
impl Codegen for Kwarg {
    fn code_gen(&self) -> String {
        format!("{}={}", self.name.code_gen(), self.value.code_gen())
    }
}

#[typetag::serde]
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

#[typetag::serde]
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

#[typetag::serde]
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

#[typetag::serde]
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

#[typetag::serde]
impl Codegen for TupleExpr {
    fn code_gen(&self) -> String {
        let res = self
            .elems
            .iter()
            .map(|elem| elem.code_gen())
            .collect::<Vec<String>>()
            .join(", ");

        format!("({})", res)
    }
}

#[typetag::serde]
impl Codegen for ListExpr {
    fn code_gen(&self) -> String {
        let res = self
            .elems
            .iter()
            .map(|elem| elem.code_gen())
            .collect::<Vec<String>>()
            .join(", ");

        format!("[{}]", res)
    }
}

#[typetag::serde]
impl Codegen for DictExpr {
    fn code_gen(&self) -> String {
        let res = self
            .elems
            .iter()
            .map(|elem| elem.code_gen())
            .collect::<Vec<String>>()
            .join(", ");
        format!("{{ {} }}", res)
    }
}

#[typetag::serde]
impl Codegen for DictKvPair {
    fn code_gen(&self) -> String {
        format!("{}: {}", self.key.code_gen(), self.value.code_gen())
    }
}
