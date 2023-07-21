mod expr;
mod ops;
mod stmt;
mod toplevel;

use crate::codegen::Codegen;
use serde::{Deserialize, Serialize};

#[cfg(test)]
mod ast_tests;

#[derive(Clone, Copy)]
pub enum Stmt {}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct Block {
    content: Box<[Value]>,
    indents: Option<usize>,
}

pub type Value = Box<dyn Codegen>;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct Ident {
    name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
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
        let indents = self.get_indents().unwrap();
        let indents_string = "    ".repeat(indents);

        let lines = self
            .content
            .iter()
            .map(|val| {
                dbg!(val);
                let generated_code = match val.get_indents() {
                    Some(_) => {
                        let mut new_val = val.clone();
                        new_val.set_indents(self.get_indents().unwrap() + 1);
                        new_val.code_gen()
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

    fn get_indents(&self) -> Option<usize> {
        Some(self.indents.unwrap_or(1))
    }

    fn set_indents(&mut self, n: usize) {
        self.indents = Some(n)
    }
}
