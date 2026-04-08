const MAIN_URL: &str = "https://shinden.pl";
pub mod anime;
pub mod client;
pub mod episode;
pub mod error;
pub mod player;
pub mod statics;

pub use client::ShindenClient;
pub use episode::Episode;
pub use error::ShindenError;
pub use player::Player;
