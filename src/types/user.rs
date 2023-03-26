use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct User {
    pub id: String,
    pub username: String,
    pub discriminator: String,
    pub avatar: String,
    pub bot: Option<bool>,
    pub system: Option<bool>,
    pub mfa_enabled: Option<bool>,
    pub banner: Option<String>,
    pub accent_color: Option<i128>,
    pub locale: Option<String>,
    pub verified: Option<bool>,
    pub email: Option<String>,
    pub flags: Option<i128>,
    pub premium_type: Option<i128>,
    pub public_flags: Option<i128>,
}
