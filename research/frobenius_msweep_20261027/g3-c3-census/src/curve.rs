//! Binary curve E_{a,b}: y^2 + xy = x^3 + a x^2 + b over F_{2^n}: point
//! arithmetic (for planted instances and exact identities), abscissa test and
//! the solver-free exact decomposition oracle used to label targets.

use crate::gf::{Basis, Field, VMember};
use std::collections::HashSet;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pt {
    Inf,
    Aff(u64, u64),
}

pub struct Curve<'a> {
    pub f: &'a Field,
    pub a: u64,
    pub b: u64,
}

impl<'a> Curve<'a> {
    pub fn is_abscissa(&self, x: u64) -> bool {
        if x == 0 {
            return true; // (0, sqrt(b))
        }
        let f = self.f;
        let g = x ^ self.a ^ f.mul(self.b, f.sq(f.inv(x)));
        f.trace(g) == 0
    }

    /// One y with (x,y) on E (the other is x+y). None if x is not an abscissa.
    pub fn lift(&self, x: u64) -> Option<u64> {
        let f = self.f;
        if x == 0 {
            return Some(f.sqrt(self.b));
        }
        if !self.is_abscissa(x) {
            return None;
        }
        let g = x ^ self.a ^ f.mul(self.b, f.sq(f.inv(x)));
        let z = f.half_trace(g); // z^2 + z = g (trace 0)
        Some(f.mul(x, z))
    }

    pub fn on_curve(&self, p: Pt) -> bool {
        match p {
            Pt::Inf => true,
            Pt::Aff(x, y) => {
                let f = self.f;
                let l = f.sq(y) ^ f.mul(x, y);
                let r = f.mul(f.sq(x), x) ^ f.mul(self.a, f.sq(x)) ^ self.b;
                l == r
            }
        }
    }

    pub fn neg(&self, p: Pt) -> Pt {
        match p {
            Pt::Inf => Pt::Inf,
            Pt::Aff(x, y) => Pt::Aff(x, x ^ y),
        }
    }

    pub fn add(&self, p: Pt, q: Pt) -> Pt {
        let f = self.f;
        match (p, q) {
            (Pt::Inf, r) | (r, Pt::Inf) => r,
            (Pt::Aff(x1, y1), Pt::Aff(x2, y2)) => {
                if x1 == x2 {
                    if y1 != y2 || x1 == 0 {
                        return Pt::Inf; // q = -p (or 2-torsion doubling)
                    }
                    let lam = x1 ^ f.mul(y1, f.inv(x1));
                    let x3 = f.sq(lam) ^ lam ^ self.a;
                    let y3 = f.sq(x1) ^ f.mul(lam ^ 1, x3);
                    Pt::Aff(x3, y3)
                } else {
                    let lam = f.mul(y1 ^ y2, f.inv(x1 ^ x2));
                    let x3 = f.sq(lam) ^ lam ^ x1 ^ x2 ^ self.a;
                    let y3 = f.mul(lam, x1 ^ x3) ^ x3 ^ y1;
                    Pt::Aff(x3, y3)
                }
            }
        }
    }
}

/// Deterministic splitmix64 stream.
pub struct Rng(pub u64);
impl Rng {
    pub fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }
    pub fn below(&mut self, k: u64) -> u64 {
        self.next() % k
    }
}

pub fn mix(a: u64, b: u64) -> u64 {
    let mut r = Rng(a ^ b.wrapping_mul(0xD6E8FEB86659FD93));
    r.next();
    r.next()
}

/// Solver-free exact oracle: does S_{m+1}(x_1..x_m, xR)=0 have a solution with
/// all x_i in V (chain semantics, X in F_{2^n}; the same zero set the SAT
/// encodings describe)?
pub struct Oracle<'a> {
    pub f: &'a Field,
    pub b: u64,
    pub nb: &'a Basis,
    pub vmem: VMember,
    pub velems: Vec<u64>,
    d2: Option<HashSet<u32>>,
}

