//! Historical ordinary N3 semantic identity admission. No feature emission.
use super::*;
use crate::cryptanalysis::pq_groebner_f2::F2BoolPoly;
#[path = "boolean_identity.rs"]
mod identity;
use std::io::Write;
#[derive(serde::Serialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub(super) enum Assessment {
    Absent,
    Unknown {
        reason: Unknown,
    },
    Identity {
        target_degree: Option<usize>,
        max_quotient_summand_degree: Option<usize>,
        max_before_quotient_summand_degree: Option<usize>,
        zero_target: bool,
        native_operation_witness: bool,
        producer_execution_verified: bool,
        training_export_admitted: bool,
    },
    Rejected {
        kind: &'static str,
    },
}
#[derive(serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub(super) enum Unknown {
    ReplayBudget,
    ReplayDeadline,
    MissingCheck,
    MissingAnchor,
    SourceNotReached,
    ProducerRefusal,
    RecordedUnknown,
    PostCheckRefusal,
    MissingInput,
}
fn limits() -> identity::Limits {
    identity::Limits {
        variables: 3,
        generators: 61,
        polynomial_terms: 8,
        proof_terms: 128,
        accumulator_entries: 8,
        caps: [
            16384, 262144, 32768, 32768, 65536, 32768, 256, 2097152, 524288,
        ],
    }
}
// Versioned producer checker profile. v1 retains the historical full checker.
fn produced_limits_v2() -> identity::Limits {
    identity::Limits {
        variables: 3,
        generators: 61,
        polynomial_terms: 8,
        proof_terms: 8,
        accumulator_entries: 8,
        caps: [4096, 32768, 4096, 4096, 4096, 4096, 16, 262144, 65536],
    }
}
fn checker_quote_v2() -> (u64, u64) {
    let caps = produced_limits_v2();
    let arithmetic = caps
        .caps
        .iter()
        .enumerate()
        .filter(|(i, _)| *i != identity::Resource::AllocationBytes as usize)
        .map(|(_, v)| v)
        .sum::<u64>();
    (
        arithmetic + 8 * 4096 + 16384,
        caps.caps[identity::Resource::AllocationBytes as usize] + 65536,
    )
}
fn checked_read(dir: &std::path::Path, a: &Artifact, cap: usize) -> Result<Vec<u8>, String> {
    let bytes = read(dir, a, cap)?;
    if bytes.len() as u64 != a.bytes || crate::ml_dataset::digest(&bytes) != a.sha256 {
        return Err("Boolean replay bytes differ from admitted manifest".into());
    }
    Ok(bytes)
}
fn original_member(
    artifacts: &[Artifact],
    path: &str,
    role: &str,
    bytes: &[u8],
) -> Result<(), String> {
    let mut rows = artifacts.iter().filter(|a| a.path == path);
    let a = rows
        .next()
        .ok_or("Boolean original manifest member absent")?;
    if rows.next().is_some()
        || a.role != role
        || a.encoding != "json"
        || a.bytes != bytes.len() as u64
        || a.sha256 != crate::ml_dataset::digest(bytes)
    {
        return Err("Boolean cached original manifest identity mismatch".into());
    }
    Ok(())
}
fn decode_hex(v: &Value, cap: usize) -> Result<Vec<u8>, String> {
    let s = v.as_str().ok_or("Boolean wire hex absent")?;
    if s.len() > cap * 2 {
        return Err("Boolean wire decode cap".into());
    }
    let bytes = hex::decode(s).map_err(|_| "Boolean wire hex malformed")?;
    if hex::encode(&bytes) != s {
        return Err("Boolean wire hex noncanonical".into());
    }
    Ok(bytes)
}
fn coefficients(p: &F2BoolPoly) -> Result<u8, String> {
    if p.n_vars != 3 || p.terms.len() > 8 {
        return Err("Boolean retained row shape".into());
    }
    let mut bits = 0u8;
    for term in &p.terms {
        if term.mask > 7 || bits & (1 << term.mask) != 0 {
            return Err("Boolean retained row mask".into());
        }
        bits |= 1 << term.mask;
    }
    Ok(bits)
}
fn polynomial(out: &mut Vec<u8>, bits: u8) {
    write!(out, " {}", bits.count_ones()).expect("prepaid Vec writer");
    for mask in [0u8, 1, 3, 7, 5, 2, 6, 4] {
        if bits & (1 << mask) == 0 {
            continue;
        }
        write!(out, " {}", mask.count_ones()).expect("Vec writer");
        for v in 0..3 {
            if mask & (1 << v) != 0 {
                write!(out, " {v}").expect("Vec writer");
            }
        }
    }
}
fn expected_prefix(g: &[u8], target: u8) -> Result<Vec<u8>, String> {
    if g.is_empty() || g.len() > 61 {
        return Err("Boolean original coordinate count".into());
    }
    // At most61 polynomials of8 three-variable monomials, plus target/header.
    let mut out = Vec::new();
    out.try_reserve_exact(8192)
        .map_err(|_| "Boolean prefix allocation")?;
    write!(out, "BQI1 3 {}", g.len()).expect("Vec writer");
    for &p in g {
        polynomial(&mut out, p);
    }
    polynomial(&mut out, target);
    assert!(out.len() < 8192);
    Ok(out)
}
fn ledger(v: &Value, original: OriginalLimits) -> Result<SavedBudgetLedger, String> {
    let bytes = serde_json::to_vec(v).map_err(|e| e.to_string())?;
    let assessment = verify_budget_ledger(&bytes, original)?;
    if assessment.reported_over_limit {
        return Err("Boolean receipt ledger exceeds original allowance".into());
    }
    serde_json::from_slice(&bytes).map_err(|e| e.to_string())
}
fn sticky_stop(before: Option<Stop>, after: Option<Stop>) -> Result<(), String> {
    if before.is_some() && before != after {
        return Err("Boolean cumulative ledger cleared or replaced first stop".into());
    }
    Ok(())
}
fn successful_interval(
    before: &SavedBudgetLedger,
    after: &SavedBudgetLedger,
) -> Result<(), String> {
    monotone(before, after)?;
    if before.phase != Phase::MatrixComplete
        || after.phase != Phase::MatrixComplete
        || before.first_stop.is_some()
        || after.first_stop.is_some()
    {
        return Err(
            "Boolean successful checker interval is stopped or outside MatrixComplete".into(),
        );
    }
    Ok(())
}
#[cfg(test)]
pub(super) fn test_successful_interval(before: &Value, after: &Value) -> Result<(), String> {
    let before = serde_json::from_value(before.clone()).map_err(|e| e.to_string())?;
    let after = serde_json::from_value(after.clone()).map_err(|e| e.to_string())?;
    successful_interval(&before, &after)
}
fn monotone(before: &SavedBudgetLedger, after: &SavedBudgetLedger) -> Result<(), String> {
    sticky_stop(before.first_stop, after.first_stop)?;
    if before.original_limits != after.original_limits
        || after.checking_reserved_work < before.checking_reserved_work
        || after.adapter_reserved_bytes < before.adapter_reserved_bytes
        || after.native_reserved.work < before.native_reserved.work
        || after.native_reserved.bytes < before.native_reserved.bytes
        || after.charged_trace_bytes < before.charged_trace_bytes
        || after.group_additions < before.group_additions
    {
        return Err("Boolean cumulative ledger moves backwards".into());
    }
    Ok(())
}
// This entry is private to original ordinary central replay. It independently
// rechecks input equations against cached originals using the SAME cumulative
// historical admission. Historical charges never substitute producer charges.
#[allow(clippy::too_many_arguments)]
pub(super) fn verify_optional(
    dir: &std::path::Path,
    artifacts: &[Artifact],
    binding: &capture::AttemptBinding,
    original_limits: OriginalLimits,
    capture_limits: &capture::CaptureLimits,
    config_bytes: &[u8],
    public_bytes: &[u8],
    factor_bytes: &[u8],
    input_bytes: Option<&[u8]>,
    terminal: Option<&SavedResult>,
    admission: &mut ReplayAdmission,
    mut projection: Option<&mut Option<Projection>>,
) -> Result<Assessment, String> {
    let anchor_root = root(artifacts, Slot::BooleanAnchor)?;
    let check_root = root(artifacts, Slot::BooleanCheck)?;
    if anchor_root.is_none() && check_root.is_none() {
        return Ok(Assessment::Absent);
    }
    // Covers bounded JSON/original decode, comparison, ledger checks, hex and
    // canonical-prefix construction. Actual stream/manifest/checker work below
    // is separately reserved; no per-row/retry admission reset.
    if admission
        .reserve(Quote {
            work: 4 * 1024 * 1024,
            bytes: 2 * 1024 * 1024,
        })
        .is_err()
    {
        return Ok(Assessment::Unknown {
            reason: Unknown::ReplayBudget,
        });
    }
    if admission.check_deadline().is_err() {
        return Ok(Assessment::Unknown {
            reason: Unknown::ReplayDeadline,
        });
    }
    let Some(check_root) = check_root else {
        return Ok(Assessment::Unknown {
            reason: Unknown::MissingCheck,
        });
    };
    let check_bytes = checked_read(dir, check_root, 49152)?;
    let check: Value = serde_json::from_slice(&check_bytes).map_err(|e| e.to_string())?;
    let version2 = check["schema"] == "crypto.ml-dataset.binary-m3-boolean-check/v2";
    let requested_config:CaseConfig=serde_json::from_slice(config_bytes).map_err(|e|e.to_string())?;
    if let Some(channels)=requested_config.channels.as_ref() {
        let Some(crate::ml_dataset::types::BooleanRequest::Produce{row_index})=channels.boolean_consequence.as_ref() else {
            return Err("Boolean receipt was not requested by frozen channel config".into());
        };
        if !version2 || check["requested_row_index"] != *row_index || !check["supplied_proof_suffix_hex"].as_str().is_some_and(str::is_empty) {
            return Err("Boolean receipt differs from frozen production request".into());
        }
    }

    if (!version2 && check["schema"] != "crypto.ml-dataset.binary-m3-boolean-check/v1")
        || check["binding"] != serde_json::to_value(binding).map_err(|e| e.to_string())?
        || check["native_operation_witness"] != false
        || check["training_export_admitted"] != false
    {
        return Err("Boolean receipt original binding or unsupported authority".into());
    }
    let _supplied = decode_hex(&check["supplied_proof_suffix_hex"], 4096)?;
    if check["reason"] == "original_target_not_reached" {
        ledger(&check["ledger"], original_limits)?;
        if !check["attempt"].is_null() {
            return Err("Boolean unreached source has target attempt".into());
        }
        return Ok(Assessment::Unknown {
            reason: Unknown::SourceNotReached,
        });
    }
    let outer_before = ledger(&check["before"], original_limits)?;
    let outer_after = ledger(&check["after"], original_limits)?;
    monotone(&outer_before, &outer_after)?;
    let Some(anchor_root) = anchor_root else {
        return Ok(Assessment::Unknown {
            reason: Unknown::MissingAnchor,
        });
    };
    let Some(input_bytes) = input_bytes else {
        return Ok(Assessment::Unknown {
            reason: Unknown::MissingInput,
        });
    };
    original_member(
        artifacts,
        "config.json",
        "restricted-provenance",
        config_bytes,
    )?;
    original_member(artifacts, "public.json", "public-instance", public_bytes)?;
    original_member(artifacts, "factor-base.json", "factor-base", factor_bytes)?;
    let admitted_math = match verify_input_owned(
        config_bytes,
        public_bytes,
        factor_bytes,
        binding,
        original_limits,
        input_bytes,
        admission,
    ) {
        Ok(math) => math,
        Err(Failure::Admission) => {
            return Ok(Assessment::Unknown {
                reason: if Instant::now() >= admission.deadline {
                    Unknown::ReplayDeadline
                } else {
                    Unknown::ReplayBudget
                },
            })
        }
        Err(e) => {
            return Err(format!(
                "Boolean independently admitted original equations: {e:?}"
            ))
        }
    };
    let saved: InputArtifact = serde_json::from_slice(input_bytes).map_err(|e| e.to_string())?;
    let construction: ConstructionArtifact =
        serde_json::from_str(&saved.construction_json).map_err(|e| e.to_string())?;
    let evidence = construction
        .evidence
        .ok_or("Boolean input has no checked coordinate ANFs")?;
    let input_hash: [u8; 32] = sha2::Sha256::digest(input_bytes).into();
    let anchor_bytes = checked_read(dir, anchor_root, 8192)?;
    let anchor: SavedResult = serde_json::from_slice(&anchor_bytes).map_err(|e| e.to_string())?;
    if anchor.schema != "crypto.ml-dataset.binary-m3-matrix-result/v4"
        || anchor.input_sha256 != input_hash
        || anchor.source_completion_claimed
        || anchor.ledger.phase != Phase::MatrixComplete
        || anchor.ledger.first_stop.is_some()
        || anchor.input_capture_sequence == 0
        || anchor.unacknowledged_event_tail_bytes != 0
        || anchor.retained_event_bytes != anchor.acknowledged_event_prefix_bytes
        || anchor.event_accounting.acknowledged_wire_bytes
            != anchor.acknowledged_event_prefix_bytes as u64
    {
        return Err("Boolean anchor original/phase/retained-prefix mismatch".into());
    }
    let anchored_ledger = ledger(
        &serde_json::to_value(&anchor.ledger).map_err(|e| e.to_string())?,
        original_limits,
    )?;
    monotone(&anchored_ledger, &outer_after)?;
    if check["before"] != serde_json::to_value(&anchor.ledger).map_err(|e| e.to_string())? {
        return Err("Boolean anchor/outer ledger join mismatch".into());
    }
    let report = anchor
        .report
        .as_ref()
        .ok_or("Boolean anchor lacks actual report")?;
    if !report.native_step_complete
        || (report.rows.len() > 8 || report.columns > 8 || report.rank.is_some_and(|r| r > 8))
    {
        return Err("Boolean anchor is not a complete bounded matrix report".into());
    }
    if let Some(terminal) = terminal {
        if serde_json::to_value(&terminal.report).map_err(|e| e.to_string())?
            != serde_json::to_value(&anchor.report).map_err(|e| e.to_string())?
        {
            return Err(
                "Boolean anchor target report differs from terminal retained report".into(),
            );
        }
        monotone(&anchored_ledger, &terminal.ledger)?;
    }
    if check["input_sha256"] != serde_json::to_value(input_hash).map_err(|e| e.to_string())?
        || check["input_capture_sequence"] != anchor.input_capture_sequence
        || check["original_public_sha256"]
            != serde_json::to_value(saved.original_public_sha256).map_err(|e| e.to_string())?
        || check["original_factor_base_sha256"]
            != serde_json::to_value(saved.original_factor_base_sha256).map_err(|e| e.to_string())?
    {
        return Err("Boolean check/anchor/original identity mismatch".into());
    }
    let produced = check["derivation_origin"] == "separately_searched_boolean_generator_identity";
    if version2
        && (!produced
            || (check["producer"]["status"] != "producer_admission_refused"
                && check["producer"]["checker_profile"] != "bounded-n3-proof8/v2"))
    {
        return Err("Boolean v2 producer checker profile mismatch".into());
    }
    if !version2 && !check["producer"]["checker_profile"].is_null() {
        return Err("Boolean v1 receipt cannot assert a newer checker profile".into());
    }
    let (check_work, check_bytes) = if version2 {
        checker_quote_v2()
    } else {
        (3_064_064, 6_291_456)
    };

    let attempt = if produced {
        if check["producer"]["status"] != "producer_attempt" || check["producer"]["check"].is_null()
        {
            return Ok(Assessment::Unknown {
                reason: Unknown::ProducerRefusal,
            });
        }
        let p = &check["producer"];
        for (key, expected) in [
            ("input_sha256", check["input_sha256"].clone()),
            (
                "input_capture_sequence",
                check["input_capture_sequence"].clone(),
            ),
            ("anchor_sha256", Value::String(anchor_root.sha256.clone())),
            ("row_index", check["requested_row_index"].clone()),
            (
                "selector_codes",
                serde_json::to_value(saved.codes).map_err(|e| e.to_string())?,
            ),
        ] {
            if p[key] != expected {
                return Err("Boolean producer target binding mismatch".into());
            }
        }
        let a = ledger(&p["before"], original_limits)?;
        let b = ledger(&p["after_producer"], original_limits)?;
        let c = ledger(&p["after_check"], original_limits)?;
        monotone(&a, &b)?;
        monotone(&b, &c)?;
        if p["before"] != check["before"]
            || p["after_check"] != check["after"]
            || p["after_producer"] != p["check"]["before"]
            || p["after_check"] != p["check"]["after"]
        {
            return Err("Boolean producer/check cumulative ledger join mismatch".into());
        }
        let candidates = 8 * evidence.coordinate_anfs.len() as u64;
        let quoted_work = 128 + 8 * candidates + 168 * (candidates + 1);
        if p["reserved_work"] != quoted_work
            || p["reserved_bytes"] != 4096u64
            || b.checking_reserved_work - a.checking_reserved_work != 2_097_152 + quoted_work
            || b.adapter_reserved_bytes - a.adapter_reserved_bytes != 4_194_304 + 4096
        {
            return Err("Boolean producer reservation quote mismatch".into());
        }
        &p["check"]
    } else {
        if check["derivation_origin"] != "separately_supplied_boolean_generator_identity" {
            return Err("Boolean derivation origin unknown".into());
        }
        &check["attempt"]
    };
    if attempt.is_null() {
        return Ok(Assessment::Unknown {
            reason: Unknown::ProducerRefusal,
        });
    }
    let before = ledger(&attempt["before"], original_limits)?;
    let after = ledger(&attempt["after"], original_limits)?;
    monotone(&before, &after)?;
    if !produced && (attempt["before"] != check["before"] || attempt["after"] != check["after"]) {
        return Err("Boolean supplied-check cumulative ledger join mismatch".into());
    }
    for key in [
        "native_reserved",
        "charged_trace_bytes",
        "group_additions",
        "checking_reported_work",
    ] {
        if attempt["before"][key] != attempt["after"][key] {
            return Err("Boolean checker changed unrelated ledger reservation".into());
        }
    }
    let outcome = &attempt["outcome"];
    if outcome["status"] != "arithmetic_audit" {
        return Ok(Assessment::Unknown {
            reason: Unknown::PostCheckRefusal,
        });
    }
    successful_interval(&before, &after)?;
    if after.checking_reserved_work - before.checking_reserved_work != check_work
        || after.adapter_reserved_bytes - before.adapter_reserved_bytes != check_bytes
    {
        return Err("Boolean checker original shared reservation mismatch".into());
    }
    let row = check["requested_row_index"]
        .as_u64()
        .and_then(|n| usize::try_from(n).ok())
        .ok_or("Boolean row index")?;
    let target = report
        .rows
        .get(row)
        .ok_or("Boolean requested retained row absent")?;
    if outcome["input_sha256"] != check["input_sha256"]
        || outcome["input_capture_sequence"] != anchor.input_capture_sequence
        || outcome["anchor_sha256"] != anchor_root.sha256
        || outcome["row_index"] != check["requested_row_index"]
    {
        return Err("Boolean audited target identity mismatch".into());
    }
    let position: capture::CommittedEventPosition =
        serde_json::from_value(outcome["watermark"].clone()).map_err(|e| e.to_string())?;
    if produced && check["producer"]["watermark"] != outcome["watermark"] {
        return Err("Boolean producer/check watermark mismatch".into());
    }
    if position.binding_sha256 != binding.digest()?
        || position.event_count <= anchor.input_capture_sequence
    {
        return Err("Boolean capture position precedes original input".into());
    }
    let prefix = expected_prefix(&evidence.coordinate_anfs, coefficients(target)?)?;
    let wire = decode_hex(&outcome["wire_hex"], 16384)?;
    if !wire.starts_with(&prefix) {
        return Err("Boolean wire substitutes original generator order or retained target".into());
    }
    if !produced && wire[prefix.len()..] != decode_hex(&check["supplied_proof_suffix_hex"], 4096)? {
        return Err("Boolean supplied proof suffix differs from checked wire".into());
    }
    if !verify_capture_and_stability(
        dir,
        artifacts,
        binding,
        capture_limits,
        &anchor,
        &position,
        admission,
    )? {
        return Ok(Assessment::Unknown {
            reason: if Instant::now() >= admission.deadline {
                Unknown::ReplayDeadline
            } else {
                Unknown::ReplayBudget
            },
        });
    }
    let recorded = &outcome["audit"]["result"];
    if outcome["audit"]["native_operation_witness"] != false
        || outcome["audit"]["source_authenticated_by_arithmetic_checker"] != false
    {
        return Err("Boolean arithmetic receipt asserts unsupported authority".into());
    }
    if recorded["kind"] == "unknown" {
        return Ok(Assessment::Unknown {
            reason: Unknown::RecordedUnknown,
        });
    }
    let caps = if version2 {
        produced_limits_v2()
    } else {
        limits()
    };
    if admission
        .reserve(Quote {
            work: check_work,
            bytes: check_bytes,
        })
        .is_err()
    {
        return Ok(Assessment::Unknown {
            reason: Unknown::ReplayBudget,
        });
    }
    let audit = identity::verify_untrusted(&wire, caps, admission.deadline);
    match audit.result {
        Err(identity::Failure::Unknown(identity::Exhaustion::Deadline)) => {
            return Ok(Assessment::Unknown {
                reason: Unknown::ReplayDeadline,
            })
        }
        Err(identity::Failure::Unknown(_)) => {
            return Ok(Assessment::Unknown {
                reason: Unknown::ReplayBudget,
            })
        }
        _ => {}
    }
    if outcome["audit"]["charges"]
        != serde_json::to_value(audit.charges.used).map_err(|e| e.to_string())?
    {
        return Err("Boolean deterministic checker charges differ".into());
    }
    if admission.check_deadline().is_err() {
        return Ok(Assessment::Unknown {
            reason: Unknown::ReplayDeadline,
        });
    }
    storage::verify_manifest(dir, artifacts)?;
    match audit.result {
        Ok(d) => {
            let expected = serde_json::json!({"kind":"equal","target_degree":d.target,
                "max_quotient_summand_degree":d.max_quotient_summand,
                "max_before_quotient_summand_degree":d.max_before_quotient_summand});
            if *recorded != expected {
                return Err(
                    "Boolean saved equality/degree claim differs from independent checker".into(),
                );
            }
            if let Some(slot)=projection.as_deref_mut() {
                let final_custody_work=artifacts.iter().try_fold(0u64,|sum,a|sum.checked_add(a.bytes))
                    .and_then(|bytes|bytes.checked_mul(16)).and_then(|work|work.checked_add(2_097_152))
                    .ok_or("Boolean final custody quote overflow")?;
                if let Err(reason)=reserve_projection_output(admission,final_custody_work) {
                    return Ok(Assessment::Unknown{reason});
                }
                let config:CaseConfig=serde_json::from_slice(config_bytes).map_err(|e|e.to_string())?;
                let feature=if d.target.is_some() {
                    Some(PublicEquations::from_admitted(&admitted_math,&config,&evidence.coordinate_anfs)?)
                } else {None};
                let record=RestrictedEquationRecord {
                    schema:"crypto.ml-dataset.boolean-equation-record/v1",
                    status:if d.target.is_some(){EquationStatus::VerifiedNonzero}else{EquationStatus::VerifiedZero},
                    assessment:expected.clone(),
                    target:Some(polynomial_terms(coefficients(target)?)),
                    receipt:Some(check.clone()), native_operation_witness:false,producer_execution_verified:false,
                };
                // Bound both retained output objects before granting the private seal.
                bounded_projection_json(&feature,32_768)?;
                bounded_projection_json(&record,65_536)?;
                if admission.check_deadline().is_err(){return Ok(Assessment::Unknown{reason:Unknown::ReplayDeadline});}
                storage::verify_manifest(dir,artifacts)?;
                if admission.check_deadline().is_err(){return Ok(Assessment::Unknown{reason:Unknown::ReplayDeadline});}
                *slot=Some(Projection{feature,record,deadline:admission.deadline,needs_final_custody:true});
            }
            Ok(Assessment::Identity {
                target_degree: d.target,
                max_quotient_summand_degree: d.max_quotient_summand,
                max_before_quotient_summand_degree: d.max_before_quotient_summand,
                zero_target: d.target.is_none(),
                native_operation_witness: false,
                producer_execution_verified: false,
                training_export_admitted: false,
            })
        }
        Err(identity::Failure::EqualityMismatch) if recorded["kind"] == "equality_mismatch" => {
            Ok(Assessment::Rejected {
                kind: "equality_mismatch",
            })
        }
        Err(identity::Failure::Malformed(s))
            if recorded["kind"] == "malformed" && recorded["syntax"] == format!("{s:?}") =>
        {
            Ok(Assessment::Rejected { kind: "malformed" })
        }
        _ => Err("Boolean saved rejection differs from independent checker".into()),
    }
}

