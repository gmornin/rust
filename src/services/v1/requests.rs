#[cfg(feature = "req-serde-any")]
use serde::*;

use super::V1Visibility;

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
    #[cfg_attr(feature = "req-serde-any", serde(rename = "identifier-type"))]
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
pub struct V1FromTo {
    pub token: String,
    pub from: String,
    pub from_userid: String,
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
    pub visibility: V1Visibility,
}

#[cfg_attr(feature = "req-ser", derive(Serialize))]
#[cfg_attr(feature = "req-de", derive(Deserialize))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone, Copy)]
pub enum V1IdentifierType {
    #[cfg_attr(feature = "req-serde-any", serde(rename = "email"))]
    Email,
    #[cfg_attr(feature = "req-serde-any", serde(rename = "id"))]
    Id,
    #[cfg_attr(feature = "req-serde-any", serde(rename = "username"))]
    Username,
}
