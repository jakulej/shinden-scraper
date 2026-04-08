use std::time::Duration;

use super::statics::RODO_COOKIE;
use super::ShindenError;
use super::MAIN_URL;
use reqwest::header::{
    HeaderMap, HeaderValue, ACCEPT, ACCEPT_LANGUAGE, COOKIE, HOST, REFERER, USER_AGENT,
};
use reqwest::Client;

pub struct ShindenClient {
    pub client: Client,
}

impl ShindenClient {
    pub fn new() -> Result<Self, ShindenError> {
        let client = Client::builder()
            .default_headers(append_headers())
            .build()
            .map_err(|_| ShindenError::NetworkError)?;

        Ok(Self { client })
    }

    pub async fn fetch(&self, url: &str) -> Result<String, reqwest::Error> {
        Ok(self.client.get(url).send().await?.text().await?)
    }
}
fn append_headers() -> HeaderMap {
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
fn append_headers_player() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.append(HOST, "api4.shinden.pl".parse().unwrap());
    headers.append(ACCEPT_LANGUAGE, "pl,en-US;q=0.7,en;q=0.3".parse().unwrap());
    headers.append(COOKIE, "cf_clearance=5Tkd_e2.698xiAbLQarF794nhyRdqx9rWIR5jNsyfZM-1764607698-1.2.1.1-nshovSr9WsxwkU54DU8HAf_7H_U0peZ2i6gIwjXrkUoYBo7u2PcANFU5F4RatW0WTo9s8ddDAsGkl_AuEo6WugwZUh.QerXKNZqynMFsJcYnjE29D6mpPb6AoQV2Z94hREnXRLWo74GQOTDF3RpgUq5DITfBypwTCwi1u8jK3McZhNokwTv6Zqrvpg6iKilkfPLgmagy3Uby3KHGRHMPf8ITA_qbPuo4P1VVPQ38mbw; api.shinden=s%3A155eYiEjD1mR-owG9PITMgk4ShC50y44.BRb0BvIBq9IYzLRUWMBhIc3gcdt6y%2Bp8BcRT8gGy1EI; _passenger_route=1290000822".parse().unwrap());
    headers.append(USER_AGENT, "Mozilla/5.0".parse().unwrap());
    headers.append("X-Requested-With", "XMLHttpRequest".parse().unwrap());
    headers.append(ACCEPT, "*/*".parse().unwrap());
    headers.append(REFERER, "https://shinden.pl/".parse().unwrap());
    headers
}
