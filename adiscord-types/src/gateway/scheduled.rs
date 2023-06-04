use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct UserAdd {
    /// ID of the guild scheduled event
    pub guild_scheduled_event_id: String,

    /// ID of the user
    pub user_id: String,

    /// ID of the guild
    pub guild_id: String,
}

#[derive(Deserialize, Debug)]
pub struct UserRemove {
    /// ID of the guild scheduled event
    pub guild_scheduled_event_id: String,

    /// ID of the user
    pub user_id: String,

    /// ID of the guild
    pub guild_id: String,
}
