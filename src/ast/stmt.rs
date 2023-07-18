use super::expr::*;
use super::*;

pub enum IfBranchClause {
    Initial,
    Elif,
    Else,
}

pub struct IfBranch {
    clause_t: IfBranchClause,
    content: Block,
}

pub struct SetStmt {
    name: Ident,
    value: Value,
}

pub struct CallStmt {
    base: Value,
    args: Box<[Expr]>,
    kw_args: Box<[Kwarg]>,
}

pub struct PropertyStmt {
    base: Value,
    props: Box<[Ident]>,
}

pub struct IfStmt {
    cond: CondExpr,
    branches: Box<[IfBranch]>,
}

pub struct ForStmt {
    iter_subj: Value,
    iter_vals: Box<[Ident]>,
    content: Block,
}

pub struct WhileStmt {
    cond: CondExpr,
    content: Block,
}

pub struct DefStmt {
    name: Ident,
    args: Box<[Expr]>,
    kw_args: Box<[Kwarg]>,
    content: Block,
}

impl Codegen for SetStmt {
    fn codegen(&self) -> String {
        format!("{} = {}", self.name.codegen(), self.value.codegen())
    }
}

impl Codegen for CallStmt {
    fn codegen(&self) -> String {
        let mut args = self
            .args
            .iter()
            .map(|expr| expr.codegen())
            .collect::<Vec<String>>()
            .join(", ");
        args.push_str(", ");

        let kwargs_vec = self
            .kw_args
            .iter()
            .map(|kwarg| kwarg.codegen())
            .collect::<Vec<String>>();

        let kwargs = if !kwargs_vec.is_empty() {
            let mut s = kwargs_vec.join(", ");
            s.push_str(", ");
            s
        } else {
            "".to_owned()
        };

        format!("{}({}{})", self.base.codegen(), args, kwargs)
    }
}

impl Codegen for PropertyStmt {
    fn codegen(&self) -> String {
        let mut prop_items = self
            .props
            .iter()
            .map(|ident| ident.codegen())
            .collect::<Vec<String>>();

        let mut res = Vec::new();
        res.push(self.base.codegen());
        res.append(&mut prop_items);

        res.join(".")
    }
}

impl Codegen for IfStmt {
    fn codegen(&self) -> String {
        todo!()
    }
}
