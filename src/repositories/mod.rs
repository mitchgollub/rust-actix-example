use crate::config::Config;
use eyre::Error;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Debug, Deserialize, Serialize)]
pub struct Movie {
    #[serde(rename(deserialize = "title"))]
    pub movie_name: String,
}

pub struct MoviesRepository {
    config: Config,
    client: Client,
}

impl MoviesRepository {
    pub fn new(config: &Config) -> Self {
        MoviesRepository {
            config: config.clone(),
            client: reqwest::Client::new(),
        }
    }

    #[tokio::main]
    pub async fn get(&self, movie_query: String) -> Result<Movie, Error> {
        let response = self
            .client
            .get(format!("{}{}", &self.config.movies_url, movie_query))
            .send()
            .await?
            .json::<Movie>()
            .await?;

        info!("Response: {:?}", response);

        Ok(response)
    }
}
