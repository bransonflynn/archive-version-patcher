use ba2::Reader;

#[derive(Clone)]
pub struct FalloutArchive<'a> {
    pub archive: ba2::fo4::Archive<'a>,
    pub options: ba2::fo4::ArchiveOptions,
}

impl<'a> FalloutArchive<'a> {}

// converts ba2 crate archive tuples into structs for easier handling and passing
pub fn tuple_to_struct<'a>(
    archive_tuple: &'a (ba2::fo4::Archive<'a>, ba2::fo4::ArchiveOptions),
) -> FalloutArchive<'a> {
    return FalloutArchive {
        archive: archive_tuple.0.clone(),
        options: archive_tuple.1,
    };
}

pub fn get_version(archive: &FalloutArchive) -> ba2::fo4::Version {
    return archive.options.version();
}

pub fn get_version_string(archive: &FalloutArchive) -> String {
    match archive.options.version() {
        ba2::fo4::Version::v1 => return String::from("v1"),
        ba2::fo4::Version::v7 => return String::from("v7"),
        ba2::fo4::Version::v8 => return String::from("v8"),
        ba2::fo4::Version::v2 => return String::from("v2"), // sf version, report error
        ba2::fo4::Version::v3 => return String::from("v3"), // sf version, report error
    }
}

pub fn needs_patch(archive: &FalloutArchive) -> bool {
    match archive.options.version() {
        ba2::fo4::Version::v1 => return false,
        ba2::fo4::Version::v7 => return true,
        ba2::fo4::Version::v8 => return true,
        ba2::fo4::Version::v2 => return false, // sf version, report error
        ba2::fo4::Version::v3 => return false, // sf version, report error
    }
}

pub fn patch_version_temp(path: &std::path::Path) {
    let Ok((_archive, options)) = ba2::fo4::Archive::read(path) else {
        todo!()
    };
    let version_number: ba2::fo4::Version = options.version();
    match version_number {
        ba2::fo4::Version::v1 => std::println!("no patch needed"),
        ba2::fo4::Version::v7 => {
            todo!();
            //options.OptionsBuilder.version(options, );
        }
        ba2::fo4::Version::v8 => todo!(),
        ba2::fo4::Version::v2 => std::println!("invalid ba2"), // sf version, report error
        ba2::fo4::Version::v3 => std::println!("invalid ba2 needed"), // sf version, report error
    }
}

pub fn patch_version(_archive: &FalloutArchive) {
    //let archive_options: ArchiveOptions = archive.1;
    //let old_version: Version = ba2::fo4::Version::v1;
    //let options_new: ArchiveOptions = ba2::fo4::ArchiveOptions::builder()
    //    .version(Version::v1)
    //    .build();
    //archive.1 = options_new;

    //let _ = ba2::fo4::FileWriteOptions::builder()
    //    .compression_format(ba2::fo4::CompressionFormat::Zip)
    //    .build();
}

pub fn appgui_button_select_archive() -> Option<()> {
    std::println!("Select Archive button clicked"); // temp

    let archive_path_buf: std::path::PathBuf = rfd::FileDialog::new()
        .add_filter("ba2", &["ba2"])
        .set_directory("/")
        .pick_file()?;
    let archive_path: &std::path::Path = archive_path_buf.as_path();
    let archive_tuple: (ba2::fo4::Archive<'_>, ba2::fo4::ArchiveOptions) =
        ba2::fo4::Archive::read(archive_path).ok()?;
    let archive_file: FalloutArchive<'_> = tuple_to_struct(&archive_tuple);
    let archive_name: &std::ffi::OsStr = std::path::Path::new(&archive_path).file_name().unwrap();
    //return Some(archive_file);

    // temp
    std::println!("archive_name: {:?}", archive_name);
    std::println!("archive_version: {:#?}", get_version(&archive_file));

    return Some(());
}

pub fn appgui_button_select_directory() {
    std::println!("Select Directory button clicked"); // temp
}

pub fn to_string(archive: &FalloutArchive) {
    // std::println!("name: {:?}", &archive); TODO
    std::println!("version: {:?}", get_version(&archive));
    std::println!("needs patch: {:?}", needs_patch(&archive));
}

// pub fn parse_config() -> avp_data::Language {
//     let config_toml: &str = "avp_config.toml";
//     let config_contents: String = match std::fs::read_to_string(config_toml) {
//         Ok(c) => c,
//         Err(_) => todo!(),
//     };
//     let config_data: ConfigData = match toml::from_str(&config_contents) {
//         Ok(d) => d,
//         Err(_) => todo!(),
//     };
// }
