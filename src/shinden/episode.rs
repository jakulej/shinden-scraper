use super::webdrive_utils::{
    get_episode_url, init_shinden_cookies, setup_web_driver, wait_for_player_iframe,
};
use crate::shinden::player;
use crate::shinden::webdrive_utils::click_robust;
use super::downloader;

use super::statics::RODO_COOKIE;
use super::Player;
use super::PlayerKind;
use super::MAIN_URL;
use futures::future::join_all;
use std::print;
use std::time::Duration;
use thirtyfour::common::capabilities::firefox::FirefoxPreferences;
use thirtyfour::prelude::*;

pub struct Episode {
    url: String,
}

impl Episode {
    pub fn new(url: String) -> Self {
        Self { url }
    }

    pub async fn download(&self) -> Result<String, WebDriverError> {
        let driver: WebDriver = setup_web_driver("http://127.0.0.1:4444").await?;
        init_shinden_cookies(&driver).await?;

        driver.goto(self.url.clone()).await?;

        let mut player_list = get_player_list(&driver).await?;

        let player = select_player(player_list)?;
        player.button.click().await?;
        click_robust(&driver, &player.button).await?;

        wait_for_player_iframe(&driver).await?;
        let output = "anime.mp4";

        match player.kind {
            PlayerKind::RUMBLE => downloader::rumble(&driver, output).await?,
            PlayerKind::GOOGLE => downloader::gdrive(&driver, output).await?,
            _ => ()
        }
        //let url = get_episode_url(&driver).await?;
        //println!("Episode url: {}", url);
        driver.quit().await?;

        //print!("Hello {:?}", url);
        Ok("Hello".to_string())
    }
}
async fn get_player_list(driver: &WebDriver) -> Result<Vec<Player>, WebDriverError> {
    let players: Vec<WebElement> = driver.find_all(By::Css("[id^='player_data_']")).await?;

    let mut vector = Vec::new();

    for web_element in players {
        match Player::new(web_element).await? {
            Some(element) => vector.push(element),
            None => (),
        }
    }

    return Ok(vector)
}

fn select_player(mut players: Vec<Player>) -> Result<Player, WebDriverError> {
    //Random player
    //let player = players.pop().unwrap();
    let player = players.pop().unwrap();
    Ok(player)
}
