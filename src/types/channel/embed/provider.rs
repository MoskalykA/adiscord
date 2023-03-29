use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct EmbedProvider {
    /// name of provider
    pub name: Option<String>,

    /// url of provider
    pub url: Option<String>,
}
