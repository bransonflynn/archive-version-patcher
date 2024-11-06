use ba2::{fo4::Archive, Reader};
use std::path::Path;
pub mod avp;
pub mod avplang;

/*
use clap::Parser;
#[derive(Parser)]
struct CLI {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}
impl CLI {}
 */

fn main() {
    println!("[archive-version-patcher]\n");
    // setup language
    // read from config toml for data
    // setup lang enum with int read from config toml with match statement
    let _language: avplang::Language = avplang::Language::English;

    //let args = CLI::parse();
    //std::println!("pattern: {:?}, path: {:?}", args.pattern, args.path);

    main_impl();
}

pub fn main_impl() -> Option<()> {
    let path: &Path = Path::new(r"./src/test_archives/fo4_ng.ba2");
    let archive: (ba2::fo4::Archive, ba2::fo4::ArchiveOptions) = Archive::read(path).ok()?;

    std::println!("name: {:?}", path.file_name().unwrap());
    std::println!("version: {:?}", avp::get_version(&archive));
    std::println!("needs patch: {:?}", avp::needs_patch(&archive));
    return Some(());
}
