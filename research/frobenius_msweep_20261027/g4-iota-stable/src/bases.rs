//! Factor-base constructions (sets of x-coordinates). All sets are returned sorted, unique.
//! Raw sets may contain 0 and 1; `core_set` removes them (x = 0 is T, x = 1 = b^(1/4) is the
//! iota-fixed x with order-4 points; neither is a usable factor-base element).

use crate::field::Fq;
use std::collections::HashSet;

pub struct Rng(pub u64);
impl Rng {
    pub fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    pub fn below(&mut self, n: u64) -> u64 {
        self.next() % n
    }
}

/// Derived seed: deterministic function of the cell, arm and replicate (no shared RNG stream).
pub fn derive_seed(base: u64, parts: &[u64]) -> u64 {
    let mut r = Rng(base ^ 0xD1B5_4A32_D192_ED03);
    let mut s = r.next();
    for &p in parts {
        r.0 = s ^ p.wrapping_mul(0x9E37_79B9_7F4A_7C15);
        s = r.next();
    }
    s
}

pub fn span(basis: &[u32]) -> Vec<u32> {
    let mut v = vec![0u32];
    for &b in basis {
        let k = v.len();
        for i in 0..k {
            let x = v[i] ^ b;
            v.push(x);
        }
    }
    v.sort_unstable();
    v
}

pub fn poly_window(l: u32) -> Vec<u32> {
    span(&(0..l).map(|i| 1u32 << i).collect::<Vec<_>>())
}

pub fn random_basis(n: u32, l: u32, rng: &mut Rng) -> Vec<u32> {
    let mut basis: Vec<u32> = vec![];
    let mut reduced: Vec<u32> = vec![]; // echelon vectors for independence testing
    while (basis.len() as u32) < l {
        let v = (rng.next() as u32) & ((1u32 << n) - 1);
        let mut w = v;
        for &r in &reduced {
            if w ^ r < w {
                w ^= r;
            }
        }
        if w != 0 {
            reduced.push(w);
            reduced.sort_unstable_by(|a, b| b.cmp(a));
            basis.push(v);
        }
    }
    basis
}

/// W union c/W* : with c = 1 (b = 1) this is W union iota(W*), the iota-stable base.
/// With c != 1 it is the "wrong involution" control (same construction, no stabiliser).
pub fn involution_union(f: &Fq, w: &[u32], c: u32) -> Vec<u32> {
    let mut s: Vec<u32> = w.to_vec();
    for &x in w {
        if x != 0 {
            s.push(f.mul(c, f.inv(x)));
        }
    }
    s.sort_unstable();
    s.dedup();
    s
}

pub fn core_set(raw: &[u32]) -> Vec<u32> {
    raw.iter().copied().filter(|&x| x > 1).collect()
}

/// Random k-subset of {2..q-1}.
pub fn random_set(q: u32, k: usize, rng: &mut Rng) -> Vec<u32> {
    let mut s = HashSet::new();
    while s.len() < k {
        s.insert(2 + rng.below((q - 2) as u64) as u32);
    }
    let mut v: Vec<u32> = s.into_iter().collect();
    v.sort_unstable();
    v
}

/// Union of k random iota-orbits {x, 1/x}, x in {2..q-1} (never the fixed x = 1): size 2k.
pub fn random_iota_set(f: &Fq, k: usize, rng: &mut Rng) -> Vec<u32> {
    let mut s = HashSet::new();
    while s.len() < 2 * k {
        let x = 2 + rng.below((f.q - 2) as u64) as u32;
        s.insert(x);
        s.insert(f.inv(x));
    }
    let mut v: Vec<u32> = s.into_iter().collect();
    v.sort_unstable();
    v
}

pub fn is_iota_stable(f: &Fq, xs: &[u32]) -> bool {
    let set: HashSet<u32> = xs.iter().copied().collect();
    xs.iter().all(|&x| x != 0 && set.contains(&f.inv(x)))
}
