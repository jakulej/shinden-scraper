#![allow(dead_code)]
use core::fmt;
use std::fmt::write;

use reqwest::Client;
use serde_json::Value;
use thirtyfour::prelude::*;

use super::{ShindenClient, ShindenError, PlayerKind};
#[derive(Debug)]
pub struct Player {
    pub kind: PlayerKind,
    pub button: WebElement,
}
impl fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kind)
    }
}
impl Player {
    pub async fn new(element: WebElement) -> Result<Option<Self>, WebDriverError> {
        let data = element.attr("data-episode").await?.unwrap();
        let json: Value = serde_json::from_str(&data).unwrap();
        let player_name = json["player"].as_str().unwrap();

        match PlayerKind::from_name(player_name) {
            Some(kind) => Ok(Some(Player { kind, button: element})),
            None => Ok(None)
        }
    }
}
