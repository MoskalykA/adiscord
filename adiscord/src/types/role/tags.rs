use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct RoleTags {
    /// the id of the bot this role belongs to
    pub bot_id: Option<String>,

    /// the id of the integration this role belongs to
    pub integration_id: Option<String>,

    /// whether this is the guild's Booster role
    pub premium_subscriber: Option<bool>,

    /// the id of this role's subscription sku and listing
    pub subscription_listing_id: Option<String>,

    /// whether this role is available for purchase
    pub available_for_purchase: Option<bool>,

    /// whether this role is a guild's linked role
    pub guild_connections: Option<bool>,
}
