use std::{error::Error, fmt::Display};

#[cfg(feature = "serde-any")]
use serde::*;

#[cfg(feature = "restrait")]
use crate::traits::ErrorTrait;

#[cfg_attr(feature = "ser", derive(Serialize))]
#[cfg_attr(feature = "de", derive(Deserialize))]
#[derive(Clone, Debug)]
pub enum V1Error {
    // accounts
    #[serde(rename = "username taken")]
    UsernameTaken,
    #[serde(rename = "email taken")]
    EmailTaken,
    #[serde(rename = "no such user")]
    NoSuchUser,
    #[serde(rename = "password incorrect")]
    PasswordIncorrect,
    #[serde(rename = "invalid token")]
    InvalidToken,

    // triggers
    #[serde(rename = "email mismatch")]
    EmailMismatch,
    #[serde(rename = "trigger not found")]
    TriggerNotFound,

    // storage
    #[serde(rename = "path occupied")]
    PathOccupied,
    #[serde(rename = "file not found")]
    FileNotFound,
    #[serde(rename = "filesystem error")]
    FsError { content: String },
    #[serde(rename = "file too large")]
    FileTooLarge,
    #[serde(rename = "no parent")]
    NoParent,
    #[serde(rename = "not editable")]
    NotEditable,

    #[serde(rename = "external")]
    External { content: String },
}

impl Display for V1Error {
    fn fmt(&self, f: &mut __private::Formatter<'_>) -> std::fmt::Result {
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
