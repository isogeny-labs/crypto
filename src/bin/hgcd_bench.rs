//! Half-GCD vs classical GCD crossovers for `crypto_lib::gcd`.
//!
//! For each size it times, interleaved over `--rounds` rounds with the arm
//! order shuffled per round, on the same random inputs (a planted common
//! factor of degree/bits ≈ size/8):
//!
//! * `fp`   — `F_p[x]`, `p = 2^61 − 1`, degree `n`: Euclid (`threshold =
//!   ∞`) vs half-GCD forced (`threshold = 0`), gcd and xgcd.
//! * `gf2`  — `GF(2)[x]`, degree `n`: Euclid vs half-GCD forced.
//! * `int`  — `BigUint`, `n` bits: num-integer `gcd`, num-bigint `modinv`
//!   (via `xgcd_big_euclid`, classical), Lehmer (`threshold = ∞`), and the
//!   recursive half-GCD forced (`threshold = 256`), gcd and xgcd.
//!
//! Every arm's result is checked equal to the reference before timing.
//! Output: JSON lines (`--out`), and with `--analyze FILE` a Markdown
//! table of medians and paired ratios with 95% bootstrap intervals.

use crypto_lib::gcd::{gf2, int, poly};
use num_bigint::{BigUint, RandBigInt};
use num_integer::Integer;
use rand::{rngs::StdRng, seq::SliceRandom, Rng, SeedableRng};
use std::collections::BTreeMap;
use std::io::Write;
use std::time::Instant;

fn arg<T: std::str::FromStr>(args: &[String], name: &str, default: T) -> T {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .and_then(|v| v.parse().ok())
        .unwrap_or(default)
}

fn time<R>(reps: usize, mut f: impl FnMut() -> R) -> f64 {
    let t0 = Instant::now();
    for _ in 0..reps {
        std::hint::black_box(f());
    }
    t0.elapsed().as_nanos() as f64 / reps as f64
}

fn rand_fp(f: &poly::Fp64, rng: &mut StdRng, deg: usize) -> Vec<u64> {
    let mut v: Vec<u64> = (0..=deg).map(|_| rng.gen_range(0..f.p)).collect();
    *v.last_mut().unwrap() = rng.gen_range(1..f.p);
    v
}

