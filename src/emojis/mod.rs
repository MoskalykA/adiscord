use reqwest::Client;

pub mod gets;

pub struct Emojis {
    pub url: String,
    pub client: Client,
    pub token: String,
}

impl Emojis {
    pub fn new(url: String, client: Client, token: String) -> Self {
        Self { url, client, token }
    }
}
