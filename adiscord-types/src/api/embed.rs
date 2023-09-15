use serde::{Deserialize, Serialize};
use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};

#[derive(Deserialize_enum_str, Serialize_enum_str, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Type {
    /// generic embed rendered from embed attributes
    Rich,

    /// image embed
    Image,

    /// video embed
    Video,

    /// animated gif image embed rendered as a video embed
    GifV,

    /// article embed
    Article,

    /// link embed
    Link,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Author {
    /// name of author
    pub name: String,

    /// url of author (only supports http(s))
    pub url: Option<String>,

    /// url of author icon (only supports http(s) and attachments)
    pub icon_url: Option<String>,

    /// a proxied url of author icon
    pub proxy_icon_url: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Field {
    /// name of the field
    pub name: String,

    /// value of the field
    pub value: String,

    /// whether or not this field should display inline
    pub inline: Option<bool>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Footer {
    /// footer text
    pub text: String,

    /// url of footer icon (only supports http(s) and attachments)
    pub icon_url: Option<String>,

    /// a proxied url of footer icon
    pub proxy_icon_url: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Image {
    /// source url of video
    pub url: String,

    /// a proxied url of the video
    pub proxy_url: Option<String>,

    /// height of video
    pub height: Option<u16>,

    /// width of video
    pub width: Option<u16>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Provider {
    /// name of provider
    pub name: Option<String>,

    /// url of provider
    pub url: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Thumbnail {
    /// source url of video
    pub url: String,

    /// a proxied url of the video
    pub proxy_url: Option<String>,

    /// height of video
    pub height: Option<u16>,

    /// width of video
    pub width: Option<u16>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Video {
    /// source url of video
    pub url: String,

    /// a proxied url of the video
    pub proxy_url: Option<String>,

    /// height of video
    pub height: Option<u16>,

    /// width of video
    pub width: Option<u16>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Embed {
    /// title of embed
    pub title: Option<String>,

    /// type of embed (always "rich" for webhook embeds)
    pub r#type: Option<Type>,

    /// description of embed
    pub description: Option<String>,

    /// url of embed
    pub url: Option<String>,

    /// timestamp of embed content
    pub timestamp: Option<String>,

    /// color code of the embed
    pub color: Option<u16>,

    /// footer information
    pub footer: Option<Footer>,

    /// image information
    pub image: Option<Image>,

    /// thumbnail information
    pub thumbnail: Option<Thumbnail>,

    /// video information
    pub video: Option<Video>,

    /// provider information
    pub provider: Option<Provider>,

    /// author information
    pub author: Option<Author>,

    /// fields information
    pub fields: Option<Vec<Field>>,
}
