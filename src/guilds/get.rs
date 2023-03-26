use super::Guilds;
use crate::{types::test::Guild, Error};
use reqwest::StatusCode;

impl Guilds {
    pub async fn get(&self, index: &str) -> Result<Guild, Error> {
        let client = reqwest::Client::new();
        let response = client
            .get(format!("{}/guilds/{index}", self.url))
            .header("Authorization", self.token.clone())
            .send()
            .await
            .unwrap();

        let status = response.status();
        match status {
            StatusCode::OK => {
                let body: Guild = response.json().await.unwrap();
                Ok(body)
            }
            _ => {
                let body: Error = response.json().await.unwrap();
                Err(body)
            }
        }
    }
}
