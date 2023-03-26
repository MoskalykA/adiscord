use reqwest::Client;

pub mod get_regions;

pub struct Voice {
    pub url: String,
    pub client: Client,
    pub token: String,
}

impl Voice {
    pub fn new(url: String, client: Client, token: String) -> Self {
        Self { url, client, token }
    }
}
