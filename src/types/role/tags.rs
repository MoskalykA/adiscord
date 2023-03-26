use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct RoleTags {
    pub bot_id: Option<String>,
    pub integration_id: Option<String>,
    pub premium_subscriber: Option<bool>,
    pub subscription_listing_id: Option<String>,
    pub available_for_purchase: Option<bool>,
    pub guild_connections: Option<bool>,
}
