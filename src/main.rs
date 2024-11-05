use clap::Parser;
use std::path::Path;
pub mod avp;
pub mod avplang;

#[derive(Parser)]
struct CLI {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}
impl CLI {}

fn main() {
    println!("[archive-version-patcher]\n");
    let args = CLI::parse();
    std::println!("pattern: {:?}, path: {:?}", args.pattern, args.path);

    //example_write();
    let path =
        Path::new(r"C:\Users\brans\Desktop\archive patcher testing\Fallout4 - Interface.ba2");
    //std::println!("{}", needs_patch(path));
    avp::temp_get_version(path);
}
