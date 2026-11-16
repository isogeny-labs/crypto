//! F_2-subspaces of F_{2^n} (polynomial-basis bit vectors), the sigma action on them, sigma-stable
//! subspaces (kernels of f(sigma) for f | x^n + 1), and factor-base class sets.

use crate::curve::{Group, NONE};
use crate::field::Fq;
use crate::rng::Rng;

/// Row-reduce `vecs` over F_2; returns an independent generating list (reduced echelon basis).
pub fn independent(vecs: &[u32]) -> Vec<u32> {
    let mut basis: Vec<u32> = vec![];
    for &v in vecs {
        let mut w = v;
        for &b in &basis {
            if w ^ b < w {
                w ^= b;
            }
        }
        if w != 0 {
            basis.push(w);
            basis.sort_unstable_by(|a, b| b.cmp(a));
        }
    }
    basis
}

pub fn rank(vecs: &[u32]) -> usize {
    independent(vecs).len()
}

/// All 2^l elements of span(basis) (basis must be independent), in Gray-code-free simple order.
pub fn span(basis: &[u32]) -> Vec<u32> {
    let mut out = Vec::with_capacity(1usize << basis.len());
    out.push(0u32);
    for &b in basis {
        let len = out.len();
        for i in 0..len {
            let v = out[i] ^ b;
            out.push(v);
        }
    }
    out
}

pub fn poly_window(l: u32) -> Vec<u32> {
    (0..l).map(|i| 1u32 << i).collect()
}

pub fn random_subspace(f: &Fq, l: u32, rng: &mut Rng) -> Vec<u32> {
    assert!(l <= f.n);
    loop {
        let v: Vec<u32> = (0..l).map(|_| f.rand(rng)).collect();
        if rank(&v) == l as usize {
            return v;
        }
    }
}

pub fn frob_basis(f: &Fq, basis: &[u32], j: u32) -> Vec<u32> {
    basis.iter().map(|&b| f.frob(b, j)).collect()
}

/// Is span(a) == span(b)? (same dimension and a inside b)
pub fn same_span(a: &[u32], b: &[u32]) -> bool {
    let ra = rank(a);
    ra == rank(b) && rank(&[a, b].concat()) == ra
}

/// Coordinates of v in the basis (v must lie in the span); None otherwise.
pub fn coords(basis: &[u32], v: u32) -> Option<u32> {
    // solve by elimination keeping track of combinations
    let mut rows: Vec<(u32, u32)> = vec![]; // (vector, combination mask)
    for (i, &b) in basis.iter().enumerate() {
        let mut w = b;
        let mut c = 1u32 << i;
        for &(rv, rc) in &rows {
            if w ^ rv < w {
                w ^= rv;
                c ^= rc;
            }
        }
        if w != 0 {
            rows.push((w, c));
            rows.sort_unstable_by(|a, b| b.0.cmp(&a.0));
        }
    }
    let mut w = v;
    let mut c = 0u32;
    for &(rv, rc) in &rows {
        if w ^ rv < w {
            w ^= rv;
            c ^= rc;
        }
    }
    if w == 0 {
        Some(c)
    } else {
        None
    }
}

// ---- polynomials over F_2 as u32 bit vectors (degree < 32) ----

pub fn pdeg(p: u32) -> i32 {
    31 - p.leading_zeros() as i32
}

pub fn pmod(mut a: u32, m: u32) -> u32 {
    let dm = pdeg(m);
    while a != 0 && pdeg(a) >= dm {
        a ^= m << (pdeg(a) - dm);
    }
    a
}

/// Monic divisors of x^n + 1 of degree d (all of them, irreducible or not).
pub fn divisors_of_xn1(n: u32, d: u32) -> Vec<u32> {
    let target = (1u32 << n) | 1;
    let mut v = vec![];
    for low in 0..(1u32 << d) {
        let cand = (1u32 << d) | low;
        if cand & 1 == 1 && pmod(target, cand) == 0 {
            v.push(cand);
        }
    }
    v
}

