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

    // effect

    /// Kernel size for average
    const AVG_K_SIZE: i32 = 32;
    /// Kernel size for finding most distant
    const MD_K_SIZE: i32 = 24;

    let mut new_img = RgbaImage::new(width, height);

    let (tx, rx) = mpsc::channel();

    (0..height)
        .flat_map(|y| (0..width).map(move |x| (x, y)))
        .par_bridge()
        .for_each_with(tx, |s, (x, y)| {
            let (r_sum, g_sum, b_sum, sum) = (-AVG_K_SIZE..=AVG_K_SIZE)
                .flat_map(|j| (-AVG_K_SIZE..=AVG_K_SIZE).map(move |i| (i, j)))
                .fold(
                    (0., 0., 0., 0.),
                    |(acc_r, acc_g, acc_b, acc_sum), (i, j)| {
                        let xx = x.wrapping_add_signed(i) % width;
                        let yy = y.wrapping_add_signed(j) % height;

                        let &Rgba([r, g, b, _]) = img.get_pixel(xx, yy);

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

            for j in -MD_K_SIZE..=MD_K_SIZE {
                for i in -MD_K_SIZE..=MD_K_SIZE {
                    let xx = x.saturating_add_signed(i).min(width - 1);
                    let yy = y.saturating_add_signed(j).min(height - 1);

                    let &color = img.get_pixel(xx, yy);

                    let distance = (center_color[0] - color[0]).pow(2)
                        + (center_color[1] - color[1]).pow(2)
                        + (center_color[2] - color[2]).pow(2);

                    if distance > max_distance {
                        most_distant = color;
                        max_distance = distance;
                    }
                }
            }

            // make color more pastel

            let Rgba([r, g, b, a]) = most_distant;

            let r = (r as f32 / 255. + 0.2).min(1.) * 0.9 + 0.1;
            let g = (g as f32 / 255. + 0.2).min(1.) * 0.9 + 0.1;
            let b = (b as f32 / 255. + 0.2).min(1.) * 0.9 + 0.1;

            s.send((
                x,
                y,
                Rgba([(r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8, a]),
            ))
            .unwrap();
        });

    for (x, y, color) in rx {
        new_img.put_pixel(x, y, color);
    }

    new_img
}
