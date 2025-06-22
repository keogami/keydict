use std::{path::Path, time::Instant};

use anyhow::Context;

use crate::tree::Tree;

pub fn complete_string(word: &str, tree_file: impl AsRef<Path>) -> anyhow::Result<()> {
    let measure = Instant::now();
    println!("loading file");
    let tree = Tree::from_path(tree_file).context("Couldn't open tree file")?;
    println!("took {:?}", measure.elapsed());

    let measure = Instant::now();
    let res = tree.prefix_complete(word);
    println!("search took {:?}", measure.elapsed());
    println!(
        "found the following words by the prefix:\n{res:#?}"
    );

    Ok(())
}
