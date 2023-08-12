use std::fmt::Debug;

#[typetag::serde(tag = "type")]
pub trait TaskRes: dyn_clone::DynClone + Debug + Send {}

dyn_clone::clone_trait_object!(TaskRes);
