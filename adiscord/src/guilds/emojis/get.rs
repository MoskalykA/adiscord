use crate::{Error};
use adiscord_types::api::emojis::Emoji;
use reqwest::StatusCode;

impl crate::Emoji {
    /// # Examples
    ///
    /// ```
    /// match client.emoji.gets("1089521338286342195").await {
    ///     Ok(emojis) => println!("{:?}", emojis),
    ///     Err(error) => println!("{:?}", error),
    /// };
    /// ```
    pub async fn gets(&self, index: &str) -> Result<Vec<Emoji>, Error> {
        let response = self
            .client
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
