use reqwest::Client;

use super::{ShindenClient, ShindenError};

pub struct Player {
    url: String,
    name: String,
}

impl Player {
    pub fn from_url(url: &str, client: &ShindenClient) -> Result<Self, ShindenError> {
        !todo!();
    }
}
