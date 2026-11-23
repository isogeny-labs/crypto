use crate::{sampler::Fixture, valuation};
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Serialize)]
pub struct Cell {
    pub absolute_labels: Vec<u64>,
    pub parts: Vec<Vec<u64>>,
    pub valuation_safe: bool,
    pub tuples: Vec<Vec<u64>>,
}

/// Enumerate the coordinate product, rejecting collisions between slots.
pub fn product(parts: &[Vec<u64>]) -> Result<BTreeSet<Vec<u64>>, String> {
    if parts.len() != 4 || parts.iter().any(|p| p.is_empty() || p.len() > 2) {
        return Err("quartic cell requires four nonempty slots of size at most two".into());
    }
    let mut seen = BTreeSet::new();
    if parts.iter().flatten().any(|x| !seen.insert(*x)) {
        return Err("quartic slots must be pairwise disjoint and unique".into());
    }
    let mut out = BTreeSet::new();
    for &a in &parts[0] {
        for &b in &parts[1] {
            for &c in &parts[2] {
                for &d in &parts[3] {
                    let mut tuple = vec![a, b, c, d];
                    tuple.sort_unstable();
                    out.insert(tuple);
                }
            }
        }
    }
    Ok(out)
}

/// Separate combination traversal, independent of the label-cell construction.
pub fn universe(f: &Fixture) -> BTreeSet<Vec<u64>> {
    let x: Vec<_> = f.base.points.keys().copied().collect();
    let mut out = BTreeSet::new();
    for a in 0..x.len() {
        for b in a + 1..x.len() {
            for c in b + 1..x.len() {
                for d in c + 1..x.len() {
                    out.insert(vec![x[a], x[b], x[c], x[d]]);
                }
            }
        }
    }
    out
}

pub fn validate(f: &Fixture, cells: &[Cell]) -> Result<(), String> {
    let mut union = BTreeSet::new();
    for cell in cells {
        let actual = product(&cell.parts)?;
        if actual != cell.tuples.iter().cloned().collect() {
            return Err("cell tuple metadata disagrees with coordinate product".into());
        }
        for tuple in actual {
            if !union.insert(tuple) {
                return Err("quartic cells overlap".into());
            }
        }
    }
    if union != universe(f) {
        return Err("quartic cells do not cover the complete tuple universe".into());
    }
    Ok(())
}

/// Fixed two-points-per-absolute-label profile. Repeated labels use both
/// coordinates as ordered singletons; unique labels retain both choices.
pub fn plan(f: &Fixture) -> Result<Vec<Cell>, String> {
    let mut groups: BTreeMap<u64, Vec<u64>> = BTreeMap::new();
    for p in f.base.points.values() {
        if p.v >= f.curve.n {
            return Err("label outside subgroup".into());
        }
        groups
            .entry(p.v.min(f.curve.n - p.v))
            .or_default()
            .push(p.x);
    }
    if groups.keys().copied().collect::<Vec<_>>() != [1, 2, 4, 8]
        || groups.values().any(|v| v.len() != 2)
    {
        return Err("quartic planner requires two points of each absolute label1/2/4/8".into());
    }
    let groups: Vec<_> = groups.into_iter().collect();
    let mut cells = Vec::new();
    for a in 0..=2 {
        for b in 0..=2 {
            for c in 0..=2 {
                for d in 0..=2 {
                    let counts = [a, b, c, d];
                    if counts.iter().sum::<usize>() != 4 {
                        continue;
                    }
                    let mut parts = Vec::new();
                    let mut labels = Vec::new();
                    for (&count, (label, xs)) in counts.iter().zip(&groups) {
                        for i in 0..count {
                            labels.push(*label);
                            parts.push(if count == 2 { vec![xs[i]] } else { xs.clone() });
                        }
                    }
                    let tuples = product(&parts)?.into_iter().collect();
                    cells.push(Cell {
                        valuation_safe: valuation::guaranteed_nonzero(&labels, f.curve.n),
                        absolute_labels: labels,
                        parts,
                        tuples,
                    });
                }
            }
        }
    }
    validate(f, &cells)?;
    Ok(cells)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sampler;

    #[test]
    fn full_cover_is_disjoint_and_rejects_omission_overlap_and_slot_collision() {
        for seed in sampler::SEEDS {
            let f = sampler::even_fixture(seed).unwrap();
            let cells = plan(&f).unwrap();
            assert_eq!(cells.len(), 19);
            assert_eq!(cells.iter().map(|c| c.tuples.len()).sum::<usize>(), 70);
            assert_eq!(cells.iter().filter(|c| c.valuation_safe).count(), 9);
            assert_eq!(
                cells
                    .iter()
                    .filter(|c| c.valuation_safe)
                    .map(|c| c.tuples.len())
                    .sum::<usize>(),
                48
            );
            assert!(validate(&f, &cells[1..]).is_err());
            let mut doubled = cells.clone();
            doubled.push(cells[0].clone());
            assert!(validate(&f, &doubled).is_err());
            let mut bad = cells[0].parts.clone();
            bad[1] = bad[0].clone();
            assert!(product(&bad).is_err());
        }
    }
}
