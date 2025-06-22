use std::{collections::BTreeMap, path::Path};

use anyhow::Context;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;

use crate::keys::Keys;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Node {
    pub end: bool,
    pub children: Tree,
}

impl Node {
    pub fn new() -> Self {
        Self {
            end: false,
            children: Tree::new(),
        }
    }

    #[allow(unused)]
    pub fn end(self) -> Self {
        let Self { children, .. } = self;
        Self {
            end: true,
            children,
        }
    }

    pub fn with_end(self, end: bool) -> Self {
        let Self { children, .. } = self;
        Self { end, children }
    }

    #[allow(unused)]
    pub fn children(self, nodes: &[(char, Node)]) -> Self {
        let Self { mut children, end } = self;

        children
            .0
            .extend(nodes.iter().map(|(c, n)| (*c, n.clone())));

        Self { end, children }
    }
}

impl Default for Node {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Tree(BTreeMap<char, Node>);

impl Tree {
    pub fn new() -> Self {
        Self(Default::default())
    }

    #[allow(unused)]
    pub fn from_path_json(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let file = std::fs::File::open(path).context("Couldn't open file")?;
        let tree: Self = serde_json::from_reader(file).context("Couldn't parse as json")?;

        Ok(tree)
    }

    #[allow(unused)]
    pub fn store_json(self, path: impl AsRef<Path>) -> anyhow::Result<()> {
        let file = std::fs::File::create(path).context("Couldn't create file")?;
        serde_json::to_writer(file, &self).context("Couldn't store tree")?;

        Ok(())
    }

    pub fn from_path(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let file = std::fs::File::open(path).context("Couldn't open file")?;
        let tree: Self = serde_cbor::from_reader(file).context("Couldn't parse as cbor")?;

        Ok(tree)
    }

    pub fn store(self, path: impl AsRef<Path>) -> anyhow::Result<()> {
        let file = std::fs::File::create(path).context("Couldn't create file")?;
        serde_cbor::to_writer(file, &self).context("Couldn't store tree")?;

        Ok(())
    }

    pub fn add_word(&mut self, word: &str) {
        if word.is_empty() {
            return;
        }
        let first = word.chars().next().unwrap();

        let sub_tree = self
            .0
            .entry(first)
            .or_insert_with(|| Node::new().with_end(word.len() == 1));

        if word.len() == 1 {
            return;
        }

        sub_tree.children.add_word(&word[1..]);
    }

    pub fn choose(&self, c: char) -> Option<&Node> {
        self.0.get(&c)
    }

    // pub fn choose_mut(&mut self, c: char) -> Option<&mut Node> {
    //     self.0.get_mut(&c)
    // }

    fn search_keys_inner(&self, acc: String, keys: Keys) -> Vec<String> {
        if keys.0.is_empty() {
            // println!("acc = {acc} | stopping");
            return Vec::new();
        }

        let head = *keys.0.first().unwrap();
        // println!("acc = {acc} | head = {head:?}");

        let chars = head.chars();
        let res: Vec<String> = chars.iter().filter_map(move |&c| {
                let mut acc = acc.clone();
                acc.push(c);
                // println!("acc = {acc} && char = {c} && head = {head:?}");
                let sub_tree =  self.choose(c)?;
                // println!("acc = {acc} && char = {c} && head = {head:?} | found sub_tree");

                if keys.0.len() == 1 && sub_tree.end {
                    return Some(vec![acc]);
                }

                Some(sub_tree.children.search_keys_inner(acc, Keys(keys.0[1..].into())))
            }).fold(Vec::new(), |mut a, b| {
                a.extend(b);
                a
            });

        res
    }

    pub fn search_keys(&self, keys: Keys) -> Vec<String> {
        self.search_keys_inner(Default::default(), keys)
    }


    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn search(&self, pin: &str) -> bool {
        if pin.is_empty() {
            return false;
        }

        if self.is_empty() {
            return false;
        }

        let first = pin.chars().next().unwrap();
        let Some(sub_tree) = self.choose(first) else {
            return false;
        };

        if pin.len() == 1 && sub_tree.end {
            return true;
        }

        sub_tree.children.search(&pin[1..])
    }

    #[allow(unused)]
    pub fn new_with_nodes(nodes: &[(char, Node)]) -> Self {
        Self(nodes.iter().map(|(c, n)| (*c, n.clone())).collect())
    }
}

impl Default for Tree {
    fn default() -> Self {
        Self::new()
    }
}
