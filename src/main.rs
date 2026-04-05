use reqwest::header::{HeaderMap, HeaderValue, COOKIE, HOST, USER_AGENT};
use reqwest::Client;

use self::shinden::anime::Anime;
use self::shinden::ShindenClient;
mod shinden;

#[tokio::main]
async fn main() {
    let url = "https://shinden.pl/series/68588-saigo-ni-hitotsu-dake-onegai-shitemo-yoroshii-deshou-ka/all-episodes";
    let _second_url = "https://shinden.pl/series/7431-hagane-no-renkinjutsushi-2009/all-episodes";
    let _player_url = "https://shinden.pl/episode/68588-saigo-ni-hitotsu-dake-onegai-shitemo-yoroshii-deshou-ka/view/253454";

    let client = ShindenClient::new().unwrap();
    //let _anime = Anime::from_url(url, &client).await.unwrap();
    //let _episodes = anime.get_episodes(&client).await.unwrap();

    let _http = client.fetch_player(_player_url).await.unwrap();

    //print!("{:?}", episodes);

    //let anime = Anime::from_url(url, &client).unwrap().get_episodes(&client);
}
