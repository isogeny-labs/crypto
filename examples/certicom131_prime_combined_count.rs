//! Falsifier B of `docs/ic/CERTICOM131_IC_PROGRAM.md`: the combined-system
//! solution count of `RESEARCH_PRIME_FIELD_BREAKTHROUGH_PROGRAM.md` §4 item 2
//! (listed in `research_notes/Index calculus leads next steps/` as never run).
//!
//! For a base `V ⊂ F_p` of size `B` and a target abscissa `x_R`, the combined
//! system is `S_{m+1}(x_1, …, x_m, x_R) = 0` with `L_V(x_i) = 0`, where
//! `L_V = ∏_{v∈V}(X − v)`.  `L_V` is square-free and splits over `F_p`, so the
//! ideal is radical and its number of solutions over `F̄_p` equals its number
//! of `F_p`-solutions, `N_m(V, x_R)`: the decompositions of `x_R` over `V`.
//! The note's claim is `N_m ≥ B^{m−1}` for some structured `V` ("the lever is
//! dead"); the sharper, directly informative quantity is the **excess**
//! `N_m(V) / N_m(V_random)` against a uniformly random `B`-subset of `F_p`
//! (and the on-curve version against the random-oracle expectation
//! `2^m B^m / r`).  An excess ≫ 1 on a structured base would be an
//! algebraic lever in relation probability; an excess ≈ 1 is the null.
//!
//! Counts are exact: `m = 2` by solving the quadratic `S₃(x_1, x_2, x_R) = 0`
//! for `x_2` per `x_1 ∈ V`; `m = 3` by solving for the partial sum `u` per
//! pair and then for `x_3`, with membership in `V` by hash set.  Tonelli–Shanks
//! square roots, `p < 2^63`.
//!
//! ```sh
//! cargo run --release --example certicom131_prime_combined_count -- \
//!   --curves research/pkm_tower_ecbench_20261013/curves.jsonl --targets 8 --seed 20261020
//! ```
//! One JSON line per (curve, base, m, target) on stdout; summary on stderr.

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::collections::HashSet;

#[derive(Clone, Copy)]
struct Fp(u64);

impl Fp {
    fn mul(self, a: u64, b: u64) -> u64 {
        ((a as u128 * b as u128) % self.0 as u128) as u64
    }
    fn add(self, a: u64, b: u64) -> u64 {
        let s = a as u128 + b as u128;
        (s % self.0 as u128) as u64
    }
    fn sub(self, a: u64, b: u64) -> u64 {
        if a >= b {
            a - b
        } else {
            a + self.0 - b
        }
    }
    fn pow(self, mut a: u64, mut e: u64) -> u64 {
        let mut r = 1u64;
        while e > 0 {
            if e & 1 == 1 {
                r = self.mul(r, a);
            }
            a = self.mul(a, a);
            e >>= 1;
        }
        r
    }
    fn inv(self, a: u64) -> u64 {
        self.pow(a, self.0 - 2)
    }
    fn is_square(self, a: u64) -> bool {
        a == 0 || self.pow(a, (self.0 - 1) / 2) == 1
    }
}

struct Sqrt {
    f: Fp,
    q: u64,
    s: u32,
    z: u64, // non-residue^q
}

impl Sqrt {
    fn new(f: Fp) -> Self {
        let mut q = f.0 - 1;
        let mut s = 0;
        while q.is_multiple_of(2) {
            q /= 2;
            s += 1;
        }
        let mut n = 2;
        while f.is_square(n) {
            n += 1;
        }
        Sqrt {
            f,
            q,
            s,
            z: f.pow(n, q),
        }
    }
    fn sqrt(&self, a: u64) -> Option<u64> {
        let f = self.f;
        if a == 0 {
            return Some(0);
        }
        if !f.is_square(a) {
            return None;
        }
        let (mut m, mut c) = (self.s, self.z);
        let mut t = f.pow(a, self.q);
        let mut r = f.pow(a, self.q.div_ceil(2));
        while t != 1 {
            let mut i = 0;
            let mut tt = t;
            while tt != 1 {
                tt = f.mul(tt, tt);
                i += 1;
            }
            let mut b = c;
            for _ in 0..m - i - 1 {
                b = f.mul(b, b);
            }
            m = i;
            c = f.mul(b, b);
            t = f.mul(t, c);
            r = f.mul(r, b);
        }
        Some(r)
    }
}

/// Coefficients of `S₃(x, y, X)` as a quadratic `A X² + Bc X + C` in `X`.
fn s3_quad(f: Fp, a: u64, b: u64, x: u64, y: u64) -> (u64, u64, u64) {
    let d = f.sub(x, y);
    let big_a = f.mul(d, d);
    let xy = f.mul(x, y);
    let s = f.add(x, y);
    let t = f.add(f.mul(s, f.add(xy, a)), f.add(b, b));
    let big_b = f.sub(0, f.add(t, t));
    let u = f.sub(xy, a);
    let big_c = f.sub(f.mul(u, u), f.mul(f.add(f.add(b, b), f.add(b, b)), s));
    (big_a, big_b, big_c)
}

