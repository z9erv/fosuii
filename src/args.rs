use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Clone)]

pub struct Cli {
    #[arg(long, short, conflicts_with = "use")]
    pub swap: Option<PathBuf>,

    #[arg(long, short, conflicts_with = "swap")]
    pub r#use: Option<String>,
    /* pub r#use: Option<Use>, */
}

#[derive(Subcommand, Clone)]

pub enum Use {
    Swap { #[arg(long, short)] path: PathBuf },

}
