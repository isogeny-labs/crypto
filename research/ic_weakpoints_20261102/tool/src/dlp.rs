//! Large-prime variation (0, 1, 2 large primes) for n = 3 Weil descent.
//!
//! One relation stream per curve (r-adding walk R = aG + bQ, every decomposition of R by the exact
//! oracle). For each factor-base scaling |FB_r| = ceil(F * p^(r-1)) (F = all x-classes in F_p) the
//! stream feeds three eliminators: 0LP keeps full relations, 1LP also merges relations with one point
//! outside FB_r, 2LP also merges relations with two (graph-cycle elimination of the large-prime
//! variables, done as sparse Gaussian elimination on the large-prime part). Each variant stops at
//! |FB_r| + 1 + 20 full rows and is then solved mod ell; log Q is checked against the planted k.
use crate::curve::*;
use crate::field::*;
use crate::poly::macaulay_probe;
use crate::{arg, decompose3, esystem, factor_base};
use std::collections::{BTreeMap, HashMap};

fn mm(a: u128, b: u128, m: u128) -> u128 {
    (a * b) % m
}
fn inv(a: u128, m: u128) -> u128 {
    let (mut r, mut b, mut e) = (1u128, a % m, m - 2);
    while e > 0 {
        if e & 1 == 1 {
            r = mm(r, b, m);
        }
        b = mm(b, b, m);
        e >>= 1;
    }
    r
}

#[derive(Clone)]
struct Row {
    lp: Vec<(usize, u128)>, // large-prime part, at most 2 entries
    fb: BTreeMap<usize, u128>,
    a: u128,
    b: u128,
}

struct Elim {
    max_lp: usize,
    ell: u128,
    piv: HashMap<usize, Row>,
    full: Vec<Row>,
    ops: u64, // mod-ell multiplications spent merging
    target: usize,
    reached_at: Option<u64>,
    partial_in: u64,
}

impl Elim {
    fn insert(&mut self, mut r: Row) {
        let m = self.ell;
        if r.lp.len() > self.max_lp || self.reached_at.is_some() {
            return;
        }
        if !r.lp.is_empty() {
            self.partial_in += 1;
        }
        loop {
            r.lp.retain(|x| x.1 != 0);
            if r.lp.is_empty() {
                r.fb.retain(|_, c| *c != 0);
                self.full.push(r);
                return;
            }
            let v = r.lp.iter().map(|x| x.0).max().unwrap();
            let Some(pv) = self.piv.get(&v) else {
                self.piv.insert(v, r);
                return;
            };
            let cr = r.lp.iter().find(|x| x.0 == v).unwrap().1;
            let cp = pv.lp.iter().find(|x| x.0 == v).unwrap().1;
            let f = mm(cr, inv(cp, m), m);
            let mut lp: BTreeMap<usize, u128> = r.lp.iter().cloned().collect();
            for &(u, c) in &pv.lp {
                let e = lp.entry(u).or_insert(0);
                *e = (*e + m - mm(f, c, m)) % m;
            }
            for (&u, &c) in &pv.fb {
                let e = r.fb.entry(u).or_insert(0);
                *e = (*e + m - mm(f, c, m)) % m;
            }
            self.ops += (pv.lp.len() + pv.fb.len() + 3) as u64;
            r.a = (r.a + m - mm(f, pv.a, m)) % m;
            r.b = (r.b + m - mm(f, pv.b, m)) % m;
            r.lp = lp.into_iter().filter(|x| x.1 != 0).collect();
        }
    }
}

