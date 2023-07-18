use super::expr::*;
use super::*;

#[derive(Clone)]
pub enum IfBranchClause {
    First,
    Middle,
    End,
}

#[derive(Clone)]
pub struct IfBranch {
    clause_t: IfBranchClause,
    content: Block,
}

#[derive(Clone)]
pub struct SetStmt {
    name: Ident,
    value: Value,
}

#[derive(Clone)]
pub struct CallStmt {
    base: Value,
    args: Box<[Expr]>,
    kw_args: Box<[Kwarg]>,
}

#[derive(Clone)]
pub struct PropertyStmt {
    base: Value,
    props: Box<[Ident]>,
}

#[derive(Clone)]
pub struct IfStmt {
    cond: CondExpr,
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
    cond: CondExpr,
    content: Block,
}

#[derive(Clone)]
pub struct DefStmt {
    name: Ident,
    args: Box<[Expr]>,
    kw_args: Box<[Kwarg]>,
    content: Block,
}

impl Codegen for SetStmt {
    fn code_gen(&self) -> String {
        format!("{} = {}", self.name.code_gen(), self.value.code_gen())
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
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

    fn as_any(&self) -> &dyn std::any::Any {
        self
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

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Codegen for IfStmt {
    fn code_gen(&self) -> String {
        todo!()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
