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

    let tree = QuadTree::new(pts.clone().as_mut_slice(), 1000, 1000);
    tree.plot_to("img/quad.png")?;

    let tree = KDTree::new(pts.clone().as_mut_slice(), 1000, 1000);
    println!("{:?}", tree.nearest([500, 500]));
    tree.plot_to("img/kd.png")?;

    Ok(())
}
