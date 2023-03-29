use serde_repr::Deserialize_repr;

#[allow(non_camel_case_types)]
#[derive(Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum ChannelType {
    /// atext channel within a server
    GUILD_TEXT,

    /// a direct message between users
    DM,

    /// a voice channel within a server
    GUILD_VOICE,

    /// a direct message between multiple users
    GROUP_DM,

    /// an organizational category that contains up to 50 channels
    GUILD_CATEGORY,

    /// a channel that users can follow and crosspost into their own server (formerly news channels)
    GUILD_ANNOUNCEMENT,

    /// a temporary sub-channel within a GUILD_ANNOUNCEMENT channel
    ANNOUNCEMENT_THREAD = 10,

    /// a temporary sub-channel within a GUILD_TEXT or GUILD_FORUM channel
    PUBLIC_THREAD,

    /// a temporary sub-channel within a GUILD_TEXT channel that is only viewable by those invited and those with the MANAGE_THREADS permission
    PRIVATE_THREAD,

    /// a voice channel for hosting events with an audience
    GUILD_STAGE_VOICE,

    /// the channel in a hub containing the listed servers
    GUILD_DIRECTORY,

    /// Channel that can only contain threads
    GUILD_FORUM,
}
