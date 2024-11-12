// modules
pub mod avp;
pub mod avp_config;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
struct AppGUI {
    name: String,
    #[serde(skip)] // opt-out of serialization of this field
    version: (u32, u32, u32), // semver 2.0.0
    selected_archive_name: String,
    selected_archive_path: std::path::PathBuf,
    selected_dir_path: std::path::PathBuf,
    selected_dir_archive_count: u64,
    // add language field here to read from based on config
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
            selected_archive_name: String::from("Not specified"),
            selected_archive_path: Default::default(),
            selected_dir_path: String::from("Not specified").into(),
            selected_dir_archive_count: 0,
            // add language field here to read from based on config
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
                ui.menu_button("File", |_ui: &mut egui::Ui| {
                    // todo maybe: add open archive here
                    // todo maybe: add open dir here
                });
                ui.menu_button("Options", |_ui: &mut egui::Ui| {
                    //
                });
                ui.menu_button("About", |_ui: &mut egui::Ui| {
                    //
                });
                ui.menu_button("Quit", |_ui: &mut egui::Ui| {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
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
                ui.label("Target Archive: ".to_owned() + &self.selected_archive_name);
            });
            if ui
                .button(egui::RichText::new("Select Archive").color(egui::Color32::ORANGE))
                .clicked()
            {
                std::println!("Select Archive button clicked"); // temp
                let selected_archive: Option<avp::FalloutArchive<'_>> =
                    avp::appgui_button_select_archive();
                match selected_archive {
                    Some(arch) => {
                        std::println!("some: main_appgui_button_select_archive");
                        std::println!("{}", avp::needs_patch_archive(&arch));
                        std::println!("{:?}", avp::get_version(&arch));
                        self.selected_archive_name = avp::get_archive_name_path(&arch.path_buf);
                        let _ = avp::patch_version_test(&arch);
                    }
                    None => std::println!("error: main_appgui_button_select_archive"),
                }
            }

            ui.add_space(12.5); // space between button selectors

            ui.horizontal(|ui: &mut egui::Ui| {
                ui.label(
                    "Target Directory: ".to_owned()
                        + &self
                            .selected_dir_path
                            .clone()
                            .into_os_string()
                            .into_string()
                            .unwrap(),
                );
            });
            ui.horizontal(|ui: &mut egui::Ui| {
                ui.label(
                    "Archive Count: ".to_owned() + &self.selected_dir_archive_count.to_string(),
                );
            });
            if ui
                .button(egui::RichText::new("Select Directory").color(egui::Color32::ORANGE))
                .clicked()
            {
                std::println!("Select Directory button clicked"); // temp
                let selected_dir: Option<std::path::PathBuf> = avp::appgui_button_select_dir();
                match selected_dir {
                    Some(dir) => {
                        std::println!("some: main_appgui_button_select_directory");
                        std::println!("dir: {:?}", dir);
                        self.selected_dir_path = dir.clone();
                        //
                        self.selected_dir_archive_count = avp::count_archives_in_dir(&dir);
                        std::println!("{:#?}", avp::get_archives_in_dir(&dir));
                    }
                    None => std::println!("error none: main_appgui_button_select_directory"),
                }
            }

            ui.separator();

            // section: status/target/run
            //ui.add_space(15.0);

            if ui
                .button(egui::RichText::new("Patch").color(egui::Color32::GREEN))
                .clicked()
            {
                std::println!("Patch button clicked"); // temp
            }
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
    //let config_options = avp::parse_config();

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
