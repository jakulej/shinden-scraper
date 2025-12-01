use reqwest::blocking::Client;
use scraper::ElementRef;
use scraper::Html;
use scraper::Selector;
use super::Player;

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
    pub fn get_players(self, client: &ShindenClient) -> Result<Vec<Player>, ShindenError> {

        let tr_selector = scraper::Selector::parse("tr").unwrap();

        self.html_document
            .select(&tr_selector)
            .skip(1)
            .map(|element| Player::from_url(&get_player_url(element), client))
            .collect()
    }
}
fn get_player_url(element: ElementRef) -> String {
    let td_selector = Selector::parse("td").unwrap();
    let a_selector = Selector::parse("a").unwrap();
    let button = element.select(&td_selector).last().unwrap();
        
    let url = String::new();

    url
}
