#![feature(box_syntax, box_patterns)]

mod aabb;
mod draw;
mod kdtree;
mod quadtree;

use std::error::Error;
use std::iter;

use rand::{thread_rng, Rng};

use kdtree::KDTree;
use quadtree::QuadTree;

const WIDTH: u32 = 1000;
const HEIGHT: u32 = 1000;

fn main() -> Result<(), Box<dyn Error>> {
    fn random_point() -> [u32; 2] {
        [
            thread_rng().gen_range(0, WIDTH),
            thread_rng().gen_range(0, HEIGHT),
        ]
    }

    let pts = iter::repeat_with(random_point)
        .take(100)
        .collect::<Vec<_>>();

    let qtree = QuadTree::new(pts.clone().as_mut_slice(), WIDTH, HEIGHT);
    qtree.plot_points("img/quad".to_string(), &pts);

    let ktree = KDTree::new(pts.clone().as_mut_slice(), WIDTH, HEIGHT);
    ktree.plot_points("img/kd".to_string(), &pts);

    let point = random_point();
    println!("The nearest neighbour of {:?} is {:?}", point, ktree.nearest(point));

    let mut range = qtree.range_search([400, 400], [600, 600]);
    range.sort();
    println!("{:?}", range);

    let mut range = ktree.range_search([400, 400], [600, 600]);
    range.sort();
    println!("{:?}", range);

    Ok(())
}
