use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct EmbedField {
    /// name of the field
    pub name: String,

    /// value of the field
    pub value: String,

    /// whether or not this field should display inline
    pub inline: Option<bool>,
}
