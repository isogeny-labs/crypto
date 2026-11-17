#![allow(dead_code, unused_imports)]
#[path = "../batched/inverse.rs"]
mod inverse;
#[path = "../normalized/source.rs"]
mod source_normalization;
use hec_native_ic::{
    fp::Fp,
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
pub fn solve(c: &Curve, source: &Div, target: &Div, ell: u64, n: u64, gate: bool) -> Report {
    let f = Fp::new(ell);
    let (fb, map, reps) = crate::setup::prepare(c, source, ell, n);
    let cols = reps.len();
    let mut rep = Report {
        m: fb.len(),
        unknowns: cols,
        state_len: cols,
        ..Report::default()
    };
    let mut rows: Vec<(Row, u64)> = vec![];
    let mut linear = crate::sparse::Sparse::new(cols + 1, ell);
    if let Some((_, v)) = source_normalization::equation(c, source, &fb, &map, cols, ell) {
        if gate {
            rows.push((sparse(&v, 0), 1));
        }
        let mut full = v;
        full.push(0);
        linear.insert(&full, 1);
        rep.source_normalized = true;
    }

    let mut anchor = target.clone();
    for a in 1..=128u64 {
        anchor = c.add(&anchor, source);
        rep.anchors = a;
        if anchor.u.deg() != c.g as isize {
            continue;
        }
        let mut buckets = vec![Vec::<(usize, u64)>::new(); c.fp.p as usize];
        for begin in (0..fb.len()).step_by(16) {
        let mut ready=vec![];
        let mut entries=vec![];let mut denominators=vec![];
        for j in begin..(begin+16).min(fb.len()) {
            let (x,y)=fb[j];let u=anchor.u.eval(x,&c.fp);
            if u!=0 {entries.push((j,x,y));denominators.push(u);}
        }
        let inverses=inverse::invert(&denominators,&c.fp);
        for ((j,x,y),inv) in entries.into_iter().zip(inverses) {
            let v=anchor.v.eval(x,&c.fp);
            for yy in if y==0 {vec![y]}else{vec![y,c.fp.neg(y)]} {
                let t=c.fp.mul(c.fp.sub(yy,v),inv) as usize;
                buckets[t].push((j,yy));
                assert!(buckets[t].len()<=c.g+1);
                if buckets[t].len()==c.g+1 {ready.push(t);}
            }
        }
        ready.sort_unstable();ready.dedup();
        for t in ready {
            let bucket=&buckets[t];
            let vt = anchor.v.add(&anchor.u.scale(t as u64, &c.fp), &c.fp);
            let (h, rem) = c.f.sub(&vt.mul(&vt, &c.fp), &c.fp).divrem(&anchor.u, &c.fp);
            assert_eq!(rem, Poly::zero());
            assert_eq!(h.deg(), (c.g + 1) as isize);
            let mut reconstructed = Poly::constant(h.lc());
            let mut coeff = vec![0; cols];
            let mut last = None;
            for &(j, y) in bucket {
                let (x, canonical) = fb[j];
                assert_ne!(Some(x), last);
                last = Some(x);
                assert_eq!(vt.eval(x, &c.fp), y);
                reconstructed = reconstructed.mul(&Poly::x_minus(x, &c.fp), &c.fp);
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
            assert_eq!(reconstructed, h);
            let row = sparse(&coeff, 1);
            rep.max_row_len = rep.max_row_len.max(row.len());
            if gate {
                rows.push((row, f.neg(a % ell)));
            }
            rep.relations += 1;
            let mut full = coeff;
            full.push(1);
            linear.insert(&full, f.neg(a % ell));
            let k = linear.k();
            if gate {
                let mut stats = SolveStats::default();
                let reference = solve_for_last(&rows, cols, ell, Solver::Fast, &[], &mut stats);
                assert_eq!(k, reference, "prefix mismatch");
            }
            if let Some(k) = k {
                assert_eq!(c.mul(source, k), *target);
                rep.k = Some(k);
                rep.parameter = t as u64;
                return rep;
            }
        }
        }
    }
    rep
}
