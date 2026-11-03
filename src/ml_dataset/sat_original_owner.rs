//! Draft generalized binary SAT original owner. This is not runtime dispatch.
//! Keep this module a child of koblitz_m3 to reuse its original cumulative
//! budget privately, without making a new budget authority available to callers.
use super::super::VerifiedWorkerOriginal;
use super::{Budget, Stop};
use crate::binary_ecc::{F2mElement, IrreduciblePoly};
use crate::cryptanalysis::binary_semaev_chain_sat::provenance::{self, Admission, Quote};
use crate::ml_dataset::{capture, execution};
use num_bigint::BigUint;
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::time::Instant;

/// Decoded solely from the worker's verified public inputs. No private witness.
/// Signed points remain available for later exact group lifting; x codes alone
/// do not certify a signed decomposition.
struct Math {
    a: F2mElement,
    b: F2mElement,
    irr: IrreduciblePoly,
    target: Option<F2mElement>,
    signed: Vec<(u64, u64)>,
    codes: Vec<BigUint>,
}

/// All intermediate owning buffers live in the admission frame before any
/// fallible decode operation. A failed decoder never drops its paid prefix.
#[derive(Default)]
struct DecodedParts {
    a: Option<F2mElement>,
    b: Option<F2mElement>,
    target: Option<Option<F2mElement>>,
    low_terms: Vec<u32>,
    signed: Vec<(u64, u64)>,
    codes: Vec<BigUint>,
    complete: Option<Math>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum AdmissionStage {
    AttemptBinding,
    Context,
    Reservation,
    Directory,
    Capture,
    Identity,
    Decode,
    BindingDigest,
    FinalDeadline,
}
/// Fixed-size typed reporting, with no format!/String/Box allocation on refusal.
/// An error before the budget is returned is distinct from charged admission.
#[allow(clippy::large_enum_variant)]
pub(super) enum AdmissionFailure<'a> {
    BeforeBudget(AdmissionStage),
    Retained(FailedAdmission<'a>),
}
pub(super) struct FailedAdmission<'a> {
    original: &'a VerifiedWorkerOriginal<'a>,
    budget: Budget,
    decoded: DecodedParts,
    stage: AdmissionStage,
}
impl FailedAdmission<'_> {
    fn stop(&mut self, stage: AdmissionStage, requested: Stop) {
        self.stage = stage;
        // fail preserves an already installed first_stop, e.g. Deadline must
        // not turn into InvalidLimits merely because reporting follows it.
        let _: Result<(), Stop> = self.budget.fail(requested);
    }
    pub(super) fn first_stop(&self) -> Option<Stop> {
        self.budget.stopped
    }
}

pub(super) struct OriginalOwner<'a> {
    original: &'a VerifiedWorkerOriginal<'a>,
    budget: Budget,
    binding: [u8; 32],
    launch_sha256: [u8; 32],
    math: Math,
    checked: Option<provenance::CheckedMap>,
    retained_unchecked: Option<provenance::Map>,
    pattern: Option<u32>,
    retained_bytes: Vec<u8>,
    map_ack: Option<capture::CommittedEventPosition>,
    publication_attempted: bool,
    map_published: bool,
    capture_limits: capture::CaptureLimits,
}

struct Admit<'a>(&'a mut Budget);
impl Admission for Admit<'_> {
    fn reserve(&mut self, q: Quote) -> Result<(), provenance::Error> {
        self.0
            .reserve_checking(q.work, q.bytes, Instant::now())
            .map_err(|_| provenance::Error::Admission)
    }
    fn check(&mut self) -> Result<(), provenance::Error> {
        self.0
            .check(Instant::now())
            .map_err(|_| provenance::Error::Admission)
    }
}

fn profile(family: &str, n: u32, m: usize) -> bool {
    matches!(family, "binary" | "koblitz")
        && ((23..=31).contains(&n) || (47..=61).contains(&n))
        && (2..=15).contains(&m)
}
fn word(v: &Value) -> Result<u64, Stop> {
    let s = v.as_str().ok_or(Stop::InvalidLimits)?;
    if s.is_empty() || s.len() > 20 || (s.len() > 1 && s.starts_with('0')) {
        return Err(Stop::InvalidLimits);
    }
    s.bytes().try_fold(0u64, |n, b| {
        if !b.is_ascii_digit() {
            return Err(Stop::InvalidLimits);
        }
        n.checked_mul(10)
            .and_then(|n| n.checked_add((b - b'0') as u64))
            .ok_or(Stop::Overflow)
    })
}
fn field(value: u64, n: u32) -> F2mElement {
    let mut bits = [0u32; 61];
    let mut count = 0;
    for bit in 0..n {
        if value & (1u64 << bit) != 0 {
            bits[count] = bit;
            count += 1;
        }
    }
    F2mElement::from_bit_positions(&bits[..count], n)
}

