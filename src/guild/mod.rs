use reqwest::Client;

pub mod get;
pub mod get_channels;
pub mod get_preview;

pub struct Guild {
    pub url: String,
    pub client: Client,
    pub token: String,
}

impl Guild {
    pub fn new(url: String, client: Client, token: String) -> Self {
        Self { url, client, token }
    }
}
