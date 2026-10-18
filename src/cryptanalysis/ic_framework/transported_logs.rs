//! Exact-log transport followed by a signed-Frobenius target descent.
//!
//! This candidate removes relation collection by selecting factor-base
//! representatives as known multiples `[s]G`.  It remains a complete cold
//! solve: base construction, a folded m=3 pair table, every failed target
//! probe, witness replay, log combination, and final scalar replay are
//! charged.  The construction is a mechanism screen; it does not assert that
//! these points belong to a separately prescribed geometric factor base.

use std::time::Instant;

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde::Serialize;

use super::plugins::{FrobeniusMitmOracle, KnownLogOrbitBase};
use super::stages::{DecompositionOracle, InstanceCtx, Params};
use crate::cryptanalysis::ecbench::canonical::sha256_hex;
use crate::cryptanalysis::ic_boundary::{
    price_phase, BinaryGroup, BinaryInstance, Calibration, CountedGroup, FactorBase, GroupOps,
    OracleCounters, PhaseCost,
};
use crate::cryptanalysis::ic_measurement;
use crate::cryptanalysis::koblitz_fast::FastPoint;

#[derive(Clone, Debug, Serialize)]
pub struct TransportedLogSpec {
    pub columns: usize,
    pub scalar_cap: u64,
    pub residual_seed: u64,
    pub max_attempts: u32,
}

#[derive(Clone, Debug, Serialize)]
pub struct TransportedLogAttempt {
    pub number: u32,
    pub residual_scalar: u64,
    pub residual_x: u64,
    pub residual_y: u64,
    pub witness: Option<Vec<usize>>,
    pub row: Option<Vec<u64>>,
}

#[derive(Clone, Debug, Serialize)]
pub struct TransportedLogReport {
    pub curve: String,
    pub spec: TransportedLogSpec,
    pub base_sha256: String,
    pub signed_points: usize,
    pub columns: usize,
    pub column_logs: Vec<u64>,
    pub attempts: Vec<TransportedLogAttempt>,
    pub recovered_log: Option<u64>,
    pub base: PhaseCost,
    pub table: PhaseCost,
    pub transport_verification: PhaseCost,
    pub query: PhaseCost,
    pub pdp: PhaseCost,
    pub relation_check: PhaseCost,
    pub descent: PhaseCost,
    pub recovery_check: PhaseCost,
    pub online_wall_ns: u64,
    pub online_gae: f64,
    pub total_gae: f64,
    pub verified: bool,
}

#[inline]
fn mulmod(a: u64, b: u64, r: u64) -> u64 {
    ((a as u128 * b as u128) % r as u128) as u64
}

fn base_digest(fb: &FactorBase<FastPoint>, column_logs: &[u64]) -> String {
    let mut bytes = b"ic-transported-log-base-v1\0".to_vec();
    bytes.extend_from_slice(&(fb.columns as u64).to_le_bytes());
    for &log in column_logs {
        bytes.extend_from_slice(&log.to_le_bytes());
    }
    for (p, (&col, &coef)) in fb.points.iter().zip(fb.col_of.iter().zip(&fb.coef_of)) {
        bytes.extend_from_slice(&p.x.to_le_bytes());
        bytes.extend_from_slice(&p.y.to_le_bytes());
        bytes.push(u8::from(p.infinity));
        bytes.extend_from_slice(&(col as u64).to_le_bytes());
        bytes.extend_from_slice(&coef.to_le_bytes());
    }
    sha256_hex(&bytes)
}

/// Solve one point-only target with exact transported factor-base logs.
/// The target scalar is neither accepted nor reconstructed outside the
/// measured group operations; success is a final full-point equality.
pub fn run_transported_log_target(
    inst: &BinaryInstance,
    spec: &TransportedLogSpec,
    target: FastPoint,
    calib: &Calibration,
) -> Result<TransportedLogReport, String> {
    let mut base_params = Params::default();
    base_params.set("columns", spec.columns.to_string());
    base_params.set("scalar_cap", spec.scalar_cap.to_string());
    run_transported_log_target_with_params(inst, spec, target, calib, base_params)
}

