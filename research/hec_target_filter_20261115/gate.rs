use hec_native_ic::{
    fp::Fp,
    rng::Rng,
    solve::{solve_fast, Row, SolveStats},
};
use std::collections::{BTreeMap, VecDeque};
type Equation = (Row, u64);

pub fn normalize(rows: &[Equation], f: &Fp) -> Vec<Equation> {
    rows.iter()
        .map(|(row, rhs)| {
            let mut terms = BTreeMap::new();
            for &(j, v) in row {
                let value = f.add(terms.get(&j).copied().unwrap_or(0), v % f.p);
                terms.insert(j, value);
            }
            (
                terms.into_iter().filter(|&(_, v)| v != 0).collect(),
                rhs % f.p,
            )
        })
        .collect()
}
pub struct Filtered {
    pub rows: Vec<Equation>,
    pub pivots: Vec<(usize, usize)>,
    pub normalized: Vec<Equation>,
}
/// Exact singleton elimination; the target is never an eligible pivot.
pub fn peel(rows: &[Equation], n: usize, target: usize, f: &Fp) -> Filtered {
    assert!(target < n);
    let rows = normalize(rows, f);
    let mut holders = vec![Vec::new(); n];
    for (i, (row, _)) in rows.iter().enumerate() {
        for &(j, _) in row {
            assert!((j as usize) < n);
            holders[j as usize].push(i);
        }
    }
    let mut counts: Vec<_> = holders.iter().map(Vec::len).collect();
    let mut live = vec![true; rows.len()];
    let mut queue = VecDeque::new();
    let mut pivots = Vec::new();
    for (j, &count) in counts.iter().enumerate() {
        if j != target && count == 1 {
            queue.push_back(j);
        }
    }
    while let Some(j) = queue.pop_front() {
        if j == target || counts[j] != 1 {
            continue;
        }
        let i = *holders[j]
            .iter()
            .find(|&&i| live[i])
            .expect("singleton holder");
        live[i] = false;
        pivots.push((i, j));
        for &(c, _) in &rows[i].0 {
            let c = c as usize;
            assert!(counts[c] > 0);
            counts[c] -= 1;
            if c != target && counts[c] == 1 {
                queue.push_back(c);
            }
        }
    }
    let kept = rows
        .iter()
        .enumerate()
        .filter(|(i, _)| live[*i])
        .map(|(_, r)| r.clone())
        .collect();
    Filtered {
        rows: kept,
        pivots,
        normalized: rows,
    }
}

fn feasible(rows: &[Equation], n: usize, target: usize, f: &Fp) -> u128 {
    let mut mask = 0;
    for mut encoded in 0..f.p.pow(n as u32) {
        let values: Vec<_> = (0..n)
            .map(|_| {
                let v = encoded % f.p;
                encoded /= f.p;
                v
            })
            .collect();
        if rows.iter().all(|(row, rhs)| {
            row.iter()
                .fold(0, |s, &(j, c)| f.add(s, f.mul(c % f.p, values[j as usize])))
                == rhs % f.p
        }) {
            mask |= 1u128 << values[target];
        }
    }
    mask
}

