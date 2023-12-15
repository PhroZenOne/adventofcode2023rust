use std::cell::{Ref, RefCell};
use std::cmp::{min, Ordering};
use std::fs;
use std::rc::Rc;
use rayon::prelude::*;

use regex::Regex;


#[derive(Clone, Debug, Eq, Hash)]
struct TreeNodeData {
    start: usize,
    end: usize,
    target: usize,
}

impl PartialEq<Self> for TreeNodeData {
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start
    }
}

impl PartialOrd<Self> for TreeNodeData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TreeNodeData {
    fn cmp(&self, other: &Self) -> Ordering {
        self.start.cmp(&other.start)
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct TreeNode {
    data: TreeNodeData,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Tree {
    id: String,
    root: Option<Box<TreeNode>>,
}

impl TreeNodeData {
    #[inline]
    pub fn new(start: usize,
               end: usize,
               target: usize) -> Self {
        TreeNodeData {
            start,
            end,
            target,
        }
    }
}

impl Tree {
    pub fn new(id: String) -> Self {
        Tree { id, root: None }
    }

    fn insert(&mut self, data: TreeNodeData) {
        let new_node = Box::new(TreeNode::new(data));
        Self::push_node(new_node, &mut self.root);
    }

    fn push_node(new_node: Box<TreeNode>, current_node: &mut Option<Box<TreeNode>>) {
        if let Some(node) = current_node {
            match node.data.cmp(&new_node.data) {
                Ordering::Greater | Ordering::Equal => Self::push_node(new_node, &mut node.left),
                Ordering::Less => Self::push_node(new_node, &mut node.right),
            }
        } else {
            let _ = current_node.insert(new_node);
        }
    }

    fn find<'a>(&self, val: usize) -> usize {
        return Self::find_internal(val, &self.root);
    }

    fn find_internal(val: usize, current_node: &Option<Box<TreeNode>>) -> usize {
        if let Some(node) = current_node {
            return if node.data.start > val {
                Self::find_internal(val, &node.left)
            } else if node.data.start <= val && node.data.end >= val {
                node.data.target + (val - node.data.start)
            } else {
                Self::find_internal(val, &node.right)
            }
        }
        return val;
    }
}


impl TreeNode {
    #[inline]
    pub fn new(data: TreeNodeData) -> Self {
        TreeNode {
            data,
            left: None,
            right: None,
        }
    }
}


pub(crate) fn parse_file(x: &str) -> u32 {
    let mut trees = vec![];
    let regex = Regex::new(r"(\d+)").unwrap();
    let mut seeds = vec![];
    let mut tree: Option<Rc<RefCell<Tree>>> = None;
    read_lines_from_file(x)
        .lines()
        .for_each(|l| {
            let values: Vec<_> = regex
                .captures_iter(&*l)
                .filter_map(|cap| cap.get(0).map(|m| m.as_str().parse::<usize>().unwrap()))
                .collect();
            if l.starts_with("seeds:") {
                let mut i = 0;
                let vec1 = values.to_vec();
                while i < vec1.len() {
                    let start = vec1[i];
                    let size = vec1[i+1];
                    for j in start..start+size {
                        seeds.push(j);
                    }
                    i += 2;
                }
            } else if l.ends_with(":") {
                let tree_ = Rc::new(RefCell::new(Tree::new(l.to_string())));
                tree = Some(tree_.clone());
                trees.push(tree_);
            } else {
                if values.len() > 0 {
                    tree
                        .as_ref()
                        .expect("tree should exist")
                        .borrow_mut()
                        .insert(
                            TreeNodeData::new(
                                values[1],
                                values[1] + values[2],
                                values[0],
                            )
                        );
                }
            }
        });

    let x1: Vec<Tree> = trees.into_iter().map(|tree| tree.borrow().to_owned()).collect();
    let val = seeds.par_iter().map(|&seed| {
        let mut val = seed;
        for tree in &x1 {
            val = tree.find(val);
        }
        val
    }).reduce(|| usize::MAX,|v, a| min(v, a));

    return val as u32;
}

fn read_lines_from_file(path: &str) -> String {
    fs::read_to_string(path).expect(format!("{path} file should be in place").as_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn order_check() {
        let data = TreeNodeData::new(5, 10, 10);
        let data2 = TreeNodeData::new(10, 15, 10);
        let ordering = data.cmp(&data2);
        assert_eq!(ordering, Ordering::Less);
    }

    #[test]
    fn basic_parse() {
        let i = parse_file("./data/day5/calibration_day5.dat");
        println!("{}", i);
    }
}
