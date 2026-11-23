//! Complete single-target prime-field IC incumbent, using the exact maintained
//! `prime_fast::ic_solve_fast` implementation. This adapter supplies public
//! input validation, bounded configuration, independent scalar replay and
//! explicit accounting. It does not replace the baseline's factor base,
//! relation collector, table, matrix solve or walk.

use std::time::Instant;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::collector::{group_ledger, PublicTarget};
use crate::cryptanalysis::ic_boundary::GroupOps;
use crate::cryptanalysis::prime_fast::{self, FastCurve, IcFastReport};

pub const FAST_IC_SOURCE_REVISION: &str = "33b3bc3c7485df0fe80aae41e2093cf940d87ff3";
pub const FAST_IC_SOURCE_SHA256: &str =
    "03397cbcd5146df5341eea15febe34912bfed346d25759343a33a7b123712731";

/// These are adapter input bounds, not a performance claim or memory receipt.
/// The unmodified baseline also allocates an abscissa index and temporary
/// tables/matrices. Only an outer process envelope bounds total live memory.
pub const MAX_FAST_IC_BASE: usize = 4096;
pub const MAX_FAST_IC_EXTRA_RELATIONS: usize = 4096;
pub const MAX_FAST_IC_TARGET_WALK: u64 = 1u64 << 32;
pub const FAST_IC_SETUP_ROUND_GUARD: u64 = 50_000_000;
pub const FAST_IC_SETUP_LANES: usize = 256;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FastIcConfig {
    /// None preserves the maintained solver's auto-size rule.
    pub fb_size: Option<usize>,
    pub extra_relations: usize,
    /// This limits only the target phase. It does not bound setup collection.
    pub max_target_walk: u64,
    pub seed: u64,
}

impl Default for FastIcConfig {
    fn default() -> Self {
        Self {
            fb_size: None,
            extra_relations: 16,
            max_target_walk: 65_536,
            seed: 4188,
        }
    }
}

