use super::MAIN_URL;
use super::ShindenError;
use reqwest::blocking::Client;
use super::Episode;
use scraper;

pub struct Anime{
    pub name: String,
    pub episodes_number: u8,
    pub id: u8,
    
}

impl Anime {
    pub fn from_url(url: &str, client: &Client) -> Result<Self, ShindenError>{
        let html = client
            .get(url)
            .send()
            .map_err(|_| ShindenError::NetworkError)?
            .text()
            .map_err(|_| ShindenError::NetworkError)?;
        let document = scraper::Html::parse_document(&html);
        let tr_selector = scraper::Selector::parse("tr").unwrap();
        document.select(&tr_selector).for_each(|element| println!("{:?}", element.value()));
        
        Ok(Self{name: String::new() , episodes_number: 10u8 , id:100u8 })

    }
    pub fn select_episode(episode_nr: u8) -> Result<Episode, ShindenError> {
        !todo!();
    }

}
