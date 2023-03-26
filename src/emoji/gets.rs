use crate::{types::emoji::Emoji, Error};
use reqwest::StatusCode;

impl super::Emoji {
    /// # Examples
    ///
    /// ```
    /// match client.emoji.gets("1089521338286342195").await {
    ///     Ok(emojis) => println!("{:?}", emojis),
    ///     Err(error) => println!("{:?}", error),
    /// };
    /// ```
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
