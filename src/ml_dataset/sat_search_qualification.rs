//! Dedicated supervised qualification tests, absent from ordinary test builds.
//! No solve invocation and no native gate changes: this checks prerequisites.
use super::*;
use crate::ml_dataset::{self as dataset, curves, storage};
use provenance::qualification_reference::{self as reference, Reference};
use provenance::Admission;
#[path = "sat_qualification_builder.rs"]
mod builder;

struct BigWords<'a>(&'a [num_bigint::BigUint]);
impl serde::Serialize for BigWords<'_> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeSeq;
        struct Word<'a>(&'a num_bigint::BigUint);
        impl serde::Serialize for Word<'_> {
            fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
                let mut seq = s.serialize_seq(None)?;
                for digit in self.0.iter_u64_digits() {
                    seq.serialize_element(&digit)?;
                }
                seq.end()
            }
        }
        let mut seq = serializer.serialize_seq(Some(self.0.len()))?;
        for code in self.0 {
            seq.serialize_element(&Word(code))?;
        }
        seq.end()
    }
}
#[derive(Serialize)]
struct Record<T> {
    stage: &'static str,
    data: T,
}
#[derive(Serialize)]
struct MapView<'a> {
    original_id: u64,
    map: Option<&'a provenance::Map>,
    raw_map: Option<&'a provenance::Map>,
    pattern: Option<u32>,
    map_bytes: &'a [u8],
    map_ack: Option<&'a capture::CommittedEventPosition>,
    budget: String,
    error: Option<String>,
}
#[derive(Serialize)]
struct NativeView<'a> {
    stop: String,
    events: &'a [SolverEvent],
    pending: Option<&'a SolverEvent>,
    ack: Option<&'a capture::CommittedEventPosition>,
    budget: String,
}
#[derive(Serialize)]
struct TerminalView<'a> {
    qualified: bool,
    binding: [u8; 32],
    pattern: u32,
    initial_ack: Option<&'a capture::CommittedEventPosition>,
    native_decisions: u64,
    native_search_acceptance: bool,
    original_id: u64,
    budget: String,
    model_hashes: &'a [Option<String>; 3],
}
#[derive(Serialize)]
struct ReferenceLedger {
    work: u64,
    bytes: u64,
    additions: u64,
    #[serde(skip)]
    deadline: Instant,
    stopped: Option<&'static str>,
}
impl provenance::Admission for ReferenceLedger {
    fn reserve(&mut self, q: provenance::Quote) -> Result<(), provenance::Error> {
        self.check()?;
        let w = self
            .work
            .checked_add(q.work)
            .ok_or(provenance::Error::Cap)?;
        let b = self
            .bytes
            .checked_add(q.bytes)
            .ok_or(provenance::Error::Cap)?;
        if w > 8_000_000 || b > 16 << 20 {
            self.stopped = Some("reference-cap");
            return Err(provenance::Error::Cap);
        }
        self.work = w;
        self.bytes = b;
        Ok(())
    }
    fn check(&mut self) -> Result<(), provenance::Error> {
        if self.stopped.is_some() {
            return Err(provenance::Error::Admission);
        }
        if Instant::now() >= self.deadline {
            self.stopped = Some("reference-deadline");
            Err(provenance::Error::Admission)
        } else {
            Ok(())
        }
    }
}
impl checked_model::VerificationAdmission for ReferenceLedger {
    fn addition(&mut self) -> Result<(), provenance::Error> {
        self.check()?;
        if self.additions == 100_000 {
            self.stopped = Some("reference-additions");
            return Err(provenance::Error::Cap);
        }
        self.additions += 1;
        Ok(())
    }
}
#[derive(Serialize)]
struct References {
    models: [Reference; 3],
    verification: [checked_model::Progress; 3],
    outcomes: [Option<String>; 3],
    model_hashes: [Option<String>; 3],
    hash_errors: [Option<String>; 3],
    distinct_pairs: usize,
    qualified: bool,
    ledger: ReferenceLedger,
}
fn qualify(family: &str, twist: bool) -> Result<(), String> {
    use dataset::solvers::verified_worker_original_tests::Fixture;
    let output = std::path::PathBuf::from(
        std::env::var_os("SAT_REFERENCE_RESTRICTED_OUTPUT")
            .ok_or("supervisor output is mandatory")?,
    );
    let deadline_ns: u64 = std::env::var("SAT_REFERENCE_ORIGINAL_DEADLINE_MONOTONIC_NS")
        .map_err(|_| "supervisor original deadline mandatory")?
        .parse()
        .map_err(|_| "invalid monotonic deadline")?;
    if deadline_ns <= execution::monotonic_ns()? {
        return Err("original deadline expired before fixture creation".into());
    }
    // No fixture destructor deletes failed or completed acceptance evidence.
    let f = std::mem::ManuallyDrop::new(Fixture::with_fixed_sat_qualification(family, twist));
    let launch = execution::LaunchContext {
        schema: "crypto.ml-dataset.execution-context/v1".into(),
        binding: f.binding.clone(),
        boot_id: std::fs::read_to_string("/proc/sys/kernel/random/boot_id")
            .map_err(|e| e.to_string())?
            .trim()
            .into(),
        start_monotonic_ns: deadline_ns
            .checked_sub(60_000_000_000)
            .ok_or("invalid original interval")?,
        deadline_monotonic_ns: deadline_ns,
        seconds_per_case: 60,
        memory_bytes: 256 << 20,
        trace_bytes: 128 << 20,
    };
    let run: serde_json::Value = dataset::read_json(&f.root.join("run.json"))?;
    launch.validate(&run, &f.binding)?;
    storage::write_json(&f.root, execution::PATH, execution::ROLE, &launch)?;
    let context = execution::admit(&f.root)?.ok_or("original execution context missing")?;
    let mut writer = builder::CheckpointWriter::new(output);
    let setup =
        builder::fixed_nonroot_positive(&f.config, context.deadline, &mut |s| writer.save(s));
    writer.save(&setup)?;
    if let Some(error) = setup
        .first_error
        .as_ref()
        .or(setup.checkpoint_error.as_ref())
    {
        return Err(format!("retained fixed candidate setup failure: {error}"));
    }
    let case = setup.case.as_ref().ok_or("retained setup has no case")?;
    let mut registry = Vec::new();
    let token = dataset::solvers::verify_and_store_worker_originals(
        &f.config,
        case,
        &f.root,
        Some(&f.binding),
        &mut registry,
    )?;
    let (_, limits) = dataset::admit_capture_context(&f.root)?.ok_or("capture context missing")?;
    let sink =
        capture::CaptureEmitter::create(&f.root.join("live"), f.binding.clone(), limits.clone())
            .map_err(|e| e.to_string())?;
    let mut owner = match OriginalOwner::admit(&token, case, &context, &sink, &limits) {
        Ok(v) => v,
        Err(AdmissionFailure::BeforeBudget(stage)) => {
            return Err(format!("original admission before budget:{stage:?}"))
        }
        Err(AdmissionFailure::Retained(frame)) => {
            writer.save(&Record {
                stage: "original-admission",
                data: (
                    format!("{:?}", frame.stage),
                    format!("{:?}", frame.budget),
                    &frame.decoded.signed,
                    BigWords(&frame.decoded.codes),
                ),
            })?;
            return Err(format!("original owner admission:{:?}", frame.first_stop()));
        }
    };
    // Reserve the entire future nonproducer envelope BEFORE any producer write:
    // 64MiB journals + 32MiB logs + 4MiB immutable originals/inventory/receipts.
    // This is a debit against the unchanged128MiB original budget, not extra credit.
    let out =
        std::path::PathBuf::from(std::env::var_os("SAT_REFERENCE_RESTRICTED_OUTPUT").unwrap());
    let existing = builder::retained_bytes(&out)?;
    let logs =
        ["stdout.log", "stderr.log"]
            .iter()
            .try_fold(0u64, |n, name| -> Result<u64, String> {
                Ok(n + std::fs::metadata(out.join(name))
                    .map_err(|e| e.to_string())?
                    .len())
            })?;
    let originals = existing
        .checked_sub(writer.journal_bytes())
        .and_then(|n| n.checked_sub(logs))
        .ok_or("nonproducer accounting mismatch")?;
    if originals > 1 << 20 {
        return Err("frozen immutable-original envelope exceeded before capture".into());
    }
    owner
        .budget
        .reserve_trace(100 << 20, Instant::now())
        .map_err(|e| format!("original finite trace partition:{e:?}"))?;
    let original_id = owner.budget.ownership_id;
    let original_deadline = owner.budget.deadline;
    let gate_limits = provenance::Limits {
        variables: 100_000,
        codes: owner.math.codes.len(),
        domain_clauses: 1_000_000,
        work: owner.budget.native_work_limit,
        bytes: owner.budget.memory_limit,
    };
    let mut sink = sink;
    let construction = (|| {
        owner.build_map(0, gate_limits)?;
        owner.persist_map(&mut sink)
    })();
    // Actual source/partial owner survives errors until restricted serialization.
    writer.save(&Record {
        stage: "map",
        data: MapView {
            original_id,
            map: owner.checked.as_ref().map(|m| m.map()),
            raw_map: owner.retained_unchecked.as_ref(),
            pattern: owner.pattern,
            map_bytes: &owner.retained_bytes,
            map_ack: owner.map_ack.as_ref(),
            budget: format!("{:?}", owner.budget),
            error: construction.err().map(|e| format!("{e:?}")),
        },
    })?;
    construction.map_err(|e| format!("original map: {e:?}"))?;
    let initial_limits = provenance::initial::Limits {
        work: owner.budget.native_work_limit,
        bytes: owner.budget.memory_limit,
        events: 100_000,
        constraints: 1_000_000,
    };
    let stage = match NativeInitial::prepare(owner, sink, initial_limits) {
        Ok(stage) => stage,
        Err(Failure::BeforeNative { owner, stop, .. }) => {
            writer.save(&Record {
                stage: "before-native",
                data: (
                    format!("{stop:?}"),
                    owner.checked.as_ref().map(|m| m.map()),
                    format!("{:?}", owner.budget),
                ),
            })?;
            return Err(format!("native prerequisite: {stop:?}"));
        }
        Err(Failure::Retained { stage, stop }) => {
            let state = stage.shared.lock().unwrap_or_else(|e| e.into_inner());
            let ack = state.sink.committed_event_position();
            writer.save(&Record {
                stage: "retained-native",
                data: NativeView {
                    stop: format!("{stop:?}"),
                    events: &state.events,
                    pending: state.pending.as_ref(),
                    ack: Some(&ack),
                    budget: format!("{:?}", state.budget),
                },
            })?;
            return Err(format!("native prerequisite: {stop:?}"));
        }
    };
    let initial = match super::super::finish(stage.detach()) {
        Ok(v) => v,
        Err(f) => {
            let state = f
                .owner
                .stage
                .shared
                .lock()
                .unwrap_or_else(|e| e.into_inner());
            writer.save(&Record {
                stage: "initial-ledger",
                data: NativeView {
                    stop: format!("{:?}", f.stop),
                    events: &state.events,
                    pending: state.pending.as_ref(),
                    ack: f.owner.ack.as_ref(),
                    budget: format!("{:?}", state.budget),
                },
            })?;
            return Err(format!("initial ledger: {:?}", f.stop));
        }
    };
    let mut refs = References {
        models: std::array::from_fn(|_| Reference::default()),
        verification: std::array::from_fn(|_| checked_model::Progress::default()),
        outcomes: std::array::from_fn(|_| None),
        model_hashes: std::array::from_fn(|_| None),
        hash_errors: std::array::from_fn(|_| None),
        distinct_pairs: 0,
        qualified: false,
        ledger: ReferenceLedger {
            work: 0,
            bytes: 0,
            additions: 0,
            deadline: original_deadline,
            stopped: None,
        },
    };
    writer.save(&refs)?;
    let map = initial
        .stage
        .evidence
        .checked
        .as_ref()
        .ok_or("actual authenticated map missing")?;
    let arithmetic = curves::arithmetic(&case.public_instance)?;
    let [p, _, q] = setup.tuple;
    let tuples = [[p, p, q], [p, q, p], [q, p, p]];
    for (index, tuple) in tuples.into_iter().enumerate() {
        let result = (|| -> Result<(), provenance::Error> {
            refs.ledger.reserve(provenance::Quote {
                work: 4096,
                bytes: 256,
            })?;
            let partial = arithmetic.reference_add(tuple[0], tuple[1]);
            refs.ledger.check()?;
            let xs = [
                tuple[0].ok_or(provenance::Error::Semantics)?.0,
                tuple[1].ok_or(provenance::Error::Semantics)?.0,
                tuple[2].ok_or(provenance::Error::Semantics)?.0,
                partial.ok_or(provenance::Error::Semantics)?.0,
                setup.target.ok_or(provenance::Error::Semantics)?.0,
            ];
            reference::complete(
                map,
                &xs,
                &mut refs.models[index],
                &mut refs.ledger,
                &mut |state| {
                    writer
                        .save(&Record {
                            stage: "reference-progress",
                            data: (index, state),
                        })
                        .map_err(|_| provenance::Error::Admission)
                },
            )?;
            checked_model::verify(
                map,
                &refs.models[index].model,
                &initial.stage.evidence.math.signed,
                initial.stage.evidence.math.signed_target,
                &mut refs.verification[index],
                &mut refs.ledger,
            )
        })();
        refs.outcomes[index] = Some(match result {
            Ok(()) => "complete-map-and-signed-prefix-verified".into(),
            Err(e) => format!("refused:{e:?}"),
        });
        // Persist the primary verification result before optional hashing.
        writer.save(&refs)?;
        let hash_result = (|| -> Result<(), String> {
            if refs.models[index].model.len() == map.variables() as usize {
                // Canonical hash: domain tag, u64 little-endian length, one0/1 byte per wire.
                use sha2::{Digest, Sha256};
                refs.ledger
                    .reserve(provenance::Quote {
                        work: refs.models[index].model.len() as u64 + 64,
                        bytes: 64,
                    })
                    .map_err(|e| format!("model hashing admission:{e:?}"))?;
                let mut h = Sha256::new();
                h.update(b"sat-reference-model/v1\0");
                h.update((refs.models[index].model.len() as u64).to_le_bytes());
                for (offset, &bit) in refs.models[index].model.iter().enumerate() {
                    if offset % 64 == 0 {
                        refs.ledger
                            .check()
                            .map_err(|e| format!("model hash deadline:{e:?}"))?;
                    }
                    h.update([u8::from(bit)]);
                }
                refs.model_hashes[index] = Some(hex::encode(h.finalize()));
            }
            Ok(())
        })();
        if let Err(error) = hash_result {
            refs.hash_errors[index] = Some(error.clone());
            writer.save(&refs)?;
            return Err(error);
        }
        writer.save(&refs)?;
        // One cumulative reference owner: never renew after an exhausted limit.
        if refs.ledger.stopped.is_some()
            || Instant::now() >= original_deadline
            || refs.ledger.work >= 8_000_000
            || refs.ledger.bytes >= 16 << 20
        {
            break;
        }
    }
    for i in 0..3 {
        for j in i + 1..3 {
            if refs.verification[i].signed_relation && refs.verification[j].signed_relation {
                match reference::different_summands(
                    map,
                    &refs.models[i],
                    &refs.models[j],
                    &mut refs.ledger,
                ) {
                    Ok(true) => refs.distinct_pairs += 1,
                    Ok(false) => {}
                    Err(e) => {
                        writer.save(&refs)?;
                        return Err(format!("distinct-model check:{e:?}"));
                    }
                }
            }
        }
    }
    refs.qualified = refs.distinct_pairs > 0;
    writer.save(&refs)?;
    {
        let state = initial
            .stage
            .shared
            .lock()
            .map_err(|_| "original owner mutex")?;
        if state.budget.ownership_id != original_id
            || state.budget.deadline != original_deadline
            || initial
                .stage
                .solver
                .as_ref()
                .ok_or("native solver missing")?
                .stats
                .decisions
                != 0
        {
            return Err("reference checks changed original solver authority".into());
        }
        writer.save(&Record {
            stage: "qualification-terminal",
            data: TerminalView {
                qualified: refs.qualified,
                binding: map.original_binding(),
                pattern: map.pattern(),
                initial_ack: initial.ack.as_ref(),
                native_decisions: 0,
                native_search_acceptance: false,
                original_id,
                budget: format!("{:?}", state.budget),
                model_hashes: &refs.model_hashes,
            },
        })?;
    }
    if !refs.qualified {
        return Err("fixed candidate failed complete-model qualification; no fallback".into());
    }
    Ok(())
}
#[test]
fn binary_untwisted_complete_model_qualification() {
    qualify("binary", false).unwrap();
}
#[test]
fn binary_twisted_complete_model_qualification() {
    qualify("binary", true).unwrap();
}
#[test]
fn koblitz_untwisted_complete_model_qualification() {
    qualify("koblitz", false).unwrap();
}
#[test]
fn koblitz_twisted_complete_model_qualification() {
    qualify("koblitz", true).unwrap();
}
