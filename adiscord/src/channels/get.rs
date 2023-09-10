use crate::{Client, Error};
use adiscord_types::api::channel::Channel;
use reqwest::StatusCode;

impl Client {
    /// # Examples
    ///
    /// ```
    /// match client.channel.get("1089521338827427852").await {
    ///     Ok(channel) => println!("{:?}", channel.name),
    ///     Err(error) => println!("{:?}", error),
    /// };
    /// ```
    pub async fn get(&self, index: &str) -> Result<Channel, Error> {
        let response = self
            .client
            .get(format!("{}/channels/{index}", self.url))
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
