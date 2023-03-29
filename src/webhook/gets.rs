use crate::{types::webhook::Webhook, Error};
use reqwest::StatusCode;

impl crate::Webhook {
    /// # Examples
    ///
    /// ```
    /// match client.webhook.gets("1089521338286342195").await {
    ///     Ok(webhooks) => println!("{:?}", webhooks),
    ///     Err(error) => println!("{:?}", error),
    /// };
    /// ```
    pub async fn gets(&self, index: &str) -> Result<Vec<Webhook>, Error> {
        let response = self
            .client
            .get(format!("{}/guilds/{index}/webhooks", self.url))
            .header("Authorization", self.token.clone())
            .send()
            .await
            .unwrap();

        let status = response.status();
        match status {
            StatusCode::OK => {
                let body: Vec<Webhook> = response.json().await.unwrap();
                Ok(body)
            }
            _ => {
                let body: Error = response.json().await.unwrap();
                Err(body)
            }
        }
    }
}
