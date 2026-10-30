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
    NativeFailure,
    Allocation,
    Serialization,
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
    let candidates = case.factor_base["construction_candidates"]
        .as_array()
        .ok_or("missing original construction candidates")?;
    if candidates.len() > 255
        || candidates
            .iter()
            .any(|row| row["lifts"].as_array().is_none_or(|lifts| lifts.len() > 2))
    {
        return Err("Koblitz two-code setup candidate replay cap exceeded".into());
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

/// Retain completed original equations even if the final adapter boundary fails.
/// Construction never claims a matrix invocation or a complete decomposition.
pub(super) struct Construction {
    pub output: Option<crate::cryptanalysis::koblitz_groebner::bounded_s4::Output>,
    pub native_failure: Option<crate::cryptanalysis::koblitz_groebner::bounded_s4::Failure>,
    pub adapter_stop: Option<Stop>,
}

pub(super) fn construct_original<F>(
    original: &MathInput,
    budget: &mut Budget,
    forward: &mut F,
) -> Construction
where
    F: FnMut(
        crate::cryptanalysis::koblitz_groebner::bounded_s4::Event,
        &mut Budget,
    ) -> Result<(), ()>,
{
    use crate::cryptanalysis::koblitz_groebner::bounded_s4 as native;
    let limits = match budget.begin_native(false, Instant::now()) {
        Ok(limits) => limits,
        Err(stop) => {
            return Construction {
                output: None,
                native_failure: None,
                adapter_stop: Some(stop),
            }
        }
    };
    let deadline = budget.construction_deadline;
    let result = native::build(
        &original.curve.irreducible,
        1,
        &original.codes,
        original.target.0,
        native::Limits {
            work: limits.work,
            bytes: limits.bytes,
        },
        deadline,
        &mut |event| {
            budget
                .observe(
                    Reservation {
                        work: event.reserved.work,
                        bytes: event.reserved.bytes,
                    },
                    Instant::now(),
                )
                .map_err(|_| ())?;
            if forward(event, budget).is_err() {
                let _ = budget.observer_failed();
                return Err(());
            }
            // A callback cannot hide a failed ledger reservation by returning Ok.
            budget.check(Instant::now()).map_err(|_| ())
        },
    );
    match result {
        Ok(output) => {
            let adapter_stop = budget.finish_native(Instant::now()).err();
            Construction {
                output: Some(output),
                native_failure: None,
                adapter_stop,
            }
        }
        Err(failure) => {
            let adapter_stop = budget.stopped;
            // Keep the precise native cause in the report; seal the shared
            // ledger so a caller cannot run a matrix after failed construction.
            let _ = budget.fail::<()>(Stop::NativeFailure);
            Construction {
                output: None,
                native_failure: Some(failure),
                adapter_stop,
            }
        }
    }
}

const INPUT_BYTE_CAP: usize = 64 * 1024;

#[derive(serde::Serialize)]
struct InputView<'a> {
    schema: &'static str,
    binding: &'a super::super::capture::AttemptBinding,
    public_sha256: &'a str,
    factor_base_sha256: &'a str,
    field_degree: u32,
    irreducible_low_terms: &'a [u32],
    curve_a: u64,
    curve_b: u64,
    target: (u64, u64),
    codes: [u64; 2],
    signed_domain: [(u64, u64); 4],
    selector_offsets: [u8; 3],
    n_vars: usize,
    equations: &'a [crate::cryptanalysis::pq_groebner_f2::F2BoolPoly],
    native_work_limit: u64,
    memory_limit: u64,
    trace_limit: u64,
    addition_limit: u64,
}

struct InputWriter<'a> {
    bytes: Vec<u8>,
    capacity: usize,
    budget: &'a mut Budget,
}
impl std::io::Write for InputWriter<'_> {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        if bytes.len() > self.capacity.saturating_sub(self.bytes.len()) {
            let _ = self.budget.fail::<()>(Stop::Trace);
            return Err(std::io::Error::other("Koblitz input serialization cap"));
        }
        self.budget
            .reserve_trace(bytes.len() as u64, Instant::now())
            .map_err(|s| std::io::Error::other(format!("Koblitz input ledger: {s:?}")))?;
        self.bytes.extend_from_slice(bytes);
        Ok(bytes.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

/// Only successful bounded serialization plus persistence can create this token.
/// Its exclusive budget borrow prevents substituting a fresh phase allowance.
pub(super) struct PersistedInput<'a> {
    output: &'a crate::cryptanalysis::koblitz_groebner::bounded_s4::Output,
    budget: &'a mut Budget,
    bytes: Vec<u8>,
}

#[derive(Debug)]
pub(super) struct PersistenceFailure {
    pub reason: String,
    pub bytes: Vec<u8>,
    pub serialized_complete: bool,
}
impl From<String> for PersistenceFailure {
    fn from(reason: String) -> Self {
        Self {
            reason,
            bytes: Vec::new(),
            serialized_complete: false,
        }
    }
}
impl From<&str> for PersistenceFailure {
    fn from(reason: &str) -> Self {
        reason.to_owned().into()
    }
}

pub(super) fn persist_original<'a, F>(
    original: &MathInput,
    output: &'a crate::cryptanalysis::koblitz_groebner::bounded_s4::Output,
    budget: &'a mut Budget,
    binding: &super::super::capture::AttemptBinding,
    public_sha256: &str,
    factor_base_sha256: &str,
    forward: &mut F,
) -> Result<PersistedInput<'a>, PersistenceFailure>
where
    F: FnMut(&[u8], &mut Budget) -> Result<(), String>,
{
    use num_traits::ToPrimitive;
    budget
        .check(Instant::now())
        .map_err(|s| format!("input admission: {s:?}"))?;
    if budget.phase != Phase::Constructed {
        let _ = budget.fail::<()>(Stop::Phase);
        return Err("original construction has not completed".into());
    }
    let valid = binding
        .validate()
        .and_then(|_| crate::store::index::validate_sha256(public_sha256))
        .and_then(|_| crate::store::index::validate_sha256(factor_base_sha256));
    if let Err(error) = valid {
        let _ = budget.fail::<()>(Stop::Serialization);
        return Err(error.into());
    }
    if output.n_vars != 3
        || output.equations.len() != original.curve.m as usize
        || output
            .equations
            .iter()
            .any(|p| p.n_vars != 3 || p.terms.len() > 8 || p.terms.iter().any(|t| t.mask > 7))
    {
        let _ = budget.fail::<()>(Stop::Serialization);
        return Err("original equations exceed two-code shape".into());
    }
    let view = InputView {
        schema: "crypto.ml-dataset.koblitz-m3-input/v1",
        binding,
        public_sha256,
        factor_base_sha256,
        field_degree: original.curve.m,
        irreducible_low_terms: &original.curve.irreducible.low_terms,
        curve_a: original
            .curve
            .a
            .to_biguint()
            .to_u64()
            .ok_or("curve a exceeds u64")?,
        curve_b: 1,
        target: original.target,
        codes: original.codes,
        signed_domain: original.signed,
        selector_offsets: [0, 1, 2],
        n_vars: 3,
        equations: &output.equations,
        native_work_limit: budget.native_work_limit,
        memory_limit: budget.memory_limit,
        trace_limit: budget.trace_limit,
        addition_limit: budget.addition_limit,
    };
    let capacity = (budget.trace_limit - budget.trace_bytes).min(INPUT_BYTE_CAP as u64) as usize;
    budget
        .reserve_bytes(capacity as u64, Instant::now())
        .map_err(|s| format!("input buffer: {s:?}"))?;
    let mut bytes = Vec::new();
    if bytes.try_reserve_exact(capacity).is_err() {
        let _ = budget.fail::<()>(Stop::Allocation);
        return Err("input buffer allocation failed".into());
    }
    let mut writer = InputWriter {
        bytes,
        capacity,
        budget,
    };
    if let Err(error) = serde_json::to_writer(&mut writer, &view) {
        let _ = writer.budget.fail::<()>(Stop::Serialization);
        return Err(PersistenceFailure {
            reason: error.to_string(),
            bytes: writer.bytes,
            serialized_complete: false,
        });
    }
    let InputWriter { bytes, budget, .. } = writer;
    if let Err(error) = forward(&bytes, budget) {
        let _ = budget.observer_failed();
        return Err(PersistenceFailure {
            reason: error,
            bytes,
            serialized_complete: true,
        });
    }
    if let Err(stop) = budget.check(Instant::now()) {
        return Err(PersistenceFailure {
            reason: format!("input persistence: {stop:?}"),
            bytes,
            serialized_complete: true,
        });
    }
    Ok(PersistedInput {
        output,
        budget,
        bytes,
    })
}

