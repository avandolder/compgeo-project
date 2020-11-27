#![feature(box_syntax, box_patterns, min_const_generics)]

mod kdtree;

use kdtree::KDTree;

fn main() {
    let tree = KDTree::from_iter((0..8).map(|x| [(x & 4) >> 2, (x & 2) >> 1, x & 1]));
    println!("{:?}", tree);
}
