const MAIN_URL: &str = "https://shinden.pl";
pub mod client;
pub mod episode;
pub mod player;
pub mod anime;
pub mod error;

pub use client::ShindenClient;
pub use episode::Episode;
pub use error::ShindenError;
