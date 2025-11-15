use reqwest::blocking::Client; 
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT, HOST, COOKIE}; 

fn main() {
    
    const MAIN_URL: &str = "https://shinden.pl/";
    const QUERY_URL: &str = "https://shinden.pl/series?search=";
    const ANIME_URL: &str = "https://shinden.pl/series/";

    let client = Client::builder()
        .default_headers(append_cookies())
        .build()
        .unwrap();

    
    let html_content = client.get(MAIN_URL)
        .send() 
        .unwrap();
    print!("{}",html_content.text().unwrap()); 
}

fn append_cookies() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/101.0.4951.64 Safari/537.36 Edg/101.0.1210.47"));
    headers.insert(HOST, "shinden.pl".parse().unwrap());
    headers.insert(COOKIE,"_rnd=1763207037; sess_shinden=o6b625pejmuhcottqha915c5tp".parse().unwrap());

    headers
}
