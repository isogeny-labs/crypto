#![allow(dead_code)]
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
pub fn solve(c: &Curve, source: &Div, target: &Div, ell: u64, n: u64) -> Report {
    hec_native_ic::counted::stage("entry");
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
    hec_native_ic::counted::stage("source_normalization");
    if let Some((_, v)) = source_normalization::equation(c, source, &fb, &map, cols, ell) {
        rows.push((sparse(&v, 0), 1));
        rep.source_normalized = true;
    }

    let mut anchor = target.clone();
    for a in 1..=128u64 {
        hec_native_ic::counted::stage("anchor");
        anchor = c.add(&anchor, source);
        rep.anchors = a;
        if anchor.u.deg() != c.g as isize {
            continue;
        }
        hec_native_ic::counted::stage("scan");
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
            hec_native_ic::counted::stage("residual");
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
            rows.push((row, f.neg(a % ell)));
            rep.relations += 1;
            let mut stats = SolveStats::default();
            hec_native_ic::counted::stage("elimination");
            let k = solve_for_last(&rows, cols, ell, Solver::Fast, &[], &mut stats);
            rep.solve_mulmods += stats.mulmods;
            if let Some(k) = k {
                hec_native_ic::counted::stage("verification");
                assert_eq!(c.mul(source, k), *target);
                rep.k = Some(k);
                rep.parameter = t as u64;
                return rep;
            }
        }
    }
    rep
}
