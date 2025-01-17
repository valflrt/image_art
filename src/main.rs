mod exp1;
mod exp2;
mod exp3;
mod mat;
mod util;

use util::scale_no_interpolation;

fn main() {
    let img = scale_no_interpolation(&exp3::gen(64, 64), 8);
    img.save("img.png").unwrap();
}
