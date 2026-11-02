#![allow(dead_code, unused)]
mod shinden;

use std::{print, println};

use crate::shinden::Episode;

use self::shinden::anime::Anime;
use self::shinden::ShindenClient;

#[tokio::main]
async fn main() {
    let _url: &str = "https://shinden.pl/series/68588-saigo-ni-hitotsu-dake-onegai-shitemo-yoroshii-deshou-ka/all-episodes";

    let client = ShindenClient::new().unwrap();
    let _anime = Anime::from_url(_url, &client).await.unwrap();
    let _episodes = _anime.get_episodes(&client).await.unwrap();
    println!("{:?}",_episodes);
    let result = _episodes.get(1).unwrap().download("anime_dir").await;
    println!("Result {:?}",result.unwrap());

    // for episode in _episodes {
    //     let result = episode.download("").await;
    //     println!("Result: {:?}", result.unwrap());
    // }

    //let result = _episodes.first().unwrap().download("anime.mp4").await;

    //let _http = client.fetch_player(_third_player_url).await.unwrap();
    //let episode = Episode::new(_player_url.to_string());
    //episode.download().await.unwrap();
    //print!("{:?}", episodes);

    //let anime = Anime::from_url(url, &client).unwrap().get_episodes(&client);
}
