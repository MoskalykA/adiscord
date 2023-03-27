pub mod r#type;

use self::r#type::InteractionType;
use crate::types::user::User;
use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct MessageInteraction {
    pub id: String,
    pub r#type: InteractionType,
    pub name: String,
    pub user: User,
    pub member: Option<User>,
}
