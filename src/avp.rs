use ba2::{
    fo4::{Archive, Version},
    Reader,
};
use std::path::Path;

pub struct FalloutArchive<'a> {
    pub archive: ba2::fo4::Archive<'a>,
    pub options: ba2::fo4::ArchiveOptions,
}

pub fn tuple_to_struct<'a>(archive_tuple: &'a (ba2::fo4::Archive<'a>, ba2::fo4::ArchiveOptions)) -> FalloutArchive<'a> {
    return FalloutArchive {
        archive: archive_tuple.0.clone(),
        options: archive_tuple.1,
    }
}

pub fn get_version(archive: &(ba2::fo4::Archive, ba2::fo4::ArchiveOptions)) -> ba2::fo4::Version {
    return archive.1.version();
}

pub fn version_to_string(vers: ba2::fo4::Version) -> String {
    match vers {
        Version::v1 => return String::from("v1"),
        Version::v7 => return String::from("v7"),
        Version::v8 => return String::from("v8"),
        Version::v2 => return String::from("v2"), // sf version, report error
        Version::v3 => return String::from("v3"), // sf version, report error
    }
}

pub fn needs_patch(archive: &(ba2::fo4::Archive, ba2::fo4::ArchiveOptions)) -> bool {
    match archive.1.version() {
        Version::v1 => return false,
        Version::v7 => return true,
        Version::v8 => return true,
        Version::v2 => return false, // sf version, report error
        Version::v3 => return false, // sf version, report error
    }
}

pub fn patch_version_temp(path: &Path) {
    let Ok((_archive, options)) = Archive::read(path) else {
        todo!()
    };
    let version_number: Version = options.version();
    match version_number {
        Version::v1 => std::println!("no patch needed"),
        Version::v7 => {
            todo!();
            //options.OptionsBuilder.version(options, );
        }
        Version::v8 => todo!(),
        Version::v2 => std::println!("invalid ba2"), // sf version, report error
        Version::v3 => std::println!("invalid ba2 needed"), // sf version, report error
    }
}

pub fn patch_version(_archive: (ba2::fo4::Archive, ba2::fo4::ArchiveOptions)) {
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

    let archive_pathbuf: std::path::PathBuf = rfd::FileDialog::new()
        .add_filter("ba2", &["ba2"])
        .set_directory("/")
        .pick_file()?;
    let archive_path: &Path = archive_pathbuf.as_path();
    let archive_file: (Archive<'_>, ba2::fo4::ArchiveOptions) = Archive::read(archive_path).ok()?;
    let archive_name: &std::ffi::OsStr = Path::new(&archive_path).file_name().unwrap();

    // temp
    std::println!("archive_name: {:?}", archive_name);
    std::println!("archive_version: {:#?}", get_version(&archive_file));

    return Some(());
}

pub fn appgui_button_select_directory() {
    std::println!("Select Directory button clicked"); // temp
}

pub fn to_string(archive: &(ba2::fo4::Archive, ba2::fo4::ArchiveOptions)) {
    // std::println!("name: {:?}", &archive); TODO
    std::println!("version: {:?}", get_version(&archive));
    std::println!("needs patch: {:?}", needs_patch(&archive));

}
