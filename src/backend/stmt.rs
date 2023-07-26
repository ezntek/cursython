use super::expr::*;
use super::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum IfBranch {
    If { condition: Value, content: Block },
    Elif { condition: Value, content: Block },
    Else { content: Block },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct SetStmt {
    name: Ident,
    value: Value,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct ImportStmt {
    mod_name: Value,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct ImportAsStmt {
    mod_name: Value,
    import_as: Ident,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct FromImportStmt {
    mod_name: Ident,
    import_from: Value,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct CallStmt {
    base: Value,
    args: Box<[Value]>,
    kw_args: Box<[Kwarg]>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct ReturnStmt {
    value: Value,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct YieldStmt {
    value: Value,
}

// TODO: Tuple, List and Dict syntax

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct PropertyStmt {
    base: Value,
    props: Box<[Ident]>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct IfStmt {
    branches: Box<[IfBranch]>,
    block_indents: Option<usize>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct ForStmt {
    iter_subj: Value,
    iter_vals: Box<[Ident]>,
    content: Block,
    block_indents: Option<usize>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct WhileStmt {
    cond: Value,
    content: Block,
    block_indents: Option<usize>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct DefStmt {
    name: Ident,
    args: Box<[Ident]>,
    kw_args: Box<[Kwarg]>,
    content: Block,
    decorator: Option<Value>,
    block_indents: Option<usize>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct ClassStmt {
    name: Ident,
    parents: Box<[Value]>,
    methods: Box<[DefStmt]>,
    block_indents: Option<usize>,
}

#[typetag::serde]
impl Codegen for SetStmt {
    fn code_gen(&self) -> String {
        format!("{} = {}", self.name.code_gen(), self.value.code_gen())
    }
}

#[typetag::serde]
impl Codegen for CallStmt {
    fn code_gen(&self) -> String {
        let args = self
            .args
            .iter()
            .map(|expr| expr.code_gen())
            .collect::<Vec<String>>()
            .join(", ");

        let kwargs_vec = self
            .kw_args
            .iter()
            .map(|kwarg| kwarg.code_gen())
            .collect::<Vec<String>>();

        let kwargs = if !kwargs_vec.is_empty() {
            let s = kwargs_vec.join(", ");
            let s = format!(", {}", s);
            s
        } else {
            "".to_owned()
        };

        format!("{}({}{})", self.base.code_gen(), args, kwargs)
    }
}

#[typetag::serde]
impl Codegen for ReturnStmt {
    fn code_gen(&self) -> String {
        format!("return {}", self.value.code_gen())
    }
}

#[typetag::serde]
impl Codegen for YieldStmt {
    fn code_gen(&self) -> String {
        format!("yield {}", self.value.code_gen())
    }
}

#[typetag::serde]
impl Codegen for PropertyStmt {
    fn code_gen(&self) -> String {
        let mut prop_items = self
            .props
            .iter()
            .map(|ident| ident.code_gen())
            .collect::<Vec<String>>();

        let mut res = Vec::new();
        res.push(self.base.code_gen());
        res.append(&mut prop_items);

        res.join(".")
    }
}

#[typetag::serde]
impl Codegen for ImportStmt {
    fn code_gen(&self) -> String {
        format!("import {}", self.mod_name.code_gen())
    }
}

#[typetag::serde]
impl Codegen for ImportAsStmt {
    fn code_gen(&self) -> String {
        format!(
            "import {} as {}",
            self.mod_name.code_gen(),
            self.import_as.code_gen()
        )
    }
}

#[typetag::serde]
impl Codegen for FromImportStmt {
    fn code_gen(&self) -> String {
        format!(
            "from {} import {}",
            self.import_from.code_gen(),
            self.mod_name.code_gen()
        )
    }
}

#[typetag::serde]
impl Codegen for IfStmt {
    fn code_gen(&self) -> String {
        let gen_branch = |branch: &IfBranch| {
            use IfBranch::*;

            match branch {
                If { condition, content } => {
                    format!("if {}{}", condition.code_gen(), content.code_gen())
                }
                Elif { condition, content } => {
                    format!("elif {}{}", condition.code_gen(), content.code_gen())
                }
                Else { content } => format!("else{}", content.code_gen()),
            }
        };

        self.branches
            .iter()
            .map(gen_branch)
            .collect::<Vec<String>>()
            .join("")
    }

    fn get_indents(&self) -> Option<usize> {
        Some(self.block_indents.unwrap_or(1))
    }

    fn set_indents(&mut self, n: usize) {
        self.block_indents = Some(n);

        self.branches = self
            .branches
            .iter()
            .map(|branch| match branch {
                IfBranch::If { condition, content } => {
                    let mut content = content.clone();
                    content.set_indents(n);
                    IfBranch::If {
                        condition: condition.clone(),
                        content,
                    }
                }
                IfBranch::Elif { condition, content } => {
                    let mut content = content.clone();
                    content.set_indents(n);
                    IfBranch::Elif {
                        condition: condition.clone(),
                        content,
                    }
                }
                IfBranch::Else { content } => {
                    let mut content = content.clone();
                    content.set_indents(n);
                    IfBranch::Else { content }
                }
            })
            .collect::<Vec<IfBranch>>()
            .into_boxed_slice();
    }
}

#[typetag::serde]
impl Codegen for ForStmt {
    fn code_gen(&self) -> String {
        let iter_vals = {
            let res = self
                .iter_vals
                .iter()
                .map(|ident| ident.code_gen())
                .collect::<Vec<String>>()
                .join(",");
            format!("({})", res)
        };

        format!(
            "for {} in {}{}",
            iter_vals,
            self.iter_subj.code_gen(),
            self.content.code_gen()
        )
    }

    fn get_indents(&self) -> Option<usize> {
        Some(self.block_indents.unwrap_or(1))
    }

    fn set_indents(&mut self, n: usize) {
        (self.block_indents, self.content.indents) = (Some(n), Some(n));
    }
}

#[typetag::serde]
impl Codegen for WhileStmt {
    fn code_gen(&self) -> String {
        format!("while {}{}", self.cond.code_gen(), self.content.code_gen())
    }

    fn get_indents(&self) -> Option<usize> {
        Some(self.block_indents.unwrap_or(1))
    }

    fn set_indents(&mut self, n: usize) {
        (self.block_indents, self.content.indents) = (Some(n), Some(n));
    }
}

#[typetag::serde]
impl Codegen for DefStmt {
    fn code_gen(&self) -> String {
        let indents_stmt = "    ".repeat(self.get_indents().unwrap() - 1);
        let args = self
            .args
            .iter()
            .map(|expr| expr.code_gen())
            .collect::<Vec<String>>()
            .join(", ");
        let mut kwargs = self
            .kw_args
            .iter()
            .map(|expr| expr.code_gen())
            .collect::<Vec<String>>()
            .join(", ");

        if self.kw_args.is_empty() {
            kwargs = "".to_owned()
        }

        let decorator = if let Some(deco) = &self.decorator {
            format!("@{}\n", deco.code_gen())
        } else {
            "".to_owned()
        };

        format!(
            "{}{}def {}({}{}){}",
            decorator,
            indents_stmt,
            self.name.code_gen(),
            args,
            kwargs,
            self.content.code_gen()
        )
    }

    fn get_indents(&self) -> Option<usize> {
        Some(self.block_indents.unwrap_or(1))
    }

    fn set_indents(&mut self, n: usize) {
        (self.block_indents, self.content.indents) = (Some(n), Some(n));
    }
}

#[typetag::serde]
impl Codegen for ClassStmt {
    fn code_gen(&self) -> String {
        let indents = "    ".repeat(self.get_indents().unwrap());
        let methods = self
            .methods
            .iter()
            .map(|blk| {
                let mut res = blk.clone();
                res.set_indents(self.get_indents().unwrap() + 1);
                format!("{}{}", indents, res.code_gen())
            })
            .collect::<Vec<String>>()
            .join("\n");

        let parents = self
            .parents
            .iter()
            .map(|expr| expr.code_gen())
            .collect::<Vec<String>>()
            .join(", ");

        format!("class {}({}):\n{}", self.name.code_gen(), parents, methods)
    }

    fn get_indents(&self) -> Option<usize> {
        Some(self.block_indents.unwrap_or(1))
    }

    fn set_indents(&mut self, n: usize) {
        self.block_indents = Some(n);
        self.methods = self
            .methods
            .iter()
            .map(|stmt| {
                let mut s = stmt.clone();
                s.set_indents(n);
                s
            })
            .collect::<Vec<DefStmt>>()
            .into_boxed_slice();
    }
}
