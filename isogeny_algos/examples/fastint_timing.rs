//! Wall-clock timing of the concrete integer multipliers used by `subnlogn`:
//! schoolbook `Big::mul` and the Fermat-ring `fastint::multiply`. Development
//! diagnostic only (no benchmark record); every product is checked for equality.
//!
//! cargo run --release --features experimental-subnlogn --example fastint_timing [max_log2_bits] [fine] [square]
//!
//! `fine` steps operand sizes by 2^(1/4) instead of 2; `square` times a*a;
//! `threads=N` caps fastint worker threads (default: all cores).
use isogeny_algos::bigint::Big;
use isogeny_algos::subnlogn::fastint;
use std::time::Instant;

fn fixture(limbs: usize, mut s: u64) -> Big {
    let mut v = Vec::with_capacity(limbs);
    for _ in 0..limbs {
        s ^= s << 13;
        s ^= s >> 7;
        s ^= s << 17;
        v.push(s);
    }
    Big::from_limbs(&v)
}

fn time<F: FnMut() -> Big>(mut f: F) -> (f64, Big) {
    let mut out = f();
    let mut best = f64::INFINITY;
    let mut total = 0.0;
    let mut reps = 0;
    while reps < 3 || (total < 0.3 && reps < 50) {
        let t = Instant::now();
        out = f();
        let dt = t.elapsed().as_secs_f64();
        best = best.min(dt);
        total += dt;
        reps += 1;
    }
    (best, out)
}

fn main() {
    let max_log: u32 = std::env::args()
        .nth(1)
        .map(|s| s.parse().unwrap())
        .unwrap_or(20);
    println!(
        "{:>10} {:>14} {:>14} {:>8}",
        "bits/op", "schoolbook_s", "fastint_s", "ratio"
    );
    let args: Vec<String> = std::env::args().collect();
    let fine = args.iter().any(|a| a == "fine");
    let square = args.iter().any(|a| a == "square");
    if let Some(t) = args.iter().find_map(|a| a.strip_prefix("threads=")) {
        fastint::set_threads(t.parse().expect("threads=N"));
    }
    let sizes: Vec<(u32, usize)> = if fine {
        (40..=4 * max_log)
            .map(|q| {
                (
                    q,
                    ((2f64.powf(q as f64 / 4.0) / 64.0).round() as usize) * 64,
                )
            })
            .collect()
    } else {
        (10..=max_log).map(|l| (l, 1usize << l)).collect()
    };
    for (log, bits) in sizes {
        let a = fixture(bits / 64, 0x9e3779b97f4a7c15 ^ log as u64);
        let b = if square {
            a.clone()
        } else {
            fixture(bits / 64, 0xd1b54a32d192ed03 ^ log as u64)
        };
        let (tf, pf) = time(|| fastint::multiply(&a, &b));
        let school = if bits <= 1 << 18 {
            let (ts, ps) = time(|| a.mul(&b));
            assert_eq!(ps, pf, "product mismatch at {bits} bits");
            Some(ts)
        } else {
            None
        };
        match school {
            Some(ts) => println!("{bits:>10} {ts:>14.6e} {tf:>14.6e} {:>8.2}", ts / tf),
            None => println!("{bits:>10} {:>14} {tf:>14.6e} {:>8}", "-", "-"),
        }
    }
}
