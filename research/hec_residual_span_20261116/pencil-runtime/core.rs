#[path = "../multi/algebra.rs"]
mod algebra;
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
fn partition(c: &Curve, fb: &[(u64, u64)], ell: u64) -> (Vec<(usize, u64)>, Vec<usize>) {
    let n = 2 * c.g + 1;
    assert_eq!(c.f.deg(), n as isize);
    assert_eq!(c.f.lc(), 1);
    assert!(c.f.c[1..n].iter().all(|&v| v == 0));
    assert_ne!(c.f.c[0], 0);
    assert_eq!((c.fp.p - 1) % n as u64, 0);
    let f = Fp::new(ell);
    let (z, a) = algebra::character(c);
    let (q, _, degree, lambda) = algebra::certificate(&a, ell);
    assert_eq!(q.deg(), n as isize - 1);
    assert_eq!(degree, 1);
    let lambda = lambda.unwrap();
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
pub fn solve(c: &Curve, source: &Div, target: &Div, ell: u64) -> Report {
    let f = Fp::new(ell);
    let fb = factor_base(c);
    let (map, reps) = partition(c, &fb, ell);
    let cols = reps.len();
    let mut rep = Report {
        m: fb.len(),
        unknowns: cols,
        state_len: cols,
        ..Report::default()
    };
    let mut rows: Vec<(Row, u64)> = vec![];
    let mut anchor = target.clone();
    for a in 1..=16u64 {
        anchor = c.add(&anchor, source);
        rep.anchors = a;
        if anchor.u.deg() != c.g as isize {
            continue;
        }
        let mut buckets = vec![Vec::<(usize, u64)>::new(); c.fp.p as usize];
        for (j, &(x, y)) in fb.iter().enumerate() {
            let u = anchor.u.eval(x, &c.fp);
            if u == 0 {
                continue;
            }
            let inv = c.fp.inv(u);
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
            let k = solve_for_last(&rows, cols, ell, Solver::Fast, &[], &mut stats);
            rep.solve_mulmods += stats.mulmods;
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
