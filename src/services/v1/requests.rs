#[cfg(feature = "serde-any")]
use serde::*;

use crate::structs::Profile;

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
    pub profile: Profile,
}
