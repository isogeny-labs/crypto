//! Exact decomposition coverage of PKM factor bases over prime fields.
//!
//! Round 3, item 2 (`research/weil_descent_angles_20261018/PROTOCOL.md`,
//! Addendum A, Part P). For a prime-order `E/F_p` and a factor base
//! `F = {P : x(P) ∈ V}` with `V` the `2^t` roots of a degree-2 tower
//! (Kummer `x ↦ x^2`, Dickson `x ↦ x^2 − 2`, split or torus), compute
//! exactly the share `π` of the group reached by sums of `m` factor-base
//! points, the mean number of decompositions, and the same quantities for a
//! random `V` of equal size (null control) and for an interval
//! `{±jG}` (positive control).
//!
//! std-only; `rustc --edition 2021 -O examples/pkm_coverage.rs`.
//! Usage: `pkm_coverage <out-dir>`; env `PKM_THREADS`, `PKM_SCALE` (0..1, shrinks the ranges for smoke runs).
#![allow(dead_code)]
#![allow(
    clippy::manual_is_multiple_of,
    clippy::manual_div_ceil,
    clippy::too_many_arguments,
    clippy::type_complexity,
    clippy::needless_range_loop,
    clippy::unusual_byte_groupings,
    clippy::write_with_newline
)]

use std::collections::BTreeMap;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::time::Instant;

fn mulmod(a: u64, b: u64, p: u64) -> u64 {
    ((a as u128 * b as u128) % p as u128) as u64
}

fn powmod(mut b: u64, mut e: u64, p: u64) -> u64 {
    let mut r = 1u64;
    b %= p;
    while e > 0 {
        if e & 1 == 1 {
            r = mulmod(r, b, p);
        }
        b = mulmod(b, b, p);
        e >>= 1;
    }
    r
}

fn inv(a: u64, p: u64) -> u64 {
    powmod(a, p - 2, p)
}

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    for &q in &[2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        if n % q == 0 {
            return n == q;
        }
    }
    let (mut d, mut s) = (n - 1, 0);
    while d % 2 == 0 {
        d /= 2;
        s += 1;
    }
    'w: for &a in &[2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        let mut x = powmod(a, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }
        for _ in 0..s - 1 {
            x = mulmod(x, x, n);
            if x == n - 1 {
                continue 'w;
            }
        }
        return false;
    }
    true
}

fn xorshift(s: &mut u64) -> u64 {
    *s ^= *s << 13;
    *s ^= *s >> 7;
    *s ^= *s << 17;
    *s
}

