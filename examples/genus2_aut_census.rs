//! Exact census of genus-2 curves over small prime fields: which isomorphism
//! classes have an even L-polynomial (`a_1 = 0`, the Weil-restriction shape),
//! whether the Jacobian splits over `F_p`, and the automorphism group over `F_p`.
//!
//! Addendum H5 (`research/weil_descent_angles_20261018/PROTOCOL.md`).
//!
//! A curve is `y² = f(X, Z)`, `f` a squarefree binary sextic form. Isomorphisms
//! over `F_p` act as `f ↦ e²·f∘M` with `M ∈ PGL_2(F_p)` (a fixed representative
//! per class) and `e ∈ F_p^*`. That group `G` has order `(p³ − p)(p − 1)`, and
//! `|Aut(C)| = |G| / |orbit(f)|` counts the hyperelliptic involution.
//! The L-polynomial `1 + a_1 T + a_2 T² + p a_1 T³ + p² T⁴` comes from `#C(F_p)`
//! and `#C(F_{p²})`. With `a_1 = 0` the Jacobian is isogenous over `F_p` to
//! a product of elliptic curves iff `2p − a_2` is a square (Tate). Otherwise it
//! is simple over `F_p`, with `L(T) = L_E(T²)` for a trace `t = −a_2` over
//! `F_{p²}`.
//!
//! std-only; `rustc --edition 2021 -O examples/genus2_aut_census.rs`.
//! Usage: `genus2_aut_census <out-dir> [p ...]` (default 5 7 11 13).
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
use std::fmt::Write as _;
use std::path::Path;

fn pw(b: u64, e: u32, p: u64) -> u64 {
    let mut r = 1;
    for _ in 0..e {
        r = r * b % p;
    }
    r
}

fn inv(a: u64, p: u64) -> u64 {
    pw(a, (p - 2) as u32, p)
}

/// χ over F_p
fn chi(a: u64, p: u64) -> i64 {
    if a == 0 {
        0
    } else if pw(a, ((p - 1) / 2) as u32, p) == 1 {
        1
    } else {
        -1
    }
}

/// F_{p²} = F_p[s]/(s² − κ): (a, b) = a + b s
#[derive(Clone, Copy)]
struct F2 {
    p: u64,
    k: u64,
}

impl F2 {
    fn mul(&self, x: (u64, u64), y: (u64, u64)) -> (u64, u64) {
        let p = self.p;
        (
            (x.0 * y.0 + self.k * (x.1 * y.1 % p)) % p,
            (x.0 * y.1 + x.1 * y.0) % p,
        )
    }
    fn add(&self, x: (u64, u64), y: (u64, u64)) -> (u64, u64) {
        ((x.0 + y.0) % self.p, (x.1 + y.1) % self.p)
    }
    /// χ over F_{p²}: the norm's χ over F_p
    fn chi(&self, x: (u64, u64)) -> i64 {
        let p = self.p;
        let n = (x.0 * x.0 % p + p - self.k * (x.1 * x.1 % p) % p) % p;
        chi(n, p)
    }
}

fn binom(n: u64, k: u64) -> u64 {
    (1..=k).fold(1, |a, i| a * (n + 1 - i) / i)
}

/// 7×7 matrix T with (f∘M)_j = Σ_i T[j][i] f_i for f(X, Z) = Σ f_i X^i Z^{6−i},
/// M: (X, Z) ↦ (aX + bZ, cX + dZ)
fn transform(a: u64, b: u64, c: u64, d: u64, p: u64) -> [[u64; 7]; 7] {
    let mut t = [[0u64; 7]; 7];
    for i in 0..7u64 {
        // (aX + bZ)^i (cX + dZ)^{6−i}: coefficient of X^j Z^{6−j}
        for u in 0..=i {
            // from (aX+bZ)^i: X^u Z^{i−u}
            let c1 = binom(i, u) % p * pw(a, u as u32, p) % p * pw(b, (i - u) as u32, p) % p;
            for v in 0..=(6 - i) {
                let c2 =
                    binom(6 - i, v) % p * pw(c, v as u32, p) % p * pw(d, (6 - i - v) as u32, p) % p;
                let j = (u + v) as usize;
                t[j][i as usize] = (t[j][i as usize] + c1 * c2) % p;
            }
        }
    }
    t
}

