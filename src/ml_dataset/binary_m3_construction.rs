//! Ordinary-binary construction bridge; test-only registration, no worker dispatch.
//! The caller owns an already-admitted ledger and authenticated original token.
use super::super::{
    binary_m3_original::Profile, binary_m3_resultant as resultant, VerifiedWorkerOriginal,
};
use super::*;
use resultant::{Failure, OriginalAdmission, Quote};
use serde::{Deserialize, Serialize};
use std::io::Write;

#[path = "binary_m3_persisted.rs"]
pub(in crate::ml_dataset::solvers) mod persisted;

const CAP: usize = 8192;
const SCHEMA: &str = "crypto.ml-dataset.binary-m3-resultant-construction/v1";
pub(super) const PATH: &str = "binary-m3-resultant-construction.json";
pub(super) const ROLE: &str = "restricted-binary-m3-resultant-construction/v1";

/// Incremental builder reservations become cumulative native observations.
/// Verifier and encoding work instead debit the SAME ledger's checking pool.
struct Admission<'a> {
    ledger: &'a mut Budget,
    native: bool,
    cumulative: Reservation,
}
impl OriginalAdmission for Admission<'_> {
    fn reserve(&mut self, q: Quote) -> Result<(), Failure> {
        let result = if self.native {
            match (
                self.cumulative.work.checked_add(q.work),
                self.cumulative.bytes.checked_add(q.bytes),
            ) {
                (Some(work), Some(bytes)) => {
                    let next = Reservation { work, bytes };
                    let r = self.ledger.observe(next, Instant::now());
                    if r.is_ok() {
                        self.cumulative = next;
                    }
                    r
                }
                _ => self.ledger.fail(Stop::Overflow),
            }
        } else {
            self.ledger
                .reserve_checking(q.work, q.bytes, Instant::now())
        };
        result.map_err(failure_from_stop)
    }
    fn check_deadline(&mut self) -> Result<(), Failure> {
        self.ledger.check(Instant::now()).map_err(failure_from_stop)
    }
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ConstructionArtifact {
    schema: String,
    original_public_sha256: [u8; 32],
    original_factor_base_sha256: [u8; 32],
    original_config_sha256: String,
    binding_sha256: String,
    // Policy/seed identity is retained through the authenticated factor/config
    // hashes. No caller-provided policy replacement enters construction.
    original_limits: OriginalLimits,
    before: SavedBudgetLedger,
    after: SavedBudgetLedger,
    failure: Option<Failure>,
    determinant_verified: bool,
    evidence: Option<resultant::Evidence>,
}
pub(super) struct Retained {
    pub bytes: Vec<u8>,
    pub constructed: Option<resultant::Constructed>,
    pub failure: Option<Failure>,
    pub encoding_failed: bool,
}
struct Capped<'a>(&'a mut Vec<u8>);
impl Write for Capped<'_> {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        if bytes.len() > CAP - self.0.len() {
            return Err(std::io::Error::other("ordinary construction encoding cap"));
        }
        self.0.extend_from_slice(bytes);
        Ok(bytes.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
/// Preserve error class while the borrowed ledger retains its original first
/// Stop. No conversion resets/refunds the ledger or manufactures completion.
fn failure_from_stop(stop: Stop) -> Failure {
    match stop {
        Stop::Deadline => Failure::Deadline,
        Stop::Allocation => Failure::Allocation,
        Stop::InvalidLimits => Failure::Input,
        Stop::Overflow
        | Stop::Phase
        | Stop::Nonmonotone
        | Stop::Work
        | Stop::Bytes
        | Stop::Trace
        | Stop::Additions
        | Stop::Observer
        | Stop::NativeFailure
        | Stop::Serialization
        | Stop::Events => Failure::Admission,
    }
}
fn note_failure(ledger: &mut Budget, failure: Failure) {
    let stop = match failure {
        Failure::Deadline => Stop::Deadline,
        Failure::Allocation => Stop::Allocation,
        Failure::Admission => ledger.stopped.unwrap_or(Stop::NativeFailure),
        Failure::Input | Failure::Mismatch | Failure::TallyActive => Stop::NativeFailure,
    };
    let _: Result<(), _> = ledger.fail(stop);
}
fn elements(math: &MathInput) -> [crate::binary_ecc::F2mElement; 3] {
    [math.target.0, math.codes[0], math.codes[1]]
        .map(|word| crate::binary_ecc::F2mElement::from_words(&[word], math.curve.m))
}
fn input<'a>(
    math: &'a MathInput,
    values: &'a [crate::binary_ecc::F2mElement; 3],
) -> resultant::Input<'a> {
    resultant::Input {
        field: &math.curve.irreducible,
        b: &math.curve.b,
        target_x: &values[0],
        codes: [&values[1], &values[2]],
    }
}
/// The worker wrapper must already have called token.matches_bound. This entry
/// additionally checks that the borrowed ledger has the original limits and
/// deadline. These equalities do NOT authenticate the ledger's attempt identity:
/// the trusted caller must borrow the actual attempt ledger, never construct a
/// same-limit replacement. This component creates no budget and renews no clock.
pub(super) fn construct(
    token: &VerifiedWorkerOriginal<'_>,
    context: &crate::ml_dataset::execution::SolverExecutionContext,
    ledger: &mut Budget,
) -> Result<Retained, Stop> {
    ledger.check(Instant::now())?;
    if ledger.phase != Phase::Setup
        || token.binding != Some(&context.launch.binding)
        || ledger.deadline != context.deadline
        || ledger.original_limits()
            != OriginalLimits::from_original(token.config, &context.launch)
                .map_err(|_| Stop::InvalidLimits)?
        || token.config.family != "binary"
        || !matches!(token.config.solver.as_str(), "f4" | "f5")
        || !(1..=6).contains(&token.config.degree_bound)
    {
        return ledger.fail(Stop::InvalidLimits);
    }
    // Fixed bounded envelope/copy/serializer overhead is prepaid. Encoding can
    // retain partial evidence after a stop without a new resource admission.
    ledger.reserve_checking(
        32 * CAP as u64 + 16384,
        2 * CAP as u64 + 4096,
        Instant::now(),
    )?;
    let mut bytes = Vec::new();
    if bytes.try_reserve_exact(CAP).is_err() {
        return ledger.fail(Stop::Allocation);
    }
    let binding = token.binding.unwrap();
    let binding_sha256 = binding.digest().map_err(|_| Stop::InvalidLimits)?;
    let before = ledger.saved_ledger();
    let mut constructed = None;
    let mut evidence = None;
    let mut verified = false;
    let operation = (|| -> Result<(), Failure> {
        let math = decode_verified_profile_math(token, ledger, Profile::OrdinaryBinary)
            .map_err(failure_from_stop)?;
        let values = elements(&math);
        let source = input(&math, &values);
        ledger
            .begin_native(false, Instant::now())
            .map_err(failure_from_stop)?;
        let mut native = Admission {
            ledger,
            native: true,
            cumulative: Reservation::default(),
        };
        constructed = Some(resultant::build(&source, &mut native)?);
        let mut checking = Admission {
            ledger: native.ledger,
            native: false,
            cumulative: Reservation::default(),
        };
        // Retain source-derived evidence before independent checking so its
        // refusal does not discard the completed builder result.
        evidence = Some(resultant::evidence(
            constructed.as_ref().unwrap(),
            &mut checking,
        )?);
        resultant::verify_evidence(&source, evidence.as_ref().unwrap(), &mut checking)?;
        verified = true;
        checking
            .ledger
            .finish_native(Instant::now())
            .map_err(failure_from_stop)?;
        Ok(())
    })();
    let failure = operation.err();
    if let Some(f) = failure {
        note_failure(ledger, f);
    }
    let artifact = ConstructionArtifact {
        schema: SCHEMA.into(),
        original_public_sha256: token.public_sha256,
        original_factor_base_sha256: token.factor_base_sha256,
        original_config_sha256: binding.config_sha256.clone(),
        binding_sha256,
        original_limits: ledger.original_limits(),
        before,
        after: ledger.saved_ledger(),
        failure,
        determinant_verified: verified,
        evidence,
    };
    let encoding_failed = serde_json::to_writer(Capped(&mut bytes), &artifact).is_err();
    if encoding_failed {
        let _: Result<(), _> = ledger.fail(Stop::Serialization);
    }
    Ok(Retained {
        bytes,
        constructed,
        failure,
        encoding_failed,
    })
}
/// Independently verify retained algebra against a separately authenticated
/// original token. Replay uses the caller's admitted checking ledger; reported
/// counters/clock/producer completion are not independently certified here.
pub(super) fn verify_artifact(
    token: &VerifiedWorkerOriginal<'_>,
    bytes: &[u8],
    ledger: &mut Budget,
) -> Result<bool, Failure> {
    ledger
        .reserve_checking(32 * CAP as u64, 8 * CAP as u64, Instant::now())
        .map_err(failure_from_stop)?;
    if bytes.len() > CAP {
        return Err(Failure::Input);
    }
    let saved: ConstructionArtifact = serde_json::from_slice(bytes).map_err(|_| Failure::Input)?;
    let binding = token.binding.ok_or(Failure::Input)?;
    if saved.schema != SCHEMA
        || saved.original_public_sha256 != token.public_sha256
        || saved.original_factor_base_sha256 != token.factor_base_sha256
        || saved.original_config_sha256 != binding.config_sha256
        || saved.binding_sha256 != binding.digest().map_err(|_| Failure::Input)?
        || saved.original_limits != ledger.original_limits()
    {
        return Err(Failure::Mismatch);
    }
    let math = decode_verified_profile_math(token, ledger, Profile::OrdinaryBinary)
        .map_err(failure_from_stop)?;
    let values = elements(&math);
    if let Some(evidence) = saved.evidence {
        let mut checking = Admission {
            ledger,
            native: false,
            cumulative: Reservation::default(),
        };
        resultant::verify_evidence(&input(&math, &values), &evidence, &mut checking)?;
        Ok(true)
    } else if saved.determinant_verified {
        Err(Failure::Mismatch)
    } else {
        Ok(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ml_dataset::{execution, solvers, storage};
    #[test]
    fn ordinary_original_bound_artifact_replays_actual_b_and_rejects_mutations() {
        for twist in [false, true] {
            let f = solvers::verified_worker_original_tests::Fixture::with_ordinary_runtime_shape(
                23, twist, "f4",
            );
            execution::create(&f.root).unwrap();
            let context = execution::admit(&f.root).unwrap().unwrap();
            let mut roots = Vec::new();
            let case = crate::ml_dataset::telemetry::measured(
                &f.root,
                &mut roots,
                "curve-construction",
                || solvers::curves::generate(&f.config),
            )
            .unwrap();
            let token = solvers::verify_and_store_worker_originals(
                &f.config,
                &case,
                &f.root,
                Some(&f.binding),
                &mut roots,
            )
            .unwrap();
            token
                .matches_bound(&f.config, &case, &f.root.join("live"), &f.binding)
                .unwrap();
            let now = Instant::now();
            let mut ledger = Budget::new(
                f.config.work_limit,
                context.launch.memory_bytes,
                context.launch.trace_bytes,
                context.deadline,
                now,
            )
            .unwrap();
            let retained = construct(&token, &context, &mut ledger).unwrap();
            assert!(!retained.encoding_failed);
            assert_eq!(retained.failure, None);
            assert!(retained.constructed.is_some());
            assert_eq!(ledger.phase, Phase::Constructed);
            let saved: ConstructionArtifact = serde_json::from_slice(&retained.bytes).unwrap();
            use num_traits::ToPrimitive;
            let independent = solvers::binary_m3_original::authenticate(
                Profile::OrdinaryBinary,
                &f.config,
                &case,
            )
            .unwrap();
            assert_eq!(
                saved.evidence.as_ref().unwrap().b,
                independent.curve.b.to_biguint().to_u64().unwrap()
            );
            assert_ne!(saved.evidence.as_ref().unwrap().b, 1);
            let before = ledger.checking_work;
            assert!(verify_artifact(&token, &retained.bytes, &mut ledger).unwrap());
            assert!(ledger.checking_work > before);
            for mutation in 0..5 {
                let mut changed: ConstructionArtifact =
                    serde_json::from_slice(&retained.bytes).unwrap();
                match mutation {
                    0 => changed.original_factor_base_sha256[0] ^= 1,
                    1 => changed.evidence.as_mut().unwrap().b ^= 1,
                    2 => changed.evidence.as_mut().unwrap().target_x ^= 1,
                    3 => changed.evidence.as_mut().unwrap().coordinate_anfs[0] ^= 1,
                    _ => changed.evidence.as_mut().unwrap().codes.swap(0, 1),
                }
                assert!(verify_artifact(
                    &token,
                    &serde_json::to_vec(&changed).unwrap(),
                    &mut ledger
                )
                .is_err());
            }
            // An actual expired replay admission keeps its Deadline class;
            // no parse or original decode happens after this first refusal.
            ledger.deadline = Instant::now() - Duration::from_secs(1);
            let before_expiry = ledger.saved_ledger();
            assert_eq!(
                verify_artifact(&token, &retained.bytes, &mut ledger),
                Err(Failure::Deadline)
            );
            assert_eq!(ledger.stopped, Some(Stop::Deadline));
            assert_eq!(ledger.native, before_expiry.native_reserved);
            assert_eq!(ledger.checking_work, before_expiry.checking_reserved_work);
            storage::verify_manifest(&f.root, &roots).unwrap();
        }
    }
    #[test]
    fn incremental_bridge_never_refunds_or_renews_and_checker_shares_work() {
        let now = Instant::now();
        let mut ledger =
            Budget::new(10, 1 << 20, 1 << 20, now + Duration::from_secs(10), now).unwrap();
        let absolute = ledger.deadline;
        let construction = ledger.construction_deadline;
        ledger.begin_native(false, now).unwrap();
        {
            let mut a = Admission {
                ledger: &mut ledger,
                native: true,
                cumulative: Reservation::default(),
            };
            a.reserve(Quote {
                work: 1000,
                bytes: 64,
            })
            .unwrap();
            a.reserve(Quote {
                work: 2000,
                bytes: 64,
            })
            .unwrap();
            assert_eq!(a.ledger.native.work, 3000);
            assert_eq!(a.ledger.native.bytes, 128);
        }
        {
            let mut a = Admission {
                ledger: &mut ledger,
                native: false,
                cumulative: Reservation::default(),
            };
            a.reserve(Quote {
                work: 7240,
                bytes: 64,
            })
            .unwrap();
            assert_eq!(
                a.reserve(Quote { work: 1, bytes: 0 }),
                Err(Failure::Admission)
            );
            assert_eq!(a.ledger.stopped, Some(Stop::Work));
            assert_eq!(a.ledger.checking_work, 7240);
            assert_eq!(a.check_deadline(), Err(Failure::Admission));
        }
        assert_eq!(ledger.native.work, 3000);
        assert_eq!(ledger.deadline, absolute);
        assert_eq!(ledger.construction_deadline, construction);
    }
    #[test]
    fn original_shared_work_refusals_retain_partial_artifact_and_builder() {
        for remaining in [400_000, 900_000] {
            let f = solvers::verified_worker_original_tests::Fixture::with_ordinary_runtime_shape(
                23, false, "f4",
            );
            execution::create(&f.root).unwrap();
            let context = execution::admit(&f.root).unwrap().unwrap();
            let mut roots = Vec::new();
            let case = crate::ml_dataset::telemetry::measured(
                &f.root,
                &mut roots,
                "curve-construction",
                || solvers::curves::generate(&f.config),
            )
            .unwrap();
            let token = solvers::verify_and_store_worker_originals(
                &f.config,
                &case,
                &f.root,
                Some(&f.binding),
                &mut roots,
            )
            .unwrap();
            token
                .matches_bound(&f.config, &case, &f.root.join("live"), &f.binding)
                .unwrap();
            let now = Instant::now();
            let mut ledger = Budget::new(
                f.config.work_limit,
                context.launch.memory_bytes,
                context.launch.trace_bytes,
                context.deadline,
                now,
            )
            .unwrap();
            ledger
                .reserve_checking(ledger.native_work_limit - remaining, 0, now)
                .unwrap();
            let spent = ledger.checking_work;
            let retained = construct(&token, &context, &mut ledger).unwrap();
            assert!(!retained.encoding_failed);
            assert_eq!(retained.failure, Some(Failure::Admission));
            assert_eq!(ledger.stopped, Some(Stop::Work));
            assert!(ledger.checking_work >= spent);
            assert_eq!(ledger.deadline, context.deadline);
            assert_eq!(retained.constructed.is_some(), remaining == 900_000);
            let saved: ConstructionArtifact = serde_json::from_slice(&retained.bytes).unwrap();
            assert_eq!(saved.after.first_stop, Some(Stop::Work));
            assert!(!saved.determinant_verified);
            assert_eq!(saved.evidence.is_some(), remaining == 900_000);
        }
    }
    #[test]
    fn bridge_error_class_and_sticky_first_stop_are_preserved() {
        let now = Instant::now();
        for (stop, expected) in [
            (Stop::Deadline, Failure::Deadline),
            (Stop::Allocation, Failure::Allocation),
            (Stop::InvalidLimits, Failure::Input),
            (Stop::Work, Failure::Admission),
            (Stop::Bytes, Failure::Admission),
            (Stop::Phase, Failure::Admission),
            (Stop::Overflow, Failure::Admission),
            (Stop::Nonmonotone, Failure::Admission),
            (Stop::Trace, Failure::Admission),
            (Stop::Additions, Failure::Admission),
            (Stop::Observer, Failure::Admission),
            (Stop::NativeFailure, Failure::Admission),
            (Stop::Serialization, Failure::Admission),
            (Stop::Events, Failure::Admission),
        ] {
            assert_eq!(failure_from_stop(stop), expected);
            for native in [false, true] {
                let mut ledger =
                    Budget::new(10, 1 << 20, 1 << 20, now + Duration::from_secs(10), now).unwrap();
                ledger.reserve_checking(77, 64, now).unwrap();
                if native {
                    ledger.begin_native(false, now).unwrap();
                }
                let _: Result<(), Stop> = ledger.fail(stop);
                let before = ledger.saved_ledger();
                let mut admission = Admission {
                    ledger: &mut ledger,
                    native,
                    cumulative: Reservation::default(),
                };
                assert_eq!(
                    admission.reserve(Quote { work: 1, bytes: 0 }),
                    Err(expected)
                );
                assert_eq!(admission.check_deadline(), Err(expected));
                note_failure(admission.ledger, expected);
                assert_eq!(admission.ledger.saved_ledger(), before);
                assert_eq!(admission.cumulative, Reservation::default());
                // A later, different failure cannot replace the original stop.
                note_failure(admission.ledger, Failure::Mismatch);
                assert_eq!(admission.ledger.saved_ledger(), before);
            }
        }
    }
}
