use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct EmbedVideo {
    /// source url of video
    pub url: String,

    /// a proxied url of the video
    pub proxy_url: Option<String>,

    /// height of video
    pub height: Option<u16>,

    /// width of video
    pub width: Option<u16>,
}