impl FastIcConfig {
    /// Reject unsupported dimensions before constructing native tables or
    /// matrices. The group certificate is checked separately inside the cold
    /// interval. Returning an error does not certify the target itself.
    pub fn validate_limits(&self, target: &PublicTarget) -> Result<usize, String> {
        if !(5..(1u64 << 32)).contains(&target.p)
            || target.p & 1 == 0
            || !(3..(1u64 << 32)).contains(&target.order)
            || target.order & 1 == 0
        {
            return Err(
                "fast IC requires an odd Prime31-study field and odd subgroup order below 2^32"
                    .into(),
            );
        }
        let requested_base = self
            .fb_size
            .unwrap_or_else(|| prime_fast::ic_fast_auto_fb(target.p));
        if !(1..=MAX_FAST_IC_BASE).contains(&requested_base) {
            return Err(format!(
                "fast IC factor-base request must be in 1..={MAX_FAST_IC_BASE}"
            ));
        }
        if self.extra_relations > MAX_FAST_IC_EXTRA_RELATIONS {
            return Err(format!(
                "fast IC extra relations exceed {MAX_FAST_IC_EXTRA_RELATIONS}"
            ));
        }
        if !(1..=MAX_FAST_IC_TARGET_WALK).contains(&self.max_target_walk) {
            return Err(format!(
                "fast IC target walk must be in 1..={MAX_FAST_IC_TARGET_WALK}"
            ));
        }
        // The native setup collector finishes the current 256-lane round;
        // include that possible overshoot in the checked matrix shape.
        let rows = requested_base
            .checked_add(self.extra_relations)
            .and_then(|v| v.checked_add(FAST_IC_SETUP_LANES - 1))
            .ok_or("fast IC relation-row dimension overflow")?;
        rows.checked_mul(requested_base)
            .and_then(|v| v.checked_mul(std::mem::size_of::<u64>()))
            .ok_or("fast IC matrix size overflow")?;
        requested_base
            .checked_mul(requested_base)
            .and_then(|v| v.checked_mul(16))
            .ok_or("fast IC pair-table size overflow")?;
        Ok(requested_base)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FastIcStatus {
    Recovered,
    /// The maintained Option API does not distinguish setup guard, target
    /// cap, undetermined base logs and an empty base on its failure path.
    UnrecoveredUnspecified,
}

#[derive(Clone, Debug, Serialize)]
pub struct FastIcNativeReport {
    pub factor_base_size: usize,
    pub setup_ms_diagnostic: f64,
    pub target_walk_ms_diagnostic: f64,
    pub total_ms_diagnostic: f64,
    pub target_points_walked: u64,
    pub determined_logs: usize,
}

impl From<IcFastReport> for FastIcNativeReport {
    fn from(report: IcFastReport) -> Self {
        Self {
            factor_base_size: report.factor_base_size,
            setup_ms_diagnostic: report.setup_ms,
            target_walk_ms_diagnostic: report.target_walk_ms,
            total_ms_diagnostic: report.total_ms,
            target_points_walked: report.target_points_walked,
            determined_logs: report.determined_logs,
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct FastIcResult {
    pub schema: &'static str,
    pub backend: &'static str,
    pub source_revision: &'static str,
    pub source_sha256: &'static str,
    pub config: FastIcConfig,
    pub requested_factor_base_size: usize,
    pub status: FastIcStatus,
    pub recovered_scalar: Option<u64>,
    pub independent_replay: bool,
    pub adapter_cold_seconds_diagnostic: f64,
    pub validation_seconds_diagnostic: f64,
    pub backend_seconds_diagnostic: f64,
    pub independent_replay_seconds_diagnostic: f64,
    pub native_report: Option<FastIcNativeReport>,
    /// This ledger covers public-target validation only. It is never the
    /// complete IC ledger, even when its values happen to be zero.
    pub validation_group_and_field: Value,
    /// The maintained IC entry point exposes no complete operation ledger.
    pub native_operation_counters: Option<Value>,
    pub accounting: Value,
}

/// Run one cold target through the maintained baseline. No prepared state or
/// known target logarithm is accepted. Every call constructs and pays for its
/// own factor base, table, setup relations and factor-base-log solve.
///
/// This synchronous API has no cooperative setup deadline: the native guard
/// is 50 million setup rounds of 256 lanes. The caller must retain an outer
/// wall/memory limit and record interrupted calls as incomplete. Native None
/// is an unclassified non-recovery, never an exact negative relation verdict.
/// Target transport and process input/output outside this call belong in the
/// caller's broader cold interval, not in these internal diagnostic clocks.
pub fn solve_fast_cold(
    target: &PublicTarget,
    config: &FastIcConfig,
) -> Result<FastIcResult, String> {
    let cold = Instant::now();
    let requested_factor_base_size = config.validate_limits(target)?;
    let validation = Instant::now();
    let mut validation_ops = GroupOps::default();
    target.validate(&mut validation_ops)?;
    let validation_seconds_diagnostic = validation.elapsed().as_secs_f64();

    let backend = Instant::now();
    let curve = FastCurve::new(
        "prime31-complete-ic-incumbent",
        target.p,
        target.a,
        target.b,
        target.order,
    );
    let generator = curve.point(target.generator.x, target.generator.y);
    let public_target = curve.point(target.target.x, target.target.y);
    let native = prime_fast::ic_solve_fast(
        &curve,
        generator,
        public_target,
        config.fb_size,
        config.extra_relations,
        config.max_target_walk,
        config.seed,
    );
    let backend_seconds_diagnostic = backend.elapsed().as_secs_f64();

    let replay = Instant::now();
    let (status, recovered_scalar, independent_replay, native_report) = match native {
        Some((scalar, report)) => {
            if scalar >= target.order
                || target.replay_curve().scalar(target.generator, scalar)? != target.target
            {
                return Err(
                    "fast IC returned a scalar failing independent public-point replay".into(),
                );
            }
            (
                FastIcStatus::Recovered,
                Some(scalar),
                true,
                Some(report.into()),
            )
        }
        None => (FastIcStatus::UnrecoveredUnspecified, None, false, None),
    };
    let independent_replay_seconds_diagnostic = replay.elapsed().as_secs_f64();
    let mut out = FastIcResult {
        schema: "prime31-complete-ic-incumbent/v1",
        backend: "prime_fast.ic_solve_fast",
        source_revision: FAST_IC_SOURCE_REVISION,
        source_sha256: FAST_IC_SOURCE_SHA256,
        config: config.clone(),
        requested_factor_base_size,
        status,
        recovered_scalar,
        independent_replay,
        adapter_cold_seconds_diagnostic: 0.,
        validation_seconds_diagnostic,
        backend_seconds_diagnostic,
        independent_replay_seconds_diagnostic,
        native_report,
        validation_group_and_field: group_ledger(&validation_ops),
        native_operation_counters: None,
        accounting: json!({
            "cold_targets": 1,
            "shared_setup_reuse": false,
            "factor_base_policy": "maintained_lowest_x_auto_or_explicit_size",
            "setup_included": ["public_target_validation", "native_field_context", "factor_base", "pair_table", "batched_setup_relation_collection", "factor_base_log_solve"],
            "target_phase_included": ["native_target_walk", "native_scalar_verification", "independent_affine_scalar_replay"],
            "setup_round_guard": FAST_IC_SETUP_ROUND_GUARD,
            "setup_lanes": FAST_IC_SETUP_LANES,
            "target_walk_cap_bounds_setup": false,
            "outer_process_envelope_required": true,
            "peak_process_memory": null,
            "complete_operation_ledger": false,
            "native_failure_phase_counters": null,
            "native_none_reason": "unavailable_from_maintained_option_api",
            "excluded_from_adapter_clock": ["caller_target_transport", "process_startup", "input_parsing", "returned_result_serialization", "process_shutdown"],
            "ic_pair_sorted_environment_present": std::env::var_os("IC_PAIR_SORTED").is_some(),
            "ic_multi_timing_environment_present": std::env::var_os("IC_MULTI_TIMING").is_some(),
            "timings_are_admitted_performance_evidence": false
        }),
    };
    out.adapter_cold_seconds_diagnostic = cold.elapsed().as_secs_f64();
    Ok(out)
}

/// Exact source/interface receipt for the broader specialized control. Its
/// separate source closure retains its own table and framework types; build
/// inclusion does not establish executed correctness or performance.
pub fn specialized_mitm_integration_status() -> Value {
    json!({
        "backend": "cryptanalysis.mitm-prime-batched",
        "status": "separate_exact_source_adapter_implemented",
        "repository": "isogeny-labs/cryptanalysis",
        "revision": "457ae1818aaf8134bd833ad9c1b9d57015264510",
        "full_source_manifest": "research/prime31_aux_matched_20261122/fiber-domain-followup/upstream/manifest.json",
        "full_source_manifest_schema": "prime31-specialized-incumbent-upstream/v1",
        "sources": [
            {"path":"suite/src/cryptanalysis/ic_boundary.rs", "git_blob":"ceb34fe324d4c1f3df17e2adc3e44b35dbd68502"},
            {"path":"suite/src/cryptanalysis/ic_boundary/prime_batch.rs", "git_blob":"319ead436ba8fc4a9873a83d2d7f6d1ab2db1a65"},
            {"path":"suite/src/cryptanalysis/ic_framework/stages.rs", "git_blob":"2e68ce715a21070488b471617426fc7cdd6fb952"},
            {"path":"suite/src/cryptanalysis/ic_framework/prime_batch_mitm.rs", "git_blob":"b77edb2569cf0bdde83f0456434d559ea84460fd"}
        ],
        "incompatible_existing_include": [
            "specialized builder requires PairWitnessMap; frozen crypto table uses FastMap and other layouts",
            "specialized PairTable fields prefix_inverse_bytes/vector_pairs/vector_pclmul128_pairs/vector_packed_square_pairs/vector_const_pairs/indexed_prepare_pairs differ from frozen fields parts/part_shift/packed/lines/dense/insert_attempts/index_lookups/filter_bit_sets",
            "full oracle requires its DecompositionOracle/InstanceCtx/Params framework interfaces"
        ],
        "integration": "separate pinned table/arithmetic crate with explicit checked boundary conversion; exact scalar reference on identity queries",
        "adapter": "native/src/specialized_control.rs",
        "algorithm_substitution": false,
        "qualification_evidence_required": true,
        "build_inclusion_is_not_execution_evidence": true,
        "execution_evidence_provided_by_this_receipt": false,
        "performance_evidence_provided_by_this_receipt": false
    })
}
