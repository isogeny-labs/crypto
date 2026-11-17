//! L4-C of `research/prime_s5_small_root_leads_20261028/`: is the number of
//! small 3-decompositions of a target `R` on a prime-order curve over `F_p`
//! distinguishable from a null model in the gap between lattice reach and
//! the S₄ milestone?
//!
//! For a factor base `F` (one point per abscissa `x ≤ B = ⌊p^δ⌋` on the
//! curve) the census enumerates every multiset of three signed points of
//! `±F` that contains no pair `{P, −P}`, adds them, and counts how many land
//! on each `R`.  `N_R` is that count; a decomposition `±P₁ ± P₂ ± P₃ = R` is
//! one such multiset, and generically one multiset per vanishing of
//! `S₄(x₁, x₂, x₃, x(R))`.  The census is exact over all `r` points of the
//! group, so no target is sampled.  Null 1 replaces `F` by the same number
//! of uniformly random points.
//!
//! Output, one JSON line per (curve, δ, factor base): total multisets, the
//! mean `λ = total/(r − 1)`, the dispersion index `Var/λ` over all `R ≠ O`,
//! the count of `R` with `N_R ≥ 2` against the Poisson value, and the
//! fraction of those whose decompositions share a summand.
//!
//! ```text
//! cargo run --release --example semaev_gap_census -- --bits 26,28,30 \
//!     --deltas 0.0417,0.0833,0.125,0.1667,0.2,0.25,0.3 --nulls 5 --seed 20261028
//! ```

use crypto_lib::cryptanalysis::semaev_prime_poly::Fp;
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Pt {
    Inf,
    A(u64, u64),
}

struct Ec {
    f: Fp,
    a: u64,
    b: u64,
}

fn inv(f: Fp, a: u64) -> u64 {
    // Extended Euclid; `a` nonzero mod p.
    let (mut r0, mut r1) = (f.p as i128, a as i128);
    let (mut t0, mut t1) = (0i128, 1i128);
    while r1 != 0 {
        let q = r0 / r1;
        (r0, r1) = (r1, r0 - q * r1);
        (t0, t1) = (t1, t0 - q * t1);
    }
    t0.rem_euclid(f.p as i128) as u64
}

impl Ec {
    fn rhs(&self, x: u64) -> u64 {
        let f = self.f;
        f.add(f.mul(f.add(f.mul(x, x), self.a), x), self.b)
    }
    fn neg(&self, p: Pt) -> Pt {
        match p {
            Pt::Inf => Pt::Inf,
            Pt::A(x, y) => Pt::A(x, self.f.neg(y)),
        }
    }
    fn add(&self, p: Pt, q: Pt) -> Pt {
        let f = self.f;
        match (p, q) {
            (Pt::Inf, _) => q,
            (_, Pt::Inf) => p,
            (Pt::A(x1, y1), Pt::A(x2, y2)) => {
                let lam = if x1 == x2 {
                    if f.add(y1, y2) == 0 {
                        return Pt::Inf;
                    }
                    let num = f.add(f.mul(3, f.mul(x1, x1)), self.a);
                    f.mul(num, inv(f, f.mul(2, y1)))
                } else {
                    f.mul(f.sub(y2, y1), inv(f, f.sub(x2, x1)))
                };
                let x3 = f.sub(f.sub(f.mul(lam, lam), x1), x2);
                let y3 = f.sub(f.mul(lam, f.sub(x1, x3)), y1);
                Pt::A(x3, y3)
            }
        }
    }
    fn mul(&self, mut k: u64, p: Pt) -> Pt {
        let mut acc = Pt::Inf;
        let mut base = p;
        while k > 0 {
            if k & 1 == 1 {
                acc = self.add(acc, base);
            }
            base = self.add(base, base);
            k >>= 1;
        }
        acc
    }
    /// A square root of `rhs(x)` if `x` is an abscissa (Tonelli–Shanks).
    fn y_of(&self, x: u64) -> Option<u64> {
        sqrt(self.f, self.rhs(x))
    }
    fn key(p: Pt) -> u64 {
        match p {
            Pt::Inf => u64::MAX,
            Pt::A(x, y) => (x << 1) | (y & 1),
        }
    }
}

