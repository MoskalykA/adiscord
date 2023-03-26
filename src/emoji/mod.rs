use reqwest::Client;

pub mod gets;

pub struct Emoji {
    pub url: String,
    pub client: Client,
    pub token: String,
}

impl Emoji {
    pub fn new(url: String, client: Client, token: String) -> Self {
        Self { url, client, token }
    }
}
