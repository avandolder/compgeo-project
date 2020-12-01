#![feature(box_syntax, box_patterns)]

mod aabb;
mod draw;
mod kdtree;
mod quadtree;

use std::error::Error;

use rand::{thread_rng, Rng};

use kdtree::KDTree;
use quadtree::QuadTree;

fn main() -> Result<(), Box<dyn Error>> {
    let pts = (0..100)
        .map(|_| {
            [
                thread_rng().gen_range(0, 1000),
                thread_rng().gen_range(0, 1000),
            ]
        })
        .collect::<Vec<_>>();

    let qtree = QuadTree::new(pts.clone().as_mut_slice(), 1000, 1000);
    qtree.plot_to("img/quad.png")?;

    let ktree = KDTree::new(pts.clone().as_mut_slice(), 1000, 1000);
    println!("{:?}", ktree);
    println!("{:?}", ktree.nearest([500, 500]));
    //tree.plot_to("img/kd.png")?;

    fn dist([x1, y1]: [u32; 2], [x2, y2]: [u32; 2]) -> u32 {
        ((x1 as i32 - x2 as i32).pow(2) + (y1 as i32 - y2 as i32).pow(2)) as u32
    }
    let nn = pts
        .iter()
        .map(|pt| (*pt, dist(*pt, [500, 500])))
        .min_by_key(|(_, d)| *d)
        .unwrap()
        .0;
    println!("{:?}", nn);

    let mut range = qtree.range_search([400, 400], [600, 600]);
    range.sort();
    println!("{:?}", range);

    let mut range = ktree.range_search([400, 400], [600, 600]);
    range.sort();
    println!("{:?}", range);

    let bb = aabb::AABB::new(400, 400, 200, 200);
    let mut range = pts
        .iter()
        .filter(|pt| bb.contains(**pt))
        .collect::<Vec<_>>();
    range.sort();
    println!("{:?}", range);

    Ok(())
}
