//! Is p_1^j * S (S = S_3 or S_4 with x_R fixed, as a polynomial in the factor-base
//! abscissae) in the GF(2^n)-span of products of odd power sums p_1, p_3, p_5, ...?
//! Exact linear algebra over GF(2^n).

use crate::gf::Gf;
use std::collections::HashMap;

pub type Exp = [u8; 3];
#[derive(Clone, Debug, Default)]
pub struct P(pub HashMap<Exp, u64>);

pub struct R<'a> {
    pub f: &'a Gf,
    pub k: usize, // number of variables (2 or 3)
}

impl<'a> R<'a> {
    pub fn c(&self, v: u64) -> P {
        let mut p = P::default();
        if v != 0 {
            p.0.insert([0; 3], v);
        }
        p
    }
    pub fn x(&self, i: usize) -> P {
        let mut e = [0u8; 3];
        e[i] = 1;
        let mut p = P::default();
        p.0.insert(e, 1);
        p
    }
    pub fn add(&self, a: &P, b: &P) -> P {
        let mut r = a.clone();
        for (e, &v) in &b.0 {
            let t = r.0.entry(*e).or_insert(0);
            *t ^= v;
            if *t == 0 {
                r.0.remove(e);
            }
        }
        r
    }
    pub fn mul(&self, a: &P, b: &P) -> P {
        let mut r: HashMap<Exp, u64> = HashMap::new();
        for (ea, &va) in &a.0 {
            for (eb, &vb) in &b.0 {
                let e = [ea[0] + eb[0], ea[1] + eb[1], ea[2] + eb[2]];
                *r.entry(e).or_insert(0) ^= self.f.mul(va, vb);
            }
        }
        r.retain(|_, v| *v != 0);
        P(r)
    }
    pub fn pow(&self, a: &P, k: u32) -> P {
        (0..k).fold(self.c(1), |acc, _| self.mul(&acc, a))
    }
    pub fn deg(&self, a: &P) -> u32 {
        a.0.keys()
            .map(|e| e.iter().map(|&d| d as u32).sum())
            .max()
            .unwrap_or(0)
    }
    pub fn power_sum(&self, k: u32) -> P {
        (0..self.k).fold(P::default(), |acc, i| {
            self.add(&acc, &self.pow(&self.x(i), k))
        })
    }
    pub fn elem_sym(&self, k: usize) -> P {
        // e_k of the k variables via the generating product
        let mut e = vec![self.c(1)];
        for i in 0..self.k {
            let mut nxt = e.clone();
            nxt.push(P::default());
            for j in 0..e.len() {
                let t = self.mul(&e[j], &self.x(i));
                nxt[j + 1] = self.add(&nxt[j + 1], &t);
            }
            e = nxt;
        }
        e[k].clone()
    }
    pub fn eval(&self, a: &P, xs: &[u64]) -> u64 {
        a.0.iter().fold(0, |acc, (e, &v)| {
            let mut t = v;
            for i in 0..self.k {
                t = self.f.mul(t, self.f.pow(xs[i], e[i] as u128));
            }
            acc ^ t
        })
    }
}

/// S_3(x1, x2, xr) and S_4(x1, x2, x3, xr) for y^2 + xy = x^3 + a x^2 + b (depends on b only).
pub fn summation(r: &R, b: u64, xr: u64) -> P {
    let sq = |p: &P| r.mul(p, p);
    let (x1, x2) = (r.x(0), r.x(1));
    if r.k == 2 {
        let xrp = r.c(xr);
        let e2 = r.add(
            &r.add(&r.mul(&x1, &x2), &r.mul(&x1, &xrp)),
            &r.mul(&x2, &xrp),
        );
        return r.add(&r.add(&sq(&e2), &r.mul(&r.mul(&x1, &x2), &xrp)), &r.c(b));
    }
    let (x3, xrp) = (r.x(2), r.c(xr));
    let a = r.add(&sq(&x1), &sq(&x2));
    let bb = r.mul(&x1, &x2);
    let c = r.add(&r.mul(&sq(&x1), &sq(&x2)), &r.c(b));
    let d = r.add(&sq(&x3), &sq(&xrp));
    let e = r.mul(&x3, &xrp);
    let ff = r.add(&r.mul(&sq(&x3), &sq(&xrp)), &r.c(b));
    // Res_X(a X^2 + bb X + c, d X^2 + e X + ff) in characteristic 2
    let t1 = sq(&r.add(&r.mul(&a, &ff), &r.mul(&c, &d)));
    let t2 = r.mul(
        &r.add(&r.mul(&a, &e), &r.mul(&bb, &d)),
        &r.add(&r.mul(&bb, &ff), &r.mul(&c, &e)),
    );
    r.add(&t1, &t2)
}

