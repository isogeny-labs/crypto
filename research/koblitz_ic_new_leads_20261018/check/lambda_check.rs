//! Exhaustive check of the λ-coordinate factor-base closure (lead L1 of
//! `../README.md`) on the Koblitz curves  E_a : y² + xy = x³ + a·x² + 1,
//! a ∈ {0, 1}, over small fields F_{2^n}.
//!
//! Standalone, no dependencies:  rustc -O lambda_check.rs && ./lambda_check
//!
//! For every point P = (x, y) with x ≠ 0 the λ-coordinate is λ = x + y/x.
//! Checked, for every field and both curves:
//!
//!  C1  x(2P) = λ² + λ + a                       (all P with x ≠ 0)
//!  C2  λ(−P) = λ(P) + 1                         (negation flips λ by 1)
//!  C3  for random F₂-subspaces W ∌ 1 of dimension d, with
//!        A  = {P : λ(P) ∈ W}
//!        B  = {P : x(2P) ∈ W'},  W' = {w² + w + a : w ∈ W}
//!      B = A ⊔ (−A)  (disjoint union) and |W'| = |W|.
//!
//! C3 is the closure: a λ-subspace factor base is one sign from each ± pair
//! of the halved preimage of the affine x-subspace base W' = L(W) + a, with
//! L(w) = w² + w F₂-linear and injective on W. Point arithmetic is affine
//! Weierstrass, independent of the λ formula being tested.

const POLYS: &[(u32, u64)] = &[
    (11, (1 << 11) | (1 << 2) | 1),
    (13, (1 << 13) | (1 << 4) | (1 << 3) | (1 << 1) | 1),
    (17, (1 << 17) | (1 << 3) | 1),
    (19, (1 << 19) | (1 << 5) | (1 << 2) | (1 << 1) | 1),
];

struct F {
    n: u32,
    m: u64,
}

impl F {
    fn mul(&self, mut a: u64, mut b: u64) -> u64 {
        let mut r = 0u64;
        while b != 0 {
            if b & 1 == 1 {
                r ^= a;
            }
            b >>= 1;
            a <<= 1;
            if a >> self.n & 1 == 1 {
                a ^= self.m;
            }
        }
        r
    }
    fn sq(&self, a: u64) -> u64 {
        self.mul(a, a)
    }
    fn pow(&self, mut a: u64, mut e: u64) -> u64 {
        let mut r = 1u64;
        while e != 0 {
            if e & 1 == 1 {
                r = self.mul(r, a);
            }
            a = self.sq(a);
            e >>= 1;
        }
        r
    }
    fn inv(&self, a: u64) -> u64 {
        self.pow(a, (1u64 << self.n) - 2)
    }
    fn tr(&self, a: u64) -> u64 {
        let (mut t, mut x) = (0u64, a);
        for _ in 0..self.n {
            t ^= x;
            x = self.sq(x);
        }
        t
    }
    /// Half-trace (n odd): z with z² + z = c when Tr(c) = 0.
    fn half_trace(&self, c: u64) -> u64 {
        let (mut h, mut x) = (0u64, c);
        for i in 0..self.n {
            if i % 2 == 0 {
                h ^= x;
            }
            x = self.sq(x);
        }
        h
    }
}

type P = Option<(u64, u64)>; // None = point at infinity

fn neg(p: P) -> P {
    p.map(|(x, y)| (x, y ^ x))
}

fn add(f: &F, a: u64, p: P, q: P) -> P {
    let (x1, y1) = match p {
        None => return q,
        Some(v) => v,
    };
    let (x2, y2) = match q {
        None => return p,
        Some(v) => v,
    };
    if x1 == x2 {
        if y1 ^ y2 == x1 || x1 == 0 {
            return None; // P = −Q, or 2-torsion doubled
        }
        // tangent doubling (used only inside double_indep for coincidences)
        let s = x1 ^ f.mul(y1, f.inv(x1));
        let x3 = f.sq(s) ^ s ^ a;
        let y3 = f.sq(x1) ^ f.mul(s ^ 1, x3);
        return Some((x3, y3));
    }
    let s = f.mul(y1 ^ y2, f.inv(x1 ^ x2));
    let x3 = f.sq(s) ^ s ^ x1 ^ x2 ^ a;
    let y3 = f.mul(s, x1 ^ x3) ^ x3 ^ y1;
    Some((x3, y3))
}

/// Doubling by the chord-free formula 2P = P + P computed as (P + Q) − Q with
/// Q a fixed auxiliary point, so C1 is not checked against its own formula.
fn double_indep(f: &F, a: u64, p: (u64, u64), aux: (u64, u64)) -> P {
    let s1 = add(f, a, Some(p), Some(aux));
    let s2 = add(f, a, s1, Some(p));
    add(f, a, s2, neg(Some(aux)))
}

