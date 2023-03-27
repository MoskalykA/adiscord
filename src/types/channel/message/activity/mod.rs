pub mod r#type;

use self::r#type::ActivityType;
use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct MessageActivity {
    pub r#type: ActivityType,
    pub party_id: Option<String>,
}
