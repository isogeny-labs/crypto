#![allow(dead_code, unused_imports)]
#[path = "../multi/algebra.rs"]
mod algebra;
use hec_native_ic::{
    fp::Fp,
    ic::{solve_dlp, IcParams},
    instance::{factor_base, order_bsgs, prime_factors},
    jac::{Curve, Div},
    oracle::{split_reference, OracleStats},
    poly::Poly,
    rng::Rng,
    solve::{solve_for_last, Row, SolveStats, Solver},
};
use std::{fs, io::Write};
const NONE: usize = usize::MAX;
fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t
    }
    a
}
fn pack(v: &[u64], k: u64) -> Row {
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
fn rank(mut a: Vec<Vec<u64>>, f: &Fp) -> usize {
    if a.is_empty() {
        return 0;
    }
    let mut r = 0;
    for j in 0..a[0].len() {
        let Some(i) = (r..a.len()).find(|&i| a[i][j] != 0) else {
            continue;
        };
        a.swap(r, i);
        let inv = f.inv(a[r][j]);
        for k in j..a[r].len() {
            a[r][k] = f.mul(a[r][k], inv)
        }
        for i in r + 1..a.len() {
            let z = a[i][j];
            for k in j..a[i].len() {
                a[i][k] = f.sub(a[i][k], f.mul(z, a[r][k]))
            }
        }
        r += 1;
        if r == a.len() {
            break;
        }
    }
    r
}
fn dense(rows: &[(Row, u64)], m: usize, aug: bool, drop_k: bool) -> Vec<Vec<u64>> {
    rows.iter()
        .map(|(row, rhs)| {
            let mut v = vec![0; m + 1];
            for &(j, x) in row {
                v[j as usize] = x
            }
            if drop_k {
                v.pop();
            }
            if aug {
                v.push(*rhs)
            }
            v
        })
        .collect()
}
#[derive(Clone)]
pub struct Fixture {
    pub c: Curve,
    pub fb: Vec<(u64, u64)>,
    pub z: u64,
    pub ell: u64,
    pub h: u64,
    pub lambda: u64,
    pub map: Vec<(usize, u64)>,
    pub reps: Vec<usize>,
    pub source: Div,
    pub source_index: usize,
    pub constant: u64,
    pub l: u64,
}
pub fn fixture(g: usize, p: u64, receipt: &mut fs::File) -> Option<Fixture> {
    for cc in 1..p.min(33) {
        let f = Fp::new(p);
        let n = 2 * g + 1;
        let mut v = vec![0; n + 1];
        v[0] = cc;
        v[n] = 1;
        let c = Curve::new(f, Poly::from_coeffs(&v));
        if !algebra::irreducible(&c.f, &c.fp) {
            writeln!(receipt, "{g}\t{p}\t{cc}\treducible_model").unwrap();
            continue;
        }
        let fb = factor_base(&c);
        let (z, a) = algebra::character(&c);
        let mut orders = vec![];
        let mut l = 1u64;
        let mut budget = false;
        for &(x, y) in &fb {
            let point = c.place(x, y);
            if let Some(o) = order_bsgs(&c, &point, 200000) {
                assert!(c.is_zero(&c.mul(&point, o)));
                for q in prime_factors(o) {
                    assert!(!c.is_zero(&c.mul(&point, o / q)))
                }
                l = (l / gcd(l, o)).checked_mul(o).unwrap();
                orders.push(o);
            } else {
                budget = true;
                break;
            }
        }
        if budget || l <= 1 {
            writeln!(receipt, "{g}\t{p}\t{cc}\torder_budget").unwrap();
            continue;
        }
        let ell = *prime_factors(l).last().unwrap();
        let h = l / ell;
        if ell <= n as u64 || ell == p || h % ell == 0 || ell >= (1u64 << 62) {
            writeln!(receipt, "{g}\t{p}\t{cc}\tsubgroup_contract").unwrap();
            continue;
        }
        let fl = Fp::new(ell);
        let (q, _, degree, lambda) = algebra::certificate(&a, ell);
        assert_eq!(q.deg(), n as isize - 1);
        if degree != 1 {
            writeln!(
                receipt,
                "{g}\t{p}\t{cc}\tnonlinear_certificate_degree{degree}"
            )
            .unwrap();
            continue;
        }
        let lambda = lambda.unwrap();
        assert_eq!(q.eval(lambda, &fl), 0);
        assert_ne!(lambda, 1);
        let mut seen = vec![false; fb.len()];
        let mut map = vec![(NONE, 0); fb.len()];
        let mut reps = vec![];
        for start in 0..fb.len() {
            if seen[start] {
                continue;
            }
            if fb[start].0 == 0 {
                seen[start] = true;
                assert!(c.is_zero(&c.mul(&c.place(fb[start].0, fb[start].1), h)));
                continue;
            }
            let r = reps.len();
            reps.push(start);
            let (mut x, y, mut w) = (fb[start].0, fb[start].1, 1);
            for _ in 0..n {
                let j = fb.iter().position(|v| v.0 == x).unwrap();
                assert!(!seen[j]);
                seen[j] = true;
                map[j] = (
                    r,
                    if y == fb[j].1 {
                        w
                    } else {
                        assert_eq!(y, c.fp.neg(fb[j].1));
                        fl.neg(w)
                    },
                );
                x = c.fp.mul(x, z);
                w = fl.mul(w, lambda);
            }
            assert_eq!(x, fb[start].0);
            assert_eq!(w, 1);
        }
        assert!(seen.iter().all(|v| *v));
        if reps.len() < 2 {
            writeln!(receipt, "{g}\t{p}\t{cc}\tinsufficient_orbits").unwrap();
            continue;
        }
        // Independent original-group operator/quotient qualification on all catalogue points.
        for &(x, y) in &fb {
            let d = c.place(x, y);
            let mut point = d;
            let mut p_sum = c.zero();
            let mut s_sum = c.zero();
            for &co in &a {
                s_sum = c.add(&s_sum, &point);
                let term = c.mul(&point, co.unsigned_abs());
                p_sum = c.add(&p_sum, &if co < 0 { c.neg(&term) } else { term });
                point = algebra::action(&c, &point, z);
            }
            assert_eq!(point, d);
            assert_eq!(p_sum, d);
            assert!(c.is_zero(&s_sum));
        }
        for (j, &(r, w)) in map.iter().enumerate() {
            let point = c.mul(&c.place(fb[j].0, fb[j].1), h);
            assert!(c.is_zero(&c.mul(&point, ell)));
            if r == NONE {
                assert!(c.is_zero(&point));
            } else {
                let rep = c.mul(&c.place(fb[reps[r]].0, fb[reps[r]].1), h);
                assert_eq!(point, c.mul(&rep, w));
                assert_eq!(algebra::action(&c, &rep, z), c.mul(&rep, lambda));
            }
        }
        let source_index = (0..fb.len())
            .find(|&j| !c.is_zero(&c.mul(&c.place(fb[j].0, fb[j].1), h)))
            .expect("nonzero source projection");
        let source = c.mul(&c.place(fb[source_index].0, fb[source_index].1), h);
        let sr = map[source_index].0;
        let bad = (0..fb.len())
            .find(|&j| map[j].0 == sr && j != reps[sr])
            .unwrap();
        let projected_bad = c.mul(&c.place(fb[bad].0, fb[bad].1), h);
        let projected_rep = c.mul(&c.place(fb[reps[sr]].0, fb[reps[sr]].1), h);
        assert_ne!(projected_bad, c.mul(&projected_rep, fl.add(map[bad].1, 1)));

        assert!(c.is_zero(&c.mul(&source, ell)));
        writeln!(
            receipt,
            "{g}\t{p}\t{cc}\tselected_m{}_orbits{}_ell{}_h{}",
            fb.len(),
            reps.len(),
            ell,
            h
        )
        .unwrap();

        return Some(Fixture {
            c,
            fb,
            z,
            ell,
            h,
            lambda,
            map,
            reps,
            source,
            source_index,
            constant: cc,
            l,
        });
    }
    None
}
pub fn gate(x: &Fixture, case: usize, target: &Div) -> (Option<u64>, u64, usize, u64, u64) {
    let c = &x.c;
    let f = Fp::new(x.ell);
    let m = x.fb.len();
    let d = x.reps.len();
    let mut by_x = vec![NONE; c.fp.p as usize];
    for (j, v) in x.fb.iter().enumerate() {
        by_x[v.0 as usize] = j
    }
    let mut constraints = vec![];
    for j in 0..m {
        let (r, w) = x.map[j];
        if r != NONE && x.reps[r] == j {
            assert_eq!(w, 1);
            continue;
        }
        let mut v = vec![0; m];
        v[j] = 1;
        if r != NONE {
            v[x.reps[r]] = f.sub(v[x.reps[r]], w);
        }
        constraints.push((pack(&v, 0), 0));
    }
    assert_eq!(constraints.len(), m - d);
    let mut full = constraints;
    let mut folded: Vec<(Row, u64)> = vec![];
    let branches = m.min(64);
    let mut order: Vec<_> = (0..m).collect();
    let mut rng = Rng::new(2026111600 + case as u64);
    for i in 0..branches {
        let j = i + rng.below((m - i) as u64) as usize;
        order.swap(i, j)
    }
    let steps = &order[..branches];
    let mut taken = vec![0i64; branches];
    let mut folded_taken = vec![0; d];
    let (mut aa, mut bb) = (1, 1);
    let mut position = c.add(&x.source, target);
    let mut oracle = OracleStats::default();
    let (mut relations, mut dense_checks, mut folded_muls, mut full_muls) = (0, 0, 0, 0);
    for trial in 1..=200000u64 {
        match rng.below(4) {
            0 => {
                aa = f.add(aa, 1);
                position = c.add(&position, &x.source)
            }
            1 => {
                bb = f.add(bb, 1);
                position = c.add(&position, target)
            }
            _ => {
                let s = rng.below(branches as u64) as usize;
                taken[s] += 1;
                let j = steps[s];
                let (r, w) = x.map[j];
                if r != NONE {
                    folded_taken[r] = f.add(folded_taken[r], w)
                }
                position = c.add_place(&position, x.fb[j].0, x.fb[j].1)
            }
        }
        let Some(roots) = split_reference(&position.u, &c.fp, &mut oracle) else {
            continue;
        };
        let mut raw = vec![0i64; m];
        let mut independently_folded = vec![0; d];
        for &(xx, mult) in roots.as_slice() {
            let j = by_x[xx as usize];
            assert_ne!(j, NONE);
            let y = position.v.eval(xx, &c.fp);
            let sign = if y == x.fb[j].1 {
                1
            } else {
                assert_eq!(y, c.fp.neg(x.fb[j].1));
                -1
            };
            let co = sign * mult as i64;
            raw[j] += co;
            let (r, w) = x.map[j];
            if r != NONE {
                independently_folded[r] = f.add(independently_folded[r], f.mul(f.from_i64(co), w));
            }
        }
        assert_eq!(algebra::raw_sum(c, &x.fb, &raw), position);
        for (s, &count) in taken.iter().enumerate() {
            raw[steps[s]] -= count;
        }
        let group_right = c.add(&c.mul(&x.source, aa), &c.mul(target, bb));
        assert_eq!(algebra::raw_sum(c, &x.fb, &raw), group_right);
        let mut contracted = vec![0; d];
        for (j, &co) in raw.iter().enumerate() {
            let (r, w) = x.map[j];
            if r != NONE {
                contracted[r] = f.add(contracted[r], f.mul(f.from_i64(co), w));
            }
        }
        for j in 0..d {
            independently_folded[j] = f.sub(independently_folded[j], folded_taken[j]);
        }
        assert_eq!(contracted, independently_folded);
        let kc = f.neg(bb);
        let point_row: Vec<_> = raw.iter().map(|&v| f.from_i64(v)).collect();
        full.push((pack(&point_row, kc), aa));
        folded.push((pack(&independently_folded, kc), aa));
        relations += 1;
        let mut s1 = SolveStats::default();
        let mut s2 = SolveStats::default();
        let k1 = solve_for_last(&full, m, x.ell, Solver::Reference, &[], &mut s1);
        let k2 = solve_for_last(&folded, d, x.ell, Solver::Fast, &[], &mut s2);
        full_muls += s1.mulmods;
        folded_muls += s2.mulmods;
        assert_eq!(
            k1, k2,
            "same-prefix target determination must match expanded constraints"
        );
        if [1, 2, 4, 8, 16, 32, 64, 128].contains(&relations) || k2.is_some() {
            dense_checks += 1;
            let ra = rank(dense(&full, m, false, false), &f);
            let rb = rank(dense(&folded, d, false, false), &f);
            assert_eq!(ra, m - d + rb);
            let aug_a = rank(dense(&full, m, true, false), &f);
            let aug_b = rank(dense(&folded, d, true, false), &f);
            assert_eq!(aug_a, m - d + aug_b);
            assert_eq!(ra, aug_a);
            let without = rank(dense(&folded, d, false, true), &f);
            assert_eq!(rb - without, usize::from(k2.is_some()));
        }
        if let Some(k) = k2 {
            assert_eq!(c.mul(&x.source, k), *target);
            return (
                Some(k),
                trial,
                relations,
                dense_checks,
                full_muls + folded_muls,
            );
        }
    }
    (
        None,
        200000,
        relations,
        dense_checks,
        full_muls + folded_muls,
    )
}