/// The one allowed post-holdout variant: retain the exact-log boundary but
/// draw representative scalars from a deterministic hash-seeded schedule.
/// This breaks the additive correlations of consecutive small scalars while
/// charging every scalar multiplication used to construct the base.
pub fn run_hash_transported_log_target(
    inst: &BinaryInstance,
    spec: &TransportedLogSpec,
    target: FastPoint,
    base_seed: u64,
    calib: &Calibration,
) -> Result<TransportedLogReport, String> {
    let mut base_params = Params::default();
    base_params.set("columns", spec.columns.to_string());
    base_params.set("scalar_cap", spec.scalar_cap.to_string());
    base_params.set("schedule", "hash");
    base_params.set("seed", base_seed.to_string());
    run_transported_log_target_with_params(inst, spec, target, calib, base_params)
}

fn run_transported_log_target_with_params(
    inst: &BinaryInstance,
    spec: &TransportedLogSpec,
    target: FastPoint,
    calib: &Calibration,
    base_params: Params,
) -> Result<TransportedLogReport, String> {
    if inst.koblitz.is_none()
        || inst.r < 3
        || spec.columns == 0
        || spec.scalar_cap == 0
        || spec.max_attempts == 0
    {
        return Err("transported-log descent needs a Koblitz instance and positive caps".into());
    }
    let group = BinaryGroup(&inst.fast);
    let ctx = InstanceCtx {
        group: &group,
        generator: inst.generator,
        target,
        r: inst.r,
        cofactor: inst.cofactor,
        group_order: inst.group_order,
        name: inst.name.clone(),
        field_degree: Some(inst.n),
    };
    let builder = KnownLogOrbitBase { instance: inst };
    let mut base_ops = GroupOps::default();
    let base_start = Instant::now();
    let (fb, column_logs) = builder.build_with_logs(&ctx, &base_params, &mut base_ops)?;
    let mut base = fb.cost.clone();
    base.group_ops.merge(base_ops);
    base.wall_ns = base_start.elapsed().as_nanos() as u64;
    if fb.columns != spec.columns || column_logs.len() != fb.columns {
        return Err("transported base has the wrong number of columns or logs".into());
    }
    let base_sha256 = base_digest(&fb, &column_logs);

    let mut oracle = FrobeniusMitmOracle::new_counted(3, inst);
    let mut table_ops = GroupOps::default();
    let table_start = Instant::now();
    oracle.prepare(&ctx, &fb, &Params::default(), &mut table_ops)?;
    let mut table = PhaseCost {
        group_ops: table_ops,
        native: oracle.setup_native(),
        wall_ns: table_start.elapsed().as_nanos() as u64,
        ..PhaseCost::default()
    };

    // Independently replay every transported column representative.  Orbit
    // coefficients are certified by the charged generator eigenvalue check
    // in the builder and by closure of each materialised orbit.
    let verify_start = Instant::now();
    let mut transport_verification = PhaseCost::default();
    let mut representatives = vec![None; fb.columns];
    for (index, &column) in fb.col_of.iter().enumerate() {
        if representatives[column].is_none() {
            representatives[column] = Some(index);
        }
    }
    for (column, (&log, representative)) in column_logs.iter().zip(representatives).enumerate() {
        let index = representative.ok_or_else(|| format!("column {column} has no point"))?;
        if fb.coef_of[index] != 1 {
            return Err(format!(
                "column {column} representative has non-unit coefficient"
            ));
        }
        if group.mul(&mut transport_verification.group_ops, inst.generator, log) != fb.points[index]
        {
            return Err(format!("transported log for column {column} failed replay"));
        }
        transport_verification.count("transported_columns_checked", 1);
    }
    transport_verification.wall_ns = verify_start.elapsed().as_nanos() as u64;

    let profile_online = std::env::var("ECBENCH_CALLGRIND_TARGET").as_deref() == Ok("1");
    if profile_online {
        ic_measurement::callgrind_dump(b"ecbench_before_online\0");
    }
    let online_start = Instant::now();
    let mut query = PhaseCost::default();
    let mut pdp = PhaseCost::default();
    let mut relation_check = PhaseCost::default();
    let mut descent = PhaseCost::default();
    let mut recovery_check = PhaseCost::default();
    let mut counters = OracleCounters::default();
    let mut attempts = Vec::new();
    let mut recovered_log = None;
    let mut verified = false;
    let mut rng = StdRng::seed_from_u64(spec.residual_seed);

    let validation_start = Instant::now();
    let valid = !target.infinity
        && inst.fast.is_on_curve(target)
        && group.is_identity(&group.mul(&mut query.group_ops, target, inst.r));
    query.wall_ns += validation_start.elapsed().as_nanos() as u64;
    if !valid {
        return Err("target is not a valid subgroup point".into());
    }
    for number in 0..spec.max_attempts {
        let query_start = Instant::now();
        let a = if number == 0 {
            0
        } else {
            rng.gen_range(1..inst.r)
        };
        let residual = if a == 0 {
            target
        } else {
            let shift = group.mul(&mut query.group_ops, inst.generator, a);
            group.add(&mut query.group_ops, target, shift)
        };
        query.wall_ns += query_start.elapsed().as_nanos() as u64;
        query.count("attempts", 1);
        let pdp_start = Instant::now();
        let witness = oracle.decompose(&ctx, &fb, &mut pdp.group_ops, &mut counters, residual);
        pdp.wall_ns += pdp_start.elapsed().as_nanos() as u64;
        let mut row = None;
        if let Some(indices) = &witness {
            let check_start = Instant::now();
            let mut sum = group.identity();
            let mut dense = vec![0u64; fb.columns];
            for &index in indices {
                let point = *fb
                    .points
                    .get(index)
                    .ok_or("target witness index outside transported base")?;
                sum = group.add(&mut relation_check.group_ops, sum, point);
                let column = fb.col_of[index];
                dense[column] =
                    ((dense[column] as u128 + fb.coef_of[index] as u128) % inst.r as u128) as u64;
            }
            relation_check.wall_ns += check_start.elapsed().as_nanos() as u64;
            relation_check.count("witnesses_checked", 1);
            if sum != residual {
                return Err("target witness does not sum to the residual".into());
            }
            let descent_start = Instant::now();
            let mut residual_log = 0u64;
            for (&coefficient, &column_log) in dense.iter().zip(&column_logs) {
                residual_log = ((residual_log as u128
                    + mulmod(coefficient, column_log, inst.r) as u128)
                    % inst.r as u128) as u64;
            }
            let log = (residual_log + inst.r - a) % inst.r;
            descent.wall_ns += descent_start.elapsed().as_nanos() as u64;
            let replay_start = Instant::now();
            verified = group.mul(&mut recovery_check.group_ops, inst.generator, log) == target;
            recovery_check.wall_ns += replay_start.elapsed().as_nanos() as u64;
            recovery_check.count("scalar_replays", 1);
            if !verified {
                return Err("recovered transported-log scalar failed full-point replay".into());
            }
            recovered_log = Some(log);
            row = Some(dense);
        }
        attempts.push(TransportedLogAttempt {
            number,
            residual_scalar: a,
            residual_x: residual.x,
            residual_y: residual.y,
            witness,
            row,
        });
        if verified {
            break;
        }
    }
    pdp.count("lookups", counters.lookups);
    pdp.count("canonicalisations", counters.canonicalisations);
    pdp.count("frobfold_mismatches", counters.frobfold_mismatches);
    pdp.count("lift_failures", counters.lift_failures);
    let mut online_wall_ns = online_start.elapsed().as_nanos() as u64;
    if profile_online {
        ic_measurement::callgrind_dump(b"ecbench_online\0");
    }
    let timed_phases = query.wall_ns
        + pdp.wall_ns
        + relation_check.wall_ns
        + descent.wall_ns
        + recovery_check.wall_ns;
    if timed_phases <= online_wall_ns {
        query.wall_ns += online_wall_ns - timed_phases;
    } else {
        online_wall_ns = timed_phases;
    }
    for phase in [
        &mut base,
        &mut table,
        &mut transport_verification,
        &mut query,
        &mut pdp,
        &mut relation_check,
        &mut descent,
        &mut recovery_check,
    ] {
        price_phase(phase, calib);
    }
    let online_gae = query.gae + pdp.gae + relation_check.gae + descent.gae + recovery_check.gae;
    let total_gae = base.gae + table.gae + transport_verification.gae + online_gae;
    Ok(TransportedLogReport {
        curve: inst.name.clone(),
        spec: spec.clone(),
        base_sha256,
        signed_points: fb.points.len(),
        columns: fb.columns,
        column_logs,
        attempts,
        recovered_log,
        base,
        table,
        transport_verification,
        query,
        pdp,
        relation_check,
        descent,
        recovery_check,
        online_wall_ns,
        online_gae,
        total_gae,
        verified,
    })
}

