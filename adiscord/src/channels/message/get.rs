use crate::{Client, Error};
use adiscord_types::api::message::Message;
use reqwest::StatusCode;

impl Client {
    /// # Examples
    ///
    /// ```
    /// match client
    ///     .channel
    ///     .get_message("1089521338827427852", "1089869456764837888")
    ///     .await
    /// {
    ///     Ok(message) => println!("{:?}", message),
    ///     Err(error) => println!("{:?}", error),
    /// };
    /// ```
    pub async fn get_message(&self, index: &str, message_index: &str) -> Result<Message, Error> {
        let response = self
            .client
            .get(format!(
                "{}/channels/{index}/messages/{message_index}",
                self.url
            ))
            .header("Authorization", self.token.clone())
            .send()
            .await
            .unwrap();

        let status = response.status();
        match status {
            StatusCode::OK => {
                let body: Message = response.json().await.unwrap();
                Ok(body)
            }
            _ => {
                let body: Error = response.json().await.unwrap();
                Err(body)
            }
        }
    }
}
