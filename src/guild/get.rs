use crate::{types::guild::Guild, Error};
use reqwest::StatusCode;

impl super::Guild {
    /// # Examples
    ///
    /// ```
    /// match client.guild.get("1089521338286342195").await {
    ///     Ok(guild) => println!("{:?}", guild.name),
    ///     Err(error) => println!("{:?}", error),
    /// };
    /// ```
    pub async fn get(&self, index: &str) -> Result<Guild, Error> {
        let response = self
            .client
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
