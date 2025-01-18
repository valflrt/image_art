use image::{Rgba, RgbaImage};

use crate::mat::Mat2D;

fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}

pub fn gen(width: u32, height: u32) -> RgbaImage {
    let mut gcd_img = Mat2D::filled_with(None, width as usize, height as usize);

    let mut max = 0;
    for i in 0..width {
        for j in 0..height {
            let d = gcd(i, j);
            max = d.max(max);
            gcd_img
                .set((i as usize, (height - j - 1) as usize), Some(d))
                .unwrap();
        }
    }

    let mut img = RgbaImage::new(width, height);

    for i in 0..width {
        for j in 0..height {
            if let &Some(d) = gcd_img.get((i as usize, j as usize)).unwrap() {
                let v = 1. - d as f64 / max as f64;

                img.put_pixel(i, j, hsv_to_rgb(255. * v, 1., 1.));
            }
        }
    }

    img
}

pub fn hsv_to_rgb(hue: f64, saturation: f64, value: f64) -> Rgba<u8> {
    fn is_between(value: f64, min: f64, max: f64) -> bool {
        min <= value && value < max
    }

    let c = value * saturation;
    let h = hue / 60.0;
    let x = c * (1.0 - ((h % 2.0) - 1.0).abs());
    let m = value - c;

    let (r, g, b): (f64, f64, f64) = if is_between(h, 0.0, 1.0) {
        (c, x, 0.0)
    } else if is_between(h, 1.0, 2.0) {
        (x, c, 0.0)
    } else if is_between(h, 2.0, 3.0) {
        (0.0, c, x)
    } else if is_between(h, 3.0, 4.0) {
        (0.0, x, c)
    } else if is_between(h, 4.0, 5.0) {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    Rgba([
        ((r + m) * 255.0) as u8,
        ((g + m) * 255.0) as u8,
        ((b + m) * 255.0) as u8,
        255,
    ])
}
