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
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Embed {
    pub title: Option<String>,
    pub r#type: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
    pub timestamp: Option<String>,
    pub color: Option<u16>,
    pub footer: Option<EmbedFooter>,
    pub image: Option<EmbedImage>,
    pub thumbnail: Option<EmbedThumbnail>,
    pub video: Option<EmbedVideo>,
    pub provider: Option<EmbedProvider>,
    pub author: Option<EmbedAuthor>,
    pub fields: Option<Vec<EmbedField>>,
}
