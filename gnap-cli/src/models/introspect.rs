use crate::models::access_token::AccessRequest;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IntrospectRequest {
    pub access_token: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof: Option<String>,
    pub resource_server: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access: Option<AccessRequest>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InstrospectResponse {
    pub active: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access: Option<Vec<AccessRequest>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}
