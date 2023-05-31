use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Attachment {
    /// attachment id
    pub id: String,

    /// name of file attached
    pub filename: String,

    /// description for the file (max 1024 characters)
    pub description: Option<String>,

    /// the attachment's media type
    pub content_type: Option<String>,

    /// size of file in bytes
    pub size: u64,

    /// source url of file
    pub url: String,

    /// a proxied url of file
    pub proxy_url: String,

    /// height of file (if image)
    pub height: Option<u16>,

    /// width of file (if image)
    pub width: Option<u16>,

    /// whether this attachment is ephemeral
    pub ephemeral: Option<bool>,
}