pub(super) struct MatrixStep {
    pub original_bytes: Vec<u8>,
    pub report: Option<crate::cryptanalysis::koblitz_groebner::bounded_matrix::Report>,
    pub adapter_stop: Option<Stop>,
}

pub(super) fn matrix_step<F>(
    input: PersistedInput<'_>,
    engine: crate::cryptanalysis::koblitz_groebner::bounded_matrix::Engine,
    degree: u32,
    forward: &mut F,
) -> MatrixStep
where
    F: FnMut(
        &crate::cryptanalysis::koblitz_groebner::bounded_matrix::Event,
        &mut Budget,
    ) -> Result<(), ()>,
{
    use crate::cryptanalysis::koblitz_groebner::bounded_matrix as native;
    let PersistedInput {
        output,
        budget,
        bytes,
    } = input;
    let limits = match budget.begin_native(true, Instant::now()) {
        Ok(limits) => limits,
        Err(stop) => {
            return MatrixStep {
                original_bytes: bytes,
                report: None,
                adapter_stop: Some(stop),
            }
        }
    };
    let deadline = budget.deadline;
    let native_limits = native::Limits {
        work: limits.work,
        bytes: limits.bytes,
        main_rows: native::MAX_ROWS as u64,
        events: native::MAX_EVENTS,
    };
    let observer = |event: &native::Event| {
        budget
            .observe(
                Reservation {
                    work: event.reserved.work,
                    bytes: event.reserved.bytes,
                },
                Instant::now(),
            )
            .map_err(|_| ())?;
        if forward(event, budget).is_err() {
            let _ = budget.observer_failed();
            return Err(());
        }
        budget.check(Instant::now()).map_err(|_| ())
    };
    let report = match engine {
        native::Engine::F4Macaulay => native::matrix_f4_f2_bounded(
            &output.equations,
            output.n_vars,
            degree,
            native_limits,
            deadline,
            observer,
        ),
        native::Engine::F5KoszulFrobenius => {
            crate::cryptanalysis::matrix_f5_f2::matrix_f5_f2_bounded(
                &output.equations,
                output.n_vars,
                degree,
                native_limits,
                deadline,
                observer,
            )
        }
    };
    let adapter_stop = if report.stop == native::Stop::Complete && report.native_step_complete {
        budget.finish_native(Instant::now()).err()
    } else {
        let stop = budget.stopped;
        let _ = budget.fail::<()>(Stop::NativeFailure);
        stop
    };
    MatrixStep {
        original_bytes: bytes,
        report: Some(report),
        adapter_stop,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct OriginalLimits {
    native_work: u64,
    memory: u64,
    trace: u64,
    additions: u64,
}
impl OriginalLimits {
    /// Historical caller first validates the saved launch against the saved run
    /// and attempt. No current clock is read when binding these resource values.
    pub fn from_original(
        config: &super::super::types::CaseConfig,
        launch: &super::super::execution::LaunchContext,
    ) -> Result<Self, String> {
        if config.work_limit == 0 || launch.memory_bytes == 0 || launch.trace_bytes == 0 {
            return Err("zero original Koblitz resource limit".into());
        }
        Ok(Self {
            native_work: config
                .work_limit
                .checked_mul(1024)
                .ok_or("original work overflow")?
                .min(NATIVE_WORK_MAX),
            memory: launch.memory_bytes,
            trace: launch.trace_bytes,
            additions: config.work_limit,
        })
    }
}
impl Budget {
    pub fn original_limits(&self) -> OriginalLimits {
        OriginalLimits {
            native_work: self.native_work_limit,
            memory: self.memory_limit,
            trace: self.trace_limit,
            additions: self.addition_limit,
        }
    }
}

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct SavedMonomial {
    mask: u8,
}
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct SavedEquation {
    n_vars: usize,
    terms: Vec<SavedMonomial>,
}
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct SavedInput {
    schema: String,
    binding: super::super::capture::AttemptBinding,
    public_sha256: String,
    factor_base_sha256: String,
    field_degree: u32,
    irreducible_low_terms: Vec<u32>,
    curve_a: u64,
    curve_b: u64,
    target: (u64, u64),
    codes: [u64; 2],
    signed_domain: [(u64, u64); 4],
    selector_offsets: [u8; 3],
    n_vars: usize,
    equations: Vec<SavedEquation>,
    native_work_limit: u64,
    memory_limit: u64,
    trace_limit: u64,
    addition_limit: u64,
}

/// Verify finite-domain ANF independently of the producer's symbolic builder.
/// Expected inputs/digests/limits must come from validated original files.
/// This verifies equations, not native row derivations or a complete solver.
pub(super) fn verify_original_input(
    bytes: &[u8],
    original: &MathInput,
    binding: &super::super::capture::AttemptBinding,
    public_sha256: &str,
    factor_base_sha256: &str,
    limits: OriginalLimits,
) -> Result<(), String> {
    use crate::cryptanalysis::pq_groebner_f2::{cmp_mono, mask_from64, F2BoolMono};
    use crate::cryptanalysis::semaev_decomp::{eval_f3, Gf2};
    use num_traits::ToPrimitive;
    if bytes.len() > INPUT_BYTE_CAP {
        return Err("saved Koblitz input exceeds byte cap".into());
    }
    let saved: SavedInput =
        serde_json::from_slice(bytes).map_err(|e| format!("strict Koblitz input: {e}"))?;
    saved.binding.validate()?;
    if saved.schema != "crypto.ml-dataset.koblitz-m3-input/v1"
        || saved.binding != *binding
        || saved.public_sha256 != public_sha256
        || saved.factor_base_sha256 != factor_base_sha256
        || saved.field_degree != original.curve.m
        || saved.irreducible_low_terms != original.curve.irreducible.low_terms
        || Some(saved.curve_a) != original.curve.a.to_biguint().to_u64()
        || saved.curve_b != 1
        || saved.target != original.target
        || saved.codes != original.codes
        || saved.signed_domain != original.signed
        || saved.selector_offsets != [0, 1, 2]
        || saved.n_vars != 3
        || saved.native_work_limit != limits.native_work
        || saved.memory_limit != limits.memory
        || saved.trace_limit != limits.trace
        || saved.addition_limit != limits.additions
        || saved.equations.len() != original.curve.m as usize
    {
        return Err("saved Koblitz input differs from originals".into());
    }
    let mut coefficients = [0u64; 8];
    let gf = Gf2::new(&original.curve.irreducible);
    for (assignment, value) in coefficients.iter_mut().enumerate() {
        *value = eval_f3(
            original.codes[assignment & 1],
            original.codes[(assignment >> 1) & 1],
            original.codes[(assignment >> 2) & 1],
            original.target.0,
            &gf,
        );
    }
    // Boolean Mobius transform: unique square-free polynomial coefficients.
    for bit in 0..3 {
        for mask in 0..8 {
            if mask & (1 << bit) != 0 {
                coefficients[mask] ^= coefficients[mask ^ (1 << bit)];
            }
        }
    }
    for (coordinate, equation) in saved.equations.iter().enumerate() {
        if equation.n_vars != 3
            || equation.terms.len() > 8
            || equation.terms.iter().any(|t| t.mask > 7)
            || !equation.terms.windows(2).all(|pair| {
                cmp_mono(
                    F2BoolMono::from_mask(mask_from64(pair[0].mask as u64)),
                    F2BoolMono::from_mask(mask_from64(pair[1].mask as u64)),
                )
                .is_gt()
            })
        {
            return Err("noncanonical saved Koblitz equation".into());
        }
        let actual = equation
            .terms
            .iter()
            .fold(0u8, |set, t| set | (1u8 << t.mask));
        let expected = coefficients
            .iter()
            .enumerate()
            .fold(0u8, |set, (mask, value)| {
                set | ((((value >> coordinate) & 1) as u8) << mask)
            });
        if actual != expected {
            return Err("saved Koblitz equation differs from independent scalar ANF".into());
        }
    }
    Ok(())
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
        assert_eq!(
            b.original_limits(),
            OriginalLimits::from_original(&f.config, &f.context.launch).unwrap()
        );
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
        for mutation in 0..8 {
            let mut changed = case.clone();
            match mutation {
                0 => changed.public_instance["curve"]["b"] = json!("2"),
                1 => changed.public_instance["target"] = Value::Null,
                2 => changed.public_instance["twist"] = json!(true),
                3 => changed.factor_base["points"][0] = changed.factor_base["points"][1].clone(),
                4 => changed.public_instance["field"]["irreducible_low_terms"] = json!([0]),
                5 => changed.factor_base["requested_size"] = json!(2),
                6 => {
                    changed.factor_base["construction_candidates"] =
                        json!(vec![json!({"lifts":[]}); 256])
                }
                7 => {
                    changed.factor_base["construction_candidates"][0]["lifts"] =
                        json!([null, null, null])
                }
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

#[cfg(test)]
mod construction_tests {
    use super::*;
    use crate::cryptanalysis::koblitz_groebner::bounded_s4 as native;
    use crate::ml_dataset::{curves, types::CaseConfig};

    pub(super) fn fixture() -> MathInput {
        let config = CaseConfig {
            family: "koblitz".into(),
            n: 23,
            m: 3,
            twist: false,
            curve_seed: 7,
            generator_index: 1,
            key_index: 1,
            seed: 42,
            factor_base_size: 4,
            degree_bound: 3,
            work_limit: 16384,
            solver: "weil".into(),
        };
        original_math(&config, &curves::generate(&config).unwrap()).unwrap()
    }
    pub(super) fn budget() -> Budget {
        let now = Instant::now();
        Budget::new(
            16384,
            512 << 20,
            16 << 20,
            now + Duration::from_secs(20),
            now,
        )
        .unwrap()
    }

    #[test]
    fn native_construction_shares_reservations_and_leaves_matrix_headroom() {
        let input = fixture();
        let mut budget = budget();
        let mut events = 0;
        let result = construct_original(&input, &mut budget, &mut |_, _| {
            events += 1;
            Ok(())
        });
        assert!(
            result.native_failure.is_none(),
            "{:?}",
            result.native_failure
        );
        assert_eq!(result.adapter_stop, None);
        let output = result.output.unwrap();
        assert_eq!(output.n_vars, 3);
        assert_eq!(output.equations.len(), 23);
        assert!(events > 1);
        assert!(output.reserved.work > 0);
        assert_eq!(
            budget.native,
            Reservation {
                work: output.reserved.work,
                bytes: output.reserved.bytes
            }
        );
        let remaining = budget.begin_native(true, Instant::now()).unwrap();
        assert_eq!(remaining.work, NATIVE_WORK_MAX - output.reserved.work);
    }

    #[test]
    fn native_stop_and_callback_failure_prevent_later_matrix_work() {
        let input = fixture();
        let mut baseline = budget();
        let complete = construct_original(&input, &mut baseline, &mut |_, _| Ok(()));
        let used = complete.output.unwrap().reserved.work;
        let mut capped = budget();
        capped.native_work_limit = used - 1;
        let result = construct_original(&input, &mut capped, &mut |_, _| Ok(()));
        assert!(result.output.is_none());
        assert_eq!(result.native_failure.unwrap().stop, native::Stop::WorkLimit);
        assert_eq!(result.adapter_stop, None);
        assert_eq!(
            capped.begin_native(true, Instant::now()),
            Err(Stop::NativeFailure)
        );
        let mut refused = budget();
        let result = construct_original(&input, &mut refused, &mut |_, _| Err(()));
        assert_eq!(result.native_failure.unwrap().stop, native::Stop::Observer);
        assert_eq!(result.adapter_stop, Some(Stop::Observer));
        assert_eq!(
            refused.begin_native(true, Instant::now()),
            Err(Stop::Observer)
        );
    }

    #[test]
    fn callback_cannot_swallow_ledger_failure_and_expired_setup_does_no_native_work() {
        let input = fixture();
        let mut limited = budget();
        let result = construct_original(&input, &mut limited, &mut |_, b| {
            let _ = b.reserve_trace(u64::MAX, Instant::now());
            Ok(())
        });
        assert_eq!(result.native_failure.unwrap().stop, native::Stop::Observer);
        assert_eq!(result.adapter_stop, Some(Stop::Trace));
        let mut expired = budget();
        expired.construction_deadline = Instant::now();
        let mut called = false;
        let result = construct_original(&input, &mut expired, &mut |_, _| {
            called = true;
            Ok(())
        });
        assert!(!called);
        assert!(result.output.is_none());
        assert!(result.native_failure.is_none());
        assert_eq!(result.adapter_stop, Some(Stop::Deadline));
        assert_eq!(expired.native, Reservation::default());
    }
}

#[cfg(test)]
mod matrix_tests {
    use super::*;
    use crate::cryptanalysis::koblitz_groebner::bounded_matrix as native;
    use crate::ml_dataset::capture::AttemptBinding;
    pub(super) fn binding() -> AttemptBinding {
        AttemptBinding {
            schema: "crypto.ml-dataset.attempt-binding/v1".into(),
            run_sha256: "1".repeat(64),
            plan_sha256: "2".repeat(64),
            protocol_sha256: "3".repeat(64),
            config_sha256: "4".repeat(64),
            source_commit: "5".repeat(40),
            binary_sha256: "6".repeat(64),
            cell_index: 0,
            attempt_sha256: "7".repeat(64),
        }
    }
    #[test]
    fn both_named_matrix_engines_require_persisted_original_and_keep_native_reports() {
        for engine in [
            native::Engine::F4Macaulay,
            native::Engine::F5KoszulFrobenius,
        ] {
            let original = construction_tests::fixture();
            let mut budget = construction_tests::budget();
            let construction = construct_original(&original, &mut budget, &mut |_, _| Ok(()));
            let output = construction.output.unwrap();
            let constructed_work = budget.native.work;
            let mut persisted = false;
            let input = persist_original(
                &original,
                &output,
                &mut budget,
                &binding(),
                &"8".repeat(64),
                &"9".repeat(64),
                &mut |bytes, _| {
                    let value: serde_json::Value = serde_json::from_slice(bytes).unwrap();
                    assert_eq!(value["schema"], "crypto.ml-dataset.koblitz-m3-input/v1");
                    assert_eq!(
                        value["equations"],
                        serde_json::to_value(&output.equations).unwrap()
                    );
                    persisted = true;
                    Ok(())
                },
            )
            .unwrap();
            let mut events = 0;
            let result = matrix_step(input, engine, 3, &mut |_, _| {
                assert!(persisted);
                events += 1;
                Ok(())
            });
            assert_eq!(result.adapter_stop, None);
            let report = result.report.unwrap();
            assert_eq!(report.engine, engine);
            assert_eq!(report.stop, native::Stop::Complete);
            assert!(report.backend_invoked && report.native_step_complete);
            assert!(report.counters.nonempty_rows > 0);
            assert!(report.counters.reduction_word_xors > 0);
            assert!(events > 0);
            assert!(!result.original_bytes.is_empty());
            assert_eq!(budget.native.work, constructed_work + report.reserved.work);
        }
    }
    #[test]
    fn failed_input_flush_retains_complete_bytes_and_prevents_matrix_admission() {
        let original = construction_tests::fixture();
        let mut budget = construction_tests::budget();
        let output = construct_original(&original, &mut budget, &mut |_, _| Ok(()))
            .output
            .unwrap();
        let error = persist_original(
            &original,
            &output,
            &mut budget,
            &binding(),
            &"8".repeat(64),
            &"9".repeat(64),
            &mut |_, _| Err("selected I/O failure".into()),
        )
        .err()
        .unwrap();
        assert_eq!(error.reason, "selected I/O failure");
        assert!(error.serialized_complete);
        let value: serde_json::Value = serde_json::from_slice(&error.bytes).unwrap();
        assert_eq!(
            value["equations"],
            serde_json::to_value(&output.equations).unwrap()
        );
        assert_eq!(
            budget.begin_native(true, Instant::now()),
            Err(Stop::Observer)
        );
    }
    #[test]
    fn exact_input_trace_cap_passes_and_one_less_never_reaches_sink() {
        let original = construction_tests::fixture();
        let mut baseline = construction_tests::budget();
        let output = construct_original(&original, &mut baseline, &mut |_, _| Ok(()))
            .output
            .unwrap();
        // Fix the serialized trace limit to a stable value; consume all but the
        // exact payload allowance before serialization in the two controls.
        let token = persist_original(
            &original,
            &output,
            &mut baseline,
            &binding(),
            &"8".repeat(64),
            &"9".repeat(64),
            &mut |_, _| Ok(()),
        )
        .unwrap();
        let length = token.bytes.len() as u64;
        drop(token);
        for short in [false, true] {
            let mut budget = construction_tests::budget();
            let output = construct_original(&original, &mut budget, &mut |_, _| Ok(()))
                .output
                .unwrap();
            budget
                .reserve_trace(
                    budget.trace_limit - length + u64::from(short),
                    Instant::now(),
                )
                .unwrap();
            let mut called = false;
            let result = persist_original(
                &original,
                &output,
                &mut budget,
                &binding(),
                &"8".repeat(64),
                &"9".repeat(64),
                &mut |_, _| {
                    called = true;
                    Ok(())
                },
            );
            if short {
                let error = result.err().unwrap();
                assert!(!error.serialized_complete);
                assert!(!called);
                assert_eq!(budget.stopped, Some(Stop::Trace));
            } else {
                assert!(result.is_ok());
                assert!(called);
            }
        }
    }
    #[test]
    fn matrix_callback_failure_retains_input_and_seals_budget() {
        let original = construction_tests::fixture();
        let mut budget = construction_tests::budget();
        let output = construct_original(&original, &mut budget, &mut |_, _| Ok(()))
            .output
            .unwrap();
        let token = persist_original(
            &original,
            &output,
            &mut budget,
            &binding(),
            &"8".repeat(64),
            &"9".repeat(64),
            &mut |_, _| Ok(()),
        )
        .unwrap();
        let result = matrix_step(token, native::Engine::F4Macaulay, 3, &mut |_, _| Err(()));
        assert_eq!(result.report.unwrap().stop, native::Stop::Observer);
        assert_eq!(result.adapter_stop, Some(Stop::Observer));
        assert!(!result.original_bytes.is_empty());
        assert_eq!(
            budget.begin_extraction(true, Instant::now()),
            Err(Stop::Observer)
        );
    }
}

#[cfg(test)]
mod input_verification_tests {
    use super::*;
    use serde_json::{json, Value};
    fn fixture() -> (MathInput, OriginalLimits, Vec<u8>) {
        let original = construction_tests::fixture();
        let mut budget = construction_tests::budget();
        let output = construct_original(&original, &mut budget, &mut |_, _| Ok(()))
            .output
            .unwrap();
        let limits = budget.original_limits();
        let input = persist_original(
            &original,
            &output,
            &mut budget,
            &matrix_tests::binding(),
            &"8".repeat(64),
            &"9".repeat(64),
            &mut |_, _| Ok(()),
        )
        .unwrap();
        let PersistedInput { bytes, .. } = input;
        (original, limits, bytes)
    }
    fn verify(original: &MathInput, limits: OriginalLimits, bytes: &[u8]) -> Result<(), String> {
        verify_original_input(
            bytes,
            original,
            &matrix_tests::binding(),
            &"8".repeat(64),
            &"9".repeat(64),
            limits,
        )
    }
    #[test]
    fn saved_input_matches_independent_scalar_coefficients_without_runtime_clock() {
        let (original, limits, bytes) = fixture();
        verify(&original, limits, &bytes).unwrap();
        // The historical verifier accepts only original values, no live Budget
        // or Instant. A different original allowance still fails binding.
        let mut changed = limits;
        changed.memory += 1;
        assert!(verify(&original, changed, &bytes).is_err());
    }
    #[test]
    fn reserialized_semantic_mutations_and_unknown_nested_fields_fail() {
        let (original, limits, bytes) = fixture();
        let baseline: Value = serde_json::from_slice(&bytes).unwrap();
        for mutation in 0..12 {
            let mut value = baseline.clone();
            match mutation {
                0 => value["codes"][0] = json!(0),
                1 => value["target"][1] = json!(value["target"][1].as_u64().unwrap() ^ 1),
                2 => value["curve_b"] = json!(2),
                3 => value["public_sha256"] = json!("a".repeat(64)),
                4 => value["selector_offsets"] = json!([2, 1, 0]),
                5 => value["native_work_limit"] = json!(1),
                6 => value["binding"]["attempt_sha256"] = json!("b".repeat(64)),
                7 => value["unexpected"] = json!(true),
                8 => value["equations"][0]["unexpected"] = json!(true),
                9 => {
                    let p = value["equations"]
                        .as_array_mut()
                        .unwrap()
                        .iter_mut()
                        .find(|p| !p["terms"].as_array().unwrap().is_empty())
                        .unwrap();
                    p["terms"][0]["unexpected"] = json!(true);
                }
                10 => {
                    let p = value["equations"]
                        .as_array_mut()
                        .unwrap()
                        .iter_mut()
                        .find(|p| !p["terms"].as_array().unwrap().is_empty())
                        .unwrap();
                    p["terms"].as_array_mut().unwrap().remove(0);
                }
                11 => {
                    let p = value["equations"]
                        .as_array_mut()
                        .unwrap()
                        .iter_mut()
                        .find(|p| !p["terms"].as_array().unwrap().is_empty())
                        .unwrap();
                    let term = p["terms"][0].clone();
                    p["terms"].as_array_mut().unwrap().push(term);
                }
                _ => unreachable!(),
            }
            assert!(
                verify(&original, limits, &serde_json::to_vec(&value).unwrap()).is_err(),
                "accepted mutation {mutation}"
            );
        }
        assert!(verify(&original, limits, &vec![b' '; INPUT_BYTE_CAP + 1]).is_err());
    }
    #[test]
    fn independent_input_check_covers_all_requested_boundary_twists() {
        use crate::ml_dataset::{curves, types::CaseConfig};
        for n in [23, 31, 47, 53, 61] {
            for twist in [false, true] {
                let config = CaseConfig {
                    family: "koblitz".into(),
                    n,
                    m: 3,
                    twist,
                    curve_seed: 7,
                    generator_index: 1,
                    key_index: 1,
                    seed: 42,
                    factor_base_size: 4,
                    degree_bound: 3,
                    work_limit: 16384,
                    solver: "weil".into(),
                };
                let original = original_math(&config, &curves::generate(&config).unwrap()).unwrap();
                let mut budget = construction_tests::budget();
                let limits = budget.original_limits();
                let output = construct_original(&original, &mut budget, &mut |_, _| Ok(()))
                    .output
                    .unwrap();
                let input = persist_original(
                    &original,
                    &output,
                    &mut budget,
                    &matrix_tests::binding(),
                    &"8".repeat(64),
                    &"9".repeat(64),
                    &mut |_, _| Ok(()),
                )
                .unwrap();
                verify(&original, limits, &input.bytes).unwrap();
            }
        }
    }
}
/// A single actual addition in the lexicographic four-point signed domain.
/// `prefix == None` denotes infinity, including an inverse-pair prefix.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct SignedPrefix {
    sequence: u16,
    tuple: u8,
    indices: [u8; 3],
    summand: u8,
    prefix: Option<(u64, u64)>,
    matches_target: bool,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct PrefixAssessment {
    performed_additions: usize,
    acknowledged_additions: usize,
    completed_tuples: usize,
    candidate_tuples: Vec<u8>,
    all_tuples_acknowledged: bool,
}

/// Replay a bounded historical prefix using only original-domain arithmetic.
/// The final optional event was performed but not acknowledged by the sink.
/// This function establishes neither durable witness custody nor an uncensored
/// solver outcome. Outer binding/role validation and budget accounting remain
/// mandatory before using its assessment in a dataset label.
pub(super) fn verify_signed_prefix(
    original: &MathInput,
    acknowledged: &[SignedPrefix],
    unacknowledged: Option<&SignedPrefix>,
) -> Result<PrefixAssessment, String> {
    use crate::binary_ecc::{curve::point_add, BinaryPoint, F2mElement};
    use num_traits::ToPrimitive;
    let performed = acknowledged
        .len()
        .checked_add(usize::from(unacknowledged.is_some()))
        .ok_or("signed prefix length overflow")?;
    if performed > 192 {
        return Err("signed prefix exceeds 64 ordered triples".into());
    }
    let domain = original.signed.map(|(x, y)| BinaryPoint::Affine {
        x: F2mElement::from_words(&[x], original.curve.m),
        y: F2mElement::from_words(&[y], original.curve.m),
    });
    let mut prefix = BinaryPoint::Infinity;
    let mut candidates = Vec::new();
    candidates
        .try_reserve_exact(64)
        .map_err(|_| "signed prefix allocation")?;
    for (sequence, saved) in acknowledged.iter().chain(unacknowledged).enumerate() {
        let tuple = sequence / 3;
        let summand = sequence % 3;
        let indices = [tuple / 16, (tuple / 4) % 4, tuple % 4];
        if summand == 0 {
            prefix = BinaryPoint::Infinity;
        }
        prefix = point_add(&original.curve, &prefix, &domain[indices[summand]]);
        let expected = match &prefix {
            BinaryPoint::Infinity => None,
            BinaryPoint::Affine { x, y } => Some((
                x.to_biguint().to_u64().ok_or("reference x exceeds word")?,
                y.to_biguint().to_u64().ok_or("reference y exceeds word")?,
            )),
        };
        let matched = summand == 2 && expected == Some(original.target);
        if saved.sequence as usize != sequence
            || saved.tuple as usize != tuple
            || saved.indices != indices.map(|i| i as u8)
            || saved.summand as usize != summand
            || saved.prefix != expected
            || saved.matches_target != matched
        {
            return Err(format!("signed prefix mismatch at addition {sequence}"));
        }
        if matched {
            candidates.push(tuple as u8);
        }
    }
    Ok(PrefixAssessment {
        performed_additions: performed,
        acknowledged_additions: acknowledged.len(),
        completed_tuples: performed / 3,
        candidate_tuples: candidates,
        all_tuples_acknowledged: acknowledged.len() == 192 && unacknowledged.is_none(),
    })
}

#[cfg(test)]
mod signed_prefix_tests {
    use super::*;
    use crate::cryptanalysis::koblitz_fast_arith::FastBinaryCurve;
    use num_traits::ToPrimitive;

    fn fast_events(original: &MathInput) -> Vec<SignedPrefix> {
        let curve = FastBinaryCurve::new(
            &original.curve.irreducible,
            original.curve.a.to_biguint().to_u64().unwrap(),
        )
        .unwrap();
        let mut events = Vec::new();
        for tuple in 0..64usize {
            let indices = [tuple / 16, (tuple / 4) % 4, tuple % 4];
            let mut prefix = None;
            for summand in 0..3 {
                prefix = curve.add(prefix, Some(original.signed[indices[summand]]));
                events.push(SignedPrefix {
                    sequence: events.len() as u16,
                    tuple: tuple as u8,
                    indices: indices.map(|i| i as u8),
                    summand: summand as u8,
                    prefix,
                    matches_target: summand == 2 && prefix == Some(original.target),
                });
            }
        }
        events
    }

    #[test]
    fn independent_signed_replay_keeps_every_partial_boundary_and_infinity() {
        let mut original = construction_tests::fixture();
        // A fixed positive control: P + (-P) + Q = Q, without resampling.
        original.target = original.signed[0];
        let events = fast_events(&original);
        assert!(events.iter().any(|e| e.prefix.is_none()));
        let full = verify_signed_prefix(&original, &events, None).unwrap();
        assert!(full.all_tuples_acknowledged);
        assert!(!full.candidate_tuples.is_empty());
        assert_eq!(full.completed_tuples, 64);
        for n in 0..192 {
            let partial = verify_signed_prefix(&original, &events[..n], None).unwrap();
            assert_eq!(partial.performed_additions, n);
            assert!(!partial.all_tuples_acknowledged);
            let refused = verify_signed_prefix(&original, &events[..n], Some(&events[n])).unwrap();
            assert_eq!(refused.performed_additions, n + 1);
            assert_eq!(refused.acknowledged_additions, n);
            assert!(!refused.all_tuples_acknowledged);
        }
    }

    #[test]
    fn independent_signed_replay_rejects_semantic_and_order_mutations() {
        let original = construction_tests::fixture();
        let events = fast_events(&original);
        for mutation in 0..8 {
            let mut changed = events.clone();
            match mutation {
                0 => changed[1].sequence += 1,
                1 => changed[1].tuple += 1,
                2 => changed[1].indices[0] ^= 1,
                3 => changed[1].summand = 0,
                4 => changed[0].prefix = None,
                5 => changed[2].matches_target = !changed[2].matches_target,
                6 => {
                    changed.remove(1);
                }
                7 => changed.push(events[0].clone()),
                _ => unreachable!(),
            }
            assert!(
                verify_signed_prefix(&original, &changed, None).is_err(),
                "mutation {mutation}"
            );
        }
        let mut value = serde_json::to_value(&events[0]).unwrap();
        value["unchecked"] = serde_json::json!(true);
        assert!(serde_json::from_value::<SignedPrefix>(value).is_err());
    }

    #[test]
    fn independent_signed_replay_covers_both_twists_at_field_boundaries() {
        use crate::ml_dataset::{curves, types::CaseConfig};
        for n in [23, 31, 47, 53, 61] {
            for twist in [false, true] {
                let cfg = CaseConfig {
                    family: "koblitz".into(),
                    n,
                    m: 3,
                    twist,
                    curve_seed: 7,
                    generator_index: 1,
                    key_index: 1,
                    seed: 42,
                    factor_base_size: 4,
                    degree_bound: 3,
                    work_limit: 16384,
                    solver: "weil".into(),
                };
                let original = original_math(&cfg, &curves::generate(&cfg).unwrap()).unwrap();
                let events = fast_events(&original);
                let result = verify_signed_prefix(&original, &events, None).unwrap();
                assert!(result.all_tuples_acknowledged);
                assert_eq!(result.performed_additions, 192);
            }
        }
    }
}

const SIGNED_REPLAY_BYTE_CAP: usize = 128 * 1024;

/// A separately bound arithmetic-prefix artifact, not a native execution log.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct SavedSignedReplay {
    schema: String,
    binding: super::super::capture::AttemptBinding,
    public_sha256: String,
    factor_base_sha256: String,
    input_sha256: String,
    addition_limit: u64,
    performed_additions: usize,
    acknowledged_additions: usize,
    acknowledged: Vec<SignedPrefix>,
    unacknowledged: Option<SignedPrefix>,
}

/// Expected digests and limits come from authenticated original artifacts and
/// the admitted launch, never from the saved transcript. A historical check has
/// no live attempt deadline. Decode size and arithmetic replay are bounded.
pub(super) fn verify_signed_replay(
    bytes: &[u8],
    original: &MathInput,
    binding: &super::super::capture::AttemptBinding,
    public_sha256: &str,
    factor_base_sha256: &str,
    input_sha256: &str,
    addition_limit: u64,
) -> Result<PrefixAssessment, String> {
    if bytes.len() > SIGNED_REPLAY_BYTE_CAP {
        return Err("signed replay exceeds byte cap".into());
    }
    let saved: SavedSignedReplay =
        serde_json::from_slice(bytes).map_err(|e| format!("strict signed replay: {e}"))?;
    saved.binding.validate()?;
    let hash_valid = |s: &str| s.len() == 64 && s.bytes().all(|b| b.is_ascii_hexdigit());
    if saved.schema != "crypto.ml-dataset.koblitz-m3-signed-prefix/v1"
        || saved.binding != *binding
        || !hash_valid(public_sha256)
        || !hash_valid(factor_base_sha256)
        || !hash_valid(input_sha256)
        || saved.public_sha256 != public_sha256
        || saved.factor_base_sha256 != factor_base_sha256
        || saved.input_sha256 != input_sha256
        || saved.addition_limit != addition_limit
        || saved.performed_additions as u64 > addition_limit
        || saved.performed_additions > 192
        || saved.acknowledged_additions != saved.acknowledged.len()
        || saved.performed_additions
            != saved
                .acknowledged
                .len()
                .checked_add(usize::from(saved.unacknowledged.is_some()))
                .ok_or("signed replay counter overflow")?
    {
        return Err("signed replay binding, limit or counter mismatch".into());
    }
    verify_signed_prefix(original, &saved.acknowledged, saved.unacknowledged.as_ref())
}

#[cfg(test)]
mod signed_binding_tests {
    use super::*;
    #[test]
    fn signed_replay_rejects_binding_counter_limit_and_schema_mutations() {
        let original = construction_tests::fixture();
        let binding = matrix_tests::binding();
        let original_value = serde_json::json!({
            "schema": "crypto.ml-dataset.koblitz-m3-signed-prefix/v1",
            "binding": binding,
            "public_sha256": "8".repeat(64),
            "factor_base_sha256": "9".repeat(64),
            "input_sha256": "a".repeat(64),
            "addition_limit": 192,
            "performed_additions": 0,
            "acknowledged_additions": 0,
            "acknowledged": [],
            "unacknowledged": null
        });
        let check = |bytes: &[u8]| {
            verify_signed_replay(
                bytes,
                &original,
                &binding,
                &"8".repeat(64),
                &"9".repeat(64),
                &"a".repeat(64),
                192,
            )
        };
        let empty = check(&serde_json::to_vec(&original_value).unwrap()).unwrap();
        assert!(!empty.all_tuples_acknowledged);
        for mutation in 0..9 {
            let mut v = original_value.clone();
            match mutation {
                0 => v["schema"] = serde_json::json!("wrong"),
                1 => v["binding"]["cell_index"] = serde_json::json!(1),
                2 => v["public_sha256"] = serde_json::json!("b".repeat(64)),
                3 => v["factor_base_sha256"] = serde_json::json!("b".repeat(64)),
                4 => v["input_sha256"] = serde_json::json!("b".repeat(64)),
                5 => v["addition_limit"] = serde_json::json!(193),
                6 => v["performed_additions"] = serde_json::json!(1),
                7 => v["acknowledged_additions"] = serde_json::json!(1),
                8 => v["negative"] = serde_json::json!(true),
                _ => unreachable!(),
            }
            assert!(
                check(&serde_json::to_vec(&v).unwrap()).is_err(),
                "mutation {mutation}"
            );
        }
        assert!(check(&vec![b' '; SIGNED_REPLAY_BYTE_CAP + 1]).is_err());
    }
}
/// Checked mathematical candidate. Durable custody is a separate publication.
pub(super) struct CheckedSignedWitness {
    pub indices: [u8; 3],
    pub points: [(u64, u64); 3],
    pub target: (u64, u64),
}

pub(super) struct SignedEnumeration {
    pub acknowledged: Vec<SignedPrefix>,
    pub unacknowledged: Option<SignedPrefix>,
    pub enumeration_additions: u64,
    pub witness_check_additions: u64,
    pub first_receipt: Option<crate::ml_dataset::capture::WitnessReceipt>,
    pub stop: Option<Stop>,
    pub complete: bool,
}

/// Stage-only producer. The caller must admit field setup and generic-check
/// scratch before entering, and supply trusted capture callbacks which charge
/// serialization, retention and transport work to this same ledger. Runtime
/// activation additionally requires a bound-capture bridge, not arbitrary
/// caller receipts. Counts below are actual addition API calls, including O+P.
pub(super) fn enumerate_signed(
    original: &MathInput,
    curve: &crate::cryptanalysis::koblitz_fast_arith::FastBinaryCurve,
    budget: &mut Budget,
    matrix_required: bool,
    event_sink: &mut impl FnMut(&SignedPrefix, &mut Budget) -> Result<(), Stop>,
    publish: &mut impl FnMut(
        &CheckedSignedWitness,
        &mut Budget,
    ) -> Result<crate::ml_dataset::capture::WitnessReceipt, Stop>,
) -> SignedEnumeration {
    use crate::binary_ecc::{curve::point_add, BinaryPoint, F2mElement};
    use num_traits::ToPrimitive;
    let mut result = SignedEnumeration {
        acknowledged: Vec::new(),
        unacknowledged: None,
        enumeration_additions: 0,
        witness_check_additions: 0,
        first_receipt: None,
        stop: None,
        complete: false,
    };
    let outcome = (|| -> Result<(), Stop> {
        budget.begin_extraction(matrix_required, Instant::now())?;
        // Check prepared-kernel metadata against the original curve. This is
        // not authentication of its private reduction table: runtime entry must
        // use an opaque original-bound setup token rather than a caller kernel.
        let modulus = original
            .curve
            .irreducible
            .low_terms
            .iter()
            .fold(1u64 << original.curve.m, |bits, &term| {
                bits | (1u64 << term)
            });
        if curve.n != original.curve.m
            || curve.gf.n != original.curve.m
            || curve.gf.irr != modulus
            || Some(curve.a) != original.curve.a.to_biguint().to_u64()
        {
            return budget.fail(Stop::InvalidLimits);
        }
        budget.reserve_bytes(
            (192 * std::mem::size_of::<SignedPrefix>()) as u64,
            Instant::now(),
        )?;
        result
            .acknowledged
            .try_reserve_exact(192)
            .map_err(|_| Stop::Allocation)?;
        for tuple in 0..64usize {
            let indices = [tuple / 16, (tuple / 4) % 4, tuple % 4];
            let mut prefix = None;
            for summand in 0..3 {
                budget.reserve_addition(Instant::now())?;
                prefix = curve.add(prefix, Some(original.signed[indices[summand]]));
                result.enumeration_additions += 1;
                let event = SignedPrefix {
                    sequence: (result.enumeration_additions - 1) as u16,
                    tuple: tuple as u8,
                    indices: indices.map(|i| i as u8),
                    summand: summand as u8,
                    prefix,
                    matches_target: summand == 2 && prefix == Some(original.target),
                };
                result.unacknowledged = Some(event.clone());
                budget.check(Instant::now())?;
                if event.matches_target && result.first_receipt.is_none() {
                    let points = indices.map(|i| original.signed[i]);
                    let mut reference = BinaryPoint::Infinity;
                    for (x, y) in points {
                        let point = BinaryPoint::Affine {
                            x: F2mElement::from_words(&[x], original.curve.m),
                            y: F2mElement::from_words(&[y], original.curve.m),
                        };
                        budget.reserve_addition(Instant::now())?;
                        reference = point_add(&original.curve, &reference, &point);
                        result.witness_check_additions += 1;
                        budget.check(Instant::now())?;
                    }
                    let expected = match reference {
                        BinaryPoint::Infinity => None,
                        BinaryPoint::Affine { x, y } => {
                            x.to_biguint().to_u64().zip(y.to_biguint().to_u64())
                        }
                    };
                    if expected != Some(original.target) {
                        return budget.fail(Stop::NativeFailure);
                    }
                    let checked = CheckedSignedWitness {
                        indices: indices.map(|i| i as u8),
                        points,
                        target: original.target,
                    };
                    // Retain a returned durable receipt BEFORE subsequent checks.
                    result.first_receipt = Some(publish(&checked, budget)?);
                    budget.check(Instant::now())?;
                }
                event_sink(&event, budget)?;
                // Sink acknowledgement is a fact even if it consumed the final
                // deadline or swallowed a failed reservation. Preserve it first.
                result.acknowledged.push(event);
                result.unacknowledged = None;
                budget.check(Instant::now())?;
            }
        }
        budget.finish_extraction(Instant::now())?;
        result.complete = true;
        Ok(())
    })();
    if let Err(stop) = outcome {
        result.stop = Some(*budget.stopped.get_or_insert(stop));
    }
    result
}

#[cfg(test)]
mod signed_producer_tests {
    use super::*;
    use crate::cryptanalysis::koblitz_fast_arith::FastBinaryCurve;
    use crate::ml_dataset::capture::{WitnessCommit, WitnessProvenance, WitnessReceipt};
    use num_traits::ToPrimitive;
    fn prepared(original: &MathInput, additions: u64) -> (FastBinaryCurve, Budget) {
        let curve = FastBinaryCurve::new(
            &original.curve.irreducible,
            original.curve.a.to_biguint().to_u64().unwrap(),
        )
        .unwrap();
        let now = Instant::now();
        let mut b = Budget::new(
            additions,
            1 << 20,
            1 << 20,
            now + Duration::from_secs(20),
            now,
        )
        .unwrap();
        b.begin_native(false, now).unwrap();
        b.finish_native(now).unwrap();
        (curve, b)
    }
    fn receipt() -> WitnessReceipt {
        WitnessReceipt {
            commit: WitnessCommit::NewCommit,
            payload_sha256: [1; 32],
            payload_bytes: 128,
            sequence: Some(1),
            provenance: WitnessProvenance::Dedicated,
        }
    }
    #[test]
    fn signed_producer_exact_positive_budget_publishes_only_once() {
        let mut original = construction_tests::fixture();
        original.target = original.signed[0];
        for allowance in [194, 195] {
            let (curve, mut b) = prepared(&original, allowance);
            let mut published = 0;
            let r = enumerate_signed(
                &original,
                &curve,
                &mut b,
                false,
                &mut |_, _| Ok(()),
                &mut |_, _| {
                    published += 1;
                    Ok(receipt())
                },
            );
            assert_eq!(published, 1);
            assert_eq!(r.first_receipt, Some(receipt()));
            assert_eq!(r.witness_check_additions, 3);
            assert_eq!(r.enumeration_additions, allowance - 3);
            assert_eq!(b.additions, allowance);
            assert_eq!(r.complete, allowance == 195);
            assert_eq!(
                r.stop,
                if allowance == 195 {
                    None
                } else {
                    Some(Stop::Additions)
                }
            );
        }
    }
    #[test]
    fn signed_producer_retains_receipt_after_publisher_consumes_deadline() {
        let mut original = construction_tests::fixture();
        original.target = original.signed[0];
        let (curve, mut b) = prepared(&original, 195);
        let r = enumerate_signed(
            &original,
            &curve,
            &mut b,
            false,
            &mut |_, _| Ok(()),
            &mut |_, b| {
                b.deadline = Instant::now();
                Ok(receipt())
            },
        );
        assert_eq!(r.first_receipt, Some(receipt()));
        assert_eq!(r.stop, Some(Stop::Deadline));
        assert!(r.unacknowledged.as_ref().unwrap().matches_target);
        assert!(!r.complete);
    }
    #[test]
    fn signed_producer_exact_and_one_less_negative_allowance() {
        let original = construction_tests::fixture();
        for allowance in [191, 192] {
            let (curve, mut b) = prepared(&original, allowance);
            let r = enumerate_signed(
                &original,
                &curve,
                &mut b,
                false,
                &mut |_, _| Ok(()),
                &mut |_, _| panic!("fixed negative control unexpectedly matched"),
            );
            assert_eq!(r.enumeration_additions, allowance);
            assert_eq!(r.witness_check_additions, 0);
            assert_eq!(b.additions, allowance);
            assert_eq!(r.complete, allowance == 192);
            assert_eq!(
                r.stop,
                if allowance == 192 {
                    None
                } else {
                    Some(Stop::Additions)
                }
            );
            let replay =
                verify_signed_prefix(&original, &r.acknowledged, r.unacknowledged.as_ref())
                    .unwrap();
            assert_eq!(replay.all_tuples_acknowledged, allowance == 192);
            assert!(replay.candidate_tuples.is_empty());
        }
    }
    #[test]
    fn signed_producer_retains_first_receipt_when_match_event_is_refused() {
        let mut original = construction_tests::fixture();
        original.target = original.signed[0];
        let (curve, mut b) = prepared(&original, 195);
        let mut published = 0;
        let r = enumerate_signed(
            &original,
            &curve,
            &mut b,
            false,
            &mut |e, _| {
                if e.matches_target {
                    Err(Stop::Observer)
                } else {
                    Ok(())
                }
            },
            &mut |w, _| {
                assert_eq!(w.target, original.target);
                assert_eq!(w.points, w.indices.map(|i| original.signed[i as usize]));
                published += 1;
                Ok(receipt())
            },
        );
        assert_eq!(published, 1);
        assert_eq!(r.first_receipt, Some(receipt()));
        assert_eq!(r.witness_check_additions, 3);
        assert_eq!(r.stop, Some(Stop::Observer));
        assert!(!r.complete);
        assert!(r.unacknowledged.as_ref().unwrap().matches_target);
        assert_eq!(
            b.additions,
            r.enumeration_additions + r.witness_check_additions
        );
        verify_signed_prefix(&original, &r.acknowledged, r.unacknowledged.as_ref()).unwrap();
    }
    #[test]
    fn signed_producer_keeps_acknowledgement_after_swallowed_sink_budget_failure() {
        let original = construction_tests::fixture();
        let (curve, mut b) = prepared(&original, 192);
        let r = enumerate_signed(
            &original,
            &curve,
            &mut b,
            false,
            &mut |_, b| {
                assert_eq!(b.reserve_trace(u64::MAX, Instant::now()), Err(Stop::Trace));
                Ok(())
            },
            &mut |_, _| panic!("unexpected witness"),
        );
        assert_eq!(r.acknowledged.len(), 1);
        assert!(r.unacknowledged.is_none());
        assert_eq!(r.stop, Some(Stop::Trace));
        assert!(!r.complete);
    }
    #[test]
    fn signed_producer_rejects_wrong_prepared_kernel_before_any_addition() {
        let original = construction_tests::fixture();
        let (mut curve, mut b) = prepared(&original, 192);
        curve.a ^= 1;
        let r = enumerate_signed(
            &original,
            &curve,
            &mut b,
            false,
            &mut |_, _| panic!("unexpected event"),
            &mut |_, _| panic!("unexpected witness"),
        );
        assert_eq!(r.stop, Some(Stop::InvalidLimits));
        assert_eq!(b.additions, 0);
    }
}
