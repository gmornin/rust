use std::{
    error::Error,
    fmt::{self, Display},
};

#[cfg(feature = "serde-any")]
use serde::*;

#[cfg(feature = "restrait")]
use crate::traits::ErrorTrait;

#[cfg_attr(feature = "res-ser", derive(Serialize))]
#[cfg_attr(feature = "res-de", derive(Deserialize))]
#[derive(Clone, Debug)]
pub enum V1Error {
    // accounts
    #[cfg_attr(feature = "serde-any", serde(rename = "username taken"))]
    UsernameTaken,
    #[cfg_attr(feature = "serde-any", serde(rename = "email taken"))]
    EmailTaken,
    #[cfg_attr(feature = "serde-any", serde(rename = "no such user"))]
    NoSuchUser,
    #[cfg_attr(feature = "serde-any", serde(rename = "password incorrect"))]
    PasswordIncorrect,
    #[cfg_attr(feature = "serde-any", serde(rename = "invalid token"))]
    InvalidToken,
    #[cfg_attr(feature = "serde-any", serde(rename = "not verified"))]
    NotVerified,
    #[cfg_attr(feature = "serde-any", serde(rename = "invalid username"))]
    InvalidUsername,

    // triggers
    #[cfg_attr(feature = "serde-any", serde(rename = "email mismatch"))]
    EmailMismatch,
    #[cfg_attr(feature = "serde-any", serde(rename = "trigger not found"))]
    TriggerNotFound,

    // storage
    #[cfg_attr(feature = "serde-any", serde(rename = "path occupied"))]
    PathOccupied,
    #[cfg_attr(feature = "serde-any", serde(rename = "file not found"))]
    FileNotFound,
    #[cfg_attr(feature = "serde-any", serde(rename = "filesystem error"))]
    FsError { content: String },
    #[cfg_attr(feature = "serde-any", serde(rename = "file too large"))]
    FileTooLarge,
    #[cfg_attr(feature = "serde-any", serde(rename = "no parent"))]
    NoParent,
    #[cfg_attr(feature = "serde-any", serde(rename = "permission denied"))]
    PermissionDenied,
    #[cfg_attr(feature = "serde-any", serde(rename = "type mismatch"))]
    TypeMismatch,
    #[cfg_attr(feature = "serde-any", serde(rename = "file type mismatch"))]
    FileTypeMismatch { expected: String, got: String },
    #[cfg_attr(feature = "serde-any", serde(rename = "extension mismatch"))]
    ExtensionMismatch,

    // gmt
    #[cfg_attr(feature = "serde-any", serde(rename = "already created"))]
    AlreadyCreated,
    #[cfg_attr(feature = "serde-any", serde(rename = "not created"))]
    NotCreated,
    #[cfg_attr(feature = "serde-any", serde(rename = "too many profile details"))]
    TooManyProfileDetails,
    #[cfg_attr(feature = "serde-any", serde(rename = "exceeds maximum length"))]
    ExceedsMaximumLength,
    #[cfg_attr(feature = "serde-any", serde(rename = "birth cake conflict"))]
    BirthCakeConflict,
    #[cfg_attr(feature = "serde-any", serde(rename = "invalid detail"))]
    InvalidDetail { index: u8 },

    #[cfg_attr(feature = "serde-any", serde(rename = "external"))]
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
        Self::External {
            content: e.to_string(),
        }
    }

    fn status_code(&self) -> u16 {
        match self {
            Self::NoParent
            | Self::InvalidUsername
            | Self::TooManyProfileDetails
            | Self::InvalidDetail { .. } => 400,
            Self::PasswordIncorrect | Self::InvalidToken => 401,
            Self::NotVerified | Self::EmailMismatch | Self::PermissionDenied => 403,
            Self::NoSuchUser | Self::TriggerNotFound | Self::FileNotFound | Self::NotCreated => 404,
            Self::UsernameTaken
            | Self::EmailTaken
            | Self::PathOccupied
            | Self::TypeMismatch
            | Self::BirthCakeConflict
            | Self::AlreadyCreated => 409,
            Self::FileTooLarge | Self::ExceedsMaximumLength => 413,
            Self::FileTypeMismatch { .. } | Self::ExtensionMismatch => 415,
            Self::FsError { .. } | Self::External { .. } => 500,
        }
    }
}
