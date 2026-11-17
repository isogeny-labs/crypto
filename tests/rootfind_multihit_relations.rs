//! Bounded first-hit versus all-hit relation screen.
//! Frozen specification: research/rootfind_multihit_relations_20261116/PROTOCOL.md
//! and its before-data AMENDMENT-1.md.

use crypto_lib::cryptanalysis::ic_boundary::{
    koblitz_factor_base, koblitz_instance, BinaryGroup, ColumnFold, FactorBase, GroupOps, PairTable,
};
use crypto_lib::cryptanalysis::koblitz_fast::{FastCurve, FastPoint};
use crypto_lib::cryptanalysis::koblitz_index_calculus::build_standard_subspace_factor_base;
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

fn mul_mod(a: u64, b: u64, r: u64) -> u64 {
    ((a as u128 * b as u128) % r as u128) as u64
}

fn sub_mod(a: u64, b: u64, r: u64) -> u64 {
    (a + r - b) % r
}

fn inverse(a: u64, r: u64) -> u64 {
    let (mut t, mut next_t) = (0i128, 1i128);
    let (mut x, mut next_x) = (r as i128, a as i128);
    while next_x != 0 {
        let q = x / next_x;
        (t, next_t) = (next_t, t - q * next_t);
        (x, next_x) = (next_x, x - q * next_x);
    }
    assert_eq!(x, 1, "pivot is invertible modulo subgroup order");
    t.rem_euclid(r as i128) as u64
}

#[derive(Clone)]
struct Basis {
    r: u64,
    vars: usize,
    pivots: Vec<Option<Vec<u64>>>,
    rank: usize,
}

impl Basis {
    fn new(r: u64, vars: usize) -> Self {
        Self {
            r,
            vars,
            pivots: vec![None; vars],
            rank: 0,
        }
    }

    fn add(&mut self, mut row: Vec<u64>, rhs: u64) -> bool {
        assert_eq!(row.len(), self.vars);
        assert!(row.iter().all(|&x| x < self.r));
        row.push(rhs % self.r);
        for col in 0..self.vars {
            let coefficient = row[col];
            if coefficient == 0 {
                continue;
            }
            if let Some(pivot) = &self.pivots[col] {
                for j in col..=self.vars {
                    row[j] = sub_mod(row[j], mul_mod(coefficient, pivot[j], self.r), self.r);
                }
            } else {
                let scale = inverse(coefficient, self.r);
                for value in &mut row[col..=self.vars] {
                    *value = mul_mod(*value, scale, self.r);
                }
                self.pivots[col] = Some(row);
                self.rank += 1;
                return true;
            }
        }
        assert_eq!(row[self.vars], 0, "all retained equations are consistent");
        false
    }

    fn target_scalar(&self) -> Option<u64> {
        if self.rank != self.vars {
            return None;
        }
        let mut answer = vec![0u64; self.vars];
        for col in (0..self.vars).rev() {
            let row = self.pivots[col].as_ref().expect("full rank has each pivot");
            let tail = ((col + 1)..self.vars).fold(0, |sum, j| {
                (sum + mul_mod(row[j], answer[j], self.r)) % self.r
            });
            answer[col] = sub_mod(row[self.vars], tail, self.r);
        }
        Some(answer[self.vars - 1])
    }
}

fn scalar_mul(curve: &FastCurve, point: FastPoint, mut scalar: u64) -> FastPoint {
    let mut result = FastPoint::INFINITY;
    let mut addend = point;
    while scalar > 0 {
        if scalar & 1 == 1 {
            result = curve.add(result, addend);
        }
        addend = curve.add(addend, addend);
        scalar >>= 1;
    }
    result
}

fn collision_row(
    curve: &FastCurve,
    fb: &FactorBase<FastPoint>,
    [i, j, fi, fj]: [usize; 4],
    r: u64,
) -> Vec<u64> {
    let current = curve.add(fb.points[i], fb.points[j]);
    let first = curve.add(fb.points[fi], fb.points[fj]);
    assert!(!current.infinity && !first.infinity);
    assert_eq!(current.x, first.x);
    let indices = if current == first {
        [i, j, fb.neg_index[fi], fb.neg_index[fj]]
    } else {
        assert_eq!(current, curve.neg(first));
        [i, j, fi, fj]
    };
    let sum = indices.iter().fold(FastPoint::INFINITY, |acc, &idx| {
        curve.add(acc, fb.points[idx])
    });
    assert!(sum.infinity, "collision row is a full group equation");
    let mut row = vec![0u64; fb.columns + 1];
    for &idx in &indices {
        let col = fb.col_of[idx];
        row[col] = (row[col] + fb.coef_of[idx]) % r;
    }
    row
}

