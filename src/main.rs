mod exp1;
mod util;

use util::scale_no_interpolation;

fn main() {
    let img = scale_no_interpolation(
        &exp1::gen(256, 256, exp1::ColorSource::Random { save_colors: true }),
        4,
    );
    img.save("img.png").unwrap();
}
