//! **Fused residual matching** (`targets=residual-fused`): the residual
//! collector of [`super::residual`] with one group addition per step.
//!
//! The original walk adds an r-adding jump `J_j` and then subtracts a base
//! point `P_i` chosen from the *post-jump* point's hash; the intermediate
//! point is never looked up or stored, so one of the two additions is
//! wasted.  Here both `j` and `i` are drawn from the hash of the *current*
//! residual `L` (independent bits: `j` from `mix(key)`, `i` from
//! `mix(key ^ 0xA5A5)`) and the precomputed combined jump
//! `C_{j,i} = J_j − P_i` is added once.  The state update is unchanged
//! (`a += a_j`, `b += b_j`, `row[col(i)] += coef(i)`), equal residuals still
//! take equal steps, so the walk stays collision-preserving.
//!
//! Setup computes all `16·|F|` combined points once; those additions are
//! charged to the relation phase's group-operation ledger beside the
//! jumps' own setup.  The table is capped at `16·|F| ≤ 2^16`
//! ([`MAX_COMBINED_JUMPS`]); protocols use base sizes 2–64.
//!
//! Layout differences from the original (arithmetic is otherwise the
//! same): a `FastMap` instead of the std `HashMap`, rows stored as sparse
//! `(column, coefficient)` lists of nonzeros, expanded to dense only when a
//! collision or direct hit yields a relation, and no per-step dense clone.
//! The old variant is untouched, so sealed evidence replays byte-identically.
use std::time::Instant;

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

use super::residual::{addmod, mix, mulmod, outcome, submod, TABLE_CAP};
use crate::cryptanalysis::ic_boundary::{
    fast_map, CountedGroup, FactorBase, FastMap, OracleCounters, PhaseCost, PipelineOutcome,
    RelationSolver, RowStatus,
};

/// Largest `16·|F|` the combined-jump table may hold.
pub const MAX_COMBINED_JUMPS: usize = 1 << 16;

struct Stored {
    a: u64,
    b: u64,
    /// Nonzero `(column, coefficient)` pairs, sorted by column.
    row: Box<[(u32, u64)]>,
}

/// Bytes actually held by one entry: key, a, b, the boxed slice's fat
/// pointer and its elements (hash-map control bytes and slack excluded).
fn entry_bytes(nnz: usize) -> u64 {
    (8 + 8 + 8 + std::mem::size_of::<Box<[(u32, u64)]>>() + nnz * std::mem::size_of::<(u32, u64)>())
        as u64
}

