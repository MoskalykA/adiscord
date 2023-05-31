pub mod author;
pub mod field;
pub mod footer;
pub mod image;
pub mod provider;
pub mod thumbnail;
pub mod video;

use self::{
    author::EmbedAuthor, field::EmbedField, footer::EmbedFooter, image::EmbedImage,
    provider::EmbedProvider, thumbnail::EmbedThumbnail, video::EmbedVideo,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Embed {
    /// title of embed
    pub title: Option<String>,

    /// type of embed (always "rich" for webhook embeds)
    pub r#type: Option<String>,

    /// description of embed
    pub description: Option<String>,

    /// url of embed
    pub url: Option<String>,

    /// timestamp of embed content
    pub timestamp: Option<String>,

    /// color code of the embed
    pub color: Option<u16>,

    /// footer information
    pub footer: Option<EmbedFooter>,

    /// image information
    pub image: Option<EmbedImage>,

    /// thumbnail information
    pub thumbnail: Option<EmbedThumbnail>,

    /// video information
    pub video: Option<EmbedVideo>,

    /// provider information
    pub provider: Option<EmbedProvider>,

    /// author information
    pub author: Option<EmbedAuthor>,

    /// fields information
    pub fields: Option<Vec<EmbedField>>,
}
