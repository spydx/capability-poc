use std::time::Duration;

use log::debug;
use models::introspect::{InstrospectResponse, IntrospectRequest};
use models::options::GnapOptions;
use reqwest::Client;
use serde::ser::StdError;

pub mod models;
pub mod serde_utils;

#[derive(Clone)]
pub struct GnapClient {
    pub basepath: String,
    pub rs_ref: String,
    pub client: Client,
}

impl GnapClient {
    pub fn build(as_host: String, rs_ref: String) -> Self {
        let c = reqwest::Client::builder()
            .timeout(Duration::from_secs(2))
            .build()
            .unwrap();
        Self {
            basepath: as_host,
            rs_ref: rs_ref,
            client: c,
        }
    }

    fn as_path(&self, part: Option<String>) -> String {
        let url = self.basepath.clone();
        if part.is_some() {
            format!("{}/{}", url, part.unwrap())
        } else {
            format!("{}", url)
        }
    }

    pub async fn discover(&self) -> Result<GnapOptions, Box<dyn StdError>> {
        let path = self.as_path(Some(".well-known/gnap-as-rs".to_string()));
        debug!("Using path: {}", &path);
        let response: GnapOptions = self.client.get(&path).send().await?.json().await?;
        debug!("Options: {:#?}", &response);
        Ok(response)
    }

    pub async fn introspect(
        &self,
        token: String,
    ) -> Result<InstrospectResponse, Box<dyn StdError>> {
        let path = self.as_path(Some("introspect".to_string()));
        debug!("Using path: {}", path);

        let ir = IntrospectRequest {
            access_token: token.to_owned(),
            proof: Some(String::from("jwks")),
            resource_server: self.rs_ref.to_owned(),
            access: None,
        };

        let response = self
            .client
            .post(&path)
            .json(&ir)
            .send()
            .await?
            .json()
            .await?;

        Ok(response)
    }
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
        let gnap_cli = GnapClient::build(gnap_host.to_string(), "test".to_string());
        let options = gnap_cli.discover().await.unwrap();

        println!("{:#?}", options);
    }
}
