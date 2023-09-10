use crate::{Client, Error};
use adiscord_types::api::voice::VoiceRegion;
use reqwest::StatusCode;

impl Client {
    /// # Examples
    ///
    /// ```
    /// match client.voice.get_regions().await {
    ///     Ok(regions) => println!("{:?}", regions),
    ///     Err(error) => println!("{:?}", error),
    /// };
    /// ```
    pub async fn get_regions(&self) -> Result<Vec<VoiceRegion>, Error> {
        let response = self
            .client
            .get(format!("{}/voice/regions", self.url))
            .send()
            .await
            .unwrap();

        let status = response.status();
        match status {
            StatusCode::OK => {
                let body: Vec<VoiceRegion> = response.json().await.unwrap();
                Ok(body)
            }
            _ => {
                let body: Error = response.json().await.unwrap();
                Err(body)
            }
        }
    }
}
