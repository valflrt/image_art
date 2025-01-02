use image::RgbaImage;

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
