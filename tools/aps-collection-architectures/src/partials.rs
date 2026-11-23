//! Ordinary point-keyed one/two-large-point partials and coupled target rows.
//! Existing signed graph performs elimination; this adapter validates each row.
use crate::{key, valid_points, Affine, Fixture, Work};
use pencil_symbolic_graph::{meter, Outcome, SymbolicGraph};
use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Clone, Debug, Serialize)]
pub struct Partial {
    pub large: Vec<(usize, u64)>,
    /// c[0] P + sum c[i+1] Q_i + sum large[j] L_j = 0.
    pub small: Vec<u64>,
}

#[derive(Debug, Serialize)]
pub struct Collection {
    pub draws: u64,
    pub rejected_no_pool_match: u64,
    pub partials: Vec<Partial>,
    pub joined: u64,
    pub anchored: u64,
    pub dependent: u64,
    pub constraints: Vec<Vec<u64>>,
    pub graph_work: meter::Snapshot,
    pub recovered: Option<Vec<u64>>,
}

pub fn verify_partial(f: Fixture, targets: &[Affine], pool: &[Affine], p: &Partial) -> bool {
    if p.small.len() != targets.len() + 1
        || p.large.len() > 2
        || p.large.iter().any(|(i, _)| *i >= pool.len())
        || !valid_points(f, targets)
        || !valid_points(f, pool)
    {
        return false;
    }
    let mut sum = f.curve.mul(Some(f.generator), p.small[0] % f.order);
    for (&c, &q) in p.small[1..].iter().zip(targets) {
        sum = f.curve.add(sum, f.curve.mul(q, c % f.order));
    }
    for &(i, c) in &p.large {
        sum = f.curve.add(sum, f.curve.mul(pool[i], c % f.order));
    }
    sum.is_none()
}

/// A bounded generic affine producer, NOT a Semaev/APS smoothness engine.
/// Inputs contain target points, never their secret scalars. Draws, unmatched
/// residuals, dependent rows and every full emitted constraint are retained.
pub fn collect(
    f: Fixture,
    targets: &[Affine],
    pool: &[Affine],
    two: bool,
    cap: u64,
    w: &mut Work,
) -> Result<Collection, String> {
    if targets.is_empty()
        || targets.len() > 3
        || targets.contains(&None)
        || pool.is_empty()
        || pool.contains(&None)
        || !valid_points(f, targets)
        || !valid_points(f, pool)
        || cap > 4096
    {
        return Err("invalid bounded collector configuration".into());
    }
    let mut index = BTreeMap::new();
    for (i, &point) in pool.iter().enumerate() {
        if index.insert(key(point), i).is_some() {
            return Err("duplicate large-point key".into());
        }
    }
    // Fixed known small factor-base labels. Creation is inside the charged API boundary.
    let small: Vec<_> = (1..=3).map(|s| w.mul(f, Some(f.generator), s)).collect();
    if pool.iter().any(|p| small.contains(p)) {
        return Err("large pool overlaps small base".into());
    }
    let before = meter::snapshot();
    let mut graph =
        SymbolicGraph::new(f.order, pool.len(), targets.len() + 1).map_err(|e| format!("{e:?}"))?;
    let mut out = Collection {
        draws: 0,
        rejected_no_pool_match: 0,
        partials: vec![],
        joined: 0,
        anchored: 0,
        dependent: 0,
        constraints: vec![],
        graph_work: meter::Snapshot::default(),
        recovered: None,
    };
    for draw in 0..cap {
        out.draws += 1;
        w.draws += 1;
        // Deliberately repeat every draw once: a deterministic duplicate-row
        // control, not an independent random stream. Both attempts are charged.
        let draw = draw / 2;
        let a = (draw * 7 + 3) % f.order;
        let mut b = vec![0; targets.len()];
        b[draw as usize % targets.len()] = 1 + (draw / targets.len() as u64) % (f.order - 1);
        let sf = draw as usize % small.len();
        let mut residual = w.mul(f, Some(f.generator), a);
        for (&coeff, &q) in b.iter().zip(targets) {
            let term = w.mul(f, q, coeff);
            residual = w.add(f, residual, term);
        }
        residual = w.add(f, residual, f.neg(small[sf]));
        w.lookup_calls += 1;
        let large = if two {
            let left = draw as usize % pool.len();
            let right = w.add(f, pool[left], f.neg(residual));
            index
                .get(&key(right))
                .map(|&right| vec![(left, 1), (right, f.order - 1)])
        } else {
            index.get(&key(residual)).map(|&i| vec![(i, 1)])
        };
        let Some(large) = large else {
            out.rejected_no_pool_match += 1;
            continue;
        };
        let mut coeffs = vec![(f.order - a + (sf + 1) as u64) % f.order];
        coeffs.extend(b.iter().map(|&v| (f.order - v) % f.order));
        let row = Partial {
            large,
            small: coeffs,
        };
        w.witness_checks += 1;
        if !verify_partial(f, targets, pool, &row) {
            return Err("partial witness failed".into());
        }
        let terms: Vec<_> = row.small.iter().copied().enumerate().collect();
        match graph
            .insert(&row.large, &terms)
            .map_err(|e| format!("{e:?}"))?
        {
            Outcome::Joined => out.joined += 1,
            Outcome::Anchored => out.anchored += 1,
            Outcome::Constraint(s) => {
                let mut c = vec![0; targets.len() + 1];
                for (&i, &v) in &s {
                    c[i] = v;
                }
                if c.iter().all(|&v| v == 0) {
                    out.dependent += 1;
                } else {
                    out.constraints.push(c);
                }
            }
        }
        out.partials.push(row);
    }
    out.graph_work = meter::snapshot() - before;
    for c in &out.constraints {
        if !verify_partial(
            f,
            targets,
            &[],
            &Partial {
                large: vec![],
                small: c.clone(),
            },
        ) {
            return Err("eliminated relation invalid on original curve".into());
        }
    }
    out.recovered = solve_rows(f.order, targets.len(), &out.constraints)?;
    if let Some(ref ks) = out.recovered {
        for (&k, &q) in ks.iter().zip(targets) {
            w.witness_checks += 1;
            if w.mul(f, Some(f.generator), k) != q {
                return Err("source scalar verification failed".into());
            }
        }
    }
    Ok(out)
}

