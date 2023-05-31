#[cfg(feature = "serde-any")]
use serde::*;

#[cfg_attr(feature = "ser", derive(Serialize))]
#[cfg_attr(feature = "de", derive(Deserialize))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone, Copy)]
pub struct Visibility {
    pub inherited: bool,
    pub visibility: ItemVisibility,
}

#[cfg_attr(feature = "ser", derive(Serialize))]
#[cfg_attr(feature = "de", derive(Deserialize))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone, Copy)]
pub enum ItemVisibility {
    #[serde(rename = "hidden")]
    Hidden,
    #[serde(rename = "public")]
    Public,
    #[serde(rename = "private")]
    Private,
}
