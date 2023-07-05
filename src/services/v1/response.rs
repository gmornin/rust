#[cfg(feature = "restrait")]
use crate::traits::ResTrait;
use crate::{
    structs::{ProfileAccount, ProfileCustomisable},
    traits::ErrorTrait,
};

use super::V1Error;
#[cfg(feature = "serde-any")]
use serde::*;

#[cfg_attr(feature = "res-ser", derive(Serialize))]
#[cfg_attr(feature = "res-de", derive(Deserialize))]
#[cfg_attr(feature = "serde-any", serde(tag = "type"))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone)]
pub enum V1Response {
    // account
    #[cfg_attr(feature = "serde-any", serde(rename = "created"))]
    Created { id: i64, token: String },
    #[cfg_attr(feature = "serde-any", serde(rename = "deleted"))]
    Deleted,
    #[cfg_attr(feature = "serde-any", serde(rename = "login"))]
    Login { id: i64, token: String },
    #[cfg_attr(feature = "serde-any", serde(rename = "regenerated"))]
    RegenerateToken { token: String },
    #[cfg_attr(feature = "serde-any", serde(rename = "renamed"))]
    Renamed,

    // trigger
    #[cfg_attr(feature = "serde-any", serde(rename = "triggered"))]
    Triggered,
    #[cfg_attr(feature = "serde-any", serde(rename = "revoked"))]
    Revoked,

    // storage
    #[cfg_attr(feature = "serde-any", serde(rename = "overwritten"))]
    Overwritten,
    #[cfg_attr(feature = "serde-any", serde(rename = "dir content"))]
    DirContent { content: Vec<V1DirItem> },
    #[cfg_attr(feature = "serde-any", serde(rename = "visibility changed"))]
    VisibilityChanged,
    #[cfg_attr(feature = "serde-any", serde(rename = "file item created"))]
    FileItemCreated,
    #[cfg_attr(feature = "serde-any", serde(rename = "file item deleted"))]
    FileItemDeleted,
    #[cfg_attr(feature = "serde-any", serde(rename = "copied"))]
    Copied,
    #[cfg_attr(feature = "serde-any", serde(rename = "moved"))]
    Moved,

    // gmt
    #[cfg_attr(feature = "serde-any", serde(rename = "service created"))]
    ServiceCreated,
    #[cfg_attr(feature = "serde-any", serde(rename = "profile updated"))]
    ProfileUpdated,
    #[cfg_attr(feature = "serde-any", serde(rename = "profile"))]
    Profile {
        profile: ProfileCustomisable,
        account: ProfileAccount,
    },
    #[cfg_attr(feature = "serde-any", serde(rename = "profile-only"))]
    ProfileOnly { profile: ProfileCustomisable },
    #[cfg_attr(feature = "serde-any", serde(rename = "pfp reset"))]
    PfpReset,
    #[cfg_attr(feature = "serde-any", serde(rename = "compiled"))]
    Compiled {
        id: u64,
        newpath: String,
        message: String,
    },

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

    fn status_code(&self) -> u16 {
        match self {
            Self::Login { .. }
            | Self::RegenerateToken { .. }
            | Self::Renamed
            | Self::Deleted
            | Self::Triggered
            | Self::Revoked
            | Self::Overwritten { .. }
            | Self::DirContent { .. }
            | Self::VisibilityChanged
            | Self::FileItemDeleted
            | Self::ProfileUpdated
            | Self::Profile { .. }
            | Self::PfpReset
            | Self::Moved { .. }
            | Self::ProfileOnly { .. } => 200,
            Self::Created { .. }
            | Self::FileItemCreated { .. }
            | Self::Compiled { .. }
            | Self::Copied { .. }
            | Self::ServiceCreated => 201,
            Self::NothingChanged => 304,
            Self::Error { kind } => kind.status_code(),
        }
    }
}

#[cfg_attr(feature = "res-ser", derive(Serialize))]
#[cfg_attr(feature = "res-de", derive(Deserialize))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone)]
pub struct V1DirItem {
    pub visibility: V1Visibility,
    pub is_file: bool,
    pub name: String,
    pub last_modified: u64,
    pub size: u64,
}

#[cfg_attr(any(feature = "res-res", feature = "req-ser"), derive(Serialize))]
#[cfg_attr(any(feature = "res-de", feature = "req-de"), derive(Deserialize))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone, Copy)]
pub struct V1Visibility {
    pub inherited: bool,
    pub visibility: ItemVisibility,
}

#[cfg_attr(any(feature = "res-res", feature = "req-ser"), derive(Serialize))]
#[cfg_attr(any(feature = "res-de", feature = "req-de"), derive(Deserialize))]
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
