use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct EmbedProvider {
    pub name: Option<String>,
    pub url: Option<String>,
}
