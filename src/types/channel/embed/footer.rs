use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct EmbedFooter {
    /// footer text
    pub text: String,

    /// url of footer icon (only supports http(s) and attachments)
    pub icon_url: Option<String>,

    /// a proxied url of footer icon
    pub proxy_icon_url: Option<String>,
}
