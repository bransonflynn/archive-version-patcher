#[allow(unused_imports)]
use ba2::{
    fo4::{Archive, ArchiveKey, ArchiveOptions, Chunk, File, Version},
    prelude::*,
    Reader,
};
use std::path::Path;

// maybe use tauri_api to prompt for file/dir select
// https://github.com/tauri-apps/tauri/discussions/3275
// https://docs.rs/tauri-api/latest/tauri_api/index.html

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

pub fn patch_version(mut archive: (ba2::fo4::Archive, ba2::fo4::ArchiveOptions)) {
    //let archive_options: ArchiveOptions = archive.1;
    //let old_version: Version = ba2::fo4::Version::v1;
    //let options_format = ba2::fo4::FileReadOptions::builder().format(ba2::fo4::Format::DX10).build();
    //let options_version: ArchiveOptions = ba2::fo4::ArchiveOptions::builder().version(Version::v1).build();
    let options_new: ArchiveOptions = ba2::fo4::ArchiveOptions::builder()
        .version(Version::v1)
        .build();

    archive.1 = options_new;
}

pub fn select_archive() {}

pub fn select_directory() {}

pub fn display(archive: &(ba2::fo4::Archive, ba2::fo4::ArchiveOptions)) {
    // std::println!("name: {:?}", &archive); TODO
    std::println!("version: {:?}", get_version(&archive));
    std::println!("needs patch: {:?}", needs_patch(&archive));
}

/*

//let builder = ba2::fo4::archive::OptionsBuilder::OptionsBuilder(archive.1);
    //archive.0.header
    //let _ = ArchiveOptions::builder()
    //    .version(Version::v1)
    //    .build();
    //let options = ba2::fo4::FileReadOptions::builder().version(ba2::fo4::Version::v1).build();

// READ
pub fn example() -> Option<()> {
    let path = Path::new(r"path/to/fallout4/Data/Fallout4 - Interface.ba2");
    let (archive, meta) = Archive::read(path).ok()?;
    let key: ArchiveKey = b"Interface/HUDMenu.swf".into();
    let file = archive.get(&key)?;
    let mut dst = fs::File::create("HUDMenu.swf").ok()?;
    let options: FileWriteOptions = meta.into();
    file.write(&mut dst, &options).ok()?;
    Some(())
}

// WRITE
pub fn example() -> Option<()> {
    let chunk = Chunk::from_decompressed(b"Hello world!\n");
    let file: File = [chunk].into_iter().collect();
    let key: ArchiveKey = b"hello.txt".into();
    let archive: Archive = [(key, file)].into_iter().collect();
    let mut dst = fs::File::create("example.ba2").ok()?;
    let options = ArchiveOptions::default();
    archive.write(&mut dst, &options).ok()?;
    Some(())
} */