/// One target's online accounting inside a multi-target amortized run.
#[derive(Clone, Debug, Serialize)]
pub struct MultiTransportedLogTargetReport {
    pub attempts: Vec<TransportedLogAttempt>,
    pub recovered_log: Option<u64>,
    pub query: PhaseCost,
    pub pdp: PhaseCost,
    pub relation_check: PhaseCost,
    pub descent: PhaseCost,
    pub recovery_check: PhaseCost,
    pub online_wall_ns: u64,
    pub verified: bool,
}

/// A multi-target amortized variant of the hash-scheduled transported-log
/// candidate: one cold factor base and one folded pair table are built and
/// charged once, then every target in `targets` is solved and verified by
/// full-point scalar replay.  The amortized per-target unit divides
/// `total_gae` by the number of targets; that division belongs to the
/// analysis, not to this report, which always carries the whole cost.
#[derive(Clone, Debug, Serialize)]
pub struct MultiTransportedLogReport {
    pub curve: String,
    pub spec: TransportedLogSpec,
    pub base_sha256: String,
    pub signed_points: usize,
    pub columns: usize,
    pub column_logs: Vec<u64>,
    pub targets: Vec<MultiTransportedLogTargetReport>,
    pub base: PhaseCost,
    pub table: PhaseCost,
    pub transport_verification: PhaseCost,
    pub total_gae: f64,
    pub verified: bool,
}

