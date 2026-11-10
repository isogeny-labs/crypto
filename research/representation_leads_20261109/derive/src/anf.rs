//! Boolean polynomials in algebraic normal form, and GF(2^n)-valued
//! expressions over them (one ANF per polynomial-basis coordinate).

use crate::gf::Gf;
use std::collections::{BTreeMap, HashSet};

pub const WORDS: usize = 8; // up to 512 Boolean variables
pub type Mono = [u64; WORDS];

pub fn mono_var(v: usize) -> Mono {
    let mut m = [0u64; WORDS];
    m[v / 64] |= 1 << (v % 64);
    m
}
pub fn mono_deg(m: &Mono) -> u32 {
    m.iter().map(|w| w.count_ones()).sum()
}
fn mono_mul(a: &Mono, b: &Mono) -> Mono {
    let mut r = *a;
    for i in 0..WORDS {
        r[i] |= b[i];
    }
    r
}

/// A Boolean polynomial: a set of monomials (the empty monomial is 1).
#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct Anf(pub HashSet<Mono>);

impl Anf {
    pub fn zero() -> Self {
        Anf(HashSet::new())
    }
    pub fn one() -> Self {
        let mut s = HashSet::new();
        s.insert([0u64; WORDS]);
        Anf(s)
    }
    pub fn var(v: usize) -> Self {
        let mut s = HashSet::new();
        s.insert(mono_var(v));
        Anf(s)
    }
    pub fn is_zero(&self) -> bool {
        self.0.is_empty()
    }
    fn toggle(&mut self, m: Mono) {
        if !self.0.remove(&m) {
            self.0.insert(m);
        }
    }
    pub fn add_assign(&mut self, o: &Anf) {
        for m in &o.0 {
            self.toggle(*m);
        }
    }
    pub fn mul(&self, o: &Anf) -> Anf {
        let mut r = Anf::zero();
        for a in &self.0 {
            for b in &o.0 {
                r.toggle(mono_mul(a, b));
            }
        }
        r
    }
    pub fn degree(&self) -> u32 {
        self.0.iter().map(mono_deg).max().unwrap_or(0)
    }
    pub fn eval(&self, bits: &[bool]) -> bool {
        let mut on = [0u64; WORDS];
        for (v, &b) in bits.iter().enumerate() {
            if b {
                on[v / 64] |= 1 << (v % 64);
            }
        }
        self.eval_mask(&on)
    }
    pub fn eval_mask(&self, on: &Mono) -> bool {
        self.0
            .iter()
            .filter(|m| (0..WORDS).all(|i| m[i] & !on[i] == 0))
            .count()
            % 2
            == 1
    }
}

/// A GF(2^n) element whose coordinates are Boolean polynomials.
#[derive(Clone, Debug)]
pub struct Fe(pub Vec<Anf>);

pub struct Ctx {
    pub f: Gf,
    pub nvars: usize,
}

impl Ctx {
    pub fn new(f: Gf) -> Self {
        Ctx { f, nvars: 0 }
    }
    pub fn constant(&self, c: u64) -> Fe {
        Fe((0..self.f.n)
            .map(|i| {
                if c >> i & 1 == 1 {
                    Anf::one()
                } else {
                    Anf::zero()
                }
            })
            .collect())
    }
    /// A fresh unknown restricted to span(basis): one Boolean variable per basis element.
    pub fn in_span(&mut self, basis: &[u64]) -> Fe {
        let mut x = self.constant(0);
        for &v in basis {
            let b = Anf::var(self.nvars);
            self.nvars += 1;
            for i in 0..self.f.n as usize {
                if v >> i & 1 == 1 {
                    x.0[i].add_assign(&b);
                }
            }
        }
        x
    }
    /// A fresh unconstrained unknown: n Boolean variables.
    pub fn free(&mut self) -> Fe {
        let basis: Vec<u64> = (0..self.f.n).map(|i| 1u64 << i).collect();
        self.in_span(&basis)
    }
    pub fn add(&self, a: &Fe, b: &Fe) -> Fe {
        let mut r = a.clone();
        for i in 0..r.0.len() {
            r.0[i].add_assign(&b.0[i]);
        }
        r
    }
    pub fn mul(&self, a: &Fe, b: &Fe) -> Fe {
        let n = self.f.n as usize;
        let mut t: Vec<Anf> = vec![Anf::zero(); 2 * n - 1];
        for i in 0..n {
            if a.0[i].is_zero() {
                continue;
            }
            for j in 0..n {
                if !b.0[j].is_zero() {
                    let p = a.0[i].mul(&b.0[j]);
                    t[i + j].add_assign(&p);
                }
            }
        }
        for k in (n..2 * n - 1).rev() {
            let hi = std::mem::take(&mut t[k]);
            for &e in self.f.low {
                t[k - n + e as usize].add_assign(&hi);
            }
        }
        t.truncate(n);
        Fe(t)
    }
    /// Squaring is F_2-linear: coordinate i moves to t^(2i), reduced.
    pub fn sq(&self, a: &Fe) -> Fe {
        let mut r = self.constant(0);
        for i in 0..self.f.n as usize {
            let img = self.f.sq(1u64 << i);
            for k in 0..self.f.n as usize {
                if img >> k & 1 == 1 {
                    r.0[k].add_assign(&a.0[i]);
                }
            }
        }
        r
    }
}

/// Size statistics of a system of field equations (each Fe = 0 gives n Boolean equations).
#[derive(Debug, Clone)]
pub struct Stats {
    pub nvars: usize,
    pub neqs: usize,
    pub max_degree: u32,
    pub terms: usize,
    pub distinct_monomials: usize,
    pub terms_by_degree: BTreeMap<u32, usize>,
}

pub fn stats(nvars: usize, eqs: &[Fe]) -> Stats {
    let mut all: HashSet<Mono> = HashSet::new();
    let (mut neqs, mut terms, mut maxd) = (0, 0, 0);
    let mut by: BTreeMap<u32, usize> = BTreeMap::new();
    for e in eqs {
        for c in &e.0 {
            if c.is_zero() {
                continue;
            }
            neqs += 1;
            terms += c.0.len();
            maxd = maxd.max(c.degree());
            for m in &c.0 {
                *by.entry(mono_deg(m)).or_default() += 1;
                all.insert(*m);
            }
        }
    }
    Stats {
        nvars,
        neqs,
        max_degree: maxd,
        terms,
        distinct_monomials: all.len(),
        terms_by_degree: by,
    }
}
