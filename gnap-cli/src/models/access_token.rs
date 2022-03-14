use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
#[serde(untagged)]
pub enum AccessRequest {
    Reference(String),
    Value {
        #[serde(rename = "type")]
        resource_type: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        actions: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        locations: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        data_types: Option<Vec<String>>,
    },
}

/// Access Token portion of a grant request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessTokenRequest {
    /// Describes the rights that the
    /// client instance is requesting for one or more access tokens to be
    /// used at RS's.   This field is REQUIRED.  Section 8
    pub access: Vec<AccessRequest>,
    /// A unique name chosen by the client instance to refer
    /// to the resulting access token.  The value of this field is opaque
    /// to the AS.  If this field is included in the request, the AS MUST
    /// include the same label in the token response (Section 3.2).  This
    /// field is REQUIRED if used as part of a multiple access token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<Vec<AccessTokenFlag>>,
}

impl AccessTokenRequest {
    pub fn new() -> Self {
        AccessTokenRequest {
            label: None,
            access: Vec::<AccessRequest>::new(),
            flags: None,
        }
    }
}

impl Default for AccessTokenRequest {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessToken {
    // The value of the access token as a string.
    //  The value is opaque to the client instance.  The value SHOULD be
    //  limited to ASCII characters to facilitate transmission over HTTP
    //  headers within other protocols without requiring additional
    //  encoding.
    pub value: String,

    // REQUIRED for multiple access tokens, OPTIONAL for
    //  single access token.  The value of the label the client instance
    //  provided in the associated token request (Section 2.1), if
    //  present.  If the token has been split by the AS, the value of the
    //  label field is chosen by the AS and the split flag is used.
    pub label: Option<String>,

    // The management URI for this access token.
    //  If provided, the client instance MAY manage its access token as
    //  described in Section 6.  This management URI is a function of the
    //  AS and is separate from the RS the client instance is requesting
    //  access to.  This URI MUST NOT include the access token value and
    //  SHOULD be different for each access token issued in a request.
    pub manage: Option<String>,

    // RECOMMENDED.  A description of the
    //  rights associated with this access token, as defined in Section 8.
    //  If included, this MUST reflect the rights associated with the
    //  issued access token.  These rights MAY vary from what was
    //  requested by the client instance.
    pub access: Option<Vec<AccessRequest>>,

    //  OPTIONAL. The number of seconds in which the
    //  access will expire.  The client instance MUST NOT use the access
    //  token past this time.  An RS MUST NOT accept an access token past
    //  this time.  Note that the access token MAY be revoked by the AS or
    //  RS at any point prior to its expiration.
    pub expires_in: Option<u32>,

    // OPTIONAL.  The key that the token is bound to,
    //  if different from the client instance's presented key.  The key
    //  MUST be an object or string in a format described in Section 7.1.
    //  The client instance MUST be able to dereference or process the key
    //  information in order to be able to sign the request.
    pub key: Option<String>,

    // OPTIONAL.  A set of flags that represent
    //  attributes or behaviors of the access token issued by the AS.
    pub flags: Option<Vec<AccessTokenFlag>>,
}

/// AccessToken request flags.
/// A set of flags that indicate desired
/// attributes or behavior to be attached to the access token by the
/// AS.  This field is OPTIONAL.
/// Flag values MUST NOT be included more than once.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AccessTokenFlag {
    // This flag indicates whether the token is a bearer token,
    //  not bound to a key and proofing mechanism.  If the bearer flag is
    //  present, the access token is a bearer token, and the key field in
    //  this response MUST be omitted.  If the bearer flag is omitted and
    //  the key field in this response is omitted, the token is bound the
    //  key used by the client instance (Section 2.3) in its request for
    //  access.  If the bearer flag is omitted, and the key field is
    //  present, the token is bound to the key and proofing mechanism
    //  indicated in the key field.  See Section 12.7 for additional
    //  considerations on the use of bearer tokens.
    Bearer,

    // Flag indicating a hint of AS behavior on token
    //  rotation.  If this flag is present, then the client instance can
    //  expect a previously-issued access token to continue to work after
    //  it has been rotated (Section 6.1) or the underlying grant request
    //  has been modified (Section 5.3), resulting in the issuance of new
    //  access tokens.  If this flag is omitted, the client instance can
    //  anticipate a given access token will stop working after token
    //  rotation or grant request modification.  Note that a token flagged
    //  as durable can still expire or be revoked through any normal
    //  means.
    Durable,

    // Flag indicating that this token was generated by
    //  issuing multiple access tokens in response to one of the client
    //  instance's token request (Section 2.1) objects.  This behavior
    //  MUST NOT be used unless the client instance has specifically
    //  requested it by use of the split flag.
    Split,
}
