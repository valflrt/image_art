use std::{fs, sync::mpsc};

use image::{Rgba, RgbaImage};
use rayon::prelude::*;

use crate::util::scale_no_interpolation;

#[allow(dead_code)]
pub enum ColorSource {
    FromFile,
    Random { save_colors: bool },
}

pub fn gen() {
    const WIDTH: u32 = 256;
    const HEIGHT: u32 = 256;
    // const WIDTH: u32 = 1920 / 4;
    // const HEIGHT: u32 = 1080 / 4;
    const COLOR_SOURCE: ColorSource = ColorSource::Random { save_colors: true };
    // const COLOR_SOURCE: ColorSource = ColorSource::FromFile;
    const SCALE: u32 = 4;

    let colors = match COLOR_SOURCE {
        ColorSource::Random { save_colors } => {
            const COLOR_COUNT: usize = 8;

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

            if save_colors {
                fs::write(
                    "img.colors.json",
                    serde_json::to_string_pretty(&colors.iter().map(|c| c.0).collect::<Vec<_>>())
                        .unwrap(),
                )
                .unwrap();
            }

            colors
        }
        ColorSource::FromFile => serde_json::from_reader::<_, Vec<[u8; 4]>>(
            fs::OpenOptions::new()
                .read(true)
                .open("img.colors.json")
                .unwrap(),
        )
        .unwrap()
        .iter()
        .map(|&c| Rgba(c))
        .collect::<Vec<_>>(),
    };

    let mut img = RgbaImage::new(WIDTH, HEIGHT);

    img.par_pixels_mut()
        .for_each(|p| *p = *fastrand::choice(&colors).unwrap());

    // apply effect1

    const AVG_K_SIZE: i32 = 32;
    const MD_K_SIZE: i32 = 24;
    effect1(&mut img, AVG_K_SIZE, MD_K_SIZE);

    scale_no_interpolation(&img, SCALE).save("img.png").unwrap();
}

pub fn effect1(img: &mut RgbaImage, avg_k_size: i32, md_k_size: i32) {
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
