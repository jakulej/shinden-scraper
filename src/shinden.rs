pub mod client;
pub mod episode;
pub mod player;
pub mod anime;
pub mod error;

pub use client::ShindenClient;
pub use episode::{EpisodeInfo, EpisodeId};
pub use player::PlayerData;
pub use error::ShindenError;