fn sqrt(f: Fp, a: u64) -> Option<u64> {
    if a == 0 {
        return Some(0);
    }
    if f.pow(a, (f.p - 1) / 2) != 1 {
        return None;
    }
    let (mut q, mut s) = (f.p - 1, 0u32);
    while q % 2 == 0 {
        q /= 2;
        s += 1;
    }
    let mut z = 2;
    while f.pow(z, (f.p - 1) / 2) != f.p - 1 {
        z += 1;
    }
    let (mut m, mut c, mut t, mut r) = (s, f.pow(z, q), f.pow(a, q), f.pow(a, q.div_ceil(2)));
    while t != 1 {
        let mut i = 0;
        let mut tt = t;
        while tt != 1 {
            tt = f.mul(tt, tt);
            i += 1;
        }
        let b = f.pow(c, 1 << (m - i - 1));
        m = i;
        c = f.mul(b, b);
        t = f.mul(t, c);
        r = f.mul(r, b);
    }
    Some(r)
}

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    for q in [2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        if n.is_multiple_of(q) {
            return n == q;
        }
    }
    let f = Fp { p: n };
    let (mut d, mut s) = (n - 1, 0);
    while d % 2 == 0 {
        d /= 2;
        s += 1;
    }
    'w: for a in [2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        let mut x = f.pow(a, d);
        if x == 1 || x == n - 1 {
            continue;
        }
        for _ in 1..s {
            x = f.mul(x, x);
            if x == n - 1 {
                continue 'w;
            }
        }
        return false;
    }
    true
}

/// The group order when it is prime: the unique `N` in the Hasse interval
/// with `[N]P = O` for a random point `P`, found by baby-step giant-step.
fn prime_order(ec: &Ec, rng: &mut StdRng) -> Option<u64> {
    let p = ec.f.p;
    let pt = loop {
        let x = rng.gen_range(0..p);
        if let Some(y) = ec.y_of(x) {
            break Pt::A(x, y);
        }
    };
    let w = 2 * ((p as f64).sqrt() as u64 + 1);
    let lo = p + 1 - w;
    let m = ((2 * w) as f64).sqrt() as u64 + 1;
    let mut baby = HashMap::new();
    let mut cur = Pt::Inf;
    for j in 0..m {
        baby.entry(Ec::key(cur)).or_insert(j);
        cur = ec.add(cur, pt);
    }
    let giant = ec.mul(m, pt);
    let mut g = ec.mul(lo, pt);
    let mut found = Vec::new();
    for i in 0..=(2 * w / m + 1) {
        // [lo + i·m]P = [j]P  ⇒  [lo + i·m − j]P = O.
        if let Some(&j) = baby.get(&Ec::key(g)) {
            let n = lo + i * m - j;
            if n >= lo && n <= p + 1 + w && ec.mul(n, pt) == Pt::Inf {
                found.push(n);
            }
        }
        g = ec.add(g, giant);
    }
    found.sort();
    found.dedup();
    match found.as_slice() {
        [n] if is_prime(*n) => Some(*n),
        _ => None,
    }
}

struct CurveRow {
    ec: Ec,
    r: u64,
    shape: &'static str,
}

/// The largest prime `p < 2^bits` with `p ≡ 3 (mod 4)` (and `≡ 1 (mod 3)`
/// for the `a = 0` shape, which is ordinary only then) that has a
/// prime-order curve of the shape: for `a = −3`, the first of 200 seeded
/// random `b`; for `a = 0`, the smallest `b ∈ [1, 64]` (all six sextic
/// classes appear there).  Primes without one are skipped.
fn find_curve(bits: u32, shape: &'static str, rng: &mut StdRng) -> CurveRow {
    let mut p = (1u64 << bits) - 1;
    loop {
        p -= 1;
        if !(p % 4 == 3 && (shape != "a0" || p % 3 == 1) && is_prime(p)) {
            continue;
        }
        let f = Fp::new(p);
        for t in 0..200u64 {
            let (a, b) = if shape == "a0" {
                if t >= 64 {
                    break;
                }
                (0, t + 1)
            } else {
                (f.neg(3), rng.gen_range(1..p))
            };
            let ec = Ec { f, a, b };
            let disc = f.add(f.mul(4, f.pow(a, 3)), f.mul(27, f.mul(b, b)));
            if disc == 0 {
                continue;
            }
            if let Some(r) = prime_order(&ec, rng) {
                return CurveRow { ec, r, shape };
            }
        }
    }
}

