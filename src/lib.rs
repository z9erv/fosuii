mod args;
mod swap;
mod database;
mod usage_dir;

pub const APP_USE_FONT: &str = "/data/data/com.termux/files/home/.termux/font.ttf";
pub const APP_CONF_DIR: &str = "/data/data/com.termux/files/home/.termux";
pub const DATABASE: &str = "/data/data/com.termux/files/home/.local/share/fs/database.db";
pub const FONT_DIR: &str = "/data/data/com.termux/files/home/.fonts";
pub const USE_CONF_DIR: &str = "/data/data/com.termux/files/home/.config/fs";
pub const YAML_CONF: &str = "/data/data/com.termux/files/home/.config/fs/config.yaml";

pub fn flush() -> Result<(), std::io::Error> {

    std::process::Command::new("bash")
        .arg("termux-reload-settings")
        .status()?;

    Ok(())
}
