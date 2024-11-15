use serde::*;

use crate::structs::ProfileCustomisable;

use super::ItemVisibility;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct V1All3 {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct V1TokenOnly {
    pub token: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct V1TokenAccessType {
    pub token: String,
    pub access_type: AccessType,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct V1TokenIdentifier {
    pub token: String,
    pub identifier: String,
    #[serde(rename = "identifier-type")]
    pub identifier_type: V1IdentifierType,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct V1TokenAccessTypeOptionIdentifier {
    pub token: String,
    pub access_type: AccessType,
    pub identifier: Option<String>,
    #[serde(rename = "identifier-type")]
    pub identifier_type: Option<V1IdentifierType>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct V1PasswordId {
    pub identifier: String,
    #[serde(rename = "identifier-type")]
    pub identifier_type: V1IdentifierType,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct V1RenameAccount {
    pub token: String,
    pub new: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct V1ChangePassword {
    pub token: String,
    pub old: String,
    pub new: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct V1SetStatus {
    pub token: String,
    pub new: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct V1FromTo {
    pub token: String,
    pub from: String,
    #[serde(rename = "from-userid")]
    pub from_userid: i64,
    pub to: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct V1SelfFromTo {
    pub token: String,
    pub from: String,
    pub to: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct V1PathOnly {
    pub token: String,
    pub path: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct V1MulpiplePaths {
    pub token: String,
    pub paths: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct V1PathVisibility {
    pub token: String,
    pub path: String,
    pub visibility: ItemVisibility,
}

#[derive(Serialize, Deserialize, Clone, Debug, Copy)]
pub enum V1IdentifierType {
    #[serde(rename = "email")]
    Email,
    #[serde(rename = "id")]
    Id,
    #[serde(rename = "username")]
    Username,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct V1ProfileOnly {
    pub token: String,
    pub profile: ProfileCustomisable,
}

#[cfg(feature = "tex")]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct V1Compile {
    pub token: String,
    pub path: String,
    pub from: FromFormat,
    pub to: ToFormat,
    pub compiler: Option<Compiler>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct V1ChangeEmail {
    pub new: String,
    pub token: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct V1Unqueue {
    pub token: String,
    pub id: u64,
}

#[cfg(feature = "tex")]
#[derive(Serialize, Deserialize, Clone, Debug, Copy)]
pub enum FromFormat {
    #[serde(rename = "markdown")]
    Markdown,
    #[serde(rename = "latex")]
    Latex,
}

#[cfg(feature = "tex")]
#[derive(Serialize, Deserialize, Clone, Debug, Copy)]
pub enum ToFormat {
    #[serde(rename = "html")]
    Html,
    #[serde(rename = "pdf")]
    Pdf,
}

#[cfg(feature = "tex")]
#[derive(Serialize, Deserialize, Clone, Debug, Copy)]
pub enum Compiler {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "pulldown cmark")]
    PulldownCmark,
    #[serde(rename = "pdflatex")]
    Pdflatex,
    #[serde(rename = "lualatex")]
    Lualatex,
    #[serde(rename = "xelatex")]
    Xelatex,
}

#[cfg(feature = "tex")]
impl Default for Compiler {
    fn default() -> Self {
        Self::Default
    }
}

#[cfg(feature = "tex")]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct V1Publish {
    pub token: String,
    pub path: String,
    pub title: String,
    pub desc: String,
}

#[cfg(feature = "tex")]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct V1UpdatePublish {
    pub token: String,
    pub id: i64,
    pub path: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct V1TokenPassword {
    pub token: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct V1Access {
    pub token: String,
    pub identifier: String,
    #[serde(rename = "identifier-type")]
    pub identifier_type: V1IdentifierType,
    #[serde(rename = "type")]
    pub r#type: AccessType,
}

#[derive(Serialize, Deserialize, Clone, Debug, Hash, Copy, PartialEq, Eq)]
pub enum AccessType {
    #[serde(rename = "file")]
    File,
    #[serde(rename = "access")]
    Access,
}

impl AccessType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::File => "file",
            Self::Access => "access",
        }
    }
}

#[cfg(feature = "blue")]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct V1Render {
    pub token: String,
    pub from: String,
    pub to: String,
    pub preset: String,
}
