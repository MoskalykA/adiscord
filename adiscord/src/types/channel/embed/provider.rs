use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct EmbedProvider {
    /// name of provider
    pub name: Option<String>,

    /// url of provider
    pub url: Option<String>,
}
