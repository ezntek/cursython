use core::fmt::Debug;
use dyn_clone::{clone_trait_object, DynClone};

#[typetag::serde(tag = "type")]
pub trait Codegen: DynClone + Debug {
    fn code_gen(&self) -> String;

    fn get_indents(&self) -> Option<usize> {
        None
    }

    fn set_indents(&mut self, n: usize) {}
}

clone_trait_object!(Codegen);
