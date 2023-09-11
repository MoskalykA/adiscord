use super::{
    channel::{Attachment, Channel, Mention},
    embed::Embed,
    reaction::Reaction,
    sticker::{Item, Sticker},
};
use crate::{
    api::{application::Application, role::Role, user::User},
    Snowflake,
};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[repr(u8)]
#[allow(non_camel_case_types)]
#[derive(Deserialize_repr, Serialize_repr, Debug)]
pub enum ActivityType {
    JOIN = 1,
    SPECTATE,
    LISTEN,
    JOIN_REQUEST = 5,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Activity {
    /// type of message activity
    pub r#type: ActivityType,

    /// party_id from a Rich Presence event
    pub party_id: Option<Snowflake>,
}

#[repr(u8)]
#[allow(non_camel_case_types)]
#[derive(Deserialize_repr, Serialize_repr, Debug)]
pub enum InteractionType {
    PING = 1,
    APPLICATION_COMMAND,
    MESSAGE_COMPONENT,
    APPLICATION_COMMAND_AUTOCOMPLETE,
    MODAL_SUBMIT,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Interaction {
    /// ID of the interaction
    pub id: Snowflake,

    /// Type of interaction
    pub r#type: InteractionType,

    /// Name of the application command, including subcommands and subcommand groups
    pub name: String,

    /// User who invoked the interaction
    pub user: User,

    /// Member who invoked the interaction in the guild
    pub member: Option<User>,
}

#[repr(u8)]
#[derive(Deserialize_repr, Serialize_repr, Debug)]
pub enum Component {
    /// Container for other components
    ActionRow = 1,

    /// Button object
    Button,

    /// Select menu for picking from defined text options
    StringSelect,

    /// Text input object
    TextInput,

    /// Select menu for users
    UserSelect,

    /// Select menu for roles
    RoleSelect,

    /// Select menu for mentionables (users and roles)
    MentionableSelect,

    /// Select menu for channels
    ChannelSelect,
}

#[repr(u16)]
#[allow(non_camel_case_types)]
#[derive(Deserialize_repr, Serialize_repr, Debug)]
pub enum Flags {
    None,

    /// this message has been published to subscribed channels (via Channel Following)
    CROSSPOSTED = 1 << 0,

    /// this message originated from a message in another channel (via Channel Following)
    IS_CROSSPOST = 1 << 1,

    /// do not include any embeds when serializing this message
    SUPPRESS_EMBEDS = 1 << 2,

    /// the source message for this crosspost has been deleted (via Channel Following)
    SOURCE_MESSAGE_DELETED = 1 << 3,

    /// this message came from the urgent message system
    URGENT = 1 << 4,

    /// this message has an associated thread, with the same id as the message
    HAS_THREAD = 1 << 5,

    /// this message is only visible to the user who invoked the Interaction
    EPHEMERAL = 1 << 6,

    /// this message is an Interaction Response and the bot is "thinking"
    LOADING = 1 << 7,

    /// this message failed to mention some roles and add their members to the thread
    FAILED_TO_MENTION_SOME_ROLES_IN_THREAD = 1 << 8,

