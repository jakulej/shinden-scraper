use reqwest::blocking::Client;
use crate::shinden::anime::Anime;

use super::ShindenError;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT, HOST, COOKIE}; 

pub struct ShindenClient {
    client: Client,
}

impl ShindenClient {
    pub fn new() -> Result<Self,ShindenError>{
        let client = Client::builder()
        .default_headers(append_cookies())
        .build()
        .map_err(|_| ShindenError::NetworkError)?;

        Ok(Self{client})
    }

    pub fn get_anime_from_url(self: ShindenClient, url: &str) -> Result<Anime, ShindenError> {
        Anime::from_url(url, &self.client)
    }

}
    fn append_cookies() -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/101.0.4951.64 Safari/537.36 Edg/101.0.1210.47"));
        headers.insert(HOST, "shinden.pl".parse().unwrap());
        headers.insert(COOKIE,"_rnd=1763207037; sess_shinden=o6b625pejmuhcottqha915c5tp".parse().unwrap());

        headers
    }
