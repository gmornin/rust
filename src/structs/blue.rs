use serde::{Deserialize, Serialize};

use crate::traits::SerdeAny;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BlueRenderDisplay {
    pub from: String,
    pub to: String,
    pub preset: String,
}

#[typetag::serde(name = "blue render")]
impl SerdeAny for BlueRenderDisplay {
    fn exit_status(&self) -> u16 {
        200
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BlueRenderRes {
    pub newpath: String,
    pub id: u64,
}

#[typetag::serde(name = "blue rendered")]
impl SerdeAny for BlueRenderRes {
    fn exit_status(&self) -> u16 {
        200
    }
}
