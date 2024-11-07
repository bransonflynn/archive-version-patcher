use serde_derive::Deserialize;

// top level struct to hold the TOML data.
#[derive(Deserialize)]
pub struct ConfigData {
    config_options: ConfigOptions,
}
impl ConfigData {}

// config struct holds to data from the `[config]` section.
#[derive(Deserialize)]
pub struct ConfigOptions {
    language: u32,
}
impl ConfigOptions {}
