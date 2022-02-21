use serde::ser::StdError;
use models::options::GnapOptions;
pub mod models;
pub mod serde_utils;


const GNAP_AS_HOST: &str = "http://localhost:8000";

pub fn as_path(url:&str,  part: &str) -> String {
    format!("{}/{}", url, part)
}
pub async fn discover(url: &str) -> Result<GnapOptions, Box<dyn StdError>> {
    let path = as_path(url, ".well-known/gnap-as-rs");
    println!("Using path: {}", &path);
    let response: GnapOptions = reqwest::Client::new()
        .get(&path)
        .send()
        .await?
        .json()
        .await?;
    println!("Options: {:#?}", &response);
    Ok(response)
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
