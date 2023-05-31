pub use super::Visibility;

#[cfg(feature = "serde-any")]
use serde::*;

#[cfg_attr(feature = "ser", derive(Serialize))]
#[cfg_attr(feature = "de", derive(Deserialize))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone)]
pub struct DirItem {
    pub visibility: Visibility,
    pub is_file: bool,
    pub name: String,
}