/// ker f(sigma) as a basis, for f a polynomial over F_2 (bit i = coefficient of x^i).
pub fn kernel_of_poly_sigma(f: &Fq, poly: u32) -> Vec<u32> {
    // images of the standard basis e_k = t^k under f(sigma)
    let n = f.n as usize;
    let mut rows: Vec<(u32, u32)> = vec![]; // (image, combination of e_k)
    let mut ker: Vec<u32> = vec![];
    for k in 0..n {
        let ek = 1u32 << k;
        let mut img = 0u32;
        for i in 0..=pdeg(poly) {
            if (poly >> i) & 1 == 1 {
                img ^= f.frob(ek, i as u32);
            }
        }
        let mut c = ek;
        for &(rv, rc) in &rows {
            if img ^ rv < img {
                img ^= rv;
                c ^= rc;
            }
        }
        if img == 0 {
            ker.push(c);
        } else {
            rows.push((img, c));
            rows.sort_unstable_by(|a, b| b.0.cmp(&a.0));
        }
    }
    independent(&ker)
}

/// sigma-stable subspaces of dimension d from the degree-d divisors of x^n + 1 (n prime).
pub fn stable_subspaces(f: &Fq, d: u32) -> Vec<Vec<u32>> {
    divisors_of_xn1(f.n, d)
        .into_iter()
        .map(|p| kernel_of_poly_sigma(f, p))
        .filter(|k| k.len() == d as usize)
        .collect()
}

pub fn is_sigma_stable(f: &Fq, basis: &[u32]) -> bool {
    same_span(&frob_basis(f, basis, 1), basis)
}

/// Factor-base classes of G with x in `elems`: (x values, class logarithms).
#[derive(Clone, Debug)]
pub struct BaseSet {
    pub xs: Vec<u32>,
    pub ds: Vec<u32>,
}

impl BaseSet {
    pub fn from_elems(g: &Group, elems: &[u32]) -> BaseSet {
        let mut xs = vec![];
        let mut ds = vec![];
        for &x in elems {
            let d = g.dlog[x as usize];
            if d != NONE {
                xs.push(x);
                ds.push(d);
            }
        }
        BaseSet { xs, ds }
    }
    pub fn from_classes(g: &Group, ids: &[u32]) -> BaseSet {
        // ids are logarithms d in 1..=half
        let xs: Vec<u32> = ids.iter().map(|&d| g.xs[(d - 1) as usize]).collect();
        BaseSet {
            xs,
            ds: ids.to_vec(),
        }
    }
    pub fn len(&self) -> usize {
        self.xs.len()
    }
    pub fn is_empty(&self) -> bool {
        self.xs.is_empty()
    }
    /// The classes x with sigma^j(x) also in the set (the shared-leg set X_j of the overlap bound).
    pub fn shared_with_shift(&self, f: &Fq, g: &Group, j: u32) -> BaseSet {
        let mut xs = vec![];
        let mut ds = vec![];
        let inset: std::collections::HashSet<u32> = self.xs.iter().cloned().collect();
        for (&x, &d) in self.xs.iter().zip(self.ds.iter()) {
            if inset.contains(&f.frob(x, j)) {
                xs.push(x);
                ds.push(d);
            }
        }
        let _ = g;
        BaseSet { xs, ds }
    }
    pub fn is_sigma_closed(&self, f: &Fq) -> bool {
        let inset: std::collections::HashSet<u32> = self.xs.iter().cloned().collect();
        self.xs.iter().all(|&x| inset.contains(&f.sq(x)))
    }
}

/// A uniformly random set of `k` distinct classes of G.
pub fn random_class_set(g: &Group, k: usize, rng: &mut Rng) -> BaseSet {
    let half = g.half() as usize;
    let k = k.min(half);
    let mut chosen: Vec<u32> = Vec::with_capacity(k);
    let mut seen = std::collections::HashSet::new();
    while chosen.len() < k {
        let d = 1 + rng.below(half as u64) as u32;
        if seen.insert(d) {
            chosen.push(d);
        }
    }
    chosen.sort_unstable();
    BaseSet::from_classes(g, &chosen)
}

/// A union of `t` distinct full sigma-orbits of classes of G (a sigma-closed non-subspace base; every orbit has n classes).
pub fn orbit_union(f: &Fq, g: &Group, t: usize, rng: &mut Rng) -> BaseSet {
    let half = g.half() as u64;
    let mut reps: Vec<u32> = vec![];
    let mut elems: Vec<u32> = vec![];
    while reps.len() < t {
        let d = 1 + rng.below(half) as u32;
        let x0 = g.xs[(d - 1) as usize];
        let mut orbit = vec![x0];
        let mut z = f.sq(x0);
        while z != x0 {
            orbit.push(z);
            z = f.sq(z);
        }
        let rep = *orbit.iter().min().unwrap();
        if reps.contains(&rep) {
            continue;
        }
        reps.push(rep);
        elems.extend(orbit);
    }
    BaseSet::from_elems(g, &elems)
}
