use reqwest::blocking::Client; 
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT, HOST, COOKIE};

use self::shinden::ShindenClient; 
mod shinden;

fn main() {
    const MAIN_URL: &str = "https://shinden.pl/";
    const QUERY_URL: &str = "https://shinden.pl/series?search=mashoku";
    const ANIME_URL: &str = "https://shinden.pl/series/";

    let url = "https://shinden.pl/series/68588-saigo-ni-hitotsu-dake-onegai-shitemo-yoroshii-deshou-ka/episodes";

    let client = ShindenClient::new().unwrap();
    let anime = client.get_anime_from_url(url);


}

