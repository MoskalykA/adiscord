use crate::{types::gateway::bot::GatewayBot, Error};
use reqwest::StatusCode;

impl crate::Gateway {
    pub async fn bot(&self) -> Result<GatewayBot, Error> {
        let response = self
            .client
            .get(format!("{}/gateway/bot", self.url))
            .header("Authorization", self.token.clone())
            .send()
            .await
            .unwrap();

        let status = response.status();
        match status {
            StatusCode::OK => {
                let body: GatewayBot = response.json().await.unwrap();
                Ok(body)
            }
            _ => {
                let body: Error = response.json().await.unwrap();
                Err(body)
            }
        }
    }
}
