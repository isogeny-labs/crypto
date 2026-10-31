//! Causal signed-group examples from original-bound committed native output.
//! Matrix candidate intervals are intentionally not implemented by this module.
use super::*;
use crate::ml_dataset::{
    capture, storage,
    types::{Artifact, CaseConfig, CaseData},
};
use serde_json::{json, Value};
use std::path::Path;

const GROUP_RECORDS: usize = 192;
const PROJECTOR_GROUP_ADDS: usize = 256;
const PROJECTOR_STREAM_BYTES: u64 = 64 * 1024 * 1024;

/// Bind the bytes actually parsed, not merely a later reread of the path.
fn authenticate_cached_original(
    bytes: &[u8],
    artifacts: &[Artifact],
    path: &str,
    role: &str,
) -> Result<(), String> {
    let descriptor =
        crate::ml_dataset::solvers::prime_f4_wide::artifact(artifacts, path, role, "json", true)?
            .ok_or("missing cached original descriptor")?;
    if descriptor.bytes != bytes.len() as u64
        || descriptor.sha256 != crate::ml_dataset::digest(bytes)
    {
        return Err(format!(
            "cached Koblitz original differs from manifest: {path}"
        ));
    }
    Ok(())
}

fn signed_capture_event(event: Value) -> Result<Option<SignedPrefix>, String> {
    if event["kind"] != "koblitz_m3" {
        return Ok(None);
    }
    let event: SavedSignedEvent = serde_json::from_value(event)
        .map_err(|e| format!("strict Koblitz group projection event: {e}"))?;
    if event.kind != "koblitz_m3" || event.payload.phase != "signed-domain" {
        return Err("Koblitz causal signed event identity mismatch".into());
    }
    Ok(Some(event.payload.progress))
}

fn static_inputs(original: &MathInput, twist: bool) -> Value {
    json!({
        "family":"koblitz", "m":3, "n":original.curve.m, "twist":twist,
        "field":{"characteristic":"2", "degree":original.curve.m,
            "basis":"polynomial", "irreducible_low_terms":&original.curve.irreducible.low_terms},
        "curve":{"model":"y2+xy=x3+a*x2+b",
            "a":original.curve.a.to_biguint().to_string(),
            "b":original.curve.b.to_biguint().to_string()},
        "target":DecimalPoint::from(original.target),
        "signed_domain":original.signed.map(DecimalPoint::from),
        "x_codes":original.codes.map(DecimalWord)
    })
}

