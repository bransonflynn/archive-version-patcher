use ba2::{fo4::Archive, Reader};
use serde_derive::Deserialize;
use std::fs;
use std::path::Path;
//use std::process::exit;
use eframe::egui;
use toml;
pub mod avp;
pub mod avp_data;

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

#[derive(Default)]
struct AppGUI {}

impl AppGUI {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for AppGUI {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui: &mut egui::Ui| {
            ui.heading("[archive-version-patcher]");
            if ui
                .button(egui::RichText::new("Select Archive").color(egui::Color32::GREEN))
                .clicked()
            {
                appgui_button_select_archive()
            }
            if ui
                .button(egui::RichText::new("Select Directory").color(egui::Color32::GREEN))
                .clicked()
            {
                appgui_button_select_directory()
            }
            ui.hyperlink_to(
                "Source",
                "https://github.com/bransonflynn/archive-version-patcher/",
            );
        });
    }
}

pub fn appgui_button_select_archive() {
    std::println!("Select Archive button clicked");
}

pub fn appgui_button_select_directory() {
    std::println!("Select Directory button clicked");
}

fn main() {
    println!("[archive-version-patcher]\n");

    // parse config options
    let _lang: avp_data::Language = parse_config();
    let _msg: avp_data::Message = avp_data::Message::Default;

    let native_options: eframe::NativeOptions = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "[archive-version-patcher]",
        native_options,
        Box::new(|cc| Ok(Box::new(AppGUI::new(cc)))),
    );

    //let args = CLI::parse();
    //std::println!("pattern: {:?}, path: {:?}", args.pattern, args.path);

    //main_impl();
}

#[allow(dead_code)]
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
