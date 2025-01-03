mod exp1;
mod exp2;
mod util;

use util::scale_no_interpolation;

fn main() {
    let img = scale_no_interpolation(&exp2::gen(256, 256), 4);
    img.save("img.png").unwrap();
}
