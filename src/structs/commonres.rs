use std::error::Error;
use std::fmt::Display;

use crate::{
    services::v1::{V1Error, V1Response},
    traits::SerdeAny,
};

#[derive(Debug)]
pub enum CommonRes {
    V1(Result<V1Response, V1Error>),
}

impl CommonRes {
    pub fn is_v1(&self) -> bool {
        matches!(self, Self::V1(_))
    }

    pub fn as_v1(self) -> Result<V1Response, V1Error> {
        #[allow(irrefutable_let_patterns)]
        if let Self::V1(res) = self {
            res
        } else {
            Err(V1Error::External {
                content: String::from("Not a V1 res (something's wrong with the code)"),
            })
        }
    }

    pub fn timedout(ver: &ApiVer) -> Self {
        match ver {
            ApiVer::V1 => Self::V1(Err(V1Error::TimedOut)),
        }
    }

    pub fn external(s: String, ver: &ApiVer) -> Self {
        match ver {
            ApiVer::V1 => Self::V1(Err(V1Error::External { content: s })),
        }
    }

    pub fn any(e: Box<dyn SerdeAny>, ver: &ApiVer) -> Self {
        match ver {
            ApiVer::V1 => Self::V1(Ok(V1Response::Any { value: e })),
        }
    }

    pub fn any_err(e: Box<dyn SerdeAny>, ver: &ApiVer) -> Self {
        match ver {
            ApiVer::V1 => Self::V1(Err(V1Error::Any { value: e })),
        }
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

#[derive(Clone, Copy)]
pub enum ApiVer {
    V1,
}

#[macro_export]
macro_rules! catch {
    ($res:expr, $ver:expr) => {
        match $res {
            Ok(res) => res,
            Err(e) => return CommonRes::external(e.to_string(), $ver),
        }
    };
}
