use std::fmt::Debug;

use crate::{
    structs::{ProfileAccount, ProfileCustomisable},
    traits::{ErrorTrait, ResTrait, SerdeAny},
};

use super::V1Error;
use serde::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type")]
pub enum V1Response {
    // account
    #[serde(rename = "created")]
    Created {
        id: i64,
        token: String,
        verify: bool,
    },
    #[serde(rename = "deleted")]
    Deleted,
    #[serde(rename = "login")]
    Login { id: i64, token: String },
    #[serde(rename = "regenerated")]
    RegenerateToken { token: String },
    #[serde(rename = "renamed")]
    Renamed,
    #[serde(rename = "email changed")]
    EmailChanged { verify: bool },
    #[serde(rename = "password changed")]
    PasswordChanged,
    #[serde(rename = "verification sent")]
    VerificationSent,
    #[serde(rename = "tree")]
    Tree { content: V1DirTreeNode },
    #[serde(rename = "jobs")]
    Jobs {
        current: Vec<V1Job>,
        queue: Vec<V1Job>,
    },
    #[serde(rename = "unqueued")]
    Unqueued,

    // trigger
    #[serde(rename = "triggered")]
    Triggered,
    #[serde(rename = "revoked")]
    Revoked,
    #[serde(rename = "trigger peek")]
    TriggerPeek { value: Box<dyn SerdeAny> },

    // storage
    #[serde(rename = "overwritten")]
    Overwritten,
    #[serde(rename = "dir content")]
    DirContent { content: Vec<V1DirItem> },
    #[serde(rename = "visibility changed")]
    VisibilityChanged,
    #[serde(rename = "file item created")]
    FileItemCreated,
    #[serde(rename = "file item deleted")]
    FileItemDeleted,
    #[serde(rename = "copied")]
    Copied,
    #[serde(rename = "moved")]
    Moved,
    #[serde(rename = "exists")]
    Exists { value: bool },

    // gmt
    #[serde(rename = "service created")]
    ServiceCreated,
    #[serde(rename = "profile updated")]
    ProfileUpdated,
    #[serde(rename = "profile")]
    Profile {
        profile: ProfileCustomisable,
        account: ProfileAccount,
    },
    #[serde(rename = "profile-only")]
    ProfileOnly { profile: ProfileCustomisable },
    #[serde(rename = "pfp reset")]
    PfpReset,
    #[serde(rename = "compiled")]
    TexCompiled { id: u64, newpath: String },
    #[serde(rename = "tex published")]
    TexPublished { id: u64 },
    #[serde(rename = "tex user publishes")]
    TexUserPublishes { items: Vec<V1TexUserPublish> },
    #[serde(rename = "tex user publish")]
    TexUserPublish { value: V1SingleTexUserPublish },
    #[serde(rename = "tex publish updated")]
    TexPublishUpdated,

    #[serde(rename = "multi")]
    Multi { res: Vec<Self> },
    #[serde(rename = "nothing changed")]
    NothingChanged,
    #[serde(rename = "error")]
    Error { kind: V1Error },
    #[serde(rename = "any")]
    Any { value: Box<dyn SerdeAny> },
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct V1SingleTexUserPublish {
    pub id: i64,
    pub published: u64,
    pub title: String,
    pub desc: String,
    pub ext: String,
}

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
            | Self::EmailChanged { .. }
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
            | Self::TexUserPublishes { .. }
            | Self::TexUserPublish { .. }
            | Self::NothingChanged
            | Self::TriggerPeek { .. }
            | Self::TexPublishUpdated
            | Self::ProfileOnly { .. } => 200,
            Self::Created { .. }
            | Self::FileItemCreated { .. }
            | Self::TexCompiled { .. }
            | Self::Copied { .. }
            | Self::TexPublished { .. }
            | Self::ServiceCreated => 201,
            Self::Error { kind } => kind.status_code(),
            Self::Any { value } => value.exit_status(),
            Self::Multi { res } => res
                .iter()
                .find_map(|res| match res {
                    Self::Error { kind } => Some(kind.status_code()),
                    _ => None,
                })
                .unwrap_or_else(|| match res.first() {
                    Some(res) => res.status_code(),
                    None => 200,
                }),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct V1DirItem {
    pub visibility: V1Visibility,
    pub is_file: bool,
    pub name: String,
    pub last_modified: u64,
    pub size: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct V1Visibility {
    pub inherited: bool,
    pub visibility: ItemVisibility,
}

#[derive(Serialize, Deserialize, Clone, Debug, Copy)]
pub enum ItemVisibility {
    #[serde(rename = "hidden")]
    Hidden,
    #[serde(rename = "public")]
    Public,
    #[serde(rename = "private")]
    Private,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct V1DirTreeNode {
    pub visibility: V1Visibility,
    pub name: String,
    #[serde(flatten)]
    pub content: V1DirTreeItem,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum V1DirTreeItem {
    #[serde(rename = "file")]
    File { last_modified: u64, size: u64 },
    #[serde(rename = "dir")]
    Dir { content: Vec<V1DirTreeNode> },
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct V1Job {
    pub id: u64,
    #[serde(flatten)]
    pub task: Box<dyn SerdeAny>,
}

// #[cfg_attr(any(feature = "res-res", feature = "req-ser"), derive(Serialize)]
// #[cfg_attr(any(feature = "res-de", feature = "req-de"), derive(Deserialize)]
// #[serde(tag = "type")]
// #[cfg_attr(feature = "debug", derive(Debug)]
// #[derive(Clone)]
// pub enum V1Task {
//     #[serde(rename = "compile")]
//     Compile {
//         from: FromFormat,
//         to: ToFormat,
//         compiler: Compiler,
//         path: String,
//     },
// }

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct V1TexUserPublish {
    pub id: i64,
    pub published: u64,
    pub title: String,
    pub desc: String,
    pub ext: String,
}
