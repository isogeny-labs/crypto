//! Actual Koblitz pair-stream parity for the standalone paged first-witness index.
//! Frozen boundary: research/rootfind_paged_curve_parity_20261116/PROTOCOL.md.

#[path = "../research/rootfind_paged_pair_index_20261116/paged_index.rs"]
mod paged_index;

use std::collections::HashMap;

use crypto_lib::cryptanalysis::ic_boundary::{
    koblitz_factor_base, koblitz_instance, BinaryGroup, ColumnFold, FactorBase, PairTable,
};
use crypto_lib::cryptanalysis::koblitz_fast::FastPoint;
use crypto_lib::cryptanalysis::koblitz_index_calculus::build_standard_subspace_factor_base;
use paged_index::{Insert, PagedPairIndex};
use serde_json::json;
use sha2::{Digest, Sha256};

fn base_digest(fb: &FactorBase<FastPoint>) -> String {
    let mut hasher = Sha256::new();
    for (idx, point) in fb.points.iter().enumerate() {
        hasher.update(point.pack().to_le_bytes());
        hasher.update((fb.col_of[idx] as u64).to_le_bytes());
        hasher.update(fb.coef_of[idx].to_le_bytes());
    }
    format!("{:x}", hasher.finalize())
}

#[test]
fn actual_torsion_pair_stream_matches_paged_first_witnesses_and_collisions() {
    for (n, dimension, a, points, keys, collisions_expected, curve_id, base_sha256) in [
        (
            13,
            5,
            17,
            33,
            261,
            11,
            "ICV1:f2m-13-752c6463:181:8012:0x1:-7:unk:r:515ee5696294",
            "d9f69121c63a04c3f744c5be0e4ea488a030db0315794a4e05c7a9d3cd341807",
        ),
        (
            19,
            7,
            70,
            139,
            4_800,
            30,
            "ICV1:f2m-19-87f45376:797:523492:0x1:-7:unk:r:b6cf2467887c",
            "013a0d4223494500cd06b48bf41b51dda2dd5add0009d1b89cdb0b7bd49c0a5a",
        ),
        (
            23,
            8,
            138,
            275,
            18_842,
            64,
            "ICV1:f2m-23-c07a6b8c:5197:8383412:0x1:-7:unk:r:69e76b739aa7",
            "686cba2fafe973e49139c2722df47bf728340e25a4df5faad8b51dfa91df3877",
        ),
    ] {
        let inst = koblitz_instance(0, n).expect("frozen Koblitz cell");
        assert_eq!(inst.curve_id().icv1, curve_id);
        let parent = build_standard_subspace_factor_base(
            inst.koblitz.as_ref().expect("Koblitz view"),
            dimension,
        )
        .expect("frozen standard subspace");
        let fb = koblitz_factor_base(
            &inst,
            &parent,
            ColumnFold::SignedFrobeniusOrbit,
            format!("standard subspace d={dimension}"),
        )
        .expect("materialised signed-orbit base");
        assert_eq!(fb.abscissa_list.len(), a);
        assert_eq!(fb.points.len(), points);
        assert_eq!(base_digest(&fb), base_sha256);

        let group = BinaryGroup(&inst.fast);
        let (production, production_collisions, torsion_stats) =
            PairTable::build_negation_folded_binary_batch_torsion_with_collisions(&group, &fb)
                .expect("production torsion table");
        let groups: Vec<&Vec<usize>> = fb
            .abscissa_list
            .iter()
            .map(|x| fb.x_index.get(x).expect("indexed abscissa"))
            .collect();
        let mut paged = PagedPairIndex::from_base_abscissae(n, a).expect("bounded page capacity");
        let mut reference = HashMap::<u32, (usize, usize, FastPoint)>::new();
        let mut expected_collisions = Vec::<[usize; 4]>::new();
        let mut attempts = 0u64;
        let mut full_group_checks = 0u64;

        let mut insert_pair = |i: usize, j: usize, sum: FastPoint| {
            if sum.infinity {
                return;
            }
            let key = u32::try_from(sum.x + 1).expect("binary key fits u32");
            let reference_result = if let Some(&(fi, fj, first_sum)) = reference.get(&key) {
                assert_eq!(sum.x, first_sum.x);
                assert!(sum == first_sum || sum == inst.fast.neg(first_sum));
                let row = if sum == first_sum {
                    [i, j, fb.neg_index[fi], fb.neg_index[fj]]
                } else {
                    [i, j, fi, fj]
                };
                let mut checked_sum = fb.points[row[0]];
                for &idx in &row[1..] {
                    checked_sum = inst.fast.add(checked_sum, fb.points[idx]);
                }
                assert!(
                    checked_sum.infinity,
                    "full group equality at n={n}, key={key}"
                );
                full_group_checks += 1;
                expected_collisions.push([i, j, fi, fj]);
                Insert::Existing {
                    first_i: fi as u32,
                    first_j: fj as u32,
                }
            } else {
                reference.insert(key, (i, j, sum));
                Insert::New
            };
            assert_eq!(
                paged.insert(key, i as u32, j as u32),
                Ok(reference_result),
                "paged insertion at n={n}, key={key}"
            );
            attempts += 1;
        };

        for (ai, group_i) in groups.iter().enumerate() {
            let i = group_i[0];
            if ai == 0 {
                for group_j in groups.iter().skip(1) {
                    let j = group_j[0];
                    let sum = inst.fast.add(fb.points[i], fb.points[j]);
                    assert!(!sum.infinity, "torsion first-row sum at n={n}");
                    insert_pair(i, j, sum);
                }
                continue;
            }
            for group_j in groups.iter().skip(ai) {
                for &j in group_j.iter() {
                    insert_pair(i, j, inst.fast.add(fb.points[i], fb.points[j]));
                }
            }
        }

        assert_eq!(reference.len(), keys, "retained keys at n={n}");
        assert_eq!(paged.entries(), keys, "paged keys at n={n}");
        assert_eq!(
            production.entries as usize, keys,
            "production keys at n={n}"
        );
        assert_eq!(attempts, production.insert_attempts, "insertions at n={n}");
        assert_eq!(expected_collisions.len(), collisions_expected);
        assert_eq!(production_collisions, expected_collisions);
        assert_eq!(full_group_checks as usize, collisions_expected);
        assert_eq!(torsion_stats.skipped_twin_collisions, (a - 1) as u64);

        for (&key, &(i, j, _)) in &reference {
            assert_eq!(
                paged.get(key),
                Some((i as u32, j as u32)),
                "n={n}, key={key}"
            );
        }
        let max_key = 1u32 << n;
        for key in [0, 1, 2, max_key / 2, max_key, max_key + 1] {
            assert_eq!(
                paged.get(key),
                reference.get(&key).map(|&(i, j, _)| (i as u32, j as u32)),
                "boundary probe n={n}, key={key}"
            );
        }
        if n == 13 {
            for key in 1..=max_key {
                assert_eq!(
                    paged.get(key),
                    reference.get(&key).map(|&(i, j, _)| (i as u32, j as u32)),
                    "all K13 probes key={key}"
                );
            }
        }

        println!(
            "PARITY_JSON={}",
            json!({
                "schema": "rootfind-paged-curve-parity/v1",
                "status": "passed",
                "field_degree": n,
                "curve_id": curve_id,
                "base_sha256": base_sha256,
                "base_points": points,
                "base_abscissae": a,
                "insert_attempts": attempts,
                "first_witness_keys": keys,
                "production_collisions": production_collisions.len(),
                "full_group_checks": full_group_checks,
                "used_pages": paged.used_pages(),
                "page_capacity": paged.page_capacity(),
                "requested_backing_bytes": paged.requested_backing_bytes(),
            })
        );
    }
}
