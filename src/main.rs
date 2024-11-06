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
    //let args = CLI::parse();
    //std::println!("pattern: {:?}, path: {:?}", args.pattern, args.path);

    let path: &Path = Path::new(r"./src/test_archives/fo4_ng.ba2");
    avp::temp_get_version(path);
    std::println!("needs patch: {}", avp::needs_patch(path));
}
