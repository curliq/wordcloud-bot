
use std::fs::File;
use std::io::prelude::*;

#[derive(Deserialize)]
pub struct Config {
    pub discord_bot_token: String,
    pub prefix: String,
    pub expression_length: u16,
}


pub fn get_config() -> Config {
    let mut content = String::new();
    let _file = File::open("config.toml")
        .unwrap()
        .read_to_string(&mut content)
        .unwrap();
    return toml::from_str(&content.as_str()).expect("Failed to parse config.toml");
}
