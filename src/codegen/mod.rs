use dyn_clone::{clone_trait_object, DynClone};

#[typetag::serde(tag = "type")]
pub trait Codegen: DynClone {
    fn code_gen(&self) -> String;
}

clone_trait_object!(Codegen);
