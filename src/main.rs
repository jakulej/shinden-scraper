use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, COOKIE, HOST, USER_AGENT};

use self::shinden::ShindenClient;
use self::shinden::anime::Anime;
mod shinden;

fn main() {
    const MAIN_URL: &str = "https://shinden.pl/";
    const QUERY_URL: &str = "https://shinden.pl/series?search=mashoku";
    const ANIME_URL: &str = "https://shinden.pl/series/";

    let url = "https://shinden.pl/series/68588-saigo-ni-hitotsu-dake-onegai-shitemo-yoroshii-deshou-ka/all-episodes";
    let second_url = "https://shinden.pl/series/7431-hagane-no-renkinjutsushi-2009/all-episodes";

    let client = ShindenClient::new().unwrap();
    //let episodes =client.get_anime_from_url(url).unwrap()
      //  .fetch_episodes(&client.client);

    let anime = Anime::from_url(url, &client).unwrap().get_episodes(&client);
}
