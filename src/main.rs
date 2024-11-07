// imports
use ba2::{fo4::Archive, Reader};
use eframe::egui;
use rfd::FileDialog;
use serde_derive::Deserialize;
use std::path::Path;
// modules
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
    name: String,
    #[serde(skip)] // opt-out of serialization of this field
    version: (u32, u32, u32), // semver 2.0.0
}
impl AppGUI {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}
impl Default for AppGUI {
    fn default() -> Self {
        Self {
            name: String::from("[archive-version-patcher]"),
            version: (0, 1, 0),
        }
    }
}

impl eframe::App for AppGUI {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        // top panel with options
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui: &mut egui::Ui| {
            egui::menu::bar(ui, |ui: &mut egui::Ui| {
                ui.menu_button("Program", |ui: &mut egui::Ui| {
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                ui.menu_button("Options", |ui: &mut egui::Ui| {
                    if ui.button("Quit_Temp").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                ui.menu_button("About", |ui: &mut egui::Ui| {
                    if ui.button("Quit_Temp").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                ui.add_space(16.0);
                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        // body of the gui
        egui::CentralPanel::default().show(ctx, |ui: &mut egui::Ui| {
            // section: title
            ui.heading("[archive-version-patcher]");
            ui.horizontal(|ui: &mut egui::Ui| {
                ui.label("by bp42s");
            });

            ui.separator();

            // section: archive selection
            ui.horizontal(|ui: &mut egui::Ui| {
                ui.label("Select an archive to patch:");
            });
            if ui
                .button(egui::RichText::new("Select Archive").color(egui::Color32::GREEN))
                .clicked()
            {
                let _ = avp::appgui_button_select_archive();
            }

            ui.add_space(7.5); // space between button selectors

            ui.horizontal(|ui: &mut egui::Ui| {
                ui.label("Select a directory to patch:");
            });
            if ui
                .button(egui::RichText::new("Select Directory").color(egui::Color32::GREEN))
                .clicked()
            {
                avp::appgui_button_select_directory()
            }

            // section: status/target/run
            ui.separator();
            ui.horizontal(|ui: &mut egui::Ui| {
                ui.label("Target archive name: ");
            });
            ui.horizontal(|ui: &mut egui::Ui| {
                ui.label("Patching progress:");
            });

            ui.separator();

            // section: footer
            ui.with_layout(
                egui::Layout::bottom_up(egui::Align::LEFT),
                |ui: &mut egui::Ui| {
                    ui.separator();
                    // these elements are displayed in reverse order
                    //appgui_footer(ui);
                    ui.hyperlink_to("MIT License", "https://mit-license.org/");
                    ui.hyperlink_to(
                        "Source",
                        "https://www.nexusmods.com/fallout4/mods/89051?tab=files",
                    );
                    ui.hyperlink_to(
                        "Nexus Page",
                        "https://www.nexusmods.com/fallout4/mods/89051",
                    );
                    ui.separator();
                    egui::warn_if_debug_build(ui);
                },
            );
        });
    }
}

fn main() {
    println!("[archive-version-patcher]\n");

    // parse config options
    let _lang: avp_data::Language = parse_config();

    let native_options: eframe::NativeOptions = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "[archive-version-patcher]",
        native_options,
        Box::new(|cc: &eframe::CreationContext<'_>| Ok(Box::new(AppGUI::new(cc)))),
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
    avp::to_string(&archive);

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
    let config_toml: &str = "avp_config.toml";
    let config_contents: String = match std::fs::read_to_string(config_toml) {
        Ok(c) => c,
        Err(_) => todo!(),
    };
    let config_data: ConfigData = match toml::from_str(&config_contents) {
        Ok(d) => d,
        Err(_) => todo!(),
    };

    match config_data.config.language {
        0 => return avp_data::Language::English,
        1 => return avp_data::Language::German,
        _ => return avp_data::Language::English,
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
