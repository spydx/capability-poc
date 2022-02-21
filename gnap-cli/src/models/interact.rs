use serde::{Serialize, Deserialize};
use crate::models::request::RequestContinuation;

// 2.5.1 Start Mode Definitions
// This specification defines the following interaction start modes as
// an array of string values under the start key:
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InteractStartMode {
    // Indicates that the client instance can direct the end-
    //  user to an arbitrary URL for interaction.  Section 2.5.1.1
    Redirect,

    // Indicates that the client instance can launch an application
    //  on the end-user's device for interaction.  Section 2.5.1.2
    App,

    // Indicates that the client instance can communicate a
    //  human-readable short code to the end-user for use with a stable
    //  URL.  Section 2.5.1.3
    UserCode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum InteractFinishMethodType {
    Redirect,
    Push,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractFinishRequest {
    pub method: InteractFinishMethodType,
    pub uri: String,
    pub nonce: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractRequest {
    pub start: Vec<InteractStartMode>,
    pub finish: Option<InteractFinishRequest>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractResponse {
    #[serde(rename = "continue")]
    pub tx_continue: RequestContinuation,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect: Option<String>,
}
