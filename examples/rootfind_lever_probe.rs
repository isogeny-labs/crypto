//! Cheap falsifying tests for levers 2–4 of
//! `research/rootfind_baseline_100x_20261117/PLAN.md`, on top of the lever-1
//! kernel of `rootfind_floor_probe` (shared denominators, abscissa only,
//! Montgomery-resident, one inversion a target).
//!
//! * **Lever 2, targets side by side.**  The row kernel is two serial product
//!   chains (`acc *= d` forward, `inv *= d` backward); one target is latency
//!   bound.  `lanes=K` runs K targets in lockstep, so K independent chains
//!   overlap and the per-lane loops are written to vectorize (the Montgomery
//!   reduction avoids `u128`).  Prediction: arithmetic falls from ~7.7 ns a
//!   point towards 1–2 ns; falsified if four lanes stay above 4 ns.
//! * **Lever 3, the filter alone.**  Probes of precomputed abscissae against
//!   word-blocked Bloom filters of 16, 8 and 4 bits a key (2 MiB, 1 MiB and
//!   512 KiB at A = 1024) and against the exact `HashSet`.  Prediction: the
//!   L2-resident filter probes in ~2 ns; falsified if the 1 MiB filter is no
//!   faster than the 2 MiB one.
//! * **Lever 4, threads over targets.**  The full pipeline (best lanes, the
//!   chosen filter, exact check on positives) at 1, 2 and 4 threads.
//!   Falsified as a 3.5x lever if 4 threads give under 2.5x.
//!
//! Correctness is checked, not assumed: every lockstep abscissa of the first
//! `--check` targets equals the scalar kernel's, the scalar kernel is checked
//! against the library's group law, and every timed pipeline finds exactly
//! the reference number of table hits.  A prototype for planning;
//! `[pcc:unverified]`, no isolation, shared host.
//!
//!     rootfind_lever_probe [--bits 32] [--sizes 256,1024] [--targets 4096]

use std::collections::HashSet;
use std::hint::black_box;
use std::time::Instant;

use crypto_lib::cryptanalysis::ic_boundary::{prime_instance_for, CountedGroup, GroupOps, PrimePoint};
use crypto_lib::cryptanalysis::ic_framework::plugins::PrimeAbscissaBase;
use crypto_lib::cryptanalysis::ic_framework::stages::{FactorBaseBuilder, InstanceCtx, Params};
use serde_json::json;

/// Montgomery arithmetic for an odd modulus below 2^32, `R = 2^32`, written
/// without `u128` so per-lane loops can vectorize.
#[derive(Clone, Copy)]
struct Mont {
    p: u64,
    ninv: u64,
    r2: u64,
}

impl Mont {
    fn new(p: u64) -> Self {
        assert!(p < (1 << 32) && p & 1 == 1);
        let mut inv = 1u32;
        for _ in 0..5 {
            inv = inv.wrapping_mul(2u32.wrapping_sub((p as u32).wrapping_mul(inv)));
        }
        let r = (1u128 << 32) % p as u128;
        Self {
            p,
            ninv: u64::from(inv.wrapping_neg()),
            r2: ((r * r) % p as u128) as u64,
        }
    }
    /// `a b / 2^32 mod p` for `a, b < p`.  The low halves of `t` and `m p`
    /// sum to 0 mod 2^32, so their carry is 1 exactly when `t`'s low half is
    /// nonzero.
    #[inline(always)]
    fn mul(&self, a: u64, b: u64) -> u64 {
        let t = a * b;
        let m = (t & 0xFFFF_FFFF) * self.ninv & 0xFFFF_FFFF;
        let u = (t >> 32) + ((m * self.p) >> 32) + u64::from(t & 0xFFFF_FFFF != 0);
        if u >= self.p {
            u - self.p
        } else {
            u
        }
    }
    #[inline(always)]
    fn sub(&self, a: u64, b: u64) -> u64 {
        let s = a + self.p - b;
        if s >= self.p {
            s - self.p
        } else {
            s
        }
    }
    #[inline(always)]
    fn add(&self, a: u64, b: u64) -> u64 {
        let s = a + b;
        if s >= self.p {
            s - self.p
        } else {
            s
        }
    }
    fn to(&self, a: u64) -> u64 {
        self.mul(a % self.p, self.r2)
    }
    fn inv(&self, a: u64) -> u64 {
        let (mut base, mut e, mut acc) = (a, self.p - 2, self.to(1));
        while e > 0 {
            if e & 1 == 1 {
                acc = self.mul(acc, base);
            }
            base = self.mul(base, base);
            e >>= 1;
        }
        acc
    }
}

