use std::{path::Path, time::Instant};

use anyhow::Context;

use crate::{keys::Keys, tree::Tree};

pub fn search(word: &str, tree_file: impl AsRef<Path>) -> anyhow::Result<()> {
    let measure = Instant::now();
    println!("loading file");
    let tree = Tree::from_path(tree_file).context("Couldn't open tree file")?;
    println!("took {:?}", measure.elapsed());

    let word: Keys = word.parse().context("Couldn't parse into valid keys")?;
    let measure = Instant::now();
    println!(
        "Found the following words with the given key:\n{:#?}",
        tree.search_keys(word)
    );
    println!("search took {:?}", measure.elapsed());

    Ok(())
}