fn idx(f: &[u64; 7], p: u64) -> usize {
    f.iter()
        .rev()
        .fold(0usize, |a, &c| a * p as usize + c as usize)
}

fn from_idx(mut i: usize, p: u64) -> [u64; 7] {
    let mut f = [0u64; 7];
    for c in f.iter_mut() {
        *c = (i % p as usize) as u64;
        i /= p as usize;
    }
    f
}

/// squarefree as a binary form of degree 6
fn squarefree(f: &[u64; 7], p: u64) -> bool {
    if f[6] == 0 && f[5] == 0 {
        return false; // double root at infinity
    }
    // affine polynomial squarefree: gcd(f, f') = 1
    let mut a: Vec<u64> = f.to_vec();
    while a.last() == Some(&0) {
        a.pop();
    }
    let mut b: Vec<u64> = (1..a.len()).map(|i| a[i] * i as u64 % p).collect();
    while b.last() == Some(&0) {
        b.pop();
    }
    if b.is_empty() {
        return false;
    }
    // Euclid
    while !b.is_empty() {
        let mut r = a.clone();
        let lb = inv(*b.last().unwrap(), p);
        while r.len() >= b.len() {
            let c = r.last().unwrap() * lb % p;
            let sh = r.len() - b.len();
            for (k, &bk) in b.iter().enumerate() {
                r[sh + k] = (r[sh + k] + p - c * bk % p) % p;
            }
            while r.last() == Some(&0) {
                r.pop();
            }
            if r.is_empty() {
                break;
            }
        }
        a = b;
        b = r;
    }
    a.len() == 1
}

fn counts(f: &[u64; 7], p: u64, f2: &F2) -> (i64, i64) {
    let ev = |x: u64| f.iter().rev().fold(0u64, |a, &c| (a * x + c) % p);
    let mut n1 = 0i64;
    for x in 0..p {
        n1 += 1 + chi(ev(x), p);
    }
    n1 += if f[6] != 0 { 1 + chi(f[6], p) } else { 1 };
    let mut n2 = 0i64;
    for x0 in 0..p {
        for x1 in 0..p {
            let x = (x0, x1);
            let mut v = (0u64, 0u64);
            for &c in f.iter().rev() {
                v = f2.add(f2.mul(v, x), (c, 0));
            }
            n2 += 1 + f2.chi(v);
        }
    }
    // at infinity over F_{p²}: f6 is a square there (F_p ⊂ squares), or one point if f6 = 0
    n2 += if f[6] != 0 { 2 } else { 1 };
    (n1, n2)
}