/// Dense independent scalar RREF. Rank deficiency is retained as None.
pub fn solve_rows(r: u64, n: usize, rows: &[Vec<u64>]) -> Result<Option<Vec<u64>>, String> {
    // This public lab accepts only its two prime subgroup moduli.
    if ![19, 107].contains(&r) || n == 0 || n > 3 || rows.iter().any(|v| v.len() != n + 1) {
        return Err("invalid small-system dimensions/modulus".into());
    }
    let mut a: Vec<Vec<u64>> = rows
        .iter()
        .map(|v| {
            let mut t: Vec<_> = v[1..].iter().map(|v| v % r).collect();
            t.push((r - v[0] % r) % r);
            t
        })
        .collect();
    let mut pivot = 0;
    let mut columns = vec![];
    for col in 0..n {
        let Some(ix) = (pivot..a.len()).find(|&i| a[i][col] != 0) else {
            continue;
        };
        a.swap(ix, pivot);
        // Brute inverse is intentionally independent of graph field arithmetic.
        let inv = (1..r)
            .find(|v| v * a[pivot][col] % r == 1)
            .ok_or("nonunit pivot")?;
        for c in &mut a[pivot] {
            *c = *c * inv % r;
        }
        let pivot_row = a[pivot].clone();
        for (i, row) in a.iter_mut().enumerate() {
            if i != pivot {
                let scale = row[col];
                for (value, &pivot_value) in row.iter_mut().zip(&pivot_row) {
                    *value = (*value + r - scale * pivot_value % r) % r;
                }
            }
        }
        columns.push(col);
        pivot += 1;
    }
    if a.iter()
        .any(|v| v[..n].iter().all(|&c| c == 0) && v[n] != 0)
    {
        return Err("inconsistent rows".into());
    }
    if columns.len() < n {
        return Ok(None);
    }
    let mut out = vec![0; n];
    for (row, &col) in columns.iter().enumerate() {
        out[col] = a[row][n];
    }
    Ok(Some(out))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::FixtureId;
    #[test]
    fn one_two_large_point_and_shared_target_recovery() {
        for id in [FixtureId::P17, FixtureId::P101] {
            let f = Fixture::new(id);
            let pool: Vec<_> = (9..15).map(|k| f.point(k)).collect();
            for n in 1..=3 {
                let ks: Vec<_> = [7, 11, 16][..n].to_vec();
                let qs: Vec<_> = ks.iter().map(|&k| f.point(k)).collect();
                for two in [false, true] {
                    let mut w = Work::default();
                    let got = collect(f, &qs, &pool, two, 512, &mut w).unwrap();
                    assert_eq!(got.recovered, Some(ks.clone()));
                    assert_eq!(got.draws, 512);
                    assert_eq!(got.partials.len() as u64 + got.rejected_no_pool_match, 512);
                    assert!(got
                        .partials
                        .iter()
                        .all(|p| verify_partial(f, &qs, &pool, p)));
                    assert!(got.dependent > 0);
                }
            }
        }
    }
    #[test]
    fn poisoned_partial_and_wrong_target_are_rejected() {
        let f = Fixture::new(FixtureId::P17);
        let pool = vec![f.point(9)];
        let qs = vec![f.point(7)];
        let valid = Partial {
            large: vec![(0, 1)],
            small: vec![3, 1],
        };
        assert!(verify_partial(f, &qs, &pool, &valid));
        let mut bad = valid.clone();
        bad.small[0] += 1;
        assert!(!verify_partial(f, &qs, &pool, &bad));
        assert!(!verify_partial(f, &[f.point(8)], &pool, &valid));
        bad.large = vec![(9, 1)];
        assert!(!verify_partial(f, &qs, &pool, &bad));
    }
    #[test]
    fn deficient_inconsistent_and_coupled_systems_are_distinct() {
        assert_eq!(solve_rows(19, 2, &[vec![1, 1, 0]]).unwrap(), None);
        assert!(solve_rows(19, 2, &[vec![1, 0, 0]]).is_err());
        assert_eq!(
            solve_rows(19, 2, &[vec![1, 1, 1], vec![5, 1, 2]]).unwrap(),
            Some(vec![3, 15])
        );
    }
    #[test]
    fn cap_and_bad_pool_retained_or_refused() {
        let f = Fixture::new(FixtureId::P17);
        let qs = vec![f.point(7)];
        let pool = vec![f.point(9)];
        let got = collect(f, &qs, &pool, false, 0, &mut Work::default()).unwrap();
        assert_eq!(got.recovered, None);
        assert_eq!(got.draws, 0);
        assert!(collect(f, &qs, &[pool[0], pool[0]], false, 10, &mut Work::default()).is_err());
        assert!(collect(f, &qs, &[f.point(1)], false, 10, &mut Work::default()).is_err());
    }
}
