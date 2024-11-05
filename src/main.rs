use ba2::{
    fo4::{Archive, ArchiveKey, ArchiveOptions, Chunk, File, FileWriteOptions, Version},
    prelude::*,
};
use std::fs;
use std::path::Path;

fn main() {
    println!("[archive-version-patcher]\n");
    //example_write();
    let path = Path::new(r"C:\Users\brans\Desktop\archive patcher testing\Fallout4 - Interface.ba2");
    get_version(path);
}

#[allow(dead_code)]
fn patch_archive() -> Option<()> {
    let path = Path::new(r"C:\Users\brans\Desktop\archive patcher testing\Fallout4 - Interface.ba2");
    let (archive, options) = Archive::read(path).ok()?;
    //assert_eq!(options.version(), Version::v7);
    std::println!("{:?}", options.version());
    return Some(());
}

fn get_version(path: &Path) -> Option<()> {
    let (archive, options) = Archive::read(path).ok()?;
    std::println!("version: {:?}", options.version());
    Some(())
}


fn select_archive() {

}

pub enum Language {
    English,
}

// #[test]
//     fn invalid_version() -> anyhow::Result<()> {
//         let path = Path::new("data/fo4_invalid_test/invalid_version.ba2");
//         match Archive::read(path) {
//             Err(Error::InvalidVersion(0x101)) => Ok(()),
//             Err(err) => Err(anyhow::Error::from(err)),
//             Ok(_) => anyhow::bail!("read should have failed"),
//         }
//     }