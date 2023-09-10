use crate::Error;
use adiscord_types::api::stickers::Sticker;
use reqwest::StatusCode;

impl crate::Sticker {
    /// # Examples
    ///
    /// ```
    /// match client.sticker.gets("1089521338286342195").await {
    ///     Ok(stickers) => println!("{:?}", stickers),
    ///     Err(error) => println!("{:?}", error),
    /// };
    /// ```
    pub async fn gets(&self, index: &str) -> Result<Vec<Sticker>, Error> {
        let response = self
            .client
            .get(format!("{}/guilds/{index}/stickers", self.url))
            .send()
            .await
            .unwrap();

        let status = response.status();
        match status {
            StatusCode::OK => {
                let body: Vec<Sticker> = response.json().await.unwrap();
                Ok(body)
            }
            _ => {
                let body: Error = response.json().await.unwrap();
                Err(body)
            }
        }
    }
}
