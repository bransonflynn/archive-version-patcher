extern crate walkdir;
use ba2::Reader;
use std::io::{Read, Write};

#[derive(Clone)]
pub struct FalloutArchive<'a> {
    pub archive: ba2::fo4::Archive<'a>,
    pub options: ba2::fo4::ArchiveOptions,
    pub path_buf: std::path::PathBuf,
}

impl<'a> Default for FalloutArchive<'a> {
    fn default() -> Self {
        Self {
            archive: Default::default(),
            options: Default::default(),
            path_buf: Default::default(),
        }
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

pub fn struct_to_tuple<'a>(
    archive_struct: &'a FalloutArchive<'a>,
) -> (ba2::fo4::Archive<'a>, ba2::fo4::ArchiveOptions) {
    let archive_tuple: (ba2::fo4::Archive<'_>, ba2::fo4::ArchiveOptions) =
        (archive_struct.archive.clone(), archive_struct.options);
    return archive_tuple;
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
        ba2::fo4::Version::v2 => return String::from("v2"), // sf version
        ba2::fo4::Version::v3 => return String::from("v3"), // sf version
    }
}

pub fn version_to_string(vers: ba2::fo4::Version) -> String {
    match vers {
        ba2::fo4::Version::v1 => return String::from("v1"),
        ba2::fo4::Version::v7 => return String::from("v7"),
        ba2::fo4::Version::v8 => return String::from("v8"),
        ba2::fo4::Version::v2 => return String::from("v2"), // sf version
        ba2::fo4::Version::v3 => return String::from("v3"), // sf version
    }
}

pub fn needs_patch_archive(archive: &FalloutArchive) -> bool {
    match archive.options.version() {
        ba2::fo4::Version::v1 => return false,
        ba2::fo4::Version::v7 | ba2::fo4::Version::v8 => return true,
        ba2::fo4::Version::v2 | ba2::fo4::Version::v3 => return false, // sf version
    }
}

pub fn needs_patch_version(vers: ba2::fo4::Version) -> bool {
    match vers {
        ba2::fo4::Version::v1 => return false,
        ba2::fo4::Version::v7 | ba2::fo4::Version::v8 => return true,
        ba2::fo4::Version::v2 | ba2::fo4::Version::v3 => return false, // sf version
    }
}

pub fn patch_version(_archive: &FalloutArchive) {
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
    //let archive_tuple: (Archive<'_>, ArchiveOptions) = struct_to_tuple(archive);
}

pub fn patch_version_test(archive: &FalloutArchive) -> std::io::Result<()> {
    // open the file
    let mut file: std::fs::File =
        std::fs::File::create(&archive.path_buf).expect("ERROR: Opening archive failed");
    std::println!("archive name: {}", get_archive_name_path(&archive.path_buf));

    // setup first 5 bytes
    let mut buf: [u8; 5] = [0u8; 5];
    file.read_exact(&mut buf)?;
    std::println!("read {} bytes: {:?}", buf.len(), buf);
    buf[4] = 1; // setup archive version 1 at index 4
    std::println!("new buf: read {} bytes: {:?}", buf.len(), buf);

    // write bytes
    match file.write(&mut buf) {
        Ok(_) => {
            std::println!("file write OK");
        }
        Err(_) => {
            std::println!("file write ERR");
        }
    }
    std::println!("read {} bytes: {:?}", buf.len(), buf);

    return Ok(());
}

/*
    doubleyou's py code for editing bytes:
    //
        with open(archive, "r+b") as f:
            f.seek(4)
            byte = f.read(1)
            if byte == b'\x07' or byte == b'\x08':
                f.seek(4)
                f.write(b'\x01')
                patched = True
            elif byte == b'\x01':
                sm(archive + text['No patch needed'][language.get()])
            else:
                sm(archive + text['Unexpected version'][language.get()], True)
            f.close()
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

    Some(archive_file)
}

pub fn appgui_button_select_dir() -> Option<std::path::PathBuf> {
    let dir_path_buf: std::path::PathBuf =
        rfd::FileDialog::new().set_directory("/").pick_folder()?;
    return Some(dir_path_buf);
}

pub fn count_archives_in_dir(dir: &std::path::PathBuf) -> u64 {
    let mut total: u64 = 0;
    let extension: Option<&std::ffi::OsStr> = Some(std::ffi::OsStr::new("ba2"));
    for file in walkdir::WalkDir::new(dir)
        .into_iter()
        .filter_map(|file: Result<walkdir::DirEntry, walkdir::Error>| file.ok())
    {
        if file.metadata().unwrap().is_file() {
            if file.path().extension() == extension {
                total += 1;
            }
        }
    }
    return total;
}

pub fn get_archives_in_dir(dir: &std::path::PathBuf) -> Vec<walkdir::DirEntry> {
    let mut result: Vec<walkdir::DirEntry> = Vec::new();
    let extension: Option<&std::ffi::OsStr> = Some(std::ffi::OsStr::new("ba2"));
    for file in walkdir::WalkDir::new(dir)
        .into_iter()
        .filter_map(|file: Result<walkdir::DirEntry, walkdir::Error>| file.ok())
    {
        if file.metadata().unwrap().is_file() {
            if file.path().extension() == extension {
                result.push(file);
            }
        }
    }
    return result;
}

pub fn to_string(archive: &FalloutArchive) {
    // TODO should be added as a default/proper impl in the struct
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
