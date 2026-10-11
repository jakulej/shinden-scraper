use super::webdrive_utils::get_gdrive_url;
use super::yt_dlp;
use std::process::Command;
use std::time::Duration;

use thirtyfour::{
    error::{WebDriverError, WebDriverResult},
    extensions::query::ElementQueryable,
    By, WebDriver,
};
use tokio::time::Instant;

pub async fn rumble(driver: &WebDriver, output: &str) -> WebDriverResult<()> {
    let m3u8_url = get_m3u8_url(driver).await?;
    yt_dlp::download_m3u8(&m3u8_url, output);
    Ok(())
}
pub async fn gdrive(driver: &WebDriver, output: &str) -> WebDriverResult<()> {
    let url = get_gdrive_url(driver).await?;
    yt_dlp::download_drive(&url, output);
    Ok(())
}

async fn get_m3u8_url(driver: &WebDriver) -> WebDriverResult<String> {
    let iframe = driver
        .query(By::Css("#player-block iframe[src^='http']"))
        .first()
        .await?;
    iframe.enter_frame().await?;

    driver
        .execute("performance.setResourceTimingBufferSize(1000);", vec![])
        .await?;

    let start = Instant::now();
    let found = loop {
        let ret = driver
            .execute(
                r#"return performance.getEntriesByType('resource')
                           .map(e => e.name)
                           .filter(n => n.includes('.m3u8'));"#,
                vec![],
            )
            .await?;
        let urls: Vec<String> = ret.convert()?;

        if let Some(u) = urls.into_iter().next() {
            break Some(u);
        }
        if start.elapsed() > Duration::from_secs(30) {
            break None;
        }
        tokio::time::sleep(Duration::from_millis(500)).await;
    };

    driver.enter_default_frame().await?;

    found.ok_or_else(|| WebDriverError::Timeout("nie znaleziono m3u8 w 30 s".to_string()))
}
