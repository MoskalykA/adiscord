use crate::{types::voice::region::VoiceRegion, Error};
use reqwest::StatusCode;

impl super::Voice {
    /// # Examples
    ///
    /// ```
    /// match client.voice.get_regions().await {
    ///     Ok(regions) => println!("{:?}", regions),
    ///     Err(error) => println!("{:?}", error),
    /// };
    /// ```
    pub async fn get_regions(&self) -> Result<Vec<VoiceRegion>, Error> {
        let client = reqwest::Client::new();
        let response = client
            .get(format!("{}/voice/regions", self.url))
            .header("Authorization", self.token.clone())
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
