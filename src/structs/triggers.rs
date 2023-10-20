use serde::{Deserialize, Serialize};

use crate::traits::SerdeAny;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ServicesTriggerTypes {
    #[serde(rename = "email verification")]
    EmailVerification {
        email: String,
        username: String,
        id: i64,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ServicesTriggers {
    pub expiry: u64,
    #[serde(flatten)]
    pub value: ServicesTriggerTypes,
}

#[typetag::serde(name = "service triggers")]
impl SerdeAny for ServicesTriggers {
    fn exit_status(&self) -> u16 {
        200
    }
}
