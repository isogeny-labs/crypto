//! A falsifying probe for the first levers of
//! `research/rootfind_baseline_100x_20261117/PLAN.md`: how fast can one
//! thread of scalar code enumerate and filter the `2A` signed points
//! `R ∓ P_k` of a target on a prime curve below 32 bits?
//!
//! Three changes against the library's best arm, measured separately:
//!
//! 1. **Shared denominators.**  `R − P_k` and `R + P_k` both divide by
//!    `x_R − x_k`, so one inverse serves two points.
//! 2. **Abscissa only, Montgomery-resident.**  The table is keyed by the
//!    abscissa, so no ordinate is formed, and the base, the target and the
//!    keys stay in Montgomery form: no conversion in or out per point.
//! 3. **A row-wide inversion and a cache-resident filter.**  One inversion a
//!    target (not one per 32 points), and a word-blocked Bloom filter of 16
//!    bits a key in front of the exact table, since nearly every probe misses.
//!
//! Correctness is checked, not assumed: for the first `--check` targets every
//! abscissa is compared with the library's group law, and over all targets
//! the set of exact hits equals the set found through the reference keys.
//! A prototype for planning; `[pcc:unverified]`, no isolation.
//!
//!     rootfind_floor_probe [--bits 32] [--sizes 64,256,1024] [--targets 2048]

use std::collections::HashSet;
use std::hint::black_box;
use std::time::Instant;

use crypto_lib::cryptanalysis::ic_boundary::{
    prime_instance_for, CountedGroup, GroupOps, PrimePoint,
};
use crypto_lib::cryptanalysis::ic_framework::plugins::PrimeAbscissaBase;
use crypto_lib::cryptanalysis::ic_framework::stages::{FactorBaseBuilder, InstanceCtx, Params};
use serde_json::json;

