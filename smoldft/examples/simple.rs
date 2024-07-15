use smoldft::*;
use num_complex::c64;

fn main() {
    let src = &[
        c64(-1.0, -1.0),
        c64(0.0, -1.0),
        c64(1.0, -1.0),
        c64(1.0, 0.0),
        c64(1.0, 1.0),
        c64(0.0, 1.0),
        c64(-1.0, 1.0),
        c64(-1.0, 0.0),
        c64(-1.0, -1.0),
    ];
    let mut dst = [c64(0.0, 0.0); 3];

    compute_dft_to(&mut dst, src);
    dst.iter().for_each(|x| println!("{x}"));
}
