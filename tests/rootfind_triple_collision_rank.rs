//! Source-bound triple-sum collision rank screen; see the frozen protocol at
//! research/rootfind_triple_collision_rank_20261116/PROTOCOL.md.
#![recursion_limit = "256"]

use crypto_lib::binary_ecc::BinaryPoint;
use crypto_lib::cryptanalysis::ic_boundary::{
    koblitz_factor_base, koblitz_instance, BinaryGroup, BinaryInstance, ColumnFold, FactorBase,
    PairTable,
};
use crypto_lib::cryptanalysis::koblitz_fast::{FastCurve, FastPoint};
use crypto_lib::cryptanalysis::koblitz_index_calculus::build_standard_subspace_factor_base;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::{HashMap, HashSet};

fn digest(fb: &FactorBase<FastPoint>) -> String {
    let mut h = Sha256::new();
    for (i, p) in fb.points.iter().enumerate() {
        h.update(p.pack().to_le_bytes());
        h.update((fb.col_of[i] as u64).to_le_bytes());
        h.update(fb.coef_of[i].to_le_bytes());
    }
    format!("{:x}", h.finalize())
}

fn sub(a: u64, b: u64, r: u64) -> u64 {
    (a + r - b) % r
}

fn mul(a: u64, b: u64, r: u64) -> u64 {
    ((a as u128 * b as u128) % r as u128) as u64
}

fn inverse(a: u64, r: u64) -> u64 {
    let (mut t, mut next_t, mut x, mut next_x) = (0i128, 1i128, r as i128, a as i128);
    while next_x != 0 {
        let q = x / next_x;
        (t, next_t) = (next_t, t - q * next_t);
        (x, next_x) = (next_x, x - q * next_x);
    }
    assert_eq!(x, 1);
    t.rem_euclid(r as i128) as u64
}

#[derive(Clone)]
struct Basis {
    r: u64,
    pivots: Vec<Option<Vec<u64>>>,
    rank: usize,
    row_ops: u64,
}

impl Basis {
    fn new(r: u64, cols: usize) -> Self {
        Self {
            r,
            pivots: vec![None; cols],
            rank: 0,
            row_ops: 0,
        }
    }

    fn add(&mut self, mut row: Vec<u64>) -> bool {
        assert_eq!(row.len(), self.pivots.len());
        for col in 0..row.len() {
            let a = row[col];
            if a == 0 {
                continue;
            }
            if let Some(pivot) = &self.pivots[col] {
                for j in col..row.len() {
                    row[j] = sub(row[j], mul(a, pivot[j], self.r), self.r);
                    self.row_ops += 1;
                }
            } else {
                let scale = inverse(a, self.r);
                for v in &mut row[col..] {
                    *v = mul(*v, scale, self.r);
                    self.row_ops += 1;
                }
                self.pivots[col] = Some(row);
                self.rank += 1;
                return true;
            }
        }
        false
    }
}

fn vector(fb: &FactorBase<FastPoint>, ids: &[usize], r: u64) -> Vec<u64> {
    let mut row = vec![0; fb.columns];
    for &i in ids {
        let col = fb.col_of[i];
        row[col] = (row[col] + fb.coef_of[i]) % r;
    }
    row
}

fn pair_seed(
    curve: &FastCurve,
    fb: &FactorBase<FastPoint>,
    collisions: &[[usize; 4]],
    basis: &mut Basis,
) {
    for &[i, j, fi, fj] in collisions {
        let a = curve.add(fb.points[i], fb.points[j]);
        let b = curve.add(fb.points[fi], fb.points[fj]);
        assert!(!a.infinity && !b.infinity);
        let mut row = vector(fb, &[i, j], basis.r);
        let other = vector(fb, &[fi, fj], basis.r);
        if a == b {
            for (x, y) in row.iter_mut().zip(other) {
                *x = sub(*x, y, basis.r);
            }
        } else {
            assert_eq!(a, curve.neg(b));
            for (x, y) in row.iter_mut().zip(other) {
                *x = (*x + y) % basis.r;
            }
        }
        basis.add(row);
    }
    let mut torsion = vec![0; fb.columns];
    torsion[0] = 1;
    basis.add(torsion);
}

