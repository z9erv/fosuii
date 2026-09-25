use crate::args::{Cli};
use std::{fs, path::PathBuf};
use crate::APP_USE_FONT;
use crate::flush;

pub fn swap(path: Cli) -> Result<(), std::io::Error> {

    if let Some(handle) = path.swap {

        if font_is_work() {

            fs::rename(APP_USE_FONT, "/data/data/com.termux/files/home/.font.ttf.bak")?;
            // fs::remove_file(APP_USE_FONT)?;
        }

        fs::copy(handle, APP_USE_FONT)?;

        flush()?;
    }

    Ok(())
}

pub fn font_is_work() -> bool {

    let path: PathBuf = APP_USE_FONT.into();

    if path.exists() {
        return true;
    }
    false
}