/// Montgomery arithmetic for an odd modulus below 2^32, `R = 2^32`.
#[derive(Clone, Copy)]
struct Mont {
    p: u64,
    ninv: u32,
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
            ninv: inv.wrapping_neg(),
            r2: ((r * r) % p as u128) as u64,
        }
    }
    #[inline(always)]
    fn mul(&self, a: u64, b: u64) -> u64 {
        let t = a * b;
        let m = u64::from((t as u32).wrapping_mul(self.ninv));
        let u = ((t as u128 + (m * self.p) as u128) >> 32) as u64;
        if u >= self.p {
            u - self.p
        } else {
            u
        }
    }
    #[inline(always)]
    fn sub(&self, a: u64, b: u64) -> u64 {
        if a >= b {
            a - b
        } else {
            a + self.p - b
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
        self.mul(a, self.r2)
    }
    fn inv(&self, a: u64) -> u64 {
        // a^(p-2), in Montgomery form.
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

/// A word-blocked Bloom filter: one 64-bit word a key, five bits in it.
struct WordBloom {
    words: Vec<u64>,
    mask: usize,
}

impl WordBloom {
    fn new(keys: usize) -> Self {
        // 16 bits a key, rounded up to a power of two of words.
        let words = (keys / 4).next_power_of_two().max(64);
        Self {
            words: vec![0; words],
            mask: words - 1,
        }
    }
    #[inline(always)]
    fn slot(&self, key: u64) -> (usize, u64) {
        let h = key.wrapping_mul(0x9E37_79B9_7F4A_7C15);
        let h2 = (h ^ (h >> 29)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        let mut bits = 0u64;
        let mut g = h2;
        for _ in 0..5 {
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
}

struct Row {
    d: Vec<u64>,
    pre: Vec<u64>,
    xs: Vec<u64>,
}

/// The `2A` abscissae of `R ∓ P_k`, Montgomery form, into `row.xs` as
/// `[x(R − P_0), x(R + P_0), x(R − P_1), …]`.  A base point with
/// `x_k = x_R` (so `R = ±P_k`) gets the sentinel `u64::MAX` twice.
#[inline(always)]
fn row_abscissae(f: &Mont, bx: &[u64], by: &[u64], xr: u64, yr: u64, row: &mut Row) {
    let a = bx.len();
    let one = f.to(1);
    let mut acc = one;
    for k in 0..a {
        let d = f.sub(xr, bx[k]);
        row.d[k] = d;
        row.pre[k] = acc;
        if d != 0 {
            acc = f.mul(acc, d);
        }
    }
    let mut inv = f.inv(acc);
    for k in (0..a).rev() {
        let d = row.d[k];
        if d == 0 {
            row.xs[2 * k] = u64::MAX;
            row.xs[2 * k + 1] = u64::MAX;
            continue;
        }
        let dinv = f.mul(inv, row.pre[k]);
        inv = f.mul(inv, d);
        let base = f.add(xr, bx[k]);
        let lm = f.mul(f.add(yr, by[k]), dinv);
        let lp = f.mul(f.sub(yr, by[k]), dinv);
        row.xs[2 * k] = f.sub(f.mul(lm, lm), base);
        row.xs[2 * k + 1] = f.sub(f.mul(lp, lp), base);
    }
}

fn list(s: &str) -> Vec<usize> {
    s.split(',').filter_map(|x| x.parse().ok()).collect()
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
    let sizes = list(&get("--sizes", "64,256,1024"));
    let n_targets: usize = get("--targets", "2048").parse().unwrap();
    let n_check: usize = get("--check", "64").parse().unwrap();

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
            g.mul(
                &mut scratch,
                inst.generator_point(),
                1 + state % (inst.r - 1),
            )
        })
        .collect();

    for &size in &sizes {
        let (_, bparams) = Params::parse_spec(&format!("prime-abscissa:size={size}")).unwrap();
        let fb = (PrimeAbscissaBase { instance: &inst })
            .build(&ctx, &bparams, &mut scratch)
            .expect("the base builds");
        // One representative an abscissa, finite and with y != 0 only.
        let reps: Vec<PrimePoint> = fb
            .abscissa_list
            .iter()
            .map(|x| fb.points[fb.x_index[x][0]])
            .filter(|p| !p.infinity && p.y != 0)
            .collect();
        let a = reps.len();
        let bx: Vec<u64> = reps.iter().map(|p| f.to(p.x)).collect();
        let by: Vec<u64> = reps.iter().map(|p| f.to(p.y)).collect();

        // Reference keys: the abscissae of P_i ± P_j over representatives.
        let mut keys: HashSet<u64> = HashSet::new();
        for i in 0..a {
            for j in i..a {
                for q in [reps[j], g.neg(reps[j])] {
                    let s = g.add(&mut scratch, reps[i], q);
                    if !s.infinity {
                        keys.insert(f.to(s.x));
                    }
                }
            }
        }
        let mut bloom = WordBloom::new(keys.len());
        for &k in &keys {
            bloom.insert(k);
        }

        let mut row = Row {
            d: vec![0; a],
            pre: vec![0; a],
            xs: vec![0; 2 * a],
        };
        let tm: Vec<(u64, u64)> = targets.iter().map(|t| (f.to(t.x), f.to(t.y))).collect();

        // Correctness 1: every abscissa equals the library's group law.
        let mut mismatches = 0u64;
        for (t, &(xr, yr)) in targets.iter().zip(&tm).take(n_check) {
            row_abscissae(&f, &bx, &by, xr, yr, &mut row);
            for k in 0..a {
                let minus = g.add(&mut scratch, *t, g.neg(reps[k]));
                let plus = g.add(&mut scratch, *t, reps[k]);
                for (got, want) in [(row.xs[2 * k], minus), (row.xs[2 * k + 1], plus)] {
                    let ok = if t.x == reps[k].x {
                        got == u64::MAX
                    } else {
                        !want.infinity && got == f.to(want.x)
                    };
                    mismatches += u64::from(!ok);
                }
            }
        }

        // Correctness 2: exact hits through the filter equal exact hits
        // without it (the filter has no false negatives), over all targets.
        let (mut hits_ref, mut hits_filtered, mut positives) = (0u64, 0u64, 0u64);
        for &(xr, yr) in &tm {
            row_abscissae(&f, &bx, &by, xr, yr, &mut row);
            for &x in &row.xs {
                if x == u64::MAX {
                    continue;
                }
                let exact = keys.contains(&x);
                hits_ref += u64::from(exact);
                if bloom.maybe(x) {
                    positives += 1;
                    hits_filtered += u64::from(exact);
                }
            }
        }

        // Timing: arithmetic alone, then arithmetic with the filter and the
        // exact check on filter positives.  Best of five passes.
        let points = (n_targets * 2 * a) as f64;
        let (mut t_arith, mut t_full) = (f64::INFINITY, f64::INFINITY);
        let mut sink = 0u64;
        for _ in 0..5 {
            let t0 = Instant::now();
            for &(xr, yr) in &tm {
                row_abscissae(&f, &bx, &by, xr, yr, &mut row);
                sink ^= row.xs[a];
            }
            t_arith = t_arith.min(t0.elapsed().as_secs_f64());
            let t1 = Instant::now();
            let mut found = 0u64;
            for &(xr, yr) in &tm {
                row_abscissae(&f, &bx, &by, xr, yr, &mut row);
                for &x in &row.xs {
                    if bloom.maybe(x) && x != u64::MAX && keys.contains(&x) {
                        found += 1;
                    }
                }
            }
            t_full = t_full.min(t1.elapsed().as_secs_f64());
            assert_eq!(found, hits_ref);
        }
        black_box(sink);
        println!(
            "{}",
            json!({
                "schema": "rootfind.floor-probe/v1",
                "isolation": "none",
                "bits": bits,
                "p": g.p,
                "abscissae": a,
                "table_keys": keys.len(),
                "filter_bytes": bloom.words.len() * 8,
                "targets": n_targets,
                "abscissa_mismatches": mismatches,
                "abscissae_checked": n_check.min(n_targets) * 2 * a,
                "exact_hits": hits_ref,
                "exact_hits_through_filter": hits_filtered,
                "filter_positives": positives,
                "filter_false_positive_rate": (positives - hits_filtered) as f64 / points,
                "ns_per_point_arithmetic": t_arith * 1e9 / points,
                "ns_per_point_with_filter": t_full * 1e9 / points,
                "field_muls_per_point": 3.5,
            })
        );
    }
}