fn splitmix64(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9e3779b97f4a7c15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
    z ^ (z >> 31)
}

fn triple_sum(curve: &FastCurve, fb: &FactorBase<FastPoint>, ids: [usize; 3]) -> FastPoint {
    curve.add(
        curve.add(fb.points[ids[0]], fb.points[ids[1]]),
        fb.points[ids[2]],
    )
}

fn recorded_triple(value: &Value, name: &str, points: usize) -> [usize; 3] {
    let parts = value[name].as_array().expect("serialized triple array");
    assert_eq!(parts.len(), 3);
    let mut out = [0; 3];
    for (dst, item) in out.iter_mut().zip(parts) {
        *dst = item.as_u64().expect("serialized triple index") as usize;
        assert!(*dst < points);
    }
    out
}

fn replay_general_sum(
    inst: &BinaryInstance,
    fb: &FactorBase<FastPoint>,
    ids: [usize; 3],
) -> BinaryPoint {
    let general = inst.koblitz.as_ref().expect("Koblitz replay curve");
    let mut sum = BinaryPoint::Infinity;
    for idx in ids {
        sum = general.add(&sum, &inst.fast.lower(fb.points[idx]));
    }
    sum
}

fn replay_rows(
    inst: &BinaryInstance,
    fb: &FactorBase<FastPoint>,
    seed_basis: &Basis,
    rows: &[Value],
) -> usize {
    let general = inst.koblitz.as_ref().expect("Koblitz replay curve");
    let mut basis = seed_basis.clone();
    for record in rows {
        let later = recorded_triple(record, "triple", fb.points.len());
        let a = replay_general_sum(inst, fb, later);
        let mut rebuilt = vec![0u64; fb.columns];
        for idx in later {
            let col = fb.col_of[idx];
            rebuilt[col] = (rebuilt[col] + fb.coef_of[idx]) % inst.r;
        }
        match record["kind"].as_str().expect("serialized row kind") {
            "identity" => assert_eq!(a, BinaryPoint::Infinity),
            "collision" => {
                let first = recorded_triple(record, "first", fb.points.len());
                let b = replay_general_sum(inst, fb, first);
                match record["sign"].as_str().expect("serialized collision sign") {
                    "equal" => {
                        assert_eq!(a, b);
                        for idx in first {
                            let col = fb.col_of[idx];
                            rebuilt[col] = sub(rebuilt[col], fb.coef_of[idx], inst.r);
                        }
                    }
                    "opposite" => {
                        assert_eq!(general.add(&a, &b), BinaryPoint::Infinity);
                        for idx in first {
                            let col = fb.col_of[idx];
                            rebuilt[col] = (rebuilt[col] + fb.coef_of[idx]) % inst.r;
                        }
                    }
                    other => panic!("unexpected collision sign: {other}"),
                }
            }
            other => panic!("unexpected row kind: {other}"),
        }
        let recorded: Vec<u64> = record["row"]
            .as_array()
            .expect("serialized coefficient row")
            .iter()
            .map(|v| v.as_u64().expect("serialized coefficient"))
            .collect();
        assert_eq!(rebuilt, recorded);
        assert_eq!(basis.add(rebuilt), record["independent"].as_bool().unwrap());
    }
    basis.rank
}

