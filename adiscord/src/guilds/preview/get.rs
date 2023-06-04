use crate::{Error};
use adiscord_types::api::guilds::previews::Preview;
use reqwest::StatusCode;

impl crate::Guild {
    /// # Examples
    ///
    /// ```
    /// match client.guild.get_preview("1089521338286342195").await {
    ///     Ok(preview) => println!("{:?}", preview.name),
    ///     Err(error) => println!("{:?}", error),
    /// };
    /// ```
    pub async fn get_preview(&self, index: &str) -> Result<Preview, Error> {
        let response = self
            .client
            .get(format!("{}/guilds/{index}/preview", self.url))
            .header("Authorization", self.token.clone())
            .send()
            .await
            .unwrap();

        let status = response.status();
        match status {
            StatusCode::OK => {
                let body: Preview = response.json().await.unwrap();
                Ok(body)
            }
            _ => {
                let body: Error = response.json().await.unwrap();
                Err(body)
            }
        }
    }
}
