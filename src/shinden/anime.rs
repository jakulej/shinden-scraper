use super::ShindenError;
use reqwest::blocking::Client;
use scraper;

pub struct Anime{
    pub name: String,
    pub episodes_number: u8,
    pub id: u8,
    
}

impl Anime {
    pub fn from_url(url: &str, client: Client) -> Result<Self, ShindenError>{
        !todo!();
    }

}
