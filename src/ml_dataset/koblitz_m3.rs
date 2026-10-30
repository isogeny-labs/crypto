//! Adapter-side cumulative reservations for the bounded Koblitz m3 pipeline.
//!
//! This module is not yet registered in runtime dispatch. Its constructor must
//! receive limits from a validated original SolverExecutionContext, never from
//! a producer result. Reservations model admitted work/bytes, not RSS or retired
//! instructions. Native work and actual group additions have different units.
use std::time::{Duration, Instant};

const NATIVE_WORK_MAX: u64 = 16_777_216;
const NATIVE_BYTES_MAX: u64 = 64 * 1024 * 1024;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Stop {
    InvalidLimits,
    Overflow,
    Deadline,
    Phase,
    Nonmonotone,
    Work,
    Bytes,
    Trace,
    Additions,
    Observer,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Phase {
    Setup,
    Construction,
    Constructed,
    Matrix,
    MatrixComplete,
    Extraction,
    Complete,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) struct Reservation {
    pub work: u64,
    pub bytes: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct NativeLimits {
    pub work: u64,
    pub bytes: u64,
}

/// One ledger per original attempt. No phase refunds cumulative reservations.
#[derive(Debug)]
pub(super) struct Budget {
    native_work_limit: u64,
    memory_limit: u64,
    trace_limit: u64,
    addition_limit: u64,
    deadline: Instant,
    construction_deadline: Instant,
    phase: Phase,
    native: Reservation,
    phase_native: Reservation,
    phase_limits: NativeLimits,
    adapter_bytes: u64,
    trace_bytes: u64,
    additions: u64,
    stopped: Option<Stop>,
}

impl Budget {
    pub fn new(
        configured_work: u64,
        memory_bytes: u64,
        trace_bytes: u64,
        original_deadline: Instant,
        entry: Instant,
    ) -> Result<Self, Stop> {
        if configured_work == 0 || memory_bytes == 0 || trace_bytes == 0 {
            return Err(Stop::InvalidLimits);
        }
        let work = configured_work.checked_mul(1024).ok_or(Stop::Overflow)?;
        if entry >= original_deadline {
            return Err(Stop::Deadline);
        }
        let construction_deadline = entry
            .checked_add(Duration::from_secs(5))
            .ok_or(Stop::Overflow)?
            .min(original_deadline);
        Ok(Self {
            native_work_limit: work.min(NATIVE_WORK_MAX),
            memory_limit: memory_bytes,
            trace_limit: trace_bytes,
            addition_limit: configured_work,
            deadline: original_deadline,
            construction_deadline,
            phase: Phase::Setup,
            native: Reservation::default(),
            phase_native: Reservation::default(),
            phase_limits: NativeLimits { work: 0, bytes: 0 },
            adapter_bytes: 0,
            trace_bytes: 0,
            additions: 0,
            stopped: None,
        })
    }

    fn fail<T>(&mut self, stop: Stop) -> Result<T, Stop> {
        let first = *self.stopped.get_or_insert(stop);
        Err(first)
    }

    fn check(&mut self, now: Instant) -> Result<(), Stop> {
        if let Some(stop) = self.stopped {
            return Err(stop);
        }
        let deadline = if matches!(self.phase, Phase::Setup | Phase::Construction) {
            self.construction_deadline
        } else {
            self.deadline
        };
        if now >= deadline {
            return self.fail(Stop::Deadline);
        }
        if self.phase == Phase::Complete {
            return self.fail(Stop::Phase);
        }
        Ok(())
    }

    /// Reserve adapter-owned allocations before allocating, including callback
    /// storage and serialized payload capacity. Native reservations are separate.
    pub fn reserve_bytes(&mut self, bytes: u64, now: Instant) -> Result<(), Stop> {
        self.check(now)?;
        let Some(next) = self.adapter_bytes.checked_add(bytes) else {
            return self.fail(Stop::Overflow);
        };
        if next > self.memory_limit - self.native.bytes {
            return self.fail(Stop::Bytes);
        }
        self.adapter_bytes = next;
        Ok(())
    }

    pub fn reserve_trace(&mut self, bytes: u64, now: Instant) -> Result<(), Stop> {
        self.check(now)?;
        let Some(next) = self.trace_bytes.checked_add(bytes) else {
            return self.fail(Stop::Overflow);
        };
        if next > self.trace_limit {
            return self.fail(Stop::Trace);
        }
        self.trace_bytes = next;
        Ok(())
    }

    /// The returned allowance is a phase ceiling. Call observe() at every native
    /// boundary; callback allocations can reduce remaining shared headroom.
    pub fn begin_native(&mut self, matrix: bool, now: Instant) -> Result<NativeLimits, Stop> {
        self.check(now)?;
        let expected = if matrix {
            Phase::Constructed
        } else {
            Phase::Setup
        };
        if self.phase != expected {
            return self.fail(Stop::Phase);
        }
        let remaining_bytes = self.memory_limit - self.adapter_bytes - self.native.bytes;
        let bytes = if matrix {
            remaining_bytes.min(NATIVE_BYTES_MAX)
        } else {
            remaining_bytes
                .min(self.memory_limit / 4)
                .min(NATIVE_BYTES_MAX)
        };
        let limits = NativeLimits {
            work: self.native_work_limit - self.native.work,
            bytes,
        };
        self.phase = if matrix {
            Phase::Matrix
        } else {
            Phase::Construction
        };
        self.phase_native = Reservation::default();
        self.phase_limits = limits;
        Ok(limits)
    }

    /// Native APIs report cumulative phase reservations. Convert them to deltas
    /// exactly once and reject regression instead of silently refunding work.
    pub fn observe(&mut self, current: Reservation, now: Instant) -> Result<(), Stop> {
        self.check(now)?;
        if !matches!(self.phase, Phase::Construction | Phase::Matrix) {
            return self.fail(Stop::Phase);
        }
        if current.work < self.phase_native.work || current.bytes < self.phase_native.bytes {
            return self.fail(Stop::Nonmonotone);
        }
        let dw = current.work - self.phase_native.work;
        let db = current.bytes - self.phase_native.bytes;
        if current.work > self.phase_limits.work || dw > self.native_work_limit - self.native.work {
            return self.fail(Stop::Work);
        }
        if current.bytes > self.phase_limits.bytes
            || db > self.memory_limit - self.native.bytes - self.adapter_bytes
        {
            return self.fail(Stop::Bytes);
        }
        self.native.work += dw;
        self.native.bytes += db;
        self.phase_native = current;
        Ok(())
    }

    /// Successful phase completion is supplied only after native terminal status
    /// validation. A stopped native report must instead stop this ledger.
    pub fn finish_native(&mut self, now: Instant) -> Result<(), Stop> {
        self.check(now)?;
        self.phase = match self.phase {
            Phase::Construction => Phase::Constructed,
            Phase::Matrix => Phase::MatrixComplete,
            _ => return self.fail(Stop::Phase),
        };
        Ok(())
    }

    pub fn begin_extraction(&mut self, matrix_required: bool, now: Instant) -> Result<(), Stop> {
        self.check(now)?;
        let expected = if matrix_required {
            Phase::MatrixComplete
        } else {
            Phase::Constructed
        };
        if self.phase != expected {
            return self.fail(Stop::Phase);
        }
        self.phase = Phase::Extraction;
        Ok(())
    }

    /// Call immediately before each actual group.add, including identity starts.
    pub fn reserve_addition(&mut self, now: Instant) -> Result<(), Stop> {
        self.check(now)?;
        if self.phase != Phase::Extraction {
            return self.fail(Stop::Phase);
        }
        if self.additions == self.addition_limit {
            return self.fail(Stop::Additions);
        }
        self.additions += 1;
        Ok(())
    }

    /// The caller must separately establish the exact final cursor. Consuming
    /// exactly the allowance does not itself prevent successful completion.
    pub fn finish_extraction(&mut self, now: Instant) -> Result<(), Stop> {
        self.check(now)?;
        if self.phase != Phase::Extraction {
            return self.fail(Stop::Phase);
        }
        self.phase = Phase::Complete;
        Ok(())
    }

    pub fn observer_failed(&mut self) -> Result<(), Stop> {
        self.fail(Stop::Observer)
    }
}

/// Admit persisted originals and the actual capture sink before any arithmetic.
/// This function is staged with the module under test-only registration until
/// the full adapter and its semantic verifier are ready.
pub(super) fn admit_original(
    dir: &std::path::Path,
    config: &super::super::types::CaseConfig,
    context: &super::super::execution::SolverExecutionContext,
    sink: &super::super::capture::CaptureEmitter,
) -> Result<Budget, String> {
    use super::super::{admit_capture_context, execution, read_json};
    // Sample entry before file admission: setup consumes construction time.
    let entry = Instant::now();
    let saved: super::super::types::CaseConfig = read_json(&dir.join("config.json"))?;
    if saved != *config {
        return Err("Koblitz m3 config differs from original attempt".into());
    }
    let original =
        execution::admit(dir)?.ok_or("Koblitz m3 requires original execution context")?;
    if original.launch != context.launch {
        return Err("Koblitz m3 execution context differs from saved original".into());
    }
    let (binding, limits) =
        admit_capture_context(dir)?.ok_or("Koblitz m3 requires original capture context")?;
    if binding != original.launch.binding {
        return Err("Koblitz m3 capture and execution attempts differ".into());
    }
    sink.verify_attempt_limits(&binding, &limits)?;
    let mut budget = Budget::new(
        saved.work_limit,
        original.launch.memory_bytes,
        original.launch.trace_bytes,
        context.deadline.min(original.deadline),
        entry,
    )
    .map_err(|s| format!("Koblitz m3 original resource admission: {s:?}"))?;
    budget
        .check(Instant::now())
        .map_err(|s| format!("Koblitz m3 setup deadline: {s:?}"))?;
    Ok(budget)
}

/// Canonical original mathematical inputs for the initial two-code matrix
/// profile. Setup admission/accounting and persistence must precede runtime use.
pub(super) struct MathInput {
    pub curve: Box<crate::binary_ecc::BinaryCurve>,
    pub target: (u64, u64),
    pub codes: [u64; 2],
    pub signed: [(u64, u64); 4],
}

/// Validate public inputs using the existing independent curve/factor-base
/// replay. Never consult private_witness or solver-produced input artifacts.
/// The caller authenticates case provenance; this checks mathematical content.
pub(super) fn original_math(
    config: &super::super::types::CaseConfig,
    case: &super::super::types::CaseData,
) -> Result<MathInput, String> {
    use super::{curves, domain, Element, Group};
    use num_traits::ToPrimitive;
    use serde_json::json;
    if config.family != "koblitz"
        || config.m != 3
        || !((23..=31).contains(&config.n) || (47..=61).contains(&config.n))
    {
        return Err("outside Koblitz m3 field/arity profile".into());
    }
    let public = &case.public_instance;
    if public["family"] != config.family
        || public["n"] != json!(config.n)
        || public["m"] != json!(config.m)
        || public["twist"] != json!(config.twist)
        || case.factor_base["requested_size"] != json!(config.factor_base_size)
    {
        return Err("Koblitz original public inputs differ from configuration".into());
    }
    let points = case.factor_base["points"]
        .as_array()
        .ok_or("missing original factor base")?;
    if points.len() > 4 || points.is_empty() {
        return Err("Koblitz two-code profile requires at most four original signed points".into());
    }
    // Full mathematical checks include actual twist, certified order,
    // irreducibility, subgroup checks and factor-base construction replay.
    // Their setup cost is not represented by native S4 reservation counters.
    curves::verify_public(public)?;
    curves::verify_factor_base(public, &case.factor_base)?;
    let group = Group::read(case)?;
    let Group::Binary(curve) = &group else {
        return Err("Koblitz profile requires actual binary curve".into());
    };
    if curve.m != config.n || curve.b.to_biguint().to_u64() != Some(1) {
        return Err("Koblitz S4 profile requires actual b=1 and original field degree".into());
    }
    let word = |p: &Element| -> Result<(u64, u64), String> {
        match p {
            Element::Binary(crate::binary_ecc::BinaryPoint::Affine { x, y }) => Ok((
                x.to_biguint().to_u64().ok_or("x exceeds u64")?,
                y.to_biguint().to_u64().ok_or("y exceeds u64")?,
            )),
            _ => Err("Koblitz original point must be affine".into()),
        }
    };
    let target = word(&group.point(&public["target"])?)?;
    let points = domain(&group, case)?;
    if points.len() != 4 {
        return Err(
            "Koblitz matrix profile requires two distinct x codes, not two signed points".into(),
        );
    }
    let mut signed = [(0, 0); 4];
    for (out, point) in signed.iter_mut().zip(&points) {
        *out = word(point)?;
    }
    signed.sort_unstable();
    if signed.windows(2).any(|p| p[0] == p[1])
        || signed[0].0 != signed[1].0
        || signed[2].0 != signed[3].0
        || signed[0].0 == signed[2].0
        || signed[0].1 ^ signed[0].0 != signed[1].1
        || signed[2].1 ^ signed[2].0 != signed[3].1
    {
        return Err("noncanonical original signed two-code domain".into());
    }
    let codes = [signed[0].0, signed[2].0];
    let Group::Binary(mut curve) = group else {
        unreachable!("validated binary group")
    };
    curve.irreducible.low_terms.sort_unstable();
    Ok(MathInput {
        curve,
        target,
        codes,
        signed,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn budget(work: u64, bytes: u64) -> (Budget, Instant) {
        let now = Instant::now();
        (
            Budget::new(work, bytes, 100, now + Duration::from_secs(20), now).unwrap(),
            now,
        )
    }
    fn constructed(b: &mut Budget, now: Instant, r: Reservation) {
        b.begin_native(false, now).unwrap();
        b.observe(r, now).unwrap();
        b.finish_native(now).unwrap();
    }

    #[test]
    fn native_phases_share_work_and_retained_memory() {
        let (mut b, now) = budget(10, 1000);
        b.reserve_bytes(50, now).unwrap();
        constructed(
            &mut b,
            now,
            Reservation {
                work: 3000,
                bytes: 250,
            },
        );
        assert_eq!(
            b.begin_native(true, now),
            Ok(NativeLimits {
                work: 7240,
                bytes: 700
            })
        );
        b.observe(
            Reservation {
                work: 7240,
                bytes: 700,
            },
            now,
        )
        .unwrap();
        assert_eq!(
            b.native,
            Reservation {
                work: 10240,
                bytes: 950
            }
        );
        assert_eq!(b.reserve_bytes(1, now), Err(Stop::Bytes));
    }

    #[test]
    fn repeated_cumulative_event_is_not_double_charged() {
        let (mut b, now) = budget(10, 1000);
        b.begin_native(false, now).unwrap();
        let r = Reservation {
            work: 100,
            bytes: 100,
        };
        b.observe(r, now).unwrap();
        b.observe(r, now).unwrap();
        assert_eq!(b.native, r);
        assert_eq!(
            b.observe(
                Reservation {
                    work: 99,
                    bytes: 100
                },
                now
            ),
            Err(Stop::Nonmonotone)
        );
        assert_eq!(b.finish_native(now), Err(Stop::Nonmonotone));
    }

    #[test]
    fn callback_memory_can_reduce_native_headroom() {
        let (mut b, now) = budget(10, 1000);
        constructed(
            &mut b,
            now,
            Reservation {
                work: 1,
                bytes: 200,
            },
        );
        b.begin_native(true, now).unwrap();
        b.reserve_bytes(750, now).unwrap();
        b.observe(Reservation { work: 1, bytes: 50 }, now).unwrap();
        assert_eq!(
            b.observe(Reservation { work: 1, bytes: 51 }, now),
            Err(Stop::Bytes)
        );
        assert_eq!(b.native.bytes, 250);
    }

    #[test]
    fn construction_deadline_includes_setup_and_never_restarts() {
        let (mut b, now) = budget(10, 1000);
        b.begin_native(false, now + Duration::from_secs(4)).unwrap();
        assert_eq!(
            b.observe(Reservation::default(), now + Duration::from_secs(5)),
            Err(Stop::Deadline)
        );
        assert_eq!(b.reserve_trace(1, now), Err(Stop::Deadline));
    }

    #[test]
    fn matrix_uses_original_deadline() {
        let (mut b, now) = budget(10, 1000);
        constructed(&mut b, now, Reservation::default());
        b.begin_native(true, now + Duration::from_secs(6)).unwrap();
        assert_eq!(
            b.finish_native(now + Duration::from_secs(20)),
            Err(Stop::Deadline)
        );
    }

    #[test]
    fn exact_addition_limit_can_finish_but_extra_call_fails() {
        for extra in [false, true] {
            let (mut b, now) = budget(2, 1000);
            constructed(&mut b, now, Reservation::default());
            b.begin_extraction(false, now).unwrap();
            b.reserve_addition(now).unwrap();
            b.reserve_addition(now).unwrap();
            if extra {
                assert_eq!(b.reserve_addition(now), Err(Stop::Additions));
                assert_eq!(b.finish_extraction(now), Err(Stop::Additions));
            } else {
                b.finish_extraction(now).unwrap();
            }
        }
    }

    #[test]
    fn matrix_cannot_be_skipped_when_required_or_run_twice() {
        let (mut b, now) = budget(2, 1000);
        constructed(&mut b, now, Reservation::default());
        assert_eq!(b.begin_extraction(true, now), Err(Stop::Phase));
        let (mut b, now) = budget(2, 1000);
        constructed(&mut b, now, Reservation::default());
        b.begin_native(true, now).unwrap();
        b.finish_native(now).unwrap();
        assert_eq!(b.begin_native(true, now), Err(Stop::Phase));
    }

    #[test]
    fn overflow_zero_trace_limit_and_observer_failure_are_explicit() {
        let (mut b, now) = budget(2, 1000);
        assert!(matches!(
            Budget::new(u64::MAX, 1000, 100, now + Duration::from_secs(1), now),
            Err(Stop::Overflow)
        ));
        assert!(matches!(
            Budget::new(1, 1000, 0, now + Duration::from_secs(1), now),
            Err(Stop::InvalidLimits)
        ));
        b.reserve_trace(100, now).unwrap();
        assert_eq!(b.reserve_trace(1, now), Err(Stop::Trace));
        assert_eq!(b.observer_failed(), Err(Stop::Trace));
        let (mut b, now) = budget(2, 1000);
        assert_eq!(b.observer_failed(), Err(Stop::Observer));
        assert_eq!(b.begin_native(false, now), Err(Stop::Observer));
    }

    #[test]
    fn native_one_over_work_or_builder_bytes_is_rejected_before_mutation() {
        for (r, stop) in [
            (
                Reservation {
                    work: 1025,
                    bytes: 0,
                },
                Stop::Work,
            ),
            (
                Reservation {
                    work: 0,
                    bytes: 251,
                },
                Stop::Bytes,
            ),
        ] {
            let (mut b, now) = budget(1, 1000);
            b.begin_native(false, now).unwrap();
            assert_eq!(b.observe(r, now), Err(stop));
            assert_eq!(b.native, Reservation::default());
        }
    }
}

#[cfg(test)]
mod admission_tests {
    use super::*;
    use crate::ml_dataset::{self as dataset, capture, execution, storage, telemetry};
    use serde_json::json;
    use std::path::PathBuf;

    struct Fixture {
        dir: PathBuf,
        config: dataset::types::CaseConfig,
        context: execution::SolverExecutionContext,
        sink: capture::CaptureEmitter,
    }
    impl Fixture {
        fn new() -> Self {
            let nonce = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos();
            let dir = std::env::temp_dir()
                .join(format!("koblitz-admission-{}-{nonce}", std::process::id()));
            std::fs::create_dir(&dir).unwrap();
            let mut protocol = json!({"schema":"experiment.protocol/v1", "purpose":"Koblitz adapter software admission control"});
            let body = dataset::protocol_canonical_bytes(&protocol).unwrap();
            protocol["freeze"] = json!({"status":"frozen", "protocol_sha256":dataset::digest(&body), "filed_before_affected_data":true, "frozen_at":"2026-10-30T00:00:00Z"});
            let bytes = serde_json::to_vec(&protocol).unwrap();
            let plan = dataset::DatasetPlan {
                schema: "crypto.ml-dataset.plan/v1".into(),
                field_sizes: vec![23],
                arities: vec![3],
                families: vec!["koblitz".into()],
                solvers: vec!["f4".into()],
                curve_seeds: 1,
                generators: 1,
                keys: 1,
                seed: 42,
                factor_base_size: 2,
                degree_bound: 3,
                work_limit: 100,
                twists: true,
                protocol_sha256: Some(dataset::digest(&bytes)),
                split_rule: "curve-twist-component-sha256-v1".into(),
            };
            let hash = plan.binding_sha256().unwrap();
            let config = plan.cell(0).unwrap();
            let run = json!({"schema":"crypto.ml-dataset.run/v1", "plan_sha256":hash,
                "source_commit":"a".repeat(40), "binary_sha256":"b".repeat(64),
                "seconds_per_case":30,"memory_mib":64,"trace_mib":16,"execution_context_required":true});
            storage::write_json(&dir, "run.json", "run", &run).unwrap();
            storage::write_json(&dir, "plan.json", "plan", &plan).unwrap();
            storage::write_json(&dir, "config.json", "config", &config).unwrap();
            storage::write_json(
                &dir,
                "attempt.json",
                "attempt",
                &json!({"schema":"crypto.ml-dataset.attempt/v1","cell_index":0,"plan_sha256":hash}),
            )
            .unwrap();
            storage::write_bytes(&dir, "protocol.json", "protocol", "json", &bytes).unwrap();
            execution::create(&dir).unwrap();
            telemetry::checked::install_policy(&dir).unwrap();
            let binding = capture::AttemptBinding::from_cell_files(&dir).unwrap();
            let limits = capture::capture_limits(16).unwrap();
            storage::write_json(
                &dir,
                "capture-binding.json",
                "partial-capture-binding",
                &binding,
            )
            .unwrap();
            storage::write_json(
                &dir,
                "capture-limits.json",
                "partial-capture-limits",
                &limits,
            )
            .unwrap();
            let sink = capture::CaptureEmitter::create(&dir.join("live"), binding, limits).unwrap();
            let context = execution::admit(&dir).unwrap().unwrap();
            Self {
                dir,
                config,
                context,
                sink,
            }
        }
        fn admit(&self) -> Result<Budget, String> {
            admit_original(&self.dir, &self.config, &self.context, &self.sink)
        }
    }
    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.dir);
        }
    }

    #[test]
    fn admission_uses_original_limits_and_never_extends_earlier_deadline() {
        let mut f = Fixture::new();
        f.context.deadline = Instant::now() + Duration::from_secs(2);
        let b = f.admit().unwrap();
        assert_eq!(b.native_work_limit, 102400);
        assert_eq!(b.addition_limit, 100);
        assert_eq!(b.memory_limit, 64 << 20);
        assert_eq!(b.trace_limit, 16 << 20);
        assert_eq!(b.deadline, f.context.deadline);
        let before = std::fs::read(f.dir.join("live/committed.json")).unwrap();
        f.context.deadline = Instant::now();
        assert!(f.admit().unwrap_err().contains("Deadline"));
        assert_eq!(
            std::fs::read(f.dir.join("live/committed.json")).unwrap(),
            before
        );
    }

    #[test]
    fn admission_rejects_config_context_capture_and_saved_run_mutations() {
        let mut f = Fixture::new();
        f.config.work_limit += 1;
        assert!(f.admit().unwrap_err().contains("config differs"));
        f.config.work_limit -= 1;
        f.context.launch.memory_bytes += 1;
        assert!(f.admit().unwrap_err().contains("context differs"));
        f.context.launch.memory_bytes -= 1;
        let original = std::fs::read(f.dir.join("capture-limits.json")).unwrap();
        std::fs::write(
            f.dir.join("capture-limits.json"),
            serde_json::to_vec(&capture::capture_limits(8).unwrap()).unwrap(),
        )
        .unwrap();
        assert!(f.admit().unwrap_err().contains("capture limits differ"));
        std::fs::write(f.dir.join("capture-limits.json"), original).unwrap();
        let mut run: serde_json::Value = dataset::read_json(&f.dir.join("run.json")).unwrap();
        run["memory_mib"] = json!(128);
        std::fs::write(f.dir.join("run.json"), serde_json::to_vec(&run).unwrap()).unwrap();
        assert!(f.admit().is_err());
    }

    #[test]
    fn admission_rejects_another_actual_sink_and_missing_context() {
        let f = Fixture::new();
        let other = Fixture::new();
        // Byte-identical fixtures can share their content binding; explicitly
        // alter the attempt identity to test a genuinely different sink.
        let mut binding = other.context.launch.binding.clone();
        binding.attempt_sha256 = "9".repeat(64);
        let sink = capture::CaptureEmitter::create(
            &other.dir.join("different"),
            binding,
            capture::capture_limits(16).unwrap(),
        )
        .unwrap();
        assert!(admit_original(&f.dir, &f.config, &f.context, &sink).is_err());
        std::fs::remove_file(f.dir.join(execution::PATH)).unwrap();
        assert!(f.admit().unwrap_err().contains("execution context missing"));
    }
}

#[cfg(test)]
mod mathematical_tests {
    use super::*;
    use crate::ml_dataset::{curves, types::CaseConfig};
    use serde_json::{json, Value};

    fn config(n: u32, twist: bool, size: usize) -> CaseConfig {
        CaseConfig {
            family: "koblitz".into(),
            n,
            m: 3,
            twist,
            curve_seed: 7,
            generator_index: 1,
            key_index: 1,
            seed: 42,
            factor_base_size: size,
            degree_bound: 3,
            work_limit: 16384,
            solver: "f4".into(),
        }
    }

    #[test]
    fn original_math_replays_both_actual_twists_at_requested_boundaries() {
        for n in [23, 31, 47, 53, 61] {
            for twist in [false, true] {
                let cfg = config(n, twist, 4);
                let mut case = curves::generate(&cfg).unwrap();
                case.private_witness = Value::Null;
                let original = original_math(&cfg, &case).unwrap();
                assert_eq!(original.curve.m, n);
                assert!(original.codes[0] < original.codes[1]);
                assert_eq!(
                    original.signed.map(|p| p.0),
                    [
                        original.codes[0],
                        original.codes[0],
                        original.codes[1],
                        original.codes[1]
                    ]
                );
                assert_eq!(
                    curves::point_words(&case.public_instance["target"]).unwrap(),
                    Some(original.target)
                );
                assert_eq!(original.curve.irreducible.low_terms[0], 0);
                case.private_witness = json!({"must_not_be_read":"deliberately unusable"});
                assert_eq!(original_math(&cfg, &case).unwrap().signed, original.signed);
            }
        }
    }

    #[test]
    fn two_signed_points_are_not_two_selector_codes() {
        let cfg = config(23, false, 2);
        let case = curves::generate(&cfg).unwrap();
        assert_eq!(case.factor_base["points"].as_array().unwrap().len(), 2);
        assert!(original_math(&cfg, &case)
            .err()
            .unwrap()
            .contains("two distinct x codes"));
    }

    #[test]
    fn original_math_rejects_curve_target_domain_and_configuration_mutations() {
        let cfg = config(23, false, 4);
        let case = curves::generate(&cfg).unwrap();
        original_math(&cfg, &case).unwrap();
        for mutation in 0..6 {
            let mut changed = case.clone();
            match mutation {
                0 => changed.public_instance["curve"]["b"] = json!("2"),
                1 => changed.public_instance["target"] = Value::Null,
                2 => changed.public_instance["twist"] = json!(true),
                3 => changed.factor_base["points"][0] = changed.factor_base["points"][1].clone(),
                4 => changed.public_instance["field"]["irreducible_low_terms"] = json!([0]),
                5 => changed.factor_base["requested_size"] = json!(2),
                _ => unreachable!(),
            }
            assert!(
                original_math(&cfg, &changed).is_err(),
                "accepted mutation {mutation}"
            );
        }
        let mut changed = cfg.clone();
        changed.family = "binary".into();
        assert!(original_math(&changed, &case).is_err());
        changed = cfg;
        changed.m = 4;
        assert!(original_math(&changed, &case).is_err());
    }
}
