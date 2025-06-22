use std::path::PathBuf;

use clap::{Subcommand, arg};

mod generate_tree;
mod search_string;
mod search;
mod complete_string;
mod complete;

pub use generate_tree::*;
pub use search_string::*;
pub use search::*;
pub use complete_string::*;
pub use complete::*;

use crate::keys::Keys;

#[derive(clap::Parser, Debug)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    GenerateTree {
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
    Complete {
        #[arg(short, long)]
        /// path to the file with the dictionary
        words: PathBuf,
        /// word to complete
        query: Keys,
    },
}
