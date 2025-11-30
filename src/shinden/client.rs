use crate::shinden::anime::Anime;
use reqwest::blocking::Client;

use super::ShindenError;
use reqwest::header::{HeaderMap, HeaderValue, COOKIE, HOST, USER_AGENT};

pub struct ShindenClient {
    pub client: Client,
}

impl ShindenClient {
    pub fn new() -> Result<Self, ShindenError> {
        let client = Client::builder()
            .default_headers(append_cookies())
            .build()
            .map_err(|_| ShindenError::NetworkError)?;

        Ok(Self { client })
    }
    pub fn fetch(&self, url: &str) -> Result<String, ShindenError> {
        Ok(self.client
            .get(url)
            .send()
            .map_err(|_| ShindenError::NetworkError)?
            .text()
            .map_err(|_| ShindenError::NetworkError)?)
    }

}
fn append_cookies() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/101.0.4951.64 Safari/537.36 Edg/101.0.1210.47"));
    headers.insert(HOST, "shinden.pl".parse().unwrap());
    headers.insert(
        COOKIE,
        "_rnd=1763207037; sess_shinden=o6b625pejmuhcottqha915c5tp"
            .parse()
            .unwrap(),
    );

    headers
}
