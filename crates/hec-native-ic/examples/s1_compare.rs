//! S1 table: native engine vs the BigUint engine (round eight's example) on
//! the θ = 1 rows, from the two raw outputs.
//!
//! ```sh
//! cargo run --release --example s1_compare -- biguint_round8.txt native_s1.txt
//! ```
//!
//! BigUint `wall_ic_ms` is its mean over 5 seeds (20260916 + t); the native
//! column is the mean of `total_ms` over the same seed numbers.  Smoothness
//! is reported per engine (different walk streams).

use std::collections::BTreeMap;

fn field(line: &str, key: &str) -> Option<f64> {
    let pat = format!("\"{key}\":");
    let i = line.find(&pat)? + pat.len();
    let rest = &line[i..];
    let end = rest.find([',', '}']).unwrap_or(rest.len());
    rest[..end].trim().trim_matches('"').parse().ok()
}

fn flag(line: &str, key: &str) -> Option<bool> {
    let pat = format!("\"{key}\":");
    let i = line.find(&pat)? + pat.len();
    Some(line[i..].starts_with("true"))
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let (big, nat) = (
        std::fs::read_to_string(&args[1]).expect("biguint output"),
        std::fs::read_to_string(&args[2]).expect("native output"),
    );
    // (g, p) -> (wall_ms, smoothness, ok)
    let mut b: BTreeMap<(u64, u64), (f64, f64, bool, f64)> = BTreeMap::new();
    for l in big.lines().filter(|l| l.starts_with("JSON")) {
        if field(l, "theta") != Some(1.0) {
            continue;
        }
        let key = (field(l, "genus").unwrap() as u64, field(l, "p").unwrap() as u64);
        b.insert(
            key,
            (
                field(l, "wall_ic_ms").unwrap(),
                field(l, "smoothness").unwrap(),
                flag(l, "ic_correct").unwrap(),
                field(l, "N").unwrap(),
            ),
        );
    }
    // (g, p) -> Vec<(total_ms, smooth, solve_ms, ok, ops)>
    let mut n: BTreeMap<(u64, u64), Vec<(f64, f64, f64, bool, f64, f64)>> = BTreeMap::new();
    for l in nat.lines().filter(|l| l.starts_with("JSON")) {
        let key = (field(l, "genus").unwrap() as u64, field(l, "p").unwrap() as u64);
        n.entry(key).or_default().push((
            field(l, "total_ms").unwrap(),
            field(l, "smoothness").unwrap(),
            field(l, "solve_ms").unwrap(),
            flag(l, "correct").unwrap(),
            field(l, "group_ops").unwrap(),
            field(l, "ell").unwrap(),
        ));
    }
    println!("| g | p | ℓ | BigUint ms (mean of 5) | native ms (mean of 5) | native min–max | **speedup** | native solve share | smoothness BigUint / native (range) | native ops (mean) | verified |");
    println!("|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|:--|");
    for (key, runs) in &n {
        let k = runs.len() as f64;
        let mean = runs.iter().map(|r| r.0).sum::<f64>() / k;
        let lo = runs.iter().map(|r| r.0).fold(f64::INFINITY, f64::min);
        let hi = runs.iter().map(|r| r.0).fold(0.0, f64::max);
        let solve = runs.iter().map(|r| r.2).sum::<f64>() / runs.iter().map(|r| r.0).sum::<f64>();
        let smin = runs.iter().map(|r| r.1).fold(f64::INFINITY, f64::min);
        let smax = runs.iter().map(|r| r.1).fold(0.0, f64::max);
        let ops = runs.iter().map(|r| r.4).sum::<f64>() / k;
        let all_ok = runs.iter().all(|r| r.3);
        let ell = runs[0].5;
        match b.get(key) {
            Some(&(bw, bs, bok, bn)) => {
                assert_eq!(bn, ell, "instance mismatch at {key:?}");
                println!(
                    "| {} | {} | {} | {:.2} | {:.3} | {:.3}–{:.3} | **{:.0}×** | {:.0}% | {:.3} / {:.3}–{:.3} | {:.0} | {} |",
                    key.0, key.1, ell, bw, mean, lo, hi, bw / mean, 100.0 * solve, bs, smin, smax, ops,
                    if all_ok && bok { format!("{}/{} + BigUint", runs.len(), runs.len()) } else { "NO".into() }
                );
            }
            None => println!(
                "| {} | {} | {} | — | {:.3} | {:.3}–{:.3} | — | {:.0}% | — / {:.3}–{:.3} | {:.0} | {} |",
                key.0, key.1, ell, mean, lo, hi, 100.0 * solve, smin, smax, ops,
                if all_ok { format!("{}/{}", runs.len(), runs.len()) } else { "NO".into() }
            ),
        }
    }
}
