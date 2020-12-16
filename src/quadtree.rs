use std::fmt::Debug;

use image::{ImageResult, RgbImage};

use crate::aabb::AABB;
use crate::draw::{draw_line, draw_point, BLUE, RED, WHITE};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum QuadTree {
    Empty,
    Leaf([u32; 2]),
    Quad([Box<QuadTree>; 4], AABB),
}

use QuadTree::*;

impl QuadTree {
    pub fn new(pts: &mut [[u32; 2]], w: u32, h: u32) -> Self {
        let bb = AABB::new(0, 0, w, h);
        Self::split(pts, bb)
    }

    fn split(pts: &mut [[u32; 2]], bb: AABB) -> Self {
        match pts {
            [] => Empty,
            [pt] => Leaf(*pt),
            pts => {
                let (cx, cy) = bb.center();

                pts.sort_by_key(|[_, y]| *y);
                let n_count = pts.iter().take_while(|[_, y]| *y < cy).count();
                let (n_pts, s_pts) = pts.split_at_mut(n_count);

                // Extract the points in each quadrant.
                n_pts.sort_by_key(|[x, _]| *x);
                let nw_count = n_pts.iter().take_while(|[x, _]| *x < cx).count();
                let (nw_pts, ne_pts) = n_pts.split_at_mut(nw_count);

                s_pts.sort_by_key(|[x, _]| *x);
                let sw_count = s_pts.iter().take_while(|[x, _]| *x < cx).count();
                let (sw_pts, se_pts) = s_pts.split_at_mut(sw_count);

                let [nw_bb, ne_bb, sw_bb, se_bb] = bb.quads();

                Quad(
                    [
                        box Self::split(nw_pts, nw_bb),
                        box Self::split(ne_pts, ne_bb),
                        box Self::split(sw_pts, sw_bb),
                        box Self::split(se_pts, se_bb),
                    ],
                    bb,
                )
            }
        }
    }

    pub fn plot_points(&self, path: String, pts: &[[u32; 2]]) {
        for i in 0.. {
            let img = &mut match self {
                Quad(_, bb) => RgbImage::from_pixel(bb.d[0], bb.d[1], WHITE),
                _ => return,
            };

            if self.plot_inner(img, 0, i) {
                break;
            }
            for pt in pts {
                draw_point(img, *pt, 5, RED);
            }
            img.save(path.clone() + &i.to_string() + ".png").unwrap();
        }
    }

    pub fn plot_to(&self, path: &str) -> ImageResult<()> {
        let img = &mut match self {
            Quad(_, bb) => RgbImage::from_pixel(bb.d[0], bb.d[1], WHITE),
            _ => return Ok(()),
        };
        self.plot_inner(img, 0, 1000);
        img.save(path)
    }

    fn plot_inner(&self, img: &mut RgbImage, depth: usize, limit: usize) -> bool {
        if depth > limit { return false; }

        match self {
            Empty | Leaf(_) => true,
            // Draw the bounding box of each quadrant, then recurse.
            Quad(qs, bb) => {
                let (cx, cy) = bb.center();
                let AABB {
                    p: [x, y],
                    d: [w, h],
                } = *bb;
                draw_line(img, 0, [x, cy], [x + w, cy], BLUE);
                draw_line(img, 1, [cx, y], [cx, y + h], BLUE);
                qs.iter()
                    .map(|q| q.plot_inner(img, depth + 1, limit))
                    .fold(true, |x, y| x && y)
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
            Leaf(pt) if bb.contains(*pt) => pts.push(*pt),
            Quad(qs, b) if b.intersects(bb) => qs.iter().for_each(|q| q.range_inner(bb, pts)),
            _ => (),
        }
    }
}
