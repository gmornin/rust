#[cfg(feature = "serde-any")]
use serde::*;

use crate::structs::ProfileCustomisable;

use super::ItemVisibility;

#[cfg_attr(feature = "req-ser", derive(Serialize))]
#[cfg_attr(feature = "req-de", derive(Deserialize))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone)]
pub struct V1All3 {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[cfg_attr(feature = "req-ser", derive(Serialize))]
#[cfg_attr(feature = "req-de", derive(Deserialize))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone)]
pub struct V1TokenOnly {
    pub token: String,
}

#[cfg_attr(feature = "req-ser", derive(Serialize))]
#[cfg_attr(feature = "req-de", derive(Deserialize))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone)]
pub struct V1PasswordId {
    pub identifier: String,
    #[cfg_attr(feature = "serde-any", serde(rename = "identifier-type"))]
    pub identifier_type: V1IdentifierType,
    pub password: String,
}

#[cfg_attr(feature = "req-ser", derive(Serialize))]
#[cfg_attr(feature = "req-de", derive(Deserialize))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone)]
pub struct V1RenameAccount {
    pub token: String,
    pub new: String,
}

#[cfg_attr(feature = "req-ser", derive(Serialize))]
#[cfg_attr(feature = "req-de", derive(Deserialize))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone)]
pub struct V1ChangePassword {
    pub token: String,
    pub old: String,
    pub new: String,
}

#[cfg_attr(feature = "req-ser", derive(Serialize))]
#[cfg_attr(feature = "req-de", derive(Deserialize))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone)]
pub struct V1SetStatus {
    pub token: String,
    pub new: String,
}

#[cfg_attr(feature = "req-ser", derive(Serialize))]
#[cfg_attr(feature = "req-de", derive(Deserialize))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone)]
pub struct V1FromTo {
    pub token: String,
    pub from: String,
    #[cfg_attr(feature = "serde-any", serde(rename = "from-userid"))]
    pub from_userid: i64,
    pub to: String,
}

#[cfg_attr(feature = "req-ser", derive(Serialize))]
#[cfg_attr(feature = "req-de", derive(Deserialize))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone)]
pub struct V1SelfFromTo {
    pub token: String,
    pub from: String,
    pub to: String,
}

#[cfg_attr(feature = "req-ser", derive(Serialize))]
#[cfg_attr(feature = "req-de", derive(Deserialize))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone)]
pub struct V1PathOnly {
    pub token: String,
    pub path: String,
}

#[cfg_attr(feature = "req-ser", derive(Serialize))]
#[cfg_attr(feature = "req-de", derive(Deserialize))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone)]
pub struct V1PathVisibility {
    pub token: String,
    pub path: String,
    pub visibility: ItemVisibility,
}

#[cfg_attr(feature = "req-ser", derive(Serialize))]
#[cfg_attr(feature = "req-de", derive(Deserialize))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone, Copy)]
pub enum V1IdentifierType {
    #[cfg_attr(feature = "serde-any", serde(rename = "email"))]
    Email,
    #[cfg_attr(feature = "serde-any", serde(rename = "id"))]
    Id,
    #[cfg_attr(feature = "serde-any", serde(rename = "username"))]
    Username,
}

#[cfg_attr(feature = "req-ser", derive(Serialize))]
#[cfg_attr(feature = "req-de", derive(Deserialize))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone)]
pub struct V1ProfileOnly {
    pub token: String,
    pub profile: ProfileCustomisable,
}

#[cfg_attr(feature = "req-ser", derive(Serialize))]
#[cfg_attr(feature = "req-de", derive(Deserialize))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone)]
pub struct V1Compile {
    pub token: String,
    pub path: String,
    pub from: FromFormat,
    pub to: ToFormat,
    pub compiler: Option<Compiler>,
}

#[cfg_attr(feature = "req-ser", derive(Serialize))]
#[cfg_attr(feature = "req-de", derive(Deserialize))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone)]
pub struct V1ChangeEmail {
    pub token: String,
    pub email: String,
}

#[cfg_attr(feature = "req-ser", derive(Serialize))]
#[cfg_attr(feature = "req-de", derive(Deserialize))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone)]
pub struct V1Unqueue {
    pub token: String,
    pub id: u64,
}

#[cfg_attr(feature = "req-ser", derive(Serialize))]
#[cfg_attr(feature = "req-de", derive(Deserialize))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone, Copy)]
pub enum FromFormat {
    #[cfg_attr(feature = "serde-any", serde(rename = "markdown"))]
    Markdown,
    #[cfg_attr(feature = "serde-any", serde(rename = "latex"))]
    Latex,
}

#[cfg_attr(feature = "req-ser", derive(Serialize))]
#[cfg_attr(feature = "req-de", derive(Deserialize))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone, Copy)]
pub enum ToFormat {
    #[cfg_attr(feature = "serde-any", serde(rename = "html"))]
    Html,
    #[cfg_attr(feature = "serde-any", serde(rename = "pdf"))]
    Pdf,
}

#[cfg_attr(feature = "req-ser", derive(Serialize))]
#[cfg_attr(feature = "req-de", derive(Deserialize))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone, Copy)]
pub enum Compiler {
    #[cfg_attr(feature = "serde-any", serde(rename = "default"))]
    Default,
    #[cfg_attr(feature = "serde-any", serde(rename = "pulldown cmark"))]
    PulldownCmark,
    #[cfg_attr(feature = "serde-any", serde(rename = "pdflatex"))]
    Pdflatex,
}

impl Default for Compiler {
    fn default() -> Self {
        Self::Default
    }
}

#[cfg_attr(feature = "req-ser", derive(Serialize))]
#[cfg_attr(feature = "req-de", derive(Deserialize))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone)]
pub struct V1Publish {
    pub token: String,
    pub path: String,
    pub title: String,
    pub desc: String,
}

#[cfg_attr(feature = "req-ser", derive(Serialize))]
#[cfg_attr(feature = "req-de", derive(Deserialize))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone)]
pub struct V1UpdatePublish {
    pub token: String,
    pub id: i64,
    pub title: String,
    pub desc: String,
}
