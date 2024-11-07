// implements the icon for the .exe

fn main() -> std::io::Result<()> {
    if std::env::var_os("CARGO_CFG_WINDOWS").is_some() {
        winresource::WindowsResource::new()
            .set_icon("assets/icons/avp_icon.ico")
            .compile()?;
    }
    return Ok(());
}
