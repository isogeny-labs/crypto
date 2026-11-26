//! Useful rank per second of the library's Semaev four-summand oracle
//! (`semaev_projection::PairProjection`) on the frozen B677 M31 workload.
//!
//! The oracle is called through its public API exactly as merged (PR #4846);
//! nothing in the library is modified. Each query is a fresh public
//! R = aG + bQ drawn from the same seed sequence as the B677 graph-control
//! study (crypto PR #5097 / #5184). A returned x-only witness is lifted to a
//! signed relation by the group law, deduplicated, and fed to the same
//! large-prime-then-factor-base elimination. The run stops when the target
//! logarithm is recovered (and checked by d*G = Q) or at the wall cap.
//!
//! Arms (first argument):
//!   generic  build_generic + decompose_four_generic   (per-u invm + sqrtm, full scan)
//!   fast     build_fast    + decompose_four_fast_path (batched, full scan, first witness kept)
//!   first    build_fast    + decompose_four_first     (batched, early exit)
//!   default  build         + decompose_four           (the public defaults)
//!
//! Usage: semaev_fast_path_rank_rate <arm> [cap_seconds=120] [max_queries=100000]

use crypto_lib::cryptanalysis::pkm_smooth_tower::{Curve, Pt};
use crypto_lib::cryptanalysis::semaev_projection::{FourSumWitness, PairProjection, QueryStats};
use serde_json::json;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::time::Instant;

const POINTS_CSV: &str = include_str!("../research/semaev_fast_path_rank_20261125/b677_points.csv");
const P: u64 = 2_147_483_647;
const N: u64 = 2_147_446_831;
const A: u64 = 847_514_846;
const B: u64 = 2_087_444_724;
const G: (u64, u64) = (625_037_715, 2_100_627_251);
const Q: (u64, u64) = (976_979_610, 1_228_979_761);
const FB: usize = 509;
const LP: usize = 168;
const TARGET: usize = FB + LP;
const RHS: usize = TARGET + 1;
const WIDTH: usize = RHS + 1;

fn mod_mul(x: u64, y: u64) -> u64 {
    ((x as u128 * y as u128) % N as u128) as u64
}
fn mod_sub(x: u64, y: u64) -> u64 {
    (x + N - y) % N
}
fn mod_inverse(mut x: u64) -> u64 {
    let (mut r, mut e) = (1, N - 2);
    while e != 0 {
        if e & 1 != 0 {
            r = mod_mul(r, x);
        }
        x = mod_mul(x, x);
        e >>= 1;
    }
    r
}

/// Same elimination as the B677 graph control (`mitm_four.rs`).
struct GraphRank {
    lp: Vec<Option<Vec<u64>>>,
    fb: Vec<Option<Vec<u64>>>,
    lp_rank: usize,
    fb_rank: usize,
}
impl GraphRank {
    fn new() -> Self {
        Self {
            lp: vec![None; LP],
            fb: vec![None; FB + 1],
            lp_rank: 0,
            fb_rank: 0,
        }
    }
    fn elim(row: &mut [u64], piv: &[u64], c: u64) {
        for k in 0..WIDTH {
            if piv[k] != 0 {
                row[k] = mod_sub(row[k], mod_mul(c, piv[k]));
            }
        }
    }
    fn norm(row: &mut [u64], c: u64) {
        let inv = mod_inverse(c);
        for v in row.iter_mut() {
            if *v != 0 {
                *v = mod_mul(*v, inv);
            }
        }
    }
    /// Returns true when the row raised the rank.
    fn add(&mut self, mut row: Vec<u64>) -> bool {
        for at in 0..LP {
            let c = row[FB + at];
            if c == 0 {
                continue;
            }
            if let Some(p) = &self.lp[at] {
                Self::elim(&mut row, p, c);
            } else {
                Self::norm(&mut row, c);
                self.lp[at] = Some(row);
                self.lp_rank += 1;
                return true;
            }
        }
        for col in 0..=TARGET {
            if (FB..TARGET).contains(&col) {
                continue;
            }
            let c = row[col];
            if c == 0 {
                continue;
            }
            let at = if col == TARGET { FB } else { col };
            if let Some(p) = &self.fb[at] {
                Self::elim(&mut row, p, c);
            } else {
                Self::norm(&mut row, c);
                self.fb[at] = Some(row);
                self.fb_rank += 1;
                return true;
            }
        }
        assert_eq!(row[RHS], 0, "replayed relations cannot contradict");
        false
    }
    fn target(&self) -> Option<u64> {
        self.fb[FB].as_ref().map(|r| r[RHS])
    }
}

