#[cfg(feature = "restrait")]
use crate::traits::ResTrait;
use crate::{
    structs::{ProfileAccount, ProfileCustomisable},
    traits::ErrorTrait,
};

use super::{Compiler, FromFormat, ToFormat, V1Error};
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
    #[cfg_attr(feature = "serde-any", serde(rename = "email changed"))]
    EmailChanged,
    #[cfg_attr(feature = "serde-any", serde(rename = "password changed"))]
    PasswordChanged,
    #[cfg_attr(feature = "serde-any", serde(rename = "verification sent"))]
    VerificationSent,
    #[cfg_attr(feature = "serde-any", serde(rename = "tree"))]
    Tree { content: V1DirTreeNode },
    #[cfg_attr(feature = "serde-any", serde(rename = "jobs"))]
    Jobs {
        current: Vec<V1Job>,
        queue: Vec<V1Job>,
    },
    #[cfg_attr(feature = "serde-any", serde(rename = "unqueued"))]
    Unqueued,

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
    #[cfg_attr(feature = "serde-any", serde(rename = "exists"))]
    Exists { value: bool },

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
    Compiled { id: u64, newpath: String },

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
            | Self::Tree { .. }
            | Self::Jobs { .. }
            | Self::RegenerateToken { .. }
            | Self::Renamed
            | Self::EmailChanged
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
            | Self::PasswordChanged
            | Self::VerificationSent
            | Self::Exists { .. }
            | Self::Unqueued
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
    #[cfg_attr(feature = "serde-any", serde(rename = "hidden"))]
    Hidden,
    #[cfg_attr(feature = "serde-any", serde(rename = "public"))]
    Public,
    #[cfg_attr(feature = "serde-any", serde(rename = "private"))]
    Private,
}

#[cfg_attr(any(feature = "res-res", feature = "req-ser"), derive(Serialize))]
#[cfg_attr(any(feature = "res-de", feature = "req-de"), derive(Deserialize))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone)]
pub struct V1DirTreeNode {
    pub visibility: V1Visibility,
    pub name: String,
    #[cfg_attr(feature = "serde-any", serde(flatten))]
    pub content: V1DirTreeItem,
}

#[cfg_attr(any(feature = "res-res", feature = "req-ser"), derive(Serialize))]
#[cfg_attr(any(feature = "res-de", feature = "req-de"), derive(Deserialize))]
#[cfg_attr(feature = "serde-any", serde(tag = "type"))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone)]
pub enum V1DirTreeItem {
    #[cfg_attr(feature = "serde-any", serde(rename = "file"))]
    File { last_modified: u64, size: u64 },
    #[cfg_attr(feature = "serde-any", serde(rename = "dir"))]
    Dir { content: Vec<V1DirTreeNode> },
}

#[cfg_attr(any(feature = "res-res", feature = "req-ser"), derive(Serialize))]
#[cfg_attr(any(feature = "res-de", feature = "req-de"), derive(Deserialize))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone)]
pub struct V1Job {
    pub id: u64,
    #[cfg_attr(feature = "serde-any", serde(flatten))]
    pub task: V1Task,
}

#[cfg_attr(any(feature = "res-res", feature = "req-ser"), derive(Serialize))]
#[cfg_attr(any(feature = "res-de", feature = "req-de"), derive(Deserialize))]
#[cfg_attr(feature = "serde-any", serde(tag = "type"))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone)]
pub enum V1Task {
    #[cfg_attr(feature = "serde-any", serde(rename = "compile"))]
    Compile {
        from: FromFormat,
        to: ToFormat,
        compiler: Compiler,
        path: String,
    },
}
