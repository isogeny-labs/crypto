//! Bounded 0/1/2-large-prime baseline using exact partial elimination.
//! The full catalogue is materialized; this is a correctness baseline.
use crate::{
    fp::Fp,
    instance::{factor_base, order_bsgs, Instance},
    oracle::{split, OracleStats},
    rng::Rng,
    solve::{solve_fast, Row, SolveStats},
};
use std::time::Instant;

#[derive(Default, Debug)]
pub struct Report {
    pub k: Option<u64>,
    pub trials: u64,
    pub partials: u64,
    pub partial_pivots: usize,
    pub rows: usize,
    pub relation_checks: u64,
    pub projected_checks: u64,
    pub max_partial_entries: usize,
    pub solve: SolveStats,
    pub total_ns: u128,
}
fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

/// All validation, catalogue, projection, and exact elimination are charged.
pub fn run(inst: &Instance, m: usize, max_lp: usize, seed: u64, limit: u64) -> Report {
    let clock = Instant::now();
    let mut rep = Report::default();
    let curve = &inst.curve;
    let fp = &curve.fp;
    let f = Fp::new(inst.ell);
    let fb = factor_base(curve);
    let len = fb.len();
    assert!(m > 0 && m <= len && max_lp <= 2);
    // Find a common annihilator for this catalogue, then remove its single
    // ell factor. This certifies a projection on every place actually used.
    let mut common = inst.ell;
    for &(x, y) in &fb {
        let order =
            order_bsgs(curve, &curve.place(x, y), 200_000).expect("bounded catalogue order");
        common = (common / gcd(common, order))
            .checked_mul(order)
            .expect("catalogue exponent overflow");
    }
    assert_eq!(common % inst.ell, 0);
    let cofactor = common / inst.ell;
    assert_ne!(cofactor % inst.ell, 0);
    let projected: Vec<_> = fb
        .iter()
        .map(|&(x, y)| curve.mul(&curve.place(x, y), cofactor))
        .collect();
    for d in &projected {
        assert!(curve.is_zero(&curve.mul(d, inst.ell)));
    }
    let d1 = curve.mul(&inst.d1, cofactor);
    assert!(!curve.is_zero(&d1));
    let kt = inst.ell / 3 + 7;
    let target = curve.mul(&inst.d1, kt);
    let d2 = curve.mul(&target, cofactor);
    let mut by_x = vec![usize::MAX; fp.p as usize];
    for (j, &(x, _)) in fb.iter().enumerate() {
        by_x[x as usize] = j;
    }
    let mut rng = Rng::new(seed);
    let branches = m.min(64);
    let mut steps: Vec<usize> = (0..m).collect();
    for i in 0..branches {
        let j = i + rng.below((m - i) as u64) as usize;
        steps.swap(i, j);
    }
    steps.truncate(branches);
    let mut taken = vec![0u64; m];
    let (mut a, mut b) = (1, 1);
    let mut r = curve.add(&inst.d1, &target);
    let mut pivots: Vec<Option<(Vec<u64>, u64)>> = vec![None; len - m];
    let mut rows: Vec<(Row, u64)> = Vec::new();
    let mut wanted = m + 8;
    while rep.trials < limit {
        rep.trials += 1;
        match rng.below(4) {
            0 => {
                a = f.add(a, 1);
                r = curve.add(&r, &inst.d1);
            }
            1 => {
                b = f.add(b, 1);
                r = curve.add(&r, &target);
            }
            _ => {
                let j = steps[rng.below(branches as u64) as usize];
                taken[j] += 1;
                let (x, y) = fb[j];
                r = curve.add_place(&r, x, y);
            }
        }
        let Some(roots) = split(&r.u, fp, &mut OracleStats::default()) else {
            continue;
        };
        let mut off = 0;
        for &(x, _) in roots.as_slice() {
            if by_x[x as usize] >= m {
                off += 1;
            }
        }
        if off > max_lp {
            continue;
        }
        if off > 0 {
            rep.partials += 1;
        }
        let mut v = vec![0; len + 1];
        let mut decomposition = curve.zero();
        for &(x, mult) in roots.as_slice() {
            let j = by_x[x as usize];
            assert_ne!(j, usize::MAX);
            let positive = r.v.eval(x, fp) == fb[j].1;
            v[j] = f.add(
                v[j],
                if positive {
                    mult as u64
                } else {
                    f.neg(mult as u64)
                },
            );
            let d = curve.mul(&curve.place(x, fb[j].1), mult as u64);
            decomposition = curve.add(&decomposition, &if positive { d } else { curve.neg(&d) });
        }
        assert_eq!(decomposition, r, "decomposition identity");
        rep.relation_checks += 1;
        for j in 0..m {
            v[j] = f.sub(v[j], taken[j] % inst.ell);
        }
        v[len] = f.neg(b);
        let mut rhs = a;
        let mut stored = false;
        for j in m..len {
            if v[j] == 0 {
                continue;
            }
            if let Some((p, prhs)) = &pivots[j - m] {
                let factor = v[j];
                for k in 0..=len {
                    v[k] = f.sub(v[k], f.mul(factor, p[k]));
                }
                rhs = f.sub(rhs, f.mul(factor, *prhs));
            } else {
                let inv = f.inv(v[j]);
                for x in &mut v {
                    *x = f.mul(*x, inv);
                }
                rhs = f.mul(rhs, inv);
                rep.max_partial_entries = rep
                    .max_partial_entries
                    .max(v.iter().filter(|&&x| x != 0).count());
                pivots[j - m] = Some((v.clone(), rhs));
                rep.partial_pivots += 1;
                stored = true;
                break;
            }
        }
        // Certify every partial and every recombination against projected places.
        let mut check = curve.zero();
        for j in 0..len {
            if v[j] != 0 {
                check = curve.add(&check, &curve.mul(&projected[j], v[j]));
            }
        }
        check = curve.add(&check, &curve.mul(&d2, v[len]));
        assert_eq!(check, curve.mul(&d1, rhs), "partial elimination identity");
        rep.projected_checks += 1;
        if stored {
            continue;
        }
        let mut row: Row = (0..m)
            .filter(|&j| v[j] != 0)
            .map(|j| (j as u32, v[j]))
            .collect();
        if v[len] != 0 {
            row.push((m as u32, v[len]));
        }
        if row.is_empty() {
            assert_eq!(rhs, 0);
            continue;
        }
        rows.push((row, rhs));
        rep.rows = rows.len();
        if rows.len() >= wanted {
            if let Some(k) = solve_fast(&rows, m, inst.ell, &mut rep.solve) {
                if curve.mul(&inst.d1, k) == target {
                    rep.k = Some(k);
                    break;
                }
            }
            wanted += 8 + m / 16;
        }
    }
    rep.total_ns = clock.elapsed().as_nanos();
    rep
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn two_large_prime_identities() {
        let inst = crate::instance::pick_instance(31, 4).unwrap();
        let m = factor_base(&inst.curve).len() / 2;
        for lp in [1, 2] {
            let r = run(&inst, m, lp, 20260916, 10000);
            assert_eq!(r.k, Some(inst.ell / 3 + 7));
            assert!(r.partial_pivots > 0);
            assert_eq!(r.relation_checks, r.projected_checks);
        }
    }
}
