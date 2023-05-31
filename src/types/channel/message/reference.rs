use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct MessageReference {
    /// id of the originating message
    pub message_id: Option<String>,

    /// id of the originating message's channel
    pub channel_id: Option<String>,

    /// id of the originating message's guild
    pub guild_id: Option<String>,

    /// when sending, whether to error if the referenced message doesn't exist instead of sending as a normal (non-reply) message, default true
    pub fail_if_not_exists: Option<bool>,
}
