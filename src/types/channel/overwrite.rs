use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Overwrite {
    pub id: String,
    pub r#type: u8,
    pub allow: String,
    pub deny: String,
}
