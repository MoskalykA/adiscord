use crate::{types::guild::member::GuildMember, Error};
use reqwest::StatusCode;
use serde_derive::Serialize;

#[derive(Serialize)]
pub struct Query {
    /// Max number of members to return (1-1000)
    pub limit: Option<u16>,

    /// The highest user id in the previous page
    pub after: Option<String>,
}

impl crate::Guild {
    /// # Examples
    ///
    /// ```
    /// match client.guild.get_members("1089521338286342195", None).await {
    ///     Ok(members) => println!("{:?}", members),
    ///     Err(error) => println!("{:?}", error),
    /// };
    /// ```
    /// ## With a filter
    ///
    /// ```
    /// match client
    ///     .guild
    ///     .get_members(
    ///         "1089521338286342195",
    ///         Some(adiscord::guild::get_members::Query {
    ///             limit: Some(100),
    ///             after: None,
    ///         }),
    ///     )
    ///     .await
    /// {
    ///     Ok(members) => println!("{:?}", members),
    ///     Err(error) => println!("{:?}", error),
    /// };
    /// ```
    pub async fn get_members(
        &self,
        index: &str,
        query: Option<Query>,
    ) -> Result<Vec<GuildMember>, Error> {
        let response = self
            .client
            .get(format!("{}/guilds/{index}/members", self.url))
            .header("Authorization", self.token.clone())
            .query(&query)
            .send()
            .await
            .unwrap();

        let status = response.status();
        match status {
            StatusCode::OK => {
                let body: Vec<GuildMember> = response.json().await.unwrap();
                Ok(body)
            }
            _ => {
                let body: Error = response.json().await.unwrap();
                Err(body)
            }
        }
    }
}
