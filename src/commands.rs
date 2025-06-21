use std::path::PathBuf;

use clap::{Subcommand, arg};

mod assimilate;
mod search;

pub use assimilate::*;
pub use search::*;

#[derive(clap::Parser, Debug)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Assimilate {
        word_list: PathBuf,
    },
    Search {
        #[arg(short, long)]
        /// path to the file with the dictionary
        words: PathBuf,
        /// word to search for
        query: String,
    },
}
