#![allow(dead_code)]
use core::slice;
use std::fmt::format;

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
        let row_selector = Selector::parse("li.ep-row").unwrap();
        let iter = self.html_document.select(&row_selector).skip(1);
        let episodes: Vec<Episode> = iter
            .map(|element| Episode::new(get_episode_url(element),get_episode_number(element)))
            .collect();
        Ok(episodes)
    }
    pub fn _select_episode(_episode_nr: u8) -> Result<Episode, ShindenError> {
        !todo!();
    }
}

fn parse_episode_name(anime_name: &String, episode_number: &usize) -> String {
    format!("{anime_name}/{episode_number}")
}


fn get_episode_url(element: ElementRef) -> String {
    let a_selector = Selector::parse("a.ep-row__link").unwrap();

    let href = element
        .select(&a_selector)
        .next()
        .unwrap()
        .value()
        .attr("href")
        .unwrap();

    MAIN_URL.to_string() + href
}
fn get_episode_number(element: ElementRef) -> usize {
    let no_selector = Selector::parse("span.ep-row__no").unwrap();

    element
        .select(&no_selector)
        .next()
        .unwrap()
        .text()
        .collect::<String>()
        .trim()
        .parse()
        .unwrap()
}