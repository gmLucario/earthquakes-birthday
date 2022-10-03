use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub url_data: String,
}
