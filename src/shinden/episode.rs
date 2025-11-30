use reqwest::blocking::Client;
use scraper::Html;

use super::{ShindenClient, ShindenError};

pub struct Episode {
    html_document: Html,
}

impl Episode {
    pub fn from_url(url: String, client: &ShindenClient) -> Result<Self, ShindenError> {
        let html = client.fetch(&url).unwrap();
        let document = scraper::Html::parse_document(&html);
        Ok(Self {
            html_document: document,
        })
    }
    pub fn get_players(self, client: &ShindenClient) -> Result<Vec<Episode>, ShindenError> {
        let tr_selector = scraper::Selector::parse("tr").unwrap();
        self.html_document
            .select(&tr_selector)
            .skip(1)
            .map(|element| Player::from_url()
            .collect()
    }
}