fn decode_into(
    token: &VerifiedWorkerOriginal<'_>,
    budget: &mut impl DecodeBudget,
    state: &mut DecodedParts,
) -> Result<(), Stop> {
    // Length/shape-only admission precedes content scans and allocation. Existing
    // original verification supplies irreducibility, group and factor-base math.
    budget.reserve_checking(1024, 0, Instant::now())?;
    let c = token.config;
    let p = token.public;
    let base = token.factor_base;
    if !profile(&c.family, c.n, c.m)
        || token.binding.is_none()
        || p["family"] != c.family
        || p["n"].as_u64() != Some(c.n as u64)
        || p["m"].as_u64() != Some(c.m as u64)
        || p["twist"].as_bool() != Some(c.twist)
        || p["field"]["degree"].as_u64() != Some(c.n as u64)
        || p["field"]["characteristic"] != "2"
        || base["requested_size"].as_u64() != Some(c.factor_base_size as u64)
    {
        return budget.fail(Stop::InvalidLimits);
    }
    let rows = base["points"].as_array().ok_or(Stop::InvalidLimits)?;
    let terms = p["field"]["irreducible_low_terms"]
        .as_array()
        .ok_or(Stop::InvalidLimits)?;
    if rows.is_empty() || terms.is_empty() || terms.len() > c.n as usize {
        return budget.fail(Stop::InvalidLimits);
    }
    let count = u64::try_from(rows.len()).map_err(|_| Stop::Overflow)?;
    // Bounded word decoding, sign completion, sorting, canonical BigUint copies.
    // Sorting work is conservatively quadratic; these are modeled units, never
    // reported as retired instructions or a measured performance improvement.
    let work = count
        .checked_mul(count)
        .and_then(|v| v.checked_mul(32))
        .and_then(|v| v.checked_add(8192))
        .and_then(|v| count.checked_mul(512).and_then(|w| v.checked_add(w)))
        .ok_or(Stop::Overflow)?;
    let bytes = count
        .checked_mul(512)
        .and_then(|v| v.checked_add(8192))
        .ok_or(Stop::Overflow)?;
    budget.reserve_checking(work, bytes, Instant::now())?;
    let result = (|| {
        let q = 1u64 << c.n;
        let scalar = |v: &Value| -> Result<u64, Stop> {
            let x = word(v)?;
            if x >= q {
                Err(Stop::InvalidLimits)
            } else {
                Ok(x)
            }
        };
        let a = scalar(&p["curve"]["a"])?;
        let b = scalar(&p["curve"]["b"])?;
        if b == 0 || (c.family == "koblitz" && b != 1) {
            return Err(Stop::InvalidLimits);
        }
        // Native binary point format uses the same infinity tag as group replay.
        state.a = Some(field(a, c.n));
        state.b = Some(field(b, c.n));
        state.target = Some(if p["target"].is_null() {
            None
        } else {
            Some(field(scalar(&p["target"]["x"])?, c.n))
        });
        state
            .low_terms
            .try_reserve_exact(terms.len())
            .map_err(|_| Stop::Allocation)?;
        for term in terms {
            budget.check(Instant::now())?;
            let value = term.as_u64().ok_or(Stop::InvalidLimits)?;
            if value >= c.n as u64 || state.low_terms.contains(&(value as u32)) {
                return Err(Stop::InvalidLimits);
            }
            state.low_terms.push(value as u32);
        }
        state.low_terms.sort_unstable();
        state
            .signed
            .try_reserve_exact(rows.len().checked_mul(2).ok_or(Stop::Overflow)?)
            .map_err(|_| Stop::Allocation)?;
        for row in rows {
            budget.check(Instant::now())?;
            let x = scalar(&row["x"])?;
            let y = scalar(&row["y"])?;
            state.signed.push((x, y));
            state.signed.push((x, x ^ y));
        }
        state.signed.sort_unstable();
        state.signed.dedup();
        state
            .codes
            .try_reserve_exact(state.signed.len())
            .map_err(|_| Stop::Allocation)?;
        let mut previous = None;
        for &(x, _) in &state.signed {
            budget.check(Instant::now())?;
            if previous != Some(x) {
                state.codes.push(BigUint::from(x));
                previous = Some(x);
            }
        }
        // All checks precede the infallible move into the complete slot. The
        // subsequent deadline check leaves that slot owned by this same frame.
        budget.check(Instant::now())?;
        state.complete = Some(Math {
            a: state.a.take().expect("decoder assigned a"),
            b: state.b.take().expect("decoder assigned b"),
            irr: IrreduciblePoly {
                degree: c.n,
                low_terms: std::mem::take(&mut state.low_terms),
            },
            target: state.target.take().expect("decoder assigned target"),
            signed: std::mem::take(&mut state.signed),
            codes: std::mem::take(&mut state.codes),
        });
        Ok(())
    })();
    match result {
        Ok(()) => budget.check(Instant::now()),
        Err(e) => budget.fail(e),
    }
}

