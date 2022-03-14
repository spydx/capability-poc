use crate::models::access_token::AccessToken;
use crate::models::interact::InteractResponse;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrantResponse {
    pub instance_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interact: Option<InteractResponse>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access: Option<AccessToken>,
}

impl GrantResponse {
    fn create_id() -> String {
        Uuid::new_v4().to_string()
    }

    pub fn new() -> Self {
        Self {
            instance_id: Self::create_id(),
            interact: None,
            access: None,
        }
    }
}

impl Default for GrantResponse {
    fn default() -> Self {
        Self::new()
    }
}
