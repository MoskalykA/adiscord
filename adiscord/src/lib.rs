pub mod channels;
pub mod guilds;
pub mod voice;

#[cfg(feature = "gateway")]
pub mod gateway;

#[cfg(feature = "gateway")]
use gateway::types::Gateway;
use reqwest::header;
use serde::Deserialize;

const BASE_URL: &str = "https://discord.com/api/v10";

pub struct Client {
    #[cfg(feature = "gateway")]
    pub token: String,

    pub client: reqwest::Client,

    #[cfg(feature = "gateway")]
    pub gateway: Gateway,
}

#[derive(Deserialize, Debug)]
pub struct Error {
    pub code: u16,
    pub message: String,
}

impl Client {
    /// # Initiating the library
    ///
    /// This function will initiate the library.
    ///
    /// ## Examples
    ///
    /// With Gateway
    ///
    /// ```
    /// use adiscord::Client;
    /// use dotenv_codegen::dotenv;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Client::new(dotenv!("TOKEN"));
    /// }
    /// ```
    ///
    /// With API
    ///
    /// ```
    /// use adiscord::Client;
    /// use dotenv_codegen::dotenv;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Client::new(dotenv!("TOKEN"));
    /// }
    /// ```
    pub fn new(my_token: &str) -> Self {
        let mut token = String::with_capacity(76);
        token.push_str("Bot ");
        token.push_str(my_token);

        let mut headers = header::HeaderMap::new();
        headers.insert(
            "Authorization",
            header::HeaderValue::from_str(my_token).unwrap(),
        );

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        Self {
            #[cfg(feature = "gateway")]
            token,

            client,

            #[cfg(feature = "gateway")]
            gateway: Gateway::default(),
        }
    }
}
