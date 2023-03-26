use super::Emojis;
use crate::{types::emoji::Emoji, Error};
use reqwest::StatusCode;

impl Emojis {
    pub async fn gets(&self, index: &str) -> Result<Vec<Emoji>, Error> {
        let client = reqwest::Client::new();
        let response = client
            .get(format!("{}/guilds/{index}/emojis", self.url))
            .header("Authorization", self.token.clone())
            .send()
            .await
            .unwrap();

        let status = response.status();
        match status {
            StatusCode::OK => {
                let body: Vec<Emoji> = response.json().await.unwrap();
                Ok(body)
            }
            _ => {
                let body: Error = response.json().await.unwrap();
                Err(body)
            }
        }
    }
}