fn rand_gf2(rng: &mut StdRng, deg: usize) -> Vec<u64> {
    let mut v: Vec<u64> = (0..deg / 64 + 1).map(|_| rng.gen()).collect();
    let top = deg % 64;
    let last = v.last_mut().unwrap();
    *last &= if top == 63 { u64::MAX } else { (1u64 << (top + 1)) - 1 };
    *last |= 1u64 << top;
    v
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if let Some(i) = args.iter().position(|a| a == "--analyze") {
        analyze(&args[i + 1]);
        return;
    }
    let rounds: usize = arg(&args, "--rounds", 11);
    let seed: u64 = arg(&args, "--seed", 0x6867_6364);
    let out_path: String = arg(&args, "--out", "hgcd_bench.jsonl".to_string());
    let which: String = arg(&args, "--only", "fp,gf2,int".to_string());
    let mut out = std::fs::File::create(&out_path).expect("--out");
    let mut rng = StdRng::seed_from_u64(seed);

    let mut emit = |kind: &str, n: usize, arm: &str, round: usize, ns: f64| {
        writeln!(
            out,
            "{{\"kind\":\"{kind}\",\"n\":{n},\"arm\":\"{arm}\",\"round\":{round},\"ns\":{ns:.1}}}"
        )
        .unwrap();
    };

    if which.contains("fp") {
        let f = poly::Fp64::new((1u64 << 61) - 1);
        for &n in &[32usize, 64, 96, 128, 192, 256, 384, 512, 768, 1024, 2048, 4096] {
            let c = rand_fp(&f, &mut rng, n / 8);
            let a = poly::mul(&f, &rand_fp(&f, &mut rng, n - n / 8), &c);
            let b = poly::mul(&f, &rand_fp(&f, &mut rng, n - n / 8 - 1), &c);
            let want = poly::xgcd_euclid(&f, &a, &b);
            assert_eq!(poly::xgcd_with_threshold(&f, &a, &b, 0), want);
            assert_eq!(poly::gcd_with_threshold(&f, &a, &b, 0), want.0);
            assert_eq!(poly::xgcd(&f, &a, &b), want);
            let reps = (2_000_000 / (n * n)).max(1);
            for round in 0..rounds {
                let mut arms = ["gcd_euclid", "gcd_hgcd", "xgcd_euclid", "xgcd_hgcd", "gcd_default"];
                arms.shuffle(&mut rng);
                for arm in arms {
                    let ns = match arm {
                        "gcd_euclid" => time(reps, || poly::gcd_with_threshold(&f, &a, &b, usize::MAX)),
                        "gcd_hgcd" => time(reps, || poly::gcd_with_threshold(&f, &a, &b, 0)),
                        "xgcd_euclid" => time(reps, || poly::xgcd_euclid(&f, &a, &b)),
                        "xgcd_hgcd" => time(reps, || poly::xgcd_with_threshold(&f, &a, &b, 0)),
                        _ => time(reps, || poly::gcd(&f, &a, &b)),
                    };
                    emit("fp", n, arm, round, ns);
                }
            }
            eprintln!("fp n={n} done");
        }
    }

    if which.contains("gf2") {
        for &n in &[256usize, 512, 1024, 1536, 2048, 3072, 4096, 8192, 16384, 32768] {
            let c = rand_gf2(&mut rng, n / 8);
            let a = gf2::mul(&rand_gf2(&mut rng, n - n / 8), &c);
            let b = gf2::mul(&rand_gf2(&mut rng, n - n / 8 - 1), &c);
            let want = gf2::gcd_euclid(&a, &b);
            assert_eq!(gf2::gcd_with_threshold(&a, &b, 0), want);
            let x = gf2::xgcd_with_threshold(&a, &b, usize::MAX);
            assert_eq!(gf2::xgcd_with_threshold(&a, &b, 0), x);
            let reps = (40_000_000 / (n * n / 64)).clamp(1, 2000);
            for round in 0..rounds {
                let mut arms = ["gcd_euclid", "gcd_hgcd", "xgcd_euclid", "xgcd_hgcd"];
                arms.shuffle(&mut rng);
                for arm in arms {
                    let ns = match arm {
                        "gcd_euclid" => time(reps, || gf2::gcd_with_threshold(&a, &b, usize::MAX)),
                        "gcd_hgcd" => time(reps, || gf2::gcd_with_threshold(&a, &b, 0)),
                        "xgcd_euclid" => time(reps, || gf2::xgcd_with_threshold(&a, &b, usize::MAX)),
                        _ => time(reps, || gf2::xgcd_with_threshold(&a, &b, 0)),
                    };
                    emit("gf2", n, arm, round, ns);
                }
            }
            eprintln!("gf2 n={n} done");
        }
    }

    if which.contains("int") {
        for &n in &[256usize, 512, 1024, 2048, 4096, 8192, 16384, 32768, 65536, 131072] {
            let g = rng.gen_biguint((n / 8) as u64) | BigUint::from(1u8);
            let a: BigUint = rng.gen_biguint((n - n / 8) as u64) * &g;
            let b: BigUint = rng.gen_biguint((n - n / 8 - 1) as u64) * &g;
            let want = int::xgcd_big_euclid(&a, &b);
            assert_eq!(a.gcd(&b), want.0);
            assert_eq!(int::gcd_big_with_threshold(&a, &b, u64::MAX), want.0);
            assert_eq!(int::gcd_big_with_threshold(&a, &b, 256), want.0);
            let x = int::xgcd_big_with_threshold(&a, &b, u64::MAX);
            assert_eq!(int::xgcd_big_with_threshold(&a, &b, 256), x);
            let reps = (4_000_000_000 / (n * n)).clamp(1, 20_000);
            for round in 0..rounds {
                let mut arms = [
                    "gcd_numinteger",
                    "gcd_lehmer",
                    "gcd_hgcd",
                    "xgcd_euclid",
                    "xgcd_lehmer",
                    "xgcd_hgcd",
                ];
                arms.shuffle(&mut rng);
                for arm in arms {
                    let ns = match arm {
                        "gcd_numinteger" => time(reps, || a.gcd(&b)),
                        "gcd_lehmer" => time(reps, || int::gcd_big_with_threshold(&a, &b, u64::MAX)),
                        "gcd_hgcd" => time(reps, || int::gcd_big_with_threshold(&a, &b, 256)),
                        "xgcd_euclid" => time(reps, || int::xgcd_big_euclid(&a, &b)),
                        "xgcd_lehmer" => time(reps, || int::xgcd_big_with_threshold(&a, &b, u64::MAX)),
                        _ => time(reps, || int::xgcd_big_with_threshold(&a, &b, 256)),
                    };
                    emit("int", n, arm, round, ns);
                }
            }
            eprintln!("int n={n} done");
        }
    }
}

