use image::{Rgb, RgbImage};

use std::iter::repeat;

use itertools::Itertools;

pub const BLUE: Rgb<u8> = Rgb([0, 0, 255]);
pub const RED: Rgb<u8> = Rgb([255, 0, 0]);
pub const WHITE: Rgb<u8> = Rgb([255, 255, 255]);

pub fn draw_line(
    img: &mut RgbImage,
    axis: usize,
    [sx, sy]: [u32; 2],
    [ex, ey]: [u32; 2],
    color: Rgb<u8>,
) {
    let w = img.width();
    let h = img.height();

    if axis == 0 {
        for (x, y) in (sx..=ex).zip(repeat(sy)).filter(|(x, y)| *x < w && *y < h) {
            img.put_pixel(x as u32, y as u32, color);
        }
    } else if axis == 1 {
        for (x, y) in repeat(sx).zip(sy..ey).filter(|(x, y)| *x < w && *y < h) {
            img.put_pixel(x as u32, y as u32, color);
        }
    }
}

pub fn draw_point(img: &mut RgbImage, [x, y]: [u32; 2], size: i32, color: Rgb<u8>) {
    let (w, h) = (img.width() as i32, img.height() as i32);
    let (min, max) = (-(size / 2), size / 2);
    for (x, y) in (min..=max)
        .cartesian_product(min..=max)
        .map(|(dx, dy)| (x as i32 + dx, y as i32 + dy))
        .filter(|(x, y)| *x >= 0 && *x < w && *y >= 0 && *y < h)
    {
        img.put_pixel(x as u32, y as u32, color);
    }
}