/// Collect relations by residual collisions with fused steps, until the
/// target column is pinned, then verify.  `max_trials` bounds the steps.
#[allow(clippy::too_many_arguments)]
pub fn collect_residual_fused_and_solve<G: CountedGroup, L: RelationSolver + ?Sized>(
    g: &G,
    generator: G::Elt,
    target: G::Elt,
    r: u64,
    h: u64,
    fb: &FactorBase<G::Elt>,
    seed: u64,
    max_trials: u64,
    matrix: &mut L,
) -> PipelineOutcome {
    let mut rng = StdRng::seed_from_u64(seed ^ 0x5245_5349_4455_414C);
    let mut rel = PhaseCost::default();
    let mut la = PhaseCost::default();
    let mut ver = PhaseCost::default();
    let ctr = OracleCounters::default();
    let d_col = fb.columns;
    let cols = d_col + 1;
    let h_mod = h % r;
    let f = fb.points.len();
    let rel_start = Instant::now();
    let mut la_ns = 0u64;
    let (mut trials, mut found, mut direct_hits, mut collisions, mut self_collisions) =
        (0u64, 0u64, 0u64, 0u64, 0u64);
    let (mut restarts, mut table_probes) = (0u64, 0u64);
    let mut recovered = None;
    let mut verified = false;
    if f == 0 {
        return outcome(
            rel, la, ver, trials, found, matrix, recovered, verified, ctr,
        );
    }

    // Sixteen jumps with tracked coefficients (same draws as `residual`).
    let jump_count = 16usize;
    let jumps: Vec<(G::Elt, u64, u64)> = (0..jump_count)
        .map(|_| {
            let c = rng.gen_range(1..r);
            let d = rng.gen_range(1..r);
            let cg = g.mul(&mut rel.group_ops, generator, c);
            let dq = g.mul(&mut rel.group_ops, target, d);
            let p = g.add(&mut rel.group_ops, cg, dq);
            (p, c, d)
        })
        .collect();
    // Combined jumps C[j·f + i] = J_j − P_i, charged as setup.
    let combined: Vec<G::Elt> = jumps
        .iter()
        .flat_map(|&(jp, _, _)| (0..f).map(move |i| (jp, i)))
        .map(|(jp, i)| g.add(&mut rel.group_ops, jp, fb.points[fb.neg_index[i]]))
        .collect();

    let mut table: FastMap<Stored> = fast_map(1 << 12);
    let mut table_bytes_stored = 0u64;
    let mut fresh = true;
    let (mut point, mut a, mut b) = (g.identity(), 0u64, 0u64);
    let mut row = vec![0u64; d_col];
    let mut touched: Vec<u32> = Vec::new();
    let mut step_adds = 0u64;

    let add_relation = |matrix: &mut L,
                        la: &mut PhaseCost,
                        ver: &mut PhaseCost,
                        row_part: Vec<u64>,
                        da: u64,
                        db: u64,
                        la_ns: &mut u64,
                        recovered: &mut Option<u64>,
                        verified: &mut bool|
     -> bool {
        let la_start = Instant::now();
        let mut full = row_part;
        full.push(submod(0, mulmod(h_mod, db, r), r));
        let rhs = mulmod(h_mod, da, r);
        if let RowStatus::Inconsistent = matrix.add_row(full, rhs) {
            la.count("inconsistent", 1);
            *la_ns += la_start.elapsed().as_nanos() as u64;
            return true;
        }
        if let Some(d) = matrix.pinned(d_col) {
            *la_ns += la_start.elapsed().as_nanos() as u64;
            let v_start = Instant::now();
            let check = g.mul(&mut ver.group_ops, generator, d);
            ver.wall_ns = v_start.elapsed().as_nanos() as u64;
            if check == target {
                *recovered = Some(d);
                *verified = true;
            } else {
                ver.count("verification_failures", 1);
            }
            return true;
        }
        *la_ns += la_start.elapsed().as_nanos() as u64;
        false
    };

    while trials < max_trials {
        if fresh {
            a = rng.gen_range(1..r);
            b = rng.gen_range(1..r);
            let ag = g.mul(&mut rel.group_ops, generator, a);
            let bq = g.mul(&mut rel.group_ops, target, b);
            point = g.add(&mut rel.group_ops, ag, bq);
            for &c in &touched {
                row[c as usize] = 0;
            }
            touched.clear();
            restarts += 1;
            if g.is_identity(&point) {
                continue;
            }
            fresh = false;
        }
        // One fused step: j and i both from the current residual.
        let key = g.key(&point);
        let j = (mix(key) as usize >> 8) % jump_count;
        let i = (mix(key ^ 0xA5A5) as usize >> 4) % f;
        point = g.add(&mut rel.group_ops, point, combined[j * f + i]);
        step_adds += 1;
        a = addmod(a, jumps[j].1, r);
        b = addmod(b, jumps[j].2, r);
        let c = fb.col_of[i];
        if row[c] == 0 {
            touched.push(c as u32);
        }
        row[c] = addmod(row[c], fb.coef_of[i], r);
        trials += 1;
        if g.is_identity(&point) {
            direct_hits += 1;
            found += 1;
            if add_relation(
                matrix,
                &mut la,
                &mut ver,
                row.clone(),
                a,
                b,
                &mut la_ns,
                &mut recovered,
                &mut verified,
            ) {
                break;
            }
            fresh = true;
            continue;
        }
        let pk = g.key(&point);
        if let Some(k) = fb.index_of_key(pk) {
            direct_hits += 1;
            found += 1;
            let mut full = row.clone();
            let kc = fb.col_of[k];
            full[kc] = addmod(full[kc], fb.coef_of[k], r);
            if add_relation(
                matrix,
                &mut la,
                &mut ver,
                full,
                a,
                b,
                &mut la_ns,
                &mut recovered,
                &mut verified,
            ) {
                break;
            }
            fresh = true;
            continue;
        }
        // Canonical sign: the representative with the smaller key.
        let kn = g.key(&g.neg(point));
        let negate = pk > kn;
        let key = if negate { kn } else { pk };
        let (sa, sb) = if negate {
            (submod(0, a, r), submod(0, b, r))
        } else {
            (a, b)
        };
        // Sparse snapshot of the nonzeros (sorted, deduplicated).
        touched.sort_unstable();
        touched.dedup();
        let srow: Vec<(u32, u64)> = touched
            .iter()
            .filter_map(|&c| {
                let v = row[c as usize];
                (v != 0).then(|| (c, if negate { submod(0, v, r) } else { v }))
            })
            .collect();
        table_probes += 1;
        if let Some(prev) = table.get(&key) {
            let da = submod(sa, prev.a, r);
            let db = submod(sb, prev.b, r);
            let mut diff = vec![0u64; d_col];
            for &(c, v) in srow.iter() {
                diff[c as usize] = v;
            }
            for &(c, v) in prev.row.iter() {
                diff[c as usize] = submod(diff[c as usize], v, r);
            }
            if da == 0 && db == 0 && diff.iter().all(|&c| c == 0) {
                self_collisions += 1;
            } else {
                collisions += 1;
                found += 1;
                if add_relation(
                    matrix,
                    &mut la,
                    &mut ver,
                    diff,
                    da,
                    db,
                    &mut la_ns,
                    &mut recovered,
                    &mut verified,
                ) {
                    break;
                }
            }
            fresh = true;
            continue;
        }
        if table.len() < TABLE_CAP {
            table_bytes_stored += entry_bytes(srow.len());
            table.insert(
                key,
                Stored {
                    a: sa,
                    b: sb,
                    row: srow.into_boxed_slice(),
                },
            );
        }
    }
    rel.wall_ns = rel_start.elapsed().as_nanos() as u64 - la_ns - ver.wall_ns;
    rel.count("trials", trials);
    rel.count("relations", found);
    rel.count("residual_collisions", collisions);
    rel.count("residual_self_collisions", self_collisions);
    rel.count("residual_direct_hits", direct_hits);
    rel.count("residual_table_entries", table.len() as u64);
    // The priced counter uses `residual`'s formula (a, b and a dense row
    // per entry) so the two collectors are compared in the same unit; the
    // bytes this layout really holds are reported beside it.
    rel.count(
        "residual_table_bytes_uncharged",
        (table.len() * (16 + 8 * d_col)) as u64,
    );
    rel.count("residual_table_bytes_stored", table_bytes_stored);
    rel.count("lookups", table_probes + trials);
    rel.count("walk_restarts", restarts);
    rel.count("walk_jumps", jump_count as u64);
    rel.count("fused_combined_jumps", combined.len() as u64);
    rel.count("walk_step_adds", step_adds);
    rel.count("walk_steps", trials);
    rel.count(
        "walk_adds_per_step_milli",
        if trials == 0 {
            0
        } else {
            step_adds * 1000 / trials
        },
    );
    la.wall_ns = la_ns;
    let (work, work_unit) = matrix.work();
    la.count(work_unit, work);
    la.count("rows", found);
    la.count("columns", cols as u64);
    la.count("rank", matrix.rank() as u64);
    outcome(
        rel, la, ver, trials, found, matrix, recovered, verified, ctr,
    )
}

