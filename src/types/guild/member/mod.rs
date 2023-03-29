pub mod flags;

use self::flags::MemberFlags;
use crate::types::user::User;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct GuildMember {
    /// the user this guild member represents
    pub user: Option<User>,

    /// this user's guild nickname
    pub nick: Option<String>,

    /// the member's guild avatar hash
    pub avatar: Option<String>,

    /// array of role object ids
    pub roles: Vec<String>,

    /// when the user joined the guild
    pub joined_at: String,

    /// when the user started boosting the guild
    pub premium_since: Option<String>,

    /// whether the user is deafened in voice channels
    pub deaf: bool,

    /// whether the user is muted in voice channels
    pub mute: bool,

    /// guild member flags represented as a bit set, defaults to 0
    pub flags: MemberFlags,

    /// whether the user has not yet passed the guild's Membership Screening requirements
    pub pending: Option<bool>,

    /// total permissions of the member in the channel, including overwrites, returned when in the interaction object
    pub permissions: Option<String>,

    /// when the user's timeout will expire and the user will be able to communicate in the guild again, null or a time in the past if the user is not timed out
    pub communication_disabled_until: Option<String>,
}
