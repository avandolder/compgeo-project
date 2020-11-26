//#![feature(box_syntax, box_patterns, min_const_generics)]

use std::fmt::Debug;

pub trait Element: PartialOrd + Clone + Debug {}
impl<T: PartialOrd + Clone + Debug> Element for T {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum KDTree<T: Element, const N: usize> {
    Leaf([T; N]),
    Split(Box<KDTree<T, N>>, Box<KDTree<T, N>>, T),
}

impl<T: Element, const N: usize> KDTree<T, N> {
    pub fn new(pts: &mut [[T; N]]) -> Option<Self> {
        if pts.is_empty() {
            None
        } else {
            Some(Self::split(pts, 0))
        }
    }

    fn split(pts: &mut [[T; N]], depth: usize) -> Self {
        pts.sort_by(|u, v| u[depth].partial_cmp(&v[depth]).unwrap());
        
        let mid = pts.len() / 2;
        let next_depth = (depth + 1) % N;
        match pts.split_at_mut(mid) {
            ([], [pt]) => KDTree::Leaf(pt.clone()),
            (left, right) => KDTree::Split(
                box Self::split(left, next_depth),
                box Self::split(right, next_depth),
                left.last().unwrap()[depth].clone(),
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::KDTree::{self, Leaf, Split};

    #[test]
    fn empty() {
        assert!(KDTree::<i32, 2>::new(&mut []).is_none());
    }

    #[test]
    fn single_point() {
        assert!(match KDTree::new(&mut [[0i32, 0]]) {
            Some(Leaf([0, 0])) => true,
            _ => false,
        })
    }

    #[test]
    fn x_split() {
        assert!(match KDTree::new(&mut [[0i32, 0], [1, 0]]) {
            Some(Split(box Leaf([0, 0]), box Leaf([1, 0]), 0)) => true,
            _ => false,
        })
    }

    #[test]
    fn x_then_y_split() {
        let tree = dbg!(KDTree::new(&mut [[0i32, 0], [1, 0], [0, 1]]));
        assert!(match tree {
            Some(Split(box Leaf([0, 0]),
                       box Split(box Leaf([1, 0]),
                                 box Leaf([0, 1]), 0), 0)) => true,
            _ => false,
        })
    }
}