/// The `F_p`-roots `X` of `S₃(x, y, X) = 0` (zero, one or two; none when
/// `x = y` degenerates the quadratic to a line, whose root is returned).
fn s3_roots(f: Fp, sq: &Sqrt, a: u64, b: u64, x: u64, y: u64, out: &mut [u64; 2]) -> usize {
    let (ca, cb, cc) = s3_quad(f, a, b, x, y);
    if ca == 0 {
        if cb == 0 {
            return 0;
        }
        out[0] = f.mul(f.sub(0, cc), f.inv(cb));
        return 1;
    }
    let disc = f.sub(
        f.mul(cb, cb),
        f.mul(f.add(f.add(ca, ca), f.add(ca, ca)), cc),
    );
    let Some(r) = sq.sqrt(disc) else {
        return 0;
    };
    let inv2a = f.inv(f.add(ca, ca));
    let nb = f.sub(0, cb);
    out[0] = f.mul(f.add(nb, r), inv2a);
    if r == 0 {
        return 1;
    }
    out[1] = f.mul(f.sub(nb, r), inv2a);
    2
}

fn count_m2(f: Fp, sq: &Sqrt, a: u64, b: u64, v: &[u64], set: &HashSet<u64>, xr: u64) -> u64 {
    let mut n = 0;
    let mut roots = [0u64; 2];
    for &x1 in v {
        let k = s3_roots(f, sq, a, b, x1, xr, &mut roots);
        n += roots[..k]
            .iter()
            .filter(|&&x2| x2 > x1 && set.contains(&x2))
            .count() as u64;
    }
    n
}

/// Number of `(x1 < x2, x3)` with `x_i ∈ V` and `S₄ = 0` (so an unordered
/// solution with distinct entries is counted three times).
fn count_m3(f: Fp, sq: &Sqrt, a: u64, b: u64, v: &[u64], set: &HashSet<u64>, xr: u64) -> u64 {
    let mut n = 0;
    let mut us = [0u64; 2];
    let mut x3s = [0u64; 2];
    for (i, &x1) in v.iter().enumerate() {
        for &x2 in &v[i + 1..] {
            let k = s3_roots(f, sq, a, b, x1, x2, &mut us);
            for &u in &us[..k] {
                let j = s3_roots(f, sq, a, b, u, xr, &mut x3s);
                n += x3s[..j].iter().filter(|x| set.contains(x)).count() as u64;
            }
        }
    }
    n
}

