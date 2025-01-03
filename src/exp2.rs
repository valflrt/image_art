use std::sync::mpsc;

use image::{Rgba, RgbaImage};
use rayon::prelude::*;

pub fn gen(width: u32, height: u32) -> RgbaImage {
    let mut img = RgbaImage::new(width, height);

    const COLOR_COUNT: usize = 256;
    let colors = (0..COLOR_COUNT)
        .map(|_| {
            Rgba([
                fastrand::u8(20..250),
                fastrand::u8(20..250),
                fastrand::u8(20..250),
                255,
            ])
        })
        .collect::<Vec<_>>();

    img.par_pixels_mut()
        .for_each(|p| *p = *fastrand::choice(&colors).unwrap());

    let old_img = img;
    let mut img = RgbaImage::new(width, height);

    let (tx, rx) = mpsc::channel();

    let mut positions = (0..height)
        .flat_map(|y| (0..width).map(move |x| (x, y)))
        .collect::<Vec<_>>();

    fastrand::shuffle(positions.as_mut_slice());

    positions
        .par_iter()
        .copied()
        .for_each_with(tx, |s, (x, y)| {
            let &current_color = old_img.get_pixel(x, y);

            let mut positions = Vec::new();

            let mut rx = x;
            let mut ry = y;

            while old_img.get_pixel(rx, ry) != &current_color || (rx, ry) == (x, y) {
                positions.push((rx, ry));
                rx = rx.wrapping_add_signed(fastrand::i32(-1..=1)) % width;
                ry = ry.wrapping_add_signed(fastrand::i32(-1..=1)) % height;
            }

            s.send((positions, current_color)).unwrap();
        });

    for (positions, color) in rx {
        for (x, y) in positions {
            img.put_pixel(x, y, color);
        }
    }

    img
}
