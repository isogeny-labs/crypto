use crate::{dense, lazy, sieve};
use hec_native_ic::{
    fp::Fp,
    instance::factor_base,
    jac::{Curve, Div},
    rng::Rng,
    solve::Row,
};
use std::{collections::HashSet, time::Instant};

enum Forest {
    Dense(dense::DenseForest),
    Lazy(lazy::LazyForest),
}
impl Forest {
    fn insert(&mut self, off: &[(usize, u64)], on: &Row) -> Option<Row> {
        match self {
            Self::Dense(f) => f.insert(off, on),
            Self::Lazy(f) => f.insert(off, on),
        }
    }
}
#[derive(Default, Debug)]
pub struct Report {
    pub k: Option<u64>,
    pub total: u128,
    pub setup: u128,
    pub sieve: u128,
    pub rows: u128,
    pub forest: u128,
    pub solve: u128,
    pub batches: u64,
    pub admitted: u64,
    pub pure: u64,
    pub rank: usize,
    pub hash: u64,
    pub anchors: usize,
}
pub fn solve(c: &Curve, d1: &Div, d2: &Div, ell: u64, seed: u64, lazy: bool, gate: bool) -> Report {
    let all = Instant::now();
    let mut r = Report::default();
    let f = Fp::new(ell);
    let fb = factor_base(c);
    let h = ((c.fp.p as f64).powf(1.0 - 1.0 / c.g as f64).ceil() as usize).min(fb.len());
    r.anchors = h;
    let mut forest = if lazy {
        Forest::Lazy(lazy::LazyForest::new(fb.len(), h + 2, ell))
    } else {
        Forest::Dense(dense::DenseForest::new(fb.len(), h + 2, ell))
    };
    let mut echelon = dense::Echelon::new(h, ell);
    let mut seen = HashSet::new();
    let mut rng = Rng::new(20261115 + seed);
    let mut jumps = Vec::new();
    for _ in 0..16 {
        let a = rng.below(ell);
        let b = rng.below(ell);
        jumps.push((a, b, c.add(&c.mul(d1, a), &c.mul(d2, b))));
    }
    let (mut a, mut b, mut d) = (0, 0, c.zero());
    let mut stats = sieve::Counts::default();
    r.setup = all.elapsed().as_nanos();
    'outer: for _ in 0..128 {
        r.batches += 1;
        let clock = Instant::now();
        let (da, db, jump) = jumps[rng.below(16) as usize];
        a = f.add(a, da);
        b = f.add(b, db);
        d = c.add(&d, &jump);
        if gate {
            assert_eq!(d, c.add(&c.mul(d1, a), &c.mul(d2, b)));
        }
        let batch = sieve::batch(c, &d, &fb, h, &mut stats);
        r.sieve += clock.elapsed().as_nanos();
        for relation in batch {
            let clock = Instant::now();
            if gate {
                assert_eq!(c.add(&d, &sieve::sum(c, &fb, &relation.terms)), c.zero());
            }
            let mut row: Row = relation
                .terms
                .iter()
                .map(|&(j, m)| (j as u32, f.from_i64(m)))
                .filter(|e| e.1 != 0)
                .collect();
            if a != 0 {
                row.push((fb.len() as u32, a));
            }
            if b != 0 {
                row.push((fb.len() as u32 + 1, b));
            }
            if row.is_empty() {
                r.rows += clock.elapsed().as_nanos();
                continue;
            }
            let inverse = f.inv(row[0].1);
            for (_, v) in &mut row {
                *v = f.mul(*v, inverse);
            }
            if !seen.insert(row.clone()) {
                r.rows += clock.elapsed().as_nanos();
                continue;
            }
            r.admitted += 1;
            for &(j, v) in &row {
                r.hash = (r.hash ^ j as u64).wrapping_mul(0x100000001b3);
                r.hash = (r.hash ^ v).wrapping_mul(0x100000001b3);
            }
            let (mut off, mut on) = (Vec::new(), Row::new());
            for (j, v) in row {
                let j = j as usize;
                if j < h {
                    on.push((j as u32, v));
                } else if j < fb.len() {
                    off.push((j + 1, v));
                } else {
                    on.push(((h + j - fb.len()) as u32, v));
                }
            }
            assert!(off.len() <= 2);
            r.rows += clock.elapsed().as_nanos();
            let clock = Instant::now();
            let got = forest.insert(&off, &on);
            r.forest += clock.elapsed().as_nanos();
            if let Some(row) = got {
                r.pure += 1;
                let clock = Instant::now();
                let mut rhs = 0;
                let mut equation = Row::new();
                for (j, v) in row {
                    if j as usize == h {
                        rhs = f.neg(v);
                    } else {
                        equation.push((if j as usize == h + 1 { h as u32 } else { j }, v));
                    }
                }
                r.k = echelon.insert(&equation, rhs);
                r.solve += clock.elapsed().as_nanos();
                if r.k.is_some() {
                    break 'outer;
                }
            }
        }
    }
    r.rank = echelon.rank;
    if let Some(k) = r.k {
        assert_eq!(c.mul(d1, k), *d2, "method final verification");
    }
    r.total = all.elapsed().as_nanos();
    r
}
