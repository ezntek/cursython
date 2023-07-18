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
    name: Value,
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
