use std::{
    error::Error,
    fmt::{self, Display},
};

use serde::{Deserialize, Serialize};

use crate::traits::{ErrorTrait, SerdeAny};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type")]
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
    #[serde(rename = "not verified")]
    NotVerified,
    #[serde(rename = "invalid username")]
    InvalidUsername,
    #[serde(rename = "already verified")]
    AlreadyVerified,
    #[serde(rename = "cooldown")]
    Cooldown { remaining: u64 },
    #[serde(rename = "entry not found")]
    EntryNotFound,
    #[serde(rename = "timed out")]
    TimedOut,

    // triggers
    #[serde(rename = "email mismatch")]
    EmailMismatch,
    #[serde(rename = "trigger not found")]
    TriggerNotFound,
    #[serde(rename = "trigger unpeakable")]
    Unpeakable,

    // storage
    #[serde(rename = "path occupied")]
    PathOccupied,
    #[serde(rename = "file not found")]
    FileNotFound,
    #[serde(rename = "filesystem error")]
    FsError { content: String },
    #[serde(rename = "file too large")]
    FileTooLarge,
    #[serde(rename = "storage full")]
    StorageFull,
    #[serde(rename = "no parent")]
    NoParent,
    #[serde(rename = "permission denied")]
    PermissionDenied,
    #[serde(rename = "type mismatch")]
    TypeMismatch,
    #[serde(rename = "file type mismatch")]
    FileTypeMismatch { expected: String, got: String },
    #[serde(rename = "extension mismatch")]
    ExtensionMismatch,
    #[serde(rename = "browser not allowed")]
    BrowserNotAllowed,
    #[serde(rename = "job not found")]
    JobNotFound,

    // gmt
    #[serde(rename = "already created")]
    AlreadyCreated,
    #[serde(rename = "not created")]
    NotCreated,
    #[serde(rename = "too many profile details")]
    TooManyProfileDetails,
    #[serde(rename = "exceeds maximum length")]
    ExceedsMaximumLength,
    #[serde(rename = "birth cake conflict")]
    BirthCakeConflict,
    #[serde(rename = "invalid detail")]
    InvalidDetail { index: u8 },
    #[serde(rename = "gmt only")]
    GmtOnly,
    #[serde(rename = "compile error")]
    CompileError { content: String },
    #[serde(rename = "invalid compile request")]
    InvalidCompileRequest,

    #[serde(rename = "external")]
    External { content: String },
    #[serde(rename = "feature disabled")]
    FeatureDisabled,
    #[serde(rename = "any")]
    Any { value: Box<dyn SerdeAny> },
}

impl Display for V1Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{self:?}"))
    }
}

impl Error for V1Error {}

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
            | Self::InvalidCompileRequest
            | Self::AlreadyVerified
            | Self::InvalidDetail { .. } => 400,
            Self::PasswordIncorrect | Self::InvalidToken => 401,
            Self::NotVerified
            | Self::EmailMismatch
            | Self::PermissionDenied
            | Self::BrowserNotAllowed
            | Self::FeatureDisabled
            | Self::Unpeakable
            | Self::GmtOnly => 403,
            Self::NoSuchUser
            | Self::TriggerNotFound
            | Self::FileNotFound
            | Self::NotCreated
            | Self::EntryNotFound
            | Self::JobNotFound => 404,
            Self::UsernameTaken
            | Self::EmailTaken
            | Self::PathOccupied
            | Self::TypeMismatch
            | Self::BirthCakeConflict
            | Self::AlreadyCreated => 409,
            Self::StorageFull | Self::ExceedsMaximumLength | Self::FileTooLarge => 413,
            Self::FileTypeMismatch { .. } | Self::ExtensionMismatch => 415,
            Self::Cooldown { .. } => 429,
            Self::FsError { .. } | Self::External { .. } | Self::CompileError { .. } => 500,
            Self::TimedOut => 503,
            Self::Any { value } => value.exit_status(),
        }
    }
}
