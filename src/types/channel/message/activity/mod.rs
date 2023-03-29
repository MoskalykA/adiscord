pub mod r#type;

use self::r#type::ActivityType;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct MessageActivity {
    /// type of message activity
    pub r#type: ActivityType,

    /// party_id from a Rich Presence event
    pub party_id: Option<String>,
}
