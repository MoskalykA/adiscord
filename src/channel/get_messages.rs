use crate::{types::channel::message::Message, Error};
use reqwest::StatusCode;
use serde_derive::Serialize;

#[derive(Serialize)]
pub struct Query {
    pub around: Option<String>,
    pub before: Option<String>,
    pub after: Option<String>,
    pub limit: Option<u8>,
}

impl crate::Channel {
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
