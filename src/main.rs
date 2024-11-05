use std::path::Path;

use ba2::{
    fo4::{Archive, ArchiveOptionsBuilder, Version}, Reader}
;

fn main() {
    println!("[archive-version-patcher]\n");
    //example_write();
    let path =
        Path::new(r"C:\Users\brans\Desktop\archive patcher testing\tester.ba2");
    std::println!("{}", needs_patch(path));
}

fn get_version(path: &Path) -> Option<()> {
    let (archive, options) = Archive::read(path).ok()?;
    std::println!("version: {:?}", options.version());
    Some(())
}

fn needs_patch(path: &Path) -> bool {
    let Ok((_archive, options)) = Archive::read(path) else { todo!() };
    let version_number = options.version();
    match version_number {
        Version::v1 => return false,
        Version::v2 => return false, // starfield version
        Version::v3 => return false, // starfield version
        Version::v7 => return true,
        Version::v8 => return true,
    }
}

fn patch_version(path: &Path) {
    let Ok((archive, options)) = Archive::read(path) else { todo!() };
    let version_number = options.version();
    match version_number {
        Version::v1 => std::println!("no patch needed"),
        Version::v2 => std::println!("invalid ba2"), // starfield version
        Version::v3 => std::println!("invalid ba2 needed"), // starfield version
        Version::v7 => {
            //options.OptionsBuilder.version(options, );
        },
        Version::v8 => todo!(),
    }
}

fn select_archive() {}

pub enum Language {
    English,
}

/*
// READ
fn example() -> Option<()> {
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
fn example() -> Option<()> {
    let chunk = Chunk::from_decompressed(b"Hello world!\n");
    let file: File = [chunk].into_iter().collect();
    let key: ArchiveKey = b"hello.txt".into();
    let archive: Archive = [(key, file)].into_iter().collect();
    let mut dst = fs::File::create("example.ba2").ok()?;
    let options = ArchiveOptions::default();
    archive.write(&mut dst, &options).ok()?;
    Some(())
} */