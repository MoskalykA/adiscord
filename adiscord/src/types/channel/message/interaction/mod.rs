pub mod r#type;

use self::r#type::InteractionType;
use crate::types::user::User;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct MessageInteraction {
    /// ID of the interaction
    pub id: String,

    /// Type of interaction
    pub r#type: InteractionType,

    /// Name of the application command, including subcommands and subcommand groups
    pub name: String,

    /// User who invoked the interaction
    pub user: User,

    /// Member who invoked the interaction in the guild
    pub member: Option<User>,
}
