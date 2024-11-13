use serde_derive::Deserialize;

// top level struct to hold the TOML data.
#[derive(Deserialize)]
#[allow(dead_code)]
pub struct ConfigData {
    config_options: ConfigOptions,
}
impl ConfigData {}

// config struct holds to data from the `[config]` section.
#[derive(Deserialize)]
#[allow(dead_code)]
pub struct ConfigOptions {
    language: u32,
}
impl ConfigOptions {}

#[allow(dead_code)]
pub enum Language {
    English,
    German,
}
impl Language {}

pub fn get_language_int() -> u8 {
    // todo
    return 0;
}
