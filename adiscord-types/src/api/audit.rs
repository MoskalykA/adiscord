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
    CHANNEL_UPDATE = 11,

    /// Channel was deleted
    CHANNEL_DELETE = 12,

    /// Permission overwrite was added to a channel
    CHANNEL_OVERWRITE_CREATE = 13,

    /// Permission overwrite was updated for a channel
    CHANNEL_OVERWRITE_UPDATE = 14,

    /// Permission overwrite was deleted from a channel
    CHANNEL_OVERWRITE_DELETE = 15,

    /// Member was removed from server
    MEMBER_KICK = 20,

    /// Members were pruned from server
    MEMBER_PRUNE = 21,

    /// Member was banned from server
    MEMBER_BAN_ADD = 22,

    /// Server ban was lifted for a member
    MEMBER_BAN_REMOVE = 23,

    /// Member was updated in server
    MEMBER_UPDATE = 24,

    /// Member was added or removed from a role
    MEMBER_ROLE_UPDATE = 25,

    /// Member was moved to a different voice channel
    MEMBER_MOVE = 26,

    /// Member was disconnected from a voice channel
    MEMBER_DISCONNECT = 27,

    /// Bot user was added to server
    BOT_ADD = 28,

    /// Role was created
    ROLE_CREATE = 30,

    /// Role was edited
    ROLE_UPDATE = 31,

    /// Role was deleted
    ROLE_DELETE = 32,

    /// Server invite was created
    INVITE_CREATE = 40,

    /// Server invite was updated
    INVITE_UPDATE = 41,

    /// Server invite was deleted
    INVITE_DELETE = 42,

    /// Webhook was created
    WEBHOOK_CREATE = 50,

    /// Webhook properties or channel were updated
    WEBHOOK_UPDATE = 51,

    /// Webhook was deleted
    WEBHOOK_DELETE = 52,

    /// Emoji was created
    EMOJI_CREATE = 60,

    /// Emoji name was updated
    EMOJI_UPDATE = 61,

    /// Emoji was deleted
    EMOJI_DELETE = 62,

    /// Single message was deleted
    MESSAGE_DELETE = 72,

    /// Multiple messages were deleted
    MESSAGE_BULK_DELETE = 73,

    /// Message was pinned to a channel
    MESSAGE_PIN = 74,

    /// Message was unpinned from a channel
    MESSAGE_UNPIN = 75,

    /// App was added to server
    INTEGRATION_CREATE = 80,

    /// App was updated (as an example, its scopes were updated)
    INTEGRATION_UPDATE = 81,

    /// App was removed from server
    INTEGRATION_DELETE = 82,

    /// Stage instance was created (stage channel becomes live)
    STAGE_INSTANCE_CREATE = 83,

    /// Stage instance details were updated
    STAGE_INSTANCE_UPDATE = 84,

    /// Stage instance was deleted (stage channel no longer live)
    STAGE_INSTANCE_DELETE = 85,

    /// Sticker was created
    STICKER_CREATE = 90,

    /// Sticker details were updated
    STICKER_UPDATE = 91,

    /// Sticker was deleted
    STICKER_DELETE = 92,

    /// Event was created
    GUILD_SCHEDULED_EVENT_CREATE = 100,

    /// Event was updated
    GUILD_SCHEDULED_EVENT_UPDATE = 101,

    /// Event was cancelled
    GUILD_SCHEDULED_EVENT_DELETE = 102,

    /// Thread was created in a channel
    THREAD_CREATE = 110,

    /// Thread was updated
    THREAD_UPDATE = 111,

    /// Thread was deleted
    THREAD_DELETE = 112,

    /// Permissions were updated for a command
    APPLICATION_COMMAND_PERMISSION_UPDATE = 121,

    /// Auto Moderation rule was created
    AUTO_MODERATION_RULE_CREATE = 140,

    /// Auto Moderation rule was updated
    AUTO_MODERATION_RULE_UPDATE = 141,

    /// Auto Moderation rule was deleted
    AUTO_MODERATION_RULE_DELETE = 142,

    /// Message was blocked by Auto Moderation
    AUTO_MODERATION_BLOCK_MESSAGE = 143,

    /// Message was flagged by Auto Moderation
    AUTO_MODERATION_FLAG_TO_CHANNEL = 144,

    /// Member was timed out by Auto Moderation
    AUTO_MODERATION_USER_COMMUNICATION_DISABLED = 145,

    /// Creator monetization request was created
    CREATOR_MONETIZATION_REQUEST_CREATED = 150,

    /// Creator monetization terms were accepted
    CREATOR_MONETIZATION_TERMS_ACCEPTED
}

#[derive(Deserialize, Serialize, Debug)]
pub struct EntryInfo {
    /// ID of the app whose permissions were targeted
    pub application_id: Option<String>,

    /// Name of the Auto Moderation rule that was triggered
    pub auto_moderation_rule_name: Option<String>,

    /// Trigger type of the Auto Moderation rule that was triggered
    pub auto_moderation_rule_trigger_type: Option<String>,

    /// Channel in which the entities were targeted
    pub channel_id: Option<String>,

    /// Number of entities that were targeted
    pub count: Option<String>,

    /// Number of days after which inactive members were kicked
    pub delete_member_days: Option<String>,

    /// ID of the overwritten entity
    pub id: Option<String>,

    /// Number of members removed by the prune
    pub members_removed: Option<String>,

    /// ID of the message that was targeted
    pub message_id: Option<String>,

    /// Name of the role if type is "0" (not present if type is "1")
    pub role_name: Option<String>,

    /// Type of overwritten entity - role ("0") or member ("1")
    pub r#type: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LogEntry {
    /// ID of the affected entity (webhook, user, role, etc.)
    pub target_id: Option<String>,

    /// Changes made to the target_id
    pub changes: Option<Vec<Change>>,

    /// User or app that made the changes
    pub user_id: Option<String>,

    /// ID of the entry
    pub id: String,

    /// Type of action that occurred
    pub action_type: Event,

    /// Additional info for certain event types
    pub options: Option<EntryInfo>,

    /// Reason for the change (1-512 characters)
    pub reason: Option<String>,
}