fn next_word(s: &mut u64) -> u64 {
    *s = s.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let mut z = *s;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}

fn main() {
    let mut args = std::env::args().skip(1);
    let arm = args.next().unwrap_or_else(|| "fast".into());
    let cap_s: f64 = args.next().and_then(|v| v.parse().ok()).unwrap_or(120.0);
    let max_q: usize = args.next().and_then(|v| v.parse().ok()).unwrap_or(100_000);
    assert!(
        ["generic", "fast", "first", "default"].contains(&arm.as_str()),
        "unknown arm"
    );

    let cold = Instant::now();
    let curve = Curve { p: P, a: A, b: B };
    // Frozen B677 support, exported from the b677_fixture constructor.
    let mut pts = Vec::new();
    let mut is_fb = Vec::new();
    for line in POINTS_CSV.lines() {
        let f: Vec<&str> = line.split(',').collect();
        let (x, y): (u64, u64) = (f[0].parse().unwrap(), f[1].parse().unwrap());
        assert_eq!(
            (y as u128 * y as u128 % P as u128) as u64,
            curve.rhs(x),
            "point on curve"
        );
        pts.push((x, y));
        is_fb.push(f[2] == "fb");
    }
    assert_eq!(pts.len(), FB + LP);
    assert_eq!(is_fb.iter().filter(|&&b| b).count(), FB);
    let index: HashMap<u64, usize> = pts.iter().enumerate().map(|(i, p)| (p.0, i)).collect();
    assert_eq!(index.len(), pts.len());
    let mut column = vec![0usize; pts.len()];
    let (mut nf, mut nl) = (0, FB);
    for i in 0..pts.len() {
        if is_fb[i] {
            column[i] = nf;
            nf += 1;
        } else {
            column[i] = nl;
            nl += 1;
        }
    }
    let base_x: Vec<u64> = pts.iter().map(|p| p.0).collect();

    let build_start = Instant::now();
    let proj = match arm.as_str() {
        "generic" => PairProjection::build_generic(curve, &base_x),
        "fast" | "first" => PairProjection::build_fast(curve, &base_x),
        _ => PairProjection::build(curve, &base_x),
    }
    .expect("projection build");
    let build_ms = build_start.elapsed().as_secs_f64() * 1e3;
    let finite_support = proj.finite_support();

    let decompose = |t: u64| -> (Option<FourSumWitness>, QueryStats) {
        match arm.as_str() {
            "generic" => proj.decompose_four_generic(t),
            "fast" => proj.decompose_four_fast_path(t),
            "first" => proj.decompose_four_first(t),
            _ => proj.decompose_four(t),
        }
    };
    let (mut scanned, mut lookups, mut compatible) = (0u64, 0u64, 0u64);
    // Order-sensitive digest of the accepted relation stream (FNV-style, std only).
    let mut stream = std::collections::hash_map::DefaultHasher::new();

    let g: Pt = Some(G);
    let q: Pt = Some(Q);
    let mut seed = 0x7033_315f_6d34_0101u64;
    let mut rank = GraphRank::new();
    let mut seen: HashSet<Vec<(usize, i64)>> = HashSet::new();
    let (mut queries, mut witnesses, mut empty, mut unliftable, mut dup, mut skipped_lp_only) =
        (0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let (mut useful, mut multi_sign) = (0u64, 0u64);
    let mut query_ms = 0.0;
    let mut trajectory = Vec::new();
    let mut recovered = None;
    while queries < max_q as u64 && cold.elapsed().as_secs_f64() < cap_s {
        let a = next_word(&mut seed) % N;
        let b = next_word(&mut seed) % N;
        let r = curve
            .add(curve.mul(a, g), curve.mul(b, q))
            .expect("finite query");
        queries += 1;
        let t0 = Instant::now();
        let (w, qs) = decompose(r.0);
        query_ms += t0.elapsed().as_secs_f64() * 1e3;
        scanned += qs.quadratics;
        lookups += qs.lookups;
        compatible += qs.compatible_keys;
        let Some(w) = w else {
            empty += 1;
            continue;
        };
        witnesses += 1;
        // Lift: all sign patterns whose group-law sum is exactly R.
        let idx: Vec<usize> =
            w.x.iter()
                .map(|x| *index.get(x).expect("witness x in base"))
                .collect();
        let mut lifts: Vec<Vec<(usize, i64)>> = Vec::new();
        for mask in 0..16u32 {
            let mut sum: Pt = None;
            for (k, &i) in idx.iter().enumerate() {
                let pt = Some(pts[i]);
                let pt = if mask >> k & 1 == 1 {
                    curve.neg(pt)
                } else {
                    pt
                };
                sum = curve.add(sum, pt);
            }
            if sum == Some(r) {
                let mut coef: HashMap<usize, i64> = HashMap::new();
                for (k, &i) in idx.iter().enumerate() {
                    *coef.entry(i).or_default() += if mask >> k & 1 == 1 { -1 } else { 1 };
                }
                let mut terms: Vec<(usize, i64)> =
                    coef.into_iter().filter(|&(_, c)| c != 0).collect();
                terms.sort_unstable();
                if !lifts.contains(&terms) {
                    lifts.push(terms);
                }
            }
        }
        if lifts.is_empty() {
            unliftable += 1;
            continue;
        }
        if lifts.len() > 1 {
            multi_sign += 1;
        }
        for terms in lifts {
            // Independent replay of the lifted relation.
            let mut sum: Pt = None;
            for &(i, c) in &terms {
                let pt = Some(pts[i]);
                for _ in 0..c.unsigned_abs() {
                    sum = curve.add(sum, if c > 0 { pt } else { curve.neg(pt) });
                }
            }
            assert_eq!(sum, Some(r), "lifted relation must replay");
            let base_terms = terms.iter().filter(|&&(i, _)| is_fb[i]).count();
            (queries, &terms).hash(&mut stream);
            if !seen.insert(terms.clone()) {
                dup += 1;
                continue;
            }
            if base_terms < 2 {
                // Same admission rule as the B677 graph control.
                skipped_lp_only += 1;
                continue;
            }
            let mut row = vec![0u64; WIDTH];
            for &(i, c) in &terms {
                row[column[i]] = if c >= 0 {
                    c as u64 % N
                } else {
                    N - ((-c) as u64 % N)
                };
            }
            row[TARGET] = mod_sub(0, b);
            row[RHS] = a;
            if rank.add(row) {
                useful += 1;
            }
        }
        if queries.is_power_of_two() || queries % 64 == 0 {
            trajectory.push(json!({
                "t_ms": cold.elapsed().as_secs_f64() * 1e3, "queries": queries,
                "relations": seen.len(), "useful_rows": useful,
                "lp_rank": rank.lp_rank, "fb_rank": rank.fb_rank,
            }));
        }
        if let Some(d) = rank.target() {
            assert_eq!(curve.mul(d, g), q, "recovered logarithm must verify");
            recovered = Some(d);
            break;
        }
    }
    let cold_ms = cold.elapsed().as_secs_f64() * 1e3;
    trajectory.push(json!({
        "t_ms": cold_ms, "queries": queries, "relations": seen.len(), "useful_rows": useful,
        "lp_rank": rank.lp_rank, "fb_rank": rank.fb_rank,
    }));
    let total_rank = rank.lp_rank + rank.fb_rank;
    println!(
        "{}",
        serde_json::to_string_pretty(&json!({
            "scope": "library PairProjection four-summand oracle, frozen B677 support, useful rank per second",
            "arm": arm,
            "cap_seconds": cap_s,
            "base_size": FB + LP,
            "finite_support": finite_support,
            "build_ms": build_ms,
            "queries": queries,
            "witnesses": witnesses,
            "empty_queries": empty,
            "unliftable_witnesses": unliftable,
            "multi_sign_witnesses": multi_sign,
            "duplicate_relations": dup,
            "lp_only_relations_skipped": skipped_lp_only,
            "distinct_relations": seen.len(),
            "relation_stream_digest": format!("{:016x}", stream.finish()),
            "entries_scanned": scanned,
            "root_lookups": lookups,
            "compatible_keys": compatible,
            "entries_scanned_per_useful_rank": scanned as f64 / total_rank.max(1) as f64,
            "useful_rows": useful,
            "lp_rank": rank.lp_rank,
            "fb_rank": rank.fb_rank,
            "total_rank": total_rank,
            "recovered_scalar": recovered,
            "query_ms_total": query_ms,
            "query_ms_mean": query_ms / queries.max(1) as f64,
            "cold_ms": cold_ms,
            "useful_rank_per_second": total_rank as f64 / (cold_ms / 1e3),
            "useful_rank_per_query": total_rank as f64 / queries.max(1) as f64,
            "trajectory": trajectory,
        }))
        .unwrap()
    );
}