#[derive(Default)]
struct Census {
    base: usize,
    total: u64,
    lambda: f64,
    dispersion: f64,
    ge2: u64,
    pair_collisions: u64,
    pair_expected: f64,
    triple_collisions: u64,
    triple_unshared: u64,
    triple_expected: f64,
    ge3: u64,
    ge2_shared: f64,
    max_n: u64,
}

fn census(ec: &Ec, r: u64, base: &[Pt]) -> Census {
    // Signed points; index 2i is P_i, 2i+1 is −P_i.
    let signed: Vec<Pt> = base.iter().flat_map(|&p| [p, ec.neg(p)]).collect();
    let n = signed.len();
    let mut sums: Vec<(u64, u32, u32, u32)> = Vec::new();
    for i in 0..n {
        for j in i..n {
            if j == (i ^ 1) {
                continue;
            }
            let pij = ec.add(signed[i], signed[j]);
            for k in j..n {
                if k == (i ^ 1) || k == (j ^ 1) {
                    continue;
                }
                let s = ec.add(pij, signed[k]);
                if s != Pt::Inf {
                    sums.push((Ec::key(s), i as u32, j as u32, k as u32));
                }
            }
        }
    }
    sums.sort_unstable();
    let total = sums.len() as u64;
    let lambda = total as f64 / (r - 1) as f64;
    let mut sq = 0f64;
    let mut out = Census {
        base: base.len(),
        total,
        lambda,
        ..Default::default()
    };
    let mut shared = 0u64;
    let mut a = 0;
    while a < sums.len() {
        let mut b = a + 1;
        while b < sums.len() && sums[b].0 == sums[a].0 {
            b += 1;
        }
        let c = (b - a) as u64;
        sq += (c * c) as f64;
        out.max_n = out.max_n.max(c);
        if c >= 2 {
            out.ge2 += 1;
            if c >= 3 {
                out.ge3 += 1;
            }
            // Do two decompositions use the same base abscissa?
            let absc = |t: &(u64, u32, u32, u32)| [t.1 / 2, t.2 / 2, t.3 / 2];
            for u in a..b {
                for v in u + 1..b {
                    out.triple_collisions += 1;
                    let (x, y) = (absc(&sums[u]), absc(&sums[v]));
                    if !x.iter().any(|e| y.contains(e)) {
                        out.triple_unshared += 1;
                    }
                }
            }
            let any = (a..b).any(|u| {
                (u + 1..b).any(|v| {
                    let (x, y) = (absc(&sums[u]), absc(&sums[v]));
                    x.iter().any(|e| y.contains(e))
                })
            });
            if any {
                shared += 1;
            }
        }
        a = b;
    }
    let mean_sq = sq / (r - 1) as f64;
    out.dispersion = if lambda > 0.0 {
        (mean_sq - lambda * lambda) / lambda
    } else {
        f64::NAN
    };
    let birthday = |m: f64| m * (m - 1.0) / 2.0 / (r - 1) as f64;
    out.triple_expected = birthday(total as f64);
    let mut pairs: Vec<u64> = Vec::new();
    for i in 0..n {
        for j in i..n {
            if j != (i ^ 1) {
                let s = ec.add(signed[i], signed[j]);
                if s != Pt::Inf {
                    pairs.push(Ec::key(s));
                }
            }
        }
    }
    pairs.sort_unstable();
    out.pair_expected = birthday(pairs.len() as f64);
    let mut a = 0;
    while a < pairs.len() {
        let mut b = a + 1;
        while b < pairs.len() && pairs[b] == pairs[a] {
            b += 1;
        }
        out.pair_collisions += ((b - a) * (b - a - 1) / 2) as u64;
        a = b;
    }
    out.ge2_shared = if out.ge2 > 0 {
        shared as f64 / out.ge2 as f64
    } else {
        f64::NAN
    };
    out
}