/// Dense solve of sum fb_i L_i - b k = a over Z/ell. Returns (rank, consistent, k, mults).
fn solve(rows: &[Row], cut: usize, ell: u128) -> (usize, bool, Option<u128>, u64) {
    let m = ell;
    let (kcol, rhs, w) = (cut, cut + 1, cut + 2);
    let mut a: Vec<Vec<u128>> = rows
        .iter()
        .map(|r| {
            let mut v = vec![0u128; w];
            for (&c, &x) in &r.fb {
                v[c] = x;
            }
            v[kcol] = (m - r.b % m) % m;
            v[rhs] = r.a % m;
            v
        })
        .collect();
    let n = a.len();
    let (mut rank, mut mults, mut piv) = (0, 0u64, vec![]);
    for col in 0..w {
        let Some(pr) = (rank..n).find(|&r| a[r][col] != 0) else { continue };
        a.swap(rank, pr);
        let iv = inv(a[rank][col], m);
        for j in col..w {
            a[rank][j] = mm(a[rank][j], iv, m);
        }
        mults += (w - col) as u64;
        let prow = a[rank].clone();
        for r in 0..n {
            if r != rank && a[r][col] != 0 {
                let f = a[r][col];
                for j in col..w {
                    if prow[j] != 0 {
                        a[r][j] = (a[r][j] + m - mm(f, prow[j], m)) % m;
                    }
                }
                mults += (w - col) as u64;
            }
        }
        piv.push(col);
        rank += 1;
    }
    let consistent = !piv.contains(&rhs);
    let mut k = None;
    if consistent {
        if let Some(ri) = piv.iter().position(|&c| c == kcol) {
            let free: Vec<usize> = (0..cut).filter(|c| !piv.contains(c)).collect();
            if free.iter().all(|&fc| a[ri][fc] == 0) {
                k = Some(a[ri][rhs]);
            }
        }
    }
    (piv.iter().filter(|&&c| c != rhs).count(), consistent, k, mults)
}

pub const RS: [f64; 5] = [1.0, 5.0 / 6.0, 0.75, 2.0 / 3.0, 0.5];

