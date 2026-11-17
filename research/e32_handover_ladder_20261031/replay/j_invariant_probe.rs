//! E32 validator probe (not a trial): j(E) of every E32 instance.
//!
//! Built as an example of the validator's export of the recorded binary commit
//! (e757710521), after and apart from the e32_ladder replay binary.  For each
//! (p, seed) of the ladder (33 paired curves and the unpaired p = 4337 seed 1)
//! it regenerates the instance with the library's own generator
//! (generate_gls_klein_instance(p, seed, 16)), computes
//! j = 1728 * 4a^3 / (4a^3 + 27b^2) in F_p4 with the library's arithmetic and
//! tests j^p = j (j in F_p), j^(p^2) = j (j in F_p2), j = 0 and j = 1728.
//! j in F_p2 \ F_p and j not in {0, 1728} means Aut(E) = {±1} and the GLS map
//! psi is the only Frobenius-type endomorphism: <±psi> is the largest fold.
//! It also prints n, #E, h and the planted d, which validate_e32 compares with
//! the records.  It uses library code, so it is a consistency check of the
//! instances, not an independent implementation.
//!
//! ```text
//! cargo build --release --offline --locked --example j_invariant_probe
//! target/release/examples/j_invariant_probe > replay/j_invariant_probe.json
//! ```

use crypto_lib::cryptanalysis::gaudry_quartic::Curve4;
use crypto_lib::cryptanalysis::quartic_folds::generate_gls_klein_instance;

fn main() {
    let sizes = [101u64, 149, 197, 269, 397, 557, 773, 1093, 1549, 2213, 3089];
    let mut list: Vec<(u64, u64)> = Vec::new();
    for p in sizes {
        for s in 1..=3u64 {
            list.push((p, s));
        }
    }
    list.push((4337, 1));
    println!("[");
    for (k, (p, s)) in list.iter().enumerate() {
        let ki = generate_gls_klein_instance(*p, *s, 16).expect("E25's curve");
        let inst = &ki.inst;
        let c = Curve4::with_params(*p, inst.a.clone(), inst.b.clone());
        let f = &c.f;
        let a3x4 = f.scale(&f.mul(&f.sq(&inst.a), &inst.a), 4);
        let den = f.add(&a3x4, &f.scale(&f.sq(&inst.b), 27));
        let j = f.mul(&f.scale(&a3x4, 1728), &f.inv(&den));
        let jp = f.pow(&j, *p as u128);
        let jp2 = f.pow(&j, (*p as u128) * (*p as u128));
        let in_fp = jp == j;
        let in_fp2 = jp2 == j;
        let is0 = j.is_zero();
        let is1728 = j == f.from_fp(1728 % *p);
        println!(
            "  {{\"p\": {p}, \"seed\": {s}, \"n\": {}, \"group_order\": {}, \"cofactor\": {}, \"planted_d\": {}, \"j\": [{}, {}, {}, {}], \"j_in_fp\": {in_fp}, \"j_in_fp2\": {in_fp2}, \"j_is_0\": {is0}, \"j_is_1728\": {is1728}}}{}",
            inst.n,
            inst.group_order,
            inst.cofactor,
            inst.d,
            j.0[0],
            j.0[1],
            j.0[2],
            j.0[3],
            if k + 1 < list.len() { "," } else { "" }
        );
    }
    println!("]");
}
