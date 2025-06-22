use std::{collections::BTreeMap, path::Path};

use anyhow::Context;
use fmmap::MmapFileExt;

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
        let file = fmmap::MmapFile::open(path).context("Couldn't open the file")?;
        let file = file.reader(0).context("Couldn't create a reader for file")?;
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
        chars.iter().filter_map(move |&c| {
            let mut acc = acc.clone();
            acc.push(c);
            // println!("acc = {acc} && char = {c} && head = {head:?}");
            let sub_tree =  self.choose(c)?;
            // println!("acc = {acc} && char = {c} && head = {head:?} | found sub_tree");

            if keys.0.len() == 1 && sub_tree.end {
                return Some(vec![acc]);
            }

            Some(sub_tree.children.search_keys_inner(acc, Keys(keys.0[1..].into())))
        }).flatten().collect()
    }

    pub fn search_keys(&self, keys: Keys) -> Vec<String> {
        self.search_keys_inner(Default::default(), keys)
    }

    fn search_key_prefixed_tree_inner(&self, acc: &str, prefix: Keys) -> Vec<(String, &Self)> {
        if self.is_empty() {
            return Vec::new();
        }

        if prefix.0.is_empty() {
            return Vec::new();
        }

        let head = prefix.0.first().unwrap();
        let sub_trees = head.chars().iter().filter_map(|&c| self.choose(c).map(|node| (c, node)));

        if prefix.0.len() == 1 {
            return sub_trees.map(|(c, n)| {
                let mut acc = acc.to_string();
                acc.push(c);
                (acc, &n.children)
            }).collect();
        }

        sub_trees.flat_map(|(c, tree)| {
            let mut acc = acc.to_string();
            acc.push(c);
            tree.children.search_key_prefixed_tree_inner(&acc, Keys(prefix.0[1..].into()))
        }).collect()
    }

    fn search_key_prefixed_tree(&self, prefix: Keys) -> Vec<(String, &Self)> {
        self.search_key_prefixed_tree_inner("", prefix)
    }

    fn search_prefixed_tree(&self, prefix: &str) -> Option<&Self> {
        if self.is_empty() {
            return None;
        }

        if prefix.is_empty() {
            return None;
        }

        let head = prefix.chars().next().unwrap();
        let sub_tree = self.choose(head)?;

        if prefix.len() == 1 {
            return Some(&sub_tree.children);
        }

        sub_tree.children.search_prefixed_tree(&prefix[1..])
    }

    fn reduce_to_list_inner(&self, acc: &str) -> Vec<String> {
        self.0.iter().flat_map(|(&c, sub_tree)| {
            let mut acc = acc.to_string();
            acc.push(c);

            let sub_list = sub_tree.children.reduce_to_list_inner(&acc);

            let mut res = if sub_tree.end {
                vec![acc]
            } else {
                vec![]
            };

            res.extend(sub_list);

            res
        }).collect()
    }

    pub fn reduce_to_list(&self, prefix: Option<&str>) -> Vec<String> {
        self.reduce_to_list_inner(prefix.unwrap_or_default())
    }

    pub fn prefix_complete(&self, prefix: &str) -> Vec<String> {
        let Some(prefix_tree) = self.search_prefixed_tree(prefix) else {
            return Vec::new();
        };

        prefix_tree.reduce_to_list(Some(prefix))
    }

    pub fn prefix_key_complete(&self, prefix: Keys) -> Vec<String> {
        self.search_key_prefixed_tree(prefix).iter().flat_map(|(prefix, tree)| {
            tree.reduce_to_list(Some(prefix))
        }).collect()
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
