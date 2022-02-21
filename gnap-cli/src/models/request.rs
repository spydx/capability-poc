//! Grant Request and Response models.
//!
//! All the models for both requesting grants and for responding to
//! a grant request.
//!
use serde::{Deserialize, Serialize};
use crate::serde_utils::deser_one_as_vec;
use crate::models::interact::InteractRequest;

use crate::models::access_token::AccessTokenRequest;



#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SubjectFormatType {
    IssSubject,
    Opaque,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SubjectAssertionType {
    IdToken,
    SAML2,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubjectRequest {
    pub formats: Option<Vec<SubjectFormatType>>,
    pub assertions: Option<Vec<SubjectAssertionType>>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged, rename_all = "lowercase")]
pub enum GnapClientInstance {
    Value {},
    Ref(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrantRequest {
    #[serde(deserialize_with = "deser_one_as_vec")]
    pub access_token: Vec<AccessTokenRequest>,
    pub subject: Option<SubjectRequest>,
    // We will only support client reference identifiers for now
    pub client: Option<GnapClientInstance>,
    // We will only support user ref ids for now
    pub user: Option<String>,
    pub interact: Option<InteractRequest>,
}


impl GrantRequest {
    pub fn add_user(self, user: String) -> Self {
        Self {
            user: Some(user),
            ..self
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContinuationAccessToken {}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ContinuationRequest {
    pub interact_ref: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interact: Option<InteractRequest>
}

impl ContinuationRequest {
    pub fn create_with_ref(int_ref: String) -> Self {
        Self {
            interact_ref: int_ref,
            interact: None
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestContinuation {
    // The URI at which the client instance can
    //  make continuation requests.  This URI MAY vary per request, or MAY
    //  be stable at the AS.  The client instance MUST use this value
    //  exactly as given when making a continuation request (Section 5).
    pub uri: String,

    // The amount of time in integer seconds
    //  the client instance SHOULD wait after receiving this continuation
    //  handle and calling the URI.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait: Option<u32>,

    // A unique access token for
    //  continuing the request, called the "continuation access token".
    //  The value of this property MUST be in the format specified in
    //  Section 3.2.1.  This access token MUST be bound to the client
    //  instance's key used in the request and MUST NOT be a bearer token.
    //  As a consequence, the flags array of this access token MUST NOT
    //  contain the string bearer and the key field MUST be omitted.  The
    //  client instance MUST present the continuation access token in all
    //  requests to the continuation URI as described in Section 7.2.
    #[serde(skip_serializing_if = "Option::is_none")]
    access_token: Option<ContinuationAccessToken>,
}
impl RequestContinuation {
    pub fn as_uri(uri: &str) -> Self {
        RequestContinuation {
            uri: uri.to_owned(),
            wait: None,
            access_token: None,
        }
    }
}