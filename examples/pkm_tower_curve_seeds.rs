//! List `find_prime_order_curve(bits, seed)` seeds whose prime `p` has
//! `2^t | p − 1`, the condition a Kummer tower of height `t`
//! (`ecbench`'s `prime-tower` base) needs, with how many of the tower's
//! `2^t` abscissae carry a point is left to the base itself.
//!
//! ```bash
//! cargo run --release --example pkm_tower_curve_seeds -- 13 7 3
//! ```
//! prints the first 3 seeds at 13 bits with `2^7 | p − 1`.

use crypto_lib::cryptanalysis::ic_boundary::find_prime_order_curve;

fn main() {
    let a: Vec<u64> = std::env::args()
        .skip(1)
        .map(|s| s.parse().expect("integer"))
        .collect();
    let (bits, t, want) = (a[0] as u32, a[1] as u32, a.get(2).copied().unwrap_or(1));
    let mut found = 0;
    for seed in 1u64.. {
        let inst = find_prime_order_curve(bits, seed);
        let p = inst.curve.p;
        if (p - 1).is_multiple_of(1u64 << t) {
            println!(
                "bits {bits} t {t} seed {seed} p {p} r {} slug {}",
                inst.r, inst.name
            );
            found += 1;
            if found == want {
                break;
            }
        }
    }
}