fn main() {
    let f = Fp::new(5);
    let a = vec![(vec![(0, 1), (1, 1), (2, 1)], 0), (vec![(0, 1), (1, 1)], 0)];
    let b = vec![(vec![(0, 1), (1, 1), (2, 1)], 0), (vec![(0, 1), (1, 2)], 0)];
    assert_eq!(
        a.iter()
            .map(|r| r.0.iter().map(|e| e.0).collect::<Vec<_>>())
            .collect::<Vec<_>>(),
        b.iter()
            .map(|r| r.0.iter().map(|e| e.0).collect::<Vec<_>>())
            .collect::<Vec<_>>()
    );
    assert_eq!(feasible(&a, 3, 2, &f), 1);
    assert_eq!(feasible(&b, 3, 2, &f), 31);
    assert_eq!(solve_fast(&a, 2, 5, &mut SolveStats::default()), Some(0));
    assert_eq!(solve_fast(&b, 2, 5, &mut SolveStats::default()), None);
    let target_only = vec![(vec![(0, 1)], 3)];
    let kept = peel(&target_only, 1, 0, &f);
    assert_eq!(kept.rows, target_only);
    assert_eq!(feasible(&target_only, 1, 0, &f), 8);
    assert_eq!(feasible(&[], 1, 0, &f), 31);
    let cancellation = vec![(vec![(0, 1), (0, 4), (1, 1)], 2)];
    assert_eq!(peel(&cancellation, 2, 1, &f).rows, vec![(vec![(1, 1)], 2)]);
    let inconsistent = vec![(vec![], 1)];
    assert_eq!(peel(&inconsistent, 3, 2, &f).rows, inconsistent);
    println!("CONTROLS same_support_masks=1,31 target_guard_masks=8,31 cancellation=true inconsistent_zero_row=true");

    let (mut systems, mut removed, mut empty) = (0, 0, 0);
    for n in 1..=3 {
        for seed in 0..128 {
            let mut rng = Rng::new(2026111500 + seed);
            let mut rows = Vec::new();
            for _ in 0..seed % 7 {
                let row = (0..rng.below(6))
                    .map(|_| (rng.below(n as u64) as u32, rng.below(5)))
                    .collect();
                rows.push((row, rng.below(5)));
            }
            let before = feasible(&rows, n, n - 1, &f);
            let kept = peel(&rows, n, n - 1, &f);
            assert_eq!(before, feasible(&kept.rows, n, n - 1, &f));
            // Every satisfying remaining assignment extends through reverse pivots.
            for mut encoded in 0..5u64.pow(n as u32) {
                let mut values: Vec<_> = (0..n)
                    .map(|_| {
                        let v = encoded % 5;
                        encoded /= 5;
                        v
                    })
                    .collect();
                if !kept.rows.iter().all(|(r, b)| {
                    r.iter()
                        .fold(0, |s, &(j, c)| f.add(s, f.mul(c, values[j as usize])))
                        == *b
                }) {
                    continue;
                }
                let target_value = values[n - 1];
                for &(i, j) in kept.pivots.iter().rev() {
                    let (row, rhs) = &kept.normalized[i];
                    let coefficient = row.iter().find(|e| e.0 as usize == j).unwrap().1;
                    let other = row
                        .iter()
                        .filter(|e| e.0 as usize != j)
                        .fold(0, |s, &(j, c)| f.add(s, f.mul(c, values[j as usize])));
                    values[j] = f.mul(f.sub(*rhs, other), f.inv(coefficient));
                }
                assert_eq!(values[n - 1], target_value);
                assert!(rows.iter().all(|(r, b)| r
                    .iter()
                    .fold(0, |s, &(j, c)| f.add(s, f.mul(c, values[j as usize])))
                    == *b));
            }
            systems += 1;
            removed += kept.pivots.len();
            empty += usize::from(before == 0);
        }
    }
    println!("EXHAUSTIVE systems={systems} inconsistent={empty} removed_rows={removed} target_sets_and_extensions=true");

    let (mut prefixes, mut removed) = (0, 0);
    for ell in [5, 7, 101] {
        let f = Fp::new(ell);
        for n in [1, 3, 8, 20] {
            for seed in 0..32 {
                let mut rng = Rng::new(99150 + seed);
                let values: Vec<_> = (0..n).map(|_| rng.below(ell)).collect();
                let mut rows = Vec::new();
                for prefix in 0..=2 * n + 4 {
                    if prefix > 0 {
                        let row: Row = (0..rng.below(5))
                            .map(|_| (rng.below(n as u64) as u32, rng.below(ell)))
                            .collect();
                        let rhs = row
                            .iter()
                            .fold(0, |s, &(j, c)| f.add(s, f.mul(c, values[j as usize])));
                        rows.push((row, rhs));
                    }
                    let normalized = normalize(&rows, &f);
                    let kept = peel(&rows, n, n - 1, &f);
                    let before = solve_fast(&normalized, n - 1, ell, &mut SolveStats::default());
                    let after = solve_fast(&kept.rows, n - 1, ell, &mut SolveStats::default());
                    assert_eq!(before, after);
                    if let Some(k) = after {
                        assert_eq!(k, values[n - 1]);
                    }
                    prefixes += 1;
                    removed += kept.pivots.len();
                }
            }
        }
    }
    println!("PREFIXES checked={prefixes} removed_rows={removed} target_results=true");
    println!("TERMINAL exact=true timing_claim=false");
}
