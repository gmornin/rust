use serde::{Deserialize, Serialize};

use crate::{
    services::v1::{Compiler, FromFormat, ToFormat},
    traits::SerdeAny,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TexCompileDisplay {
    pub from: FromFormat,
    pub to: ToFormat,
    pub compiler: Compiler,
    pub path: String,
}

#[typetag::serde(name = "tex compile")]
impl SerdeAny for TexCompileDisplay {
    fn exit_status(&self) -> u16 {
        200
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TexCompileError {
    InvalidCompileRequest,
}

#[typetag::serde(name = "tex compile error")]
impl SerdeAny for TexCompileError {
    fn exit_status(&self) -> u16 {
        500
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TexCompileRes {
    pub newpath: String,
    pub id: u64,
}

#[typetag::serde(name = "tex compiled")]
impl SerdeAny for TexCompileRes {
    fn exit_status(&self) -> u16 {
        200
    }
}
