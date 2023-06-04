use std::{error::Error, fmt::{Display, self}};

#[cfg(feature = "res-serde-any")]
use serde::*;

#[cfg(feature = "restrait")]
use crate::traits::ErrorTrait;

#[cfg_attr(feature = "res-ser", derive(Serialize))]
#[cfg_attr(feature = "res-de", derive(Deserialize))]
#[derive(Clone, Debug)]
pub enum V1Error {
    // accounts
    #[cfg_attr(feature = "res-serde-any", serde(rename = "username taken"))]
    UsernameTaken,
    #[cfg_attr(feature = "res-serde-any", serde(rename = "email taken"))]
    EmailTaken,
    #[cfg_attr(feature = "res-serde-any", serde(rename = "no such user"))]
    NoSuchUser,
    #[cfg_attr(feature = "res-serde-any", serde(rename = "password incorrect"))]
    PasswordIncorrect,
    #[cfg_attr(feature = "res-serde-any", serde(rename = "invalid token"))]
    InvalidToken,
    #[cfg_attr(feature = "res-serde-any", serde(rename = "not verified"))]
    NotVerified,

    // triggers
    #[cfg_attr(feature = "res-serde-any", serde(rename = "email mismatch"))]
    EmailMismatch,
    #[cfg_attr(feature = "res-serde-any", serde(rename = "trigger not found"))]
    TriggerNotFound,

    // storage
    #[cfg_attr(feature = "res-serde-any", serde(rename = "path occupied"))]
    PathOccupied,
    #[cfg_attr(feature = "res-serde-any", serde(rename = "file not found"))]
    FileNotFound,
    #[cfg_attr(feature = "res-serde-any", serde(rename = "filesystem error"))]
    FsError { content: String },
    #[cfg_attr(feature = "res-serde-any", serde(rename = "file too large"))]
    FileTooLarge,
    #[cfg_attr(feature = "res-serde-any", serde(rename = "no parent"))]
    NoParent,
    #[cfg_attr(feature = "res-serde-any", serde(rename = "permission denied"))]
    PermissionDenied,

    #[cfg_attr(feature = "res-serde-any", serde(rename = "external"))]
    External { content: String },
}

impl Display for V1Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{self:?}"))
    }
}

impl Error for V1Error {}

#[cfg(feature = "restrait")]
impl ErrorTrait for V1Error {
    fn external(e: Box<dyn Error>) -> Self {
        Self::External { content: e.to_string() }
    }
}
