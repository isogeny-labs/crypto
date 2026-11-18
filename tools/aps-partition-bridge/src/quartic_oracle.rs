use aps_partition_lab::curve::{Curve, Point};
use crypto_lib::cryptanalysis::aps_single_relation::LabeledBase;
use std::collections::BTreeSet;

/// Ordinary affine arithmetic, independent of polynomial construction/F4.
pub fn incidents(base: &LabeledBase, tuple: &[u64]) -> Result<(usize, usize), String> {
    if tuple.len() != 4 || tuple.windows(2).any(|w| w[0] >= w[1]) {
        return Err("quartic oracle requires four sorted distinct coordinates".into());
    }
    let curve = Curve::new(101, 59, 6)?;
    let entries: Vec<_> = tuple
        .iter()
        .map(|x| base.points.get(x).ok_or("missing oracle coordinate"))
        .collect::<Result<_, _>>()?;
    let mut informative = 0;
    let mut zero = 0;
    for mask in 0..8 {
        let mut sum = None;
        let mut b = 0;
        for (i, p) in entries.iter().enumerate() {
            let point = Point { x: p.x, y: p.y };
            if !curve.contains(point) || point.y == 0 || p.v >= 107 {
                return Err("invalid quartic oracle fixture".into());
            }
            let negative = i > 0 && mask & (1 << (i - 1)) != 0;
            sum = curve.add(sum, Some(if negative { curve.neg(point) } else { point }));
            b = (b + if negative { (107 - p.v) % 107 } else { p.v }) % 107;
        }
        if sum.is_none() {
            if b == 0 {
                zero += 1;
            } else {
                informative += 1;
            }
        }
    }
    Ok((informative, zero))
}

pub fn roots(
    base: &LabeledBase,
    tuples: &BTreeSet<Vec<u64>>,
) -> Result<BTreeSet<Vec<u64>>, String> {
    let mut roots = BTreeSet::new();
    for tuple in tuples {
        let (informative, zero) = incidents(base, tuple)?;
        if informative + zero > 0 {
            roots.insert(tuple.clone());
        }
    }
    Ok(roots)
}