fn census(p: u64) -> (String, String) {
    let k = (2..p).find(|&k| chi(k, p) == -1).unwrap();
    let f2 = F2 { p, k };
    // PGL2 representatives: (1, b; c, d) normalized with first nonzero of (a, c) = 1
    let mut mats = Vec::new();
    for a in 0..p {
        for b in 0..p {
            for c in 0..p {
                for d in 0..p {
                    if (a * d + p * p - b * c) % p == 0 {
                        continue;
                    }
                    let first = if a != 0 { a } else { c };
                    if first != 1 {
                        continue;
                    }
                    mats.push(transform(a, b, c, d, p));
                }
            }
        }
    }
    let squares: Vec<u64> = (1..p).map(|e| e * e % p).collect();
    let gsize = (mats.len() * (p as usize - 1)) as u64;
    let total = (p as usize).pow(7);
    let mut seen = vec![false; total];
    let mut rows = String::from("p,rep,orbit,aut,n1,n2,a1,a2,even,split,trace_E\n");
    let mut hist: BTreeMap<(bool, bool, u64), u64> = BTreeMap::new();
    let mut mass = 0f64;
    for i in 0..total {
        if seen[i] {
            continue;
        }
        let f = from_idx(i, p);
        if !squarefree(&f, p) {
            seen[i] = true;
            continue;
        }
        // orbit
        let mut orbit = 0u64;
        let mut stack = Vec::new();
        for m in &mats {
            let mut g = [0u64; 7];
            for (j, gj) in g.iter_mut().enumerate() {
                *gj = (0..7).fold(0, |acc, i2| (acc + m[j][i2] * f[i2]) % p);
            }
            for &e2 in &squares {
                let h: [u64; 7] = std::array::from_fn(|j| g[j] * e2 % p);
                stack.push(idx(&h, p));
            }
        }
        // the multiset over G: count distinct images; |orbit| = distinct
        stack.sort_unstable();
        stack.dedup();
        for &j in &stack {
            if !seen[j] {
                seen[j] = true;
                orbit += 1;
            }
        }
        // squares list has (p−1)/2 entries but e ranges over F_p^*: ±e give the same e²
        let aut = gsize / orbit;
        let (n1, n2) = counts(&f, p, &f2);
        let a1 = n1 - (p as i64) - 1;
        let a2 = (n2 - (p * p) as i64 - 1 + a1 * a1) / 2;
        let even = a1 == 0;
        let s = 2 * p as i64 - a2;
        let split = even && s >= 0 && ((s as f64).sqrt().round() as i64).pow(2) == s;
        mass += 1.0 / aut as f64;
        *hist.entry((even, split, aut)).or_default() += 1;
        let _ = writeln!(
            rows,
            "{p},{},{orbit},{aut},{n1},{n2},{a1},{a2},{even},{split},{}",
            f.iter()
                .map(|c| c.to_string())
                .collect::<Vec<_>>()
                .join(" "),
            if even { (-a2).to_string() } else { "-".into() }
        );
    }
    let mut sum = format!(
        "H5 p={p}: genus-2 mass sum 1/|Aut| = {mass:.4} (expected p^3 = {})\n",
        p * p * p
    );
    for ((even, split, aut), n) in &hist {
        if *even {
            let _ = writeln!(
                sum,
                "H5 p={p}: a1=0 {} |Aut|={aut}: {n} classes",
                if *split {
                    "split over F_p"
                } else {
                    "SIMPLE over F_p (Res-shaped)"
                }
            );
        }
    }
    let nonsplit_extra: u64 = hist
        .iter()
        .filter(|((e, s, a), _)| *e && !*s && *a > 2)
        .map(|(_, n)| n)
        .sum();
    let _ = writeln!(
        sum,
        "H5 p={p}: simple Res-shaped classes with |Aut| > 2: {nonsplit_extra}"
    );
    (rows, sum)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let out = Path::new(
        args.get(1)
            .expect("usage: genus2_aut_census <out-dir> [p ...]"),
    );
    std::fs::create_dir_all(out).unwrap();
    let ps: Vec<u64> = if args.len() > 2 {
        args[2..].iter().map(|s| s.parse().unwrap()).collect()
    } else {
        vec![5, 7, 11, 13]
    };
    let mut csv = String::new();
    let mut summary = String::new();
    for p in ps {
        let t0 = std::time::Instant::now();
        let (rows, sum) = census(p);
        if csv.is_empty() {
            csv.push_str(&rows);
        } else {
            csv.push_str(rows.split_once('\n').unwrap().1);
        }
        eprint!("{sum}");
        eprintln!("p={p} done in {:.1}s", t0.elapsed().as_secs_f64());
        summary.push_str(&sum);
    }
    std::fs::write(out.join("classes.csv"), csv).unwrap();
    std::fs::write(out.join("summary.txt"), summary).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transform_identity_and_squarefree() {
        let p = 7;
        let t = transform(1, 0, 0, 1, p);
        for (j, row) in t.iter().enumerate() {
            for (i, &v) in row.iter().enumerate() {
                assert_eq!(v, u64::from(i == j));
            }
        }
        // x^5 - x (squarefree, root at infinity simple), x^6 (not)
        assert!(squarefree(&[0, p - 1, 0, 0, 0, 1, 0], p));
        assert!(!squarefree(&[0, 0, 0, 0, 0, 0, 1], p));
    }
}
