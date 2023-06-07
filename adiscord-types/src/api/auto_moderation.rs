use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_repr::{Deserialize_repr, Serialize_repr};

#[repr(u8)]
#[allow(non_camel_case_types)]
#[derive(Deserialize_repr, Serialize_repr, Debug)]
pub enum EventType {
    /// when a member sends or edits a message in the guild
    MESSAGE_SEND = 1,
}

#[repr(u8)]
#[allow(non_camel_case_types)]
#[derive(Deserialize_repr, Serialize_repr, Debug)]
pub enum TriggerType {
    /// check if content contains words from a user defined list of keywords
    KEYWORD = 1,

    /// check if content represents generic spam
    SPAM = 3,

    /// check if content contains words from internal pre-defined wordsets
    KEYWORD_PRESET,

    /// check if content contains more unique mentions than allowed
    MENTION_SPAM,
}

#[repr(u8)]
#[allow(non_camel_case_types)]
#[derive(Deserialize_repr, Serialize_repr, Debug)]
pub enum KeywordPreset {
    /// words that may be considered forms of swearing or cursing
    PROFANITY = 1,

    /// words that refer to sexually explicit behavior or activity
    SEXUAL_CONTENT,

    /// personal insults or words that may be considered hate speech
    SLURS,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TriggerMetadata {
    /// substrings which will be searched for in content (Maximum of 1000)
    pub keyword_filter: Option<Vec<String>>,

    /// regular expression patterns which will be matched against content (Maximum of 10)
    pub regex_patterns: Option<Vec<String>>,

    /// the internally pre-defined wordsets which will be searched for in content
    pub presets: Option<Vec<KeywordPreset>>,

    /// substrings which should not trigger the rule (Maximum of 100 or 1000)
    pub allow_list: Option<Vec<String>>,

    /// total number of unique role and user mentions allowed per message (Maximum of 50)
    pub mention_total_limit: Option<u8>,

    /// whether to automatically detect mention raids
    pub mention_raid_protection_enabled: Option<bool>,
}

#[repr(u8)]
#[allow(non_camel_case_types)]
#[derive(Deserialize_repr, Serialize_repr, Debug)]
pub enum ActionType {
    /// blocks a member's message and prevents it from being posted. A custom explanation can be specified and shown to members whenever their message is blocked.
    BLOCK_MESSAGE = 1,

    /// logs user content to a specified channel
    SEND_ALERT_MESSAGE,

    /// timeout user for a specified duration *
    TIMEOUT,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Action {
    /// action type	the type of action
    r#type: ActionType,

    /// additional metadata needed during execution for this specific action type
    metadata: Option<Value>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Rule {
    /// the id of this rule
    pub id: String,

    /// the id of the guild which this rule belongs to
    pub guild_id: String,

    /// the rule name
    pub name: String,

    /// the user which first created this rule
    pub creator_id: String,

    /// the rule event type
    pub event_type: EventType,

    /// the rule trigger type
    pub trigger_type: TriggerType,

    /// the rule trigger metadata
    pub trigger_metadata: TriggerMetadata,

    /// the actions which will execute when the rule is triggered
    pub actions: Vec<Action>,

    /// whether the rule is enabled
    pub enabled: bool,

    /// the role ids that should not be affected by the rule (Maximum of 20)
    pub exempt_roles: Vec<String>,

    /// the channel ids that should not be affected by the rule (Maximum of 50)
    pub exempt_channels: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Execution {
    /// ID of the guild in which action was executed
    pub guild_id: String,

    /// Action which was executed
    pub action: Action,

    /// ID of the rule which action belongs to
    pub rule_id: String,

    /// Trigger type of rule which was triggered
    pub rule_trigger_type: TriggerType,

    /// ID of the user which generated the content which triggered the rule
    pub user_id: String,

    /// ID of the channel in which user content was posted
    pub channel_id: Option<String>,

    /// ID of any user message which content belongs to *
    pub message_id: Option<String>,

    /// ID of any system auto moderation messages posted as a result of this action **
    pub alert_system_message_id: Option<String>,

    /// User-generated text content
    pub content: String,

    /// Word or phrase configured in the rule that triggered the rule
    pub matched_keyword: Option<String>,

    /// Substring in content that triggered the rule
    pub matched_content: Option<String>,
}
