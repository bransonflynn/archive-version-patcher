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

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
struct AppGUI {
    // Example stuff:
    label: String,

    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,
}
impl AppGUI {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}
impl Default for AppGUI {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
        }
    }
}

impl eframe::App for AppGUI {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("[archive-version-patcher]");
            ui.horizontal(|ui| {
                ui.label("by bp42s");
            });

            ui.separator();

            ui.horizontal(|ui| {
                ui.label("Select an archive to patch:");
            });
            if ui
                .button(egui::RichText::new("Select Archive").color(egui::Color32::GREEN))
                .clicked()
            {
                appgui_button_select_archive()
            }
            ui.horizontal(|ui| {
                ui.label("Select a directory to patch:");
            });
            if ui
                .button(egui::RichText::new("Select Directory").color(egui::Color32::GREEN))
                .clicked()
            {
                appgui_button_select_directory()
            }

            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Patching progress:");
            });

            ui.separator();

            ui.with_layout(
                egui::Layout::bottom_up(egui::Align::LEFT),
                |ui: &mut egui::Ui| {
                    //appgui_footer(ui);
                    ui.hyperlink_to("Source", "https://www.nexusmods.com/fallout4/mods/89051");
                    ui.separator();
                    egui::warn_if_debug_build(ui);
                },
            );
        });
    }
}

// fn appgui_footer(ui: &mut egui::Ui) {
//     ui.horizontal(|ui| {
//         ui.spacing_mut().item_spacing.x = 0.0;
//         ui.label("Powered by ");
//         ui.hyperlink_to("egui", "https://github.com/emilk/egui");
//         ui.label(" and ");
//         ui.hyperlink_to(
//             "eframe",
//             "https://www.nexusmods.com/fallout4/mods/89051?tab=files",
//         );
//         ui.label(".");
//     });
// }

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