fn triple_row(
    curve: &FastCurve,
    fb: &FactorBase<FastPoint>,
    point: FastPoint,
    [i, j, k]: [usize; 3],
    a: u64,
    h: u64,
    r: u64,
) -> (Vec<u64>, u64) {
    let sum = curve.add(curve.add(fb.points[i], fb.points[j]), fb.points[k]);
    assert_eq!(sum, point, "triple independently re-adds to residual");
    let mut row = vec![0u64; fb.columns + 1];
    for idx in [i, j, k] {
        let col = fb.col_of[idx];
        row[col] = (row[col] + fb.coef_of[idx]) % r;
    }
    row[fb.columns] = sub_mod(0, h % r, r);
    (row, mul_mod(h % r, a, r))
}

#[derive(Clone, Copy)]
struct ScanCost {
    subtractions: u64,
    lookups: u64,
    pair_readds: u64,
}

impl ScanCost {
    fn add(&mut self, other: Self) {
        self.subtractions += other.subtractions;
        self.lookups += other.lookups;
        self.pair_readds += other.pair_readds;
    }

    fn json(self) -> serde_json::Value {
        json!({
            "subtractions": self.subtractions,
            "lookups": self.lookups,
            "pair_readds": self.pair_readds,
        })
    }
}

#[test]
fn fixed_koblitz_first_hit_vs_all_hit() {
    for (
        n,
        dimension,
        sample,
        expected_points,
        expected_keys,
        expected_collisions,
        curve_id,
        digest,
    ) in [
        (
            13,
            5,
            2_003,
            33,
            261,
            11,
            "ICV1:f2m-13-752c6463:181:8012:0x1:-7:unk:r:515ee5696294",
            "d9f69121c63a04c3f744c5be0e4ea488a030db0315794a4e05c7a9d3cd341807",
        ),
        (
            19,
            7,
            1_024,
            139,
            4_800,
            30,
            "ICV1:f2m-19-87f45376:797:523492:0x1:-7:unk:r:b6cf2467887c",
            "013a0d4223494500cd06b48bf41b51dda2dd5add0009d1b89cdb0b7bd49c0a5a",
        ),
        (
            23,
            8,
            1_024,
            275,
            18_842,
            64,
            "ICV1:f2m-23-c07a6b8c:5197:8383412:0x1:-7:unk:r:69e76b739aa7",
            "686cba2fafe973e49139c2722df47bf728340e25a4df5faad8b51dfa91df3877",
        ),
    ] {
        let inst = koblitz_instance(0, n).expect("registered Koblitz cell");
        assert_eq!(inst.curve_id().icv1, curve_id);
        assert_eq!(inst.group_order, inst.r * inst.cofactor);
        assert_eq!(inst.cofactor, 4);
        assert_ne!(inst.r % 37, 0, "stride is coprime to subgroup order");
        let parent = build_standard_subspace_factor_base(
            inst.koblitz.as_ref().expect("Koblitz view"),
            dimension,
        )
        .expect("standard subspace");
        let fb = koblitz_factor_base(
            &inst,
            &parent,
            ColumnFold::SignedFrobeniusOrbit,
            format!("standard subspace d={dimension}"),
        )
        .expect("signed orbit base");
        assert_eq!(fb.points.len(), expected_points);
        assert_eq!(base_digest(&fb), digest);
        let group = BinaryGroup(&inst.fast);
        let (table, collisions, _) =
            PairTable::build_negation_folded_binary_batch_torsion_with_collisions(&group, &fb)
                .expect("torsion pair table");
        assert_eq!(table.entries as usize, expected_keys);
        assert_eq!(collisions.len(), expected_collisions);

        let vars = fb.columns + 1;
        let mut first = Basis::new(inst.r, vars);
        let mut all = Basis::new(inst.r, vars);
        let mut nonzero_collision_rows = 0usize;
        for collision in collisions {
            let row = collision_row(&inst.fast, &fb, collision, inst.r);
            if row.iter().any(|&x| x != 0) {
                nonzero_collision_rows += 1;
            }
            first.add(row.clone(), 0);
            all.add(row, 0);
        }
        let mut torsion = vec![0u64; vars];
        torsion[0] = 1;
        first.add(torsion.clone(), 0);
        all.add(torsion, 0);
        assert_eq!(first.rank, all.rank);
        let seed_rank = first.rank;

        let q = scalar_mul(&inst.fast, inst.generator, 641);
        assert!(!q.infinity);
        let start = scalar_mul(&inst.fast, inst.generator, 758);
        let stride = scalar_mul(&inst.fast, inst.generator, 37);
        let mut point = start;
        let mut queries = 0usize;
        let (mut hit_targets, mut multihit_targets, mut total_hits, mut max_hits) =
            (0usize, 0usize, 0usize, 0usize);
        let (mut first_independent, mut all_independent, mut extra_independent) =
            (0usize, 0usize, 0usize);
        let (mut first_cost, mut all_cost) = (
            ScanCost {
                subtractions: 0,
                lookups: 0,
                pair_readds: 0,
            },
            ScanCost {
                subtractions: 0,
                lookups: 0,
                pair_readds: 0,
            },
        );
        let (mut first_full, mut all_full): (Option<(usize, ScanCost)>, Option<(usize, ScanCost)>) =
            (None, None);
        for t in 0..sample {
            let a = (117 + 37 * t as u64) % inst.r;
            assert_eq!(
                point,
                inst.fast.add(q, scalar_mul(&inst.fast, inst.generator, a))
            );
            if !point.infinity {
                queries += 1;
                let mut hits = Vec::<[usize; 3]>::new();
                let (mut lookups, mut ops) = (0u64, GroupOps::default());
                let mut prefix = None;
                for k in 0..fb.points.len() {
                    let rest = inst.fast.add(point, inst.fast.neg(fb.points[k]));
                    if rest.infinity {
                        continue;
                    }
                    if let Some((i, j)) = table.probe(&group, &fb, &mut ops, &mut lookups, rest) {
                        if prefix.is_none() {
                            prefix = Some(ScanCost {
                                subtractions: (k + 1) as u64,
                                lookups,
                                pair_readds: ops.adds + ops.doubles,
                            });
                        }
                        hits.push([i, j, k]);
                    }
                }
                let whole = ScanCost {
                    subtractions: fb.points.len() as u64,
                    lookups,
                    pair_readds: ops.adds + ops.doubles,
                };
                all_cost.add(whole);
                first_cost.add(prefix.unwrap_or(whole));
                if !hits.is_empty() {
                    hit_targets += 1;
                }
                if hits.len() > 1 {
                    multihit_targets += 1;
                }
                total_hits += hits.len();
                max_hits = max_hits.max(hits.len());
                for (position, hit) in hits.into_iter().enumerate() {
                    let (row, rhs) =
                        triple_row(&inst.fast, &fb, point, hit, a, inst.cofactor, inst.r);
                    if position == 0 && first.add(row.clone(), rhs) {
                        first_independent += 1;
                    }
                    if all.add(row, rhs) {
                        all_independent += 1;
                        if position > 0 {
                            extra_independent += 1;
                        }
                    }
                }
                if first_full.is_none() && first.rank == vars {
                    assert_eq!(first.target_scalar(), Some(641));
                    first_full = Some((queries, first_cost));
                }
                if all_full.is_none() && all.rank == vars {
                    assert_eq!(all.target_scalar(), Some(641));
                    all_full = Some((queries, all_cost));
                }
            }
            point = inst.fast.add(point, stride);
        }
        if n == 13 {
            assert_eq!(point, start, "K13 traverses whole subgroup");
            assert_eq!(queries, inst.r as usize - 1);
        }
        assert!(all.rank >= first.rank);
        if let Some(scalar) = first.target_scalar() {
            assert_eq!(scalar, 641);
        }
        if let Some(scalar) = all.target_scalar() {
            assert_eq!(scalar, 641);
        }
        println!(
            "MULTIHIT_JSON={}",
            json!({
                "schema": "rootfind-multihit-screen/v1",
                "status": "passed",
                "field_degree": n,
                "curve_id": curve_id,
                "base_sha256": digest,
                "subgroup_order": inst.r,
                "base_points": fb.points.len(),
                "columns_with_target": vars,
                "pair_keys": table.entries,
                "collision_candidates": expected_collisions,
                "nonzero_collision_rows": nonzero_collision_rows,
                "seed_rank": seed_rank,
                "sample_targets": sample,
                "nonidentity_queries": queries,
                "hit_targets": hit_targets,
                "multihit_targets": multihit_targets,
                "total_hits": total_hits,
                "extra_hits": total_hits - hit_targets,
                "max_hits_per_target": max_hits,
                "first_independent_rows": first_independent,
                "all_independent_rows": all_independent,
                "extra_independent_rows": extra_independent,
                "first_final_rank": first.rank,
                "all_final_rank": all.rank,
                "first_full_rank": first_full.map(|(queries, cost)| json!({"queries": queries, "scan": cost.json()})),
                "all_full_rank": all_full.map(|(queries, cost)| json!({"queries": queries, "scan": cost.json()})),
            })
        );
    }
}
