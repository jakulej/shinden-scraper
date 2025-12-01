use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, COOKIE, HOST, USER_AGENT};

use self::shinden::ShindenClient;
use self::shinden::anime::Anime;
mod shinden;

fn main() {
    let url = "https://shinden.pl/series/68588-saigo-ni-hitotsu-dake-onegai-shitemo-yoroshii-deshou-ka/all-episodes";
    let second_url = "https://shinden.pl/series/7431-hagane-no-renkinjutsushi-2009/all-episodes";
    let player_url = "https://api4.shinden.pl/xhr/1853498/player_show?auth=X2d1ZXN0XzowLDUsMjEwMDAwMDAsMjU1LDQxNzQyOTM2NDQ=&width=1055&height=-1";

    let client = ShindenClient::new().unwrap();
    let http = client.fetch_player(player_url).unwrap();

    print!("{:?}",http);

    //let anime = Anime::from_url(url, &client).unwrap().get_episodes(&client);
}
