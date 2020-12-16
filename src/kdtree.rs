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
            ([], [pt]) => KDTree::Leaf(*pt, bb),
            (left, [pt]) => KDTree::Split(
                box Self::split(left, next_axis, left_bb),
                box Empty,
                axis,
                *pt,
                bb,
            ),
            (left, [pt, right @ ..]) => KDTree::Split(
                box Self::split(left, next_axis, left_bb),
                box Self::split(right, next_axis, right_bb),
                axis,
                *pt,
                bb,
            ),
            _ => Empty,
        }
    }

    pub fn plot_to(&self, path: &str) -> ImageResult<()> {
        let img = &mut match self {
            Split(_, _, _, _, AABB { d: [w, h], .. }) => RgbImage::from_pixel(*w, *h, WHITE),
            _ => return Ok(()),
        };
        self.plot_inner(img, 0, 0, 100);
        img.save(path)
    }

    pub fn plot_points(&self, path: String, pts: &[[u32; 2]]) {
        for i in 0.. {
            let img = &mut match self {
                Split(_, _, _, _, AABB { d: [w, h], .. }) => RgbImage::from_pixel(*w, *h, WHITE),
                _ => return,
            };
            if self.plot_inner(img, 0, 0, i) {
                break;
            }
            for pt in pts {
                draw_point(img, *pt, 5, RED);
            }
            img.save(path.clone() + &i.to_string() + ".png").unwrap();
        }
    }

    fn plot_inner(&self, img: &mut RgbImage, axis: usize, depth: usize, limit: usize) -> bool {
        if depth > limit {
            return false;
        }

        match self {
            Empty | Leaf(_, _) => true,
            Split(left, right, _, pt, bb) => {
                let next_axis = (axis + 1) % 2;

                let mut p1 = bb.p;
                p1[axis] = pt[axis];
                let mut p2 = p1;
                p2[next_axis] += bb.d[next_axis];

                draw_line(img, next_axis, p1, p2, BLUE);

                let left_done = left.plot_inner(img, next_axis, depth + 1, limit);
                let right_done = right.plot_inner(img, next_axis, depth + 1, limit);
                left_done && right_done
            }
        }
    }

    pub fn nearest(&self, pt: [u32; 2]) -> [u32; 2] {
        fn dist([x1, y1]: [u32; 2], [x2, y2]: [u32; 2]) -> u32 {
            ((x1 as i32 - x2 as i32).pow(2) + (y1 as i32 - y2 as i32).pow(2)) as u32
        }

        match self {
            Empty => panic!(),
            Leaf(pt, _) => *pt,
            Split(left, box Empty, _, _, _) => left.nearest(pt),
            Split(box Empty, right, _, _, _) => right.nearest(pt),
            Split(left, right, axis, split_pt, _) => {
                let (p, q) = if split_pt[*axis] > pt[*axis] {
                    (left, right)
                } else {
                    (right, left)
                };

                let ppt = p.nearest(pt);
                let dist_to_split = (split_pt[*axis] as i32 - pt[*axis] as i32).abs() as u32;
                let dist_to_ppt = (dist(ppt, pt) as f64).sqrt() as u32;
                let pts = if dist_to_split > dist_to_ppt {
                    vec![ppt, *split_pt]
                } else {
                    vec![ppt, q.nearest(pt), *split_pt]
                };

                pts.into_iter()
                    .map(|p| (p, dist(p, pt)))
                    .min_by_key(|(_, d)| *d)
                    .unwrap()
                    .0
            }
        }
    }

    pub fn range_search(&self, start: [u32; 2], end: [u32; 2]) -> Vec<[u32; 2]> {
        let mut pts = vec![];
        let bb = AABB {
            p: start,
            d: [end[0] - start[0], end[1] - start[1]],
        };
        self.range_inner(bb, &mut pts);
        pts
    }

    fn range_inner(&self, bb: AABB, pts: &mut Vec<[u32; 2]>) {
        match self {
            Leaf(pt, _) if bb.contains(*pt) => pts.push(*pt),
            Split(left, right, axis, pt, sbb) => {
                if bb.contains(*pt) {
                    pts.push(*pt);
                }
                if pt[*axis] > bb.p[*axis] && bb.p[*axis] + bb.d[*axis] >= sbb.p[*axis] {
                    left.range_inner(bb, pts)
                }
                if pt[*axis] <= bb.p[*axis] + bb.d[*axis]
                    && bb.p[*axis] + bb.d[*axis] > sbb.p[*axis]
                {
                    right.range_inner(bb, pts)
                }
            }
            _ => (),
        }
    }
}
