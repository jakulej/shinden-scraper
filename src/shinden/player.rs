#![allow(dead_code)]
use reqwest::Client;
use serde_json::Value;
use thirtyfour::prelude::*;

use super::{ShindenClient, ShindenError};

pub enum Player {
    CDA(WebElement),
    GOOGLE(WebElement),
    VK(WebElement),
}

impl Player {
    pub fn download() -> Result<(), ShindenError> {
        todo!();
    }
    pub fn get_element(&self) -> &WebElement {
        match self {
            Player::CDA(element) | Player::GOOGLE(element) | Player::VK(element) => element,
        }
    }

    pub async fn new(element: WebElement) -> Result<Option<Self>, WebDriverError> {
        let data = element.attr("data-episode").await?.unwrap();
        let json: Value = serde_json::from_str(&data).unwrap();
        let player = json["player"].as_str().unwrap();

        println!("Player: {}", player);

        match player {
            "Gdrive" => Ok(Some(Player::GOOGLE(element))),
            "Cda" => Ok(Some(Player::CDA((element)))),
            "Vk" => Ok(Some(Player::VK(element))),
            _ => Ok(None),
        }
    }
}
fn download_gdrive() -> Result<(), ShindenError> {
    Ok(())
}
