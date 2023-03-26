use reqwest::Client;

pub mod gets;

pub struct Webhook {
    pub url: String,
    pub client: Client,
    pub token: String,
}

impl Webhook {
    pub fn new(url: String, client: Client, token: String) -> Self {
        Self { url, client, token }
    }
}
