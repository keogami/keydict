use std::{
    io::{BufRead, BufReader},
    path::Path,
    time::Instant,
};

use anyhow::Context;

use crate::tree::Tree;

pub fn assimilate(word_list: impl AsRef<Path>) -> anyhow::Result<()> {
    let measure = Instant::now();
    let save_path = word_list.as_ref().with_extension("kdtree");
    let file = std::fs::File::open(word_list).context("Couldn't open word list")?;
    let file = BufReader::new(file);

    let mut tree = Tree::new();

    for word in file.lines() {
        let word = word.context("Couldn't read line")?;
        tree.add_word(&word);
    }

    tree.store(save_path).context("Couldn't save tree")?;

    println!("took {:?}", measure.elapsed());

    Ok(())
}
