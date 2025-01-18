use std::sync::mpsc;

use image::{Rgba, RgbaImage};
use rayon::prelude::*;

use crate::util::scale_no_interpolation;

pub fn gen() {
    const WIDTH: u32 = 256;
    const HEIGHT: u32 = 256;
    const SCALE: u32 = 4;

    let mut img = RgbaImage::new(WIDTH, HEIGHT);

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
    let mut img = RgbaImage::new(WIDTH, HEIGHT);

    let (tx, rx) = mpsc::channel();

    let mut positions = (0..HEIGHT)
        .flat_map(|y| (0..WIDTH).map(move |x| (x, y)))
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
                rx = rx.wrapping_add_signed(fastrand::i32(-1..=1)) % WIDTH;
                ry = ry.wrapping_add_signed(fastrand::i32(-1..=1)) % HEIGHT;
            }

            s.send((positions, current_color)).unwrap();
        });

    for (positions, color) in rx {
        for (x, y) in positions {
            img.put_pixel(x, y, color);
        }
    }

    scale_no_interpolation(&img, SCALE).save("img.png").unwrap();
}
