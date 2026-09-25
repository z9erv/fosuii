mod swap;
mod args;
mod usage_dir;

use args::Cli;
use clap::Parser;
use swap::swap;
use usage_dir::usage;
use fs::flush;

use fs::{
    APP_USE_FONT,
    FONT_DIR,
    YAML_CONF,
    USE_CONF_DIR,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    swap(args.clone())?;
    usage(args.clone())?;

    Ok(())
}
