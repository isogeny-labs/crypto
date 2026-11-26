//! Exact scalar-field diagnostics. Curve/subgroup validity is a caller obligation.
//! A returned scalar solves the supplied linear equations; it is not an ECDLP verdict.
use super::{inv, minus, mul, prime};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LabelSystem {
    pub scalar_prime: u64,
    pub columns: usize,
    pub rows: Vec<Vec<u64>>,
    pub a: Vec<u64>,
    pub b: Vec<u64>,
}
#[derive(Clone, Copy, Debug)]
pub struct Limits {
    pub max_rows: usize,
    pub max_columns: usize,
    pub max_dense_cells: usize,
    pub max_provenance_cells: usize,
}
impl Default for Limits {
    fn default() -> Self {
        Self {
            max_rows: 1024,
            max_columns: 1024,
            max_dense_cells: 1_048_576,
            max_provenance_cells: 1_048_576,
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Dependency {
    pub multipliers: Vec<u64>,
    pub a: u64,
    pub b: u64,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Audit {
    pub rank_r: usize,
    pub rank_r_b: usize,
    pub rank_augmented: usize,
    pub compatible_with_one_scalar: bool,
    pub selected_original_rows: Vec<usize>,
    pub dependency: Option<Dependency>,
    pub candidate_scalar: Option<u64>,
    /// Always false: this API receives no points or curve certificates.
    pub curve_scalar_verified: bool,
}
fn validate(s: &LabelSystem, limits: Limits) -> Result<(), String> {
    let p = s.scalar_prime;
    if p > i64::MAX as u64 || !prime(p) {
        return Err("scalar modulus must be a prime below2^63".into());
    }
    let m = s.rows.len();
    if m > limits.max_rows || s.columns > limits.max_columns || s.a.len() != m || s.b.len() != m {
        return Err("label dimension or row/column limit".into());
    }
    let cells = m
        .checked_mul(s.columns.checked_add(2).ok_or("column overflow")?)
        .ok_or("cell overflow")?;
    let provenance = m.checked_mul(m).ok_or("provenance overflow")?;
    if cells > limits.max_dense_cells || provenance > limits.max_provenance_cells {
        return Err("label cell budget".into());
    }
    if s.rows
        .iter()
        .any(|r| r.len() != s.columns || r.iter().any(|&c| c >= p))
        || s.a.iter().chain(&s.b).any(|&c| c >= p)
    {
        return Err("noncanonical coefficient or ragged matrix".into());
    }
    Ok(())
}
fn sub(a: u64, b: u64, p: u64) -> u64 {
    if a >= b {
        a - b
    } else {
        p - (b - a)
    }
}
fn insert(mut row: Vec<u64>, basis: &mut [Option<Vec<u64>>], p: u64) -> bool {
    for col in 0..row.len() {
        let c = row[col];
        if c == 0 {
            continue;
        }
        if let Some(g) = &basis[col] {
            for j in col..row.len() {
                row[j] = sub(row[j], mul(c, g[j], p), p);
            }
        } else {
            let reciprocal = inv(c, p);
            for x in &mut row[col..] {
                *x = mul(*x, reciprocal, p);
            }
            basis[col] = Some(row);
            return true;
        }
    }
    false
}
fn rank(rows: impl IntoIterator<Item = Vec<u64>>, width: usize, p: u64) -> usize {
    let mut basis = vec![None; width];
    let mut result = 0;
    for row in rows {
        if insert(row, &mut basis, p) {
            result += 1;
        }
    }
    result
}
fn augmented(s: &LabelSystem, i: usize) -> Vec<u64> {
    let mut r = s.rows[i].clone();
    r.extend([s.a[i], s.b[i]]);
    r
}
/// Keeps a row-space basis of [R|a|b], never of R alone. The weight is a
/// deterministic sparsity heuristic; it does not predict elimination fill.
pub fn audit(s: &LabelSystem, limits: Limits) -> Result<Audit, String> {
    validate(s, limits)?;
    let p = s.scalar_prime;
    let m = s.rows.len();
    let n = s.columns;
    let rank_r = rank(s.rows.clone(), n, p);
    let rank_r_b = rank(
        (0..m).map(|i| {
            let mut r = s.rows[i].clone();
            r.push(s.b[i]);
            r
        }),
        n + 1,
        p,
    );
    let rank_augmented = rank((0..m).map(|i| augmented(s, i)), n + 2, p);
    let compatible = rank_r_b == rank_augmented;
    let mut order: (Vec<usize>) = (0..m).collect();
    order.sort_by_key(|&i| (augmented(s, i).iter().filter(|&&x| x != 0).count(), i));
    let mut basis = vec![None; n + 2];
    let mut selected = vec![];
    for i in order {
        if insert(augmented(s, i), &mut basis, p) {
            selected.push(i);
        }
    }
    let mut dependency = None;
    let mut candidate = None;
    if compatible && rank_r_b > rank_r {
        // Row reduction carries original-row provenance in the last m columns.
        let mut w: Vec<Vec<u64>> = (0..m)
            .map(|i| {
                let mut r = augmented(s, i);
                r.extend((0..m).map(|j| u64::from(i == j)));
                r
            })
            .collect();
        let mut at = 0;
        for col in 0..n {
            let Some(pivot) = (at..m).find(|&i| w[i][col] != 0) else {
                continue;
            };
            w.swap(at, pivot);
            let recip = inv(w[at][col], p);
            for x in &mut w[at] {
                *x = mul(*x, recip, p);
            }
            let pivot = w[at].clone();
            for i in 0..m {
                if i != at {
                    let c = w[i][col];
                    if c != 0 {
                        for j in 0..pivot.len() {
                            w[i][j] = sub(w[i][j], mul(c, pivot[j], p), p);
                        }
                    }
                }
            }
            at += 1;
        }
        let row = w
            .iter()
            .find(|r| r[..n].iter().all(|&c| c == 0) && r[n + 1] != 0)
            .ok_or("rank/extraction disagreement")?;
        let d = mul(minus(row[n], p), inv(row[n + 1], p), p);
        dependency = Some(Dependency {
            multipliers: row[n + 2..].to_vec(),
            a: row[n],
            b: row[n + 1],
        });
        candidate = Some(d);
    }
    let out = Audit {
        rank_r,
        rank_r_b,
        rank_augmented,
        compatible_with_one_scalar: compatible,
        selected_original_rows: selected,
        dependency,
        candidate_scalar: candidate,
        curve_scalar_verified: false,
    };
    replay(s, &out, limits)?;
    Ok(out)
}
/// Recompute ranks/span, then check a dependency directly against every supplied
/// original row using wide modular arithmetic. No curve input is inferred.
pub fn replay(s: &LabelSystem, a: &Audit, limits: Limits) -> Result<(), String> {
    validate(s, limits)?;
    let p = s.scalar_prime;
    let m = s.rows.len();
    let n = s.columns;
    let r = rank(s.rows.clone(), n, p);
    let rb = rank(
        (0..m).map(|i| {
            let mut v = s.rows[i].clone();
            v.push(s.b[i]);
            v
        }),
        n + 1,
        p,
    );
    let full = rank((0..m).map(|i| augmented(s, i)), n + 2, p);
    if (r, rb, full) != (a.rank_r, a.rank_r_b, a.rank_augmented)
        || a.compatible_with_one_scalar != (rb == full)
        || a.curve_scalar_verified
    {
        return Err("audit rank/model/curve flag mismatch".into());
    }
    let ids: &[usize] = &a.selected_original_rows;
    let unique: BTreeSet<_> = ids.iter().copied().collect();
    if unique.len() != ids.len()
        || ids.iter().any(|&i| i >= m)
        || ids.len() != full
        || rank(ids.iter().map(|&i| augmented(s, i)), n + 2, p) != full
    {
        return Err("augmented row-space basis not preserved".into());
    }
    let expected = rb == full && rb > r;
    if a.dependency.is_some() != expected || a.candidate_scalar.is_some() != expected {
        return Err("missing or unsupported extraction".into());
    }
    if let (Some(w), Some(d)) = (&a.dependency, a.candidate_scalar) {
        if w.multipliers.len() != m
            || w.multipliers.iter().any(|&x| x >= p)
            || w.a >= p
            || w.b == 0
            || w.b >= p
            || d >= p
        {
            return Err("noncanonical dependency".into());
        }
        let dot = |values: Vec<u64>| -> u64 {
            w.multipliers
                .iter()
                .zip(values)
                .fold(0u128, |v, (&x, y)| (v + x as u128 * y as u128) % p as u128)
                as u64
        };
        if (0..n).any(|j| dot(s.rows.iter().map(|row| row[j]).collect()) != 0)
            || dot(s.a.clone()) != w.a
            || dot(s.b.clone()) != w.b
            || ((w.a as u128 + d as u128 * w.b as u128) % p as u128) != 0
        {
            return Err("original-row dependency fails".into());
        }
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    const PRIMES: [u64; 2] = [2147483647, 9007199254740881];
    #[test]
    fn formal_labels_never_extract_and_augmented_span_is_preserved() {
        for p in PRIMES {
            let rows = vec![vec![1, 2], vec![3, 4], vec![4, 6]];
            let dot = |r: &Vec<u64>, x: u64, y: u64| {
                ((r[0] as u128 * x as u128 + r[1] as u128 * y as u128) % p as u128) as u64
            };
            let s = LabelSystem {
                scalar_prime: p,
                columns: 2,
                a: rows.iter().map(|r| dot(r, 5, 7)).collect(),
                b: rows.iter().map(|r| dot(r, 11, 13)).collect(),
                rows,
            };
            let a = audit(&s, Limits::default()).unwrap();
            assert_eq!((a.rank_r, a.rank_augmented), (2, 2));
            assert!(a.candidate_scalar.is_none());
            assert_eq!(a.selected_original_rows.len(), 2);
        }
    }
    #[test]
    fn homogeneous_connector_is_needed_for_target_dependency() {
        for p in PRIMES {
            for d in [1, 37, p - 1] {
                let s = LabelSystem {
                    scalar_prime: p,
                    columns: 2,
                    rows: vec![vec![1, p - 2], vec![0, 1], vec![1, 0]],
                    a: vec![0, 5, (10 + p - d) % p],
                    b: vec![0, 0, 1],
                };
                let a = audit(&s, Limits::default()).unwrap();
                assert_eq!((a.rank_r, a.rank_r_b, a.rank_augmented), (2, 3, 3));
                assert_eq!(a.candidate_scalar, Some(d));
                assert!(!a.curve_scalar_verified);
                assert_eq!(a.selected_original_rows.len(), 3);
                let without = LabelSystem {
                    rows: s.rows[1..].to_vec(),
                    a: s.a[1..].to_vec(),
                    b: s.b[1..].to_vec(),
                    ..s.clone()
                };
                assert!(audit(&without, Limits::default())
                    .unwrap()
                    .candidate_scalar
                    .is_none());
                let mut bad = a.clone();
                bad.dependency.as_mut().unwrap().multipliers.fill(0);
                assert!(replay(&s, &bad, Limits::default()).is_err());
                let mut bad = a.clone();
                bad.selected_original_rows.pop();
                assert!(replay(&s, &bad, Limits::default()).is_err());
                let mut bad = a.clone();
                bad.candidate_scalar = Some((d + 1) % p);
                assert!(replay(&s, &bad, Limits::default()).is_err());
            }
        }
    }
    #[test]
    fn detects_inconsistent_scalar_systems_even_when_rank_gain_is_only_one() {
        for p in PRIMES {
            for (a, b, full) in [(vec![1], vec![0], 1), (vec![1, 0], vec![0, 1], 2)] {
                let s = LabelSystem {
                    scalar_prime: p,
                    columns: 0,
                    rows: vec![vec![]; a.len()],
                    a,
                    b,
                };
                let out = audit(&s, Limits::default()).unwrap();
                assert!(!out.compatible_with_one_scalar);
                assert_eq!(out.rank_augmented, full);
                assert!(out.candidate_scalar.is_none());
            }
        }
    }
    #[test]
    fn empty_zero_and_malformed_inputs_are_explicit() {
        let s = LabelSystem {
            scalar_prime: 2147483647,
            columns: 0,
            rows: vec![],
            a: vec![],
            b: vec![],
        };
        let a = audit(&s, Limits::default()).unwrap();
        assert_eq!(a.rank_r, 0);
        assert!(a.compatible_with_one_scalar);
        let mut bad = s.clone();
        bad.scalar_prime = 15;
        assert!(audit(&bad, Limits::default()).is_err());
        let mut bad = s.clone();
        bad.rows = vec![vec![1]];
        bad.a = vec![0];
        bad.b = vec![0];
        assert!(audit(&bad, Limits::default()).is_err());
        let bad = LabelSystem {
            scalar_prime: s.scalar_prime,
            columns: 1,
            rows: vec![vec![s.scalar_prime]],
            a: vec![0],
            b: vec![0],
        };
        assert!(audit(&bad, Limits::default()).is_err());
        let bad = LabelSystem {
            rows: vec![vec![]],
            a: vec![0],
            b: vec![0],
            ..s
        };
        assert!(audit(
            &bad,
            Limits {
                max_provenance_cells: 0,
                ..Limits::default()
            }
        )
        .is_err());
    }
}
