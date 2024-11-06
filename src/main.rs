use ba2::{fo4::Archive, Reader};
use serde_derive::Deserialize;
use std::fs;
use std::path::Path;
//use std::process::exit;
use toml;
pub mod avp;
pub mod avp_data;

/*
use clap::Parser;
#[derive(Parser)]
struct CLI {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}
impl CLI {}
 */

fn main() {
    println!("[archive-version-patcher]\n");

    // parse config options
    let lang: avp_data::Language = parse_config();
    let msg: avp_data::Message = avp_data::Message::Default;

    //let args = CLI::parse();
    //std::println!("pattern: {:?}, path: {:?}", args.pattern, args.path);

    main_impl();
}

pub fn main_impl() -> Option<()> {
    let path: &Path = Path::new(r"./src/test_archives/fo4_tester.ba2");
    let archive: (ba2::fo4::Archive, ba2::fo4::ArchiveOptions) = Archive::read(path).ok()?;

    std::println!("name: {:?}", path.file_name().unwrap());
    std::println!("version: {:?}", avp::get_version(&archive));
    std::println!("needs patch: {:?}", avp::needs_patch(&archive));

    std::println!("\n");
    avp::display(&archive);

    std::println!("\n");
    avp::patch_version(archive);

    let path: &Path = Path::new(r"./src/test_archives/fo4_tester.ba2");
    let archive: (ba2::fo4::Archive, ba2::fo4::ArchiveOptions) = Archive::read(path).ok()?;
    std::println!("name: {:?}", path.file_name().unwrap());
    std::println!("version: {:?}", avp::get_version(&archive));
    std::println!("needs patch: {:?}", avp::needs_patch(&archive));

    return Some(());
}

// Top level struct to hold the TOML data.
#[derive(Deserialize)]
struct ConfigData {
    config: TomlConfig,
}
// Config struct holds to data from the `[config]` section.
#[derive(Deserialize)]
struct TomlConfig {
    language: u32,
}

pub fn parse_config() -> avp_data::Language {
    let config_toml: &str = "./config/avp_config.toml";
    let config_contents: String = match fs::read_to_string(config_toml) {
        Ok(c) => c,
        Err(_) => {
            todo!();
            //exit(1);
        }
    };
    let config_data: ConfigData = match toml::from_str(&config_contents) {
        Ok(d) => d,
        Err(_) => {
            todo!();
        }
    };

    match config_data.config.language {
        0 => return avp_data::Language::English,
        1 => return avp_data::Language::German,
        _ => return avp_data::Language::English,
    }
}
