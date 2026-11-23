//! Principal-divisor relation screen. Included by the experimental example.
//! Identity and claim boundaries: research/hec_relation_math_20261114/DERIVATION.md.
use hec_native_ic::{
    fp::Fp,
    instance::factor_base,
    jac::{Curve, Div},
    oracle::{split, OracleStats},
    poly::Poly,
    rng::Rng,
    solve::{solve_fast, Row, SolveStats},
};
use std::time::Instant;

type SignedRow = Vec<(u32, i64)>;

/// Newton interpolation with distinct abscissas; no assumption about the curve.
pub fn interpolate(points: &[(u64, u64)], fp: &Fp) -> (Poly, Poly) {
    let mut h = Poly::zero();
    let mut a = Poly::one();
    for &(x, y) in points {
        let den = a.eval(x, fp);
        assert_ne!(den, 0, "distinct interpolation abscissas");
        let k = fp.mul(fp.sub(y, h.eval(x, fp)), fp.inv(den));
        h = h.add(&a.scale(k, fp), fp);
        a = a.mul(&Poly::x_minus(x, fp), fp);
    }
    (h, a)
}
fn canonical(mut r: SignedRow) -> SignedRow {
    r.sort_unstable_by_key(|e| e.0);
    let mut out = Vec::new();
    for (c, v) in r {
        if let Some((cc, vv)) = out.last_mut() {
            if *cc == c {
                *vv += v;
                continue;
            }
        }
        out.push((c, v));
    }
    out.retain(|e| e.1 != 0);
    out
}
fn modular(r: &SignedRow, f: &Fp) -> Row {
    r.iter()
        .filter_map(|&(c, v)| {
            let v = f.from_i64(v);
            (v != 0).then_some((c, v))
        })
        .collect()
}
fn sum_row(curve: &Curve, fb: &[(u64, u64)], row: &SignedRow) -> Div {
    let mut acc = curve.zero();
    for &(j, v) in row {
        let (x, y) = fb[j as usize];
        let d = curve.mul(&curve.place(x, y), v.unsigned_abs());
        acc = curve.add(&acc, &if v < 0 { curve.neg(&d) } else { d });
    }
    acc
}
fn decompose(
    curve: &Curve,
    d: &Div,
    fb: &[(u64, u64)],
    by_x: &[u32],
    st: &mut OracleStats,
) -> Option<SignedRow> {
    let roots = split(&d.u, &curve.fp, st)?;
    let mut row = Vec::new();
    for &(x, mult) in roots.as_slice() {
        let j = by_x[x as usize];
        assert_ne!(j, u32::MAX);
        let sign = if d.v.eval(x, &curve.fp) == fb[j as usize].1 {
            1
        } else {
            -1
        };
        row.push((j, sign * mult as i64));
    }
    Some(canonical(row))
}

/// Exact raw principal row, including overlapping roots and their multiplicities.
pub fn principal(
    curve: &Curve,
    fb: &[(u64, u64)],
    by_x: &[u32],
    chosen: &[(usize, bool)],
    st: &mut OracleStats,
) -> Option<SignedRow> {
    assert_eq!(chosen.len(), curve.g + 1);
    let points: Vec<_> = chosen
        .iter()
        .map(|&(j, neg)| {
            let (x, y) = fb[j];
            (x, if neg { curve.fp.neg(y) } else { y })
        })
        .collect();
    let (h, a) = interpolate(&points, &curve.fp);
    let (q, rem) = curve
        .f
        .sub(&h.mul(&h, &curve.fp), &curve.fp)
        .divrem(&a, &curve.fp);
    assert!(rem.is_zero());
    assert_eq!(q.deg(), curve.g as isize);
    assert_eq!(q.lc(), 1);
    let roots = split(&q, &curve.fp, st)?;
    let mut row: SignedRow = chosen
        .iter()
        .map(|&(j, n)| (j as u32, if n { -1 } else { 1 }))
        .collect();
    for &(x, mult) in roots.as_slice() {
        let j = by_x[x as usize];
        assert_ne!(j, u32::MAX);
        let y = h.eval(x, &curve.fp);
        assert_eq!(curve.fp.mul(y, y), curve.f.eval(x, &curve.fp));
        let sign = if y == fb[j as usize].1 { 1 } else { -1 };
        row.push((j, sign * mult as i64));
    }
    Some(canonical(row))
}

#[derive(Default, Debug)]
pub struct Report {
    pub k: Option<u64>,
    pub trials: u64,
    pub accepted: usize,
    pub m: usize,
    pub descent_steps: u64,
    pub descent_calls: u64,
    pub solve_rounds: u32,
    pub max_weight: usize,
    pub group_checks: u64,
    pub oracle: OracleStats,
    pub solve: SolveStats,
    pub fb_ns: u128,
    pub descent_ns: u128,
    pub relation_ns: u128,
    pub solve_ns: u128,
    pub verify_ns: u128,
    pub total_ns: u128,
}
fn descend(
    curve: &Curve,
    d: &Div,
    fb: &[(u64, u64)],
    by_x: &[u32],
    rng: &mut Rng,
    gate: bool,
    rep: &mut Report,
) -> Option<SignedRow> {
    let mut r = *d;
    let mut counts = vec![0i64; fb.len()];
    for step in 0..=10000 {
        rep.descent_calls += 1;
        if let Some(mut row) = decompose(curve, &r, fb, by_x, &mut rep.oracle) {
            for (j, &n) in counts.iter().enumerate() {
                if n != 0 {
                    row.push((j as u32, -n));
                }
            }
            let row = canonical(row);
            if gate {
                assert_eq!(sum_row(curve, fb, &row), *d, "descent identity");
                rep.group_checks += 1;
            }
            return Some(row);
        }
        if step == 10000 {
            break;
        }
        let j = rng.below(fb.len() as u64) as usize;
        counts[j] += 1;
        let (x, y) = fb[j];
        r = curve.add_place(&r, x, y);
        rep.descent_steps += 1;
    }
    None
}

