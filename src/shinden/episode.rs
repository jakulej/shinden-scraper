use super::statics::RODO_COOKIE;
use super::Player;
use super::MAIN_URL;
use futures::future::join_all;
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
        let players: Vec<WebElement> = driver.find_all(By::Css("[id^='player_data_']")).await?;

        let mut vector = Vec::new();

        for web_element in players {
            match Player::new(web_element).await? {
                Some(element) => vector.push(element),
                None => (),
            }
        }

        select_player(vector).await?;

        let url = get_episode_url(&driver).await?;
        println!("Episode url: {}", url);
        driver.quit().await?;

        Ok(url)
    }
}
async fn get_episode_url(driver: &WebDriver) -> Result<String, WebDriverError> {
    let parent = driver.find(By::Id("player-block")).await?;
    let iframe = parent
        .query(By::Css("iframe"))
        .wait(Duration::from_secs(10), Duration::from_millis(500))
        .first()
        .await?;
    let url = iframe.attr("src").await?;

    Ok(url.unwrap())
}
async fn select_player(players: Vec<Player>) -> Result<(), WebDriverError> {
    players.first().unwrap().get_element().click().await?;

    Ok(())
}
async fn setup_web_driver(server_url: &str) -> Result<WebDriver, WebDriverError> {
    let mut caps = DesiredCapabilities::firefox();

    let mut prefs = FirefoxPreferences::new();
    prefs.set("intl.accept_languages", "pl-PL,pl")?;
    prefs.set("intl.locale.requested", "pl-PL")?;

    caps.set_preferences(prefs)?;
    let driver = WebDriver::new(server_url, caps).await?;

    Ok(driver)
}

async fn init_shinden_cookies(driver: &WebDriver) -> Result<(), WebDriverError> {
    driver.goto(MAIN_URL).await?;

    driver
        .add_cookie(Cookie::new("cb-rodo", RODO_COOKIE))
        .await?;
    driver.refresh().await?;
    let btn = driver
        .find(By::XPath("//button[contains(., 'Zaakceptuj')]"))
        .await?;
    btn.click().await?;
    Ok(())
}
