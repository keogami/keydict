use std::path::PathBuf;

use clap::{Subcommand, arg};

mod assimilate;
mod search_string;
mod search;
mod complete_string;

pub use assimilate::*;
pub use search_string::*;
pub use search::*;
pub use complete_string::*;

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
    SearchString {
        #[arg(short, long)]
        /// path to the file with the dictionary
        words: PathBuf,
        /// word to search for
        query: String,
    },
    Search {
        #[arg(short, long)]
        /// path to the file with the dictionary
        words: PathBuf,
        /// keypad word to search for, e.g. `364` for dog
        query: String,
    },
    CompleteString {
        #[arg(short, long)]
        /// path to the file with the dictionary
        words: PathBuf,
        /// word to complete
        query: String,
    },
}
