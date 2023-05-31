pub mod r#type;

use self::r#type::ActivityType;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct MessageActivity {
    /// type of message activity
    pub r#type: ActivityType,

    /// party_id from a Rich Presence event
    pub party_id: Option<String>,
}