#[test]
fn fixed_koblitz_triple_collision_rank() {
    for (n, d, points, keys, pair_collisions, seed_rank, curve_id, base_hash) in [
        (
            13,
            5,
            33,
            261,
            11,
            5,
            "ICV1:f2m-13-752c6463:181:8012:0x1:-7:unk:r:515ee5696294",
            "d9f69121c63a04c3f744c5be0e4ea488a030db0315794a4e05c7a9d3cd341807",
        ),
        (
            19,
            7,
            139,
            4800,
            30,
            11,
            "ICV1:f2m-19-87f45376:797:523492:0x1:-7:unk:r:b6cf2467887c",
            "013a0d4223494500cd06b48bf41b51dda2dd5add0009d1b89cdb0b7bd49c0a5a",
        ),
        (
            23,
            8,
            275,
            18842,
            64,
            23,
            "ICV1:f2m-23-c07a6b8c:5197:8383412:0x1:-7:unk:r:69e76b739aa7",
            "686cba2fafe973e49139c2722df47bf728340e25a4df5faad8b51dfa91df3877",
        ),
    ] {
        let inst = koblitz_instance(0, n).unwrap();
        assert_eq!(inst.curve_id().icv1, curve_id);
        assert_eq!(inst.group_order, inst.r * inst.cofactor);
        assert_eq!(inst.cofactor, 4);
        let parent =
            build_standard_subspace_factor_base(inst.koblitz.as_ref().unwrap(), d).unwrap();
        let fb = koblitz_factor_base(
            &inst,
            &parent,
            ColumnFold::SignedFrobeniusOrbit,
            format!("standard subspace d={d}"),
        )
        .unwrap();
        assert_eq!(fb.points.len(), points);
        assert_eq!(digest(&fb), base_hash);
        let (pair, collisions, _) =
            PairTable::build_negation_folded_binary_batch_torsion_with_collisions(
                &BinaryGroup(&inst.fast),
                &fb,
            )
            .unwrap();
        assert_eq!(pair.entries as usize, keys);
        assert_eq!(collisions.len(), pair_collisions);
        let mut basis = Basis::new(inst.r, fb.columns);
        pair_seed(&inst.fast, &fb, &collisions, &mut basis);
        assert_eq!(basis.rank, seed_rank);
        let mut direct_basis = basis.clone();
        let replay_seed_basis = basis.clone();

        let capacity = points * (points + 1) * (points + 2) / 6;
        let limit = capacity.min(if n == 13 { capacity } else { 32_768 });
        let mut state = 2_026_111_601u64 + n as u64;
        let mut seen = HashSet::<[usize; 3]>::with_capacity(limit);
        let mut sums = HashMap::<u64, ([usize; 3], FastPoint)>::with_capacity(limit);
        let mut direct_sums = HashMap::<u64, ([usize; 3], FastPoint)>::with_capacity(limit);
        let mut prefixes = HashMap::<(usize, usize), FastPoint>::with_capacity(
            limit.min(points * (points + 1) / 2),
        );
        let (mut draws, mut collisions_seen, mut identity_sums) = (0u64, 0u64, 0u64);
        let (mut direct_adds, mut prefix_adds, mut prefix_hits, mut tail_adds) =
            (0u64, 0u64, 0u64, 0u64);
        let (mut parity_checks, mut verify_adds, mut tamper_adds) = (0u64, 0u64, 0u64);
        let (mut zero, mut dependent, mut independent) = (0u64, 0u64, 0u64);
        let mut rank_at = Vec::<(usize, usize)>::new();
        let mut first_rank_ceiling: Option<Value> = None;
        let mut rows = Vec::new();
        let mut tamper_witness: Option<([usize; 3], [usize; 3])> = None;
        while seen.len() < limit {
            let mut ids = [0usize; 3];
            for idx in &mut ids {
                *idx = (splitmix64(&mut state) % points as u64) as usize;
            }
            ids.sort_unstable();
            draws += 1;
            if !seen.insert(ids) {
                continue;
            }
            let prefix = if let Some(&cached) = prefixes.get(&(ids[0], ids[1])) {
                prefix_hits += 1;
                cached
            } else {
                let new = inst.fast.add(fb.points[ids[0]], fb.points[ids[1]]);
                prefix_adds += 1;
                prefixes.insert((ids[0], ids[1]), new);
                new
            };
            let sum = inst.fast.add(prefix, fb.points[ids[2]]);
            tail_adds += 1;
            let direct = triple_sum(&inst.fast, &fb, ids);
            direct_adds += 2;
            parity_checks += 1;
            assert_eq!(sum, direct, "cached and direct triple sums must match");
            let row = vector(&fb, &ids, inst.r);
            if sum.infinity {
                identity_sums += 1;
                let nonzero = row.iter().any(|&x| x != 0);
                let raised_rank = nonzero && basis.add(row.clone());
                let direct_raised_rank = nonzero && direct_basis.add(row.clone());
                assert_eq!(raised_rank, direct_raised_rank);
                assert_eq!(basis.rank, direct_basis.rank);
                if !nonzero {
                    zero += 1;
                } else if raised_rank {
                    independent += 1;
                    rank_at.push((seen.len(), basis.rank));
                    if basis.rank == fb.columns - 1 && first_rank_ceiling.is_none() {
                        first_rank_ceiling = Some(json!({
                            "unique_triples":seen.len(), "rank":basis.rank,
                            "direct_generator_additions":direct_adds,
                            "cached_prefix_additions":prefix_adds,
                            "cached_tail_additions":tail_adds,
                            "cached_prefix_hits":prefix_hits,
                            "prefix_cache_entries":prefixes.len(),
                            "collision_verification_additions":verify_adds,
                            "sum_key_lookups":seen.len() as u64-identity_sums,
                            "rows":rows.len()+1,
                        }));
                    }
                } else {
                    dependent += 1;
                }
                rows.push(json!({"kind":"identity", "triple":ids,
                    "row":row, "independent":raised_rank, "unique_index":seen.len()}));
                assert!(
                    basis.rank < fb.columns,
                    "homogeneous rank cannot span a nonzero log vector"
                );
                continue;
            }
            let key = sum.pack().min(inst.fast.neg(sum).pack());
            let direct_key = direct.pack().min(inst.fast.neg(direct).pack());
            assert_eq!(key, direct_key);
            assert_eq!(sums.get(&key), direct_sums.get(&direct_key));
            if let Some(&(first, _)) = sums.get(&key) {
                collisions_seen += 1;
                let a = triple_sum(&inst.fast, &fb, ids);
                let b = triple_sum(&inst.fast, &fb, first);
                verify_adds += 4;
                let first_row = vector(&fb, &first, inst.r);
                let mut signed = row;
                let sign;
                if a == b {
                    sign = "equal";
                    assert!(inst.fast.add(a, inst.fast.neg(b)).infinity);
                    verify_adds += 1;
                    for (x, y) in signed.iter_mut().zip(first_row) {
                        *x = sub(*x, y, inst.r);
                    }
                } else {
                    sign = "opposite";
                    assert_eq!(a, inst.fast.neg(b));
                    assert!(inst.fast.add(a, b).infinity);
                    verify_adds += 1;
                    for (x, y) in signed.iter_mut().zip(first_row) {
                        *x = (*x + y) % inst.r;
                    }
                }
                tamper_witness.get_or_insert((first, ids));
                let nonzero = signed.iter().any(|&x| x != 0);
                let raised_rank = nonzero && basis.add(signed.clone());
                let direct_raised_rank = nonzero && direct_basis.add(signed.clone());
                assert_eq!(raised_rank, direct_raised_rank);
                assert_eq!(basis.rank, direct_basis.rank);
                if !nonzero {
                    zero += 1;
                } else if raised_rank {
                    independent += 1;
                    rank_at.push((seen.len(), basis.rank));
                    if basis.rank == fb.columns - 1 && first_rank_ceiling.is_none() {
                        first_rank_ceiling = Some(json!({
                            "unique_triples":seen.len(), "rank":basis.rank,
                            "direct_generator_additions":direct_adds,
                            "cached_prefix_additions":prefix_adds,
                            "cached_tail_additions":tail_adds,
                            "cached_prefix_hits":prefix_hits,
                            "prefix_cache_entries":prefixes.len(),
                            "collision_verification_additions":verify_adds,
                            "sum_key_lookups":seen.len() as u64-identity_sums,
                            "rows":rows.len()+1,
                        }));
                    }
                } else {
                    dependent += 1;
                }
                rows.push(json!({"kind":"collision", "first":first, "triple":ids,
                    "sign":sign, "row":signed, "independent":raised_rank,
                    "unique_index":seen.len()}));
            } else {
                sums.insert(key, (ids, sum));
                direct_sums.insert(direct_key, (ids, direct));
            }
            assert!(
                basis.rank < fb.columns,
                "homogeneous rank cannot span a nonzero log vector"
            );
        }
        let (first, later) =
            tamper_witness.expect("each cell needs a collision for tamper control");
        let first_sum = triple_sum(&inst.fast, &fb, first);
        tamper_adds += 2;
        let mut rejected = false;
        for pos in 0..3 {
            for replacement in 0..points {
                let mut wrong = later;
                wrong[pos] = replacement;
                let check = triple_sum(&inst.fast, &fb, wrong);
                tamper_adds += 2;
                if check != first_sum && check != inst.fast.neg(first_sum) {
                    rejected = true;
                    break;
                }
            }
            if rejected {
                break;
            }
        }
        assert!(rejected, "mutated witness must be rejected");
        assert_eq!(sums, direct_sums);
        assert_eq!(basis.rank, direct_basis.rank);
        assert_eq!(basis.row_ops, direct_basis.row_ops);
        assert_eq!(rows.len() as u64, zero + dependent + independent);
        let replayed_rows = rows.len();
        let independently_replayed_rank = replay_rows(&inst, &fb, &replay_seed_basis, &rows);
        assert_eq!(independently_replayed_rank, basis.rank);
        assert_eq!(prefix_adds + prefix_hits, seen.len() as u64);
        assert_eq!(tail_adds, seen.len() as u64);
        assert_eq!(direct_adds, 2 * seen.len() as u64);
        println!(
            "TRIPLE_COLLISION_JSON={}",
            json!({
                "schema":"rootfind-triple-collision-rank/v3", "n":n, "dimension":d,
                "curve_id":curve_id, "base_sha256":base_hash, "subgroup_order":inst.r,
                "base_points":points, "columns":fb.columns, "pair_seed_rank":seed_rank,
                "triples_possible":capacity, "unique_triples":seen.len(), "draws":draws,
                "duplicate_draws":draws-seen.len() as u64,
                "pair_table_group_additions":pair.build_ops.adds+pair.build_ops.doubles,
                "pair_seed_verification_additions":2*pair_collisions,
                "direct_generator_additions":direct_adds,
                "cached_prefix_additions":prefix_adds,
                "cached_prefix_hits":prefix_hits,
                "cached_tail_additions":tail_adds,
                "prefix_cache_entries":prefixes.len(),
                "prefix_cache_capacity":prefixes.capacity(),
                "prefix_cache_retained_payload_bytes_floor":prefixes.len()
                    *(std::mem::size_of::<(usize, usize)>()
                      +std::mem::size_of::<FastPoint>()),
                "prefix_cache_requested_payload_bytes_floor":prefixes.capacity()
                    *(std::mem::size_of::<(usize, usize)>()
                      +std::mem::size_of::<FastPoint>()),
                "parity_point_comparisons":parity_checks,
                "parity_validation_additions":0,
                "collision_verification_additions":verify_adds,
                "tamper_validation_additions":tamper_adds,
                "sum_key_lookups":seen.len() as u64-identity_sums,
                "sum_key_inserts":sums.len(), "sum_key_collisions":collisions_seen,
                "identity_sums":identity_sums, "zero_rows":zero,
                "dependent_rows":dependent, "independent_rows":independent,
                "final_homogeneous_rank":basis.rank,
                "cached_row_ops":basis.row_ops, "direct_row_ops":direct_basis.row_ops,
                "rank_at_unique_triple":rank_at, "rows":rows,
                "first_rank_ceiling_work_projection":first_rank_ceiling,
                "independently_replayed_rows":replayed_rows,
                "independently_replayed_rank":independently_replayed_rank,
                "tamper_control":rejected,
            })
        );
    }
}
