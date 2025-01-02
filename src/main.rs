mod util;

use std::sync::mpsc;

use image::{Rgba, RgbaImage};
use rayon::iter::{ParallelBridge, ParallelIterator};

use util::scale_no_interpolation;

fn main() {
    let img = scale_no_interpolation(&experiment1(256, 256), 4);
    img.save("outputs/experiment1.png").unwrap();
}

fn experiment1(width: u32, height: u32) -> RgbaImage {
    // init image

    let mut img = RgbaImage::new(width, height);

    for p in img.pixels_mut() {
        *p = Rgba([
            fastrand::u8(10..225),
            fastrand::u8(10..225),
            fastrand::u8(10..225),
            255,
        ]);
    }

    // quantize

    const STEP: u8 = 255 / 4;
    for (_, _, pixel) in img.enumerate_pixels_mut() {
        for channel in &mut pixel.0 {
            *channel = (*channel / STEP) * STEP;
        }
    }

    // apply effect1

    const AVG_K_SIZE: i32 = 32;
    const MD_K_SIZE: i32 = 24;
    effect1(&mut img, AVG_K_SIZE, MD_K_SIZE);

    // make colors more pastel

    for v in img.iter_mut() {
        *v = (((*v as f32 / 255. + 0.3).min(1.) * 0.8 + 0.2) * 255.) as u8;
    }

    img
}

fn effect1(img: &mut RgbaImage, avg_k_size: i32, md_k_size: i32) {
    // avg_k_size: Kernel size for average
    // md_k_size: Kernel size for finding most distant color

    let (width, height) = img.dimensions();

    let old_img = img.clone();

    let (tx, rx) = mpsc::channel();

    (0..height)
        .flat_map(|y| (0..width).map(move |x| (x, y)))
        .par_bridge()
        .for_each_with(tx, |s, (x, y)| {
            let (r_sum, g_sum, b_sum, sum) = (-avg_k_size..=avg_k_size)
                .flat_map(|j| (-avg_k_size..=avg_k_size).map(move |i| (i, j)))
                .fold(
                    (0., 0., 0., 0.),
                    |(acc_r, acc_g, acc_b, acc_sum), (i, j)| {
                        let xx = x.wrapping_add_signed(i) % width;
                        let yy = y.wrapping_add_signed(j) % height;

                        let &Rgba([r, g, b, _]) = old_img.get_pixel(xx, yy);

                        (
                            acc_r + r as f32,
                            acc_g + g as f32,
                            acc_b + b as f32,
                            acc_sum + 1.,
                        )
                    },
                );

            let center_color = Rgba([
                (r_sum / sum).floor() as u8,
                (g_sum / sum).floor() as u8,
                (b_sum / sum).floor() as u8,
                255,
            ]);

            let mut most_distant = center_color;
            let mut max_distance = 0;

            for j in -md_k_size..=md_k_size {
                for i in -md_k_size..=md_k_size {
                    let xx = x.saturating_add_signed(i).min(width - 1);
                    let yy = y.saturating_add_signed(j).min(height - 1);

                    let &color = old_img.get_pixel(xx, yy);

                    let distance = (center_color[0] - color[0]).pow(2)
                        + (center_color[1] - color[1]).pow(2)
                        + (center_color[2] - color[2]).pow(2);

                    if distance > max_distance {
                        most_distant = color;
                        max_distance = distance;
                    }
                }
            }

            s.send((x, y, most_distant)).unwrap();
        });

    for (x, y, color) in rx {
        img.put_pixel(x, y, color);
    }
}
