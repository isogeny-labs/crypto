//! Post-run trace audit. Scalar multiplication builds the action maps here;
//! the collector's Frobenius/psi tables and pair canonicalizer are not called.
use super::*;

pub(super) type SignedMap = BTreeMap<(usize, i8), (usize, i8)>;

pub(super) fn scalar_maps(case: &Case, arm: usize) -> Result<Vec<SignedMap>, &'static str> {
    let usable = case.usable(arm);
    let mut points = BTreeMap::new();
    for &id in &usable {
        for sign in [-1, 1] {
            let p = signed(&case.c, case.pool[id].p.into(), sign);
            if p.inf || points.insert((p.x, p.y), (id, sign)).is_some() {
                return Err("audit-point-domain");
            }
        }
    }
    let mut scalars = BTreeSet::new();
    for j in 0..4 {
        for k in 0..3 {
            let a = mm(fpow(case.lf, j, case.n), fpow(case.lc, k, case.n), case.n);
            scalars.extend([a, negn(a, case.n)]);
        }
    }
    if scalars.len() != 24 || scalars.contains(&0) {
        return Err("audit-action-scalars");
    }
    for &a in &scalars {
        for &b in &scalars {
            if !scalars.contains(&mm(a, b, case.n)) {
                return Err("audit-action-closure");
            }
        }
    }
    let mut result = Vec::new();
    for scalar in scalars {
        let mut map = SignedMap::new();
        for &id in &usable {
            let image = case.c.mul(case.pool[id].p.into(), scalar);
            for sign in [-1, 1] {
                let p = signed(&case.c, image, sign);
                let &to = points.get(&(p.x, p.y)).ok_or("audit-action-closure")?;
                if (id < case.l0) != (to.0 < case.l0) {
                    return Err("audit-action-component");
                }
                map.insert((id, sign), to);
            }
        }
        if map.values().copied().collect::<BTreeSet<_>>().len() != points.len() {
            return Err("audit-action-bijection");
        }
        result.push(map);
    }
    Ok(result)
}

fn pair_orbit(pair: Pair, maps: &[SignedMap]) -> Result<BTreeSet<Pair>, &'static str> {
    maps.iter()
        .map(|map| {
            let &(i, si) = map.get(&(pair.0, pair.1)).ok_or("pair-coverage")?;
            let &(j, sj) = map.get(&(pair.2, pair.3)).ok_or("pair-coverage")?;
            Ok((i, si, j, sj))
        })
        .collect()
}

pub(super) fn partition(
    pairs: &[Pair],
    ordinary: &[usize],
    images: &[usize],
    maps: &[SignedMap],
) -> Result<usize, &'static str> {
    if pairs.windows(2).any(|w| w[0] >= w[1]) {
        return Err("pair-coverage");
    }
    let mut remaining = BTreeSet::new();
    for &i in ordinary {
        for &j in images {
            for si in [-1, 1] {
                for sj in [-1, 1] {
                    remaining.insert((i, si, j, sj));
                }
            }
        }
    }
    let total = remaining.len();
    for &pair in pairs {
        let orbit = pair_orbit(pair, maps)?;
        if orbit.len() != maps.len() || orbit.first() != Some(&pair) {
            return Err("pair-coverage");
        }
        for image in orbit {
            if !remaining.remove(&image) {
                return Err("pair-coverage");
            }
        }
    }
    if !remaining.is_empty() {
        return Err("pair-coverage");
    }
    Ok(total)
}

