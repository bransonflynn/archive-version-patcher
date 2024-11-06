use ba2::{
    fo4::{Archive, Version},
    Reader,
};
use std::path::Path;

// maybe use tauri_api to prompt for file/dir select
// https://github.com/tauri-apps/tauri/discussions/3275
// https://docs.rs/tauri-api/latest/tauri_api/index.html

pub fn needs_patch(path: &Path) -> bool {
    let Ok((_archive, options)) = Archive::read(path) else {
        todo!()
    };
    let version_number: Version = options.version();
    match version_number {
        Version::v1 => return false,
        Version::v7 => return true,
        Version::v8 => return true,
        Version::v2 => return false, // sf version, report error
        Version::v3 => return false, // sf version, report error
    }
}

pub fn patch_version(path: &Path) {
    let Ok((_archive, options)) = Archive::read(path) else {
        todo!()
    };
    let version_number = options.version();
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

pub fn select_archive() {}

pub fn select_directory() {}

pub fn temp_get_version(path: &Path) -> Option<()> {
    let (_archive, options) = Archive::read(path).ok()?;
    let file_name = path.file_name()?.to_str()?;
    std::println!("name: {}, version: {:?}", file_name, options.version());
    return Some(());
}

pub fn temp_get_version_archive(archive: ba2::fo4::Archive) -> Option<()> {

}

/*
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