/// Solve several point-only targets with one shared hash-scheduled
/// transported base and one shared folded pair table.  Every phase of every
/// target is charged; nothing is amortized inside the accounting itself.
pub fn run_hash_transported_log_multi_target(
    inst: &BinaryInstance,
    spec: &TransportedLogSpec,
    targets: &[FastPoint],
    base_seed: u64,
    calib: &Calibration,
) -> Result<MultiTransportedLogReport, String> {
    if inst.koblitz.is_none()
        || inst.r < 3
        || spec.columns == 0
        || spec.scalar_cap == 0
        || spec.max_attempts == 0
        || targets.is_empty()
    {
        return Err("multi-target transported-log descent needs a Koblitz instance, positive caps and at least one target".into());
    }
    let group = BinaryGroup(&inst.fast);
    let ctx = InstanceCtx {
        group: &group,
        generator: inst.generator,
        target: group.identity(),
        r: inst.r,
        cofactor: inst.cofactor,
        group_order: inst.group_order,
        name: inst.name.clone(),
        field_degree: Some(inst.n),
    };
    let builder = KnownLogOrbitBase { instance: inst };
    let mut base_params = Params::default();
    base_params.set("columns", spec.columns.to_string());
    base_params.set("scalar_cap", spec.scalar_cap.to_string());
    base_params.set("schedule", "hash");
    base_params.set("seed", base_seed.to_string());
    let mut base_ops = GroupOps::default();
    let base_start = Instant::now();
    let (fb, column_logs) = builder.build_with_logs(&ctx, &base_params, &mut base_ops)?;
    let mut base = fb.cost.clone();
    base.group_ops.merge(base_ops);
    base.wall_ns = base_start.elapsed().as_nanos() as u64;
    if fb.columns != spec.columns || column_logs.len() != fb.columns {
        return Err("transported base has the wrong number of columns or logs".into());
    }
    let base_sha256 = base_digest(&fb, &column_logs);

    let mut oracle = FrobeniusMitmOracle::new_counted(3, inst);
    let mut table_ops = GroupOps::default();
    let table_start = Instant::now();
    oracle.prepare(&ctx, &fb, &Params::default(), &mut table_ops)?;
    let mut table = PhaseCost {
        group_ops: table_ops,
        native: oracle.setup_native(),
        wall_ns: table_start.elapsed().as_nanos() as u64,
        ..PhaseCost::default()
    };

    // Independently replay every transported column representative, exactly
    // as the one-target runner does, before any target work is charged.
    let verify_start = Instant::now();
    let mut transport_verification = PhaseCost::default();
    let mut representatives = vec![None; fb.columns];
    for (index, &column) in fb.col_of.iter().enumerate() {
        if representatives[column].is_none() {
            representatives[column] = Some(index);
        }
    }
    for (column, (&log, representative)) in column_logs.iter().zip(representatives).enumerate() {
        let index = representative.ok_or_else(|| format!("column {column} has no point"))?;
        if fb.coef_of[index] != 1 {
            return Err(format!(
                "column {column} representative has non-unit coefficient"
            ));
        }
        if group.mul(&mut transport_verification.group_ops, inst.generator, log) != fb.points[index]
        {
            return Err(format!("transported log for column {column} failed replay"));
        }
        transport_verification.count("transported_columns_checked", 1);
    }
    transport_verification.wall_ns = verify_start.elapsed().as_nanos() as u64;

    let mut counters = OracleCounters::default();
    let mut targets_out = Vec::with_capacity(targets.len());
    let mut all_verified = true;
    for (target_index, &target) in targets.iter().enumerate() {
        let mut query = PhaseCost::default();
        let mut pdp = PhaseCost::default();
        let mut relation_check = PhaseCost::default();
        let mut descent = PhaseCost::default();
        let mut recovery_check = PhaseCost::default();
        let mut attempts = Vec::new();
        let mut recovered_log = None;
        let mut verified = false;
        // Each target draws its own residual stream so the per-target
        // schedules stay independent of the workload's target order.
        let mut rng = StdRng::seed_from_u64(
            spec.residual_seed
                ^ base_seed
                ^ (0x9E37_79B9_7F4A_7C15u64.wrapping_mul(target_index as u64 + 1)),
        );
        let validation_start = Instant::now();
        let valid = !target.infinity
            && inst.fast.is_on_curve(target)
            && group.is_identity(&group.mul(&mut query.group_ops, target, inst.r));
        query.wall_ns += validation_start.elapsed().as_nanos() as u64;
        if !valid {
            return Err(format!("multi-target {target_index} is not a valid subgroup point"));
        }
        for number in 0..spec.max_attempts {
            let query_start = Instant::now();
            let a = if number == 0 {
                0
            } else {
                rng.gen_range(1..inst.r)
            };
            let residual = if a == 0 {
                target
            } else {
                let shift = group.mul(&mut query.group_ops, inst.generator, a);
                group.add(&mut query.group_ops, target, shift)
            };
            query.wall_ns += query_start.elapsed().as_nanos() as u64;
            query.count("attempts", 1);
            let pdp_start = Instant::now();
            let witness = oracle.decompose(&ctx, &fb, &mut pdp.group_ops, &mut counters, residual);
            pdp.wall_ns += pdp_start.elapsed().as_nanos() as u64;
            let mut row = None;
            if let Some(indices) = &witness {
                let check_start = Instant::now();
                let mut sum = group.identity();
                let mut dense = vec![0u64; fb.columns];
                for &index in indices {
                    let point = *fb
                        .points
                        .get(index)
                        .ok_or("target witness index outside transported base")?;
                    sum = group.add(&mut relation_check.group_ops, sum, point);
                    let column = fb.col_of[index];
                    dense[column] =
                        ((dense[column] as u128 + fb.coef_of[index] as u128) % inst.r as u128)
                            as u64;
                }
                relation_check.wall_ns += check_start.elapsed().as_nanos() as u64;
                relation_check.count("witnesses_checked", 1);
                if sum != residual {
                    return Err("target witness does not sum to the residual".into());
                }
                let descent_start = Instant::now();
                let mut residual_log = 0u64;
                for (&coefficient, &column_log) in dense.iter().zip(&column_logs) {
                    residual_log = ((residual_log as u128
                        + mulmod(coefficient, column_log, inst.r) as u128)
                        % inst.r as u128) as u64;
                }
                let log = (residual_log + inst.r - a) % inst.r;
                descent.wall_ns += descent_start.elapsed().as_nanos() as u64;
                let replay_start = Instant::now();
                verified =
                    group.mul(&mut recovery_check.group_ops, inst.generator, log) == target;
                recovery_check.wall_ns += replay_start.elapsed().as_nanos() as u64;
                recovery_check.count("scalar_replays", 1);
                if !verified {
                    return Err("recovered transported-log scalar failed full-point replay".into());
                }
                recovered_log = Some(log);
                row = Some(dense);
            }
            attempts.push(TransportedLogAttempt {
                number,
                residual_scalar: a,
                residual_x: residual.x,
                residual_y: residual.y,
                witness,
                row,
            });
            if verified {
                break;
            }
        }
        let online_wall_ns = query.wall_ns
            + pdp.wall_ns
            + relation_check.wall_ns
            + descent.wall_ns
            + recovery_check.wall_ns;
        all_verified &= verified;
        targets_out.push(MultiTransportedLogTargetReport {
            attempts,
            recovered_log,
            query,
            pdp,
            relation_check,
            descent,
            recovery_check,
            online_wall_ns,
            verified,
        });
    }
    pdp_transfer_counters(&mut targets_out, &mut counters);
    price_phase(&mut base, calib);
    price_phase(&mut table, calib);
    price_phase(&mut transport_verification, calib);
    let mut online_gae = 0.0;
    for target in &mut targets_out {
        price_phase(&mut target.query, calib);
        price_phase(&mut target.pdp, calib);
        price_phase(&mut target.relation_check, calib);
        price_phase(&mut target.descent, calib);
        price_phase(&mut target.recovery_check, calib);
        online_gae += target.query.gae
            + target.pdp.gae
            + target.relation_check.gae
            + target.descent.gae
            + target.recovery_check.gae;
    }
    let total_gae = base.gae + table.gae + transport_verification.gae + online_gae;
    Ok(MultiTransportedLogReport {
        curve: inst.name.clone(),
        spec: spec.clone(),
        base_sha256,
        signed_points: fb.points.len(),
        columns: fb.columns,
        column_logs,
        targets: targets_out,
        base,
        table,
        transport_verification,
        total_gae,
        verified: all_verified,
    })
}