/// The caller has completed custody validation. Recheck the entire bounded
/// arithmetic prefix before emitting anything: a forged later sum cannot leak
/// earlier provisional examples into the exporter's output.
fn project_signed(
    original: &MathInput,
    twist: bool,
    signed: &[SignedPrefix],
    captures: &[u64],
    emit: &mut impl FnMut(Option<Value>, Value) -> Result<(), String>,
) -> Result<Value, String> {
    if signed.len() > GROUP_RECORDS
        || signed.len() > PROJECTOR_GROUP_ADDS
        || captures.len() != signed.len()
        || captures
            .iter()
            .enumerate()
            .any(|(i, &seq)| seq == 0 || i > 0 && seq <= captures[i - 1])
    {
        return Err("Koblitz causal group record/sequence limit".into());
    }
    let checked = verify_signed_prefix(original, signed, None)?;
    let initial = static_inputs(original, twist);
    for (i, step) in signed.iter().enumerate() {
        // The schedule is independently reconstructed, not copied from a future
        // event. Current post-addition prefix/target-match stay in the label.
        let tuple = i / 3;
        let summand = i % 3;
        let indices = [tuple / 16, (tuple / 4) % 4, tuple % 4];
        let prior = if summand == 0 {
            None
        } else {
            signed[i - 1].prefix
        };
        let feature = json!({
            "schema":"crypto.ml-dataset.koblitz-causal-input/v1",
            "task":"signed-addition-outcome",
            "static_inputs":&initial,
            "action_query":{"tuple":tuple,"indices":indices,"summand":summand,
                "operand":DecimalPoint::from(original.signed[indices[summand]])},
            "prefix_state":{"completed_additions":i,
                "accumulator":prior.map(DecimalPoint::from),
                "infinity":prior.is_none()}
        });
        let label = json!({
            "kind":"koblitz-signed-addition",
            "admission_status":"verified_group_step",
            "source_capture_sequence":captures[i],
            "signed_sequence":i,
            "label":{"sum":step.prefix.map(DecimalPoint::from),
                "infinity":step.prefix.is_none(),"tuple_completed":summand==2,
                "matches_target":step.matches_target},
            "verification_scope":"independent original-domain group addition",
            "native_basis_derivation_verified":false,
            "exact_instruction_count_verified":false,"fall_degree_verified":false
        });
        emit(Some(feature), label)?;
    }
    Ok(json!({
        "schema":"crypto.ml-dataset.koblitz-causal-projection/v1",
        "admission_status":if signed.is_empty() {"no_committed_signed_events"} else {"retained_prefix_checked"},
        "group_examples":signed.len(),
        "group_domain_complete":checked.all_tuples_acknowledged,
        "unknown_group_suffix":signed.len()<GROUP_RECORDS,
        "projector_group_additions":signed.len(),
        "projector_work_scope":"verify_signed_prefix additions only; excludes source admission, original reconstruction, capture authentication, parsing, allocation and I/O",
        "limits":{"group_records":GROUP_RECORDS,"projector_group_additions":PROJECTOR_GROUP_ADDS,"stream_bytes":PROJECTOR_STREAM_BYTES},
        "matrix_projection_status":"not_implemented",
        "original_equations_projection_status":"not_implemented",
        "construction_projection_status":"not_implemented",
        "native_basis_derivation_verified":false,
        "exact_instruction_count_verified":false,"fall_degree_verified":false
    }))
}

fn check_projection_watermark(
    expected: Option<&capture::CommittedHead>,
    observed: &capture::CaptureCursor,
    custody_present: bool,
) -> Result<(), String> {
    match expected {
        Some(head)
            if custody_present
                && observed.event_count == head.event_count
                && observed.byte_end == head.byte_end
                && observed.last_event_sha256 == head.last_event_sha256 =>
        {
            Ok(())
        }
        None if !custody_present && observed.event_count == 0 && observed.byte_end == 0 => Ok(()),
        _ => Err("Koblitz projected bytes differ from original committed watermark".into()),
    }
}

pub(super) fn project(
    dir: &Path,
    config: &CaseConfig,
    outcome: &Value,
    artifacts: &[Artifact],
    emit: &mut impl FnMut(Option<Value>, Value) -> Result<(), String>,
) -> Result<Value, String> {
    project_with_hooks(dir, config, outcome, artifacts, emit, || Ok(()), || Ok(()))
}

