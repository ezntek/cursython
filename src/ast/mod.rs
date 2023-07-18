use crate::codegen::Codegen;

mod expr;
mod ops;
mod stmt;

#[cfg(test)]
mod tests {
    use std::{
        fs::File,
        io::{BufWriter, Write},
    };

    use crate::codegen::Codegen;

    use super::{Block, Ident};

    #[test]
    fn test_nested_blocks() {
        let ident_1 = Ident {
            name: "foo".to_owned(),
        };

        let ident_2 = Ident {
            name: "bar".to_owned(),
        };

        let block2 = Block {
            indented_lvls: Some(1),
            content: Box::new([
                Box::new(ident_2.clone()),
                Box::new(ident_2.clone()),
                Box::new(ident_2.clone()),
            ]),
        };

        let block1 = Block {
            indented_lvls: Some(1),
            content: Box::new([
                Box::new(ident_1.clone()),
                Box::new(ident_1.clone()),
                Box::new(ident_1.clone()),
                Box::new(block2),
            ]),
        };

        let code = block1.code_gen();

        let f = File::create("out.txt").unwrap();
        let mut f = BufWriter::new(f);

        f.write_all(code.as_bytes()).unwrap();
    }
}

#[derive(Clone, Copy)]
pub enum Stmt {}

#[derive(Clone)]
pub struct Block {
    content: Box<[Value]>,
    indented_lvls: Option<usize>,
}

pub type Value = Box<dyn Codegen>;

#[derive(Clone)]
pub enum Expr {
    Ident(Ident),
    Literal(String),
    MathExpr(expr::MathExpr),
    CondExpr(expr::CondExpr),
}

#[derive(Clone)]
pub struct Ident {
    name: String,
}

impl Codegen for Ident {
    fn code_gen(&self) -> String {
        self.name.clone()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Codegen for Expr {
    fn code_gen(&self) -> String {
        use Expr::*;

        match self {
            Ident(val) => val.code_gen(),
            Literal(val) => val.clone(),
            MathExpr(val) => val.code_gen(),
            CondExpr(val) => val.code_gen(),
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Codegen for Block {
    fn code_gen(&self) -> String {
        let indents = "    ".repeat(self.indented_lvls.unwrap_or(1));

        let lines = self
            .content
            .iter()
            .map(|val| {
                let generated_code = match val.as_any().downcast_ref::<Block>() {
                    Some(block) => {
                        let mut b = block.clone();
                        b.indented_lvls = Some(
                            b.indented_lvls
                                .expect("Must have a non-empty indented levels field!")
                                + 1,
                        );
                        b.code_gen()
                    }
                    None => val.code_gen(),
                };
                format!("{}{}", indents, generated_code) //
            })
            .collect::<Vec<String>>();

        let lines = lines.join("\n");
        let mut res = String::from(":\n");
        res.push_str(lines.as_str());
        res
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
