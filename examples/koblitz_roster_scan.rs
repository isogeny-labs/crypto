//! Every Koblitz `K_a` over `GF(2^n)`, `31 <= n <= 63`: the prime subgroup
//! the index calculus and rho both work in, its size and the cofactor.
//!
//! `cargo run --release --example koblitz_roster_scan`
use crypto_lib::cryptanalysis::koblitz_index_calculus::KoblitzCurve;
use num_traits::ToPrimitive;

fn main() {
    println!("n a log2_r cofactor slug");
    for n in std::env::var("NMIN")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(31u32)
        ..=std::env::var("NMAX")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(63u32)
    {
        for a in 0..=1u8 {
            let Some(kc) = KoblitzCurve::new(a, n) else {
                continue;
            };
            let r = kc.subgroup_order.to_f64().unwrap_or(f64::NAN);
            println!("{n} {a} {:.2} {} {}", r.log2(), kc.cofactor, kc.label());
        }
    }
}
