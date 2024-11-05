use std::path::Path;
pub mod avp;
pub mod avplang;

fn main() {
    println!("[archive-version-patcher]\n");
    //example_write();
    let path =
        Path::new(r"C:\Users\brans\Desktop\archive patcher testing\Fallout4 - Interface.ba2");
    //std::println!("{}", needs_patch(path));
    avp::temp_get_version(path);
}