/// Cold, target-connected IC. Both descents and every rank retry are charged.
pub fn solve(curve: &Curve, d1: &Div, d2: &Div, ell: u64, seed: u64, gate: bool) -> Report {
    let all = Instant::now();
    // A unique rational ell-primary component makes factor-base log
    // normalization valid. Use a conservative integral Hasse bound.
    let mut root = (curve.fp.p as f64).sqrt() as u64;
    while (root as u128) * (root as u128) < curve.fp.p as u128 {
        root += 1;
    }
    let upper = (root as u128 + 1)
        .checked_pow((2 * curve.g) as u32)
        .expect("bounded Hasse certificate");
    assert!(
        (ell as u128) * (ell as u128) > upper,
        "unique ell-primary component not certified"
    );
    let mut rep = Report::default();
    let mut rng = Rng::new(seed);
    let fl = Fp::new(ell);
    let t = Instant::now();
    let fb = factor_base(curve);
    let m = fb.len();
    rep.m = m;
    assert!(m >= curve.g + 1);
    let mut by_x = vec![u32::MAX; curve.fp.p as usize];
    for (j, &(x, _)) in fb.iter().enumerate() {
        by_x[x as usize] = j as u32;
    }
    rep.fb_ns = t.elapsed().as_nanos();
    let t = Instant::now();
    let Some(a) = descend(curve, d1, &fb, &by_x, &mut rng, gate, &mut rep) else {
        rep.total_ns = all.elapsed().as_nanos();
        return rep;
    };
    let Some(b) = descend(curve, d2, &fb, &by_x, &mut rng, gate, &mut rep) else {
        rep.total_ns = all.elapsed().as_nanos();
        return rep;
    };
    rep.descent_ns = t.elapsed().as_nanos();
    let mut target = modular(&b, &fl);
    target.push((m as u32, ell - 1));
    let mut rows = vec![(modular(&a, &fl), 1), (target, 0)];
    let mut wanted = m + 8;
    while rep.trials < 1_000_000 && rep.solve_rounds < 16 {
        rep.trials += 1;
        let t = Instant::now();
        let mut chosen = Vec::with_capacity(curve.g + 1);
        while chosen.len() < curve.g + 1 {
            let j = rng.below(m as u64) as usize;
            if chosen.iter().any(|&(jj, _)| jj == j) {
                continue;
            }
            chosen.push((j, rng.below(2) == 1));
        }
        let raw = principal(curve, &fb, &by_x, &chosen, &mut rep.oracle);
        rep.relation_ns += t.elapsed().as_nanos();
        let Some(raw) = raw else {
            continue;
        };
        if raw.is_empty() {
            continue;
        }
        if gate {
            assert_eq!(
                sum_row(curve, &fb, &raw),
                curve.zero(),
                "principal divisor identity"
            );
            rep.group_checks += 1;
        }
        rep.max_weight = rep.max_weight.max(raw.len());
        rep.accepted += 1;
        rows.push((modular(&raw, &fl), 0));
        if rows.len() < wanted {
            continue;
        }
        let t = Instant::now();
        rep.solve_rounds += 1;
        let k = solve_fast(&rows, m, ell, &mut rep.solve);
        rep.solve_ns += t.elapsed().as_nanos();
        if let Some(k) = k {
            let t = Instant::now();
            let correct = curve.mul(d1, k) == *d2;
            rep.verify_ns += t.elapsed().as_nanos();
            if correct {
                rep.k = Some(k);
                break;
            }
        }
        wanted += 8 + m / 16;
    }
    rep.total_ns = all.elapsed().as_nanos();
    rep
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn interpolation_exact() {
        let fp = Fp::new(101);
        let p = Poly::from_coeffs(&[7, 3, 11, 4]);
        let pts: Vec<_> = [1, 4, 7, 9].iter().map(|&x| (x, p.eval(x, &fp))).collect();
        let (h, a) = interpolate(&pts, &fp);
        assert_eq!(p, h);
        for (x, _) in pts {
            assert_eq!(a.eval(x, &fp), 0);
        }
    }
    #[test]
    fn principal_and_descent_gates() {
        for (g, p) in [(2, 101), (2, 251), (3, 61), (3, 211), (4, 31), (4, 61)] {
            let i = hec_native_ic::instance::pick_instance(p, g).unwrap();
            for seed in 0..3 {
                let k = 37 + 19 * seed;
                let target = i.curve.mul(&i.d1, k);
                let r = solve(&i.curve, &i.d1, &target, i.ell, 20261114 + seed, true);
                assert_eq!(r.k, Some(k), "g={g} p={p} seed={seed} {r:?}");
                assert!(r.max_weight <= 2 * g + 1);
                assert_eq!(r.group_checks, r.accepted as u64 + 2);
            }
        }
    }
}
