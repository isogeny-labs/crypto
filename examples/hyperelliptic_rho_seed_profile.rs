//! Diagnostic, not a registered measurement: per-seed walk cost and
//! restarts of the plain DP rho on the thread's largest instances, to see
//! whether round nine's and ten's large-`N` mean is driven by every seed
//! or by a few.  Counted operations only, deterministic in the seed.
//!
//! ```sh
//! cargo run --release --example hyperelliptic_rho_seed_profile
//! ```

use num_bigint::BigUint;

use crypto_lib::cryptanalysis::hyperelliptic_ic_bench::{
    pollard_rho_jacobian_with, thread_instance, RhoVariant,
};

fn main() {
    for (genus, p) in [(3u32, 211u64), (4, 61), (3, 151), (2, 251)] {
        let Some((curve, d1, n, _c)) = thread_instance(p, genus) else {
            continue;
        };
        let k = &n / BigUint::from(3u32) + BigUint::from(7u32);
        let d2 = d1.scalar_mul(&k, &curve);
        let root: f64 = n.to_string().parse::<f64>().expect("N fits f64").sqrt();
        let mut s = Vec::new();
        println!("genus {genus} p {p} N {n}: seed, walk/sqrt(N), restarts");
        for t in 0..40u64 {
            let r = pollard_rho_jacobian_with(
                &curve,
                &d1,
                &d2,
                &n,
                0xC0FFEE + t,
                200_000_000,
                &RhoVariant::DistinguishedPoints,
            );
            assert_eq!(r.k.as_ref(), Some(&k));
            let w = r.walk_ops as f64 / root;
            println!("  {t:>2} {w:>7.3} {:>4}", r.restarts);
            s.push(w);
        }
        s.sort_by(|a, b| a.partial_cmp(b).expect("finite"));
        let mean = s.iter().sum::<f64>() / s.len() as f64;
        println!(
            "  mean {mean:.3} median {:.3} min {:.3} max {:.3}",
            (s[19] + s[20]) / 2.0,
            s[0],
            s[39]
        );
    }
}
