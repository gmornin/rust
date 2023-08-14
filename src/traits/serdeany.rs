use std::{any::Any, fmt::Debug};

#[typetag::serde(tag = "type")]
pub trait SerdeAny: dyn_clone::DynClone + Debug + Send + Any + 'static {
    fn exit_status(&self) -> u16;
}

dyn_clone::clone_trait_object!(SerdeAny);
