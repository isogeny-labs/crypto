//! Independent central admission for ordinary worker artifacts.
use super::*;
use crate::ml_dataset::types::{Artifact, CaseConfig, CaseData};
use serde_json::Value;

pub(in crate::ml_dataset::solvers) fn is_source(outcome: &Value, artifacts: &[Artifact]) -> bool {
    outcome["adapter_id"] == "binary-m3-native-v1"
        || outcome["strategy"] == "binary-m3-native-v1"
        || artifacts.iter().any(|a| {
            let path = a.path.strip_prefix("live/payloads/").unwrap_or(&a.path);
            path.starts_with("binary-m3-")
                || path.starts_with("partial-binary-m3-")
                || a.role.starts_with("restricted-binary-m3-")
        })
}

// Historical checking has a separate, explicit fixed decode/algebra allowance.
// It never renews a producer deadline or changes a reported producer ledger.
struct ReplayAdmission {
    work: u64,
    bytes: u64,
}
impl OriginalAdmission for ReplayAdmission {
    fn reserve(&mut self, quote: Quote) -> Result<(), Failure> {
        self.work = self
            .work
            .checked_add(quote.work)
            .ok_or(Failure::Admission)?;
        self.bytes = self
            .bytes
            .checked_add(quote.bytes)
            .ok_or(Failure::Admission)?;
        if self.work > 64 * 1024 * 1024 || self.bytes > 16 * 1024 * 1024 {
            return Err(Failure::Admission);
        }
        Ok(())
    }
    fn check_deadline(&mut self) -> Result<(), Failure> {
        Ok(())
    }
}
#[derive(Deserialize)]
#[serde(tag = "phase", rename_all = "snake_case", deny_unknown_fields)]
enum SavedOrdinaryBoundary {
    Matrix { snapshot: native::Event },
    Signed { prefix: SignedPrefix },
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct SavedOrdinaryPayload {
    schema: String,
    binding_sha256: String,
    input_sha256: [u8; 32],
    input_capture_sequence: u64,
    sequence: u64,
    capture_sequence: u64,
    event: SavedOrdinaryBoundary,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct TypedOrdinary {
    kind: String,
    payload: SavedOrdinaryPayload,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct SavedAccounting {
    attempted: u64,
    acknowledged: u64,
    reserved_checking_work: u64,
    reserved_trace_bytes: u64,
    acknowledged_wire_bytes: u64,
    reserved_retention_work: u64,
    reserved_retention_bytes: u64,
    reserved_retention_trace: u64,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct SavedResult {
    schema: String,
    input_sha256: [u8; 32],
    input_capture_sequence: u64,
    report: Option<native::Report>,
    adapter_stop: Option<Stop>,
    ledger: SavedBudgetLedger,
    event_accounting: SavedAccounting,
    retained_event_bytes: usize,
    acknowledged_event_prefix_bytes: usize,
    unacknowledged_event_tail_bytes: usize,
    source_completion_claimed: bool,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct SavedEventIndex {
    schema: String,
    input_sha256: [u8; 32],
    input_capture_sequence: u64,
    start: capture::CommittedEventPosition,
    end: capture::CommittedEventPosition,
    acknowledged_events: u64,
    acknowledged_wire_bytes: u64,
    pending_tail: Vec<u8>,
}

// The endpoint denotes the last acknowledged ordinary event, not an arbitrary
// later authentic record. Witness records may interleave inside this span.
fn exact_index_endpoint(
    index: &SavedEventIndex,
    input_sequence: Option<u64>,
    last_native: u64,
    count: u64,
) -> bool {
    input_sequence == Some(index.input_capture_sequence)
        && index.start.event_count == index.input_capture_sequence
        && index.end.event_count
            == if count == 0 {
                index.input_capture_sequence
            } else {
                last_native
            }
}

fn root<'a>(artifacts: &'a [Artifact], slot: Slot) -> Result<Option<&'a Artifact>, String> {
    let (path, role, _encoding) = slot.identity();
    let selected: Vec<_> = artifacts
        .iter()
        .filter(|a| {
            !a.path.starts_with("live/payloads/")
                && (a.path == path
                    || a.role == role
                    || (slot == Slot::Events
                        && (a.path == "binary-m3-events.jsonl"
                            || a.role == "restricted-binary-m3-native-boundaries/v1")))
        })
        .collect();
    match selected.as_slice() {
        [] => Ok(None),
        [a] if slot.matches(a) => Ok(Some(*a)),
        // Collector custody is not a canonical semantic proof.
        [a] if a.path == path
            && a.role == "unregistered-partial-artifact"
            && a.encoding == "native" =>
        {
            Ok(None)
        }
        _ => Err(format!(
            "ordinary slot identity/uniqueness mismatch: {path}"
        )),
    }
}
fn read(dir: &std::path::Path, artifact: &Artifact, cap: usize) -> Result<Vec<u8>, String> {
    if artifact.bytes > cap as u64 {
        return Err("ordinary artifact exceeds decode cap".into());
    }
    solvers::prime_f4_wide::read_bounded(dir, &artifact.path, cap)
}
fn must_false(outcome: &Value, name: &str) -> Result<(), String> {
    if !outcome[name].is_null() && outcome[name] != false {
        return Err(format!("ordinary replay does not certify {name}"));
    }
    Ok(())
}

pub(in crate::ml_dataset::solvers) fn verify(
    dir: &std::path::Path,
    config: &CaseConfig,
    outcome: &Value,
    artifacts: &[Artifact],
) -> Result<Option<Value>, String> {
    let reserved_source = is_source(outcome, artifacts);
    let expected_route = config.family == "binary"
        && config.m == 3
        && config.factor_base_size == 4
        && matches!(config.solver.as_str(), "f4" | "f5")
        && ((23..=31).contains(&config.n) || (47..=61).contains(&config.n));
    if !reserved_source && !expected_route {
        return Ok(None);
    }
    if !outcome.is_object()
        || config.family != "binary"
        || config.m != 3
        || !((23..=31).contains(&config.n) || (47..=61).contains(&config.n))
        || config.factor_base_size != 4
        || !(1..=6).contains(&config.degree_bound)
    {
        return Err("ordinary artifact outside original profile".into());
    }
    let engine = match config.solver.as_str() {
        "f4" => native::Engine::F4Macaulay,
        "f5" => native::Engine::F5KoszulFrobenius,
        _ => return Err("ordinary engine outside profile".into()),
    };
    let saved: CaseConfig = crate::ml_dataset::read_json(&dir.join("config.json"))?;
    if &saved != config {
        return Err("ordinary config differs from saved original".into());
    }
    storage::verify_manifest(dir, artifacts)?;
    execution::verify(dir, artifacts)?;
    crate::ml_dataset::verify_capture_context(dir, artifacts)?;
    let launch = execution::load(dir)?.ok_or("ordinary replay lacks original launch")?;
    let (binding, capture_limits) = crate::ml_dataset::admit_capture_context(dir)?
        .ok_or("ordinary replay lacks original capture binding")?;
    if launch.binding != binding {
        return Err("ordinary launch/capture binding mismatch".into());
    }
    let limits = OriginalLimits::from_original(config, &launch)?;
    let config_bytes = solvers::prime_f4_wide::read_bounded(dir, "config.json", 64 * 1024)?;
    let public_bytes = solvers::prime_f4_wide::read_bounded(dir, "public.json", 64 * 1024)?;
    let factor_bytes = solvers::prime_f4_wide::read_bounded(dir, "factor-base.json", 128 * 1024)?;
    let case = CaseData {
        public_instance: serde_json::from_slice(&public_bytes).map_err(|e| e.to_string())?,
        factor_base: serde_json::from_slice(&factor_bytes).map_err(|e| e.to_string())?,
        private_witness: Value::Null,
    };
    if !reserved_source
        && outcome["strategy"] == "ordinary-binary-profile-unsupported"
        && outcome["solver_completed"] == false
        && !solvers::ordinary_worker_profile(config, &case)
    {
        return Ok(None);
    }
    let math = solvers::binary_m3_original::authenticate(Profile::OrdinaryBinary, config, &case)?;
    for slot in Slot::ALL {
        root(artifacts, slot)?;
    }
    for artifact in artifacts {
        let path = artifact
            .path
            .strip_prefix("live/payloads/")
            .unwrap_or(&artifact.path);
        if (path.starts_with("binary-m3-")
            || path.starts_with("partial-binary-m3-")
            || artifact.role.starts_with("restricted-binary-m3-"))
            && !Slot::ALL.iter().any(|s| {
                let (p, r, e) = s.identity();
                path == p && artifact.role == r && artifact.encoding == e
            })
            && !(artifact.role == "unregistered-partial-artifact" && artifact.encoding == "native")
        {
            return Err("unknown or relabeled reserved ordinary artifact".into());
        }
    }
    for name in [
        "fall_degree_verified",
        "independent_derivation_verified",
        "operation_trajectory_verified",
        "exact_instruction_count_verified",
        "full_execution_trace_verified",
        "scalar_recovered_without_witness",
        "producer_entry_anchor_verified",
        "capture_finalization_clock_verified",
        "linear_extraction_complete",
        "extraction_complete",
    ] {
        must_false(outcome, name)?;
    }
    for name in [
        "first_fall_degree",
        "last_fall_degree",
        "retired_hardware_instructions",
    ] {
        if !outcome[name].is_null() {
            return Err(format!("ordinary unsupported numerical claim {name}"));
        }
    }
    let mut admission = ReplayAdmission { work: 0, bytes: 0 };
    let admitted_prefix = capture::verify_live_committed_prefix(dir, &binding, &capture_limits)?;
    let captured_input = admitted_prefix.as_ref().and_then(|p| {
        p.payloads
            .iter()
            .find(|(_, a)| a.path == "payloads/binary-m3-input.json")
    });
    let input_artifact = root(artifacts, Slot::Input)?;
    let input_bytes = match input_artifact {
        Some(a) => Some(read(dir, a, INPUT_CAP)?),
        None => captured_input
            .map(|(_, a)| read(&dir.join("live"), a, INPUT_CAP))
            .transpose()?,
    };
    if let Some(bytes) = &input_bytes {
        verify_input(
            &config_bytes,
            &public_bytes,
            &factor_bytes,
            &binding,
            limits,
            bytes,
            &mut admission,
        )
        .map_err(|e| format!("ordinary original equations: {e:?}"))?;
    }
    // Absence is a censored producer state, not permission to omit semantic
    // checking of a present group/matrix artifact.
    if input_bytes.is_none()
        && [Slot::Result, Slot::Events, Slot::GroupReplay]
            .into_iter()
            .any(|s| root(artifacts, s).ok().flatten().is_some())
    {
        return Err("ordinary downstream proof lacks canonical original input".into());
    }
    let result_bytes = root(artifacts, Slot::Result)?
        .map(|a| read(dir, a, 32768))
        .transpose()?;
    let result: Option<SavedResult> = result_bytes
        .as_ref()
        .map(|b| serde_json::from_slice(b).map_err(|e| format!("ordinary matrix result: {e}")))
        .transpose()?;
    let input_sha: Option<[u8; 32]> = input_bytes.as_ref().map(|b| sha2::Sha256::digest(b).into());
    let events_root = root(artifacts, Slot::Events)?;
    let index: Option<SavedEventIndex> = events_root
        .filter(|a| a.encoding == "json")
        .map(|a| {
            read(dir, a, 16384)
                .and_then(|bytes| serde_json::from_slice(&bytes).map_err(|e| e.to_string()))
        })
        .transpose()?;
    let events = events_root
        .filter(|a| a.encoding == "jsonl")
        .map(|a| read(dir, a, (native::MAX_EVENTS as usize + 192) * EVENT_CAP))
        .transpose()?
        .unwrap_or_default();
    if let Some(index) = &index {
        if index.schema != "crypto.ml-dataset.binary-m3-event-index/v1"
            || Some(index.input_sha256) != input_sha
            || index.start.binding_sha256 != binding.digest()?
            || index.end.binding_sha256 != index.start.binding_sha256
            || index.start.event_count != index.input_capture_sequence
            || index
                .end
                .event_count
                .checked_sub(index.start.event_count)
                .is_none_or(|n| n < index.acknowledged_events)
            || index
                .end
                .byte_end
                .checked_sub(index.start.byte_end)
                .is_none_or(|n| n < index.acknowledged_wire_bytes)
            || index.pending_tail.len() > EVENT_CAP
            || result.as_ref().is_some_and(|r| {
                !matches!(
                    r.schema.as_str(),
                    "crypto.ml-dataset.binary-m3-matrix-result/v3"
                        | "crypto.ml-dataset.binary-m3-matrix-result/v4"
                ) || r.input_capture_sequence != index.input_capture_sequence
                    || r.event_accounting.acknowledged != index.acknowledged_events
                    || r.acknowledged_event_prefix_bytes as u64 != index.acknowledged_wire_bytes
                    || r.unacknowledged_event_tail_bytes != index.pending_tail.len()
            })
        {
            return Err("ordinary event index identity/count mismatch".into());
        }
    }
    if let Some(r) = &result {
        if !matches!(
            r.schema.as_str(),
            "crypto.ml-dataset.binary-m3-matrix-result/v1"
                | "crypto.ml-dataset.binary-m3-matrix-result/v2"
                | "crypto.ml-dataset.binary-m3-matrix-result/v3"
                | "crypto.ml-dataset.binary-m3-matrix-result/v4"
        ) || (events_root.is_some()
            && (r.schema.ends_with("/v3") || r.schema.ends_with("/v4")) != index.is_some())
            || Some(r.input_sha256) != input_sha
            || r.source_completion_claimed
            || (events_root.is_some() && index.is_none() && r.retained_event_bytes != events.len())
            || r.acknowledged_event_prefix_bytes
                .checked_add(r.unacknowledged_event_tail_bytes)
                != Some(r.retained_event_bytes)
            || r.event_accounting.acknowledged_wire_bytes
                != r.acknowledged_event_prefix_bytes as u64
            || r.event_accounting.acknowledged > r.event_accounting.attempted
            || r.event_accounting.attempted > native::MAX_EVENTS + 192
        {
            return Err("ordinary result binding/count mismatch".into());
        }
        verify_budget_ledger(
            &serde_json::to_vec(&r.ledger).map_err(|e| e.to_string())?,
            limits,
        )?;
    }
    let mut cursor = 0usize;
    let mut signed_events = Vec::new();
    let mut matrix_events = Vec::new();
    let mut last_capture_sequence = 0;
    let mut count = 0u64;
    let pending_anchor: Option<capture::EventRecord> = index
        .as_ref()
        .filter(|i| !i.pending_tail.is_empty())
        .map(|i| {
            serde_json::from_slice(&i.pending_tail)
                .map_err(|e| format!("ordinary pending index framing: {e}"))
        })
        .transpose()?;
    let mut pending_anchor_seen = false;
    let mut live_offset = 0u64;
    let mut index_start_seen = false;
    let mut index_end_seen = false;
    let prefix =
        capture::verify_live_committed_prefix_visit(dir, &binding, &capture_limits, &mut |line| {
            let wire: capture::EventRecord =
                serde_json::from_slice(line).map_err(|e| e.to_string())?;
            live_offset = live_offset
                .checked_add(line.len() as u64)
                .ok_or("ordinary live offset overflow")?;
            if let Some(pending) = &pending_anchor {
                if wire.sequence.checked_add(1) == Some(pending.sequence) {
                    if crate::ml_dataset::digest(line) != pending.previous_event_sha256 {
                        return Err("ordinary pending index previous-event hash differs".into());
                    }
                    pending_anchor_seen = true;
                }
            }
            if let Some(index) = &index {
                for (position, seen) in [
                    (&index.start, &mut index_start_seen),
                    (&index.end, &mut index_end_seen),
                ] {
                    if wire.sequence == position.event_count {
                        if live_offset != position.byte_end
                            || crate::ml_dataset::digest(line) != position.last_event_sha256
                        {
                            return Err(
                                "ordinary index position differs from actual committed bytes"
                                    .into(),
                            );
                        }
                        *seen = true;
                    }
                }
            }
            if wire.event["kind"] != "binary_m3_native" {
                return Ok(());
            }
            let typed: TypedOrdinary =
                serde_json::from_value(wire.event).map_err(|e| e.to_string())?;
            let payload = typed.payload;
            count += 1;
            if typed.kind != "binary_m3_native"
                || payload.schema != "crypto.ml-dataset.binary-m3-events/v1"
                || payload.binding_sha256 != binding.digest()?
                || Some(payload.input_sha256) != input_sha
                || payload.sequence != count
                || payload.capture_sequence != wire.sequence
                || payload.capture_sequence <= last_capture_sequence
                || captured_input.is_none_or(|(seq, _)| *seq != payload.input_capture_sequence)
                || result
                    .as_ref()
                    .is_some_and(|r| r.input_capture_sequence != payload.input_capture_sequence)
            {
                return Err("ordinary event identity or sequence mismatch".into());
            }
            {
                let end = cursor
                    .checked_add(line.len())
                    .ok_or("ordinary event cursor overflow")?;
                if events_root.is_some() && index.is_none() && events.get(cursor..end) != Some(line)
                {
                    return Err("ordinary canonical/live bytes differ".into());
                }
                cursor = end;
            }
            last_capture_sequence = payload.capture_sequence;
            match payload.event {
                SavedOrdinaryBoundary::Matrix { snapshot } => {
                    if !signed_events.is_empty() {
                        return Err("ordinary matrix after signed stage".into());
                    }
                    matrix_events.push(snapshot);
                }
                SavedOrdinaryBoundary::Signed { prefix } => signed_events.push(prefix),
            }
            Ok(())
        })?;
    if let Some(r) = &result {
        if cursor != r.acknowledged_event_prefix_bytes || count != r.event_accounting.acknowledged {
            return Err("ordinary acknowledged stream count differs".into());
        }
    }
    if let Some(index) = &index {
        if !exact_index_endpoint(
            index,
            captured_input.map(|(seq, _)| *seq),
            last_capture_sequence,
            count,
        ) || !index_start_seen
            || !index_end_seen
            || (pending_anchor.is_some() && !pending_anchor_seen)
            || index.acknowledged_events != count
            || index.acknowledged_wire_bytes != cursor as u64
        {
            return Err("ordinary index does not cover actual acknowledged prefix".into());
        }
    }
    let mut pending_signed = None;
    if let Some(r) = &result {
        let caps = matrix_events.len() as u64 * EVENT_CAP as u64
            + signed_events.len() as u64 * SIGNED_EVENT_CAP as u64;
        let a = &r.event_accounting;
        let extra = EVENT_CAP as u64;

        let encoding_bounds = if r.schema.ends_with("/v4") {
            (8 * caps + 8 * cursor as u64)..=(8 * (caps + extra) + 8 * (cursor as u64 + extra))
        } else {
            (16 * caps)..=(16 * (caps + extra))
        };
        if a.attempted > a.acknowledged + 1
            || !encoding_bounds.contains(&a.reserved_checking_work)
            || !(caps..=caps + extra).contains(&a.reserved_trace_bytes)
            || !retention_bounds(&r.schema, caps, cursor as u64, a)
            || a.reserved_checking_work
                .checked_add(a.reserved_retention_work)
                .is_none_or(|v| v > r.ledger.checking_reserved_work)
            || a.reserved_trace_bytes
                .checked_add(a.reserved_retention_trace)
                .is_none_or(|v| v > r.ledger.charged_trace_bytes)
            || a.reserved_retention_bytes > r.ledger.adapter_reserved_bytes
            || ((r.schema.ends_with("/v3") || r.schema.ends_with("/v4"))
                && a.reserved_checking_work
                    .checked_add(1024 * a.acknowledged)
                    .is_none_or(|v| v > r.ledger.checking_reserved_work))
        {
            return Err(
                "ordinary event reservations differ from actual acknowledged phases".into(),
            );
        }
    }
    // An index can outlive result publication. Validate its retained pending
    // record against the authenticated input and prefix even in that case.
    // Legacy JSONL still obtains its tail boundary from the saved result.
    let pending_tail = match &index {
        Some(i) if !i.pending_tail.is_empty() => Some(i.pending_tail.as_slice()),
        Some(_) => None,
        None if events_root.is_some()
            && result
                .as_ref()
                .is_some_and(|r| r.unacknowledged_event_tail_bytes > 0) =>
        {
            Some(events.get(cursor..).ok_or("ordinary tail offset")?)
        }
        None => None,
    };
    if let Some(tail) = pending_tail {
        if tail.len() > EVENT_CAP || !tail.ends_with(b"\n") {
            return Err("ordinary pending event cap/framing".into());
        }
        let wire: capture::EventRecord = serde_json::from_slice(tail).map_err(|e| e.to_string())?;
        let typed: TypedOrdinary = serde_json::from_value(wire.event).map_err(|e| e.to_string())?;
        let p = typed.payload;
        if typed.kind != "binary_m3_native"
            || p.schema != "crypto.ml-dataset.binary-m3-events/v1"
            || p.binding_sha256 != binding.digest()?
            || Some(p.input_sha256) != input_sha
            || p.sequence != count + 1
            || p.capture_sequence != wire.sequence
            || p.capture_sequence <= last_capture_sequence
            || Some(p.input_capture_sequence) != captured_input.map(|(seq, _)| *seq)
            || result
                .as_ref()
                .is_some_and(|r| p.input_capture_sequence != r.input_capture_sequence)
            || wire.binding_sha256 != binding.digest()?
        {
            return Err("ordinary pending event identity".into());
        }
        match p.event {
            SavedOrdinaryBoundary::Signed { prefix } => {
                if !check_matrix_events(
                    &matrix_events,
                    config.n as usize,
                    engine,
                    config.degree_bound,
                )? {
                    return Err("ordinary pending signed before matrix completion".into());
                }
                verify_signed_prefix(&math, &signed_events, Some(&prefix))?;
                pending_signed = Some(prefix);
            }
            SavedOrdinaryBoundary::Matrix { snapshot } => {
                if !signed_events.is_empty() {
                    return Err("ordinary pending matrix after signed phase".into());
                }
                let mut checked = matrix_events.clone();
                checked.push(snapshot);
                check_matrix_events(&checked, config.n as usize, engine, config.degree_bound)?;
            }
        }
    }
    if let Some(prefix) = &prefix {
        for path in ["live/committed.json", "live/events.jsonl"] {
            if artifacts.iter().filter(|a| a.path == path).count() != 1 {
                return Err("ordinary committed prefix lacks manifest member".into());
            }
        }
        if let Some(bytes) = &input_bytes {
            let captured = prefix
                .payloads
                .iter()
                .find(|(_, a)| a.path == "payloads/binary-m3-input.json")
                .ok_or("ordinary input lacks capture publication")?;
            if captured.1.sha256 != crate::ml_dataset::digest(bytes)
                || captured.1.bytes != bytes.len() as u64
                || captured.1.role != INPUT_ROLE
                || captured.1.encoding != "json"
                || result
                    .as_ref()
                    .is_some_and(|r| r.input_capture_sequence != captured.0)
            {
                return Err("ordinary input capture differs from root/result".into());
            }
        }
    } else if input_bytes.is_some() || !events.is_empty() {
        return Err("ordinary original input lacks committed capture".into());
    }
    let mut equations = Vec::new();
    if let Some(bytes) = &input_bytes {
        let saved: InputArtifact = serde_json::from_slice(bytes).map_err(|e| e.to_string())?;
        let construction: ConstructionArtifact =
            serde_json::from_str(&saved.construction_json).map_err(|e| e.to_string())?;
        let evidence = construction
            .evidence
            .ok_or("ordinary input lacks coordinate evidence")?;
        for anf in evidence.coordinate_anfs {
            let mut terms: Vec<_> = (0u8..8).filter(|&m| anf & (1 << m) != 0).collect();
            use crate::cryptanalysis::pq_groebner_f2::{cmp_mono, mask_from64, F2BoolMono};
            terms.sort_by(|&a, &b| {
                cmp_mono(
                    F2BoolMono::from_mask(mask_from64(b as u64)),
                    F2BoolMono::from_mask(mask_from64(a as u64)),
                )
            });
            equations.push(SavedEquation {
                n_vars: 3,
                terms: terms
                    .into_iter()
                    .map(|mask| SavedMonomial { mask })
                    .collect(),
            });
        }
    }
    if let Some(a) = root(artifacts, Slot::Construction)? {
        let bytes = read(dir, a, CAP)?;
        let saved: ConstructionArtifact =
            serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
        if saved.schema != SCHEMA
            || saved.original_config_sha256 != binding.config_sha256
            || saved.original_public_sha256 != <[u8; 32]>::from(sha2::Sha256::digest(&public_bytes))
            || saved.original_factor_base_sha256
                != <[u8; 32]>::from(sha2::Sha256::digest(&factor_bytes))
            || saved.binding_sha256 != binding.digest()?
            || saved.original_limits != limits
        {
            return Err("ordinary construction identity mismatch".into());
        }
        if let Some(evidence) = &saved.evidence {
            let values = elements(&math);
            resultant::verify_evidence(&input(&math, &values), evidence, &mut admission)
                .map_err(|e| format!("ordinary construction replay: {e:?}"))?;
        } else if saved.determinant_verified {
            return Err("ordinary construction success lacks evidence".into());
        }
        if let Some(input_bytes) = &input_bytes {
            let i: InputArtifact =
                serde_json::from_slice(input_bytes).map_err(|e| e.to_string())?;
            if bytes != i.construction_json.as_bytes() {
                return Err("ordinary construction/root differs from persisted input".into());
            }
        }
    }
    let matrix_complete = check_matrix_events(
        &matrix_events,
        config.n as usize,
        engine,
        config.degree_bound,
    )?;
    if let Some(report) = result.as_ref().and_then(|r| r.report.as_ref()) {
        verify_matrix_report(
            &serde_json::to_vec(report).map_err(|e| e.to_string())?,
            &equations,
            engine,
            config.degree_bound,
            limits,
        )?;
        let acknowledged = matrix_events.len() as u64;
        let late_durable_ack = report.counters.acknowledged_events.checked_add(1)
            == Some(acknowledged)
            && !report.native_step_complete
            && result.as_ref().is_some_and(|r| r.adapter_stop.is_some());
        if (report.counters.acknowledged_events != acknowledged && !late_durable_ack)
            || report.native_step_complete && !matrix_complete
        {
            return Err("ordinary native report differs from actual boundary prefix".into());
        }
        if let Some(last) = matrix_events.last() {
            if report.reserved.work < last.reserved.work
                || report.reserved.bytes < last.reserved.bytes
            {
                return Err("ordinary report reservation predates captured prefix".into());
            }
        }
    }
    let group_bytes = root(artifacts, Slot::GroupReplay)?
        .map(|a| read(dir, a, 49152))
        .transpose()?;
    let mut domain_complete = false;
    let mut candidate_tuples = Vec::new();
    let mut checked_group = None;
    if let Some(bytes) = &group_bytes {
        let hash = input_sha.ok_or("ordinary group lacks original input")?;
        let assessment = verify_signed_replay_profile(
            bytes,
            &math,
            &binding,
            &crate::ml_dataset::digest(&public_bytes),
            &crate::ml_dataset::digest(&factor_bytes),
            &hex::encode(hash),
            limits.additions,
            Profile::OrdinaryBinary,
        )?;
        let group: SavedSignedReplay = serde_json::from_slice(bytes).map_err(|e| e.to_string())?;
        if pending_signed
            .as_ref()
            .is_some_and(|p| group.unacknowledged.as_ref() != Some(p))
        {
            return Err("ordinary pending signed bytes differ from group owner".into());
        }
        if group.acknowledged != signed_events {
            return Err("ordinary signed group prefix differs from actual captured events".into());
        }
        domain_complete = assessment.all_tuples_acknowledged;
        candidate_tuples = assessment.candidate_tuples;
        checked_group = Some(group);
    } else if !signed_events.is_empty() {
        // Authenticated live signed arithmetic remains checkable on interruption;
        // absent terminal group storage cannot establish completed domain delivery.
        verify_signed_prefix(&math, &signed_events, None)?;
    }
    if !signed_events.is_empty() && !matrix_complete {
        return Err("ordinary signed events lack completed native matrix prefix".into());
    }
    let mut witness = false;
    let witness_root = root(artifacts, Slot::Witness)?;
    if let Some(prefix) = &prefix {
        if let Some((_, payload)) = prefix
            .payloads
            .iter()
            .find(|(_, a)| a.path == "payloads/pdp-witness.json")
        {
            if payload.role != "independently-verified-pdp" || payload.encoding != "json" {
                return Err("ordinary captured witness identity mismatch".into());
            }
            let path = dir.join("live");
            let bytes = read(&path, payload, SIGNED_WITNESS_BYTE_CAP)?;
            let verified = verify_signed_witness(&bytes, &math)?;
            if domain_complete && !candidate_tuples.contains(&verified.tuple) {
                return Err("ordinary witness missing from completed signed domain".into());
            }
            if let Some(root) = witness_root {
                if root.sha256 != payload.sha256
                    || root.bytes != payload.bytes
                    || read(dir, root, SIGNED_WITNESS_BYTE_CAP)? != bytes
                {
                    return Err("ordinary root witness differs from dedicated capture".into());
                }
            }
            witness = true;
        }
    }
    if witness_root.is_some() && !witness {
        return Err("ordinary witness root lacks dedicated proof capture".into());
    }
    if domain_complete && !candidate_tuples.is_empty() && !witness {
        return Err("ordinary completed positive domain lacks checked witness publication".into());
    }
    if let Some(r) = &result {
        check_signed_ledger(
            r.ledger.group_additions,
            &signed_events,
            pending_signed.as_ref(),
            checked_group.as_ref(),
            witness,
        )?;
    }
    let delivery_complete = check_delivery(
        dir,
        artifacts,
        input_sha,
        limits,
        &capture_limits,
        result.as_ref().map(|r| r.ledger.group_additions),
    )?;
    let eligible = domain_complete
        && matrix_complete
        && delivery_complete
        && result.as_ref().is_some_and(|r| {
            r.ledger.phase == Phase::Complete
                && r.ledger.first_stop.is_none()
                && r.adapter_stop.is_none()
                && r.report.as_ref().is_some_and(|p| p.native_step_complete)
        })
        && input_artifact.is_some()
        && (!witness || witness_root.is_some());
    if let Some(r) = &result {
        for (name, value) in [
            (
                "adapter_reserved_native_work",
                r.ledger.native_reserved.work,
            ),
            (
                "adapter_reserved_checking_work",
                r.ledger.checking_reserved_work,
            ),
            ("adapter_reserved_bytes", r.ledger.adapter_reserved_bytes),
            ("native_reserved_bytes", r.ledger.native_reserved.bytes),
            ("charged_trace_bytes", r.ledger.charged_trace_bytes),
            ("performed_group_additions", r.ledger.group_additions),
        ] {
            if !outcome[name].is_null() && outcome[name].as_u64() != Some(value) {
                return Err(format!("ordinary outcome counter differs: {name}"));
            }
        }
        if r.ledger.first_stop.is_some()
            && !outcome["first_stop"].is_null()
            && serde_json::from_value::<Option<Stop>>(outcome["first_stop"].clone())
                .map_err(|e| e.to_string())?
                != r.ledger.first_stop
        {
            return Err("ordinary outcome overwrote producer first stop".into());
        }
    }
    verify_reported_capture_finalization(outcome)?;
    let completed = outcome["solver_completed"]
        .as_bool()
        .ok_or("ordinary completion must be boolean")?;
    if completed
        && (outcome["status"] != "verified"
            || !outcome["first_stop"].is_null()
            || !eligible
            || outcome["capture_finalization"]["finish_acknowledged"] != true)
    {
        return Err("ordinary completion lacks independently joined delivery evidence".into());
    }
    if !completed
        && !matches!(
            outcome["status"].as_str(),
            Some(
                "partial" | "timeout" | "trace_budget_exhausted" | "worker_failed" | "unsupported"
            )
        )
    {
        return Err("ordinary incomplete result lacks censored status".into());
    }
    for (key, proved) in [
        ("pdp_witness_verified", witness && witness_root.is_some()),
        ("retained_pdp_witness_verified", witness),
        (
            "certificate_durably_persisted",
            witness && witness_root.is_some(),
        ),
        ("group_domain_complete", completed && domain_complete),
        (
            "exhaustive_no_decomposition",
            completed && candidate_tuples.is_empty() && !witness,
        ),
        ("algebra_completed", completed && matrix_complete),
        ("native_backend_invoked", completed && matrix_complete),
        ("native_statistics_complete", completed && matrix_complete),
    ] {
        if outcome[key] == true && !proved {
            return Err(format!("ordinary unsupported outcome claim {key}"));
        }
    }
    Ok(Some(
        serde_json::json!({"scope":"ordinary-original-bound-native-and-signed-replay",
        "solver_completed":completed,"retained_witness_verified":witness,
        "group_domain_complete":completed && domain_complete,"matrix_prefix_complete":matrix_complete,
        "offline_checking_reserved_work":admission.work,"offline_checking_reserved_bytes":admission.bytes}),
    ))
}

/// Join independently replayed arithmetic to the producer ledger. A canonical
/// group retains the complete performed prefix, including its one pending row.
/// The only additional calls are up to three reference additions for the first
/// witness. Missing terminal group storage permits only a lower-bound check.
fn check_signed_ledger(
    charged: u64,
    acknowledged: &[SignedPrefix],
    pending: Option<&SignedPrefix>,
    group: Option<&SavedSignedReplay>,
    witness: bool,
) -> Result<(), String> {
    let observed = acknowledged.len() as u64 + u64::from(pending.is_some());
    let minimum = observed
        .checked_add(if witness { 3 } else { 0 })
        .ok_or("ordinary addition join overflow")?;
    if charged < minimum {
        return Err("ordinary ledger undercounts independently replayed additions".into());
    }
    if let Some(group) = group {
        let enumeration = group.performed_additions as u64;
        let checks = charged
            .checked_sub(enumeration)
            .ok_or("ordinary ledger undercounts retained signed domain")?;
        let has_candidate = group
            .acknowledged
            .iter()
            .chain(group.unacknowledged.iter())
            .any(|e| e.matches_target);
        let acknowledged_candidate = group.acknowledged.iter().any(|e| e.matches_target);
        if checks > 3
            || (!has_candidate && checks != 0)
            || ((witness || acknowledged_candidate) && checks != 3)
        {
            return Err(
                "ordinary ledger witness reference additions lack arithmetic prerequisites".into(),
            );
        }
    }
    Ok(())
}

fn check_matrix_events(
    events: &[native::Event],
    equations: usize,
    engine: native::Engine,
    degree: u32,
) -> Result<bool, String> {
    let mut phase = 0u8;
    let mut previous = [0u64; 15];
    let mut reserved = native::Reservation::default();
    for (index, event) in events.iter().enumerate() {
        let next = match event.stage {
            native::Stage::Begin => 1,
            native::Stage::CriterionLevel { level } => {
                if engine != native::Engine::F5KoszulFrobenius || level >= degree {
                    return Err("ordinary criterion level".into());
                }
                2
            }
            native::Stage::CriterionCandidate {
                generator,
                multiplier,
                ..
            } => {
                if engine != native::Engine::F5KoszulFrobenius
                    || generator >= equations
                    || multiplier > 7
                {
                    return Err("ordinary criterion candidate".into());
                }
                2
            }
            native::Stage::MainCandidate {
                generator,
                multiplier,
            } => {
                if generator >= equations || multiplier > 7 {
                    return Err("ordinary main candidate".into());
                }
                3
            }
            native::Stage::Pack => 4,
            native::Stage::Reduce => 5,
            native::Stage::Readback => 6,
            native::Stage::Finish => 7,
        };
        let ordered = match next {
            1 => phase == 0,
            2 => matches!(phase, 1 | 2),
            3 => matches!(phase, 1 | 2 | 3),
            4 => matches!(phase, 1 | 2 | 3),
            5..=7 => phase + 1 == next,
            _ => false,
        };
        let counters = native_counter_values(event.counters);
        if !ordered
            || event.counters.attempted_events != index as u64 + 1
            || event.counters.acknowledged_events != index as u64
            || counters.iter().zip(previous).any(|(&a, b)| a < b)
            || event.reserved.work < reserved.work
            || event.reserved.bytes < reserved.bytes
            || event.reserved.work > native::MAX_WORK
            || event.reserved.bytes > native::MAX_BYTES
            || event.counters.dynamic_layout_bytes
                > event.reserved.bytes.saturating_sub(native::METADATA_BYTES)
        {
            return Err("ordinary native boundary phase/count/reservation mismatch".into());
        }
        previous = counters;
        reserved = event.reserved;
        phase = next;
    }
    Ok(phase == 7)
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct SavedDeliveryStatus {
    path: String,
    retained_bytes: usize,
    encoding_complete: bool,
    write_attempted: bool,
    root_transferred: bool,
    untransferred_root: Option<Artifact>,
    error: Option<String>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct SavedOrdinaryDelivery {
    schema: String,
    input_sha256: [u8; 32],
    source_completion_claimed: bool,
    accepted_roots_before_delivery: Vec<Artifact>,
    prior_slots: [SavedDeliveryStatus; 3],
    prepaid_storage: [RetentionQuote; 4],
    ledger: SavedBudgetLedger,
}
fn check_delivery(
    dir: &std::path::Path,
    artifacts: &[Artifact],
    input_sha: Option<[u8; 32]>,
    limits: OriginalLimits,
    capture_limits: &capture::CaptureLimits,
    result_additions: Option<u64>,
) -> Result<bool, String> {
    let Some(a) = root(artifacts, Slot::ParentDelivery)? else {
        return Ok(false);
    };
    let bytes = read(dir, a, 16384)?;
    let v: SavedOrdinaryDelivery = serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
    if v.schema != "crypto.ml-dataset.binary-m3-parent-delivery/v1"
        || Some(v.input_sha256) != input_sha
        || v.source_completion_claimed
        || v.accepted_roots_before_delivery.len() > Slot::ALL.len() - 1
    {
        return Err("ordinary parent delivery identity/cap mismatch".into());
    }
    verify_budget_ledger(
        &serde_json::to_vec(&v.ledger).map_err(|e| e.to_string())?,
        limits,
    )?;
    if result_additions.is_some_and(|count| count != v.ledger.group_additions) {
        return Err("ordinary delivery addition ledger differs from result snapshot".into());
    }
    for (i, cap) in retained_storage::ADMITTED_CAPS.iter().enumerate() {
        let quoted = quote_parent_storage_length(*cap as u64, false, capture_limits)
            .map_err(|s| format!("ordinary delivery storage quote {s:?}"))?;
        let prior_quote =
            quote_parent_storage_length(retained_storage::CAPS[i] as u64, false, capture_limits)
                .map_err(|s| format!("ordinary previous delivery quote {s:?}"))?;
        if v.prepaid_storage[i] != quoted && v.prepaid_storage[i] != prior_quote {
            return Err("ordinary parent quote differs from original limits".into());
        }
        if i == 2 && v.prepaid_storage[i] == quoted {
            if let Some(result) = root(artifacts, Slot::Result)? {
                if result.bytes > *cap as u64 {
                    return Err("ordinary result exceeds prepaid structural envelope".into());
                }
            }
        }
    }
    let roots = &v.accepted_roots_before_delivery;
    let mut paths = std::collections::BTreeSet::new();
    for root in roots {
        if !paths.insert(&root.path)
            || !artifacts.iter().any(|a| a == root)
            || !Slot::ALL.iter().any(|s| s.matches(root))
            || Slot::ParentDelivery.matches(root)
        {
            return Err("ordinary delivery invented/duplicated root receipt".into());
        }
    }
    let mut prior_complete = true;
    for (status, slot) in v
        .prior_slots
        .iter()
        .zip([Slot::Construction, Slot::Events, Slot::Result])
    {
        if status.path != slot.identity().0
            && !(slot == Slot::Events && status.path == "binary-m3-events.jsonl")
        {
            return Err("ordinary delivery prior-slot order".into());
        }
        if status.root_transferred {
            let artifact = roots
                .iter()
                .find(|a| slot.matches(a))
                .ok_or("ordinary delivery claims absent root")?;
            if !status.encoding_complete
                || !status.write_attempted
                || status.untransferred_root.is_some()
                || status.error.is_some()
                || artifact.bytes != status.retained_bytes as u64
            {
                return Err("ordinary delivery transferred-slot facts inconsistent".into());
            }
        } else {
            prior_complete = false;
            if let Some(root) = &status.untransferred_root {
                if !slot.matches(root)
                    || !status.write_attempted
                    || !status.encoding_complete
                    || !artifacts.iter().any(|a| {
                        a.path == root.path && a.sha256 == root.sha256 && a.bytes == root.bytes
                    })
                {
                    return Err("ordinary rejected root lacks actual manifest bytes".into());
                }
            }
        }
    }
    for slot in [
        Slot::Input,
        Slot::Construction,
        Slot::Events,
        Slot::Result,
        Slot::GroupReplay,
    ] {
        if !roots.iter().any(|a| slot.matches(a)) {
            return Ok(false);
        }
    }
    Ok(prior_complete && v.ledger.first_stop.is_none() && v.ledger.phase == Phase::Complete)
}

fn retention_bounds(schema: &str, caps: u64, wire: u64, a: &SavedAccounting) -> bool {
    let (base, exact_join) = match schema {
        "crypto.ml-dataset.binary-m3-matrix-result/v1" => (caps, false),
        "crypto.ml-dataset.binary-m3-matrix-result/v2" => (wire, true),
        "crypto.ml-dataset.binary-m3-matrix-result/v3"
        | "crypto.ml-dataset.binary-m3-matrix-result/v4" => {
            return a.reserved_retention_work == 0
                && a.reserved_retention_bytes == 0
                && a.reserved_retention_trace == 0
        }
        _ => return false,
    };
    let extra = EVENT_CAP as u64;
    (32 * base..=32 * (base + extra)).contains(&a.reserved_retention_work)
        && (base..=base + extra).contains(&a.reserved_retention_bytes)
        && (4 * base..=4 * (base + extra)).contains(&a.reserved_retention_trace)
        && (!exact_join || a.reserved_retention_work == 32 * a.reserved_retention_bytes)
}
#[cfg(test)]
mod accounting_version_tests {
    use super::*;
    #[test]
    fn retained_v6_partial_keeps_legacy_accounting_without_accepting_v2_relabel() {
        let r: SavedResult =
            serde_json::from_str(include_str!("binary_m3_legacy_partial.json")).unwrap();
        assert_eq!(r.event_accounting.acknowledged, 17);
        assert_eq!(r.acknowledged_event_prefix_bytes, 18528);
        let caps = 17 * EVENT_CAP as u64;
        assert!(retention_bounds(
            &r.schema,
            caps,
            18528,
            &r.event_accounting
        ));
        assert!(!retention_bounds(
            "crypto.ml-dataset.binary-m3-matrix-result/v2",
            caps,
            18528,
            &r.event_accounting
        ));
        let mut actual = r.event_accounting;
        actual.reserved_retention_work = 32 * 18528;
        actual.reserved_retention_bytes = 18528;
        actual.reserved_retention_trace = 4 * 18528;
        assert!(retention_bounds(
            "crypto.ml-dataset.binary-m3-matrix-result/v2",
            caps,
            18528,
            &actual
        ));
        assert!(!retention_bounds(
            "crypto.ml-dataset.binary-m3-matrix-result/v1",
            caps,
            18528,
            &actual
        ));
        actual.reserved_retention_work += 32;
        assert!(!retention_bounds(
            "crypto.ml-dataset.binary-m3-matrix-result/v2",
            caps,
            18528,
            &actual
        ));
    }
}

#[cfg(test)]
mod exact_index_endpoint_tests {
    use super::*;
    fn index(start: u64, end: u64) -> SavedEventIndex {
        serde_json::from_value(serde_json::json!({
            "schema":"crypto.ml-dataset.binary-m3-event-index/v1",
            "input_sha256":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
            "input_capture_sequence":start,
            "start":{"binding_sha256":"a".repeat(64),"event_count":start,"byte_end":100,"last_event_sha256":"b".repeat(64)},
            "end":{"binding_sha256":"a".repeat(64),"event_count":end,"byte_end":900,"last_event_sha256":"c".repeat(64)},
            "acknowledged_events":2,"acknowledged_wire_bytes":600,"pending_tail":[]
        })).unwrap()
    }
    #[test]
    fn interleaved_witness_is_allowed_but_unrelated_authentic_endpoint_is_not() {
        // Input at1, native at2 and4, with an authentic witness record at3.
        assert!(exact_index_endpoint(&index(1, 4), Some(1), 4, 2));
        // A later authentic non-native record at5 must not extend the index.
        assert!(!exact_index_endpoint(&index(1, 5), Some(1), 4, 2));
        assert!(!exact_index_endpoint(&index(1, 3), Some(1), 4, 2));
        assert!(!exact_index_endpoint(&index(1, 4), Some(2), 4, 2));
        assert!(!exact_index_endpoint(&index(1, 4), None, 4, 2));
        assert!(exact_index_endpoint(&index(1, 1), Some(1), 0, 0));
        assert!(!exact_index_endpoint(&index(1, 2), Some(1), 0, 0));
    }
}
