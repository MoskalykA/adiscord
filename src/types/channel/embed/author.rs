use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct EmbedAuthor {
    /// name of author
    pub name: String,

    /// url of author (only supports http(s))
    pub url: Option<String>,

    /// url of author icon (only supports http(s) and attachments)
    pub icon_url: Option<String>,

    /// a proxied url of author icon
    pub proxy_icon_url: Option<String>,
}
