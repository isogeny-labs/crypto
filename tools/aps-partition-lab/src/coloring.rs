use serde::Serialize;
use std::collections::BTreeSet;

pub const MAX_POINTS: usize = 12;
pub const MAX_FAMILY: usize = 32;

#[derive(Debug, Serialize)]
pub struct Coverage {
    pub points: usize,
    pub arity: usize,
    pub family_size: usize,
    pub total_tuples: usize,
    pub covered: usize,
    pub owner_counts: Vec<usize>,
    pub uncovered: Vec<Vec<usize>>,
    #[serde(skip_serializing)]
    pub owners: Vec<(Vec<usize>, usize)>,
}

fn validate(n: usize, m: usize, family: &[Vec<usize>]) -> Result<(), String> {
    if !(3..=4).contains(&m) || !(m..=MAX_POINTS).contains(&n) {
        return Err("arity must be 3/4 and arity <= points <= 12".into());
    }
    if family.is_empty() || family.len() > MAX_FAMILY {
        return Err("family must contain 1..=32 colorings".into());
    }
    if family
        .iter()
        .any(|c| c.len() != n || c.iter().any(|&v| v >= m))
    {
        return Err("each coloring must assign every ID a color below arity".into());
    }
    Ok(())
}

pub(crate) fn tuples(n: usize, m: usize) -> Vec<Vec<usize>> {
    fn visit(n: usize, m: usize, start: usize, acc: &mut Vec<usize>, out: &mut Vec<Vec<usize>>) {
        if acc.len() == m {
            out.push(acc.clone());
            return;
        }
        for i in start..n {
            acc.push(i);
            visit(n, m, i + 1, acc, out);
            acc.pop();
        }
    }
    let mut out = Vec::new();
    visit(n, m, 0, &mut Vec::new(), &mut out);
    out
}

/// Fixed before observing coverage. This mixer is not a cryptographic hash.
pub fn candidate_family(n: usize, m: usize, size: usize) -> Result<Vec<Vec<usize>>, String> {
    if size == 0 || size > MAX_FAMILY || !(3..=4).contains(&m) || !(m..=MAX_POINTS).contains(&n) {
        return Err("candidate-family bounds exceeded".into());
    }
    fn mix(mut z: u64) -> u64 {
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
        z ^ (z >> 31)
    }
    Ok((0..size)
        .map(|k| {
            (0..n)
                .map(|id| {
                    if k == 0 {
                        id % m
                    } else {
                        (mix((id as u64).wrapping_add((k as u64).wrapping_mul(0x9e3779b97f4a7c15)))
                            % m as u64) as usize
                    }
                })
                .collect()
        })
        .collect())
}

/// Production traversal: unordered tuples, then first distinct-color mask.
pub fn audit(n: usize, m: usize, family: &[Vec<usize>]) -> Result<Coverage, String> {
    validate(n, m, family)?;
    let all = tuples(n, m);
    let mut owners = Vec::new();
    let mut uncovered = Vec::new();
    let mut owner_counts = vec![0; family.len()];
    for tuple in &all {
        let owner = family.iter().position(|c| {
            tuple.iter().fold(0usize, |mask, &id| mask | (1 << c[id])) == (1 << m) - 1
        });
        if let Some(i) = owner {
            owner_counts[i] += 1;
            owners.push((tuple.clone(), i));
        } else {
            uncovered.push(tuple.clone());
        }
    }
    let expected = reference(n, m, family)?;
    let actual: BTreeSet<_> = owners.iter().map(|(t, _)| t.clone()).collect();
    if actual != expected || owners.len() != actual.len() {
        return Err("first-owner coverage disagrees with independent cartesian traversal".into());
    }
    Ok(Coverage {
        points: n,
        arity: m,
        family_size: family.len(),
        total_tuples: all.len(),
        covered: owners.len(),
        owner_counts,
        uncovered,
        owners,
    })
}

/// Independent reference: cartesian products of color cells, then canonicalize.
pub fn reference(
    n: usize,
    m: usize,
    family: &[Vec<usize>],
) -> Result<BTreeSet<Vec<usize>>, String> {
    validate(n, m, family)?;
    fn product(
        cells: &[Vec<usize>],
        at: usize,
        acc: &mut Vec<usize>,
        out: &mut BTreeSet<Vec<usize>>,
    ) {
        if at == cells.len() {
            let mut key = acc.clone();
            key.sort_unstable();
            out.insert(key);
        } else {
            for &id in &cells[at] {
                acc.push(id);
                product(cells, at + 1, acc, out);
                acc.pop();
            }
        }
    }
    let mut out = BTreeSet::new();
    for c in family {
        let cells: Vec<Vec<_>> = (0..m)
            .map(|color| (0..n).filter(|&id| c[id] == color).collect())
            .collect();
        product(&cells, 0, &mut Vec::new(), &mut out);
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_bounded_populations_match_independent_products_and_partition_the_universe() {
        for m in [3, 4] {
            for n in m..=12 {
                for count in [1, 8, 32] {
                    let f = candidate_family(n, m, count).unwrap();
                    let a = audit(n, m, &f).unwrap();
                    assert_eq!(a.covered + a.uncovered.len(), tuples(n, m).len());
                    assert_eq!(a.owner_counts.iter().sum::<usize>(), a.covered);
                    for (t, owner) in &a.owners {
                        for previous in &f[..*owner] {
                            assert!((0..m)
                                .any(|i| (i + 1..m).any(|j| previous[t[i]] == previous[t[j]])));
                        }
                        assert!(reference(n, m, &f[*owner..=*owner]).unwrap().contains(t));
                    }
                }
            }
        }
    }

    #[test]
    fn duplicate_colorings_never_add_owners_and_color_renaming_preserves_tuples() {
        let f = candidate_family(12, 4, 8).unwrap();
        let a = audit(12, 4, &f).unwrap();
        let doubled: Vec<_> = f.iter().chain(f.iter()).cloned().collect();
        let b = audit(12, 4, &doubled).unwrap();
        assert_eq!(a.owners, b.owners);
        let renamed: Vec<Vec<_>> = f
            .iter()
            .map(|c| c.iter().map(|v| (v + 1) % 4).collect())
            .collect();
        assert_eq!(a.owners, audit(12, 4, &renamed).unwrap().owners);
    }

    #[test]
    fn missing_tuples_are_retained_and_invalid_inputs_refused() {
        let a = audit(4, 3, &[vec![0, 0, 1, 2]]).unwrap();
        assert!(a.uncovered.contains(&vec![0, 1, 2]));
        assert!(audit(4, 3, &[vec![0, 1]]).is_err());
        assert!(audit(4, 3, &[vec![0, 1, 2, 3]]).is_err());
        assert!(candidate_family(13, 3, 8).is_err());
        assert!(candidate_family(8, 4, 33).is_err());
    }
}
