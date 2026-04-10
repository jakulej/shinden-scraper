#![allow(dead_code, unused)]
use crate::shinden::Episode;

use self::shinden::anime::Anime;
use self::shinden::ShindenClient;
mod shinden;

#[tokio::main]
async fn main() {
    let _url = "https://shinden.pl/series/68588-saigo-ni-hitotsu-dake-onegai-shitemo-yoroshii-deshou-ka/all-episodes";
    let _second_url = "https://shinden.pl/series/7431-hagane-no-renkinjutsushi-2009/all-episodes";
    let _player_url = "https://shinden.pl/episode/68588-saigo-ni-hitotsu-dake-onegai-shitemo-yoroshii-deshou-ka/view/253454";
    let _third_player_url = "https://shinden.pl/episode/68484-youkoso-jitsuryoku-shijou-shugi-no-kyoushitsu-e-4th-season-2-nensei-hen-1-gakki/view/258137";

    let client = ShindenClient::new().unwrap();
    //let _anime = Anime::from_url(url, &client).await.unwrap();
    //let _episodes = anime.get_episodes(&client).await.unwrap();

    //let _http = client.fetch_player(_third_player_url).await.unwrap();
    let episode = Episode::new(_player_url.to_string());
    episode.download().await.unwrap();
    //print!("{:?}", episodes);

    //let anime = Anime::from_url(url, &client).unwrap().get_episodes(&client);
}
