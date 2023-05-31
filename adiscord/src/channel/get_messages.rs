use crate::{types::channel::message::Message, Error};
use reqwest::StatusCode;
use serde::Serialize;

#[derive(Serialize)]
pub struct Query {
    /// Get messages around this message snowflake.
    pub around: Option<String>,

    /// The message snowflake to get messages before.
    pub before: Option<String>,

    /// The message snowflake to get messages after.
    pub after: Option<String>,

    /// The maximum number of messages to return (1-100). Defaults to 50.
    pub limit: Option<u8>,
}

impl crate::Channel {
    /// # Examples
    ///
    /// ```
    /// match client
    ///     .channel
    ///     .get_messages("1089521338827427852", None)
    ///     .await
    /// {
    ///     Ok(messages) => println!("{:?}", messages),
    ///     Err(error) => println!("{:?}", error),
    /// };
    /// ```
    /// ## With a filter
    ///
    /// ```
    /// match client
    ///     .channel
    ///     .get_messages(
    ///         "1089521338827427852",
    ///         Some(adiscord::channel::get_messages::Query {
    ///             around: None,
    ///             before: Some("1089869456764837888".into()),
    ///             after: Some("1089860259637624933".into()),
    ///             limit: Some(10),
    ///         }),
    ///     )
    ///     .await
    /// {
    ///     Ok(messages) => println!("{:?}", messages),
    ///     Err(error) => println!("{:?}", error),
    /// };
    /// ```
    pub async fn get_messages(
        &self,
        index: &str,
        query: Option<Query>,
    ) -> Result<Vec<Message>, Error> {
        let response = self
            .client
            .get(format!("{}/channels/{index}/messages", self.url))
            .header("Authorization", self.token.clone())
            .query(&query)
            .send()
            .await
            .unwrap();

        let status = response.status();
        match status {
            StatusCode::OK => {
                let body: Vec<Message> = response.json().await.unwrap();
                Ok(body)
            }
            _ => {
                let body: Error = response.json().await.unwrap();
                Err(body)
            }
        }
    }
}
