use reqwest::Client;

pub mod get;

pub struct Channel {
    pub url: String,
    pub client: Client,
    pub token: String,
}

impl Channel {
    pub fn new(url: String, client: Client, token: String) -> Self {
        Self { url, client, token }
    }
}