fn lam(f: &F, x: u64, y: u64) -> u64 {
    x ^ f.mul(y, f.inv(x))
}

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        // splitmix64
        self.0 = self.0.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }
}

fn span(basis: &[u64]) -> Vec<u64> {
    let mut v = vec![0u64];
    for &b in basis {
        let w: Vec<u64> = v.iter().map(|&x| x ^ b).collect();
        v.extend(w);
    }
    v
}

fn main() {
    let seed: u64 = 20261018;
    let mut all_ok = true;
    println!("field  a  #E        C1(x(2P)=l^2+l+a)  C2(l(-P)=l(P)+1)  C3 trials  dims  |A| range         B=A+(-A)  |W'|=|W|");
    for &(n, m) in POLYS {
        let f = F { n, m };
        let q = 1u64 << n;
        for a in 0..2u64 {
            // enumerate points with x != 0 via y = x z, z^2 + z = x + a + 1/x^2
            let mut pts: Vec<(u64, u64)> = Vec::new();
            for x in 1..q {
                let c = x ^ a ^ f.sq(f.inv(x));
                if f.tr(c) & 1 == 0 {
                    let z = f.half_trace(c);
                    let y = f.mul(x, z);
                    pts.push((x, y));
                    pts.push((x, y ^ x));
                }
            }
            let order = pts.len() as u64 + 2; // + (0,1) + infinity
            // auxiliary point for the independent doubling
            let aux = pts[pts.len() / 3];
            let (mut c1, mut c2) = (true, true);
            let mut dbl: std::collections::HashMap<(u64, u64), Option<u64>> =
                std::collections::HashMap::with_capacity(pts.len());
            for &(x, y) in &pts {
                let l = lam(&f, x, y);
                let d = double_indep(&f, a, (x, y), aux);
                let dx = d.map(|v| v.0);
                // 2P = O never happens for x != 0 (only O and (0,1) are 2-torsion)
                if dx != Some(f.sq(l) ^ l ^ a) {
                    c1 = false;
                }
                if lam(&f, x, y ^ x) != l ^ 1 {
                    c2 = false;
                }
                dbl.insert((x, y), dx);
            }
            let mut rng = Rng(seed ^ ((n as u64) << 8) ^ a);
            let (mut c3, mut c3w) = (true, true);
            let (mut amin, mut amax) = (usize::MAX, 0usize);
            let trials = 12;
            let dims = [3usize, (n as usize) / 2, (n as usize) - 3];
            for t in 0..trials {
                let d = dims[t % dims.len()];
                // random basis, rejecting dependent sets and any span containing 1
                let w = loop {
                    let b: Vec<u64> = (0..d).map(|_| rng.next() & (q - 1)).collect();
                    let s = span(&b);
                    let mut u = s.clone();
                    u.sort_unstable();
                    u.dedup();
                    if u.len() == 1 << d && !u.contains(&1) {
                        break u;
                    }
                };
                let wset: std::collections::HashSet<u64> = w.iter().copied().collect();
                let wp: std::collections::HashSet<u64> =
                    w.iter().map(|&v| f.sq(v) ^ v ^ a).collect();
                if wp.len() != w.len() {
                    c3w = false;
                }
                let aset: std::collections::HashSet<(u64, u64)> = pts
                    .iter()
                    .copied()
                    .filter(|&(x, y)| wset.contains(&lam(&f, x, y)))
                    .collect();
                let bset: std::collections::HashSet<(u64, u64)> = pts
                    .iter()
                    .copied()
                    .filter(|p| matches!(dbl[p], Some(v) if wp.contains(&v)))
                    .collect();
                let nega: std::collections::HashSet<(u64, u64)> =
                    aset.iter().map(|&(x, y)| (x, y ^ x)).collect();
                let disjoint = aset.is_disjoint(&nega);
                let union: std::collections::HashSet<(u64, u64)> =
                    aset.union(&nega).copied().collect();
                if !(disjoint && union == bset) {
                    c3 = false;
                }
                amin = amin.min(aset.len());
                amax = amax.max(aset.len());
            }
            let ok = c1 && c2 && c3 && c3w;
            all_ok &= ok;
            println!(
                "2^{:<3}  {}  {:<8}  {:<17}  {:<16}  {:<9}  {:?}  [{}, {}]  {:<14}  {:<8}  {}",
                n,
                a,
                order,
                c1,
                c2,
                trials,
                dims,
                amin,
                amax,
                c3,
                c3w,
                if ok { "PASS" } else { "FAIL" }
            );
        }
    }
    println!("overall: {}", if all_ok { "PASS" } else { "FAIL" });
    if !all_ok {
        std::process::exit(1);
    }
}
