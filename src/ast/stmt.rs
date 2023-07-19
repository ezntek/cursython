use super::expr::*;
use super::*;

#[derive(Clone)]
pub enum IfBranch {
    Top { condition: Value, content: Block },
    Middle { condition: Value, content: Block },
    Bottom { content: Block },
}

#[derive(Clone)]
pub struct SetStmt {
    name: Ident,
    value: Value,
}

#[derive(Clone)]
pub struct CallStmt {
    base: Value,
    args: Box<[Value]>,
    kw_args: Box<[Kwarg]>,
}

#[derive(Clone)]
pub struct PropertyStmt {
    base: Value,
    props: Box<[Ident]>,
}

#[derive(Clone)]
pub struct IfStmt {
    branches: Box<[IfBranch]>,
}

#[derive(Clone)]
pub struct ForStmt {
    iter_subj: Value,
    iter_vals: Box<[Ident]>,
    content: Block,
}

#[derive(Clone)]
pub struct WhileStmt {
    cond: Value,
    content: Block,
}

#[derive(Clone)]
pub struct DefStmt {
    name: Ident,
    args: Box<[Value]>,
    kw_args: Box<[Kwarg]>,
    content: Block,
}

impl Codegen for SetStmt {
    fn code_gen(&self) -> String {
        format!("{} = {}", self.name.code_gen(), self.value.code_gen())
    }
}

impl Codegen for CallStmt {
    fn code_gen(&self) -> String {
        let mut args = self
            .args
            .iter()
            .map(|expr| expr.code_gen())
            .collect::<Vec<String>>()
            .join(", ");
        args.push_str(", ");

        let kwargs_vec = self
            .kw_args
            .iter()
            .map(|kwarg| kwarg.code_gen())
            .collect::<Vec<String>>();

        let kwargs = if !kwargs_vec.is_empty() {
            let mut s = kwargs_vec.join(", ");
            s.push_str(", ");
            s
        } else {
            "".to_owned()
        };

        format!("{}({}{})", self.base.code_gen(), args, kwargs)
    }
}

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

impl Codegen for IfStmt {
    fn code_gen(&self) -> String {
        let gen_branch = |branch: &IfBranch| {
            use IfBranch::*;

            match branch {
                Top { condition, content } => {
                    format!("if {}{}", condition.code_gen(), content.code_gen())
                }
                Middle { condition, content } => {
                    format!("elif {}{}", condition.code_gen(), content.code_gen())
                }
                Bottom { content } => format!("else{}", content.code_gen()),
            }
        };

        self.branches
            .iter()
            .map(gen_branch)
            .collect::<Vec<String>>()
            .join("")
    }
}

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
}

impl Codegen for WhileStmt {
    fn code_gen(&self) -> String {
        format!("while {}{}", self.cond.code_gen(), self.content.code_gen())
    }
}

impl Codegen for DefStmt {
    fn code_gen(&self) -> String {
        let args = self
            .args
            .iter()
            .map(|expr| expr.code_gen())
            .collect::<Vec<String>>()
            .join(", ");
        let kwargs = self
            .kw_args
            .iter()
            .map(|expr| expr.code_gen())
            .collect::<Vec<String>>()
            .join(", ");

        format!(
            "def {}({}, {}){}",
            self.name.code_gen(),
            args,
            kwargs,
            self.content.code_gen()
        )
    }
}