/// A word-blocked Bloom filter: one 64-bit word a key, `h` bits in it.
struct WordBloom {
    words: Vec<u64>,
    mask: usize,
    h: u32,
}

impl WordBloom {
    fn new(keys: usize, bits_per_key: usize, h: u32) -> Self {
        let words = (keys * bits_per_key / 64).next_power_of_two().max(64);
        Self {
            words: vec![0; words],
            mask: words - 1,
            h,
        }
    }
    #[inline(always)]
    fn slot(&self, key: u64) -> (usize, u64) {
        let h = key.wrapping_mul(0x9E37_79B9_7F4A_7C15);
        let h2 = (h ^ (h >> 29)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        let mut bits = 0u64;
        let mut g = h2;
        for _ in 0..self.h {
            bits |= 1u64 << (g & 63);
            g >>= 6;
        }
        ((h >> 34) as usize & self.mask, bits)
    }
    fn insert(&mut self, key: u64) {
        let (w, bits) = self.slot(key);
        self.words[w] |= bits;
    }
    #[inline(always)]
    fn maybe(&self, key: u64) -> bool {
        let (w, bits) = self.slot(key);
        self.words[w] & bits == bits
    }
    fn bytes(&self) -> usize {
        self.words.len() * 8
    }
}

/// The scalar lever-1 row: `xs = [x(R − P_0), x(R + P_0), …]`, sentinel
/// `u64::MAX` where `x_k = x_R`.
fn row_scalar(f: &Mont, bx: &[u64], by: &[u64], xr: u64, yr: u64, pre: &mut [u64], xs: &mut [u64]) {
    let a = bx.len();
    let one = f.to(1);
    let mut acc = one;
    for k in 0..a {
        let d = f.sub(xr, bx[k]);
        pre[k] = acc;
        acc = f.mul(acc, if d == 0 { one } else { d });
    }
    let mut inv = f.inv(acc);
    for k in (0..a).rev() {
        let d = f.sub(xr, bx[k]);
        if d == 0 {
            xs[2 * k] = u64::MAX;
            xs[2 * k + 1] = u64::MAX;
            continue;
        }
        let dinv = f.mul(inv, pre[k]);
        inv = f.mul(inv, d);
        let base = f.add(xr, bx[k]);
        let lm = f.mul(f.add(yr, by[k]), dinv);
        let lp = f.mul(f.sub(yr, by[k]), dinv);
        xs[2 * k] = f.sub(f.mul(lm, lm), base);
        xs[2 * k + 1] = f.sub(f.mul(lp, lp), base);
    }
}

/// `K` targets in lockstep.  `pre` is `a * K` (index `k * K + l`), `xs` is
/// `K` rows of `2a` (row `l` at `l * 2a`).  Lane loops are branch-free.
#[inline(always)]
fn row_lanes<const K: usize>(
    f: &Mont,
    bx: &[u64],
    by: &[u64],
    xr: &[u64; K],
    yr: &[u64; K],
    pre: &mut [u64],
    xs: &mut [u64],
) {
    let a = bx.len();
    let one = f.to(1);
    let mut acc = [one; K];
    for k in 0..a {
        let b = bx[k];
        let p = &mut pre[k * K..k * K + K];
        for l in 0..K {
            let d = f.sub(xr[l], b);
            p[l] = acc[l];
            acc[l] = f.mul(acc[l], if d == 0 { one } else { d });
        }
    }
    // One inversion for all K lanes (Montgomery's trick across lanes).
    let mut lpre = [0u64; K];
    let mut all = one;
    for l in 0..K {
        lpre[l] = all;
        all = f.mul(all, acc[l]);
    }
    let mut ai = f.inv(all);
    let mut inv = [0u64; K];
    for l in (0..K).rev() {
        inv[l] = f.mul(ai, lpre[l]);
        ai = f.mul(ai, acc[l]);
    }
    let w = 2 * a;
    for k in (0..a).rev() {
        let (b, c) = (bx[k], by[k]);
        let p = &pre[k * K..k * K + K];
        let mut om = [0u64; K];
        let mut op = [0u64; K];
        for l in 0..K {
            let d = f.sub(xr[l], b);
            let zero = d == 0;
            let dinv = f.mul(inv[l], p[l]);
            inv[l] = f.mul(inv[l], if zero { one } else { d });
            let base = f.add(xr[l], b);
            let lm = f.mul(f.add(yr[l], c), dinv);
            let lp = f.mul(f.sub(yr[l], c), dinv);
            let xm = f.sub(f.mul(lm, lm), base);
            let xp = f.sub(f.mul(lp, lp), base);
            om[l] = if zero { u64::MAX } else { xm };
            op[l] = if zero { u64::MAX } else { xp };
        }
        for l in 0..K {
            xs[l * w + 2 * k] = om[l];
            xs[l * w + 2 * k + 1] = op[l];
        }
    }
}

/// Arithmetic over all targets with `K` lanes; returns an xor sink.
fn sweep_lanes<const K: usize>(f: &Mont, bx: &[u64], by: &[u64], tm: &[(u64, u64)]) -> u64 {
    let a = bx.len();
    let mut pre = vec![0u64; a * K];
    let mut xs = vec![0u64; 2 * a * K];
    let mut sink = 0;
    for chunk in tm.chunks_exact(K) {
        let xr: [u64; K] = std::array::from_fn(|l| chunk[l].0);
        let yr: [u64; K] = std::array::from_fn(|l| chunk[l].1);
        row_lanes::<K>(f, bx, by, &xr, &yr, &mut pre, &mut xs);
        sink ^= xs[a] ^ xs[xs.len() - 1];
    }
    sink
}

/// The full pipeline with `K` lanes: arithmetic, filter, exact check.
fn pipeline<const K: usize>(
    f: &Mont,
    bx: &[u64],
    by: &[u64],
    tm: &[(u64, u64)],
    bloom: &WordBloom,
    keys: &HashSet<u64>,
) -> u64 {
    let a = bx.len();
    let mut pre = vec![0u64; a * K];
    let mut xs = vec![0u64; 2 * a * K];
    let mut found = 0;
    for chunk in tm.chunks_exact(K) {
        let xr: [u64; K] = std::array::from_fn(|l| chunk[l].0);
        let yr: [u64; K] = std::array::from_fn(|l| chunk[l].1);
        row_lanes::<K>(f, bx, by, &xr, &yr, &mut pre, &mut xs);
        for &x in &xs {
            if bloom.maybe(x) && x != u64::MAX && keys.contains(&x) {
                found += 1;
            }
        }
    }
    found
}

fn best<F: FnMut() -> u64>(reps: usize, mut run: F) -> (f64, u64) {
    let mut t = f64::INFINITY;
    let mut out = 0;
    for _ in 0..reps {
        let t0 = Instant::now();
        out = black_box(run());
        t = t.min(t0.elapsed().as_secs_f64());
    }
    (t, out)
}

fn list(s: &str) -> Vec<usize> {
    s.split(',').filter_map(|x| x.parse().ok()).collect()
}

macro_rules! lanes_dispatch {
    ($k:expr, $fn:ident, $($arg:expr),*) => {
        match $k {
            1 => $fn::<1>($($arg),*),
            2 => $fn::<2>($($arg),*),
            4 => $fn::<4>($($arg),*),
            8 => $fn::<8>($($arg),*),
            16 => $fn::<16>($($arg),*),
            _ => panic!("lanes must be 1, 2, 4, 8 or 16"),
        }
    };
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let get = |k: &str, d: &str| {
        args.iter()
            .position(|a| a == k)
            .and_then(|i| args.get(i + 1).cloned())
            .unwrap_or_else(|| d.to_string())
    };
    let bits: u32 = get("--bits", "32").parse().unwrap();
    let sizes = list(&get("--sizes", "256,1024"));
    let n_targets: usize = get("--targets", "4096").parse().unwrap();
    let n_check: usize = get("--check", "64").parse().unwrap();
    let lane_set = list(&get("--lanes", "1,2,4,8,16"));
    let thread_set = list(&get("--threads", "1,2,4"));
    let reps: usize = get("--reps", "5").parse().unwrap();

    let inst = prime_instance_for(bits, 59297);
    let g = &inst.curve;
    let f = Mont::new(g.p);
    let ctx = InstanceCtx {
        group: g,
        generator: inst.generator_point(),
        target: inst.generator_point(),
        r: inst.r,
        cofactor: inst.cofactor,
        group_order: inst.group_order,
        name: inst.name.clone(),
        field_degree: None,
    };
    let mut scratch = GroupOps::default();
    let mut state = 0x9E37_79B9_7F4A_7C15u64;
    let targets: Vec<PrimePoint> = (0..n_targets)
        .map(|_| {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            g.mul(&mut scratch, inst.generator_point(), 1 + state % (inst.r - 1))
        })
        .collect();
    let tm: Vec<(u64, u64)> = targets.iter().map(|t| (f.to(t.x), f.to(t.y))).collect();

    for &size in &sizes {
        let (_, bparams) = Params::parse_spec(&format!("prime-abscissa:size={size}")).unwrap();
        let fb = (PrimeAbscissaBase { instance: &inst })
            .build(&ctx, &bparams, &mut scratch)
            .expect("the base builds");
        let reps_pts: Vec<PrimePoint> = fb
            .abscissa_list
            .iter()
            .map(|x| fb.points[fb.x_index[x][0]])
            .filter(|p| !p.infinity && p.y != 0)
            .collect();
        let a = reps_pts.len();
        let bx: Vec<u64> = reps_pts.iter().map(|p| f.to(p.x)).collect();
        let by: Vec<u64> = reps_pts.iter().map(|p| f.to(p.y)).collect();

        // The table's key set: abscissae of P_i ± P_j, i <= j.  The library's
        // negation-folded table stores the same ~A^2 sums (one key per
        // representative i and both points of every later abscissa j).
        let mut keys: HashSet<u64> = HashSet::new();
        for i in 0..a {
            for j in i..a {
                for q in [reps_pts[j], g.neg(reps_pts[j])] {
                    let s = g.add(&mut scratch, reps_pts[i], q);
                    if !s.infinity {
                        keys.insert(f.to(s.x));
                    }
                }
            }
        }

        // Correctness: scalar kernel against the group law, lanes against
        // the scalar kernel.
        let mut pre = vec![0u64; a];
        let mut xs = vec![0u64; 2 * a];
        let mut law_mismatch = 0u64;
        for (t, &(xr, yr)) in targets.iter().zip(&tm).take(n_check) {
            row_scalar(&f, &bx, &by, xr, yr, &mut pre, &mut xs);
            for k in 0..a {
                let minus = g.add(&mut scratch, *t, g.neg(reps_pts[k]));
                let plus = g.add(&mut scratch, *t, reps_pts[k]);
                for (got, want) in [(xs[2 * k], minus), (xs[2 * k + 1], plus)] {
                    let ok = if t.x == reps_pts[k].x {
                        got == u64::MAX
                    } else {
                        !want.infinity && got == f.to(want.x)
                    };
                    law_mismatch += u64::from(!ok);
                }
            }
        }
        let mut lane_mismatch = 0u64;
        for &k in &lane_set {
            let mut lpre = vec![0u64; a * k];
            let mut lxs = vec![0u64; 2 * a * k];
            for chunk in tm[..n_check.max(k)].chunks_exact(k) {
                macro_rules! go {
                    ($K:literal) => {{
                        let xr: [u64; $K] = std::array::from_fn(|l| chunk[l].0);
                        let yr: [u64; $K] = std::array::from_fn(|l| chunk[l].1);
                        row_lanes::<$K>(&f, &bx, &by, &xr, &yr, &mut lpre, &mut lxs);
                    }};
                }
                match k {
                    1 => go!(1),
                    2 => go!(2),
                    4 => go!(4),
                    8 => go!(8),
                    16 => go!(16),
                    _ => panic!(),
                }
                for (l, &(xr, yr)) in chunk.iter().enumerate() {
                    row_scalar(&f, &bx, &by, xr, yr, &mut pre, &mut xs);
                    lane_mismatch += xs
                        .iter()
                        .zip(&lxs[l * 2 * a..(l + 1) * 2 * a])
                        .filter(|(u, v)| u != v)
                        .count() as u64;
                }
            }
        }

        // Reference hit count over all targets.
        let mut hits_ref = 0u64;
        for &(xr, yr) in &tm {
            row_scalar(&f, &bx, &by, xr, yr, &mut pre, &mut xs);
            hits_ref += xs.iter().filter(|x| **x != u64::MAX && keys.contains(x)).count() as u64;
        }
        let points = (n_targets * 2 * a) as f64;

        // Lever 2: arithmetic only.
        let (t_scalar, _) = best(reps, || {
            let mut s = 0;
            for &(xr, yr) in &tm {
                row_scalar(&f, &bx, &by, xr, yr, &mut pre, &mut xs);
                s ^= xs[a];
            }
            s
        });
        let mut lanes_ns = serde_json::Map::new();
        let mut best_k = 1;
        let mut best_t = f64::INFINITY;
        for &k in &lane_set {
            let (t, _) = best(reps, || lanes_dispatch!(k, sweep_lanes, &f, &bx, &by, &tm));
            lanes_ns.insert(k.to_string(), json!(t * 1e9 / points));
            if t < best_t {
                best_t = t;
                best_k = k;
            }
        }

        // Lever 3: filters alone, over precomputed abscissae of 64 targets
        // (a working set of 64 * 2A keys streamed sequentially).
        let sample_targets = 64.min(n_targets);
        let mut sample = Vec::with_capacity(sample_targets * 2 * a);
        for &(xr, yr) in &tm[..sample_targets] {
            row_scalar(&f, &bx, &by, xr, yr, &mut pre, &mut xs);
            sample.extend(xs.iter().copied().filter(|x| *x != u64::MAX));
        }
        let sample_hits = sample.iter().filter(|x| keys.contains(x)).count() as u64;
        let probe_rounds = (8_000_000 / sample.len()).max(1);
        let probes = (probe_rounds * sample.len()) as f64;
        let mut filters = Vec::new();
        let mut chosen: Option<WordBloom> = None;
        for (bpk, h) in [(16usize, 5u32), (8, 4), (4, 3)] {
            let mut bloom = WordBloom::new(keys.len(), bpk, h);
            for &k in &keys {
                bloom.insert(k);
            }
            let (t, pos) = best(reps, || {
                let mut pos = 0u64;
                for _ in 0..probe_rounds {
                    pos += sample.iter().filter(|x| bloom.maybe(**x)).count() as u64;
                }
                pos
            });
            let pos_per_round = pos / probe_rounds as u64;
            assert!(pos_per_round >= sample_hits, "a Bloom filter has no false negatives");
            let fpr = (pos_per_round - sample_hits) as f64 / sample.len() as f64;
            filters.push(json!({
                "bits_per_key": bpk,
                "bits_set": h,
                "bytes": bloom.bytes(),
                "ns_per_probe": t * 1e9 / probes,
                "false_positive_rate": fpr,
            }));
            if bpk == 8 {
                chosen = Some(bloom);
            } else if chosen.is_none() && bpk == 16 {
                chosen = Some(bloom);
            }
        }
        let (t_exact, _) = best(reps, || {
            let mut n = 0u64;
            for _ in 0..probe_rounds {
                n += sample.iter().filter(|x| keys.contains(x)).count() as u64;
            }
            n
        });
        // Re-build the 16-bit filter for the pipeline comparison.
        let mut bloom16 = WordBloom::new(keys.len(), 16, 5);
        for &k in &keys {
            bloom16.insert(k);
        }
        let bloom8 = chosen.unwrap();

        // Full pipeline, one thread, best lanes, 16-bit and 8-bit filters.
        let (t_full16, found16) =
            best(reps, || lanes_dispatch!(best_k, pipeline, &f, &bx, &by, &tm, &bloom16, &keys));
        let (t_full8, found8) =
            best(reps, || lanes_dispatch!(best_k, pipeline, &f, &bx, &by, &tm, &bloom8, &keys));
        assert_eq!(found16, hits_ref);
        assert_eq!(found8, hits_ref);
        let (pick, pick_name) = if t_full8 <= t_full16 {
            (&bloom8, 8)
        } else {
            (&bloom16, 16)
        };

        // Lever 4: threads over targets.
        let mut threads_ns = serde_json::Map::new();
        for &nt in &thread_set {
            let per = (tm.len() / nt / best_k) * best_k;
            let (t, found) = best(reps, || {
                std::thread::scope(|s| {
                    let hs: Vec<_> = (0..nt)
                        .map(|i| {
                            let part = &tm[i * per..(i + 1) * per];
                            let (f, bx, by, keys) = (&f, &bx, &by, &keys);
                            s.spawn(move || lanes_dispatch!(best_k, pipeline, f, bx, by, part, pick, keys))
                        })
                        .collect();
                    hs.into_iter().map(|h| h.join().unwrap()).sum::<u64>()
                })
            });
            let pts = (per * nt * 2 * a) as f64;
            if per * nt == tm.len() {
                assert_eq!(found, hits_ref);
            }
            threads_ns.insert(nt.to_string(), json!(t * 1e9 / pts));
        }

        println!(
            "{}",
            json!({
                "schema": "rootfind.lever-probe/v1",
                "isolation": "none",
                "bits": bits,
                "p": g.p,
                "abscissae": a,
                "table_keys": keys.len(),
                "targets": n_targets,
                "reps_best_of": reps,
                "group_law_mismatches": law_mismatch,
                "lane_mismatches": lane_mismatch,
                "exact_hits": hits_ref,
                "lever2_ns_per_point_scalar": t_scalar * 1e9 / points,
                "lever2_ns_per_point_by_lanes": lanes_ns,
                "best_lanes": best_k,
                "lever3_filters": filters,
                "lever3_exact_hashset_ns_per_probe": t_exact * 1e9 / probes,
                "pipeline_ns_per_point_filter16": t_full16 * 1e9 / points,
                "pipeline_ns_per_point_filter8": t_full8 * 1e9 / points,
                "lever4_filter_bits": pick_name,
                "lever4_ns_per_point_by_threads": threads_ns,
            })
        );
    }
}
