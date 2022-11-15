use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Cli{
    /// Url
    #[arg(short, long, value_name = "URL")]
    url: Option<String>,

    /// Url
    #[arg(short, long, value_name = "TOKEN")]
    token: Option<String>,

    /// Set a config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands{
    ///get a value from key
    Get{
        /// the key
        #[arg(short, long)]
        key: String
    },
    ///Set a pair of key value
    Set{
        /// the key
        #[arg(short, long)]
        key: String,
        /// the value
        #[arg(short, long)]
        value: String
    },

}