    /// this message will not trigger push and desktop notifications
    SUPPRESS_NOTIFICATIONS = 1 << 12,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Reference {
    /// id of the originating message
    pub message_id: Option<Snowflake>,

    /// id of the originating message's channel
    pub channel_id: Option<Snowflake>,

    /// id of the originating message's guild
    pub guild_id: Option<Snowflake>,

    /// when sending, whether to error if the referenced message doesn't exist instead of sending as a normal (non-reply) message, default true
    pub fail_if_not_exists: Option<bool>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RoleSubscriptionData {
    /// the id of the sku and listing that the user is subscribed to
    pub role_subscription_listing_id: Snowflake,

    /// the name of the tier that the user is subscribed to
    pub tier_name: String,

    /// the cumulative number of months that the user has been subscribed for
    pub total_months: u8,

    /// whether this notification is for a renewal rather than a new purchase
    pub is_renewal: bool,
}

#[repr(u8)]
#[allow(non_camel_case_types)]
#[derive(Deserialize_repr, Serialize_repr, Debug)]
pub enum Type {
    DEFAULT,
    RECIPIENT_ADD,
    RECIPIENT_REMOVE,
    CALL,
    CHANNEL_NAME_CHANGE,
    CHANNEL_ICON_CHANGE,
    CHANNEL_PINNED_MESSAGE,
    USER_JOIN,
    GUILD_BOOST,
    GUILD_BOOST_TIER_1,
    GUILD_BOOST_TIER_2,
    GUILD_BOOST_TIER_3,
    CHANNEL_FOLLOW_ADD,
    GUILD_DISCOVERY_DISQUALIFIED = 14,
    GUILD_DISCOVERY_REQUALIFIED,
    GUILD_DISCOVERY_GRACE_PERIOD_INITIAL_WARNING,
    GUILD_DISCOVERY_GRACE_PERIOD_FINAL_WARNING,
    THREAD_CREATED,
    REPLY,
    CHAT_INPUT_COMMAND,
    THREAD_STARTER_MESSAGE,
    GUILD_INVITE_REMINDER,
    CONTEXT_MENU_COMMAND,
    AUTO_MODERATION_ACTION,
    ROLE_SUBSCRIPTION_PURCHASE,
    INTERACTION_PREMIUM_UPSELL,
    STAGE_START,
    STAGE_END,
    STAGE_SPEAKER,
    STAGE_TOPIC,
    GUILD_APPLICATION_PREMIUM_SUBSCRIPTION,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
pub enum Nonce {
    Integer(u32),
    String(String),
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Message {
    /// id of the message
    pub id: Snowflake,

    /// id of the channel the message was sent in
    pub channel_id: Snowflake,

    /// the author of this message (not guaranteed to be a valid user, see below)
    pub author: User,

    /// contents of the message
    pub content: String,

    /// when this message was sent
    pub timestamp: String,

    /// when this message was edited (or null if never)
    pub edited_timestamp: Option<String>,

    /// whether this was a TTS message
    pub tts: bool,

    /// whether this message mentions everyone
    pub mention_everyone: bool,

    /// users specifically mentioned in the message
    pub mentions: Vec<User>,

    /// roles specifically mentioned in this message
    pub mention_roles: Vec<Role>,

    /// channels specifically mentioned in this message
    pub mention_channels: Option<Vec<Mention>>,

    /// any attached files
    pub attachments: Vec<Attachment>,

    /// any embedded content
    pub embeds: Vec<Embed>,

    /// reactions to the message
    pub reactions: Option<Vec<Reaction>>,

    /// used for validating a message was sent
    pub nonce: Option<Nonce>,

    /// whether this message is pinned
    pub pinned: bool,

    /// if the message is generated by a webhook, this is the webhook's id
    pub webhook_id: Option<Snowflake>,

    /// type of message
    pub r#type: Type,

    /// sent with Rich Presence-related chat embeds
    pub activity: Option<Activity>,

    /// sent with Rich Presence-related chat embeds
    pub application: Option<Application>,

    /// if the message is an Interaction or application-owned webhook, this is the id of the application    
    pub application_id: Option<Snowflake>,

    /// data showing the source of a crosspost, channel follow add, pin, or reply message
    pub message_reference: Option<Reference>,

    /// message flags combined as a bitfield
    pub flags: Option<Flags>,

    /// the message associated with the message_reference
    pub referenced_message: Option<Box<Message>>,

    /// sent if the message is a response to an Interaction
    pub interaction: Option<Interaction>,

    /// the thread that was started from this message, includes thread member object
    pub thread: Option<Channel>,

    /// sent if the message contains components like buttons, action rows, or other interactive components
    pub components: Option<Vec<Component>>,

    /// sent if the message contains stickers
    pub sticker_items: Option<Vec<Item>>,

    #[deprecated]
    /// the stickers sent with the message
    pub stickers: Option<Vec<Sticker>>,

    /// A generally increasing integer (there may be gaps or duplicates) that represents the approximate position of the message in a thread, it can be used to estimate the relative position of the message in a thread in company with total_message_sent on parent thread
    pub position: Option<u32>,

    /// data of the role subscription purchase or renewal that prompted this ROLE_SUBSCRIPTION_PURCHASE message
    pub role_subscription_data: Option<RoleSubscriptionData>,
}
