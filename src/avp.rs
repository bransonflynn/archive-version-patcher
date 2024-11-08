use ba2::Reader;

#[derive(Clone)]
pub struct FalloutArchive<'a> {
    pub archive: ba2::fo4::Archive<'a>,
    pub options: ba2::fo4::ArchiveOptions,
    pub path_buf: std::path::PathBuf,
}

impl<'a> Default for FalloutArchive<'a> {
    fn default() -> Self {
        Self { archive: Default::default(), options: Default::default(), path_buf: Default::default() }
    }
}
impl<'a> FalloutArchive<'a> {}

// converts ba2 crate archive tuples into structs for easier handling and passing
pub fn create_archive_struct<'a>(
    archive_tuple: (ba2::fo4::Archive<'a>, ba2::fo4::ArchiveOptions),
    archive_path: &std::path::Path,
) -> FalloutArchive<'a> {
    return FalloutArchive {
        archive: archive_tuple.0.clone(),
        options: archive_tuple.1,
        path_buf: archive_path.to_path_buf(),
    };
}

pub fn get_archive_name_path(archive_path: &std::path::PathBuf) -> String {
    // christ almighty
    let file_name: Option<String> = archive_path
        .file_name()
        .unwrap()
        .to_str()
        .map(str::to_string);
    match file_name {
        Some(s) => s,
        None => String::from("name_none"),
    }
}

pub fn parse_archive(archive: &FalloutArchive) {
    std::println!(
        "parsing archive: {}",
        get_archive_name_path(&archive.path_buf)
    );
    std::println!("version: {:#?}", get_version(&archive));
    std::println!("needs patch: {:#?}", needs_patch_archive(&archive));
}

pub fn get_version(archive: &FalloutArchive) -> ba2::fo4::Version {
    return archive.options.version();
}

pub fn get_version_string(archive: &FalloutArchive) -> String {
    match archive.options.version() {
        ba2::fo4::Version::v1 => return String::from("v1"),
        ba2::fo4::Version::v7 => return String::from("v7"),
        ba2::fo4::Version::v8 => return String::from("v8"),
        ba2::fo4::Version::v2 => return String::from("v2"),
        ba2::fo4::Version::v3 => return String::from("v3"),
    }
}

pub fn version_to_string(vers: ba2::fo4::Version) -> String {
    match vers {
        ba2::fo4::Version::v1 => return String::from("v1"),
        ba2::fo4::Version::v7 => return String::from("v7"),
        ba2::fo4::Version::v8 => return String::from("v8"),
        ba2::fo4::Version::v2 => return String::from("v2"),
        ba2::fo4::Version::v3 => return String::from("v3"),
    }
}

pub fn needs_patch_archive(archive: &FalloutArchive) -> bool {
    match archive.options.version() {
        ba2::fo4::Version::v1 => return false,
        ba2::fo4::Version::v7 | ba2::fo4::Version::v8 => return true,
        ba2::fo4::Version::v2 | ba2::fo4::Version::v3 => return false, // sf version, report error
    }
}

pub fn needs_patch_version(vers: ba2::fo4::Version) -> bool {
    match vers {
        ba2::fo4::Version::v1 => return false,
        ba2::fo4::Version::v7 | ba2::fo4::Version::v8 => return true,
        ba2::fo4::Version::v2 | ba2::fo4::Version::v3 => return false, // sf version, report error
    }
}

pub fn patch_version(mut _archive: &FalloutArchive) -> Option<()> {
    //let archive_options: ArchiveOptions = archive.1;
    //let old_version: Version = ba2::fo4::Version::v1;
    //let options_new: ba2::fo4::ArchiveOptions = ba2::fo4::ArchiveOptions::builder()
    //    .version(ba2::fo4::Version::v1)
    //    .build();
    //archive.1 = options_new;

    //let _ = ba2::fo4::FileWriteOptions::builder()
    //    .compression_format(ba2::fo4::CompressionFormat::Zip)
    //    .build();
    //let options_temp: ba2::fo4::ArchiveOptions = archive.options;
    //options_temp.version() = ba2::fo4::Version::v1;
    //archive.write()
    return Some(());
}

/*
fn example() -> Option<()> {
    let chunk = Chunk::from_decompressed(b"Hello world!\n");
    let file: File = [chunk].into_iter().collect();
    let key: ArchiveKey = b"hello.txt".into();
    let archive: Archive = [(key, file)].into_iter().collect();
    let mut dst = fs::File::create("example.ba2").ok()?;
    let options = ArchiveOptions::default();
    archive.write(&mut dst, &options).ok()?;
    Some(())
}
*/

pub fn appgui_button_select_archive() -> Option<FalloutArchive<'static>> {
    let archive_path_buf: std::path::PathBuf = rfd::FileDialog::new()
        .add_filter("ba2", &["ba2"])
        .set_directory("/")
        .pick_file()?;
    let archive_path: &std::path::Path = archive_path_buf.as_path();
    let archive_tuple: (ba2::fo4::Archive<'_>, ba2::fo4::ArchiveOptions) =
        ba2::fo4::Archive::read(archive_path).ok()?;
    let archive_file: FalloutArchive<'_> = create_archive_struct(archive_tuple, &archive_path);
    let archive_name: &std::ffi::OsStr = std::path::Path::new(&archive_path).file_name().unwrap();
    //return Some(archive_file);

    // temp
    //std::println!("archive_name: {:?}", archive_name);
    //std::println!("archive_version: {:#?}", get_version(&archive_file));
    //parse_archive(&archive_file);

    Some(archive_file)
}

pub fn appgui_button_select_directory() -> Option<std::path::PathBuf> {
    let dir_path_buf: std::path::PathBuf =
        rfd::FileDialog::new().set_directory("/").pick_folder()?;
    //let dir_path: &std::path::Path = dir_path_buf.as_path();
    std::println!("dir_path: {:?}", &dir_path_buf.as_path());

    return Some(dir_path_buf);
}

pub fn to_string(archive: &FalloutArchive) {
    // std::println!("name: {:?}", &archive); TODO
    std::println!("version: {:?}", get_version(&archive));
    std::println!("needs patch: {:?}", needs_patch_archive(&archive));
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
