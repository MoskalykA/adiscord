use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Deserialize, Serialize, Debug)]
pub struct Assets {
    /// See Activity Asset Image
    pub large_image: Option<String>,

    /// Text displayed when hovering over the large image of the activity
    pub large_text: Option<String>,

    /// See Activity Asset Image
    pub small_image: Option<String>,

    /// Text displayed when hovering over the small image of the activity
    pub small_text: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
pub enum Button {
    Complete {
        /// Text shown on the button (1-32 characters)
        label: String,

        /// URL opened when clicking the button (1-512 characters)
        url: String,
    },
    Partial(String),
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Emoji {
    /// Name of the emoji
    pub name: String,

    /// ID of the emoji
    pub id: Option<String>,

    /// Whether the emoji is animated
    pub animated: Option<bool>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Party {
    /// ID of the party
    pub id: Option<String>,

    /// Used to show the party's current and maximum size
    pub size: Option<(u32, u32)>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Secrets {
    /// Secret for joining a party
    pub join: Option<String>,

    /// Secret for spectating a game
    pub spectate: Option<String>,

    /// Secret for a specific instanced match
    pub r#match: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Timestamps {
    /// Unix time (in milliseconds) of when the activity started
    pub start: Option<u64>,

    /// Unix time (in milliseconds) of when the activity ends
    pub end: Option<u64>,
}

#[repr(u8)]
#[allow(non_camel_case_types)]
#[derive(Deserialize_repr, Serialize_repr, Debug)]
pub enum Type {
    Game,
    Streaming,
    Listening,
    Watching,
    Custom,
    Competing,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Activity {
    /// Activity's name
    pub name: String,

    /// Activity type
    pub r#type: Type,

    /// Stream URL, is validated when type is 1
    pub url: Option<String>,

    /// Unix timestamp (in milliseconds) of when the activity was added to the user's session
    pub created_at: u64,

    /// Unix timestamps for start and/or end of the game
    pub timestamps: Option<Timestamps>,

    /// Application ID for the game
    pub application_id: Option<String>,

    /// What the player is currently doing
    pub details: Option<String>,

    /// User's current party status
    pub state: Option<String>,

    /// Emoji used for a custom status
    pub emoji: Option<Emoji>,

    /// Information for the current party of the player
    pub party: Option<Party>,

    /// Images for the presence and their hover texts
    pub assets: Option<Assets>,

    /// Secrets for Rich Presence joining and spectating
    pub secrets: Option<Secrets>,

    /// Whether or not the activity is an instanced game session
    pub instance: Option<bool>,

    /// Activity flags ORd together, describes what the payload includes
    pub flags: Option<u8>,

    /// Custom buttons shown in the Rich Presence (max 2)
    pub buttons: Option<Vec<Button>>,
}