impl<'a> OriginalOwner<'a> {
    /// Original attempt and sink joins precede construction. Reuses the admitted
    /// absolute deadline; no Instant::now()+10 seconds or per-pattern reset.
    pub(super) fn admit(
        original: &'a VerifiedWorkerOriginal<'a>,
        case: &crate::ml_dataset::types::CaseData,
        context: &execution::SolverExecutionContext,
        sink: &capture::CaptureEmitter,
        capture_limits: &capture::CaptureLimits,
    ) -> Result<Self, AdmissionFailure<'a>> {
        let Some(binding) = original.binding else {
            return Err(AdmissionFailure::BeforeBudget(
                AdmissionStage::AttemptBinding,
            ));
        };
        // Existing context admission is the pre-budget boundary. It returns no
        // admitted charge on error. Once a Budget is returned, immediately move
        // it into the failure frame before any further fallible operation.
        let budget = super::admit_original(original.root, original.config, context, sink)
            .map_err(|_| AdmissionFailure::BeforeBudget(AdmissionStage::Context))?;
        let mut frame = FailedAdmission {
            original,
            budget,
            decoded: DecodedParts::default(),
            stage: AdmissionStage::Reservation,
        };
        let admitted = (|| -> Result<([u8; 32], [u8; 32]), (AdmissionStage, Stop)> {
            let input_work = original
                .public_bytes
                .checked_add(original.factor_base_bytes)
                .and_then(|n| n.checked_mul(32))
                .and_then(|n| n.checked_add(65536))
                .ok_or((AdmissionStage::Reservation, Stop::Overflow))?;
            frame
                .budget
                .reserve_checking(input_work, 2 * 1024 * 1024 + 4096, Instant::now())
                .map_err(|s| (AdmissionStage::Reservation, s))?;
            sink.verify_directory(&original.root.join("live"))
                .map_err(|_| (AdmissionStage::Directory, Stop::Observer))?;
            sink.verify_attempt_limits(binding, capture_limits)
                .map_err(|_| (AdmissionStage::Capture, Stop::Observer))?;
            original
                .matches_bound(original.config, case, &original.root.join("live"), binding)
                .map_err(|_| (AdmissionStage::Identity, Stop::InvalidLimits))?;
            decode_into(original, &mut frame.budget, &mut frame.decoded)
                .map_err(|s| (AdmissionStage::Decode, s))?;
            let mut hash = Sha256::new();
            hash.update(b"ml-dataset.sat-original-owner/v1\0");
            // Fixed metadata/hash reservation above precedes this existing
            // String-producing digest API. No new allocation occurs on refusal.
            hash.update(
                binding
                    .digest()
                    .map_err(|_| (AdmissionStage::BindingDigest, Stop::InvalidLimits))?
                    .as_bytes(),
            );
            hash.update(original.public_sha256);
            hash.update(original.factor_base_sha256);
            frame
                .budget
                .check(Instant::now())
                .map_err(|s| (AdmissionStage::FinalDeadline, s))?;
            // Hash the actual admitted execution context without an encoded Vec.
            // Each write is reserved before hashing; state stays in this frame.
            let mut launch_writer = LaunchHash {
                budget: &mut frame.budget,
                hash: Sha256::new(),
                bytes: 0,
            };
            serde_json::to_writer(&mut launch_writer, &context.launch)
                .map_err(|_| (AdmissionStage::BindingDigest, Stop::Serialization))?;
            let launch_sha256 = launch_writer.hash.finalize().into();
            frame
                .budget
                .check(Instant::now())
                .map_err(|s| (AdmissionStage::FinalDeadline, s))?;
            Ok((hash.finalize().into(), launch_sha256))
        })();
        match admitted {
            Err((stage, stop)) => {
                frame.stop(stage, stop);
                Err(AdmissionFailure::Retained(frame))
            }
            Ok((binding, launch_sha256)) => {
                // No fallible operation after moving the complete math owner.
                let math = frame
                    .decoded
                    .complete
                    .take()
                    .expect("successful decode owns math");
                Ok(Self {
                    original,
                    budget: frame.budget,
                    binding,
                    launch_sha256,
                    math,
                    checked: None,
                    retained_unchecked: None,
                    pattern: None,
                    retained_bytes: Vec::new(),
                    map_ack: None,
                    publication_attempted: false,
                    map_published: false,
                    capture_limits: capture_limits.clone(),
                })
            }
        }
    }

    /// One map slot: caller must retain/archive a finished pattern before moving
    /// to the next. This draft does not grant a reset/retry or release old charges.
    pub(super) fn build_map(
        &mut self,
        pattern: u32,
        limits: provenance::Limits,
    ) -> Result<(), Stop> {
        if self.pattern.is_some() {
            return self.budget.fail(Stop::Phase);
        }
        let count = 1u32 << (self.original.config.m - 2);
        if pattern >= count {
            return self.budget.fail(Stop::InvalidLimits);
        }
        self.pattern = Some(pattern);
        let original = provenance::Original {
            binding: self.binding,
            a: &self.math.a,
            b: &self.math.b,
            irr: &self.math.irr,
            twist: self.original.config.twist,
            m: self.original.config.m,
            target: self.math.target.as_ref(),
            pattern,
            codes: &self.math.codes,
        };
        match provenance::build_original(&original, limits, &mut Admit(&mut self.budget)) {
            Ok(map) => {
                self.checked = Some(map);
                Ok(())
            }
            Err(failed) => {
                self.retained_unchecked = failed.retained;
                self.budget.fail(Stop::NativeFailure)
            }
        }
    }

    /// Persist the exact checked original map before initial attachment/solve.
    /// Failed/indeterminate publication keeps the map and bytes in this owner.
    /// A head receipt is minted only after the actual capture call succeeds;
    /// it is not semantic replay or permission to export a search prefix.
    pub(super) fn persist_map(&mut self, sink: &mut capture::CaptureEmitter) -> Result<(), Stop> {
        let result = self.persist_map_inner(sink);
        match result {
            Ok(()) => Ok(()),
            Err(stop) => self.budget.fail(stop),
        }
    }
    fn persist_map_inner(&mut self, sink: &mut capture::CaptureEmitter) -> Result<(), Stop> {
        if self.publication_attempted {
            return Err(Stop::Phase);
        }
        let map = self.checked.as_ref().ok_or(Stop::Phase)?;
        self.publication_attempted = true;
        self.budget.reserve_checking(8192, 8192, Instant::now())?;
        let binding = self.original.binding.ok_or(Stop::InvalidLimits)?;
        sink.verify_directory(&self.original.root.join("live"))
            .map_err(|_| Stop::Observer)?;
        sink.verify_attempt_limits(binding, &self.capture_limits)
            .map_err(|_| Stop::Observer)?;
        let before = sink.committed_event_position();
        let cap =
            usize::try_from(self.capture_limits.max_payload_bytes).map_err(|_| Stop::Overflow)?;
        // The first pass charges each trusted serializer write before counting;
        // no pessimistic full-cap debit and no output allocation on that pass.
        let mut count = Count {
            budget: &mut self.budget,
            len: 0,
            cap,
        };
        serde_json::to_writer(&mut count, &provenance::gate_wire::encode(map))
            .map_err(|_| Stop::Serialization)?;
        let expected = count.len;
        let work = (expected as u64).checked_mul(8).ok_or(Stop::Overflow)?;
        self.budget
            .reserve_checking(work, expected as u64, Instant::now())?;
        self.retained_bytes
            .try_reserve_exact(expected)
            .map_err(|_| Stop::Allocation)?;
        let mut writer = Bounded {
            bytes: &mut self.retained_bytes,
            cap: expected,
            budget: &mut self.budget,
        };
        serde_json::to_writer(&mut writer, &provenance::gate_wire::encode(map))
            .map_err(|_| Stop::Serialization)?;
        if self.retained_bytes.len() != expected {
            return Err(Stop::Serialization);
        }
        // Charge hashing, local write/replay, bounded payload-commit metadata and
        // fixed receipt owner before capture. This is a modeled tariff, not PMU.
        let quote =
            super::quote_parent_storage_length(expected as u64, false, &self.capture_limits)?;
        self.budget
            .reserve_checking(quote.work, quote.bytes, Instant::now())?;
        self.budget.reserve_trace(quote.trace, Instant::now())?;
        let pattern = self.pattern.ok_or(Stop::Phase)?;
        let path = format!("sat-original-pattern-{pattern}.gate.json");
        sink.payload_borrowed(&path, "sat-original-gate-map", "json", &self.retained_bytes)
            .map_err(|_| Stop::Observer)?;
        let position = sink.committed_event_position();
        // Keep the receipt immediately after acknowledged I/O, even if a later
        // deadline/integrity check fails. Do not recreate a synthetic receipt.
        let changed = position.event_count
            == before.event_count.checked_add(1).ok_or(Stop::Overflow)?
            && position.binding_sha256 == before.binding_sha256
            && position.byte_end > before.byte_end;
        self.map_ack = Some(position);
        self.map_published = changed;
        if !changed {
            return Err(Stop::Observer);
        }
        Ok(())
    }

    /// This seal authenticates insertion only. It intentionally provides no
    /// durable acknowledgement and authorizes no search or public export.
    pub(super) fn verify_snapshot<'s>(
        &'s mut self,
        events: &'s [crate::cryptanalysis::sat::SolverEvent],
        limits: provenance::initial::Limits,
    ) -> Result<provenance::initial::VerifiedInitial<'s>, Stop> {
        if !self.map_published {
            return self.budget.fail(Stop::Phase);
        }
        let map = self.checked.as_ref().ok_or(Stop::Phase)?;
        match provenance::initial::verify_initial(map, events, limits, &mut Admit(&mut self.budget))
        {
            Ok(seal) => Ok(seal),
            Err(_) => self.budget.fail(Stop::NativeFailure),
        }
    }
}