/// Square root mod p (Tonelli–Shanks); `a` must be a nonzero square.
fn sqrt_mod(a: u64, p: u64) -> u64 {
    if p % 4 == 3 {
        return powmod(a, (p + 1) / 4, p);
    }
    let (mut q, mut s) = (p - 1, 0u32);
    while q % 2 == 0 {
        q /= 2;
        s += 1;
    }
    let mut z = 2u64;
    while powmod(z, (p - 1) / 2, p) != p - 1 {
        z += 1;
    }
    let mut m = s;
    let mut c = powmod(z, q, p);
    let mut t = powmod(a, q, p);
    let mut r = powmod(a, q.div_ceil(2), p);
    while t != 1 {
        let mut i = 0u32;
        let mut tt = t;
        while tt != 1 {
            tt = mulmod(tt, tt, p);
            i += 1;
        }
        let b = powmod(c, 1u64 << (m - i - 1), p);
        m = i;
        c = mulmod(b, b, p);
        t = mulmod(t, c, p);
        r = mulmod(r, b, p);
    }
    r
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Pt {
    O,
    A(u64, u64),
}

struct Curve {
    p: u64,
    a: u64,
    b: u64,
}

impl Curve {
    fn rhs(&self, x: u64) -> u64 {
        let p = self.p;
        (mulmod(mulmod(x, x, p), x, p) + mulmod(self.a, x, p) + self.b) % p
    }
    fn neg(&self, q: Pt) -> Pt {
        match q {
            Pt::O => Pt::O,
            Pt::A(x, y) => Pt::A(x, (self.p - y) % self.p),
        }
    }
    fn add(&self, u: Pt, v: Pt) -> Pt {
        let p = self.p;
        match (u, v) {
            (Pt::O, w) | (w, Pt::O) => w,
            (Pt::A(x1, y1), Pt::A(x2, y2)) => {
                let lam = if x1 == x2 {
                    if (y1 + y2) % p == 0 {
                        return Pt::O;
                    }
                    let num = (3 * mulmod(x1, x1, p) + self.a) % p;
                    mulmod(num, inv(2 * y1 % p, p), p)
                } else {
                    mulmod((y2 + p - y1) % p, inv((x2 + p - x1) % p, p), p)
                };
                let x3 = (mulmod(lam, lam, p) + 2 * p - x1 - x2) % p;
                let y3 = (mulmod(lam, (x1 + p - x3) % p, p) + p - y1) % p;
                Pt::A(x3, y3)
            }
        }
    }
    fn idx(&self, q: Pt) -> usize {
        match q {
            Pt::O => 0,
            Pt::A(x, y) => 1 + 2 * x as usize + (y & 1) as usize,
        }
    }
}

/// Quadratic character table: chi[v] = 1 if square, -1 non-square, 0 if v = 0.
fn chi_table(p: u64) -> Vec<i8> {
    let mut t = vec![-1i8; p as usize];
    t[0] = 0;
    for x in 1..p {
        t[mulmod(x, x, p) as usize] = 1;
    }
    t
}

fn count_points(c: &Curve, chi: &[i8]) -> u64 {
    let mut n: i64 = c.p as i64 + 1;
    for x in 0..c.p {
        n += chi[c.rhs(x) as usize] as i64;
    }
    n as u64
}

fn random_prime_order_curve(p: u64, chi: &[i8], rng: &mut u64) -> (Curve, u64) {
    loop {
        let a = xorshift(rng) % p;
        let b = 1 + xorshift(rng) % (p - 1);
        if (4 * mulmod(mulmod(a, a, p), a, p) + 27 * mulmod(b, b, p)) % p == 0 {
            continue;
        }
        let c = Curve { p, a, b };
        let n = count_points(&c, chi);
        if is_prime(n) {
            return (c, n);
        }
    }
}

/// F_{p^2} = F_p[s]/(s^2 - nr) for the torus Dickson family.
fn f2mul(a: (u64, u64), b: (u64, u64), nr: u64, p: u64) -> (u64, u64) {
    (
        (mulmod(a.0, b.0, p) + mulmod(mulmod(a.1, b.1, p), nr, p)) % p,
        (mulmod(a.0, b.1, p) + mulmod(a.1, b.0, p)) % p,
    )
}

fn f2pow(mut a: (u64, u64), mut e: u64, nr: u64, p: u64) -> (u64, u64) {
    let mut r = (1u64, 0u64);
    while e > 0 {
        if e & 1 == 1 {
            r = f2mul(r, a, nr, p);
        }
        a = f2mul(a, a, nr, p);
        e >>= 1;
    }
    r
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Family {
    Kummer,
    DicksonSplit,
    DicksonTorus,
}

impl Family {
    fn name(self) -> &'static str {
        match self {
            Family::Kummer => "kummer",
            Family::DicksonSplit => "dickson_split",
            Family::DicksonTorus => "dickson_torus",
        }
    }
}

/// The tower fibre V (2^t elements of F_p) for a family, or None if the
/// field does not support it.
fn tower_fibre(fam: Family, p: u64, t: u32, rng: &mut u64) -> Option<Vec<u64>> {
    let n = 1u64 << t;
    match fam {
        Family::Kummer | Family::DicksonSplit => {
            if (p - 1) % n != 0 {
                return None;
            }
            // a generator of mu_{2^t}
            let zeta = loop {
                let g = 2 + xorshift(rng) % (p - 3);
                let z = powmod(g, (p - 1) / n, p);
                if powmod(z, n / 2, p) == p - 1 {
                    break z;
                }
            };
            let w0 = 2 + xorshift(rng) % (p - 3); // coset representative
            let mut coset = Vec::with_capacity(n as usize);
            let mut z = w0;
            for _ in 0..n {
                coset.push(z);
                z = mulmod(z, zeta, p);
            }
            let mut v: Vec<u64> = match fam {
                Family::Kummer => coset,
                _ => coset.iter().map(|&z| (z + inv(z, p)) % p).collect(),
            };
            v.sort_unstable();
            v.dedup();
            if v.len() as u64 != n {
                return None;
            }
            Some(v)
        }
        Family::DicksonTorus => {
            if (p + 1) % n != 0 {
                return None;
            }
            let nr = loop {
                let c = 2 + xorshift(rng) % (p - 3);
                if powmod(c, (p - 1) / 2, p) == p - 1 {
                    break c;
                }
            };
            // element of the norm-1 torus of order exactly n
            let zeta = loop {
                let g = (xorshift(rng) % p, 1 + xorshift(rng) % (p - 1));
                let z = f2pow(g, (p - 1) * ((p + 1) / n), nr, p);
                if f2pow(z, n / 2, nr, p) == (p - 1, 0) {
                    break z;
                }
            };
            let w0 = loop {
                let g = (xorshift(rng) % p, 1 + xorshift(rng) % (p - 1));
                let z = f2pow(g, p - 1, nr, p); // norm 1
                if z != (1, 0) {
                    break z;
                }
            };
            let mut v = Vec::with_capacity(n as usize);
            let mut z = w0;
            for _ in 0..n {
                v.push(2 * z.0 % p); // z + z^p = trace
                z = f2mul(z, zeta, nr, p);
            }
            v.sort_unstable();
            v.dedup();
            if v.len() as u64 != n {
                return None;
            }
            Some(v)
        }
    }
}

fn points_over(c: &Curve, xs: &[u64], chi: &[i8]) -> Vec<Pt> {
    let mut f = Vec::new();
    for &x in xs {
        let v = c.rhs(x);
        if v == 0 {
            f.push(Pt::A(x, 0));
        } else if chi[v as usize] == 1 {
            let y = sqrt_mod(v, c.p);
            f.push(Pt::A(x, y));
            f.push(Pt::A(x, c.p - y));
        }
    }
    f
}

/// Coverage of m-fold multiset sums of F in a group of order r:
/// (distinct sums, multisets enumerated).
fn coverage(c: &Curve, f: &[Pt], m: usize) -> (u64, u64) {
    let mut seen = vec![0u64; (2 * c.p as usize + 2).div_ceil(64)];
    let mut distinct = 0u64;
    let mut total = 0u64;
    let mut mark = |q: Pt, seen: &mut Vec<u64>| {
        let i = c.idx(q);
        let (w, bit) = (i / 64, i % 64);
        if seen[w] >> bit & 1 == 0 {
            seen[w] |= 1 << bit;
            distinct += 1;
        }
    };
    match m {
        2 => {
            for i in 0..f.len() {
                for j in i..f.len() {
                    mark(c.add(f[i], f[j]), &mut seen);
                    total += 1;
                }
            }
        }
        3 => {
            for i in 0..f.len() {
                for j in i..f.len() {
                    let s = c.add(f[i], f[j]);
                    for k in j..f.len() {
                        mark(c.add(s, f[k]), &mut seen);
                        total += 1;
                    }
                }
            }
        }
        _ => unimplemented!(),
    }
    (distinct, total)
}

#[derive(Clone, Debug)]
struct Row {
    family: Family,
    m: usize,
    p: u64,
    r: u64,
    t: u32,
    t_offset: i32,
    arm: &'static str,
    v: usize,
    f: usize,
    heuristic: f64,
    lambda: f64,
    poisson: f64,
    distinct: u64,
    multisets: u64,
    pi: f64,
}

fn choose_primes(fam: Family, m: usize, scale: f64, rng: &mut u64) -> Vec<(u64, u32)> {
    // r between 2^lo and 2^hi; t0 = round(log2((m! r)^{1/m}))
    let (lo, hi) = if m == 2 { (16.0, 22.0) } else { (14.0, 20.0) };
    let hi = lo + (hi - lo) * scale;
    let count = ((24.0 * scale).ceil() as usize).max(3);
    let mut out: Vec<(u64, u32)> = Vec::new();
    let mut tries = 0;
    while out.len() < count && tries < 1_000_000 {
        tries += 1;
        let bits = lo + (hi - lo) * (xorshift(rng) % 10_000) as f64 / 10_000.0;
        let target = 2f64.powf(bits);
        let mf = if m == 2 { 2.0 } else { 6.0 };
        let t0 = ((mf * target).powf(1.0 / m as f64)).log2().round() as u32;
        let n = 1u64 << (t0 + 1); // room for t0 + 1
        let k = (target / n as f64).max(2.0) as u64;
        let p = match fam {
            Family::DicksonTorus => k * n - 1,
            _ => k * n + 1,
        };
        if p < 1000 || !is_prime(p) || out.iter().any(|&(q, _)| q == p) {
            continue;
        }
        out.push((p, t0));
    }
    out.sort_unstable();
    out
}

fn run_prime(fam: Family, m: usize, p: u64, t0: u32, seed: u64) -> Vec<Row> {
    let mut rng = seed | 1;
    let chi = chi_table(p);
    let (c, r) = random_prime_order_curve(p, &chi, &mut rng);
    let g = loop {
        let x = xorshift(&mut rng) % p;
        let v = c.rhs(x);
        if chi[v as usize] == 1 {
            break Pt::A(x, sqrt_mod(v, p));
        }
    };
    let mfact = if m == 2 { 2.0 } else { 6.0 };
    let mut rows = Vec::new();
    for off in [-1i32, 0, 1] {
        let t = (t0 as i32 + off) as u32;
        let Some(v) = tower_fibre(fam, p, t, &mut rng) else {
            continue;
        };
        let fv = points_over(&c, &v, &chi);
        // null: random x-set of the same size
        let mut vx: Vec<u64> = Vec::new();
        while vx.len() < v.len() {
            let x = xorshift(&mut rng) % p;
            if !vx.contains(&x) {
                vx.push(x);
            }
        }
        let fn_ = points_over(&c, &vx, &chi);
        // positive: interval ±jG of the same size as F
        let mut fp = Vec::new();
        let mut q = g;
        for _ in 0..fv.len() / 2 {
            fp.push(q);
            fp.push(c.neg(q));
            q = c.add(q, g);
        }
        for (arm, f, vsize) in [
            (fam.name(), &fv, v.len()),
            ("null", &fn_, vx.len()),
            ("positive", &fp, v.len()),
        ] {
            let (distinct, total) = coverage(&c, f, m);
            let heuristic = (vsize as f64).powi(m as i32) / (mfact * r as f64);
            let lambda = total as f64 / r as f64;
            rows.push(Row {
                family: fam,
                m,
                p,
                r,
                t,
                t_offset: off,
                arm,
                v: vsize,
                f: f.len(),
                heuristic,
                lambda,
                poisson: 1.0 - (-lambda).exp(),
                distinct,
                multisets: total,
                pi: distinct as f64 / r as f64,
            });
        }
    }
    rows
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let out = Path::new(args.get(1).expect("usage: pkm_coverage <out-dir>"));
    std::fs::create_dir_all(out).unwrap();
    let threads: usize = std::env::var("PKM_THREADS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(4);
    let scale: f64 = std::env::var("PKM_SCALE")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(1.0);
    let mut log = File::create(out.join("tool_log.txt")).unwrap();
    let t0 = Instant::now();
    let mut rng = 0x5eed_0f_9a11_c0de_u64;
    let mut jobs: Vec<(Family, usize, u64, u32, u64)> = Vec::new();
    for fam in [Family::Kummer, Family::DicksonSplit, Family::DicksonTorus] {
        for m in [2usize, 3] {
            for (p, t0p) in choose_primes(fam, m, scale, &mut rng) {
                jobs.push((fam, m, p, t0p, xorshift(&mut rng)));
            }
        }
    }
    let _ = writeln!(log, "jobs: {}", jobs.len());
    let jobs = std::sync::Mutex::new(jobs.into_iter().enumerate().collect::<Vec<_>>());
    let rows = std::sync::Mutex::new(Vec::<(usize, Vec<Row>)>::new());
    std::thread::scope(|s| {
        for _ in 0..threads {
            s.spawn(|| loop {
                let job = jobs.lock().unwrap().pop();
                let Some((i, (fam, m, p, t0p, seed))) = job else {
                    break;
                };
                let r = run_prime(fam, m, p, t0p, seed);
                rows.lock().unwrap().push((i, r));
            });
        }
    });
    let mut rows = rows.into_inner().unwrap();
    rows.sort_by_key(|(i, _)| *i);
    let rows: Vec<Row> = rows.into_iter().flat_map(|(_, r)| r).collect();
    let mut f = File::create(out.join("pkm_coverage.csv")).unwrap();
    writeln!(
        f,
        "family,m,p,r,log2r,t,t_offset,arm,V,F,heuristic,lambda,poisson,distinct,multisets,pi"
    )
    .unwrap();
    for r in &rows {
        writeln!(
            f,
            "{},{},{},{},{:.3},{},{},{},{},{},{:.6},{:.6},{:.6},{},{},{:.6}",
            r.family.name(),
            r.m,
            r.p,
            r.r,
            (r.r as f64).log2(),
            r.t,
            r.t_offset,
            r.arm,
            r.v,
            r.f,
            r.heuristic,
            r.lambda,
            r.poisson,
            r.distinct,
            r.multisets,
            r.pi
        )
        .unwrap();
    }
    // Summary, paired per prime by (family, m, t_offset). Primary statistic
    // (Amendment B): the residual pi - poisson(lambda) of each arm, so that
    // the factor-base size drawn by each arm is normalised out.
    type Cell = (Option<(f64, f64)>, Option<(f64, f64)>, Option<(f64, f64)>);
    let mut idx: BTreeMap<(Family, usize, u64, i32), Cell> = BTreeMap::new();
    for r in &rows {
        let e = idx
            .entry((r.family, r.m, r.p, r.t_offset))
            .or_insert((None, None, None));
        let v = Some((r.pi, r.pi - r.poisson));
        match r.arm {
            "null" => e.1 = v,
            "positive" => e.2 = v,
            _ => e.0 = v,
        }
    }
    let mut sum: BTreeMap<(Family, usize, i32), Vec<[f64; 6]>> = BTreeMap::new();
    for ((fam, m, _p, off), (a, n, pos)) in idx {
        if let (Some(a), Some(n), Some(pos)) = (a, n, pos) {
            sum.entry((fam, m, off))
                .or_default()
                .push([a.0, n.0, pos.0, a.1, n.1, pos.1]);
        }
    }
    let mut checks = String::new();
    let mut sf = File::create(out.join("pkm_summary.csv")).unwrap();
    writeln!(
        sf,
        "family,m,t_offset,primes,mean_pi_family,mean_pi_null,mean_pi_positive,resid_family,resid_null,resid_positive,resid_diff,sd_resid_diff,z_resid_diff"
    )
    .unwrap();
    for ((fam, m, off), v) in &sum {
        let k = v.len() as f64;
        let mean = |i: usize| v.iter().map(|x| x[i]).sum::<f64>() / k;
        let md = v.iter().map(|x| x[3] - x[4]).sum::<f64>() / k;
        let sd =
            (v.iter().map(|x| (x[3] - x[4] - md).powi(2)).sum::<f64>() / (k - 1.0).max(1.0)).sqrt();
        let z = if sd > 0.0 { md / (sd / k.sqrt()) } else { 0.0 };
        writeln!(
            sf,
            "{},{},{},{},{:.6},{:.6},{:.6},{:.6},{:.6},{:.6},{:.6},{:.6},{:.2}",
            fam.name(),
            m,
            off,
            v.len(),
            mean(0),
            mean(1),
            mean(2),
            mean(3),
            mean(4),
            mean(5),
            md,
            sd,
            z
        )
        .unwrap();
        checks.push_str(&format!(
            "P {} m={} t_off={:+}: primes {} pi family {:.4} null {:.4} positive {:.4} | resid family {:+.5} null {:+.5} positive {:+.4} | paired resid diff {:+.5} (z {:.2})\n",
            fam.name(),
            m,
            off,
            v.len(),
            mean(0),
            mean(1),
            mean(2),
            mean(3),
            mean(4),
            mean(5),
            md,
            z
        ));
    }
    std::fs::write(out.join("checks.txt"), &checks).unwrap();
    let _ = write!(log, "{checks}total {:.1}s\n", t0.elapsed().as_secs_f64());
    eprint!("{checks}");
    eprintln!("done in {:.1}s", t0.elapsed().as_secs_f64());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn group_law_and_order() {
        let p = 1009u64;
        let chi = chi_table(p);
        let mut rng = 5u64;
        let (c, r) = random_prime_order_curve(p, &chi, &mut rng);
        // r * G = O for a random point
        let x = (0..p).find(|&x| chi[c.rhs(x) as usize] == 1).unwrap();
        let g = Pt::A(x, sqrt_mod(c.rhs(x), p));
        let mut acc = Pt::O;
        for _ in 0..r {
            acc = c.add(acc, g);
        }
        assert_eq!(acc, Pt::O);
    }

    #[test]
    fn fibres_have_tower_property() {
        let mut rng = 11u64;
        // Kummer: v^(2^t) is constant on V
        let p = 7681u64; // 7681 = 15 * 2^9 + 1
        let v = tower_fibre(Family::Kummer, p, 5, &mut rng).unwrap();
        let c0 = powmod(v[0], 32, p);
        assert!(v.iter().all(|&x| powmod(x, 32, p) == c0));
        // Dickson split: D_{2^t}(x) constant, D_2(x) = x^2 - 2
        let d = |mut x: u64, t: u32| {
            for _ in 0..t {
                x = (mulmod(x, x, p) + p - 2) % p;
            }
            x
        };
        let v = tower_fibre(Family::DicksonSplit, p, 5, &mut rng).unwrap();
        assert!(v.iter().all(|&x| d(x, 5) == d(v[0], 5)));
        // torus: p = 8191 = 2^13 - 1, 2^t | p + 1
        let p2 = 8191u64;
        let d2 = |mut x: u64, t: u32| {
            for _ in 0..t {
                x = (mulmod(x, x, p2) + p2 - 2) % p2;
            }
            x
        };
        let v = tower_fibre(Family::DicksonTorus, p2, 5, &mut rng).unwrap();
        assert_eq!(v.len(), 32);
        assert!(v.iter().all(|&x| d2(x, 5) == d2(v[0], 5)));
    }

    #[test]
    fn positive_control_is_detected() {
        let p = 4099u64;
        let chi = chi_table(p);
        let mut rng = 3u64;
        let (c, _r) = random_prime_order_curve(p, &chi, &mut rng);
        let x = (1..p).find(|&x| chi[c.rhs(x) as usize] == 1).unwrap();
        let g = Pt::A(x, sqrt_mod(c.rhs(x), p));
        let mut f = Vec::new();
        let mut q = g;
        for _ in 0..30 {
            f.push(q);
            f.push(c.neg(q));
            q = c.add(q, g);
        }
        let (d, _t) = coverage(&c, &f, 2);
        assert!(d <= 121, "interval sums lie in {{jG : |j| <= 60}}: {d}");
    }
}
