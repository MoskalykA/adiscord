use crate::Snowflake;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Deserialize, Serialize, Debug)]
pub struct Change {
    /// New value of the key
    pub new_value: Option<Value>,

    /// Old value of the key
    pub old_value: Option<Value>,

    /// Name of the changed entity, with a few exceptions
    pub key: String,
}

#[repr(u8)]
#[allow(non_camel_case_types)]
#[derive(Deserialize_repr, Serialize_repr, Debug)]
pub enum Event {
    /// Server settings were updated
    GUILD_UPDATE = 1,

    /// Channel was created
    CHANNEL_CREATE = 10,

    /// Channel settings were updated
    CHANNEL_UPDATE,

    /// Channel was deleted
    CHANNEL_DELETE,

    /// Permission overwrite was added to a channel
    CHANNEL_OVERWRITE_CREATE,

    /// Permission overwrite was updated for a channel
    CHANNEL_OVERWRITE_UPDATE,

    /// Permission overwrite was deleted from a channel
    CHANNEL_OVERWRITE_DELETE,

    /// Member was removed from server
    MEMBER_KICK = 20,

    /// Members were pruned from server
    MEMBER_PRUNE,

    /// Member was banned from server
    MEMBER_BAN_ADD,

    /// Server ban was lifted for a member
    MEMBER_BAN_REMOVE,

    /// Member was updated in server
    MEMBER_UPDATE,

    /// Member was added or removed from a role
    MEMBER_ROLE_UPDATE,

    /// Member was moved to a different voice channel
    MEMBER_MOVE,

    /// Member was disconnected from a voice channel
    MEMBER_DISCONNECT,

    /// Bot user was added to server
    BOT_ADD,

    /// Role was created
    ROLE_CREATE = 30,

    /// Role was edited
    ROLE_UPDATE,

    /// Role was deleted
    ROLE_DELETE,

    /// Server invite was created
    INVITE_CREATE = 40,

    /// Server invite was updated
    INVITE_UPDATE,

    /// Server invite was deleted
    INVITE_DELETE,

    /// Webhook was created
    WEBHOOK_CREATE = 50,

    /// Webhook properties or channel were updated
    WEBHOOK_UPDATE,

    /// Webhook was deleted
    WEBHOOK_DELETE,

    /// Emoji was created
    EMOJI_CREATE = 60,

    /// Emoji name was updated
    EMOJI_UPDATE,

    /// Emoji was deleted
    EMOJI_DELETE,

    /// Single message was deleted
    MESSAGE_DELETE = 72,

    /// Multiple messages were deleted
    MESSAGE_BULK_DELETE,

    /// Message was pinned to a channel
    MESSAGE_PIN,

    /// Message was unpinned from a channel
    MESSAGE_UNPIN,

    /// App was added to server
    INTEGRATION_CREATE = 80,

    /// App was updated (as an example, its scopes were updated)
    INTEGRATION_UPDATE,

    /// App was removed from server
    INTEGRATION_DELETE,

    /// Stage instance was created (stage channel becomes live)
    STAGE_INSTANCE_CREATE,

    /// Stage instance details were updated
    STAGE_INSTANCE_UPDATE,

    /// Stage instance was deleted (stage channel no longer live)
    STAGE_INSTANCE_DELETE,

    /// Sticker was created
    STICKER_CREATE = 90,

    /// Sticker details were updated
    STICKER_UPDATE,

    /// Sticker was deleted
    STICKER_DELETE,

    /// Event was created
    GUILD_SCHEDULED_EVENT_CREATE = 100,

    /// Event was updated
    GUILD_SCHEDULED_EVENT_UPDATE,

    /// Event was cancelled
    GUILD_SCHEDULED_EVENT_DELETE,

    /// Thread was created in a channel
    THREAD_CREATE = 110,

    /// Thread was updated
    THREAD_UPDATE,

    /// Thread was deleted
    THREAD_DELETE,

    /// Permissions were updated for a command
    APPLICATION_COMMAND_PERMISSION_UPDATE = 121,

    /// Auto Moderation rule was created
    AUTO_MODERATION_RULE_CREATE = 140,

    /// Auto Moderation rule was updated
    AUTO_MODERATION_RULE_UPDATE,

    /// Auto Moderation rule was deleted
    AUTO_MODERATION_RULE_DELETE,

    /// Message was blocked by Auto Moderation
    AUTO_MODERATION_BLOCK_MESSAGE,

    /// Message was flagged by Auto Moderation
    AUTO_MODERATION_FLAG_TO_CHANNEL,

    /// Member was timed out by Auto Moderation
    AUTO_MODERATION_USER_COMMUNICATION_DISABLED,

    /// Creator monetization request was created
    CREATOR_MONETIZATION_REQUEST_CREATED = 150,

    /// Creator monetization terms were accepted
    CREATOR_MONETIZATION_TERMS_ACCEPTED,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct EntryInfo {
    /// ID of the app whose permissions were targeted
    pub application_id: Option<Snowflake>,

    /// Name of the Auto Moderation rule that was triggered
    pub auto_moderation_rule_name: Option<String>,

    /// Trigger type of the Auto Moderation rule that was triggered
    pub auto_moderation_rule_trigger_type: Option<String>,

    /// Channel in which the entities were targeted
    pub channel_id: Option<Snowflake>,

    /// Number of entities that were targeted
    pub count: Option<String>,

    /// Number of days after which inactive members were kicked
    pub delete_member_days: Option<String>,

    /// ID of the overwritten entity
    pub id: Option<Snowflake>,

    /// Number of members removed by the prune
    pub members_removed: Option<String>,

    /// ID of the message that was targeted
    pub message_id: Option<Snowflake>,

    /// Name of the role if type is "0" (not present if type is "1")
    pub role_name: Option<String>,

    /// Type of overwritten entity - role ("0") or member ("1")
    pub r#type: Option<String>,

    /// The type of integration which performed the action
    pub integration_type: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LogEntry {
    /// ID of the affected entity (webhook, user, role, etc.)
    pub target_id: Option<Snowflake>,

    /// Changes made to the target_id
    pub changes: Option<Vec<Change>>,

    /// User or app that made the changes
    pub user_id: Option<Snowflake>,

    /// ID of the entry
    pub id: Snowflake,

    /// Type of action that occurred
    pub action_type: Event,

    /// Additional info for certain event types
    pub options: Option<EntryInfo>,

    /// Reason for the change (1-512 characters)
    pub reason: Option<String>,
}
