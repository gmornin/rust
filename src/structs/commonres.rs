use std::error::Error;
use std::fmt::Display;

use crate::services::v1::{V1Error, V1Response};

#[derive(Debug)]
pub enum CommonRes {
    V1(Result<V1Response, V1Error>),
}

impl CommonRes {
    pub fn is_v1(&self) -> bool {
        matches!(self, Self::V1(_))
    }
}

impl TryInto<Result<V1Response, V1Error>> for CommonRes {
    type Error = CommonResError;

    fn try_into(self) -> Result<Result<V1Response, V1Error>, Self::Error> {
        #[allow(irrefutable_let_patterns)]
        if let Self::V1(res) = self {
            Ok(res)
        } else {
            Err(CommonResError)
        }
    }
}

#[derive(Debug)]
pub struct CommonResError;

impl Display for CommonResError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{self:?}"))
    }
}

impl Error for CommonResError {}

pub enum ApiVer {
    V1,
}
