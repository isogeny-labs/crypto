//! Exact, bounded diagnostic of relations discarded by the scalar folded pair table.
//! The frozen protocol is research/rootfind_pair_collision_census_20261116/PROTOCOL.md.

use std::collections::BTreeMap;
use std::time::{Duration, Instant};

use crypto_lib::cryptanalysis::ic_boundary::{
    invmod, koblitz_factor_base, koblitz_instance, BinaryGroup, ColumnFold, PairTable,
};
use crypto_lib::cryptanalysis::koblitz_index_calculus::build_standard_subspace_factor_base;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

const MAX_PAIRS: u64 = 200_000;
const MAX_TIME: Duration = Duration::from_secs(120);

fn selected_cell(arg: &str) -> Option<(u32, u32)> {
    match arg {
        "k13" => Some((13, 5)),
        "k19" => Some((19, 7)),
        "k23" => Some((23, 8)),
        _ => None,
    }
}

fn insert_row(pivots: &mut BTreeMap<usize, Vec<u64>>, mut row: Vec<u64>, modulus: u64) -> bool {
    for (&col, pivot) in pivots.iter() {
        let factor = row[col];
        if factor == 0 {
            continue;
        }
        for k in col..row.len() {
            let sub = ((factor as u128 * pivot[k] as u128) % modulus as u128) as u64;
            row[k] = (row[k] + modulus - sub) % modulus;
        }
    }
    let Some(col) = row.iter().position(|&x| x != 0) else {
        return false;
    };
    let inverse = invmod(row[col], modulus).expect("prime subgroup modulus");
    for x in &mut row[col..] {
        *x = ((*x as u128 * inverse as u128) % modulus as u128) as u64;
    }
    pivots.insert(col, row);
    true
}

