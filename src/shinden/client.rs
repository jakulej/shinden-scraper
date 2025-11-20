use reqwest::blocking::Client;
use super::ShindenError;

pub struct ShindenClient {
    client: Client,
    base_url: String,
}

impl ShindenClient {
    pub fn new() -> Result<Self,ShindenError>{
        !todo!();
    }

}
