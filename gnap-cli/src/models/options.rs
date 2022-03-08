use serde::{Deserialize, Serialize};

pub type KeyProof = String;
pub type KeyProofs = Vec<KeyProof>;
pub type InteractionStartModes = Vec<String>;
pub type InteractionFinishMethods = Vec<String>;
pub type SubjectFormats = Vec<String>;
pub type Assertions = Vec<String>;
pub type TokenFormats = Vec<String>;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GnapServiceEndpoints {
    pub grant_request_endpoint: String,
    pub introspection_endpoint: String,
    pub resource_registration_endpoint: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GnapOptions {
    pub service_endpoints: GnapServiceEndpoints,
    pub token_formats_supported: TokenFormats,
    pub interaction_start_modes_supported: Option<InteractionStartModes>,

    /// A list of the AS's interaction finish methods.  The values of this
    /// list correspond to the possible values for the method element of
    /// the interaction finish section (Section 2.5.2) of the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interaction_finish_methods_supported: Option<InteractionFinishMethods>,

    /// A list of the AS's supported key proofing mechanisms.  The values of
    /// this list correspond to possible values of the "proof" field of the key
    ///  section (Section 7.1) of the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_proofs_supported: Option<KeyProofs>,

    /// A list of the AS's supported subject identifier types.  The values
    /// of this list correspond to possible values of the subject identifier
    /// section (Section 2.2) of the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_formats_supported: Option<SubjectFormats>,

    /// A list of the AS's supported assertion formats.  The values of this
    /// list correspond to possible values of the subject assertion section
    /// (Section 2.2) of the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assertions_supported: Option<Assertions>,
}

impl GnapOptions {
    pub fn new(base: &str) -> Self {
        GnapOptions {
            service_endpoints: GnapServiceEndpoints {
                grant_request_endpoint: format!("{}/gnap/tx", base),
                introspection_endpoint: format!("{}/gnap/introspect", base),
                resource_registration_endpoint: format!("{}/gnap/resource", base),
            },
            token_formats_supported: vec!["jwt".to_owned(), "paseto".to_owned()],
            interaction_start_modes_supported: None,
            interaction_finish_methods_supported: None,
            key_proofs_supported: None,
            subject_formats_supported: None,
            assertions_supported: None,
        }
    }
}