// Hooks are private deterministic admission controls. Production supplies no-ops.
// They never run the producer or change an original execution budget.
pub(super) fn project_with_hooks(
    dir: &Path,
    config: &CaseConfig,
    outcome: &Value,
    artifacts: &[Artifact],
    emit: &mut impl FnMut(Option<Value>, Value) -> Result<(), String>,
    before_visit: impl FnOnce() -> Result<(), String>,
    after_visit: impl FnOnce() -> Result<(), String>,
) -> Result<Value, String> {
    use crate::ml_dataset::solvers::prime_f4_wide::read_bounded;
    // Check bounded original parsing before central replay. These are source
    // custody limits, separate from the group projection arithmetic counter.
    let public_bytes = read_bounded(dir, "public.json", 64 * 1024)?;
    let factor_bytes = read_bounded(dir, "factor-base.json", 128 * 1024)?;
    let (binding, limits) = crate::ml_dataset::admit_capture_context(dir)?
        .ok_or("Koblitz causal projection requires original capture context")?;
    let binding_bytes = read_bounded(dir, "capture-binding.json", 64 * 1024)?;
    let limits_bytes = read_bounded(dir, "capture-limits.json", 64 * 1024)?;
    for (bytes, path, role) in [
        (public_bytes.as_slice(), "public.json", "public-instance"),
        (factor_bytes.as_slice(), "factor-base.json", "factor-base"),
        (
            binding_bytes.as_slice(),
            "capture-binding.json",
            "partial-capture-binding",
        ),
        (
            limits_bytes.as_slice(),
            "capture-limits.json",
            "partial-capture-limits",
        ),
    ] {
        authenticate_cached_original(bytes, artifacts, path, role)?;
    }
    let bound_binding: capture::AttemptBinding =
        serde_json::from_slice(&binding_bytes).map_err(|e| e.to_string())?;
    let bound_limits: capture::CaptureLimits =
        serde_json::from_slice(&limits_bytes).map_err(|e| e.to_string())?;
    if binding != bound_binding || limits != bound_limits {
        return Err(
            "Koblitz cached capture admission differs from manifest-bound originals".into(),
        );
    }
    let binding_sha256 = binding.digest()?;
    let head_path = dir.join("live/committed.json");
    let head = if head_path.exists() {
        let bytes = read_bounded(dir, "live/committed.json", 64 * 1024)?;
        let head: capture::CommittedHead =
            serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
        if head.byte_end > PROJECTOR_STREAM_BYTES {
            return Err("Koblitz causal projection stream cap (no rows emitted)".into());
        }
        Some(head)
    } else {
        None
    };
    if verify_worker_cell(dir, config, outcome, artifacts)?.is_none() {
        return Err("source is not an admitted native Koblitz worker cell".into());
    }
    let case = CaseData {
        public_instance: serde_json::from_slice(&public_bytes).map_err(|e| e.to_string())?,
        private_witness: Value::Null,
        factor_base: serde_json::from_slice(&factor_bytes).map_err(|e| e.to_string())?,
    };
    let original = original_math(config, &case)?;
    let mut signed = Vec::new();
    signed
        .try_reserve_exact(GROUP_RECORDS)
        .map_err(|_| "Koblitz causal prefix allocation")?;
    let mut sequences = [0u64; GROUP_RECORDS];
    let mut observed = capture::CaptureCursor::initial(&binding)?;
    before_visit()?;
    let custody_result =
        capture::verify_live_committed_prefix_visit(dir, &binding, &limits, &mut |line| {
            observed.byte_end = observed
                .byte_end
                .checked_add(line.len() as u64)
                .filter(|n| *n <= PROJECTOR_STREAM_BYTES)
                .ok_or("Koblitz causal observed stream cap")?;
            observed.event_count = observed
                .event_count
                .checked_add(1)
                .ok_or("Koblitz causal observed event overflow")?;
            // Hash the exact checked raw record, including its newline.
            observed.last_event_sha256 = crate::ml_dataset::digest(line);
            let row: capture::EventRecord =
                serde_json::from_slice(line).map_err(|e| e.to_string())?;
            let Some(progress) = signed_capture_event(row.event)? else {
                return Ok(());
            };
            if signed.len() == GROUP_RECORDS {
                return Err("Koblitz causal signed event count exceeds cap".into());
            }
            sequences[signed.len()] = row.sequence;
            signed.push(progress);
            Ok(())
        });
    // Restore a controlled fixture even when the visitor itself rejects it.
    // No callback to the exporter has occurred yet.
    after_visit()?;
    let custody = custody_result?;
    check_projection_watermark(head.as_ref(), &observed, custody.is_some())?;
    if custody.is_none() && !signed.is_empty() {
        return Err("Koblitz causal observations lack committed custody".into());
    }
    // Do not release a visitor observation if files changed during admission.
    storage::verify_manifest(dir, artifacts)?;
    if let Some(expected) = &head {
        let bytes = read_bounded(dir, "live/committed.json", 64 * 1024)?;
        let actual: capture::CommittedHead =
            serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
        if actual != *expected {
            return Err("Koblitz causal committed head changed".into());
        }
    } else if head_path.exists() {
        return Err("Koblitz causal committed head appeared during admission".into());
    }
    let mut summary = project_signed(
        &original,
        config.twist,
        &signed,
        &sequences[..signed.len()],
        emit,
    )?;
    summary["source_binding_sha256"] = json!(binding_sha256);
    summary["source_committed_head"] = json!(head);
    // Summary is restricted provenance; callers must not merge it into features.
    Ok(summary)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cached_bytes_are_bound_even_when_current_disk_files_verify() {
        let root = std::env::temp_dir().join(format!(
            "koblitz-cached-original-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        for (path, role) in [
            ("public.json", "public-instance"),
            ("factor-base.json", "factor-base"),
            ("capture-binding.json", "partial-capture-binding"),
            ("capture-limits.json", "partial-capture-limits"),
        ] {
            let original = b"{\"public_scalar\":\"11\"}";
            let stale = b"{\"public_scalar\":\"12\"}";
            let descriptor = storage::write_bytes(&root, path, role, "json", original).unwrap();
            let manifest = vec![descriptor.clone()];
            // The current file and its manifest verify; only the retained parse
            // buffer differs, modeling a read before the file was restored.
            storage::verify_manifest(&root, &manifest).unwrap();
            authenticate_cached_original(original, &manifest, path, role).unwrap();
            assert!(authenticate_cached_original(stale, &manifest, path, role).is_err());
            assert!(authenticate_cached_original(
                &original[..original.len() - 1],
                &manifest,
                path,
                role
            )
            .is_err());
            assert!(authenticate_cached_original(
                original,
                &[descriptor.clone(), descriptor.clone()],
                path,
                role
            )
            .is_err());
            let mut wrong = descriptor;
            wrong.role = "wrong-role".into();
            assert!(authenticate_cached_original(original, &[wrong], path, role).is_err());
        }
        std::fs::remove_dir_all(root).unwrap();
    }
    #[test]
    fn group_event_namespace_and_phase_are_strict_and_scalars_are_decimal_strings() {
        let original = construction_tests::fixture();
        let first = events(&original).remove(0);
        let valid =
            json!({"kind":"koblitz_m3","payload":{"phase":"signed-domain","progress":first}});
        assert!(signed_capture_event(valid.clone()).unwrap().is_some());
        let mut native = valid.clone();
        native["kind"] = "koblitz_m3_native".into();
        assert!(signed_capture_event(native).unwrap().is_none());
        let mut bad = valid.clone();
        bad["payload"]["phase"] = "construction".into();
        assert!(signed_capture_event(bad).is_err());
        let mut bad = valid;
        bad["payload"]["private_witness"] = "not an allowed field".into();
        assert!(signed_capture_event(bad).is_err());
        let projected = static_inputs(&original, false);
        assert_eq!(projected["target"]["x"], original.target.0.to_string());
        assert_eq!(projected["target"]["y"], original.target.1.to_string());
        assert!(projected["curve"]["a"].is_string());
        assert!(projected["x_codes"][0].is_string());
        assert_eq!(projected["n"], original.curve.m);
    }

    fn events(original: &MathInput) -> Vec<SignedPrefix> {
        use crate::cryptanalysis::koblitz_fast_arith::FastBinaryCurve;
        use num_traits::ToPrimitive;
        let curve = FastBinaryCurve::new(
            &original.curve.irreducible,
            original.curve.a.to_biguint().to_u64().unwrap(),
        )
        .unwrap();
        let mut result = Vec::new();
        for tuple in 0..64usize {
            let indices = [tuple / 16, (tuple / 4) % 4, tuple % 4];
            let mut prefix = None;
            for summand in 0..3 {
                prefix = curve.add(prefix, Some(original.signed[indices[summand]]));
                result.push(SignedPrefix {
                    sequence: result.len() as u16,
                    tuple: tuple as u8,
                    indices: indices.map(|i| i as u8),
                    summand: summand as u8,
                    prefix,
                    matches_target: summand == 2 && prefix == Some(original.target),
                });
            }
        }
        result
    }
    fn collect(original: &MathInput, events: &[SignedPrefix]) -> (Vec<Value>, Vec<Value>, Value) {
        let captures: Vec<_> = (1..=events.len() as u64).collect();
        let (mut features, mut labels) = (Vec::new(), Vec::new());
        let summary = project_signed(original, false, events, &captures, &mut |f, l| {
            features.push(f.unwrap());
            labels.push(l);
            Ok(())
        })
        .unwrap();
        (features, labels, summary)
    }
    #[test]
    fn prefix_features_are_suffix_invariant_and_use_pre_addition_accumulators() {
        let mut original = construction_tests::fixture();
        original.target = original.signed[0];
        let rows = events(&original);
        let (full, labels, summary) = collect(&original, &rows);
        assert_eq!(summary["group_examples"], 192);
        assert_eq!(summary["matrix_projection_status"], "not_implemented");
        assert!(rows.iter().any(|r| r.prefix.is_none()));
        assert!(labels.iter().any(|r| r["label"]["matches_target"] == true));
        for count in [0, 1, 2, 3, 19, 63, 191, 192] {
            let (prefix, _, summary) = collect(&original, &rows[..count]);
            assert_eq!(prefix, full[..count]);
            assert_eq!(summary["unknown_group_suffix"], json!(count < 192));
        }
        for (i, feature) in full.iter().enumerate() {
            let expected = if i % 3 == 0 { None } else { rows[i - 1].prefix };
            assert_eq!(
                feature["prefix_state"]["accumulator"],
                json!(expected.map(DecimalPoint::from))
            );
            assert_eq!(feature["prefix_state"]["infinity"], expected.is_none());
            let encoded = serde_json::to_string(feature).unwrap();
            for forbidden in [
                "matches_target",
                "source_capture_sequence",
                "known_answer_scalar",
                "certificate",
                "solver_completed",
                "first_receipt",
            ] {
                assert!(!encoded.contains(forbidden), "{forbidden}");
            }
        }
        // Provenance sequences shift when another event (e.g. witness publication)
        // is interleaved. They must never change mathematical feature identity.
        let shifted: Vec<_> = (100..100 + rows.len() as u64).collect();
        let mut features = Vec::new();
        project_signed(&original, false, &rows, &shifted, &mut |f, _| {
            features.push(f.unwrap());
            Ok(())
        })
        .unwrap();
        assert_eq!(features, full);
    }
    #[test]
    fn forged_late_group_state_and_over_cap_emit_no_provisional_features() {
        let original = construction_tests::fixture();
        let rows = events(&original);
        for mutation in 0..5 {
            let mut changed = rows.clone();
            match mutation {
                0 => changed[191].matches_target = !changed[191].matches_target,
                1 => changed[191].indices[2] ^= 1,
                2 => changed[191].prefix = None,
                3 => changed[191].sequence = 190,
                _ => changed.push(changed[191].clone()),
            }
            // Ensure the chosen sum mutation is actually different.
            if mutation == 2 && rows[191].prefix.is_none() {
                changed[191].prefix = Some(original.target);
            }
            let captures: Vec<_> = (1..=changed.len() as u64).collect();
            let mut emitted = 0;
            assert!(
                project_signed(&original, false, &changed, &captures, &mut |_, _| {
                    emitted += 1;
                    Ok(())
                })
                .is_err()
            );
            assert_eq!(emitted, 0);
        }
    }
}
