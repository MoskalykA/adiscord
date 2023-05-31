pub mod tags;

use self::tags::RoleTags;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Role {
    /// role id
    pub id: String,

    /// role name
    pub name: String,

    /// integer representation of hexadecimal color code
    pub color: i128,

    /// if this role is pinned in the user listing
    pub hoist: bool,

    /// role icon hash
    pub icon: Option<String>,

    /// role unicode emoji
    pub unicode_emoji: Option<String>,

    /// position of this role
    pub position: i128,

    /// permission bit set
    pub permissions: String,

    /// whether this role is managed by an integration
    pub managed: bool,

    /// whether this role is mentionable
    pub mentionable: bool,

    /// the tags this role has
    pub tags: Option<RoleTags>,
}