// Both writers are bounded before mutation. Count charges accepted work even
// when a later serializer call refuses; the original ledger is never reset.
struct Count<'a> {
    budget: &'a mut Budget,
    len: usize,
    cap: usize,
}
impl std::io::Write for Count<'_> {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        let next = self
            .len
            .checked_add(bytes.len())
            .filter(|n| *n <= self.cap)
            .ok_or_else(|| std::io::Error::from(std::io::ErrorKind::WriteZero))?;
        let work = (bytes.len() as u64)
            .checked_mul(8)
            .ok_or_else(|| std::io::Error::from(std::io::ErrorKind::InvalidInput))?;
        self.budget
            .reserve_checking(work, 0, Instant::now())
            .map_err(|_| std::io::Error::from(std::io::ErrorKind::Other))?;
        self.len = next;
        Ok(bytes.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
struct Bounded<'a> {
    budget: &'a mut Budget,
    bytes: &'a mut Vec<u8>,
    cap: usize,
}
impl std::io::Write for Bounded<'_> {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        self.budget
            .check(Instant::now())
            .map_err(|_| std::io::Error::from(std::io::ErrorKind::Other))?;
        self.bytes
            .len()
            .checked_add(bytes.len())
            .filter(|n| *n <= self.cap)
            .ok_or_else(|| std::io::Error::from(std::io::ErrorKind::WriteZero))?;
        self.bytes.extend_from_slice(bytes);
        Ok(bytes.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::verified_worker_original_tests::Fixture;
    use super::*;
    fn test_budget() -> Budget {
        let now = Instant::now();
        Budget::new(
            16384,
            512 << 20,
            16 << 20,
            now + std::time::Duration::from_secs(20),
            now,
        )
        .unwrap()
    }
    fn retained<'a>(
        result: Result<OriginalOwner<'a>, AdmissionFailure<'a>>,
    ) -> FailedAdmission<'a> {
        match result {
            Err(AdmissionFailure::Retained(frame)) => frame,
            _ => panic!("expected retained charged admission failure"),
        }
    }
    #[test]
    fn real_factory_accepts_authentic_owner_before_failure_controls() {
        let f = Fixture::with_ordinary_runtime_shape(23, false, "sat");
        execution::create(&f.root).unwrap();
        let mut registry = Vec::new();
        let generated = crate::ml_dataset::telemetry::measured(
            &f.root,
            &mut registry,
            "curve-construction",
            || crate::ml_dataset::curves::generate(&f.config),
        )
        .unwrap();
        let token = super::super::super::verify_and_store_worker_originals(
            &f.config,
            &generated,
            &f.root,
            Some(&f.binding),
            &mut registry,
        )
        .unwrap();
        let context = execution::admit(&f.root).unwrap().unwrap();
        let (_, limits) = crate::ml_dataset::admit_capture_context(&f.root)
            .unwrap()
            .unwrap();
        let sink = capture::CaptureEmitter::create(
            &f.root.join("live"),
            f.binding.clone(),
            limits.clone(),
        )
        .unwrap();
        let owner = match OriginalOwner::admit(&token, &generated, &context, &sink, &limits) {
            Ok(owner) => owner,
            Err(_) => panic!("authentic admission must pass before mutation controls"),
        };
        assert!(owner.budget.checking_work > 65536);
        assert!(owner.budget.adapter_bytes > 2 * 1024 * 1024);
        assert_eq!(owner.budget.stopped, None);
        assert!(!owner.math.codes.is_empty());
        assert_eq!(sink.committed_event_count(), 0);
    }
    #[test]
    fn real_factory_retains_ledger_after_identity_and_capture_refusal() {
        for capture_failure in [false, true] {
            let f = Fixture::with_ordinary_runtime_shape(23, false, "sat");
            execution::create(&f.root).unwrap();
            let mut registry = Vec::new();
            let generated = crate::ml_dataset::telemetry::measured(
                &f.root,
                &mut registry,
                "curve-construction",
                || crate::ml_dataset::curves::generate(&f.config),
            )
            .unwrap();
            let token = super::super::super::verify_and_store_worker_originals(
                &f.config,
                &generated,
                &f.root,
                Some(&f.binding),
                &mut registry,
            )
            .unwrap();
            let context = execution::admit(&f.root).unwrap().unwrap();
            let (_, limits) = crate::ml_dataset::admit_capture_context(&f.root)
                .unwrap()
                .unwrap();
            let sink = capture::CaptureEmitter::create(
                &f.root.join("live"),
                f.binding.clone(),
                limits.clone(),
            )
            .unwrap();
            // An equal-but-distinct CaseData cannot replace token references.
            let foreign = generated.clone();
            let mut supplied = limits.clone();
            if capture_failure {
                supplied.max_payload_count -= 1;
            }
            let case = if capture_failure {
                &generated
            } else {
                &foreign
            };
            let frame = retained(OriginalOwner::admit(
                &token, case, &context, &sink, &supplied,
            ));
            assert!(std::ptr::eq(frame.original, &token));
            assert!(frame.budget.checking_work >= 65536);
            assert!(frame.budget.adapter_bytes >= 2 * 1024 * 1024);
            assert_eq!(frame.budget.native, super::super::Reservation::default());
            assert_eq!(frame.budget.trace_bytes, 0);
            assert_eq!(
                frame.stage,
                if capture_failure {
                    AdmissionStage::Capture
                } else {
                    AdmissionStage::Identity
                }
            );
            assert_eq!(
                frame.first_stop(),
                Some(if capture_failure {
                    Stop::Observer
                } else {
                    Stop::InvalidLimits
                })
            );
            assert!(frame.decoded.complete.is_none());
            assert_eq!(sink.committed_event_count(), 0);
        }
    }
    #[test]
    fn decoder_prefix_and_complete_math_survive_refusal_without_refunding() {
        let f = Fixture::with_shape(23, false, 8);
        let mut registry = Vec::new();
        let token = super::super::super::verify_and_store_worker_originals(
            &f.config,
            &f.case,
            &f.root,
            Some(&f.binding),
            &mut registry,
        )
        .unwrap();
        // First establish that this actual original decodes successfully.
        let mut baseline = DecodedParts::default();
        decode_into(&token, &mut test_budget(), &mut baseline).unwrap();
        assert!(baseline.complete.is_some());
        let mut altered = f.case.factor_base.clone();
        altered["points"][1]["y"] = Value::String("not-canonical".into());
        // Deliberately forged test-only token exercises the decoder's defensive
        // failure path. It is never accepted as a genuine worker authority.
        let forged = VerifiedWorkerOriginal {
            config: token.config,
            public: token.public,
            factor_base: &altered,
            root: token.root,
            binding: token.binding,
            public_sha256: token.public_sha256,
            factor_base_sha256: token.factor_base_sha256,
            public_bytes: token.public_bytes,
            factor_base_bytes: token.factor_base_bytes,
        };
        let mut frame = FailedAdmission {
            original: &forged,
            budget: test_budget(),
            decoded: DecodedParts::default(),
            stage: AdmissionStage::Reservation,
        };
        frame
            .budget
            .reserve_checking(123, 456, Instant::now())
            .unwrap();
        let owner = frame.budget.ownership_id;
        let error = decode_into(&forged, &mut frame.budget, &mut frame.decoded).unwrap_err();
        frame.stop(AdmissionStage::Decode, error);
        assert_eq!(frame.budget.ownership_id, owner);
        assert!(frame.budget.checking_work > 123);
        assert!(frame.budget.adapter_bytes > 456);
        assert!(!frame.decoded.low_terms.is_empty());
        assert!(!frame.decoded.signed.is_empty());
        assert!(frame.decoded.signed.capacity() >= 2);
        assert_eq!(frame.first_stop(), Some(Stop::InvalidLimits));
        let before = (frame.budget.checking_work, frame.budget.adapter_bytes);
        frame.stop(AdmissionStage::FinalDeadline, Stop::Deadline);
        assert_eq!(frame.first_stop(), Some(Stop::InvalidLimits));
        assert_eq!(
            (frame.budget.checking_work, frame.budget.adapter_bytes),
            before
        );

        let mut completed = FailedAdmission {
            original: &token,
            budget: test_budget(),
            decoded: DecodedParts::default(),
            stage: AdmissionStage::Decode,
        };
        decode_into(&token, &mut completed.budget, &mut completed.decoded).unwrap();
        let math = completed.decoded.complete.as_ref().unwrap();
        let ptr = math.codes.as_ptr();
        let before = (
            completed.budget.checking_work,
            completed.budget.adapter_bytes,
        );
        completed.budget.construction_deadline = Instant::now() - std::time::Duration::from_secs(1);
        let stop = completed.budget.check(Instant::now()).unwrap_err();
        completed.stop(AdmissionStage::FinalDeadline, stop);
        assert_eq!(completed.first_stop(), Some(Stop::Deadline));
        assert_eq!(
            completed.decoded.complete.as_ref().unwrap().codes.as_ptr(),
            ptr
        );
        assert_eq!(
            (
                completed.budget.checking_work,
                completed.budget.adapter_bytes
            ),
            before
        );
        completed.stop(AdmissionStage::Identity, Stop::InvalidLimits);
        assert_eq!(completed.first_stop(), Some(Stop::Deadline));
    }

    #[test]
    fn bounded_two_pass_encoding_handles_escaping_and_keeps_refusal_charges() {
        use std::io::Write;
        let now = Instant::now();
        let mut budget = Budget::new(
            16384,
            512 << 20,
            16 << 20,
            now + std::time::Duration::from_secs(20),
            now,
        )
        .unwrap();
        let value =
            serde_json::json!({"strings":["quote\"", "line\n", "nul\0"],"words":[0,u64::MAX]});
        let actual = serde_json::to_vec(&value).unwrap();
        let mut count = Count {
            budget: &mut budget,
            len: 0,
            cap: actual.len(),
        };
        serde_json::to_writer(&mut count, &value).unwrap();
        assert_eq!(count.len, actual.len());
        let charged = budget.checking_work;
        assert_eq!(charged, 8 * actual.len() as u64);
        budget
            .reserve_checking(8 * actual.len() as u64, actual.len() as u64, Instant::now())
            .unwrap();
        let mut bytes = Vec::with_capacity(actual.len());
        let mut writer = Bounded {
            budget: &mut budget,
            bytes: &mut bytes,
            cap: actual.len(),
        };
        serde_json::to_writer(&mut writer, &value).unwrap();
        assert!(writer.write_all(b"surplus").is_err());
        assert_eq!(bytes, actual);
        let before = budget.checking_work;
        let mut refused = Count {
            budget: &mut budget,
            len: 0,
            cap: actual.len() - 1,
        };
        assert!(serde_json::to_writer(&mut refused, &value).is_err());
        assert!(budget.checking_work > before);
        budget.fail::<()>(Stop::Deadline).unwrap_err();
        let mut stopped = Count {
            budget: &mut budget,
            len: 0,
            cap: actual.len(),
        };
        assert!(stopped.write_all(b"x").is_err());
        assert_eq!(stopped.len, 0);
    }
    #[test]
    fn declared_full_binary_profile_is_not_m3_only() {
        for family in ["binary", "koblitz"] {
            for n in (23..=31).chain(47..=61) {
                for m in 2..=15 {
                    assert!(profile(family, n, m));
                }
            }
        }
        assert!(!profile("prime", 61, 3));
        assert!(!profile("binary", 32, 3));
        assert!(!profile("binary", 61, 16));
    }
    #[test]
    fn authentic_decoder_matches_existing_domain_both_twists_and_field_endpoints() {
        for n in [23, 61] {
            for twist in [false, true] {
                let f = Fixture::with_shape(n, twist, 8);
                let mut registry = Vec::new();
                let token = super::super::super::verify_and_store_worker_originals(
                    &f.config,
                    &f.case,
                    &f.root,
                    Some(&f.binding),
                    &mut registry,
                )
                .unwrap();
                let now = Instant::now();
                let mut budget = Budget::new(
                    16384,
                    512 << 20,
                    16 << 20,
                    now + std::time::Duration::from_secs(20),
                    now,
                )
                .unwrap();
                let mut decoded = DecodedParts::default();
                decode_into(&token, &mut budget, &mut decoded).unwrap();
                let original = decoded.complete.as_ref().unwrap();
                let group = super::super::super::Group::read(&f.case).unwrap();
                let domain = super::super::super::domain(&group, &f.case).unwrap();
                let mut codes: Vec<_> = domain
                    .iter()
                    .filter_map(super::super::super::Element::x)
                    .collect();
                codes.sort();
                codes.dedup();
                assert_eq!(original.codes, codes);
                assert_eq!(original.signed.len(), domain.len());
                let super::super::super::Group::Binary(curve) = group else {
                    panic!()
                };
                assert_eq!(original.a, curve.a);
                assert_eq!(original.b, curve.b);
                assert_eq!(original.irr.degree, curve.irreducible.degree);
                let mut terms = curve.irreducible.low_terms;
                terms.sort();
                assert_eq!(original.irr.low_terms, terms);
                budget.fail::<()>(Stop::Deadline).unwrap_err();
                let mut refused = DecodedParts::default();
                assert!(matches!(
                    decode_into(&token, &mut budget, &mut refused),
                    Err(Stop::Deadline)
                ));
                assert!(decoded.complete.is_some());
            }
        }
    }
}

