use reqwest::header::{HeaderMap, HeaderValue, COOKIE, HOST, USER_AGENT};
use reqwest::Client;

use self::shinden::anime::Anime;
use self::shinden::ShindenClient;
mod shinden;

#[tokio::main]
async fn main() {
    let url = "https://shinden.pl/series/68588-saigo-ni-hitotsu-dake-onegai-shitemo-yoroshii-deshou-ka/all-episodes";
    let second_url = "https://shinden.pl/series/7431-hagane-no-renkinjutsushi-2009/all-episodes";
    let player_url = "https://shinden.pl/episode/68588-saigo-ni-hitotsu-dake-onegai-shitemo-yoroshii-deshou-ka/view/253454";

    let client = ShindenClient::new().unwrap();
    let http = client.fetch_player(player_url).await.unwrap();

    print!("{:?}", http);

    //let anime = Anime::from_url(url, &client).unwrap().get_episodes(&client);
}
