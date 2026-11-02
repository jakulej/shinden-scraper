use super::downloader;
use super::webdrive_utils::{
    get_episode_url, init_shinden_cookies, setup_web_driver, wait_for_player_iframe,
};
use crate::shinden::player;
use crate::shinden::webdrive_utils::click_robust;

use super::statics::RODO_COOKIE;
use super::Player;
use super::PlayerKind;
use super::MAIN_URL;
use futures::future::join_all;
use std::{print, println};
use std::time::Duration;
use thirtyfour::common::capabilities::firefox::FirefoxPreferences;
use thirtyfour::prelude::*;

#[derive(Debug)]
pub struct Episode {
    url: String,
    number: usize,
}

impl Episode {
    pub fn new(url: String, number: usize) -> Self {
        Self { url, number }
    }

    pub async fn download(&self, output: &str) -> WebDriverResult<bool> {
        let driver: WebDriver = setup_web_driver("http://127.0.0.1:4444").await?;
        init_shinden_cookies(&driver).await?;

        driver.goto(self.url.clone()).await?;
        println!("Downloading Episode {}",self.number);
        let mut player_list = get_player_list(&driver).await?;
        println!("Found players {:#?}",player_list);
        
        let player = select_player(player_list)?;
        run_player(&player, &driver).await?;

        let output = format!("anime - {}.mp4",self.number);

        match player.kind {
            PlayerKind::RUMBLE => downloader::rumble(driver, &output).await,
            PlayerKind::GOOGLE => downloader::gdrive(driver, &output).await,
            _ => {
                driver.quit().await?;
                Ok(false)
            }
        }
    }
}
async fn get_player_list(driver: &WebDriver) -> WebDriverResult<Vec<Player>> {
    let players: Vec<WebElement> = driver.find_all(By::Css("[id^='player_data_']")).await?;

    let mut vector = Vec::new();

    for web_element in players {
        match Player::new(web_element).await? {
            Some(element) => vector.push(element),
            None => (),
        }
    }

    return Ok(vector);
}

async fn run_player(mut player: &Player, driver: &WebDriver) -> WebDriverResult<()> {
    println!("BUTTON: {:?}",player.button);
    click_robust(&driver, &player.button).await?;
    println!("Clicked player!");
    wait_for_player_iframe(&driver).await?;
    Ok(())
}

fn select_player(mut players: Vec<Player>) -> Result<Player, WebDriverError> {
    //Random player
    //let player = players.pop().unwrap();
    let player = players.pop().unwrap();
    println!("Selected player {}",player);
    Ok(player)
}
