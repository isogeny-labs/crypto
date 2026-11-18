//! Known point sieve. Public inputs only; no expected discrete logarithm.
use hec_native_ic::{
    jac::{Curve, Div},
    poly::Poly,
};
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Relation {
    pub lambda: u64,
    pub terms: Vec<(usize, i64)>,
}
#[derive(Default, Debug)]
pub struct Counts {
    pub batches: u64,
    pub low_degree: u64,
    pub skipped_u: u64,
    pub touched: u64,
    pub candidates: u64,
    pub repeated: u64,
    pub ramified: u64,
}

pub fn polynomial(c: &Curve, d: &Div, lambda: u64) -> (Poly, Poly) {
    let f = &c.fp;
    let t = d.v.sub(&d.u.scale(lambda, f), f);
    let q = t.mul(&t, f).sub(&c.f, f).div_exact(&d.u, f).monic(f);
    assert_eq!(q.deg(), (c.g + 1) as isize);
    (q, t)
}

fn quadratic(q: &Poly, c: &Curve) -> Option<Vec<(u64, usize)>> {
    let f = &c.fp;
    match q.deg() {
        0 => Some(Vec::new()),
        1 => Some(vec![(f.neg(q.c[0]), 1)]),
        2 => {
            let disc = f.sub(f.mul(q.c[1], q.c[1]), f.mul(4 % f.p, q.c[0]));
            let root = f.sqrt(disc)?;
            let a = f.mul(f.sub(root, q.c[1]), f.inv(2));
            let b = f.mul(f.sub(f.neg(root), q.c[1]), f.inv(2));
            if a == b {
                Some(vec![(a, 2)])
            } else {
                Some(vec![(a, 1), (b, 1)])
            }
        }
        _ => panic!("known roots did not leave a quadratic"),
    }
}

pub fn orient(c: &Curve, t: &Poly, fb: &[(u64, u64)], roots: &[(u64, usize)]) -> Vec<(usize, i64)> {
    let mut terms = BTreeMap::new();
    for &(x, m) in roots {
        let j = fb
            .binary_search_by_key(&x, |&(x, _)| x)
            .expect("root missing from catalogue");
        let y = t.eval(x, &c.fp);
        assert_eq!(c.fp.mul(y, y), c.f.eval(x, &c.fp));
        let sign = if y == fb[j].1 {
            1
        } else {
            assert_eq!(y, c.fp.neg(fb[j].1));
            -1
        };
        *terms.entry(j).or_insert(0) += sign * m as i64;
    }
    terms.into_iter().filter(|&(_, m)| m != 0).collect()
}

pub fn batch(c: &Curve, d: &Div, fb: &[(u64, u64)], h: usize, stats: &mut Counts) -> Vec<Relation> {
    stats.batches += 1;
    if d.u.deg() != c.g as isize {
        stats.low_degree += 1;
        return Vec::new();
    }
    let f = &c.fp;
    let mut buckets: BTreeMap<u64, Vec<u64>> = BTreeMap::new();
    for &(x, y) in &fb[..h] {
        let ux = d.u.eval(x, f);
        if ux == 0 {
            stats.skipped_u += 1;
            continue;
        }
        let inverse = f.inv(ux);
        let vx = d.v.eval(x, f);
        for y in [Some(y), (y != 0).then_some(f.neg(y))]
            .into_iter()
            .flatten()
        {
            let key = f.mul(f.sub(vx, y), inverse);
            buckets.entry(key).or_default().push(x);
        }
    }
    stats.touched += buckets.len() as u64;
    let mut relations = Vec::new();
    for (lambda, mut known) in buckets {
        known.sort_unstable();
        known.dedup();
        if known.len() < c.g - 1 {
            continue;
        }
        stats.candidates += 1;
        let (q, t) = polynomial(c, d, lambda);
        let mut rest = q;
        for &x in &known {
            rest = rest.div_exact(&Poly::x_minus(x, f), f);
        }
        let Some(other) = quadratic(&rest, c) else {
            continue;
        };
        let mut roots: Vec<_> = known.into_iter().map(|x| (x, 1)).collect();
        roots.extend(other);
        roots.sort_unstable();
        let mut merged: Vec<(u64, usize)> = Vec::new();
        for (x, m) in roots {
            if let Some((last, n)) = merged.last_mut() {
                if *last == x {
                    *n += m;
                    continue;
                }
            }
            merged.push((x, m));
        }
        let mut product = Poly::one();
        for &(x, m) in &merged {
            for _ in 0..m {
                product = product.mul(&Poly::x_minus(x, f), f);
            }
        }
        assert_eq!(product, q, "split reconstruction");
        stats.repeated += u64::from(merged.iter().any(|&(_, m)| m > 1));
        stats.ramified += u64::from(merged.iter().any(|&(x, _)| t.eval(x, f) == 0));
        relations.push(Relation {
            lambda,
            terms: orient(c, &t, fb, &merged),
        });
    }
    relations
}

pub fn sum(c: &Curve, fb: &[(u64, u64)], terms: &[(usize, i64)]) -> Div {
    terms.iter().fold(c.zero(), |s, &(j, m)| {
        let p = c.place(fb[j].0, fb[j].1);
        let p = if m < 0 { c.neg(&p) } else { p };
        c.add(&s, &c.mul(&p, m.unsigned_abs()))
    })
}
