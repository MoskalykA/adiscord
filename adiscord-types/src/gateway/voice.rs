use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ServerUpdate {
    /// Voice connection token
    pub token: String,

    /// Guild this voice server update is for
    pub guild_id: String,

    /// Voice server host
    pub endpoint: Option<String>,
}
