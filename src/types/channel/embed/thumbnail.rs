use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct EmbedThumbnail {
    pub url: String,
    pub proxy_url: Option<String>,
    pub height: Option<u16>,
    pub width: Option<u16>,
}
