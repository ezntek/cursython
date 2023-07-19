use crate::codegen::Codegen;
use serde::{Deserialize, Serialize};

mod expr;
mod ops;
mod stmt;

#[cfg(test)]
mod ast_tests;

#[derive(Clone, Copy)]
pub enum Stmt {}

#[derive(Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct Block {
    content: Box<[Value]>,
    indented_lvls: Option<usize>,
}

pub type Value = Box<dyn Codegen>;

#[derive(Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct Ident {
    name: String,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct Literal {
    value: String,
}

#[typetag::serde]
impl Codegen for Ident {
    fn code_gen(&self) -> String {
        self.name.clone()
    }
}

#[typetag::serde]
impl Codegen for Literal {
    fn code_gen(&self) -> String {
        self.value.clone()
    }
}

#[typetag::serde]
impl Codegen for Block {
    fn code_gen(&self) -> String {
        let indents = self.indented_lvls.unwrap_or(1);
        let indents_string = "    ".repeat(indents);

        let lines = self
            .content
            .iter()
            .map(|val| {
                let generated_code = match (val as &dyn std::any::Any).downcast_ref::<Block>() {
                    Some(block) => {
                        let mut b = block.clone();
                        b.indented_lvls = Some(indents + 1);
                        b.code_gen()
                    }
                    None => val.code_gen(),
                };
                format!("{}{}", indents_string, generated_code) //
            })
            .collect::<Vec<String>>();

        let lines = lines.join("\n");
        let mut res = String::from(":\n");
        res.push_str(lines.as_str());
        res
    }
}
