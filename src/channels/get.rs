use super::Channels;
use crate::{types::channel::Channel, Error};
use reqwest::StatusCode;

impl Channels {
    pub async fn get(&self, index: &str) -> Result<Channel, Error> {
        let client = reqwest::Client::new();
        let response = client
            .get(format!("{}/channels/{index}", self.url))
            .header("Authorization", self.token.clone())
            .send()
            .await
            .unwrap();

        let status = response.status();
        match status {
            StatusCode::OK => {
                let body: Channel = response.json().await.unwrap();
                Ok(body)
            }
            _ => {
                let body: Error = response.json().await.unwrap();
                Err(body)
            }
        }
    }
}