// True means the exact original input -> matrix Finish position was found in
// the authenticated committed stream. False is a typed replay-budget unknown.
#[allow(clippy::too_many_arguments)]
fn verify_capture_and_stability(
    dir: &std::path::Path,
    artifacts: &[Artifact],
    binding: &capture::AttemptBinding,
    limits: &capture::CaptureLimits,
    anchor: &SavedResult,
    position: &capture::CommittedEventPosition,
    admission: &mut ReplayAdmission,
) -> Result<bool, String> {
    let member = |path: &str| -> Result<&Artifact, String> {
        let mut found = artifacts.iter().filter(|a| a.path == path);
        let a = found
            .next()
            .ok_or("Boolean committed stream manifest member missing")?;
        if found.next().is_some() {
            return Err("Boolean duplicate committed stream member".into());
        }
        Ok(a)
    };
    let head_artifact = member("live/committed.json")?;
    let stream_artifact = member("live/events.jsonl")?;
    let total = artifacts
        .iter()
        .try_fold(0u64, |n, a| n.checked_add(a.bytes));
    let work = total
        .and_then(|n| n.checked_mul(16))
        .and_then(|n| {
            stream_artifact
                .bytes
                .checked_mul(16)
                .and_then(|s| n.checked_add(s))
        })
        .and_then(|n| n.checked_add(1024 * 1024));
    let Some(work) = work else {
        return Ok(false);
    };
    if admission
        .reserve(Quote {
            work,
            bytes: 1024 * 1024,
        })
        .is_err()
    {
        return Ok(false);
    }
    let head_before = checked_read(dir, head_artifact, 8192)?;
    let head: capture::CommittedHead =
        serde_json::from_slice(&head_before).map_err(|e| e.to_string())?;
    if head.byte_end > stream_artifact.bytes
        || position.byte_end > head.byte_end
        || position.event_count > head.event_count
    {
        return Err("Boolean watermark outside committed prefix".into());
    }
    let (mut offset, mut count, mut native_bytes) = (0u64, 0u64, 0u64);
    let (mut found, mut expired) = (false, false);
    let prefix = capture::verify_live_committed_prefix_visit(dir, binding, limits, &mut |line| {
        if admission.check_deadline().is_err() {
            expired = true;
            return Err("Boolean replay deadline".into());
        }
        offset = offset
            .checked_add(line.len() as u64)
            .ok_or("Boolean stream byte overflow")?;
        let wire: capture::EventRecord = serde_json::from_slice(line).map_err(|e| e.to_string())?;
        if wire.event["kind"] == "binary_m3_native" {
            let event: TypedOrdinary =
                serde_json::from_value(wire.event.clone()).map_err(|e| e.to_string())?;
            if wire.sequence <= position.event_count {
                if event.payload.input_sha256 != anchor.input_sha256
                    || event.payload.input_capture_sequence != anchor.input_capture_sequence
                    || event.payload.binding_sha256 != position.binding_sha256
                    || event.payload.capture_sequence != wire.sequence
                {
                    return Err("Boolean pre-anchor native event identity mismatch".into());
                }
                if !matches!(event.payload.event, SavedOrdinaryBoundary::Matrix { .. }) {
                    return Err("Boolean anchor appears after signed execution".into());
                }
                count += 1;
                native_bytes += line.len() as u64;
            } else if matches!(event.payload.event, SavedOrdinaryBoundary::Matrix { .. }) {
                return Err("Boolean matrix suffix after claimed Finish anchor".into());
            }
            if wire.sequence == position.event_count {
                if !matches!(
                    event.payload.event,
                    SavedOrdinaryBoundary::Matrix {
                        snapshot: native::Event {
                            stage: native::Stage::Finish,
                            ..
                        }
                    }
                ) || offset != position.byte_end
                    || crate::ml_dataset::digest(line) != position.last_event_sha256
                {
                    return Err("Boolean anchor is not exact committed matrix Finish".into());
                }
                found = true;
            }
        } else if wire.sequence == position.event_count {
            return Err("Boolean anchor points to unrelated committed event".into());
        }
        Ok(())
    });
    if expired {
        return Ok(false);
    }
    let prefix = prefix?.ok_or("Boolean original lacks committed capture")?;
    if !found
        || count != anchor.event_accounting.acknowledged
        || native_bytes != anchor.acknowledged_event_prefix_bytes as u64
        || anchor
            .report
            .as_ref()
            .is_none_or(|r| r.counters.acknowledged_events != count)
    {
        return Err("Boolean anchor capture counts differ from actual matrix prefix".into());
    }
    let published = prefix
        .payloads
        .iter()
        .find(|(_, a)| a.path == "payloads/binary-m3-input.json")
        .ok_or("Boolean original input absent from committed payloads")?;
    if published.0 != anchor.input_capture_sequence
        || published.1.role != INPUT_ROLE
        || published.1.encoding != "json"
        || published.1.sha256 != hex::encode(anchor.input_sha256)
    {
        return Err("Boolean original input capture join mismatch".into());
    }
    if checked_read(dir, head_artifact, 8192)? != head_before {
        return Err("Boolean committed head changed during replay".into());
    }
    // The work reservation above covers this and the final post-arithmetic pass.
    storage::verify_manifest(dir, artifacts)?;
    Ok(true)
}


