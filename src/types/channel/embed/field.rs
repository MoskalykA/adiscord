use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct EmbedField {
    pub name: String,
    pub value: String,
    pub inline: Option<bool>,
}
