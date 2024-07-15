use num_complex::*;

pub fn compute_dft_to(dst: &mut [Complex64], src: &[Complex64]) {
    let un = dst.len();
    dst.iter_mut().enumerate().for_each(|(k, uxk)| *uxk = compute_uxk(src, k, un))
}

fn compute_uxk(lx: &[Complex64], k: usize, un: usize) -> Complex64 {
    // LIGHT:
    // let t = -iτ(k/N)
    // .
    //      N-1      tn
    // X  =  Σ  x ⋅ e
    //  k   n=0  n

    let t = -Complex64::i() * core::f64::consts::TAU * (k as f64 / un as f64);

    lx.iter().enumerate().map(|(n, xn)| {
        xn * (t * n as f64).exp()
    }).sum()
}
