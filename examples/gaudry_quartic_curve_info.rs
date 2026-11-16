//! The `S₅` study curve's symmetries, for its rho reference.
//!
//! ```text
//! cargo run --release --example gaudry_quartic_curve_info -- 269 1
//! ```
//!
//! Rebuilds `gaudry_quartic_c4_options`' curve (`Curve4::random`, the first
//! draw of `StdRng::seed_from_u64(seed)`) and reports `j(E)` and whether it
//! lies in `F_{p²}` or `F_p` (`j^{p²} = j`, `j^p = j`): only then could the
//! `p`- or `p²`-power Frobenius map `E` to a curve isomorphic to it and fold a
//! rho walk. `a ≠ 0` and `b ≠ 0` rule out the extra automorphisms of
//! `j = 0` and `j = 1728`.

use std::env;

use crypto_lib::cryptanalysis::gaudry_quartic::Curve4;
use rand::rngs::StdRng;
use rand::SeedableRng;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let p: u64 = args.first().map_or(269, |v| v.parse().expect("p"));
    let seed: u64 = args.get(1).map_or(1, |v| v.parse().expect("seed"));
    let mut rng = StdRng::seed_from_u64(seed);
    let curve = Curve4::random(p, &mut rng);
    let f = &curve.f;
    let a3 = f.mul(&f.sq(&curve.a), &curve.a);
    let four_a3 = f.scale(&a3, 4);
    let disc = f.add(&four_a3, &f.scale(&f.sq(&curve.b), 27));
    let j = f.mul(&f.scale(&four_a3, 1728), &f.inv(&disc));
    let jp = f.pow(&j, u128::from(p));
    let jp2 = f.pow(&j, u128::from(p * p));
    println!(
        "{{\"p\": {p}, \"seed\": {seed}, \"a\": {:?}, \"b\": {:?}, \"j\": {:?}, \"a_zero\": {}, \"b_zero\": {}, \"j_in_fp2\": {}, \"j_in_fp\": {}}}",
        curve.a.0,
        curve.b.0,
        j.0,
        curve.a.is_zero(),
        curve.b.is_zero(),
        jp2 == j,
        jp == j
    );
}
