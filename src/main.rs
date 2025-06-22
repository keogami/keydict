#![feature(iterator_try_collect)]

use clap::Parser;
use commands::{Args, Commands, generate_tree, search_string, search};

use crate::commands::{complete, complete_string};

pub mod keys;
pub mod tree;

mod commands;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.command {
        Commands::GenerateTree { word_list } => generate_tree(word_list),
        Commands::SearchString { words, query } => search_string(&query, words),
        Commands::Search { words, query } => search(&query, words),
        Commands::CompleteString { words, query } => complete_string(&query, words),
        Commands::Complete { words, query } => complete(query, words),
    }
}
