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
    let (fb, mut map, reps) = crate::setup::prepare(c, source, ell, n);
    let cols = reps.len();
    let mut small=1usize;while small*small<cols {small+=1;}
    for (r,_) in &mut map {if *r!=NONE {*r=if *r<small {cols-small+*r}else{*r-small};}}

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
        let mut buckets=std::collections::BTreeMap::<usize,Vec<(usize,u64)>>::new();
        let mut entries = vec![];
        let mut denominators = vec![];
        for (j, &(x, y)) in fb.iter().enumerate() {
            if map[j].0==NONE||map[j].0<cols-small {continue;}
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
            buckets.entry(t as usize).or_default().push((j,y));
            if y != 0 {
                let yy = c.fp.neg(y);
                let t = c.fp.mul(c.fp.sub(yy, v), inv);
                buckets.entry(t as usize).or_default().push((j,yy));
            }
        }
        // Complete anchor scan is charged before processing buckets in increasing t.
        for (t, bucket) in buckets.iter().map(|(&t,b)|(t,b)) {
            if bucket.len() < c.g - 1 {
                continue;
            }
            let vt = anchor.v.add(&anchor.u.scale(t as u64, &c.fp), &c.fp);
            let (h, rem) = c.f.sub(&vt.mul(&vt, &c.fp), &c.fp).divrem(&anchor.u, &c.fp);
            assert_eq!(rem, Poly::zero());
            assert_eq!(h.deg(), (c.g + 1) as isize);
            let mut rest=h;
            for &(j,_) in bucket {rest=rest.div_exact(&Poly::x_minus(fb[j].0,&c.fp),&c.fp);}
            assert!(rest.deg()<=2);
            let mut oracle=hec_native_ic::oracle::OracleStats::default();
            let Some(roots)=hec_native_ic::oracle::split_reference(&rest,&c.fp,&mut oracle) else{continue};
            let mut complete=bucket.clone();let mut reject=false;
            for &(x,multiplicity) in roots.as_slice() {
                if multiplicity!=1||anchor.u.eval(x,&c.fp)==0||complete.iter().any(|&(j,_)|fb[j].0==x){reject=true;break;}
                let j=fb.binary_search_by_key(&x,|p|p.0).unwrap();
                complete.push((j,vt.eval(x,&c.fp)));
            }
            if reject {continue;}
            complete.sort_unstable_by_key(|v|v.0);
            assert_eq!(complete.len(),c.g+1);
            let bucket=&complete;
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
    rep
}
