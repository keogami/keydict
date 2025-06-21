use std::{path::Path, time::Instant};

use anyhow::Context;

use crate::tree::Tree;

pub fn search(word: &str, tree_file: impl AsRef<Path>) -> anyhow::Result<()> {
    let measure = Instant::now();
    println!("loading file");
    let tree = Tree::from_path(tree_file).context("Couldn't open tree file")?;
    println!("took {:?}", measure.elapsed());

    let measure = Instant::now();
    println!(
        "word `{word}` {} found in dictionary.",
        if tree.search(word) { "was" } else { "was not" }
    );
    println!("search took {:?}", measure.elapsed());

    Ok(())
}