#[path = "sat_initial_native.rs"]
mod initial_native;

// Canonical typed LaunchContext hash, bounded and prepaid before mutation.
struct LaunchHash<'a> {
    budget: &'a mut Budget,
    hash: Sha256,
    bytes: usize,
}
impl std::io::Write for LaunchHash<'_> {
    fn write(&mut self, b: &[u8]) -> std::io::Result<usize> {
        let n = self
            .bytes
            .checked_add(b.len())
            .filter(|n| *n <= 2048)
            .ok_or_else(|| std::io::Error::from(std::io::ErrorKind::WriteZero))?;
        self.budget
            .reserve_checking((b.len() as u64) * 8, 0, Instant::now())
            .map_err(|_| std::io::Error::from(std::io::ErrorKind::Other))?;
        self.hash.update(b);
        self.bytes = n;
        Ok(b.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

// Read-only historical replay uses the same decoder with its own cumulative
// replay admission. This never changes a producer Budget or original deadline.
trait DecodeBudget {
    fn reserve_checking(&mut self, work: u64, bytes: u64, now: Instant) -> Result<(), Stop>;
    fn check(&mut self, now: Instant) -> Result<(), Stop>;
    fn fail<T>(&mut self, stop: Stop) -> Result<T, Stop>;
}
impl DecodeBudget for Budget {
    fn reserve_checking(&mut self, w: u64, b: u64, n: Instant) -> Result<(), Stop> {
        Budget::reserve_checking(self, w, b, n)
    }
    fn check(&mut self, n: Instant) -> Result<(), Stop> {
        Budget::check(self, n)
    }
    fn fail<T>(&mut self, s: Stop) -> Result<T, Stop> {
        Budget::fail(self, s)
    }
}
#[path = "sat_initial_central.rs"]
pub(in crate::ml_dataset::solvers) mod central;

#[path = "sat_initial_worker.rs"]
pub(in crate::ml_dataset::solvers) mod worker;
