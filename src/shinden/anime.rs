#![allow(dead_code)]
use super::Episode;
use super::ShindenClient;
use super::ShindenError;
use super::MAIN_URL;
use scraper;
use scraper::ElementRef;
use scraper::Html;
use scraper::Selector;

pub struct Anime {
    pub name: String,
    pub episodes_number: u8,
    pub id: u8,
    html_document: Html,
}

type SelectEpisodeFn = fn(u8) -> Result<Episode, ShindenError>;

impl Anime {
    pub async fn from_url(url: &str, client: &ShindenClient) -> Result<Self, ShindenError> {
        let html = client.fetch(url).await.unwrap();
        let document = scraper::Html::parse_document(&html);
        Ok(Self {
            name: String::new(),
            episodes_number: 10u8,
            id: 100u8,
            html_document: document,
        })
    }
    pub async fn get_episodes(
        self: Anime,
        client: &ShindenClient,
    ) -> Result<Vec<Episode>, ShindenError> {
        let tr_selector = scraper::Selector::parse("tr").unwrap();
        let iter = self.html_document.select(&tr_selector).skip(1);
        let episodes: Vec<Episode> = iter
            .map(|element| Episode::new(get_episode_url(element)))
            .collect();
        Ok(episodes)
    }
    pub fn _select_episode(_episode_nr: u8) -> Result<Episode, ShindenError> {
        !todo!();
    }
}

fn get_episode_url(element: ElementRef) -> String {
    let td_selector = Selector::parse("td").unwrap();
    let a_selector = Selector::parse("a").unwrap();
    let button = element.select(&td_selector).last().unwrap();

    let url = MAIN_URL.to_string();
    let url = url
        + button
            .select(&a_selector)
            .next()
            .unwrap()
            .value()
            .attr("href")
            .unwrap();
    println!("Epsiode url: {}", url);
    url
}
