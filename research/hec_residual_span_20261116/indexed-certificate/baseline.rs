#![allow(dead_code, unused_imports)]
#[allow(dead_code)]#[path="../canonical-root/root.rs"]mod root;
#[path = "../multi/algebra.rs"]
mod algebra;
#[path = "../canonical-root/fast.rs"]
mod fast;
#[path = "../batched/inverse.rs"]
pub mod inverse;
#[path = "../normalized/source.rs"]
mod source_normalization;
use hec_native_ic::{
    fp::Fp,
    instance::factor_base,
    jac::{Curve, Div},
    poly::Poly,
    solve::{solve_for_last, Row, SolveStats, Solver},
};
const NONE: usize = usize::MAX;
#[derive(Clone, Copy, Debug)]
pub enum Mode {
    Folded,
    Expanded,
    Plain,
}
#[derive(Default, Debug)]
pub struct Report {
    pub k: Option<u64>,
    pub source_normalized: bool,
    pub trials: u64,
    pub relations: usize,
    pub m: usize,
    pub unknowns: usize,
    pub state_len: usize,
    pub solve_mulmods: u64,
    pub max_row_len: usize,
    pub anchors: u64,
    pub parameter: u64,
}
fn sparse(v: &[u64], k: u64) -> Row {
    let mut out: Row = v
        .iter()
        .enumerate()
        .filter_map(|(j, &v)| (v != 0).then_some((j as u32, v)))
        .collect();
    if k != 0 {
        out.push((v.len() as u32, k))
    }
    out
}
fn partition(
    c: &Curve,
    fb: &[(u64, u64)],
    ell: u64,
    z: u64,
    a: &[i64],
) -> (Vec<(usize, u64)>, Vec<usize>) {
    let n = 2 * c.g + 1;
    assert_eq!(c.f.deg(), n as isize);
    assert_eq!(c.f.lc(), 1);
    assert!(c.f.c[1..n].iter().all(|&v| v == 0));
    assert_ne!(c.f.c[0], 0);
    assert_eq!((c.fp.p - 1) % n as u64, 0);
    let f = Fp::new(ell);
    let (q, _, degree, lambda) = algebra::certificate(&a, ell);
    assert_eq!(q.deg(), n as isize - 1);
    assert_eq!(degree, 1);
    let lambda = lambda.unwrap();
    crate::packet::action(z,lambda);
    assert_ne!(lambda, 1);
    assert_eq!(q.eval(lambda, &f), 0);
    let mut seen = vec![false; fb.len()];
    let mut map = vec![(NONE, 0); fb.len()];
    let mut reps = vec![];
    for start in 0..fb.len() {
        if seen[start] {
            continue;
        }
        if fb[start].0 == 0 {
            seen[start] = true;
            continue;
        }
        let r = reps.len();
        reps.push(start);
        let (mut x, y, mut weight) = (fb[start].0, fb[start].1, 1);
        for _ in 0..n {
            let j = fb.iter().position(|v| v.0 == x).unwrap();
            assert!(!seen[j]);
            seen[j] = true;
            map[j] = (
                r,
                if fb[j].1 == y {
                    weight
                } else {
                    assert_eq!(fb[j].1, c.fp.neg(y));
                    f.neg(weight)
                },
            );
            x = c.fp.mul(x, z);
            weight = f.mul(weight, lambda);
        }
        assert_eq!(x, fb[start].0);
        assert_eq!(weight, 1);
    }
    assert!(seen.iter().all(|v| *v));
    (map, reps)
}
// Classical Sarkar–Singh pencil; standard public inputs only.
pub fn solve(c: &Curve, source: &Div, target: &Div, ell: u64, sm:u64, gate: bool) -> Report {
    let f = Fp::new(ell);
    let (fb, z, a) = fast::prepare(c);
    let (map, reps) = partition(c, &fb, ell, z, &a);
    crate::packet::begin(c,source,target,ell,sm,&fb,&reps);
    let cols = reps.len();
    let mut rep = Report {
        m: fb.len(),
        unknowns: cols,
        state_len: cols,
        ..Report::default()
    };
    let mut rows: Vec<(Row, u64)> = vec![];
    let mut linear = crate::sparse::Sparse::new(cols + 1, ell);let core=c.place(fb[0].0,fb[0].1);assert_eq!(c.mul(&core,sm),*source);let mut public=vec![0;cols+1];public[0]=1;let rhs=f.inv(sm);crate::packet::row(crate::packet::Kind::Public,&public,rhs);linear.insert(&public,rhs);if gate{rows.push((sparse(&public[..cols],0),rhs));}
    if let Some((_, v)) = source_normalization::equation(c, source, &fb, &map, cols, ell) {
        if gate {
            rows.push((sparse(&v, 0), 1));
        }
        let mut full = v;
        full.push(0);
        crate::packet::row(crate::packet::Kind::Source,&full,1);
        linear.insert(&full, 1);
        rep.source_normalized = true;
    }

    let mut anchor = target.clone();
    for a in 1..=16u64 {
        anchor = c.add(&anchor, source);
        rep.anchors = a;
        if anchor.u.deg() != c.g as isize {
            continue;
        }
        let mut buckets = vec![Vec::<(usize, u64)>::new(); c.fp.p as usize];
        let mut entries = vec![];
        let mut denominators = vec![];
        for (j, &(x, y)) in fb.iter().enumerate() {
            let u = anchor.u.eval(x, &c.fp);
            if u != 0 {
                entries.push((j, x, y));
                denominators.push(u)
            }
        }
        let inverses = inverse::invert(&denominators, &c.fp);
        for ((j, x, y), inv) in entries.into_iter().zip(inverses) {
            let v = anchor.v.eval(x, &c.fp);
            let t = c.fp.mul(c.fp.sub(y, v), inv);
            buckets[t as usize].push((j, y));
            if y != 0 {
                let yy = c.fp.neg(y);
                let t = c.fp.mul(c.fp.sub(yy, v), inv);
                buckets[t as usize].push((j, yy));
            }
        }
        // Complete anchor scan is charged before processing buckets in increasing t.
        for (t, bucket) in buckets.iter().enumerate() {
            if bucket.len() != c.g + 1 {
                continue;
            }
            let vt = if gate{anchor.v.add(&anchor.u.scale(t as u64, &c.fp), &c.fp)}else{Poly::zero()};
            let h=if gate{let (h,rem)=c.f.sub(&vt.mul(&vt,&c.fp),&c.fp).divrem(&anchor.u,&c.fp);assert_eq!(rem,Poly::zero());assert_eq!(h.deg(),(c.g+1)as isize);h}else{Poly::one()};
            let mut reconstructed = if gate{Poly::constant(h.lc())}else{Poly::zero()};
            let mut coeff = vec![0; cols];
            let mut last = None;
            for &(j, y) in bucket {
                let (x, canonical) = fb[j];
                assert_ne!(Some(x), last);
                last = Some(x);
                if gate{assert_eq!(vt.eval(x, &c.fp), y);}
                if gate{reconstructed = reconstructed.mul(&Poly::x_minus(x, &c.fp), &c.fp);}
                let (r, w) = map[j];
                if r != NONE {
                    let w = if y == canonical {
                        w
                    } else {
                        assert_eq!(y, c.fp.neg(canonical));
                        f.neg(w)
                    };
                    coeff[r] = f.add(coeff[r], w)
                }
            }
            if gate{assert_eq!(reconstructed, h);}
            let row = sparse(&coeff, 1);
            rep.max_row_len = rep.max_row_len.max(row.len());
            if gate {
                rows.push((row, f.neg(a % ell)));
            }
            rep.relations += 1;
            let mut full = coeff;
            full.push(1);
            crate::packet::row(crate::packet::Kind::Pencil{a,t:t as u64,points:bucket.iter().map(|&(j,y)|(fb[j].0,y)).collect()},&full,f.neg(a%ell));
            linear.insert(&full, f.neg(a % ell));
            let k = linear.k();
            if gate {
                let mut stats = SolveStats::default();
                let reference = solve_for_last(&rows, cols, ell, Solver::Fast, &[], &mut stats);
                assert_eq!(k, reference, "prefix mismatch");
            }
            if let Some(k) = k {
                let control_before=hec_native_ic::counted::snapshot();
                assert_eq!(c.mul(source, k), *target);
                crate::cost::record(control_before,hec_native_ic::counted::snapshot());
                crate::packet::finish(linear.witness(),k);
                rep.k = Some(k);
                rep.parameter = t as u64;
                return rep;
            }
        }
    }
    rep
}