// Distinct typed exhaustion reasons; no deadline reset or changed allowance.
fn reserve_projection_output(admission:&mut ReplayAdmission,work:u64)->Result<(),Unknown>{
    admission.reserve(Quote{work,bytes:1_048_576}).map_err(|_|Unknown::ReplayBudget)?;
    admission.check_deadline().map_err(|_|Unknown::ReplayDeadline)
}

// Only this module can construct a public-bearing seal. No Deserialize, Clone,
// public fields or JSON-boolean admission API is provided.
pub(in crate::ml_dataset) struct Projection {
    feature:Option<PublicEquations>, record:RestrictedEquationRecord,deadline:Instant,needs_final_custody:bool,
}
#[derive(serde::Serialize)]
struct PublicEquations {
    schema:&'static str, task:&'static str, static_inputs:Value,
    ring:Ring, selectors:Vec<Selector>, source_equations:Vec<Vec<Vec<u32>>>,
}
#[derive(serde::Serialize)]
struct Ring { characteristic:u8, quotient:&'static str, variables:u32, variable_order:Vec<u32> }
#[derive(serde::Serialize)]
struct Selector { variable:u32,summand:u32,codes:[String;2] }
#[derive(serde::Serialize)]
#[serde(rename_all="snake_case")]
enum EquationStatus { VerifiedNonzero,VerifiedZero,Unknown,Rejected }
#[derive(serde::Serialize)]
struct RestrictedEquationRecord {
    schema:&'static str,status:EquationStatus,assessment:Value,
    target:Option<Vec<Vec<u32>>>,receipt:Option<Value>,
    native_operation_witness:bool,producer_execution_verified:bool,
}
fn polynomial_terms(bits:u8)->Vec<Vec<u32>> {
    [0u8,1,3,7,5,2,6,4].into_iter().filter(|m|bits&(1<<m)!=0)
        .map(|m|(0..3).filter(|i|m&(1<<i)!=0).collect()).collect()
}
impl PublicEquations {
    fn from_admitted(math:&MathInput,config:&CaseConfig,equations:&[u8])->Result<Self,String>{
        if equations.len()!=config.n as usize || equations.len()>61 {return Err("Boolean public coordinate count".into());}
        let mut initial=crate::ml_dataset::solvers::koblitz_m3::ordinary_causal_projection::static_inputs(math,config.twist);
        if let Some(policy)=&config.factor_base_policy {
            initial["source_factor_base_policy"]=crate::ml_dataset::export::policy_projection(
                &serde_json::to_value(policy).map_err(|e|e.to_string())?)?;
        }
        Ok(Self {schema:"crypto.ml-dataset.boolean-equations/v1",task:"binary-polynomial-consequence/v1",
            static_inputs:initial,ring:Ring{characteristic:2,quotient:"v_i^2+v_i",variables:3,variable_order:vec![0,1,2]},
            selectors:(0..3).map(|i|Selector{variable:i,summand:i,codes:math.codes.map(|x|x.to_string())}).collect(),
            source_equations:equations.iter().map(|&p|polynomial_terms(p)).collect()})
    }
}
fn bounded_projection_json<T:serde::Serialize>(value:&T,cap:usize)->Result<Value,String>{
    struct Writer{bytes:Vec<u8>,cap:usize}
    impl std::io::Write for Writer {
        fn write(&mut self,b:&[u8])->std::io::Result<usize>{
            if b.len()>self.cap.saturating_sub(self.bytes.len()){return Err(std::io::Error::other("projection byte cap"));}
            self.bytes.extend_from_slice(b);Ok(b.len())
        }
        fn flush(&mut self)->std::io::Result<()>{Ok(())}
    }
    let mut writer=Writer{bytes:Vec::new(),cap};
    writer.bytes.try_reserve_exact(cap).map_err(|e|e.to_string())?;
    serde_json::to_writer(&mut writer,value).map_err(|e|e.to_string())?;
    serde_json::from_slice(&writer.bytes).map_err(|e|e.to_string())
}
impl Projection {
    pub(super) fn restricted(assessment:&Assessment,deadline:Instant)->Result<Self,String>{
        let status=if matches!(assessment,Assessment::Rejected{..}){EquationStatus::Rejected}else{EquationStatus::Unknown};
        Ok(Self{feature:None,deadline,needs_final_custody:false,record:RestrictedEquationRecord{
            schema:"crypto.ml-dataset.boolean-equation-record/v1",status,
            assessment:serde_json::to_value(assessment).map_err(|e|e.to_string())?,target:None,receipt:None,
            native_operation_witness:false,producer_execution_verified:false}})
    }
    pub(in crate::ml_dataset) fn finish_admission(&mut self,dir:&std::path::Path,artifacts:&[Artifact])->Result<(),String>{
        if self.needs_final_custody {
            if Instant::now()<self.deadline {storage::verify_manifest(dir,artifacts)?;}
            if Instant::now()>=self.deadline {
                self.feature=None;self.record.status=EquationStatus::Unknown;
                self.record.assessment=serde_json::json!({"status":"unknown","reason":"final_custody_deadline"});
                self.record.target=None;self.record.receipt=None;
            }
            self.needs_final_custody=false;
        }
        Ok(())
    }
    pub(in crate::ml_dataset) fn unknown_rows()->Result<(Option<Value>,Value),String>{
        let record=RestrictedEquationRecord{schema:"crypto.ml-dataset.boolean-equation-record/v1",
            status:EquationStatus::Unknown,assessment:serde_json::json!({"status":"unknown","reason":"original_source_or_optional_check_not_reached"}),
            target:None,receipt:None,native_operation_witness:false,producer_execution_verified:false};
        Ok((None,bounded_projection_json(&record,4096)?))
    }
    pub(in crate::ml_dataset) fn into_rows(self)->Result<(Option<Value>,Value),String>{
        self.into_rows_with_clock(Instant::now)
    }
    // Production uses the original monotonic clock; the private hook lets the
    // test cross the exact final encoding boundary without sleeps or renewal.
    fn into_rows_with_clock(self,mut now:impl FnMut()->Instant)->Result<(Option<Value>,Value),String>{
        if self.needs_final_custody {return Err("Boolean final cell custody not completed".into());}
        // Prebuild the terminal censoring record within the existing prepaid
        // terminal/output allowance. No encoding/allocation follows the final
        // deadline decision on the exhausted path.
        let unknown=RestrictedEquationRecord{schema:"crypto.ml-dataset.boolean-equation-record/v1",
            status:EquationStatus::Unknown,
            assessment:serde_json::json!({"status":"unknown","reason":"projection_deadline"}),
            target:None,receipt:None,native_operation_witness:false,producer_execution_verified:false};
        let expired_rows=(None,bounded_projection_json(&unknown,4096)?);
        if now()>=self.deadline {return Ok(expired_rows);}
        let feature=self.feature.as_ref().map(|v|bounded_projection_json(v,32_768)).transpose()?;
        let record=bounded_projection_json(&self.record,65_536)?;
        if now()>=self.deadline {return Ok(expired_rows);}
        Ok((feature,record))
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn source_feature_ignores_solver_row_proof_and_retains_equation_order() {
        let config:CaseConfig=serde_json::from_value(serde_json::json!({"family":"binary","n":23,"m":3,"twist":false,
            "curve_seed":7,"generator_index":1,"key_index":1,"seed":42,"factor_base_size":4,
            "degree_bound":3,"work_limit":16384,"solver":"f4"})).unwrap();
        let case=crate::ml_dataset::curves::generate(&config).unwrap();
        let math=crate::ml_dataset::solvers::binary_m3_original::authenticate(
            crate::ml_dataset::solvers::binary_m3_original::Profile::OrdinaryBinary,&config,&case).unwrap();
        let mut equations=vec![0u8;23];equations[0]=3;equations[1]=5;
        let baseline=bounded_projection_json(&PublicEquations::from_admitted(&math,&config,&equations).unwrap(),32768).unwrap();
        let mut changed=config.clone();changed.solver="f5".into();changed.work_limit=65536;changed.degree_bound=6;
        changed.channels=Some(crate::ml_dataset::types::ChannelRequests{schema:"crypto.ml-dataset.channel-requests/v1".into(),
            boolean_consequence:Some(crate::ml_dataset::types::BooleanRequest::Produce{row_index:7}),sat_initial:None,affine_boolean_row:None});
        let same=bounded_projection_json(&PublicEquations::from_admitted(&math,&changed,&equations).unwrap(),32768).unwrap();
        assert_eq!(baseline,same);
        assert_eq!(baseline["source_equations"][22],serde_json::json!([]));
        equations.swap(0,1);
        assert_ne!(baseline,bounded_projection_json(&PublicEquations::from_admitted(&math,&config,&equations).unwrap(),32768).unwrap());
        let text=serde_json::to_string(&baseline).unwrap();
        for private in ["wire_hex","row_index","watermark","checking_reserved_work","producer","outcome"] {assert!(!text.contains(private));}
        // Restricted proof/row/capture mutations are outside the feature constructor's input type.
        let mut record=RestrictedEquationRecord{schema:"crypto.ml-dataset.boolean-equation-record/v1",status:EquationStatus::VerifiedNonzero,
            assessment:serde_json::json!({}),target:Some(vec![vec![0]]),receipt:Some(serde_json::json!({"wire_hex":"aa","row_index":0})),
            native_operation_witness:false,producer_execution_verified:false};
        let feature=PublicEquations::from_admitted(&math,&config,&vec![0u8;23]).unwrap();
        let before=bounded_projection_json(&feature,32768).unwrap();
        record.receipt=Some(serde_json::json!({"wire_hex":"bbbb","row_index":7,"watermark":99}));
        record.target=Some(vec![vec![1,2]]);
        assert_eq!(before,bounded_projection_json(&feature,32768).unwrap());
        assert!(bounded_projection_json(&record,8).is_err());
    }
    #[test]
    fn final_encoding_boundary_censors_previously_eligible_public_and_target_rows() {
        let start=Instant::now();let deadline=start+std::time::Duration::from_secs(1);
        // Synthetic encoding control, not a substitute for real identity admission.
        let make=||Projection{deadline,needs_final_custody:false,
            feature:Some(PublicEquations{schema:"crypto.ml-dataset.boolean-equations/v1",task:"binary-polynomial-consequence/v1",
                static_inputs:serde_json::json!({"family":"binary","n":23}),
                ring:Ring{characteristic:2,quotient:"v_i^2+v_i",variables:3,variable_order:vec![0,1,2]},
                selectors:vec![],source_equations:vec![vec![vec![0]]]}),
            record:RestrictedEquationRecord{schema:"crypto.ml-dataset.boolean-equation-record/v1",status:EquationStatus::VerifiedNonzero,
                assessment:serde_json::json!({"kind":"equal"}),target:Some(vec![vec![0]]),
                receipt:Some(serde_json::json!({"wire_hex":"00"})),native_operation_witness:false,producer_execution_verified:false}};
        let (feature,record)=make().into_rows_with_clock(||start).unwrap();
        assert!(feature.is_some());assert_eq!(record["status"],"verified_nonzero");
        let mut calls=0;
        let (feature,record)=make().into_rows_with_clock(||{calls+=1;if calls==1 {start}else{deadline}}).unwrap();
        assert_eq!(calls,2,"control must reach the post-encoding boundary");
        assert!(feature.is_none());assert_eq!(record["status"],"unknown");
        assert_eq!(record["assessment"]["reason"],"projection_deadline");
        assert!(record["target"].is_null());assert!(record["receipt"].is_null());
    }
    #[test]
    fn projection_resource_and_deadline_refusals_are_distinct() {
        let mut expired=ReplayAdmission{deadline:Instant::now(),work:0,bytes:0};
        assert!(matches!(reserve_projection_output(&mut expired,1),Err(Unknown::ReplayDeadline)));
        assert_eq!(expired.work,1);assert_eq!(expired.bytes,1_048_576);
        let mut exhausted=ReplayAdmission{deadline:Instant::now()+std::time::Duration::from_secs(1),work:64*1024*1024,bytes:0};
        assert!(matches!(reserve_projection_output(&mut exhausted,1),Err(Unknown::ReplayBudget)));
    }
    #[test]
    fn unknown_and_expired_seals_never_release_public_features() {
        let (feature,record)=Projection::unknown_rows().unwrap();assert!(feature.is_none());
        assert_eq!(record["status"],"unknown");assert!(record["target"].is_null());
        let seal=Projection::restricted(&Assessment::Unknown{reason:Unknown::ReplayBudget},Instant::now()).unwrap();
        let (feature,record)=seal.into_rows().unwrap();assert!(feature.is_none());assert!(record["receipt"].is_null());
    }
    #[test]
    fn first_stop_is_sticky_but_partial_failure_is_allowed() {
        assert!(sticky_stop(Some(Stop::Work), None).is_err());
        assert!(sticky_stop(Some(Stop::Work), Some(Stop::Bytes)).is_err());
        assert!(sticky_stop(Some(Stop::Work), Some(Stop::Work)).is_ok());
        assert!(sticky_stop(None, Some(Stop::Work)).is_ok());
        assert!(sticky_stop(None, None).is_ok());
    }
    #[test]
    fn canonical_hex_and_caps() {
        assert_eq!(
            decode_hex(&Value::String("00ff".into()), 2).unwrap(),
            vec![0, 255]
        );
        for bad in ["00FF", "0", "gg", "000000"] {
            assert!(decode_hex(&Value::String(bad.into()), 2).is_err());
        }
    }
    #[test]
    fn ordered_original_prefix_and_target_are_distinct() {
        let original = expected_prefix(&[0, 2, 4], 6).unwrap();
        assert_ne!(original, expected_prefix(&[2, 0, 4], 6).unwrap());
        assert_ne!(original, expected_prefix(&[0, 2, 4], 2).unwrap());
        assert!(expected_prefix(&[], 0).is_err());
        assert!(expected_prefix(&[0; 62], 0).is_err());
    }
    #[test]
    fn all_n3_polynomials_check_against_independent_arithmetic() {
        for bits in 0..=255u8 {
            let mut wire = expected_prefix(&[bits], bits).unwrap();
            wire.extend_from_slice(b" 1 0 0\n");
            let result = identity::verify_untrusted(
                &wire,
                limits(),
                Instant::now() + std::time::Duration::from_secs(1),
            );
            assert!(result.result.is_ok(), "{bits}: {:?}", result.result);
            let mut wrong = expected_prefix(&[bits], bits ^ 1).unwrap();
            wrong.extend_from_slice(b" 1 0 0\n");
            assert!(matches!(
                identity::verify_untrusted(
                    &wrong,
                    limits(),
                    Instant::now() + std::time::Duration::from_secs(1)
                )
                .result,
                Err(identity::Failure::EqualityMismatch)
            ));
        }
    }
}