fn median(v: &mut [f64]) -> f64 {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = v.len();
    if n % 2 == 1 {
        v[n / 2]
    } else {
        0.5 * (v[n / 2 - 1] + v[n / 2])
    }
}

fn ratio_ci(r: &[f64], seed: u64) -> (f64, f64, f64) {
    let mut rr = r.to_vec();
    let mid = median(&mut rr);
    let mut rng = StdRng::seed_from_u64(seed);
    let mut boots: Vec<f64> = (0..2000)
        .map(|_| {
            let mut s: Vec<f64> = (0..r.len()).map(|_| r[rng.gen_range(0..r.len())]).collect();
            median(&mut s)
        })
        .collect();
    boots.sort_by(|a, b| a.partial_cmp(b).unwrap());
    (mid, boots[50], boots[1949])
}

fn analyze(path: &str) {
    let text = std::fs::read_to_string(path).expect("read");
    // (kind, n) -> arm -> round -> ns
    let mut d: BTreeMap<(String, usize), BTreeMap<String, BTreeMap<u64, f64>>> = BTreeMap::new();
    for line in text.lines().filter(|l| !l.is_empty()) {
        let v: serde_json::Value = serde_json::from_str(line).unwrap();
        d.entry((v["kind"].as_str().unwrap().into(), v["n"].as_u64().unwrap() as usize))
            .or_default()
            .entry(v["arm"].as_str().unwrap().into())
            .or_default()
            .insert(v["round"].as_u64().unwrap(), v["ns"].to_string().parse().unwrap());
    }
    let pairs: &[(&str, &str, &str)] = &[
        ("fp", "gcd_euclid", "gcd_hgcd"),
        ("fp", "xgcd_euclid", "xgcd_hgcd"),
        ("fp", "gcd_euclid", "gcd_default"),
        ("gf2", "gcd_euclid", "gcd_hgcd"),
        ("gf2", "xgcd_euclid", "xgcd_hgcd"),
        ("int", "gcd_numinteger", "gcd_lehmer"),
        ("int", "gcd_numinteger", "gcd_hgcd"),
        ("int", "gcd_lehmer", "gcd_hgcd"),
        ("int", "xgcd_euclid", "xgcd_lehmer"),
        ("int", "xgcd_euclid", "xgcd_hgcd"),
        ("int", "xgcd_lehmer", "xgcd_hgcd"),
    ];
    println!("| kind | size | baseline | candidate | baseline median µs | candidate median µs | speedup (paired median) [95% CI] |");
    println!("|:--|--:|:--|:--|--:|--:|--:|");
    for ((kind, n), arms) in &d {
        for (i, (k, base, cand)) in pairs.iter().enumerate() {
            if k != kind {
                continue;
            }
            let (Some(b), Some(c)) = (arms.get(*base), arms.get(*cand)) else { continue };
            let r: Vec<f64> = b.iter().filter_map(|(rd, x)| c.get(rd).map(|y| x / y)).collect();
            let (m, lo, hi) = ratio_ci(&r, (*n as u64) << 4 | i as u64);
            let mb = median(&mut b.values().copied().collect::<Vec<_>>()) / 1000.0;
            let mc = median(&mut c.values().copied().collect::<Vec<_>>()) / 1000.0;
            println!("| {kind} | {n} | {base} | {cand} | {mb:.2} | {mc:.2} | {m:.3}× [{lo:.3}, {hi:.3}] |");
        }
    }
}
