pub mod channels;
pub mod guilds;
pub mod voice;

#[cfg(feature = "gateway")]
pub mod gateway;

#[cfg(feature = "gateway")]
use gateway::types::Gateway;
use serde::Deserialize;

const BASE_URL: &str = "https://discord.com/api/v";

pub struct Client {
    pub url: String,
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
    ///     let client = Client::new("10", dotenv!("TOKEN"));
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
    ///     let client = Client::new("10", dotenv!("TOKEN"));
    /// }
    /// ```
    pub fn new(version: &str, token: &str) -> Self {
        let url = format!("{BASE_URL}{version}");
        let client = reqwest::Client::new();
        let token = format!("Bot {token}");

        Self {
            url,
            token,
            client,

            #[cfg(feature = "gateway")]
            gateway: Gateway::new(),
        }
    }
}
