use num_complex::*;

pub fn compute_dft_to(dst: &mut [Complex64], src: &[Complex64]) {
    compute_dft_to_starting_from(dst, src, 0.0)
}

pub fn compute_dft_to_starting_from(dst: &mut [Complex64], src: &[Complex64], start: f64) {
    let un = dst.len();
    dst.iter_mut().enumerate().for_each(|(k, uxk)| *uxk = compute_uxk(src, k as f64 + start, un))
}

pub fn compute_uxk(lx: &[Complex64], k: f64, un: usize) -> Complex64 {
    // LIGHT:
    // let t = -iτ(k/N)
    // .
    //      N-1      tn
    // X  =  Σ  x ⋅ e
    //  k   n=0  n

    let t = -Complex64::i() * core::f64::consts::TAU * (k / un as f64);

    lx.iter().enumerate().map(|(n, xn)| {
        xn * (t * n as f64).exp()
    }).sum()
}
