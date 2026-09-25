use std::collections::{HashMap};
use std::path::PathBuf;
use std::fs::{self, File};
use crate::swap::font_is_work;
use serde::{Serialize, Deserialize};
use crate::args::Cli;
use crate::{
    YAML_CONF,
    USE_CONF_DIR,
    FONT_DIR,
    APP_USE_FONT,
};
use crate::flush;

#[derive(Serialize, Deserialize, Clone)]
struct ConfFile {
    fonts: HashMap<String, PathBuf>,
}

impl ConfFile {
    fn load() -> Result<Self, Box<dyn std::error::Error>> {

        let handle = File::open(YAML_CONF)?;

        let target: ConfFile = noyalib::from_reader(handle)?;
        Ok(Self {
            fonts: target.fonts
        })
    }

    fn init() -> Result<(), Box<dyn std::error::Error>> {

        let conf = ConfFile {
            fonts: HashMap::new()
        };

        let file = File::create(YAML_CONF)?;

        noyalib::to_writer(file, &conf)?;

        Ok(())
    }

    fn write(&self) -> Result<(), Box<dyn std::error::Error>> {

        let file = File::create(YAML_CONF)?;

        noyalib::to_writer(file, self)?;

        Ok(())
    }
}

fn dir_is_success() -> bool {
    let (fonts, conf, yaml): (PathBuf, PathBuf, PathBuf) = (FONT_DIR.into(), USE_CONF_DIR.into(), YAML_CONF.into());

    if fonts.exists() && conf.exists() && yaml.exists() {
        return true;
    }
    false
}

fn crate_dir() -> Result<(), Box<dyn std::error::Error>> {

    if dir_is_success() {
        return Ok(());
    }
    else {
        fs::create_dir_all(FONT_DIR)?;
        fs::create_dir_all(USE_CONF_DIR)?;
        fs::create_dir_all(USE_CONF_DIR)?;
        ConfFile::init()?;
    }

    Ok(())
}

pub fn usage(args: Cli) -> Result<(), Box<dyn std::error::Error>> {

    if let Some(select) = args.r#use {

        crate_dir()?;
        let conf = ConfFile::load()?;

        for (key, value) in conf.fonts.iter() {
            if select == *key {
                if font_is_work() {
                    fs::rename(APP_USE_FONT, "/data/data/com.termux/files/home/.font.ttf.bak")?;
                }
                fs::copy(value.clone(), APP_USE_FONT)?;

                flush()?;
            }
        }
    }

    Ok(())
}
