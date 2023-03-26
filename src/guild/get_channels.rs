use crate::{types::channel::Channel, Error};
use reqwest::StatusCode;

impl super::Guild {
    /// # Examples
    ///
    /// ```
    /// match client.guild.get_channels("1089521338286342195").await {
    ///     Ok(channels) => println!("{:?}", channels),
    ///     Err(error) => println!("{:?}", error),
    /// };
    /// ```
    pub async fn get_channels(&self, index: &str) -> Result<Vec<Channel>, Error> {
        let response = self
            .client
            .get(format!("{}/guilds/{index}/channels", self.url))
            .header("Authorization", self.token.clone())
            .send()
            .await
            .unwrap();

        let status = response.status();
        match status {
            StatusCode::OK => {
                let body: Vec<Channel> = response.json().await.unwrap();
                Ok(body)
            }
            _ => {
                let body: Error = response.json().await.unwrap();
                Err(body)
            }
        }
    }
}
