use reqwest::blocking::Client;

use super::ShindenError;

struct Episode{
    pub name: String,
    pub episode_number: u8,
    id: u8,
}

impl Episode{
    pub fn from_url(html: &str, client: Client) -> Result<Self, ShindenError>{
        !todo!();
    }
}
