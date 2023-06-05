pub enum Intent {
    /// Exhaustive list of events included:
    ///
    /// - GUILD_UPDATE
    /// - GUILD_DELETE
    /// - GUILD_ROLE_CREATE
    /// - GUILD_ROLE_UPDATE
    /// - GUILD_ROLE_DELETE
    /// - CHANNEL_CREATE
    /// - CHANNEL_UPDATE
    /// - CHANNEL_DELETE
    /// - CHANNEL_PINS_UPDATE
    /// - THREAD_CREATE
    /// - THREAD_UPDATE
    /// - THREAD_DELETE
    /// - THREAD_LIST_SYNC
    /// - THREAD_MEMBER_UPDATE
    /// - THREAD_MEMBERS_UPDATE *
    /// - STAGE_INSTANCE_CREATE
    /// - STAGE_INSTANCE_UPDATE
    /// - STAGE_INSTANCE_DELETE
    Guilds = 1 << 0,

    /// Exhaustive list of events included:
    ///
    /// - GUILD_MEMBER_ADD
    /// - GUILD_MEMBER_UPDATE
    /// - GUILD_MEMBER_REMOVE
    /// - THREAD_MEMBERS_UPDATE *
    GuildMembers = 1 << 1,

    /// Exhaustive list of events included:
    ///
    /// - GUILD_AUDIT_LOG_ENTRY_CREATE
    /// - GUILD_BAN_ADD
    /// - GUILD_BAN_REMOVE
    GuildModeration = 1 << 2,

    /// Exhaustive list of events included:
    ///
    /// - GUILD_EMOJIS_UPDATE
    /// - GUILD_STICKERS_UPDATE
    GuildEmojisAndStickers = 1 << 3,

    /// Exhaustive list of events included:
    ///
    /// - GUILD_INTEGRATIONS_UPDATE
    /// - INTEGRATION_CREATE
    /// - INTEGRATION_UPDATE
    /// - INTEGRATION_DELETE
    GuildIntegrations = 1 << 4,

    /// Exhaustive list of events included:
    ///
    /// - WEBHOOKS_UPDATE
    GuildWebhooks = 1 << 5,

    /// Exhaustive list of events included:
    ///
    /// - INVITE_CREATE
    /// - INVITE_DELETE
    GuildInvites = 1 << 6,

    /// Exhaustive list of events included:
    ///
    /// - VOICE_STATE_UPDATE
    GuildVoiceStates = 1 << 7,

    /// Exhaustive list of events included:
    ///
    /// - PRESENCE_UPDATE
    GuildPresences = 1 << 8,

    /// Exhaustive list of events included:
    ///
    /// - MESSAGE_CREATE
    /// - MESSAGE_UPDATE
    /// - MESSAGE_DELETE
    /// - MESSAGE_DELETE_BULK
    GuildMessages = 1 << 9,

    /// Exhaustive list of events included:
    ///
    /// - MESSAGE_REACTION_ADD
    /// - MESSAGE_REACTION_REMOVE
    /// - MESSAGE_REACTION_REMOVE_ALL
    /// - MESSAGE_REACTION_REMOVE_EMOJI
    GuildMessageReactions = 1 << 10,

    /// Exhaustive list of events included:
    ///
    /// - TYPING_START
    GuildMessageTyping = 1 << 11,

    /// Exhaustive list of events included:
    ///
    /// - MESSAGE_CREATE
    /// - MESSAGE_UPDATE
    /// - MESSAGE_DELETE
    /// - CHANNEL_PINS_UPDATE
    DirectMessages = 1 << 12,

    /// Exhaustive list of events included:
    ///
    /// - MESSAGE_REACTION_ADD
    /// - MESSAGE_REACTION_REMOVE
    /// - MESSAGE_REACTION_REMOVE_ALL
    /// - MESSAGE_REACTION_REMOVE_EMOJI
    DirectMessageReactions = 1 << 13,

    /// Exhaustive list of events included:
    ///
    /// - TYPING_START
    DirectMessageTyping = 1 << 14,

    MessageContent = 1 << 15,

    /// Exhaustive list of events included:
    ///
    /// - GUILD_SCHEDULED_EVENT_CREATE
    /// - GUILD_SCHEDULED_EVENT_UPDATE
    /// - GUILD_SCHEDULED_EVENT_DELETE
    /// - GUILD_SCHEDULED_EVENT_USER_ADD
    /// - GUILD_SCHEDULED_EVENT_USER_REMOVE
    GuildScheduledEvents = 1 << 16,

    /// Exhaustive list of events included:
    ///
    /// - AUTO_MODERATION_RULE_CREATE
    /// - AUTO_MODERATION_RULE_UPDATE
    /// - AUTO_MODERATION_RULE_DELETE
    AutoModerationConfiguration = 1 << 20,

    /// Exhaustive list of events included:
    ///
    /// - AUTO_MODERATION_RULE_CREATE
    /// - AUTO_MODERATION_RULE_UPDATE
    /// - AUTO_MODERATION_RULE_DELETE
    AutoModerationExecution = 1 << 21,
}

/// Generate the number of itents from an intent list
pub fn generate_intent_number(intents: Vec<Intent>) -> u32 {
    let mut intent_number: u32 = 0;
    for intent in intents {
        intent_number += intent as u32;
    }

    intent_number
}