impl<'a> Oracle<'a> {
    pub fn new(f: &'a Field, nb: &'a Basis, vbasis: &[u64], g: u64, b: u64) -> Oracle<'a> {
        let l = vbasis.len();
        let mut velems = Vec::with_capacity(1 << l);
        let mut cur = 0u64;
        velems.push(0);
        for i in 1u64..(1u64 << l) {
            cur ^= vbasis[i.trailing_zeros() as usize]; // Gray code
            velems.push(cur);
        }
        Oracle {
            f,
            b,
            nb,
            vmem: VMember::new(f.n, g),
            velems,
            d2: None,
        }
    }

    #[inline]
    pub fn in_v(&self, x: u64) -> bool {
        self.vmem.contains_coords(self.nb.coords(x))
    }

    /// Roots X of S3(p, q, X) = 0.
    fn roots_in_third(&self, p: u64, q: u64) -> Vec<u64> {
        let f = self.f;
        let bb = f.mul(p, q);
        f.solve_quadratic(f.sq(p ^ q), bb, f.sq(bb) ^ self.b)
            .unwrap_or_default()
    }

    /// Number of ORDERED pairs (x1,x2) in V^2 with S3(x1,x2,xr)=0.
    pub fn count_m2_ordered(&self, xr: u64) -> u64 {
        let mut cnt = 0;
        for &x1 in &self.velems {
            for r in self.roots_in_third(x1, xr) {
                if self.in_v(r) {
                    cnt += 1;
                }
            }
        }
        cnt
    }

    pub fn build_d2(&mut self) {
        if self.d2.is_some() {
            return;
        }
        let mut s = HashSet::new();
        let nv = self.velems.len();
        for i in 0..nv {
            for j in i..nv {
                for r in self.roots_in_third(self.velems[i], self.velems[j]) {
                    s.insert(r as u32);
                }
            }
        }
        self.d2 = Some(s);
    }

    /// m = 3: exists x1,x2,x3 in V and X with S3(x1,x2,X)=0=S3(X,x3,xr)?
    pub fn exists_m3(&mut self, xr: u64) -> bool {
        self.build_d2();
        let d2 = self.d2.as_ref().unwrap();
        for &x3 in &self.velems {
            for r in self.roots_in_third(x3, xr) {
                if d2.contains(&(r as u32)) {
                    return true;
                }
            }
        }
        false
    }

    pub fn exists(&mut self, m: usize, xr: u64) -> bool {
        match m {
            2 => self.count_m2_ordered(xr) > 0,
            3 => self.exists_m3(xr),
            _ => panic!("oracle supports m in {{2,3}}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gf::*;

    fn rand_point(c: &Curve, rng: &mut Rng) -> Pt {
        loop {
            let x = rng.below(1 << c.f.n);
            if let Some(y) = c.lift(x) {
                let y = if rng.next() & 1 == 1 { y ^ x } else { y };
                return Pt::Aff(x, y);
            }
        }
    }

    #[test]
    fn group_law_and_s3_identity_exact() {
        // S3(x1,x2,x(P1+P2)) = 0 for random points: validates the Semaev formula
        // against the group law (exact identity, both curves a=0,1, ordinary b).
        for &(n, a, b) in &[
            (7u32, 0u64, 1u64),
            (7, 1, 1),
            (13, 0, 1),
            (13, 1, 1),
            (13, 1, 2),
        ] {
            let f = Field::new(n);
            let c = Curve { f: &f, a, b };
            let mut rng = Rng(12345 + n as u64 + a);
            let mut checked = 0;
            for _ in 0..400 {
                let p = rand_point(&c, &mut rng);
                let q = rand_point(&c, &mut rng);
                assert!(c.on_curve(p) && c.on_curve(q));
                let s = c.add(p, q);
                assert!(c.on_curve(s));
                // associativity spot check
                let r = rand_point(&c, &mut rng);
                assert_eq!(c.add(c.add(p, q), r), c.add(p, c.add(q, r)));
                if let (Pt::Aff(x1, _), Pt::Aff(x2, _), Pt::Aff(x3, _)) = (p, q, s) {
                    assert_eq!(f.s3(x1, x2, x3, b), 0);
                    checked += 1;
                }
            }
            assert!(checked > 300);
        }
    }

    #[test]
    fn abscissa_test_matches_enumeration_n7() {
        let f = Field::new(7);
        for &(a, b) in &[(0u64, 1u64), (1, 1)] {
            let c = Curve { f: &f, a, b };
            let mut n_abs = 0;
            for x in 0..128u64 {
                // brute-force: exists y with y^2+xy = rhs
                let rhs = f.mul(f.sq(x), x) ^ f.mul(a, f.sq(x)) ^ b;
                let brute = (0..128u64).any(|y| f.sq(y) ^ f.mul(x, y) == rhs);
                assert_eq!(c.is_abscissa(x), brute, "x={x}");
                if brute {
                    n_abs += 1;
                    let y = c.lift(x).unwrap();
                    assert!(c.on_curve(Pt::Aff(x, y)) && c.on_curve(Pt::Aff(x, y ^ x)));
                }
            }
            assert!(n_abs > 40);
        }
        // a=0 and a=1 abscissa sets partition F*_q (twist) apart from nothing: complementary
        let c0 = Curve { f: &f, a: 0, b: 1 };
        let c1 = Curve { f: &f, a: 1, b: 1 };
        for x in 1..128u64 {
            assert_ne!(
                c0.is_abscissa(x),
                c1.is_abscissa(x),
                "twist complement, x={x}"
            );
        }
    }

    /// Exhaustive ground truth at n=7: oracle vs raw enumeration of S3/S4-chain.
    #[test]
    fn oracle_matches_bruteforce_n7() {
        let f = Field::new(7);
        let nb = Basis::normal(&f);
        for l in [3u32, 4] {
            let spec = vspec_for_dim(7, l, 0).unwrap();
            let (t1, fs, _) = cyclotomic_factors(7);
            let g = spec.g(t1, &fs);
            let vb = stable_subspace(&f, &nb, g, &nb);
            let mut o = Oracle::new(&f, &nb, &vb, g, 1);
            assert_eq!(o.velems.len(), 1 << l);
            let v = o.velems.clone();
            for xr in 1..128u64 {
                // m = 2 brute force
                let mut brute2 = 0u64;
                for &x1 in &v {
                    for &x2 in &v {
                        if f.s3(x1, x2, xr, 1) == 0 {
                            brute2 += 1;
                        }
                    }
                }
                assert_eq!(o.count_m2_ordered(xr), brute2, "m2 l={l} xr={xr}");
                // m = 3 brute force over X in the whole field
                let mut brute3 = false;
                'o: for &x1 in &v {
                    for &x2 in &v {
                        for &x3 in &v {
                            for xx in 0..128u64 {
                                if f.s3(x1, x2, xx, 1) == 0 && f.s3(xx, x3, xr, 1) == 0 {
                                    brute3 = true;
                                    break 'o;
                                }
                            }
                        }
                    }
                }
                assert_eq!(o.exists_m3(xr), brute3, "m3 l={l} xr={xr}");
            }
        }
    }

    #[test]
    fn frobenius_equivariance_of_oracle_on_stable_v() {
        // sigma maps solutions for xr to solutions for xr^2 on a stable V (Koblitz b in F_2)
        let f = Field::new(17);
        let nb = Basis::normal(&f);
        let spec = vspec_for_dim(17, 8, 0).unwrap();
        let (t1, fs, _) = cyclotomic_factors(17);
        let g = spec.g(t1, &fs);
        let vb = stable_subspace(&f, &nb, g, &nb);
        let o = Oracle::new(&f, &nb, &vb, g, 1);
        let mut rng = Rng(99);
        for _ in 0..12 {
            let xr = 1 + rng.below((1 << 17) - 1);
            let c0 = o.count_m2_ordered(xr);
            assert_eq!(c0, o.count_m2_ordered(f.sq(xr)), "equivariance, xr={xr}");
        }
    }
}