pub fn run(args: &[String]) {
    let p = arg(args, "--p", 127);
    let curves = arg(args, "--curves", 2);
    let seed = arg(args, "--seed", 20261104);
    let cap = arg(args, "--cap", 40 * p);
    let c0 = arg(args, "--c0", 0);
    let extra = 20usize;
    for ci in c0..c0 + curves {
        let mut rng = Rng(derive(seed, "dlp", p, ci));
        let f = Fq::new(p, 3, &mut rng);
        let (c, _nord, h, ell) = prime_order_curve(&f, 16, &mut rng);
        let g = loop {
            let g = c.mul(h, &c.rand_point(&mut rng));
            if g.is_some() {
                break g;
            }
        };
        let k_secret = 1 + rng.below128(ell - 1);
        let q = c.mul(k_secret, &g);
        let fb = factor_base(&c);
        let nfb = fb.xs.len();
        // c(3) on this curve
        let mut c3s = vec![];
        for _ in 0..3 {
            let xr = c.rand_point(&mut rng).unwrap().0;
            let (_, sys) = esystem(&c, 3, &xr);
            let mac = macaulay_probe(p, &sys, 3, 16);
            let sd = mac.solving_degree.unwrap_or(0);
            c3s.push(mac.steps.iter().find(|s| s.d == sd).map_or(0, |s| s.mults) as f64);
        }
        let c3 = c3s.iter().sum::<f64>() / 3.0;
        let cuts: Vec<usize> = RS
            .iter()
            .map(|&r| ((nfb as f64 * (p as f64).powf(r - 1.0)).ceil() as usize).clamp(3, nfb))
            .collect();
        let mut el: Vec<Vec<Elim>> = cuts
            .iter()
            .map(|&cut| {
                (0..3)
                    .map(|v| Elim { max_lp: v, ell, piv: HashMap::new(), full: vec![], ops: 0, target: cut + 1 + extra, reached_at: None, partial_in: 0 })
                    .collect()
            })
            .collect();
        let rw = 32;
        let table: Vec<(u128, u128, Pt)> = (0..rw)
            .map(|_| {
                let (a, b) = (rng.below128(ell), rng.below128(ell));
                (a, b, c.add(&c.mul(a, &g), &c.mul(b, &q)))
            })
            .collect();
        let (mut ra, mut rb) = (rng.below128(ell), 1 + rng.below128(ell - 1));
        let mut r = c.add(&c.mul(ra, &g), &c.mul(rb, &q));
        let mut trials = 0u64;
        let mut decomps = 0u64;
        while trials < cap && el.iter().flatten().any(|e| e.reached_at.is_none()) {
            trials += 1;
            if r.is_some() {
                let (dec, _) = decompose3(&c, &fb, &r, &mut rng);
                decomps += dec.len() as u64;
                for d in &dec {
                    let mut co: BTreeMap<usize, i64> = BTreeMap::new();
                    for &(i, s) in d {
                        *co.entry(i).or_insert(0) += s as i64;
                    }
                    for (ri, &cut) in cuts.iter().enumerate() {
                        let mut row = Row { lp: vec![], fb: BTreeMap::new(), a: ra, b: rb };
                        for (&i, &s) in &co {
                            if s == 0 {
                                continue;
                            }
                            let v = if s > 0 { s as u128 } else { ell - ((-s) as u128) };
                            if i < cut {
                                row.fb.insert(i, v);
                            } else {
                                row.lp.push((i, v));
                            }
                        }
                        for e in el[ri].iter_mut() {
                            e.insert(row.clone());
                            if e.reached_at.is_none() && e.full.len() >= e.target {
                                e.reached_at = Some(trials);
                            }
                        }
                    }
                }
            }
            let j = r.map_or(0, |(x, _)| (x[0] ^ x[1].rotate_left(7) ^ x[2].rotate_left(13)) as usize % rw);
            r = c.add(&r, &table[j].2);
            ra = (ra + table[j].0) % ell;
            rb = (rb + table[j].1) % ell;
        }
        let rho_neg = (std::f64::consts::PI * ell as f64 / 4.0).sqrt();
        for (ri, &cut) in cuts.iter().enumerate() {
            for e in &el[ri] {
                let rows: Vec<Row> = e.full.iter().take(e.target).cloned().collect();
                let nnz: usize = rows.iter().map(|r| r.fb.len() + 1).sum();
                let (rank, cons, k, la) = if e.reached_at.is_some() { solve(&rows, cut, ell) } else { (0, false, None, 0) };
                let t = e.reached_at.unwrap_or(trials) as f64;
                let la_sparse = 3.0 * (cut + 1) as f64 * nnz as f64;
                let band: Vec<String> = [0.0f64, 0.1, 1.0]
                    .iter()
                    .map(|&w| {
                        let ic = t + w * (t * c3 + la_sparse + e.ops as f64);
                        format!("{:.4e}", ic / (rho_neg * (1.0 + w)))
                    })
                    .collect();
                println!(
                    "{{\"exp\":\"dlp\",\"p\":{},\"curve\":{},\"ell\":{},\"h\":{},\"F\":{},\"r\":{:.4},\"cut\":{},\"lp\":{},\"target\":{},\"reached\":{},\"T\":{},\"trials_run\":{},\"decomps_seen\":{},\"partials_in\":{},\"pivots_left\":{},\"full_rows\":{},\"nnz\":{},\"mean_row_weight\":{:.3},\"merge_ops\":{},\"rank\":{},\"consistent\":{},\"k_correct\":{},\"la_dense_mults\":{},\"la_sparse_model\":{:.0},\"c3\":{:.0},\"rho_negmap\":{:.1},\"ratio_w\":[{}]}}",
                    p, ci, ell, h, nfb, RS[ri], cut, e.max_lp, e.target, e.reached_at.is_some(), t, trials, decomps,
                    e.partial_in, e.piv.len(), e.full.len(), nnz, nnz as f64 / rows.len().max(1) as f64, e.ops,
                    rank, cons, k.map_or("null".into(), |kk| (kk == k_secret && c.mul(kk, &g) == q).to_string()),
                    la, la_sparse, c3, rho_neg, band.join(",")
                );
            }
        }
    }
}
