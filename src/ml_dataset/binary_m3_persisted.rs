//! Ordinary input persistence prerequisite; no worker dispatch is activated.
#[path = "binary_m3_input_owner.rs"]
mod input_owner;
use input_owner::InputBytes;
// Child of ordinary_construction so only this module can mint the token.
use super::*;
#[path = "binary_m3_native.rs"]
pub(in crate::ml_dataset::solvers) mod native_ownership;
use crate::ml_dataset::{capture, execution, solvers, storage, types};

const INPUT_PATH: &str = "binary-m3-input.json";
const INPUT_ROLE: &str = "restricted-binary-m3-original-equations/v1";
const INPUT_SCHEMA: &str = "crypto.ml-dataset.binary-m3-input/v1";
const INPUT_CAP: usize = 64 * 1024;

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct InputArtifact {
    schema: String,
    binding: capture::AttemptBinding,
    original_public_sha256: [u8; 32],
    original_factor_base_sha256: [u8; 32],
    original_limits: OriginalLimits,
    n: u32,
    twist: bool,
    a: u64,
    b: u64,
    low_terms: Vec<u32>,
    target: (u64, u64),
    codes: [u64; 2],
    signed: [(u64, u64); 4],
    // Retain exact construction bytes, not a regenerated JSON identity.
    construction_sha256: String,
    construction_json: String,
}

/// Root and capture acknowledgement are actual side effects, never callbacks
/// returning invented booleans. Same mutable ledger remains exclusively held.
pub(super) struct PersistedOrdinaryInput<'a> {
    retained: Retained,
    bytes: InputBytes,
    root_artifact: types::Artifact,
    capture_sequence: u64,
    math: MathInput,
    ledger: &'a mut Budget,
}
impl PersistedOrdinaryInput<'_> {
    pub(super) fn equations(&self) -> &[crate::cryptanalysis::pq_groebner_f2::F2BoolPoly] {
        &self
            .retained
            .constructed
            .as_ref()
            .expect("minted completed input")
            .equations
    }
    // A later matrix adapter must consume this owner, retain root/bytes and
    // borrow ledger. No public constructor or detached fresh allowance exists.
}

/// Retains the actual persisted input, capture acknowledgement, root receipt
/// and original ledger on completion and refusal. No detached receipt API is
/// exposed before the ordinary retained-owner registry is implemented.
pub(super) struct OrdinaryMatrixStep<'a> {
    input: PersistedOrdinaryInput<'a>,
    report: Option<crate::cryptanalysis::koblitz_groebner::bounded_matrix::Report>,
    adapter_stop: Option<Stop>,
}

pub(super) fn matrix_step<'a, F>(
    input: PersistedOrdinaryInput<'a>,
    engine: crate::cryptanalysis::koblitz_groebner::bounded_matrix::Engine,
    degree: u32,
    forward: &mut F,
) -> OrdinaryMatrixStep<'a>
where
    F: FnMut(
        &crate::cryptanalysis::koblitz_groebner::bounded_matrix::Event,
        &mut Budget,
    ) -> Result<(), ()>,
{
    // Only persistence can mint this owner after actual bound capture/root
    // storage and independent replay. Its finite selector has three variables,
    // regardless of the authenticated original field degree.
    let equations = &input
        .retained
        .constructed
        .as_ref()
        .expect("minted completed ordinary input")
        .equations;
    let (report, adapter_stop) =
        super::super::matrix_equations(equations, 3, input.ledger, engine, degree, forward);
    OrdinaryMatrixStep {
        input,
        report,
        adapter_stop,
    }
}

pub(super) struct PersistenceFailure<'a> {
    // Retain the actual exclusive origin borrow, so identity cannot outlive or
    // be confused with a moved/replaced Budget after a persistence refusal.
    ledger: &'a mut Budget,
    pub retained: Retained,
    pub input_bytes: InputBytes,
    pub encoding_complete: bool,
    pub capture_sequence: Option<u64>,
    pub root_artifact: Option<types::Artifact>,
    pub stop: Option<Stop>,
    pub error: String,
}

