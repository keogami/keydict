#![feature(iterator_try_collect)]

use clap::Parser;
use commands::{Args, Commands, assimilate, search_string, search};

pub mod keys;
pub mod tree;

mod commands;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.command {
        Commands::Assimilate { word_list } => assimilate(word_list),
        Commands::SearchString { words, query } => search_string(&query, words),
        Commands::Search { words, query } => search(&query, words),
    }
}
