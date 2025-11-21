use reqwest::blocking::Client;

use super::ShindenError;

struct Player{
    url: String,
    name: String
}

impl Player {
    pub fn from_url(url: &str, clinet: Client) -> Result<Self,ShindenError>{
        !todo!();
    }
}