fn small_x_base(ec: &Ec, bound: u64) -> Vec<Pt> {
    (0..=bound)
        .filter_map(|x| ec.y_of(x).filter(|&y| y != 0).map(|y| Pt::A(x, y)))
        .collect()
}

fn random_base(ec: &Ec, size: usize, rng: &mut StdRng) -> Vec<Pt> {
    let mut seen = std::collections::HashSet::new();
    let mut out = Vec::new();
    while out.len() < size {
        let x = rng.gen_range(0..ec.f.p);
        if let Some(y) = ec.y_of(x).filter(|&y| y != 0) {
            if seen.insert(x) {
                out.push(Pt::A(x, y));
            }
        }
    }
    out
}

fn arg<'a>(args: &'a [String], name: &str, default: &'a str) -> &'a str {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .map(String::as_str)
        .unwrap_or(default)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let bits: Vec<u32> = arg(&args, "--bits", "26,28,30")
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    let deltas: Vec<f64> = arg(&args, "--deltas", "0.0417,0.0833,0.125,0.1667,0.2,0.25,0.3")
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    let nulls: usize = arg(&args, "--nulls", "5").parse().unwrap();
    let seed: u64 = arg(&args, "--seed", "20261028").parse().unwrap();
    let max_base: usize = arg(&args, "--max-base", "256").parse().unwrap();
    for &bt in &bits {
        for shape in ["a-3", "a0"] {
            let mut rng = StdRng::seed_from_u64(seed ^ ((bt as u64) << 8) ^ (shape.len() as u64));
            let c = find_curve(bt, shape, &mut rng);
            eprintln!(
                "curve bits={bt} shape={shape} p={} b={} r={}",
                c.ec.f.p, c.ec.b, c.r
            );
            let p = c.ec.f.p;
            for &d in &deltas {
                let bound = (p as f64).powf(d).floor() as u64;
                let base = small_x_base(&c.ec, bound);
                if base.len() > max_base {
                    eprintln!(
                        "skip bits={bt} shape={shape} delta={d}: base {} > {max_base}",
                        base.len()
                    );
                    continue;
                }
                let mut runs = vec![("small_x".to_string(), base.clone())];
                for k in 0..nulls {
                    let mut nr =
                        StdRng::seed_from_u64(seed ^ 0x9e37_79b9 ^ ((k as u64) << 32) ^ bound);
                    runs.push((
                        format!("null1_{k}"),
                        random_base(&c.ec, base.len(), &mut nr),
                    ));
                }
                for (kind, fb) in runs {
                    let t = std::time::Instant::now();
                    let cs = census(&c.ec, c.r, &fb);
                    println!(
                        "{{\"bits\":{bt},\"shape\":\"{}\",\"p\":{p},\"a\":{},\"b\":{},\"r\":{},\"delta\":{d},\"bound\":{bound},\"base\":\"{kind}\",\"base_size\":{},\"multisets\":{},\"lambda\":{:.6e},\"dispersion\":{:.6},\"ge2\":{},\"pair_collisions\":{},\"pair_expected\":{:.4},\"triple_collisions\":{},\"triple_unshared\":{},\"triple_expected\":{:.4},\"ge3\":{},\"max_n\":{},\"ge2_shared\":{},\"ms\":{}}}",
                        c.shape, c.ec.a, c.ec.b, c.r, cs.base, cs.total, cs.lambda, cs.dispersion,
                        cs.ge2, cs.pair_collisions, cs.pair_expected, cs.triple_collisions, cs.triple_unshared, cs.triple_expected, cs.ge3, cs.max_n,
                        if cs.ge2_shared.is_nan() { "null".to_string() } else { format!("{:.4}", cs.ge2_shared) },
                        t.elapsed().as_millis()
                    );
                }
            }
        }
    }
}