/// The shared oracle counters are attributed once, to the last target's
/// PDP phase, so the record keeps the total lookups and failures without
/// double charging.
fn pdp_transfer_counters(
    targets: &mut [MultiTransportedLogTargetReport],
    counters: &mut OracleCounters,
) {
    if let Some(last) = targets.last_mut() {
        last.pdp.count("lookups", counters.lookups);
        last.pdp.count("canonicalisations", counters.canonicalisations);
        last.pdp.count("frobfold_mismatches", counters.frobfold_mismatches);
        last.pdp.count("lift_failures", counters.lift_failures);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cryptanalysis::ic_boundary::koblitz_instance;

    #[test]
    fn transported_logs_recover_point_only_targets_with_full_replay() {
        let inst = koblitz_instance(1, 17).expect("test curve");
        for (index, scalar) in [7u64, 113, 257].into_iter().enumerate() {
            let target = inst.fast.mul_u64(inst.generator, scalar);
            let report = run_transported_log_target(
                &inst,
                &TransportedLogSpec {
                    columns: 4,
                    scalar_cap: 100,
                    residual_seed: 19 + index as u64,
                    max_attempts: 256,
                },
                target,
                &Calibration::default(),
            )
            .unwrap();
            assert!(report.verified);
            assert_eq!(report.recovered_log, Some(scalar % inst.r));
            assert_eq!(
                report
                    .transport_verification
                    .get("transported_columns_checked"),
                4
            );
            assert_eq!(report.relation_check.get("witnesses_checked"), 1);
            assert_eq!(report.recovery_check.get("scalar_replays"), 1);
        }
    }

    #[test]
    fn hash_scheduled_transport_recovers_and_changes_the_base() {
        let inst = koblitz_instance(1, 17).expect("test curve");
        let scalar = 113u64;
        let target = inst.fast.mul_u64(inst.generator, scalar);
        let spec = TransportedLogSpec {
            columns: 4,
            scalar_cap: 100,
            residual_seed: 23,
            max_attempts: 256,
        };
        let sequential =
            run_transported_log_target(&inst, &spec, target, &Calibration::default()).unwrap();
        let hashed = run_hash_transported_log_target(
            &inst,
            &spec,
            target,
            20261014314005,
            &Calibration::default(),
        )
        .unwrap();
        assert!(hashed.verified);
        assert_eq!(hashed.recovered_log, Some(scalar % inst.r));
        assert_ne!(hashed.base_sha256, sequential.base_sha256);
        assert!(hashed.base.group_ops.scalar_mults > sequential.base.group_ops.scalar_mults);
    }

    #[test]
    fn multi_target_transport_amortizes_one_base_and_recovers_every_target() {
        let inst = koblitz_instance(1, 17).expect("test curve");
        let scalars = [7u64, 113, 257, 389];
        let targets: Vec<FastPoint> = scalars
            .iter()
            .map(|&s| inst.fast.mul_u64(inst.generator, s))
            .collect();
        let report = run_hash_transported_log_multi_target(
            &inst,
            &TransportedLogSpec {
                columns: 4,
                scalar_cap: 100,
                residual_seed: 29,
                max_attempts: 256,
            },
            &targets,
            20261014314006,
            &Calibration::default(),
        )
        .unwrap();
        assert!(report.verified);
        assert_eq!(report.targets.len(), 4);
        for (target_report, &scalar) in report.targets.iter().zip(scalars.iter()) {
            assert!(target_report.verified);
            assert_eq!(target_report.recovered_log, Some(scalar % inst.r));
            assert_eq!(target_report.recovery_check.get("scalar_replays"), 1);
        }
        // One shared base, one shared table, one shared verification pass.
        assert_eq!(
            report.transport_verification.get("transported_columns_checked"),
            4
        );
        assert_eq!(report.columns, 4);
    }
}