/// All products of the given generators with weighted degree <= dmax.
fn products(r: &R, gens: &[(P, u32)], dmax: u32) -> Vec<P> {
    let mut out = vec![(r.c(1), 0u32, 0usize)];
    let mut i = 0;
    while i < out.len() {
        let (p, d, start) = out[i].clone();
        for (g, (gp, gd)) in gens.iter().enumerate().skip(start) {
            if d + gd <= dmax {
                out.push((r.mul(&p, gp), d + gd, g));
            }
        }
        i += 1;
    }
    out.into_iter().map(|(p, _, _)| p).collect()
}

/// target in span(rows)? Gaussian elimination over GF(2^n).
pub fn in_span(f: &Gf, rows: &[P], target: &P) -> bool {
    let mut idx: HashMap<Exp, usize> = HashMap::new();
    for p in rows.iter().chain(std::iter::once(target)) {
        for e in p.0.keys() {
            let l = idx.len();
            idx.entry(*e).or_insert(l);
        }
    }
    let w = idx.len();
    let dense = |p: &P| {
        let mut v = vec![0u64; w];
        for (e, &c) in &p.0 {
            v[idx[e]] = c;
        }
        v
    };
    let mut basis: Vec<(usize, Vec<u64>)> = vec![]; // (pivot column, row with pivot 1)
    let reduce = |v: &mut Vec<u64>, basis: &Vec<(usize, Vec<u64>)>| {
        for (pc, row) in basis {
            let c = v[*pc];
            if c != 0 {
                for k in 0..w {
                    if row[k] != 0 {
                        v[k] ^= f.mul(c, row[k]);
                    }
                }
            }
        }
    };
    for p in rows {
        let mut v = dense(p);
        reduce(&mut v, &basis);
        if let Some(pc) = v.iter().position(|&c| c != 0) {
            let inv = f.inv(v[pc]);
            for c in v.iter_mut() {
                *c = f.mul(*c, inv);
            }
            for (_, row) in basis.iter_mut() {
                let c = row[pc];
                if c != 0 {
                    for k in 0..w {
                        if v[k] != 0 {
                            row[k] ^= f.mul(c, v[k]);
                        }
                    }
                }
            }
            basis.push((pc, v));
        }
    }
    let mut t = dense(target);
    reduce(&mut t, &basis);
    t.iter().all(|&c| c == 0)
}

pub fn odd_power_sum_gens(r: &R, dmax: u32) -> Vec<(P, u32)> {
    (1..=dmax).step_by(2).map(|k| (r.power_sum(k), k)).collect()
}

pub fn elem_gens(r: &R) -> Vec<(P, u32)> {
    (1..=r.k).map(|k| (r.elem_sym(k), k as u32)).collect()
}

/// Smallest j <= jmax with p_1^j * target in the span of odd-power-sum products.
pub fn min_j(r: &R, target: &P, jmax: u32) -> Option<u32> {
    let p1 = r.power_sum(1);
    (0..=jmax).find(|&j| {
        let t = r.mul(&r.pow(&p1, j), target);
        let d = r.deg(&t);
        let rows = products(r, &odd_power_sum_gens(r, d), d);
        in_span(r.f, &rows, &t)
    })
}

pub fn in_elem_span(r: &R, target: &P) -> bool {
    let d = r.deg(target);
    let rows = products(r, &elem_gens(r), d);
    in_span(r.f, &rows, target)
}
