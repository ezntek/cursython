use dyn_clone::{clone_trait_object, DynClone};
use std::any::Any;

pub trait Codegen: DynClone {
    fn code_gen(&self) -> String;
}

clone_trait_object!(Codegen);
