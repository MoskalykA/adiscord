pub mod tags;

use self::tags::RoleTags;
use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Role {
    pub id: String,
    pub name: String,
    pub color: i128,
    pub hoist: bool,
    pub icon: Option<String>,
    pub unicode_emoji: Option<String>,
    pub position: i128,
    pub permissions: String,
    pub managed: bool,
    pub mentionable: bool,
    pub tags: Option<RoleTags>,
}
