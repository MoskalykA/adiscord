use reqwest::Client;

pub mod get;
pub mod get_channel;
pub mod get_preview;

pub struct Guilds {
    pub url: String,
    pub client: Client,
    pub token: String,
}

impl Guilds {
    pub fn new(url: String, client: Client, token: String) -> Self {
        Self { url, client, token }
    }
}
