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

#[derive(PartialEq)]
pub enum TokenType {
    Bot,
    Bearer,
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
    ///     let client = Client::new("10", dotenv!("TOKEN"), adiscord::TokenType::Bot);
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
    ///     let client = Client::new("10", dotenv!("TOKEN"), adiscord::TokenType::Bearer);
    /// }
    /// ```
    pub fn new(version: &str, token: &str, token_type: TokenType) -> Self {
        let url = format!("{BASE_URL}{version}");
        let client = reqwest::Client::new();
        let token = format!(
            "{} {token}",
            if token_type == TokenType::Bot {
                "Bot"
            } else {
                "Bearer"
            }
        );

        Self {
            url,
            token,
            client,

            #[cfg(feature = "gateway")]
            gateway: Gateway::new(),
        }
    }
}