struct InputWriter<'a>(&'a mut Vec<u8>);
impl std::io::Write for InputWriter<'_> {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        if bytes.len() > INPUT_CAP.saturating_sub(self.0.len()) {
            return Err(std::io::Error::other("ordinary input encoding cap"));
        }
        self.0.extend_from_slice(bytes);
        Ok(bytes.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

// A separately prepaid bounded counting pass avoids charging the maximum
// encoding size when the complete serializable value is already available.
struct InputCount(usize);
impl std::io::Write for InputCount {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        if bytes.len() > INPUT_CAP.saturating_sub(self.0) {
            return Err(std::io::Error::other("ordinary input counting cap"));
        }
        self.0 += bytes.len();
        Ok(bytes.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

/// Record quote failures before generic observer handling; each accepted
/// reservation remains charged if a subsequent reservation refuses.
fn admit_storage(
    ledger: &mut Budget,
    len: u64,
    limits: &capture::CaptureLimits,
) -> Result<(), Stop> {
    let quote =
        quote_parent_storage_length(len, false, limits).or_else(|stop| ledger.fail(stop))?;
    ledger.reserve_checking(quote.work, quote.bytes, Instant::now())?;
    ledger.reserve_trace(quote.trace, Instant::now())
}

/// Caller passes the bridge's original retained owner and actual attempt ledger.
/// Full mathematical verification is performed again under charged checking
/// admission before any capture/root publication; no retained bytes are trusted.
pub(super) fn persist<'a>(
    token: &VerifiedWorkerOriginal<'_>,
    case: &types::CaseData,
    context: &execution::SolverExecutionContext,
    limits: &capture::CaptureLimits,
    retained: Retained,
    ledger: &'a mut Budget,
) -> Result<PersistedOrdinaryInput<'a>, PersistenceFailure<'a>> {
    let mut bytes = InputBytes::empty();
    let mut encoding_complete = false;
    let mut capture_sequence = None;
    let mut root_artifact = None;
    let result = (|| -> Result<MathInput, String> {
        ledger
            .check(Instant::now())
            .map_err(|s| format!("ordinary input: {s:?}"))?;
        if ledger.phase != Phase::Constructed
            || retained.failure.is_some()
            || retained.encoding_failed
            || retained.constructed.is_none()
            || token.binding != Some(&context.launch.binding)
            || ledger.deadline != context.deadline
            || ledger.original_limits()
                != OriginalLimits::from_original(token.config, &context.launch)?
        {
            return Err("ordinary input requires completed original construction".into());
        }
        // The installed sink, not a caller-manufactured path argument, must
        // own this exact original live directory before any publication.
        solvers::LIVE_SINK.with(|slot| match slot.borrow().as_ref() {
            Some(solvers::EventSink::Bound(sink)) => {
                sink.verify_directory(&token.root.join("live"))
            }
            _ => Err("ordinary input lacks original capture directory".into()),
        })?;
        // Rechecking original disk identity is charged to this same ledger.
        if token.public_bytes > 64 * 1024 || token.factor_base_bytes > 128 * 1024 {
            let _: Result<(), _> = ledger.fail(Stop::InvalidLimits);
            return Err("ordinary original byte bounds".into());
        }
        ledger
            .reserve_checking(
                65536 + 32 * (token.public_bytes + token.factor_base_bytes),
                64 * 1024,
                Instant::now(),
            )
            .map_err(|s| format!("ordinary original readback: {s:?}"))?;
        // matches_bound requires the actual borrowed CaseData, not a clone.
        token.matches_bound(
            token.config,
            case,
            &token.root.join("live"),
            &context.launch.binding,
        )?;
        // This is an additional charged decoder/replay, never a free duplicate
        // and never a new deadline. Future ownership extraction may remove it.
        ledger
            .reserve_checking(0, (3 * INPUT_CAP + CAP) as u64, Instant::now())
            .map_err(|s| format!("ordinary input encoding admission: {s:?}"))?;
        if !verify_artifact(token, &retained.bytes, ledger).map_err(|e| {
            note_failure(ledger, e);
            format!("ordinary construction replay: {e:?}")
        })? {
            return Err("ordinary input lacks verified algebra".into());
        }
        let math = decode_verified_profile_math(token, ledger, Profile::OrdinaryBinary)
            .map_err(|s| format!("ordinary original decode: {s:?}"))?;
        let values = elements(&math);
        let mut checking = Admission {
            ledger,
            native: false,
            cumulative: Reservation::default(),
        };
        resultant::verify(
            &input(&math, &values),
            retained.constructed.as_ref().unwrap(),
            &mut checking,
        )
        .map_err(|f| {
            note_failure(checking.ledger, f);
            format!("ordinary owned equations: {f:?}")
        })?;
        use num_traits::ToPrimitive;
        let input = InputArtifact {
            schema: INPUT_SCHEMA.into(),
            binding: context.launch.binding.clone(),
            original_public_sha256: token.public_sha256,
            original_factor_base_sha256: token.factor_base_sha256,
            original_limits: ledger.original_limits(),
            n: math.curve.m,
            twist: token.config.twist,
            a: math
                .curve
                .a
                .to_biguint()
                .to_u64()
                .ok_or("ordinary a exceeds word")?,
            b: math
                .curve
                .b
                .to_biguint()
                .to_u64()
                .ok_or("ordinary b exceeds word")?,
            low_terms: math.curve.irreducible.low_terms.clone(),
            target: math.target,
            codes: math.codes,
            signed: math.signed,
            construction_sha256: crate::ml_dataset::digest(&retained.bytes),
            construction_json: String::from_utf8(retained.bytes.clone())
                .map_err(|_| "construction not UTF8")?,
        };
        bytes.encode(&input, ledger)?;
        encoding_complete = true;
        admit_storage(ledger, bytes.len() as u64, limits)
            .map_err(|s| format!("ordinary input storage admission: {s:?}"))?;
        let binding_hash = context.launch.binding.digest()?;
        let mut digest = [0u8; 64];
        digest.copy_from_slice(binding_hash.as_bytes());
        let count = solvers::LIVE_SINK.with(|slot| match slot.borrow().as_ref() {
            Some(solvers::EventSink::Bound(sink)) => {
                sink.verify_directory(&token.root.join("live"))?;
                sink.verify_attempt_limits(&context.launch.binding, limits)?;
                Ok(sink.committed_event_count())
            }
            _ => Err("ordinary input lacks bound capture".to_owned()),
        })?;
        let next = count
            .checked_add(1)
            .ok_or("ordinary capture sequence overflow")?;
        solvers::live_borrowed_payload(INPUT_PATH, INPUT_ROLE, "json", &bytes)?;
        solvers::LIVE_SINK.with(|slot| match slot.borrow().as_ref() {
            Some(solvers::EventSink::Bound(sink)) => {
                sink.verify_directory(&token.root.join("live"))?;
                sink.verify_digest_limits_at_count(&digest, limits, next)
            }
            _ => Err("ordinary capture disappeared".into()),
        })?;
        capture_sequence = Some(next);
        ledger
            .check(Instant::now())
            .map_err(|s| format!("ordinary root deadline: {s:?}"))?;
        root_artifact = Some(storage::write_bytes(
            token.root, INPUT_PATH, INPUT_ROLE, "json", &bytes,
        )?);
        ledger
            .check(Instant::now())
            .map_err(|s| format!("ordinary input stored after deadline: {s:?}"))?;
        Ok(math)
    })();
    match result {
        Ok(math) => Ok(PersistedOrdinaryInput {
            retained,
            bytes,
            root_artifact: root_artifact.expect("successful root write"),
            capture_sequence: capture_sequence.expect("successful new capture"),
            math,
            ledger,
        }),
        Err(error) => {
            if ledger.stopped.is_none() {
                let _: Result<(), _> = ledger.fail(Stop::Observer);
            }
            Err(PersistenceFailure {
                retained,
                input_bytes: bytes,
                encoding_complete,
                capture_sequence,
                root_artifact,
                stop: ledger.stopped,
                error,
                ledger,
            })
        }
    }
}

/// Central mathematical input replay. Caller authenticates manifest/capture and
/// supplies original bytes/config/binding; no producer receipt mints a token.
/// This verifies actual algebra and source identity, not stored clock history.
pub(super) fn verify_input(
    original_config: &[u8],
    original_public: &[u8],
    original_factor_base: &[u8],
    binding: &capture::AttemptBinding,
    limits: OriginalLimits,
    bytes: &[u8],
    admission: &mut impl OriginalAdmission,
) -> Result<(), Failure> {
    admission.reserve(Quote {
        work: 32 * (INPUT_CAP as u64 + 256 * 1024),
        bytes: 2 * 1024 * 1024,
    })?;
    admission.check_deadline()?;
    if bytes.len() > INPUT_CAP
        || original_config.len() > 64 * 1024
        || original_public.len() > 64 * 1024
        || original_factor_base.len() > 128 * 1024
    {
        return Err(Failure::Input);
    }
    if crate::ml_dataset::digest(original_config) != binding.config_sha256 {
        return Err(Failure::Mismatch);
    }
    let config: types::CaseConfig =
        serde_json::from_slice(original_config).map_err(|_| Failure::Input)?;
    let case = types::CaseData {
        public_instance: serde_json::from_slice(original_public).map_err(|_| Failure::Input)?,
        factor_base: serde_json::from_slice(original_factor_base).map_err(|_| Failure::Input)?,
        private_witness: serde_json::Value::Null,
    };
    use sha2::Digest;
    let original_public_sha256: [u8; 32] = sha2::Sha256::digest(original_public).into();
    let original_factor_base_sha256: [u8; 32] = sha2::Sha256::digest(original_factor_base).into();
    let saved: InputArtifact = serde_json::from_slice(bytes).map_err(|_| Failure::Input)?;
    if saved.schema != INPUT_SCHEMA
        || &saved.binding != binding
        || saved.original_limits != limits
        || saved.original_public_sha256 != original_public_sha256
        || saved.original_factor_base_sha256 != original_factor_base_sha256
        || saved.construction_json.len() > CAP
        || crate::ml_dataset::digest(saved.construction_json.as_bytes())
            != saved.construction_sha256
    {
        return Err(Failure::Mismatch);
    }
    // Historical source authentication uses existing independent group/domain
    // checker; caller supplies its already admitted original-verification work.
    let math = solvers::binary_m3_original::authenticate(Profile::OrdinaryBinary, &config, &case)
        .map_err(|_| Failure::Input)?;
    use num_traits::ToPrimitive;
    if saved.n != math.curve.m
        || saved.twist != config.twist
        || Some(saved.a) != math.curve.a.to_biguint().to_u64()
        || Some(saved.b) != math.curve.b.to_biguint().to_u64()
        || saved.low_terms != math.curve.irreducible.low_terms
        || saved.target != math.target
        || saved.codes != math.codes
        || saved.signed != math.signed
    {
        return Err(Failure::Mismatch);
    }
    let construction: ConstructionArtifact =
        serde_json::from_str(&saved.construction_json).map_err(|_| Failure::Input)?;
    if construction.schema != SCHEMA
        || construction.original_public_sha256 != original_public_sha256
        || construction.original_factor_base_sha256 != original_factor_base_sha256
        || construction.original_config_sha256 != binding.config_sha256
        || construction.binding_sha256 != binding.digest().map_err(|_| Failure::Input)?
        || construction.original_limits != limits
        || construction.failure.is_some()
        || !construction.determinant_verified
    {
        return Err(Failure::Mismatch);
    }
    let values = elements(&math);
    resultant::verify_evidence(
        &input(&math, &values),
        construction.evidence.as_ref().ok_or(Failure::Input)?,
        admission,
    )?;
    admission.check_deadline()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[derive(Default)]
    struct ReplayAdmission {
        work: u64,
        bytes: u64,
    }
    impl OriginalAdmission for ReplayAdmission {
        fn reserve(&mut self, q: Quote) -> Result<(), Failure> {
            self.work = self.work.checked_add(q.work).ok_or(Failure::Admission)?;
            self.bytes = self.bytes.checked_add(q.bytes).ok_or(Failure::Admission)?;
            Ok(())
        }
        fn check_deadline(&mut self) -> Result<(), Failure> {
            Ok(())
        }
    }

    #[test]
    fn storage_quote_and_refusals_keep_typed_first_stop_and_charges() {
        for mode in 0..7 {
            let now = Instant::now();
            let mut ledger = Budget::new(
                16384,
                256 << 20,
                128 << 20,
                now + Duration::from_secs(30),
                now,
            )
            .unwrap();
            ledger.reserve_checking(77, 64, now).unwrap();
            let mut limits = capture::capture_limits(16).unwrap();
            let mut len = 100;
            let expected = match mode {
                0 => {
                    limits.max_capture_bytes = 0;
                    Stop::InvalidLimits
                }
                1 => {
                    len = u64::MAX;
                    Stop::Overflow
                }
                2 => {
                    ledger.checking_work = ledger.native_work_limit;
                    Stop::Work
                }
                3 => {
                    ledger.adapter_bytes = ledger.memory_limit;
                    Stop::Bytes
                }
                4 => {
                    ledger.trace_bytes = ledger.trace_limit;
                    Stop::Trace
                }
                5 => {
                    ledger.construction_deadline = now - Duration::from_secs(1);
                    Stop::Deadline
                }
                _ => {
                    let _: Result<(), _> = ledger.fail(Stop::Work);
                    limits.max_capture_bytes = 0;
                    Stop::Work
                }
            };
            let before = ledger.saved_ledger();
            assert_eq!(admit_storage(&mut ledger, len, &limits), Err(expected));
            assert_eq!(ledger.stopped, Some(expected));
            assert_eq!(ledger.native, before.native_reserved);
            assert_eq!(ledger.trace_bytes, before.charged_trace_bytes);
            if mode == 4 {
                let quote = quote_parent_storage_length(len, false, &limits).unwrap();
                assert_eq!(
                    ledger.checking_work,
                    before.checking_reserved_work + quote.work
                );
                assert_eq!(
                    ledger.adapter_bytes,
                    before.adapter_reserved_bytes + quote.bytes
                );
            } else {
                assert_eq!(ledger.checking_work, before.checking_reserved_work);
                assert_eq!(ledger.adapter_bytes, before.adapter_reserved_bytes);
            }
            let retained = ledger.saved_ledger();
            let _: Result<(), _> = ledger.fail(Stop::Observer);
            assert_eq!(ledger.saved_ledger(), retained);
        }
    }

    #[test]
    fn ordinary_persisted_originals_both_twists_endpoints_and_root_refusal() {
        for n in [23, 61] {
            for twist in [false, true] {
                for mode in 0..6 {
                    let collision = mode == 1;
                    let wrong_directory = mode == 2;
                    let f=solvers::verified_worker_original_tests::Fixture::with_ordinary_runtime_shape(n,twist,"f4");
                    execution::create(&f.root).unwrap();
                    let context = execution::admit(&f.root).unwrap().unwrap();
                    let (binding, capture_limits) =
                        crate::ml_dataset::admit_capture_context(&f.root)
                            .unwrap()
                            .unwrap();
                    let mut roots = Vec::new();
                    let case = crate::ml_dataset::telemetry::measured(
                        &f.root,
                        &mut roots,
                        "curve-construction",
                        || solvers::curves::generate(&f.config),
                    )
                    .unwrap();
                    let original = solvers::verify_and_store_worker_originals(
                        &f.config,
                        &case,
                        &f.root,
                        Some(&binding),
                        &mut roots,
                    )
                    .unwrap();
                    original
                        .matches_bound(&f.config, &case, &f.root.join("live"), &binding)
                        .unwrap();
                    std::fs::create_dir_all(f.root.join("live")).unwrap();
                    let sink_dir = f.root.join(if wrong_directory {
                        "other-live"
                    } else {
                        "live"
                    });
                    let sink = capture::CaptureEmitter::create(
                        &sink_dir,
                        binding.clone(),
                        capture_limits.clone(),
                    )
                    .unwrap();
                    assert!(solvers::LIVE_SINK.with(|s| s.borrow().is_none()));
                    solvers::LIVE_SINK
                        .with(|s| *s.borrow_mut() = Some(solvers::EventSink::Bound(sink)));
                    let _guard = solvers::LiveSinkGuard;
                    let now = Instant::now();
                    let mut ledger = Budget::new(
                        f.config.work_limit,
                        context.launch.memory_bytes,
                        context.launch.trace_bytes,
                        context.deadline,
                        now,
                    )
                    .unwrap();
                    let admitted_registry = native_ownership::OrdinaryRootRegistry::admit(
                        &original,
                        &mut ledger,
                        &capture_limits,
                    );
                    let mut registry = if wrong_directory {
                        assert!(admitted_registry.is_err());
                        None
                    } else {
                        Some(admitted_registry.unwrap())
                    };
                    let retained = construct(&original, &context, &mut ledger).unwrap();
                    assert!(retained.failure.is_none());
                    assert!(!retained.encoding_failed);
                    let input_limits = ledger.original_limits();
                    let deadline = ledger.deadline;
                    if collision {
                        std::fs::write(f.root.join(INPUT_PATH), b"original conflicting root")
                            .unwrap();
                    }
                    match persist(
                        &original,
                        &case,
                        &context,
                        &capture_limits,
                        retained,
                        &mut ledger,
                    ) {
                        Err(failed) => {
                            assert!(
                                collision || wrong_directory,
                                "unexpected original refusal n={n},twist={twist}: {}",
                                failed.error
                            );
                            if wrong_directory {
                                assert!(!failed.encoding_complete);
                                assert!(failed.input_bytes.is_empty());
                                assert!(failed.capture_sequence.is_none());
                                assert!(failed.root_artifact.is_none());
                                assert!(!f.root.join(INPUT_PATH).exists());
                                assert!(!sink_dir.join("payloads").join(INPUT_PATH).exists());
                                assert_eq!(
                                    solvers::LIVE_SINK.with(|s| {
                                        match s.borrow().as_ref().unwrap() {
                                            solvers::EventSink::Bound(sink) => {
                                                sink.committed_event_count()
                                            }
                                            _ => panic!("wrong sink"),
                                        }
                                    }),
                                    0
                                );
                                continue;
                            }
                            assert!(failed.encoding_complete);
                            assert!(failed.capture_sequence.is_some());
                            assert!(failed.root_artifact.is_none());
                            assert!(failed.retained.constructed.is_some());
                            assert_eq!(failed.stop, Some(Stop::Observer));
                            assert_eq!(
                                std::fs::read(f.root.join(INPUT_PATH)).unwrap(),
                                b"original conflicting root"
                            );
                        }
                        Ok(saved) => {
                            let now = Instant::now();
                            let other = Budget::new(
                                f.config.work_limit,
                                saved.ledger.memory_limit,
                                saved.ledger.trace_limit,
                                saved.ledger.deadline,
                                now,
                            )
                            .unwrap();
                            assert!(saved.bytes.matches(&original, &other).is_err());
                            assert!(saved.bytes.matches(&original, saved.ledger).is_ok());
                            let moved_other = other;
                            assert!(saved.bytes.matches(&original, &moved_other).is_err());

                            assert!(!collision && !wrong_directory);
                            assert_eq!(saved.ledger.deadline, deadline);
                            assert_eq!(saved.equations().len(), n as usize);
                            assert_eq!(saved.math.curve.m, n);
                            assert_eq!(saved.capture_sequence, 1);
                            let input: InputArtifact =
                                serde_json::from_slice(&saved.bytes).unwrap();
                            assert_ne!(input.b, 0);
                            assert_ne!(input.b, 1);
                            assert_eq!(input.twist, twist);
                            assert_eq!(saved.root_artifact.path, INPUT_PATH);
                            assert_eq!(saved.root_artifact.role, INPUT_ROLE);
                            roots.push(saved.root_artifact.clone());
                            storage::verify_manifest(&f.root, &roots).unwrap();
                            let config = std::fs::read(f.root.join("config.json")).unwrap();
                            let public = std::fs::read(f.root.join("public.json")).unwrap();
                            let base = std::fs::read(f.root.join("factor-base.json")).unwrap();
                            verify_input(
                                &config,
                                &public,
                                &base,
                                &binding,
                                input_limits,
                                &saved.bytes,
                                &mut ReplayAdmission::default(),
                            )
                            .unwrap();
                            for mutation in 0..7 {
                                let mut changed: InputArtifact =
                                    serde_json::from_slice(&saved.bytes).unwrap();
                                match mutation {
                                    0 => changed.b ^= 1,
                                    1 => changed.a ^= 1,
                                    2 => changed.twist = !changed.twist,
                                    3 => changed.original_public_sha256[0] ^= 1,
                                    4 => changed.original_factor_base_sha256[0] ^= 1,
                                    5 => changed.binding.config_sha256 = "0".repeat(64),
                                    _ => {
                                        changed.schema =
                                            "crypto.ml-dataset.koblitz-m3-input/v1".into()
                                    }
                                }
                                assert!(verify_input(
                                    &config,
                                    &public,
                                    &base,
                                    &binding,
                                    input_limits,
                                    &serde_json::to_vec(&changed).unwrap(),
                                    &mut ReplayAdmission::default()
                                )
                                .is_err());
                            }
                            let mut changed_config: serde_json::Value =
                                serde_json::from_slice(&config).unwrap();
                            changed_config["work_limit"] = serde_json::json!(1);
                            assert!(verify_input(
                                &serde_json::to_vec(&changed_config).unwrap(),
                                &public,
                                &base,
                                &binding,
                                input_limits,
                                &saved.bytes,
                                &mut ReplayAdmission::default()
                            )
                            .is_err());
                            check_matrix_ownership(
                                saved,
                                n,
                                mode,
                                registry.as_mut().unwrap(),
                                &capture_limits,
                            );
                        }
                    }
                }
            }
        }
    }

    fn check_matrix_ownership<'o>(
        mut saved: PersistedOrdinaryInput<'_>,
        n: u32,
        mode: usize,
        registry: &mut native_ownership::OrdinaryRootRegistry<'o>,
        limits: &'o capture::CaptureLimits,
    ) {
        use crate::cryptanalysis::koblitz_groebner::bounded_matrix as native;
        let input_ptr = saved.bytes.as_ptr();
        let equations_ptr = saved.equations().as_ptr();
        let root_ptr = saved.root_artifact.path.as_ptr();
        let root_bytes = serde_json::to_vec(&saved.root_artifact).unwrap();
        let input_bytes = saved.bytes.clone();
        let capture_sequence = saved.capture_sequence;
        let deadline = saved.ledger.deadline;
        let original_limits = saved.ledger.original_limits();
        let original_math =
            serde_json::to_vec(&saved.retained.constructed.as_ref().unwrap().equations).unwrap();
        // Same bytes and attempt on a different real ledger cannot supply the
        // sealed identity. Rejection occurs before charging the recipient.
        let now = Instant::now();
        let mut other = Budget::new(
            registry.original.config.work_limit,
            saved.ledger.memory_limit,
            saved.ledger.trace_limit,
            saved.ledger.deadline,
            now,
        )
        .unwrap();
        let artifact: InputArtifact = serde_json::from_slice(&saved.bytes).unwrap();
        let mut foreign = InputBytes::empty();
        foreign.encode(&artifact, &mut other).unwrap();
        let actual = std::mem::replace(&mut saved.bytes, foreign);
        let before = (saved.ledger.checking_work, saved.ledger.adapter_bytes);
        assert!(
            native_ownership::OrdinaryEvents::admit(registry.original, limits, &mut saved).is_err()
        );
        assert_eq!(
            (saved.ledger.checking_work, saved.ledger.adapter_bytes),
            before
        );
        saved.bytes = actual;
        let mut events =
            native_ownership::OrdinaryEvents::admit(registry.original, limits, &mut saved).unwrap();
        if mode == 3 {
            events.refuse_precommit_for_test();
        }
        let forced = match mode {
            4 => Some(Stop::Deadline),
            5 => Some(Stop::Work),
            _ => None,
        };
        if let Some(stop) = forced {
            let _ = saved.ledger.fail::<()>(stop);
        }
        let engine = if n == 23 {
            native::Engine::F4Macaulay
        } else {
            native::Engine::F5KoszulFrobenius
        };
        let mut callbacks = 0;
        let step = matrix_step(saved, engine, 3, &mut |event, ledger| {
            callbacks += 1;
            events.forward(event, ledger)
        });
        // Moving the owner never re-encodes its input, clones root custody,
        // rebuilds equations or resets the original deadline/limits.
        assert_eq!(step.input.bytes.as_ptr(), input_ptr);
        assert_eq!(step.input.equations().as_ptr(), equations_ptr);
        assert_eq!(step.input.root_artifact.path.as_ptr(), root_ptr);
        assert_eq!(
            serde_json::to_vec(&step.input.root_artifact).unwrap(),
            root_bytes
        );
        assert_eq!(step.input.bytes, input_bytes);
        assert_eq!(step.input.capture_sequence, capture_sequence);
        assert_eq!(step.input.ledger.deadline, deadline);
        assert_eq!(step.input.ledger.original_limits(), original_limits);
        assert_eq!(
            serde_json::to_vec(step.input.equations()).unwrap(),
            original_math
        );
        if let Some(stop) = forced {
            assert_eq!(step.adapter_stop, Some(stop));
            assert!(step.report.is_none());
            assert_eq!(callbacks, 0);
            assert_eq!(step.input.ledger.stopped, Some(stop));
        } else if mode == 3 {
            assert!(callbacks > 0);
            assert!(step.report.is_some());
            assert_eq!(step.adapter_stop, Some(Stop::Observer));
            assert_eq!(step.input.ledger.stopped, Some(Stop::Observer));
        } else {
            assert!(callbacks > 0);
            assert!(step.report.is_some());
            assert_eq!(step.adapter_stop, None);
            assert_eq!(step.input.ledger.phase, Phase::MatrixComplete);
        }
        native_ownership::test_transfer(registry, step, root_ptr, input_ptr, events);
    }
}

#[cfg(test)]
mod input_count_controls {
    use super::*;
    #[test]
    fn counting_uses_real_serde_escaping_and_preserves_original_cap() {
        let value = serde_json::json!({"construction_json":"\"\\\n\u{0000}é", "word":u64::MAX});
        let mut count = InputCount(0);
        serde_json::to_writer(&mut count, &value).unwrap();
        let mut bytes = Vec::new();
        serde_json::to_writer(InputWriter(&mut bytes), &value).unwrap();
        assert_eq!(count.0, bytes.len());
        let too_long = "x".repeat(INPUT_CAP);
        let mut capped = InputCount(0);
        assert!(serde_json::to_writer(&mut capped, &too_long).is_err());
        assert!(capped.0 <= INPUT_CAP);
        assert_eq!(INPUT_CAP, 65536);
    }
}
