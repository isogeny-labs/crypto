use aps_partition_lab::curve::{Curve, Point};
use crypto_lib::cryptanalysis::aps_single_relation::LabeledBase;
use std::collections::BTreeSet;

/// Cartesian coordinate traversal plus the separate ordinary affine oracle.
/// It does not inspect a polynomial, a cached trace or a recovered logarithm.
pub fn roots(base: &LabeledBase, parts: &[Vec<u64>]) -> Result<BTreeSet<Vec<u64>>, String> {
    if parts.len() != 3 || parts.iter().any(|p| p.len() > 12) {
        return Err("oracle bounds exceeded".into());
    }
    let curve = Curve::new(101, 59, 6)?;
    let mut out = BTreeSet::new();
    for &x in &parts[0] {
        for &y in &parts[1] {
            for &z in &parts[2] {
                if x == y || x == z || y == z {
                    return Err("oracle cell repeats an abscissa".into());
                }
                let coordinates = [x, y, z];
                let mut points = Vec::new();
                for coordinate in coordinates {
                    let p = base
                        .points
                        .get(&coordinate)
                        .ok_or("oracle coordinate outside labeled base")?;
                    let point = Point { x: p.x, y: p.y };
                    if !curve.contains(point) || point.y == 0 {
                        return Err("invalid nonzero-ordinate fixture point".into());
                    }
                    points.push(point);
                }
                if curve.has_signed_zero(&points) {
                    let mut tuple = coordinates.to_vec();
                    tuple.sort_unstable();
                    out.insert(tuple);
                }
            }
        }
    }
    Ok(out)
}

pub fn full_roots(base: &LabeledBase) -> Result<BTreeSet<Vec<u64>>, String> {
    let values: Vec<_> = base.points.keys().copied().collect();
    let mut out = BTreeSet::new();
    for i in 0..values.len() {
        for j in i + 1..values.len() {
            for k in j + 1..values.len() {
                out.extend(roots(
                    base,
                    &[vec![values[i]], vec![values[j]], vec![values[k]]],
                )?);
            }
        }
    }
    Ok(out)
}

/// Count every represented sign incident; never inherit the old slot guarantee.
pub fn label_incidents(
    base: &LabeledBase,
    sets: &BTreeSet<Vec<u64>>,
) -> Result<(usize, usize), String> {
    let curve = Curve::new(101, 59, 6)?;
    let mut informative = 0;
    let mut zero = 0;
    for tuple in sets {
        if tuple.len() != 3 {
            return Err("label oracle requires triples".into());
        }
        for mask in 0..4usize {
            let mut sum = None;
            let mut b = 0;
            for (i, x) in tuple.iter().enumerate() {
                let p = base.points.get(x).ok_or("label oracle missing point")?;
                if p.v >= 107 {
                    return Err("label oracle coefficient outside subgroup".into());
                }
                let point = Point { x: p.x, y: p.y };
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
    }
    Ok((informative, zero))
}
