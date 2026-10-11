use super::statics::RODO_COOKIE;
use super::Player;
use super::PlayerKind;
use super::MAIN_URL;
use futures::future::join_all;
use std::print;
use std::time::Duration;
use thirtyfour::common::capabilities::firefox::FirefoxPreferences;
use thirtyfour::prelude::*;

pub async fn setup_web_driver(server_url: &str) -> Result<WebDriver, WebDriverError> {
    let mut caps = DesiredCapabilities::firefox();

    let mut prefs = FirefoxPreferences::new();
    prefs.set("intl.accept_languages", "pl-PL,pl")?;
    prefs.set("intl.locale.requested", "pl-PL")?;

    caps.set_preferences(prefs)?;
    let driver = WebDriver::new(server_url, caps).await?;

    Ok(driver)
}

pub async fn init_shinden_cookies(driver: &WebDriver) -> Result<(), WebDriverError> {
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

pub async fn get_episode_url(driver: &WebDriver) -> Result<String, WebDriverError> {
    let parent = driver.find(By::Id("player-block")).await?;
    let iframe = parent
        .query(By::Css("iframe"))
        .wait(Duration::from_secs(10), Duration::from_millis(500))
        .first()
        .await?;
    let url = iframe.attr("src").await?;

    Ok(url.unwrap())
}

pub async fn click_robust(driver: &WebDriver, el: &WebElement) -> WebDriverResult<()> {
    el.scroll_into_view().await?;

    if el.click().await.is_ok() {
        return Ok(());
    }

    driver
        .execute("arguments[0].click();", vec![el.to_json()?])
        .await?;
    Ok(())
}

pub async fn wait_for_player_iframe(driver: &WebDriver) -> WebDriverResult<()> {
    driver
        .query(By::Css("#player-block iframe[src^='http']"))
        .wait(Duration::from_secs(30), Duration::from_millis(250))
        .first()
        .await?;
    Ok(())
}
pub async fn get_gdrive_url(driver: &WebDriver) -> WebDriverResult<String> {
    let iframe = driver
        .query(By::Css("#player-block iframe[src*='drive.google.com']"))
        .wait(Duration::from_secs(30), Duration::from_millis(250))
        .first()
        .await?;

    let src = iframe.attr("src").await?.unwrap_or_default();
    Ok(src)
}
