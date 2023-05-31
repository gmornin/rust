#[cfg(feature = "restrait")]
use crate::traits::ResTrait;

use super::{DirItem, V1Error};
#[cfg(feature = "serde-any")]
use serde::*;
use std::collections::HashMap;

#[cfg_attr(feature = "ser", derive(Serialize))]
#[cfg_attr(feature = "de", derive(Deserialize))]
#[cfg_attr(feature = "serde-any", serde(tag = "type"))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone)]
pub enum V1Response {
    // account
    #[cfg_attr(feature = "serde-any", serde(rename = "created"))]
    Created { id: String, token: String },
    #[cfg_attr(feature = "serde-any", serde(rename = "deleted"))]
    Deleted,
    #[cfg_attr(feature = "serde-any", serde(rename = "token"))]
    GetToken { token: String },
    #[cfg_attr(feature = "serde-any", serde(rename = "regenerated"))]
    RegenerateToken { token: String },
    #[cfg_attr(feature = "serde-any", serde(rename = "renamed"))]
    Renamed,

    // trigger
    #[cfg_attr(feature = "serde-any", serde(rename = "triggered"))]
    Triggered,

    // storage
    #[cfg_attr(feature = "serde-any", serde(rename = "overwritten"))]
    Overwritten { path: String },
    #[cfg_attr(feature = "serde-any", serde(rename = "dir content"))]
    DirContent(HashMap<String, DirItem>),
    #[cfg_attr(feature = "serde-any", serde(rename = "visibility changed"))]
    VisibilityChanged,
    #[cfg_attr(feature = "serde-any", serde(rename = "file item created"))]
    FileItemCreated { path: String },
    #[cfg_attr(feature = "serde-any", serde(rename = "file item deleted"))]
    FileItemDeleted,
    #[cfg_attr(feature = "serde-any", serde(rename = "copied"))]
    Copied { path: String },
    #[cfg_attr(feature = "serde-any", serde(rename = "moved"))]
    Moved { path: String },

    #[cfg_attr(feature = "serde-any", serde(rename = "nothing changed"))]
    NothingChanged,
    #[cfg_attr(feature = "serde-any", serde(rename = "error"))]
    Error { kind: V1Error },
}

#[cfg(feature = "restrait")]
impl ResTrait for V1Response {
    type Error = V1Error;

    fn error(e: <Self as ResTrait>::Error) -> Self {
        Self::Error { kind: e }
    }
}