fn execute(cell: &str) -> Result<Value, String> {
    let (n, dimension) = selected_cell(cell).ok_or("expected k13, k19, or k23")?;
    let start = Instant::now();
    let inst = koblitz_instance(0, n).ok_or("Koblitz constructor rejected frozen cell")?;
    let kc = inst.koblitz.as_ref().ok_or("missing Koblitz view")?;
    let parent = build_standard_subspace_factor_base(kc, dimension)?;
    let fb = koblitz_factor_base(
        &inst,
        &parent,
        ColumnFold::SignedFrobeniusOrbit,
        format!("standard subspace d={dimension}"),
    )
    .ok_or("base conversion failed")?;
    let group = BinaryGroup(&inst.fast);
    let production = PairTable::build_negation_folded(&group, &fb);

    let mut hasher = Sha256::new();
    for (idx, point) in fb.points.iter().enumerate() {
        hasher.update(point.pack().to_le_bytes());
        hasher.update((fb.col_of[idx] as u64).to_le_bytes());
        hasher.update(fb.coef_of[idx].to_le_bytes());
    }
    let base_sha256 = format!("{:x}", hasher.finalize());

    let groups: Vec<&Vec<usize>> = fb
        .abscissa_list
        .iter()
        .map(|x| fb.x_index.get(x).expect("indexed abscissa"))
        .collect();
    let mut first = BTreeMap::new();
    let mut attempts = 0u64;
    let mut pair_additions = 0u64;
    let mut duplicates = 0u64;
    let mut same_sum = 0u64;
    let mut opposite_sum = 0u64;
    let mut zero_rows = 0u64;
    let mut nonzero_rows = 0u64;
    let mut rank = BTreeMap::<usize, Vec<u64>>::new();
    let mut rows = Vec::<Value>::new();
    let mut negative_control_checked = false;
    let mut status = "complete";

    'outer: for (ai, group_i) in groups.iter().enumerate() {
        let i = group_i[0];
        for group_j in groups.iter().skip(ai) {
            for &j in group_j.iter() {
                if pair_additions >= MAX_PAIRS || start.elapsed() >= MAX_TIME {
                    status = "censored";
                    break 'outer;
                }
                let current = inst.fast.add(fb.points[i], fb.points[j]);
                pair_additions += 1;
                if current.infinity {
                    continue;
                }
                attempts += 1;
                let key = current.pack() >> 1;
                let Some(&(fi, fj, previous)) = first.get(&key) else {
                    first.insert(key, (i, j, current));
                    continue;
                };
                duplicates += 1;
                let indices = if current == previous {
                    same_sum += 1;
                    [i, j, fb.neg_index[fi], fb.neg_index[fj]]
                } else if current == inst.fast.neg(previous) {
                    opposite_sum += 1;
                    [i, j, fi, fj]
                } else {
                    return Err(format!("same abscissa but sums disagree at pair ({i},{j})"));
                };
                let mut check = fb.points[indices[0]];
                for &idx in &indices[1..] {
                    check = inst.fast.add(check, fb.points[idx]);
                }
                if !check.infinity {
                    return Err(format!("group identity failed at pair ({i},{j})"));
                }
                let mut row = vec![0u64; fb.columns];
                for &idx in &indices {
                    let col = fb.col_of[idx];
                    row[col] = (row[col] + fb.coef_of[idx]) % inst.r;
                }
                let independent = if row.iter().all(|&x| x == 0) {
                    zero_rows += 1;
                    false
                } else {
                    nonzero_rows += 1;
                    if !negative_control_checked {
                        let bad = inst.fast.add(check, inst.generator);
                        if bad.infinity {
                            return Err("perturbed identity unexpectedly passed".into());
                        }
                        negative_control_checked = true;
                    }
                    insert_row(&mut rank, row.clone(), inst.r)
                };
                rows.push(json!({
                    "pair": [i, j], "first_pair": [fi, fj],
                    "sum_key": key, "indices": indices,
                    "row": row, "independent": independent,
                }));
            }
        }
    }

    let parity = status == "complete"
        && attempts == production.insert_attempts
        && first.len() as u64 == production.entries
        && pair_additions == production.build_ops.adds;
    if status == "complete" && !parity {
        return Err(format!(
            "production table parity failed: attempts {attempts}/{}, entries {}/{}, additions {pair_additions}/{}",
            production.insert_attempts, first.len(), production.entries, production.build_ops.adds
        ));
    }
    Ok(json!({
        "schema": "rootfind.pair_collision_census/v1",
        "status": status,
        "cell": cell,
        "curve_id": inst.curve_id().icv1,
        "curve": inst.describe(),
        "dimension": dimension,
        "base_sha256": base_sha256,
        "base_points": fb.points.len(),
        "base_abscissae": fb.abscissae,
        "columns": fb.columns,
        "pair_additions": pair_additions,
        "insert_attempts": attempts,
        "unique_keys": first.len(),
        "duplicate_pairs": duplicates,
        "same_sum": same_sum,
        "opposite_sum": opposite_sum,
        "zero_rows": zero_rows,
        "nonzero_rows": nonzero_rows,
        "independent_rank": rank.len(),
        "first_witness_count_parity": parity,
        "negative_control": if negative_control_checked { "passed" } else { "inapplicable" },
        "elapsed_ns_diagnostic_only": start.elapsed().as_nanos().min(u64::MAX as u128) as u64,
        "rows": rows,
    }))
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 || args[1] != "--cell" {
        eprintln!("usage: rootfind_pair_collision_census --cell k13|k19|k23");
        std::process::exit(2);
    }
    match execute(&args[2]) {
        Ok(value) => println!("{}", serde_json::to_string(&value).unwrap()),
        Err(error) => {
            eprintln!("census invalid: {error}");
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::insert_row;
    use std::collections::BTreeMap;

    #[test]
    fn modular_rank_distinguishes_dependent_rows() {
        let mut pivots = BTreeMap::new();
        assert!(insert_row(&mut pivots, vec![1, 2, 0], 101));
        assert!(!insert_row(&mut pivots, vec![2, 4, 0], 101));
        assert!(insert_row(&mut pivots, vec![0, 1, 3], 101));
        assert_eq!(pivots.len(), 2);
    }
}
