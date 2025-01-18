use image::{Rgba, RgbaImage};

use crate::{
    mat::Mat2D,
    util::{hsv, scale_no_interpolation},
};

pub fn gen() {
    const WIDTH: u32 = 256;
    const HEIGHT: u32 = 256;
    const SCALE: u32 = 4;

    let mut gcd_img = Mat2D::filled_with(None, WIDTH as usize, HEIGHT as usize);

    let mut max = 0;
    for i in 0..WIDTH {
        for j in 0..i + 1 {
            let d = gcd(i + 1, j + 1);
            max = d.max(max);
            gcd_img.set((i as usize, j as usize), Some(d)).unwrap();
        }
    }

    let mut img = RgbaImage::new(WIDTH, HEIGHT);

    for i in 0..WIDTH {
        let is_prime = (0..i).fold(true, |acc, j| {
            acc && gcd_img
                .get((i as usize, j as usize))
                .unwrap()
                .map(|v| v == 1)
                .unwrap_or(false)
        });
        for j in 0..i + 1 {
            if let &Some(d) = gcd_img.get((i as usize, j as usize)).unwrap() {
                if d == 1 {
                    let color = if is_prime {
                        Rgba([220, 220, 220, 255])
                    } else {
                        Rgba([20, 20, 20, 255])
                    };
                    img.put_pixel(i, HEIGHT - j - 1, color);
                    img.put_pixel(j, HEIGHT - i - 1, color);
                } else {
                    let v = 1. - d as f64 / max as f64;
                    img.put_pixel(i, HEIGHT - j - 1, hsv(255. * v, 1., 1.));
                    img.put_pixel(j, HEIGHT - i - 1, hsv(255. * v, 1., 1.));
                }
            }
        }
    }

    scale_no_interpolation(&img, SCALE).save("img.png").unwrap();
}

fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}
