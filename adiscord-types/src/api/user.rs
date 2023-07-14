use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(Deserialize_repr, Serialize_repr, Debug)]
pub enum Flags {
    Unknow,

    /// Discord Employee
    STAFF = 1 << 0,

    /// Partnered Server Owner
    PARTNER = 1 << 1,

    /// HypeSquad Events Member
    HYPESQUAD = 1 << 2,

    /// Bug Hunter Level 1
    BUG_HUNTER_LEVEL_1 = 1 << 3,

    /// House Bravery Member
    HYPESQUAD_ONLINE_HOUSE_1 = 1 << 6,

    /// House Brilliance Member
    HYPESQUAD_ONLINE_HOUSE_2 = 1 << 7,

    /// House Balance Member
    HYPESQUAD_ONLINE_HOUSE_3 = 1 << 8,

    /// Early Nitro Supporter
    PREMIUM_EARLY_SUPPORTER = 1 << 9,

    /// User is a team
    TEAM_PSEUDO_USER = 1 << 10,

    /// Bug Hunter Level 2
    BUG_HUNTER_LEVEL_2 = 1 << 14,

    /// Verified Bot
    VERIFIED_BOT = 1 << 16,

    /// Early Verified Bot Developer
    VERIFIED_DEVELOPER = 1 << 17,

    /// Moderator Programs Alumni
    CERTIFIED_MODERATOR = 1 << 18,

    /// Bot uses only HTTP interactions and is shown in the online member list
    BOT_HTTP_INTERACTIONS = 1 << 19,

    /// User is an Active Developer
    ACTIVE_DEVELOPER = 1 << 22,
}

#[repr(u8)]
#[allow(non_camel_case_types)]
#[derive(Deserialize_repr, Serialize_repr, Debug)]
pub enum PreniumType {
    NONE,
    NITRO_CLASSIC,
    NITRO,
    NITRO_BASIC,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct User {
    /// the user's id
    pub id: String,

    /// the user's username, not unique across the platform
    pub username: String,

    /// the user's 4-digit discord-tag
    pub discriminator: String,

    /// the user's avatar hash
    pub avatar: Option<String>,

    /// whether the user belongs to an OAuth2 application
    pub bot: Option<bool>,

    /// whether the user is an Official Discord System user (part of the urgent message system)
    pub system: Option<bool>,

    /// whether the user has two factor enabled on their account
    pub mfa_enabled: Option<bool>,

    /// the user's banner hash
    pub banner: Option<String>,

    /// the user's banner color encoded as an integer representation of hexadecimal color code
    pub accent_color: Option<u32>,

    /// the user's chosen language option
    pub locale: Option<String>,

    /// whether the email on this account has been verified
    pub verified: Option<bool>,

    /// the user's email
    pub email: Option<String>,

    /// the flags on a user's account
    pub flags: Option<Flags>,

    /// the type of Nitro subscription on a user's account
    pub premium_type: Option<PreniumType>,

    /// the public flags on a user's account
    pub public_flags: Option<Flags>,

    /// the user's avatar decoration hash
    pub avatar_decoration: Option<String>
}