#[cfg(test)]
mod tests {
    use super::super::plugins::{MitmOracle, PrimeAbscissaBase};
    use super::super::stages::{InstanceCtx, Params, Targets};
    use super::super::{run_pipeline, PipelineSpec};
    use crate::cryptanalysis::ic_boundary::{
        roster_prime_instance, Calibration, CountedGroup, GroupOps,
    };

    fn run(
        bits: u32,
        size: &str,
        planted_seed: u64,
        seed: u64,
    ) -> Result<
        (
            bool,
            Option<u64>,
            u64,
            std::collections::BTreeMap<String, u64>,
        ),
        String,
    > {
        let inst = roster_prime_instance(bits).unwrap();
        let g = inst.generator_point();
        let mut ops = GroupOps::default();
        let planted = planted_seed % inst.r;
        let q = inst.curve.mul(&mut ops, g, planted);
        let ctx = InstanceCtx {
            group: &inst.curve,
            generator: g,
            target: q,
            r: inst.r,
            cofactor: inst.cofactor,
            group_order: inst.group_order,
            name: inst.name.clone(),
            field_degree: None,
        };
        let base = PrimeAbscissaBase { instance: &inst };
        let mut spec = PipelineSpec {
            factor_base: "prime-abscissa".into(),
            oracle: "mitm".into(),
            targets: Targets::ResidualFused,
            max_trials: 4_000_000,
            seed,
            ..Default::default()
        };
        spec.factor_base_params = Params::default();
        spec.factor_base_params.set("size", size);
        spec.oracle_params.set("negation_folded", "1");
        let mut oracle = MitmOracle::new(2);
        let report = run_pipeline(
            &ctx,
            &spec,
            &base,
            &mut oracle,
            planted,
            &Calibration::default(),
            None,
        )?;
        assert_eq!(report.spec.targets.name(), "residual-fused");
        let native = report.decomposition.cost.native.clone();
        Ok((
            report.verified,
            report.recovered,
            planted,
            native.into_iter().collect(),
        ))
    }

    /// **The fused collector recovers the planted logarithm** on small
    /// prime curves, several bases and seeds, with one addition per step.
    #[test]
    fn residual_fused_recovers_the_planted_logarithm() {
        for (bits, size, planted, seed) in [
            (14, "24", 30_011, 11),
            (14, "2", 777, 3),
            (12, "8", 1234, 5),
            (16, "4", 40_001, 9),
        ] {
            let (verified, recovered, planted, native) =
                run(bits, size, planted, seed).expect("runs");
            assert!(verified, "bits {bits} size {size}: not recovered");
            assert_eq!(recovered, Some(planted));
            assert!(native.get("residual_table_entries").copied().unwrap_or(0) > 0);
            assert_eq!(native["walk_adds_per_step_milli"], 1000);
            assert_eq!(native["walk_step_adds"], native["trials"]);
            assert!(native["residual_table_bytes_uncharged"] > 0);
        }
    }

    /// A base above the combined-jump cap is refused with a clear error.
    #[test]
    fn residual_fused_refuses_an_oversized_base() {
        let err = run(14, "5000", 5, 1).unwrap_err();
        assert!(err.contains("residual-fused"), "{err}");
    }
}
