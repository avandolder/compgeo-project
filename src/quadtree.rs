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
            [pt] => Leaf(pt.clone()),
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
                        box Self::split(nw_pts, nw_bb.clone()),
                        box Self::split(ne_pts, ne_bb.clone()),
                        box Self::split(sw_pts, sw_bb.clone()),
                        box Self::split(se_pts, se_bb.clone()),
                    ],
                    bb,
                )
            }
        }
    }

    pub fn plot_to(&self, path: &str) -> ImageResult<()> {
        let img = &mut match self {
            Quad(_, bb) => RgbImage::from_pixel(bb.d[0], bb.d[1], WHITE),
            _ => return Ok(()),
        };
        self.plot_inner(img);
        img.save(path)
    }

    fn plot_inner(&self, img: &mut RgbImage) {
        match self {
            Empty => {}
            Leaf(pt) => draw_point(img, pt.clone(), 5, RED),
            // Draw the bounding box of each quadrant, then recurse.
            Quad(qs, bb) => {
                let (cx, cy) = bb.center();
                draw_line(img, 0, [bb.p[0], cy], [bb.p[0] + bb.d[0], cy], BLUE);
                draw_line(img, 1, [cx, bb.p[1]], [cx, bb.p[1] + bb.d[1]], BLUE);
                qs.iter().for_each(|q| q.plot_inner(img));
            }
        }
    }
}