pub(super) fn inspect_records(
    case: &Case,
    arm: usize,
    value: &Value,
    expected: &BTreeSet<Terms>,
) -> Result<Value, &'static str> {
    let records = value["systems"].as_array().ok_or("system-record-count")?;
    if Some(records.len() as u64) != value["systems_attempted"].as_u64() {
        return Err("system-record-count");
    }
    let pairs: Vec<Pair> = records
        .iter()
        .map(|r| serde_json::from_value(r["pair"].clone()).map_err(|_| "pair-coverage"))
        .collect::<Result<_, _>>()?;
    if pairs.windows(2).any(|w| w[0] >= w[1]) {
        return Err("pair-coverage");
    }
    let saved_work = records.iter().try_fold(0u64, |sum, r| {
        sum.checked_add(
            r["f4_row_reduction_fp_muls"]
                .as_u64()
                .ok_or("f4-work-sum")?,
        )
        .ok_or("f4-work-sum")
    })?;
    if Some(saved_work) != value["f4_row_reduction_fp_muls"].as_u64() {
        return Err("f4-work-sum");
    }
    let maps = scalar_maps(case, arm)?;
    let ordinary: Vec<_> = (0..case.l0).collect();
    let images: Vec<_> = case
        .usable(arm)
        .into_iter()
        .filter(|&id| id >= case.l0)
        .collect();
    let covered = partition(&pairs, &ordinary, &images, &maps)?;
    if covered as u64 != value["signed_pairs"].as_u64().ok_or("pair-coverage")? {
        return Err("pair-coverage");
    }
    let (_, h) = integer::derive();
    let mut transported = BTreeSet::new();
    let mut work = 0u64;
    let mut accepted_occurrences = 0usize;
    for (record, &pair) in records.iter().zip(&pairs) {
        work = work
            .checked_add(
                record["f4_row_reduction_fp_muls"]
                    .as_u64()
                    .ok_or("f4-work-sum")?,
            )
            .ok_or("f4-work-sum")?;
        if record["max_rows"].as_u64().is_none() || record["max_cols"].as_u64().is_none() {
            return Err("system-work-shape");
        }
        let r = case.c.neg(case.c.add(
            signed(&case.c, case.pool[pair.0].p.into(), pair.1),
            signed(&case.c, case.pool[pair.2].p.into(), pair.3),
        ));
        let saved: PointRecord = serde_json::from_value(record["remaining_point"].clone())
            .map_err(|_| "remaining-point")?;
        if r.inf || Point::from(saved) != r {
            return Err("remaining-point");
        }
        // This equation derivation is shared. Independent physical completeness
        // comes from the immutable point-replayed census, not from this equality.
        let equations = restrict_exact(case.c.f, case.c.b, &h, r.x);
        let encoded: Vec<Vec<Value>> = equations
            .iter()
            .map(|p| p.iter().map(|(e, c)| json!([e, c])).collect())
            .collect();
        if record["equations"] != json!(encoded) {
            return Err("system-equations");
        }
        let roots: Vec<Vec<u64>> =
            serde_json::from_value(record["field_roots"].clone()).map_err(|_| "system-roots")?;
        if roots.windows(2).any(|w| w[0] >= w[1])
            || roots.iter().any(|e| {
                e.len() != 3
                    || e.iter().any(|&a| a >= case.p)
                    || equations
                        .iter()
                        .any(|p| polynomial_value(p, e, case.p) != 0)
            })
        {
            return Err("system-roots");
        }
        let accepted: BTreeSet<Terms> =
            serde_json::from_value(record["accepted_representative_relations"].clone())
                .map_err(|_| "representative-relations")?;
        let reference: BTreeSet<Terms> = expected
            .iter()
            .filter(|terms| terms.contains(&(pair.0, pair.1)) && terms.contains(&(pair.2, pair.3)))
            .cloned()
            .collect();
        if accepted != reference
            || record["accepted_representative_relations"]
                .as_array()
                .map(Vec::len)
                != Some(accepted.len())
        {
            return Err("representative-relations");
        }
        match record["verdict"].as_str() {
            Some("inconsistent") if roots.is_empty() && accepted.is_empty() => {}
            Some("complete-field-roots") => {}
            _ => return Err("system-verdict"),
        }
        for terms in &reference {
            let xs: Vec<_> = terms
                .iter()
                .filter(|&&(id, _)| id != pair.0 && id != pair.2)
                .map(|&(id, _)| case.pool[id].p.x[0])
                .collect();
            if xs.len() != 3 {
                return Err("representative-root");
            }
            let p = case.p as u128;
            let a = xs[0] as u128;
            let b = xs[1] as u128;
            let c = xs[2] as u128;
            let e = vec![
                ((a + b + c) % p) as u64,
                ((a * b + a * c + b * c) % p) as u64,
                ((a * b * c) % p) as u64,
            ];
            if roots.binary_search(&e).is_err() {
                return Err("representative-root");
            }
            for map in &maps {
                let mut image: Terms = terms.iter().map(|t| map[t]).collect();
                image.sort();
                if !zero_sum(case, &image) {
                    return Err("transported-group-relation");
                }
                transported.insert(image);
            }
        }
        accepted_occurrences += accepted.len();
    }
    if Some(work) != value["f4_row_reduction_fp_muls"].as_u64() {
        return Err("f4-work-sum");
    }
    if &transported != expected {
        return Err("transported-set");
    }
    for key in ["setup_work", "collection_and_matrix_work"] {
        serde_json::from_value::<meter::Counts>(value[key].clone())
            .map_err(|_| "arithmetic-work-shape")?;
    }
    Ok(
        json!({"signed_pairs_independently_covered":covered,"disjoint_free_pair_orbits":pairs.len(),
        "scalar_action_maps":maps.len(),"systems_inspected":records.len(),
        "accepted_representative_occurrences":accepted_occurrences,"transported_union_relations":transported.len(),
        "f4_row_reduction_fp_muls_checked_sum":work,
        "scope":"Scalar-multiplication action maps, exact signed-pair partition, immutable per-pair physical-set equality and transported union; separate RREF is checked by the caller. Shared field/curve kernel and equation deriver. Counter-shape/sum checks do not calibrate hashes, allocator CPU or complete IC prices."}),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn independent_partition_rejects_missing_and_duplicate_orbits() {
        let mut maps = Vec::new();
        for rotation in 0..3 {
            for sign in [-1, 1] {
                let mut m = SignedMap::new();
                for id in 0..6 {
                    for s in [-1, 1] {
                        m.insert((id, s), (id / 3 * 3 + (id % 3 + rotation) % 3, s * sign));
                    }
                }
                maps.push(m);
            }
        }
        let pairs = vec![
            (0, -1, 3, -1),
            (0, -1, 3, 1),
            (0, -1, 4, -1),
            (0, -1, 4, 1),
            (0, -1, 5, -1),
            (0, -1, 5, 1),
        ];
        assert_eq!(partition(&pairs, &[0, 1, 2], &[3, 4, 5], &maps), Ok(36));
        assert_eq!(
            partition(&pairs[..5], &[0, 1, 2], &[3, 4, 5], &maps),
            Err("pair-coverage")
        );
        let mut duplicate = pairs.clone();
        duplicate[1] = duplicate[0];
        assert_eq!(
            partition(&duplicate, &[0, 1, 2], &[3, 4, 5], &maps),
            Err("pair-coverage")
        );
    }
}
