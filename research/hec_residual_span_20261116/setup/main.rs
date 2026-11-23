#[path = "../multi/algebra.rs"]
mod algebra;
mod fast;
use hec_native_ic::{counted, fp::Fp, instance::factor_base, jac::Curve, poly::Poly};
fn main() {
    println!("g\tp\tc\tm\tz\told_p_redc\tnew_p_redc\told_p_add\tnew_p_add\texact");
    let mut total = 0;
    for (g, p) in [(2usize, 101u64), (3, 113), (4, 109)] {
        for cc in 1..p {
            let mut v = vec![0; 2 * g + 2];
            v[0] = cc;
            v[2 * g + 1] = 1;
            let c = Curve {
                fp: Fp::new(p),
                f: Poly::from_coeffs(&v),
                g,
            };
            counted::reset();
            let fb = factor_base(&c);
            let (z, a) = algebra::character(&c);
            let old = counted::snapshot();
            counted::reset();
            let (ff, zz, aa) = fast::prepare(&c);
            let new = counted::snapshot();
            assert_eq!(ff, fb);
            assert_eq!(zz, z);
            assert_eq!(aa, a);
            let x = old[&p];
            let y = new[&p];
            println!(
                "{g}\t{p}\t{cc}\t{}\t{z}\t{}\t{}\t{}\t{}\ttrue",
                fb.len(),
                x[8],
                y[8],
                x[1],
                y[1]
            );
            total += 1
        }
    }
    assert_eq!(total, 320)
}
