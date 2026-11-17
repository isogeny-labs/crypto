//! Decomposability check for the Kummer `m = 2` draws of
//! `research/prime_fall_degree_20261027` (PREREGISTRATION.md, Amendment 7 C).
//!
//! A draw is **decomposable** when the reduced `S₃(x₁, x₂, x_R)` has a zero on
//! the grid `V × V`, `V = {g·ζ^j}`: then the decomposition system has a
//! solution, F4 cannot refute it, and the draw is excluded from every
//! solving-degree reading. `prime_fall_degree_tools ns-degree` computes
//! `grid_zeros` as a by-product of the Nullstellensatz degree; it holds four
//! `K²`-entry arrays (about 2 GB each at `t = 14`). This tool counts the same
//! zeros in O(K) memory, so it fits a 15 GB host at `t = 14`.
//!
//! Input: the JSON-lines rows of `pkm_tower_pilot` (any engine) on Kummer
//! `m = 2` `tower` systems; each row carries `p`, `t`, `target_index`, `x_r`,
//! `curve.{a,b}` and `tower.{g,zeta}`. One output line per distinct
//! `(p, t, target_index)`:
//!
//! ```text
//! {"p":…,"t":…,"target_index":…,"x_r":…,"grid_zeros":…,"decomposable":…}
//! ```
//!
//! `cargo run --release --example pfd_grid_zeros -- ROWS.jsonl [ROWS2.jsonl …]`
//!
//! The `s3` below is a copy of the one in `prime_fall_degree_tools.rs`; the
//! protocol (`research/prime_fall_degree_t14_20261101/PLAN.md`) cross-checks
//! the two on rows both can read.
use rayon::prelude::*;
use serde_json::{json, Value};
use std::collections::BTreeSet;
use std::fs;

fn mulm(a: u64, b: u64, p: u64) -> u64 {
    ((a as u128 * b as u128) % p as u128) as u64
}

fn powm(mut a: u64, mut e: u64, p: u64) -> u64 {
    let mut r = 1;
    a %= p;
    while e > 0 {
        if e & 1 == 1 {
            r = mulm(r, a, p);
        }
        a = mulm(a, a, p);
        e >>= 1;
    }
    r
}

fn s3(x1: u64, x2: u64, x3: u64, a: u64, b: u64, p: u64) -> u64 {
    let d = (x1 + p - x2) % p;
    let t1 = mulm(mulm(mulm(d, d, p), x3, p), x3, p);
    let s = (x1 + x2) % p;
    let pr = mulm(x1, x2, p);
    let inner = (mulm(s, (pr + a) % p, p) + mulm(2, b, p)) % p;
    let t2 = mulm(p - 2, mulm(inner, x3, p), p);
    let q = (pr + p - a) % p;
    let t3 = (mulm(q, q, p) + p - mulm(mulm(4, b, p), s, p)) % p;
    (t1 + t2 + t3) % p
}

fn u(v: &Value, k: &str) -> u64 {
    match &v[k] {
        Value::Number(n) => n
            .to_string()
            .parse()
            .unwrap_or_else(|_| panic!("field {k}")),
        other => panic!("field {k} is not a number: {other}"),
    }
}

fn main() {
    let files: Vec<String> = std::env::args().skip(1).collect();
    if files.is_empty() {
        eprintln!("usage: pfd_grid_zeros ROWS.jsonl [ROWS.jsonl …]");
        std::process::exit(2);
    }
    let mut seen = BTreeSet::new();
    for f in files {
        let text = fs::read_to_string(&f).unwrap_or_else(|e| panic!("{f}: {e}"));
        for line in text.lines().filter(|l| !l.trim().is_empty()) {
            let r: Value = serde_json::from_str(line).expect("row is JSON");
            if r["kind"] != "kummer" || u(&r, "m") != 2 || r["control"] != "tower" {
                continue;
            }
            let (p, t, k) = (u(&r, "p"), u(&r, "t"), u(&r, "target_index"));
            if !seen.insert((p, t, k)) {
                continue;
            }
            let kk = 1usize << t;
            let (a, b) = (u(&r["curve"], "a"), u(&r["curve"], "b"));
            let (g, zeta) = (u(&r["tower"], "g"), u(&r["tower"], "zeta"));
            let xr = u(&r, "x_r");
            let xs: Vec<u64> = (0..kk)
                .map(|j| mulm(g, powm(zeta, j as u64, p), p))
                .collect();
            let zeros: usize = (0..kk)
                .into_par_iter()
                .map(|j1| {
                    (0..kk)
                        .filter(|&j2| s3(xs[j1], xs[j2], xr, a, b, p) == 0)
                        .count()
                })
                .sum();
            println!(
                "{}",
                json!({"p": p, "t": t, "target_index": k, "x_r": xr,
                       "grid_zeros": zeros, "decomposable": zeros > 0})
            );
        }
    }
}
