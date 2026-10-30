//! Fixed copied-control workflow; all original sources are read-only.
use super::control_custody::Recipe;
use super::*;
const SOURCE: &str = include_str!("controls.rs");
pub(super) fn source() -> &'static str {
    SOURCE
}

fn stage(slot: u64, message: &str) -> Option<&'static str> {
    match (slot, message) {
        (
            1 | 2,
            "factor base differs from expected policy/request"
            | "factor-base action, partition, metadata or domain digest differs"
            | "noncanonical orbit representative"
            | "incomplete population orbit",
        ) => Some("factor-base-policy-replay"),
        (3, "deterministic construction/counter/terminal replay differs") => {
            Some("constructor-terminal-replay")
        }
        (
            4,
            "case policy differs from exact frozen occurrence"
            | "capture attempt exact policy/plan file digest mismatch",
        ) => Some("exact-occurrence-binding"),
        (5, "legacy occurrence requires legacy factor base") => Some("expected-policy-schema"),
        (
            7,
            "alternative domain admission differs from public route/work preflight"
            | "preflight domain stop carries positive or exhaustive claim",
        ) => Some("native-route-preflight"),
        (
            8,
            "admitted PKM execution lacks separate checked domain"
            | "admitted relation execution lacks disclosed domains",
        ) => Some("required-native-domain"),
        (9, "V2 capture context missing") => Some("required-v2-capture-context"),
        (10, "capture binding differs from original attempt") => {
            Some("original-capture-binding-before-execution-context")
        }
        (
            10,
            "V2 original execution context missing" | "required original execution context missing",
        ) => Some("required-execution-context"),
        (11, "V2 protocol scope differs from complete policy plan") => {
            Some("full-policy-plan-scope")
        }
        (
            13,
            "orphan enabled ECDLP channel files in default destination"
            | "export destination belongs to enabled ECDLP profile",
        ) => Some("immutable-export-options"),
        (14, "export destination has incompatible profile") => Some("immutable-export-options"),
        (15, "orphan enabled ECDLP channel files in default destination") => {
            Some("orphan-export-channel")
        }
        (12, value) if duplicate_location(value) => Some("original-byte-duplicate-policy-field"),
        _ => None,
    }
}
fn duplicate_location(message: &str) -> bool {
    let Some(location) = message.strip_prefix("duplicate field `kind` at line ") else {
        return false;
    };
    let Some((line, column)) = location.split_once(" column ") else {
        return false;
    };
    [line, column]
        .iter()
        .all(|value| !value.is_empty() && value.bytes().all(|b| b.is_ascii_digit()))
}
fn diagnostic(slot: u64, receipt: &Value, stdout: &[u8], stderr: &[u8]) -> Result<&'static str> {
    require(
        receipt["exit_status"] == 1
            && receipt["timed_out"] == false
            && receipt["cancelled"] == false
            && receipt.get("spawn_or_wait_error").is_none(),
        "negative control requires ordinary exit1 without supervisor failure",
    )?;
    require(
        stdout.is_empty(),
        "negative control emitted unexpected stdout",
    )?;
    let text = std::str::from_utf8(stderr)?;
    let message = text
        .strip_prefix("dataset: ")
        .and_then(|s| s.strip_suffix('\n'))
        .ok_or("negative control lacks exact dataset diagnostic wrapper")?;
    require(
        !message.contains(['\n', '\r']),
        "multiline negative diagnostic",
    )?;
    stage(slot, message)
        .ok_or_else(|| format!("unexpected control diagnostic at slot{slot}: {message}").into())
}
fn stream(root: &Path, prefix: &Path, receipt: &Value, name: &str) -> Result<Vec<u8>> {
    let path = prefix.with_extension(name);
    let relative = path.strip_prefix(root)?;
    require(
        receipt[name]["path"] == path_text(relative)?,
        "command stream path differs",
    )?;
    let bytes = bounded(&checked_file(root, &path_text(relative)?)?)?;
    require(
        receipt[name]["sha256"] == digest(&bytes) && receipt[name]["bytes"] == bytes.len() as u64,
        "command stream differs from terminal receipt",
    )?;
    Ok(bytes)
}
fn copy_admitted(runner: &Runner<'_>, source: &Path, destination: &Path) -> Result<()> {
    let projected = tree_bytes(&runner.root)?
        .checked_add(tree_bytes(source)?)
        .ok_or("copied control storage overflow")?;
    admit_resources(runner.commands.len(), projected)?;
    copy_tree(source, destination)
}
fn invoke_negative(
    runner: &mut Runner<'_>,
    slot: u64,
    dir: &Path,
    argv: Vec<String>,
    protected: &[(PathBuf, Vec<Value>)],
) -> Result<Value> {
    let label = format!("control-{slot:02}");
    write_new(
        &dir.join("invocation-intent.json"),
        &json!({"label":label,"argv":argv,"started_unix_seconds":now()?}),
    )?;
    let invoked = runner.invoke(&label, argv, false);
    let checks: Vec<_> = protected
        .iter()
        .map(|(path, before)| match snapshot(path) {
            Ok(after) => json!({"source":path,"unchanged":*before==after,"after":after}),
            Err(error) => {
                json!({"source":path,"unchanged":false,"readback_error":error.to_string()})
            }
        })
        .collect();
    write_new(
        &dir.join("source-readback.json"),
        &json!({"sources":checks,
        "command_error":invoked.as_ref().err().map(ToString::to_string)}),
    )?;
    require(
        checks.iter().all(|row| row["unchanged"] == true),
        "negative control changed original or lost source readback",
    )?;
    let prefix = invoked?;
    let receipt = json_file(
        &runner.root,
        &path_text(
            prefix
                .with_extension("receipt.json")
                .strip_prefix(&runner.root)?,
        )?,
    )?;
    let stdout = stream(&runner.root, &prefix, &receipt, "stdout")?;
    let stderr = stream(&runner.root, &prefix, &receipt, "stderr")?;
    let observed = diagnostic(slot, &receipt, &stdout, &stderr)?;
    Ok(
        json!({"status":"passed","command_label":label,"observed_stage":observed,
        "command_receipt_sha256":hash(&prefix.with_extension("receipt.json"))?,
        "original_unchanged":true,"semantic_rejection_verified":true}),
    )
}
struct Selected {
    root: PathBuf,
    cell: PathBuf,
    row: Value,
}
fn select(runner: &Runner<'_>, admission: &Admission, slot: u64) -> Result<Option<Selected>> {
    for row in &admission.cells {
        let ordinal = number(row, "ordinal")?;
        let root = runner.root.join(format!("cell-{ordinal:02}"));
        let cell = root.join(format!("cells/{:012}", number(row, "global_index")?));
        let base = json_file(&cell, "factor-base.json")?;
        let signed = base["schema"] == "ml-ecdlp-factor-base/v2";
        let v2 = field(row, "plan")?.starts_with("v2-");
        let manifest = json_file(&cell, "manifest.json")?;
        let qualifies = match slot {
            1 => signed && base["points"].as_array().is_some_and(|p| p.len() >= 2),
            2 => {
                signed
                    && base["domain_digest"]
                        .as_str()
                        .is_some_and(|s| s.len() == 64)
            }
            3 => {
                signed
                    && json_file(&cell, "witness.json")?["factor_base_policy_construction"]
                        .is_object()
            }
            4 | 9 | 10 => v2,
            5 => v2 && !signed,
            7 => {
                v2 && manifest["config"]["solver"] == "pkm"
                    && manifest["outcome"]["native_alternative_domain_admission"]
                        == "preflight_budget_exhausted"
            }
            8 => manifest["artifacts"].as_array().is_some_and(|rows| {
                rows.iter().any(|a| {
                    a["role"] == "explicit-alternative-kummer-factor-base"
                        || a["role"] == "exact-large-prime-discovery-domains"
                })
            }),
            13..=15 => v2,
            _ => false,
        };
        if qualifies {
            return Ok(Some(Selected {
                root,
                cell,
                row: row.clone(),
            }));
        }
    }
    Ok(None)
}
fn missing(slot: u64, reason: &str) -> Value {
    json!({"slot":slot,"status":"missing_precondition","reason":reason,
        "command_invocation_attempted":false,"semantic_rejection_verified":false})
}
fn definition(slot: u64) -> Value {
    let expected = match slot {
        1 | 2 => "factor-base-policy-replay",
        3 => "constructor-terminal-replay",
        4 => "exact-occurrence-binding",
        5 => "expected-policy-schema",
        6 => "unsupported-profile-named-API",
        7 => "native-route-preflight",
        8 => "required-native-domain",
        9 => "required-v2-capture-context",
        10 => "original-capture-binding-or-required-execution-context; optional-flag-API-required",
        11 => "full-policy-plan-scope",
        12 => "original-byte-duplicate-policy-field",
        13 | 14 => "immutable-export-options",
        15 => "orphan-export-channel",
        16 => "retained-signed-partial-source-named-API",
        _ => "invalid-slot",
    };
    json!({"recipe_version":"copied-controls/v1","expected_stage":expected,
        "source_selector":match slot {6|16=>"named API prerequisite; no invented source",
            11|12=>"frozen v2-bsgs child plan/protocol",
            _=>"first qualifying already-verified occurrence in frozen population order"}})
}
fn execute_cell(
    runner: &mut Runner<'_>,
    admission: &Admission,
    slot: u64,
    dir: &Path,
) -> Result<Value> {
    let Some(selected) = select(runner, admission, slot)? else {
        return Ok(missing(
            slot,
            "no qualifying source in the fixed verified population",
        ));
    };
    let before = snapshot(&selected.root)?;
    let copy = dir.join("source");
    copy_admitted(runner, &selected.root, &copy)?;
    let cell = copy.join(selected.cell.strip_prefix(&selected.root)?);
    let recipe = match slot {
        1 => Recipe::ReorderSignedPoints,
        2 => {
            let old = json_file(&cell, "factor-base.json")?["domain_digest"]
                .as_str()
                .ok_or("domain digest")?
                .to_owned();
            let mut replacement = None;
            for row in &admission.cells {
                let other = runner.root.join(format!(
                    "cell-{:02}/cells/{:012}",
                    number(row, "ordinal")?,
                    number(row, "global_index")?
                ));
                let base = json_file(&other, "factor-base.json")?;
                if base["schema"] == "ml-ecdlp-factor-base/v2" {
                    if let Some(value) = base["domain_digest"]
                        .as_str()
                        .filter(|value| value.len() == 64 && *value != old)
                    {
                        replacement = Some(value.to_owned());
                        write_new(
                            &dir.join("replacement-domain-source.json"),
                            &json!({"selected":row,"domain_digest":value,"manifest_sha256":hash(&other.join("manifest.json"))?}),
                        )?;
                        break;
                    }
                }
            }
            let Some(value) = replacement else {
                return Ok(missing(
                    slot,
                    "no distinct signed domain digest in fixed verified population",
                ));
            };
            Recipe::ReplaceSignedDomainDigest(value)
        }
        3 => Recipe::ChangeConstructionTerminal,
        4 => Recipe::ChangePolicyOrigin,
        5 => Recipe::SignedSchemaOnLegacy,
        7 => Recipe::PromoteCappedPkm,
        8 => Recipe::RemoveAlternateDomain,
        9 => Recipe::RemoveCaptureBinding,
        10 => Recipe::RemoveExecutionContextAndFlag,
        _ => return Err("invalid cell control".into()),
    };
    let custody = control_custody::apply(&selected.cell, &cell, recipe)?;
    if slot == 10 {
        let mut run = json_file(&copy, "run.json")?;
        run.as_object_mut()
            .ok_or("root run object")?
            .remove("execution_context_required");
        let bytes = serde_json::to_vec(&run)?;
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(checked_file(&copy, "run.json")?)?;
        file.write_all(&bytes)?;
        file.sync_all()?;
    }
    write_new(
        &dir.join("prepared.json"),
        &json!({"selected":selected.row,"original":before,
        "source_manifest_sha256":hash(&selected.cell.join("manifest.json"))?,"custody":custody,
        "root_run_flag_removed":slot==10}),
    )?;
    let argv = strings(&["verify", "--out", &path_text(&copy)?]);
    invoke_negative(runner, slot, dir, argv, &[(selected.root, before)])
}
fn execute_export(
    runner: &mut Runner<'_>,
    admission: &Admission,
    slot: u64,
    dir: &Path,
) -> Result<Value> {
    let Some(selected) = select(runner, admission, slot)? else {
        return Ok(missing(slot, "no verified V2 export source"));
    };
    let ordinal = number(&selected.row, "ordinal")?;
    let original_destination = runner.root.join(format!(
        "{}-{ordinal:02}",
        if slot == 13 { "ecdlp" } else { "pdp" }
    ));
    let protected = vec![
        (selected.root.clone(), snapshot(&selected.root)?),
        (
            original_destination.clone(),
            snapshot(&original_destination)?,
        ),
    ];
    let copy = dir.join("source");
    let destination = dir.join("export");
    copy_admitted(runner, &selected.root, &copy)?;
    copy_admitted(runner, &original_destination, &destination)?;
    if slot == 15 {
        let mut file = OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(destination.join("ecdlp-features.jsonl"))?;
        file.write_all(b"{}\n")?;
        file.sync_all()?;
    }
    let mut argv = strings(&[
        "export",
        "--out",
        &path_text(&copy)?,
        "--destination",
        &path_text(&destination)?,
    ]);
    if slot == 14 {
        argv.push("--cyclic-subgroup-ecdlp".into());
    }
    write_new(
        &dir.join("prepared.json"),
        &json!({"selected":selected.row,"original_sources":protected,
        "mutation":if slot==15{"orphan-ecdlp-channel"}else{"incompatible-export-options"}}),
    )?;
    invoke_negative(runner, slot, dir, argv, &protected)
}
fn execute_plan(
    runner: &mut Runner<'_>,
    admission: &Admission,
    slot: u64,
    dir: &Path,
) -> Result<Value> {
    let id = "v2-bsgs";
    let i = IDS.iter().position(|v| *v == id).ok_or("V2 plan missing")?;
    let fixture = runner.root.join("fixture");
    let before = snapshot(&fixture)?;
    let (original_protocol, protocol_sha, plan_bytes) = frozen_plan_inputs(&runner.root, id)?;
    let argv = if slot == 11 {
        let mut protocol = json_file(&fixture, &format!("protocols/{id}.json"))?;
        let mut freeze = protocol
            .as_object_mut()
            .ok_or("protocol object")?
            .remove("freeze")
            .ok_or("freeze")?;
        let scope = &mut protocol["extensions"]["ml_dataset_policy_plan"];
        let old = field(scope, "scope_sha256")?.to_owned();
        scope["scope_sha256"] = json!(if old == "0".repeat(64) {
            "1".repeat(64)
        } else {
            "0".repeat(64)
        });
        freeze["protocol_sha256"] = json!(digest(canonical(&protocol)?.as_bytes()));
        protocol["freeze"] = freeze;
        let protocol_path = dir.join("protocol.json");
        write_new(&protocol_path, &protocol)?;
        let mut plan = unique(&plan_bytes, 0)?;
        plan["axes"]["protocol_sha256"] = json!(hash(&protocol_path)?);
        let plan_path = dir.join("plan.json");
        write_new(&plan_path, &plan)?;
        strings(&[
            "preflight",
            "--plan",
            &path_text(&plan_path)?,
            "--protocol",
            &path_text(&protocol_path)?,
        ])
    } else {
        let policy = dir.join("policy.json");
        let mut file = OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&policy)?;
        file.write_all(
            br#"[{"kind":"legacy-small-x-projected/v1","kind":"legacy-small-x-projected/v1"}]"#,
        )?;
        file.sync_all()?;
        plan_argv(
            &admission.plans[i],
            &dir.join("rejected-plan.json"),
            &protocol_sha,
            Some(&policy),
        )?
    };
    write_new(
        &dir.join("prepared.json"),
        &json!({"source_plan_sha256":digest(&plan_bytes),
        "source_protocol_sha256":hash(&original_protocol)?,"original":before,"slot":slot}),
    )?;
    invoke_negative(runner, slot, dir, argv, &[(fixture, before)])
}
pub(super) fn run(
    runner: &mut Runner<'_>,
    admission: &Admission,
    native_complete: bool,
) -> Result<Value> {
    let root = runner.root.join("controls");
    fs::create_dir(&root)?;
    let first = runner.commands.len();
    write_new(
        &root.join("workflow.started.json"),
        &json!({"workflow":"copied-controls","started_unix_seconds":now()?,
        "native_stage_complete":native_complete,"first_command":first,"slots":16}),
    )?;
    let mut rows = vec![];
    let mut stop = if native_complete {
        None
    } else {
        Some("native workflow did not complete; no copied commands admitted".to_owned())
    };
    for slot in 1..=16 {
        let dir = root.join(format!("slot-{slot:02}"));
        fs::create_dir(&dir)?;
        write_new(
            &dir.join("started.json"),
            &json!({"slot":slot,"started_unix_seconds":now()?}),
        )?;
        let outcome = if let Some(reason) = &stop {
            json!({"status":"stopped_before_attempt","reason":reason,"semantic_rejection_verified":false})
        } else if slot == 6 || slot == 16 {
            let mut row = missing(
                slot,
                if slot == 6 {
                    "unsupported profile absent from fixed population"
                } else {
                    "retained signed partial-source API fixture unavailable"
                },
            );
            row["mandatory_api_gate"] = json!(if slot == 6 {
                "unsupported_alternative_profile_cannot_acquire_a_rehashed_success_claim"
            } else {
                "retained-signed-partial-source-export"
            });
            row
        } else {
            let result = match slot {
                1..=10 => execute_cell(runner, admission, slot, &dir),
                11 | 12 => execute_plan(runner, admission, slot, &dir),
                13..=15 => execute_export(runner, admission, slot, &dir),
                _ => unreachable!(),
            };
            match result {
                Ok(value) => value,
                Err(error) => {
                    stop = Some(error.to_string());
                    json!({"status":"failed","error":error.to_string(),"semantic_rejection_verified":false})
                }
            }
        };
        let label = format!("control-{slot:02}");
        let start = runner
            .root
            .join("commands")
            .join(format!("{label}.started.json"));
        let mut row = outcome;
        row["slot"] = json!(slot);
        row["definition"] = definition(slot);
        if slot == 10 {
            row["mandatory_api_gate"] =
                json!("V2 optional-flag removal requires retained execution-context API assertion");
            row["api_gate_verified"] = json!(false);
        }
        row["command_invocation_attempted"] = json!(start.try_exists()?);
        row["command_receipt_present"] = json!(runner
            .root
            .join("commands")
            .join(format!("{label}.receipt.json"))
            .try_exists()?);
        row["finished_unix_seconds"] = json!(now()?);
        write_new(&dir.join("receipt.json"), &row)?;
        rows.push(row);
    }
    let commands = &runner.commands[first..];
    for command in commands {
        let label = field(command, "label")?;
        require(
            rows.iter()
                .filter(|row| {
                    row["command_invocation_attempted"] == true
                        && label == format!("control-{:02}", row["slot"].as_u64().unwrap_or(0))
                })
                .count()
                == 1,
            "control command not partitioned exactly once",
        )?;
    }
    let missing_slots: Vec<_> = rows
        .iter()
        .filter(|row| row["status"] == "missing_precondition")
        .map(|row| row["slot"].clone())
        .collect();
    let passed_slots: Vec<_> = rows
        .iter()
        .filter(|row| row["status"] == "passed")
        .map(|row| row["slot"].clone())
        .collect();
    let result = json!({"schema":"ml-dataset.copied-control-workflow/v1","workflow":"copied-controls",
        "finished_unix_seconds":now()?,"first_command":first,"end_command":runner.commands.len(),
        "controls":rows,"missing_slots":missing_slots,"passed_slots":passed_slots,
        "all_slots_passed":passed_slots.len()==16,"failure":stop,"full_acceptance":false,"source_sha256":digest(SOURCE.as_bytes())});
    write_new(&root.join("workflow.receipt.json"), &result)?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn expected_text_never_masks_process_failure_or_unrelated_output() {
        let receipt = json!({"exit_status":1,"timed_out":false,"cancelled":false});
        let text = b"dataset: V2 capture context missing\n";
        assert_eq!(
            diagnostic(9, &receipt, b"", text).unwrap(),
            "required-v2-capture-context"
        );
        for (key, value) in [
            ("exit_status", json!(-1)),
            ("exit_status", json!(127)),
            ("timed_out", json!(true)),
            ("cancelled", json!(true)),
            ("spawn_or_wait_error", json!("failure")),
        ] {
            let mut bad = receipt.clone();
            bad[key] = value;
            assert!(diagnostic(9, &bad, b"", text).is_err());
        }
        assert!(diagnostic(9, &receipt, b"unexpected", text).is_err());
        assert!(diagnostic(1, &receipt, b"", text).is_err());
        assert!(diagnostic(
            9,
            &receipt,
            b"",
            b"dataset: V2 capture context missing\nother error\n"
        )
        .is_err());
        assert!(diagnostic(9, &receipt, b"", b"dataset: artifact hash mismatch\n").is_err());
    }
    #[test]
    fn duplicate_policy_diagnostic_accepts_only_its_typed_location_suffix() {
        assert!(stage(12, "duplicate field `kind` at line 1 column 59").is_some());
        for value in [
            "duplicate field `kind` at line 1 column 59 trailing",
            "duplicate field `other` at line 1 column 59",
            "duplicate field `kind` at line  column 59",
            "some duplicate error",
        ] {
            assert!(stage(12, value).is_none());
        }
        assert_eq!(
            stage(10, "capture binding differs from original attempt"),
            Some("original-capture-binding-before-execution-context")
        );
    }
}
