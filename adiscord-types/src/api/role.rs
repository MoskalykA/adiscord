use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Tags {
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

#[derive(Deserialize, Serialize, Debug)]
pub struct Role {
    /// role id
    pub id: String,

    /// role name
    pub name: String,

    /// integer representation of hexadecimal color code
    pub color: u32,

    /// if this role is pinned in the user listing
    pub hoist: bool,

    /// role icon hash
    pub icon: Option<String>,

    /// role unicode emoji
    pub unicode_emoji: Option<String>,

    /// position of this role
    pub position: u32,

    /// permission bit set
    pub permissions: String,

    /// whether this role is managed by an integration
    pub managed: bool,

    /// whether this role is mentionable
    pub mentionable: bool,

    /// the tags this role has
    pub tags: Option<Tags>,
}
