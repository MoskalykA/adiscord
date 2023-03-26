use reqwest::Client;

pub mod gets;

pub struct Sticker {
    pub url: String,
    pub client: Client,
    pub token: String,
}

impl Sticker {
    pub fn new(url: String, client: Client, token: String) -> Self {
        Self { url, client, token }
    }
}
