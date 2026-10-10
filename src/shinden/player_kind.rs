#![allow(dead_code)]

use std::print;

pub enum PlayerKind {
    CDA, // html/css mpd
    GOOGLE, //maybe
    VK,
    MEGA,
    RUMBLE, // m3u8
    PIXELDRAIN, // Download api?
    //OKRU(WebElement),
    STREAMWISH,
    //CRUNCHYROLL(WebElement),
    //SIBNET(WebElement), // ??? TODO
    FILEMOON, // m3u8 - maybe some problems
    LYCORISCAFFE, //via pixeldrain, capture mp4 file after running
}

impl PlayerKind {
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "Gdrive" => Some(Self::GOOGLE),
            "Cda" => Some(Self::CDA),
            "Vk" => Some(Self::VK),
            "Mega" => Some(Self::MEGA),
            "Rumble" => Some(Self::RUMBLE),
            "Pixeldrain" => Some(Self::PIXELDRAIN),
            "Streamwish" => Some(Self::STREAMWISH),
            "Filemoon" => Some(Self::FILEMOON),
            "LycorisCafe" => Some(Self::LYCORISCAFFE),
            _ => None,
        }
    }
}