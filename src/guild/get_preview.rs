use crate::{types::guild::preview::Preview, Error};
use reqwest::StatusCode;

impl super::Guild {
    pub async fn get_preview(&self, index: &str) -> Result<Preview, Error> {
        let client = reqwest::Client::new();
        let response = client
            .get(format!("{}/guilds/{index}/preview", self.url))
            .header("Authorization", self.token.clone())
            .send()
            .await
            .unwrap();

        let status = response.status();
        match status {
            StatusCode::OK => {
                let body: Preview = response.json().await.unwrap();
                Ok(body)
            }
            _ => {
                let body: Error = response.json().await.unwrap();
                Err(body)
            }
        }
    }
}
