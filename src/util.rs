use image::{Rgba, RgbaImage};

pub fn scale_no_interpolation(img: &RgbaImage, factor: u32) -> RgbaImage {
    let (width, height) = img.dimensions();

    let (new_width, new_height) = (width * factor, height * factor);
    let mut new_img = RgbaImage::new(new_width, new_height);

    for y in 0..height {
        for x in 0..width {
            let &color = img.get_pixel(x, y);

            for dy in 0..factor {
                for dx in 0..factor {
                    new_img.put_pixel(x * factor + dx, y * factor + dy, color);
                }
            }
        }
    }

    new_img
}

pub fn hsv(hue: f64, saturation: f64, value: f64) -> Rgba<u8> {
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