fn is_prime_u64(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    let mut d = 2;
    while d * d <= n {
        if n.is_multiple_of(d) {
            return false;
        }
        d += 1;
    }
    true
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let flag = |name: &str| {
        args.iter()
            .position(|a| a == name)
            .and_then(|i| args.get(i + 1))
            .cloned()
    };
    let path = flag("--curves").expect("--curves curves.jsonl");
    let targets: usize = flag("--targets").map_or(8, |v| v.parse().unwrap());
    let seed: u64 = flag("--seed").map_or(20261020, |v| v.parse().unwrap());
    let max_pairs: u64 = flag("--max-b").map_or(4096, |v| v.parse().unwrap());
    let only_bits: Option<Vec<u32>> = flag("--bits").map(|s| {
        s.split(',')
            .map(|x| x.parse().expect("--bits 20,22"))
            .collect()
    });
    let text = std::fs::read_to_string(&path).expect("read curves");
    let mut summary: Vec<(u32, String, usize, f64, f64, f64)> = Vec::new();
    for line in text.lines().filter(|l| l.trim_start().starts_with('{')) {
        let v: serde_json::Value = serde_json::from_str(line).unwrap();
        let g = |k: &str| v[k].as_u64().unwrap();
        let (p, a, b, r) = (g("p"), g("a"), g("b"), g("group_order"));
        let bits = 64 - p.leading_zeros();
        if let Some(bs) = &only_bits {
            if !bs.contains(&bits) {
                continue;
            }
        }
        let f = Fp(p);
        let sq = Sqrt::new(f);
        let mut rng = StdRng::seed_from_u64(seed ^ p);
        let s2 = (p - 1).trailing_zeros();
        // Targets: abscissae of random multiples of nothing in particular --
        // the count depends on x_R only through the curve; use random x_R
        // that lie on the curve.
        let on_curve = |x: u64| {
            let rhs = f.add(f.add(f.mul(f.mul(x, x), x), f.mul(a, x)), b);
            f.is_square(rhs)
        };
        let mut xrs = Vec::new();
        while xrs.len() < targets {
            let x = rng.gen_range(1..p);
            if on_curve(x) {
                xrs.push(x);
            }
        }
        // Base sizes: B = 2^t for the Kummer tower, t from 6 to the cap.
        let tmax = (63 - max_pairs.leading_zeros()).min(s2);
        for t in 6..=tmax {
            let bsz = 1u64 << t;
            // 2^t-th roots of unity (generator ^ ((p-1)/2^t)).
            let root = {
                let mut gen = 2;
                loop {
                    let w = f.pow(gen, (p - 1) >> t);
                    if f.pow(w, 1 << (t - 1)) != 1 {
                        break w;
                    }
                    gen += 1;
                }
            };
            let mu: Vec<u64> = (0..bsz)
                .scan(1u64, |c, _| {
                    let o = *c;
                    *c = f.mul(*c, root);
                    Some(o)
                })
                .collect();
            let coset_g = loop {
                let g = rng.gen_range(2..p);
                if g != 1 {
                    break g;
                }
            };
            let coset: Vec<u64> = mu.iter().map(|&x| f.mul(x, coset_g)).collect();
            // A subgroup of non-2-power order d | p - 1 with d near B.
            let mut d_best = 0u64;
            for d in (bsz / 2)..=(bsz * 2) {
                if (p - 1) % d == 0
                    && !d.is_power_of_two()
                    && (d_best == 0 || d.abs_diff(bsz) < d_best.abs_diff(bsz))
                {
                    d_best = d;
                }
            }
            let sub: Option<Vec<u64>> = (d_best > 0).then(|| {
                let mut gen = 2;
                let w = loop {
                    let w = f.pow(gen, (p - 1) / d_best);
                    // exact order d_best: check against prime factors.
                    let ok = (2..=d_best)
                        .filter(|q| d_best.is_multiple_of(*q) && is_prime_u64(*q))
                        .all(|q| f.pow(w, d_best / q) != 1);
                    if ok {
                        break w;
                    }
                    gen += 1;
                };
                let mut c = 1;
                (0..d_best)
                    .map(|_| {
                        let o = c;
                        c = f.mul(c, w);
                        o
                    })
                    .collect()
            });
            let mut rand_set: HashSet<u64> = HashSet::new();
            while (rand_set.len() as u64) < bsz {
                rand_set.insert(rng.gen_range(1..p));
            }
            let randv: Vec<u64> = rand_set.iter().copied().collect();
            let interval: Vec<u64> = (1..=bsz).collect();
            let mut bases: Vec<(String, Vec<u64>)> = vec![
                (format!("kummer_mu_2^{t}"), mu.clone()),
                (format!("kummer_coset_2^{t}"), coset),
                ("random_subset".into(), randv),
                ("interval_1_B".into(), interval),
            ];
            if let Some(s) = sub {
                bases.push((format!("subgroup_d{d_best}"), s));
            }
            // Also each base restricted to abscissae on the curve.
            let mut extra = Vec::new();
            for (name, vs) in &bases {
                let on: Vec<u64> = vs.iter().copied().filter(|&x| on_curve(x)).collect();
                extra.push((format!("{name}|on_curve"), on));
            }
            bases.extend(extra);
            for (name, vs) in &bases {
                let set: HashSet<u64> = vs.iter().copied().collect();
                let big_b = vs.len() as f64;
                for m in [2u32, 3] {
                    if m == 3 && vs.len() > 4096 {
                        continue;
                    }
                    let mut total = 0u64;
                    for &xr in &xrs {
                        let n = if m == 2 {
                            count_m2(f, &sq, a, b, vs, &set, xr)
                        } else {
                            count_m3(f, &sq, a, b, vs, &set, xr)
                        };
                        total += n;
                        println!(
                            r#"{{"p":{p},"bits":{bits},"base":"{name}","B":{},"m":{m},"x_r":{xr},"count":{n}}}"#,
                            vs.len()
                        );
                    }
                    let mean = total as f64 / xrs.len() as f64;
                    // Random-oracle expectation on the curve: 2^m B^m / r over
                    // multiset-with-order conventions: m=2 counts x1<x2 pairs
                    // (2^2 B^2 / (2 r)); m=3 counts (x1<x2, x3) = 3 per
                    // unordered triple: 2^3 B^3 / (6 r) * 3.
                    let expect = if m == 2 {
                        4.0 * big_b * big_b / (2.0 * r as f64)
                    } else {
                        8.0 * big_b.powi(3) / (6.0 * r as f64) * 3.0
                    };
                    summary.push((bits, name.clone(), m as usize, big_b, mean, expect));
                }
            }
        }
    }
    eprintln!("| bits | base | m | B | mean N_m | on-curve random-oracle E | N/E | B^(m-1) |");
    eprintln!("|--:|:--|--:|--:|--:|--:|--:|--:|");
    for (bits, name, m, b, mean, expect) in &summary {
        let e = if name.ends_with("|on_curve") {
            *expect
        } else {
            f64::NAN
        };
        eprintln!(
            "| {bits} | {name} | {m} | {b:.0} | {mean:.2} | {} | {} | {:.0} |",
            if e.is_nan() {
                "-".into()
            } else {
                format!("{e:.2}")
            },
            if e.is_nan() {
                "-".into()
            } else {
                format!("{:.3}", mean / e)
            },
            b.powi(*m as i32 - 1)
        );
    }
}
