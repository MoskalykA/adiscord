use crate::{api::{application::Application, user::User}, Snowflake};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Create {
    /// Channel the invite is for
    pub channel_id: Snowflake,

    /// Unique invite code
    pub code: String,

    /// Time at which the invite was created
    pub created_at: String,

    /// Guild of the invite
    pub guild_id: Option<Snowflake>,

    /// User that created the invite
    pub inviter: Option<User>,

    /// How long the invite is valid for (in seconds)
    pub max_age: u16,

    /// Maximum number of times the invite can be used
    pub max_uses: u16,

    /// Type of target for this voice channel invite
    pub target_type: Option<u16>,

    /// User whose stream to display for this voice channel stream invite
    pub target_user: Option<User>,

    /// Embedded application to open for this voice channel embedded application invite
    pub target_application: Option<Application>,

    /// Whether or not the invite is temporary (invited users will be kicked on disconnect unless they're assigned a role)
    pub temporary: bool,

    /// How many times the invite has been used (always will be 0)
    pub uses: u8,
}

#[derive(Deserialize, Debug)]
pub struct Delete {
    /// Channel of the invite
    pub channel_id: Snowflake,

    /// Guild of the invite
    pub guild_id: Option<Snowflake>,

    /// Unique invite code
    pub code: String,
}
