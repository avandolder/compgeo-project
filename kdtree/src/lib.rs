#![feature(box_syntax, box_patterns)]

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum KDTree {
    Leaf((i32, i32)),
    XSplit(Box<KDTree>, Box<KDTree>, i32),
    YSplit(Box<KDTree>, Box<KDTree>, i32),
}

impl KDTree {
    pub fn new(pts: &mut [(i32, i32)]) -> Option<Self> {
        if pts.is_empty() {
            None
        } else {
            Some(Self::x_split(pts))
        }
    }

    fn x_split(pts: &mut [(i32, i32)]) -> Self {
        pts.sort_by_key(|(x, _)| *x);
        
        let mid = pts.len() / 2;
        match pts.split_at_mut(mid) {
            ([], [pt]) => KDTree::Leaf(pt.clone()),
            (left, right) => KDTree::XSplit(
                box Self::y_split(left),
                box Self::y_split(right),
                left.last().unwrap().0,
            ),
        }
    }

    fn y_split(pts: &mut [(i32, i32)]) -> Self {
        pts.sort_by_key(|(_, y)| *y);
        
        let mid = pts.len() / 2;
        match pts.split_at_mut(mid) {
            ([], [pt]) => KDTree::Leaf(pt.clone()),
            (left, right) => KDTree::YSplit(
                box Self::x_split(left),
                box Self::x_split(right),
                left.last().unwrap().1,
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::KDTree::{self, Leaf, XSplit, YSplit};

    #[test]
    fn empty() {
        assert!(KDTree::new(&mut []).is_none());
    }

    #[test]
    fn single_point() {
        assert!(match KDTree::new(&mut [(0, 0)]) {
            Some(Leaf((0, 0))) => true,
            _ => false,
        })
    }

    #[test]
    fn x_split() {
        assert!(match KDTree::new(&mut [(0, 0), (1, 0)]) {
            Some(XSplit(box Leaf((0, 0)), box Leaf((1, 0)), 0)) => true,
            _ => false,
        })
    }

    #[test]
    fn x_then_y_split() {
        assert!(match KDTree::new(&mut [(0, 0), (1, 0), (0, 1)]) {
            Some(XSplit(box Leaf((0, 0)),
                        box YSplit(box Leaf((1, 0)),
                                   box Leaf((0, 1)), 0), 0)) => true,
            _ => false,
        })
    }
}
