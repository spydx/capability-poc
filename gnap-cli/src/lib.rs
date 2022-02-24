use serde::ser::StdError;
use models::options::GnapOptions;
use models::introspect::{IntrospectRequest, InstrospectResponse};
use log::debug;


pub mod models;
pub mod serde_utils;


#[allow(dead_code)]
const GNAP_AS_HOST: &str = "http://localhost:8000";

pub fn as_path(url:&str,  part: &str) -> String {
    format!("{}/{}", url, part)
}
pub async fn discover(url: &str) -> Result<GnapOptions, Box<dyn StdError>> {
    let path = as_path(url, ".well-known/gnap-as-rs");
    debug!("Using path: {}", &path);
    let response: GnapOptions = reqwest::Client::new()
        .get(&path)
        .send()
        .await?
        .json()
        .await?;
    debug!("Options: {:#?}", &response);
    Ok(response)
}

pub async fn introspect(token: String, rs_ref: String, url: String) -> Result<InstrospectResponse, Box<dyn StdError>> {
    let path = as_path(&url, "introspect");
    debug!("Using path: {}", path);

    let ir = IntrospectRequest {
        access_token: token.to_owned(),
        proof: Some(String::from("jwks")),
        resource_server: rs_ref.to_owned(),
        access: None
    };

    let response = reqwest::Client::new()
        .post(&path)
        .json(&ir)
        .send()
        .await?
        .json()
        .await?;

    Ok(response)
}

/*
    1. Discovery
    2. Introspecting
*/

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn gnap_session_req() {
        
        let gnap_host = "http://localhost:8000";
        let options = discover(gnap_host).await;

        println!("{:#?}", options);
    }
}
