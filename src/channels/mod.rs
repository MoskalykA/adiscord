use reqwest::Client;

pub mod get;

pub struct Channels {
    pub url: String,
    pub client: Client,
    pub token: String,
}

impl Channels {
    pub fn new(url: String, client: Client, token: String) -> Self {
        Self { url, client, token }
    }
}
