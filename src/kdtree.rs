use std::fmt::Debug;

use image::{ImageResult, RgbImage};

use crate::aabb::AABB;
use crate::draw::{draw_line, draw_point, BLUE, RED, WHITE};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum KDTree {
    Empty,
    Leaf([u32; 2], AABB),
    Split(Box<KDTree>, Box<KDTree>, usize, [u32; 2], AABB),
}

use KDTree::*;

impl KDTree {
    pub fn new(pts: &mut [[u32; 2]], w: u32, h: u32) -> Self {
        let bb = AABB::new(0, 0, w, h);
        Self::split(pts, 0, bb)
    }

    fn split(pts: &mut [[u32; 2]], axis: usize, bb: AABB) -> Self {
        pts.sort_by(|u, v| u[axis].partial_cmp(&v[axis]).unwrap());

        let mid = pts.len() / 2;
        let split_coord = pts[mid][axis];
        let (left_bb, right_bb) = bb.split_along(axis, split_coord);
        let next_axis = (axis + 1) % 2;
        match pts.split_at_mut(mid) {
            ([], [pt]) => KDTree::Leaf(pt.clone(), bb),
            (left, [pt]) => KDTree::Split(
                box Self::split(left, next_axis, left_bb),
                box Empty,
                axis,
                pt.clone(),
                bb,
            ),
            (left, [pt, right @ ..]) => KDTree::Split(
                box Self::split(left, next_axis, left_bb),
                box Self::split(right, next_axis, right_bb),
                axis,
                pt.clone(),
                bb,
            ),
            _ => Empty,
        }
    }

    pub fn plot_to(&self, path: &str) -> ImageResult<()> {
        let img = &mut match self {
            Split(_, _, _, _, bb) => RgbImage::from_pixel(bb.d[0], bb.d[1], WHITE),
            _ => return Ok(()),
        };
        self.plot_inner(img, 0);
        img.save(path)
    }

    fn plot_inner(&self, img: &mut RgbImage, axis: usize) {
        match self {
            Empty => {}
            Leaf(pt, _) => draw_point(img, pt.clone(), 5, RED),
            Split(left, right, _, pt, bb) => {
                let next_axis = (axis + 1) % 2;

                let mut p1 = bb.p.clone();
                p1[axis] = pt[axis];
                let mut p2 = p1.clone();
                p2[next_axis] += bb.d[next_axis];

                draw_line(img, next_axis, p1, p2, BLUE);
                draw_point(img, pt.clone(), 5, RED);

                left.plot_inner(img, next_axis);
                right.plot_inner(img, next_axis);
            }
        }
    }

    pub fn nearest(&self, pt: [u32; 2]) -> [u32; 2] {
        fn dist([x1, y1]: [u32; 2], [x2, y2]: [u32; 2]) -> u32 {
            ((x1 as i32 - x2 as i32).pow(2) + (y1 as i32 - y2 as i32).pow(2)) as u32
        }

        match self {
            Empty => panic!(),
            Leaf(pt, _) => pt.clone(),
            Split(left, box Empty, _, _, _) => left.nearest(pt),
            Split(box Empty, right, _, _, _) => right.nearest(pt),
            Split(left, right, axis, split_pt, _) => {
                let (p, q) = if split_pt[*axis] > pt[*axis] {
                    (left, right)
                } else {
                    (right, left)
                };

                let ppt = p.nearest(pt);
                let dist_to_split_line = (split_pt[*axis] as i32 - pt[*axis] as i32).abs() as u32;
                let dist_to_ppt = (dist(ppt, pt) as f64).sqrt() as u32;
                let pts = if dist_to_split_line > dist_to_ppt {
                    vec![ppt, split_pt.clone()]
                } else {
                    vec![ppt, q.nearest(pt), split_pt.clone()]
                };

                pts.into_iter()
                    .map(|p| (p, dist(p, pt)))
                    .min_by_key(|(_, d)| *d)
                    .unwrap()
                    .0
            }
        }
    }
}
