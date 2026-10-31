//! Contract-derived negative lineage fixtures. No consumer validator is used to
//! construct expected hashes or select expected outcomes. The writer rebuilds
//! transport ancestors unless a case explicitly says FixedPin or StaleEventHash.
//!
//! N01-N30 each have a substantive representative below; this is NOT a claim
//! that every subcase in those matrix rows is covered or that tests have run.
//! N04 cryptographic receipt-hash cycles cannot be authored by this finite
//! hash-consistent writer. N28 checks rejection of an invalid early prefix but
//! cannot instrument the exact ordinal at which the context core is invoked.
//! N27 exercises projected/native disagreement, lifecycle and admission flags;
//! the full unsupported-backend/FD state matrix remains the phase suite's job.
//! TOCTOU, bounded mock transport, allocation and actual CLI gates belong to
//! separately owned tests. This file does not substitute for them.
use super::test_support::{hash, saved, wire, Fixture, Published, Stage};
use serde_json::{json, Map, Value};
use std::fs;

const FIRST_START: &str = "telemetry/curve-construction-start.json";
const FIRST_END: &str = "telemetry/curve-construction.json";
const NEXT_START: &str = "telemetry/known-answer-replay-start.json";
const CHUNK: &str = "events/000000000000.jsonl";

#[derive(Clone, Copy, Debug)]
enum Pins {
    RebuiltAncestors,
    FixedPin,
    StaleEventHash,
    RestoredTreeMutation,
}

fn rejected(p: &Published, label: &str, gate: &str, pins: Pins, before_artifacts: bool) {
    let error = p.restore().expect_err(label);
    let actual = format!("{error:#}");
    let artifact_opens = p.artifact_opens();
    assert!(
        actual.contains(gate),
        "{label}: expected gate {gate:?}, observed {actual:?}"
    );
    if before_artifacts {
        assert_eq!(
            artifact_opens, 0,
            "{label}: artifact source was opened before admission failed"
        );
        assert!(
            !p.artifact_path("context/run.json").exists(),
            "{label}: artifact transfer occurred before admission failed"
        );
    }
    p.retain(label, gate, &json!({"rejected":true,"error":actual,"pin_mode":format!("{pins:?}"),"before_artifacts":before_artifacts,"artifact_opens":artifact_opens}));
}

fn preflight(p: &Published, label: &str, gate: &str) {
    rejected(p, label, gate, Pins::RebuiltAncestors, true);
}

fn semantics(p: &Published, label: &str, gate: &str) {
    rejected(p, label, gate, Pins::RebuiltAncestors, false);
}

fn audit_rejected(p: &Published, label: &str, gate: &str) {
    let error = p.audit().expect_err(label);
    let actual = format!("{error:#}");
    assert!(
        actual.contains(gate),
        "{label}: expected {gate:?}, observed {actual:?}"
    );
    p.retain(label, gate, &json!({"rejected":true,"error":actual,"pin_mode":format!("{:?}",Pins::RestoredTreeMutation)}));
}

fn rename_descriptors(value: &mut Value, old: &str, new: &str) {
    if value.get("path").and_then(Value::as_str) == Some(old) && value.get("sha256").is_some() {
        value["path"] = json!(new);
    }
    match value {
        Value::Object(m) => {
            for child in m.values_mut() {
                rename_descriptors(child, old, new);
            }
        }
        Value::Array(a) => {
            for child in a {
                rename_descriptors(child, old, new);
            }
        }
        _ => (),
    }
}

fn one_event() -> Fixture {
    let mut f = Fixture::new(false);
    f.capture(
        0,
        vec![json!({"kind":"checkpoint","value":7})],
        vec![],
        None,
    );
    f
}

fn first_phase() -> Fixture {
    let mut f = Fixture::new(true);
    f.add_saved_phase(0, FIRST_START);
    f.add_saved_phase(0, FIRST_END);
    f
}

fn mutate_json_artifact(f: &mut Fixture, path: &str, edit: impl FnOnce(&mut Value)) {
    let mut value: Value = serde_json::from_slice(&f.blobs[path]).unwrap();
    edit(&mut value);
    f.replace_artifact(path, wire(&value));
}

/// One segment only. Refresh byte descriptors, declared byte endpoints and the
/// last raw-line digest, while deliberately preserving sequence/payload totals
/// unless the particular case changes them. Transport hashes are built later.
fn replace_chunk(f: &mut Fixture, raw: Vec<u8>) {
    assert_eq!(
        f.snapshots[0]["delta"]["segments"]
            .as_array()
            .unwrap()
            .len(),
        1
    );
    let end = raw.len() as u64;
    let last = raw
        .split_inclusive(|b| *b == b'\n')
        .next_back()
        .unwrap_or(&[]);
    let digest = hash(last);
    f.replace_artifact(CHUNK, raw);
    let s = &mut f.snapshots[0];
    s["delta"]["segments"][0]["byte_end"] = json!(end);
    s["delta"]["segments"][0]["last_event_sha256"] = json!(digest);
    s["delta"]["cursor"]["byte_end"] = json!(end);
    s["delta"]["cursor"]["last_event_sha256"] = json!(digest);
    s["delta"]["head"]["byte_end"] = json!(end);
    s["delta"]["head"]["last_event_sha256"] = json!(digest);
    s["coverage"]["captured_event_end"] = json!(end);
}

/// Independent fixed v1 plan-field order. This is fixture serialization, never
/// a call to audit::canonical_plan or a producer helper. It repairs only outer
/// protocol-file/plan/run/attempt commitments, leaving the bad protocol body.
fn replace_protocol(f: &mut Fixture, raw: Vec<u8>) {
    let mut plan: Value = serde_json::from_slice(&f.blobs["context/plan.json"]).unwrap();
    plan["protocol_sha256"] = json!(hash(&raw));
    let fields = [
        "schema",
        "field_sizes",
        "arities",
        "families",
        "solvers",
        "curve_seeds",
        "generators",
        "keys",
        "seed",
        "factor_base_size",
        "degree_bound",
        "work_limit",
        "twists",
        "protocol_sha256",
        "split_rule",
    ];
    let mut ordered = Map::new();
    for name in fields {
        ordered.insert(name.into(), plan[name].clone());
    }
    let canonical = serde_json::to_vec(&Value::Object(ordered)).unwrap();
    let commitment = hash(&canonical);
    f.replace_artifact("context/protocol.json", raw);
    f.replace_artifact("context/plan.json", wire(&plan));
    for name in ["context/run.json", "context/attempt.json"] {
        mutate_json_artifact(f, name, |v| v["plan_sha256"] = json!(commitment));
    }
    f.refresh_binding_raw();
}

#[test]
fn n01_fixed_anchor_pin_fails_before_destination_creation() {
    for corrupt_bytes in [false, true] {
        let mut p = Fixture::new(false).publish();
        if corrupt_bytes {
            let mut raw = fs::read(&p.anchor).unwrap();
            raw[0] = b'['; // Same length; hash check must precede JSON parsing.
            fs::write(&p.anchor, raw).unwrap();
        } else {
            p.pin = "0".repeat(64);
        }
        let label = if corrupt_bytes {
            "n01-same-length-anchor"
        } else {
            "n01-wrong-pin"
        };
        rejected(
            &p,
            label,
            "partial caller anchor pin mismatch",
            Pins::FixedPin,
            true,
        );
        assert!(
            !p.out.exists(),
            "{label}: destination created before pin check"
        );
    }
}

#[test]
fn n02_bounded_metadata_parser_rejects_rehashed_malformed_envelopes() {
    for (label, prefix, gate) in [
        (
            "n02-duplicate",
            b"\"ordinal\":0,".as_slice(),
            "duplicate decoded partial JSON key",
        ),
        (
            "n02-escaped-duplicate",
            b"\"\\u006frdinal\":0,".as_slice(),
            "duplicate decoded partial JSON key",
        ),
    ] {
        let p = Fixture::new(false).publish_raw(|_, stage, raw| {
            if stage == Stage::Manifest {
                raw.splice(1..1, prefix.iter().copied());
            }
        });
        preflight(&p, label, gate);
    }
    for (label, field, value) in [
        ("n02-unknown", "unknown", json!(true)),
        ("n02-null-counter", "ordinal", Value::Null),
        ("n02-negative-counter", "ordinal", json!(-1)),
        ("n02-string-counter", "ordinal", json!("0")),
    ] {
        let p = Fixture::new(false).publish_with(|_, stage, v| {
            if stage == Stage::Manifest {
                v[field] = value.clone();
            }
        });
        preflight(&p, label, "partial strict wire");
    }
    let p = Fixture::new(false).publish_raw(|_, stage, raw| {
        if stage == Stage::Manifest {
            raw.extend_from_slice(b" true");
        }
    });
    preflight(&p, "n02-trailing", "trailing characters");
    let p = Fixture::new(false).publish_raw(|_, stage, raw| {
        if stage == Stage::Manifest {
            *raw = String::from_utf8(raw.clone())
                .unwrap()
                .replacen("\"ordinal\": 0", "\"ordinal\": 18446744073709551616", 1)
                .into_bytes();
        }
    });
    preflight(&p, "n02-u64-overflow", "partial strict wire");
}

#[test]
fn n03_profiles_cannot_switch_or_smuggle_checked_context() {
    let p = Fixture::new(false).publish_with(|_, s, v| {
        if s == Stage::Manifest {
            v["schema"] = json!("ml-dataset.partial-snapshot/v999");
        }
    });
    preflight(
        &p,
        "n03-unknown-profile",
        "partial unknown manifest profile",
    );
    let mut f = Fixture::new(false);
    f.push_snapshot(true);
    preflight(
        &f.publish(),
        "n03-profile-switch",
        "partial profile or binding switch",
    );
    let mut f = Fixture::new(false);
    f.add_context(
        0,
        "measurement-policy.json",
        saved("measurement-policy.json"),
    );
    f.snapshots[0]["context"]
        .as_array_mut()
        .unwrap()
        .last_mut()
        .unwrap()["role"] = json!("opaque-legacy-role");
    preflight(
        &f.publish(),
        "n03-policy-smuggling",
        "partial legacy policy/phase smuggling",
    );
}

#[test]
fn n04_missing_and_gapped_ancestry_do_not_verify_a_suffix() {
    for (label, change) in [
        ("n04-missing-predecessor", false),
        ("n04-ordinal-gap", true),
    ] {
        let mut f = Fixture::new(false);
        f.push_snapshot(false);
        let p = f.publish_with(|at, stage, v| {
            if at == 1 && matches!(stage, Stage::Manifest | Stage::Receipt) {
                if change {
                    v["previous"]["ordinal"] = json!(7);
                } else {
                    v["previous"] = Value::Null;
                }
            }
        });
        preflight(
            &p,
            label,
            if change {
                "partial predecessor ordinal gap"
            } else {
                "partial missing or unexpected predecessor"
            },
        );
    }
    let f = Fixture::new(false);
    let prior = f.publish().anchors[0].clone();
    let p = f.publish_with(|_, stage, v| {
        if matches!(stage, Stage::Manifest | Stage::Receipt) {
            v["previous"] = prior.clone();
        }
    });
    preflight(
        &p,
        "n04-zero-has-predecessor",
        "partial missing or unexpected predecessor",
    );
    let mut f = Fixture::new(false);
    f.push_snapshot(false);
    let p = f.publish();
    fs::remove_file(
        p.mirror
            .join(p.anchors[0]["catalog"]["key"].as_str().unwrap()),
    )
    .unwrap();
    rejected(
        &p,
        "n04-absent-prefix-object",
        "No such file or directory",
        Pins::FixedPin,
        true,
    );
}

#[test]
fn n05_envelope_identities_are_cross_bound() {
    for (label, stage, field, value, gate) in [
        (
            "n05-catalog-remote",
            Stage::Catalog,
            "remote",
            json!("file:///different"),
            "partial catalog/anchor disagreement",
        ),
        (
            "n05-catalog-run",
            Stage::Catalog,
            "run_sha256",
            json!("e".repeat(64)),
            "partial catalog/anchor disagreement",
        ),
        (
            "n05-receipt-binding",
            Stage::Receipt,
            "binding_sha256",
            json!("e".repeat(64)),
            "partial receipt/anchor disagreement",
        ),
        (
            "n05-catalog-attempt",
            Stage::Catalog,
            "attempt_sha256",
            json!("e".repeat(64)),
            "partial catalog/anchor disagreement",
        ),
        (
            "n05-anchor-binding",
            Stage::Anchor,
            "binding_sha256",
            json!("e".repeat(64)),
            "partial catalog/anchor disagreement",
        ),
    ] {
        let p = Fixture::new(false).publish_with(|_, s, v| {
            if s == stage {
                v[field] = value.clone();
            }
        });
        preflight(&p, label, gate);
    }
    let mut f = Fixture::new(false);
    f.push_snapshot(false);
    let p = f.publish_with(|at, stage, v| {
        if at == 1 && matches!(stage, Stage::Manifest | Stage::Receipt) {
            v["previous"]["remote"] = json!("file:///different");
        }
    });
    preflight(
        &p,
        "n05-predecessor-remote",
        "partial predecessor anchor identity",
    );
}

#[test]
fn n06_remote_and_reference_aliases_reject_before_object_reads() {
    for (label, remote, gate) in [
        (
            "n06-bucket-slash",
            "s3://setec-astronomy-artifacts/",
            "partial noncanonical S3 prefix",
        ),
        (
            "n06-prefix-slash",
            "s3://setec-astronomy-artifacts/prefix/",
            "partial noncanonical S3 prefix",
        ),
        (
            "n06-percent",
            "s3://setec-astronomy-artifacts/%2e",
            "partial noncanonical S3 remote",
        ),
        (
            "n06-unicode-slash",
            "s3://setec-astronomy-artifacts∕prefix",
            "partial canonical S3 bucket required",
        ),
        (
            "n06-file-relative",
            "file://relative",
            "partial file remote must be absolute",
        ),
    ] {
        let mut f = Fixture::new(false);
        f.remote = remote.into();
        preflight(&f.publish(), label, gate);
    }
    let p = Fixture::new(false).publish_with(|_, s, v| {
        if s == Stage::Anchor {
            v["catalog"]["key"] = json!("catalog/wrong.json");
        }
    });
    preflight(&p, "n06-catalog-key", "partial catalog key identity");
    let p = Fixture::new(false).publish_with(|_, s, v| {
        if s == Stage::Anchor {
            v["receipt"]["key"] = json!(format!(
                "sha256/zz/{}",
                v["receipt"]["sha256"].as_str().unwrap()
            ));
        }
    });
    preflight(&p, "n06-cas-prefix", "partial noncanonical CAS key");
    let p = Fixture::new(false).publish_with(|_, s, v| {
        if s == Stage::Anchor {
            v["run_sha256"] = json!("A".repeat(64));
        }
    });
    preflight(&p, "n06-uppercase-digest", "partial anchor digest grammar");
}

#[test]
fn n07_receipts_and_ordered_membership_are_exact() {
    for (label, field, gate) in [
        (
            "n07-count",
            "artifact_count",
            "partial receipt artifact count",
        ),
        (
            "n07-bytes",
            "artifact_bytes",
            "partial receipt artifact bytes",
        ),
    ] {
        let p = Fixture::new(false).publish_with(|_, s, v| {
            if s == Stage::Receipt {
                v[field] = json!(v[field].as_u64().unwrap() + 1);
            }
        });
        preflight(&p, label, gate);
    }
    let p = Fixture::new(false).publish_with(|_, s, v| {
        if s == Stage::Manifest {
            v["objects"].as_array_mut().unwrap().swap(0, 1);
        }
    });
    preflight(
        &p,
        "n07-reordered",
        "partial exact ordered object membership",
    );
    let p = Fixture::new(false).publish_with(|_, s, v| {
        if s == Stage::Manifest {
            let extra = v["objects"][0].clone();
            v["objects"].as_array_mut().unwrap().push(extra);
        }
    });
    preflight(&p, "n07-extra-object", "snapshot artifacts/membership");
}

#[test]
fn n08_duplicate_lineage_and_prefix_conflicts_reject_before_transfer() {
    let mut f = Fixture::new(false);
    f.add_context(0, "run.json", saved("run.json"));
    preflight(
        &f.publish(),
        "n08-duplicate",
        "partial duplicate or redefined artifact path",
    );
    let mut f = Fixture::new(false);
    let at = f.push_snapshot(false);
    f.add_context(at, "run.json", saved("run.json"));
    preflight(
        &f.publish(),
        "n08-identical-redefinition",
        "partial duplicate or redefined artifact path",
    );
    for reverse in [false, true] {
        let mut f = Fixture::new(false);
        f.add_context(0, "run.json/child", b"{}".to_vec());
        if reverse {
            f.snapshots[0]["context"].as_array_mut().unwrap().reverse();
        }
        preflight(
            &f.publish(),
            if reverse {
                "n08-descendant-first"
            } else {
                "n08-parent-first"
            },
            "partial file/directory prefix conflict",
        );
    }
}

#[test]
fn n09_untrusted_paths_do_not_reach_open() {
    for (suffix, path) in [
        ("absolute", "/run.json".to_owned()),
        ("traversal", "context/../run.json".into()),
        ("backslash", "context\\run.json".into()),
        ("control", "context/\nrun.json".into()),
        ("empty", String::new()),
        ("long", "a".repeat(1025)),
    ] {
        let p = Fixture::new(false).publish_with(|_, s, v| {
            if s == Stage::Manifest {
                rename_descriptors(v, "context/run.json", &path);
            }
        });
        preflight(
            &p,
            &format!("n09-{suffix}"),
            "partial invalid relative path",
        );
    }
}

#[test]
fn n10_initial_context_is_mandatory_and_raw_hash_bound() {
    for late in [false, true] {
        let mut f = Fixture::new(false);
        f.snapshots[0]["context"]
            .as_array_mut()
            .unwrap()
            .retain(|a| a["path"] != "context/attempt.json");
        if late {
            let at = f.push_snapshot(false);
            f.add_context(at, "attempt.json", saved("attempt.json"));
        }
        preflight(
            &f.publish(),
            if late {
                "n10-late-attempt"
            } else {
                "n10-missing-attempt"
            },
            "partial initial authoritative context missing",
        );
    }
    let mut f = Fixture::new(false);
    f.binding["plan_sha256"] = json!("e".repeat(64));
    f.snapshots[0]["binding"] = f.binding.clone();
    preflight(
        &f.publish(),
        "n10-raw-plan-digest",
        "partial initial raw context digest disagreement",
    );
}

#[test]
fn n11_actual_context_is_checked_beyond_transport_hashes() {
    for (label, field, value, gate) in [
        (
            "n11-source",
            "source_commit",
            json!("c".repeat(40)),
            "partial actual authoritative binding disagreement",
        ),
        (
            "n11-binary",
            "binary_sha256",
            json!("c".repeat(64)),
            "partial actual authoritative binding disagreement",
        ),
        (
            "n11-shard",
            "shard_count",
            json!(0),
            "phase binding shard mismatch",
        ),
        (
            "n11-canonical-plan",
            "plan_sha256",
            json!("e".repeat(64)),
            "phase binding canonical plan disagreement",
        ),
    ] {
        let mut f = Fixture::new(false);
        mutate_json_artifact(&mut f, "context/run.json", |v| v[field] = value);
        f.refresh_binding_raw();
        semantics(&f.publish(), label, gate);
    }
    let mut f = Fixture::new(false);
    mutate_json_artifact(&mut f, "context/config.json", |v| v["seed"] = json!(20));
    f.refresh_binding_raw();
    semantics(
        &f.publish(),
        "n11-config",
        "phase binding config/index mismatch",
    );
    let mut f = Fixture::new(false);
    mutate_json_artifact(&mut f, "context/attempt.json", |v| {
        v["cell_index"] = json!(1)
    });
    f.refresh_binding_raw();
    semantics(
        &f.publish(),
        "n11-index",
        "phase binding run/attempt identity",
    );
}

#[test]
fn n12_protocol_failures_are_not_masked_by_stale_outer_commitments() {
    for (label, kind, gate) in [
        (
            "n12-body-digest",
            0,
            "freeze_body_digest: protocol body mismatch",
        ),
        ("n12-nonfinite", 1, "nonfinite_float: protocol number"),
    ] {
        let mut f = Fixture::new(false);
        let mut protocol: Value = serde_json::from_slice(&saved("protocol.json")).unwrap();
        protocol["purpose"] = json!("changed after freeze");
        let raw = if kind == 0 {
            wire(&protocol)
        } else {
            let mut raw = wire(&protocol);
            raw.splice(1..1, b"\"unbounded\":1e400,".iter().copied());
            raw
        };
        replace_protocol(&mut f, raw);
        semantics(&f.publish(), label, gate);
    }
}

#[test]
fn n13_observation_progression_and_bounds_are_enforced() {
    for (label, field, value) in [
        ("n13-running-exit", "exit_code", json!(0)),
        ("n13-running-signal", "signal", json!(9)),
        ("n13-signal-range", "signal", json!(128)),
        ("n13-reason", "reason", json!("x".repeat(4097))),
    ] {
        let mut f = Fixture::new(false);
        f.snapshots[0]["observation"][field] = value;
        if label == "n13-signal-range" {
            f.snapshots[0]["observation"]["status"] = json!("interrupted");
        }
        preflight(&f.publish(), label, "partial invalid observation");
    }
    for terminal in [false, true] {
        let mut f = Fixture::new(false);
        f.push_snapshot(false);
        if terminal {
            f.snapshots[0]["observation"]["status"] = json!("interrupted");
        } else {
            f.snapshots[1]["observation"]["observed_unix_seconds"] = json!(99);
        }
        preflight(
            &f.publish(),
            if terminal {
                "n13-terminal-running"
            } else {
                "n13-time-regression"
            },
            "partial observation regression",
        );
    }
}

#[test]
fn n14_partial_envelopes_cannot_promote_claims() {
    for (stage_name, stage) in [
        ("manifest", Stage::Manifest),
        ("receipt", Stage::Receipt),
        ("catalog", Stage::Catalog),
    ] {
        for field in [
            "cell_complete",
            "solver_state_restorable",
            "scientific_result_verified",
        ] {
            let p = Fixture::new(false).publish_with(|_, s, v| {
                if s == stage {
                    v[field] = json!(true);
                }
            });
            preflight(
                &p,
                &format!("n14-{stage_name}-{field}"),
                "partial prohibited completion/restoration/science claim",
            );
        }
    }
    for field in [
        "artifact_only",
        "in_flight_solver_memory_captured",
        "uncommitted_event_tail_captured",
        "unemitted_backend_internals_captured",
        "solver_resume_supported",
    ] {
        let mut f = Fixture::new(false);
        f.snapshots[0]["coverage"][field] = json!(field != "artifact_only");
        preflight(
            &f.publish(),
            &format!("n14-coverage-{field}"),
            "partial prohibited coverage claim",
        );
    }
}

#[test]
fn n15_coverage_is_derived_from_capture_not_trusted() {
    for (field, value) in [
        ("capture_present", json!(true)),
        ("committed_events", json!(1)),
        ("captured_event_end", json!(1)),
        ("capture_stopped", json!(true)),
        ("stop", json!("storage_failure")),
    ] {
        let mut f = Fixture::new(false);
        f.snapshots[0]["coverage"][field] = value;
        preflight(
            &f.publish(),
            &format!("n15-{field}"),
            "partial coverage disagreement",
        );
    }
}

#[test]
fn n16_invalid_genesis_and_cursor_shape_reject() {
    for (label, field, value) in [
        (
            "n16-genesis-hash",
            "last_event_sha256",
            json!("e".repeat(64)),
        ),
        ("n16-events-without-bytes", "event_count", json!(1)),
        ("n16-empty-with-chunk", "next_chunk", json!(1)),
    ] {
        let mut f = Fixture::new(false);
        f.capture(0, vec![], vec![], None);
        f.snapshots[0]["delta"]["cursor"][field] = value;
        preflight(&f.publish(), label, "partial capture invalid cursor");
    }
}

#[test]
fn n17_head_and_stopped_capture_are_consistent() {
    for (field, value) in [
        ("schema", json!("bad")),
        ("binding_sha256", json!("e".repeat(64))),
        ("event_count", json!(9)),
    ] {
        let mut f = one_event();
        f.snapshots[0]["delta"]["head"][field] = value;
        preflight(
            &f.publish(),
            &format!("n17-head-{field}"),
            "partial capture head/cursor mismatch",
        );
    }
    for (label, advance, clear) in [
        ("n17-stop-advances", true, false),
        ("n17-stop-clears", false, true),
        ("n17-stop-changes", false, false),
    ] {
        let mut f = Fixture::new(false);
        f.capture(0, vec![], vec![], Some("storage_failure"));
        let at = f.push_snapshot(false);
        f.capture(
            at,
            if advance {
                vec![json!({"kind":"after-stop"})]
            } else {
                vec![]
            },
            vec![],
            None,
        );
        if !advance {
            f.snapshots[at]["delta"]["head"]["stop"] = if clear {
                Value::Null
            } else {
                json!("chunk_limit")
            };
        }
        preflight(
            &f.publish(),
            label,
            "partial stopped capture advanced or changed stop",
        );
    }
}

#[test]
fn n18_segment_ranges_and_predecessors_are_exact() {
    for (field, value) in [
        ("byte_start", json!(1)),
        ("first_sequence", json!(2)),
        ("preceding_event_sha256", json!("e".repeat(64))),
    ] {
        let mut f = one_event();
        f.snapshots[0]["delta"]["segments"][0][field] = value;
        preflight(
            &f.publish(),
            &format!("n18-{field}"),
            "partial capture segment order/range/identity",
        );
    }
    let mut f = one_event();
    let raw = f.blobs.remove(CHUNK).unwrap();
    f.blobs.insert("events/000000000007.jsonl".into(), raw);
    rename_descriptors(&mut f.snapshots[0], CHUNK, "events/000000000007.jsonl");
    preflight(
        &f.publish(),
        "n18-chunk-name",
        "partial capture segment order/range/identity",
    );
}

#[test]
fn n19_committed_record_requires_complete_bounded_jsonl() {
    for (label, kind, gate, before) in [
        ("n19-empty", 0, "partial capture segment size", true),
        (
            "n19-missing-lf",
            1,
            "partial segment consumed bytes/hash/final LF mismatch",
            false,
        ),
        ("n19-blank", 2, "EOF while parsing a value", false),
        (
            "n19-oversized-line",
            3,
            "partial committed event exceeds line limit",
            false,
        ),
        ("n19-invalid-utf8", 4, "unicode", false),
    ] {
        let mut f = one_event();
        let mut raw = f.blobs[CHUNK].clone();
        match kind {
            0 => raw.clear(),
            1 => {
                raw.pop();
            }
            2 => raw = b"\n".to_vec(),
            3 => {
                raw = vec![b' '; 1024 * 1024 + 1];
                raw.push(b'\n');
            }
            _ => {
                let position = raw.iter().position(|b| *b == b'k').unwrap();
                raw[position] = 0xff;
            }
        }
        replace_chunk(&mut f, raw);
        if kind == 0 {
            // Isolate the zero-byte segment rule: leave a valid genesis cursor
            // rather than accidentally failing the event-count/byte-end rule.
            let genesis = f.snapshots[0]["delta"]["segments"][0]["preceding_event_sha256"].clone();
            for field in ["event_count", "next_chunk"] {
                f.snapshots[0]["delta"]["cursor"][field] = json!(0);
            }
            f.snapshots[0]["delta"]["cursor"]["last_event_sha256"] = genesis.clone();
            f.snapshots[0]["delta"]["head"]["event_count"] = json!(0);
            f.snapshots[0]["delta"]["head"]["last_event_sha256"] = genesis;
            f.snapshots[0]["coverage"]["committed_events"] = json!(0);
        }
        rejected(&f.publish(), label, gate, Pins::RebuiltAncestors, before);
    }
}

#[test]
fn n20_raw_event_semantics_survive_fully_rehashed_transport() {
    for (field, value) in [
        ("schema", json!("wrong")),
        ("binding_sha256", json!("e".repeat(64))),
        ("sequence", json!(7)),
        ("previous_event_sha256", json!("e".repeat(64))),
    ] {
        let mut f = one_event();
        let mut event: Value = serde_json::from_slice(&f.blobs[CHUNK]).unwrap();
        event[field] = value;
        let mut raw = serde_json::to_vec(&event).unwrap();
        raw.push(b'\n');
        replace_chunk(&mut f, raw);
        semantics(
            &f.publish(),
            &format!("n20-event-{field}"),
            "partial committed event schema/binding/sequence/hash",
        );
    }
    let mut f = one_event();
    let old = f.snapshots[0]["delta"]["cursor"]["last_event_sha256"].clone();
    let mut raw = f.blobs[CHUNK].clone();
    raw.insert(1, b' ');
    replace_chunk(&mut f, raw);
    f.snapshots[0]["delta"]["segments"][0]["last_event_sha256"] = old.clone();
    f.snapshots[0]["delta"]["cursor"]["last_event_sha256"] = old.clone();
    f.snapshots[0]["delta"]["head"]["last_event_sha256"] = old;
    rejected(
        &f.publish(),
        "n20-stale-event-hash",
        "partial segment recomputed end mismatch",
        Pins::StaleEventHash,
        false,
    );
}

#[test]
fn n21_declared_segment_end_must_equal_recomputed_events() {
    let mut f = one_event();
    for path in ["segments", "cursor", "head"] {
        if path == "segments" {
            f.snapshots[0]["delta"][path][0]["last_event_sha256"] = json!("e".repeat(64));
        } else {
            f.snapshots[0]["delta"][path]["last_event_sha256"] = json!("e".repeat(64));
        }
    }
    semantics(
        &f.publish(),
        "n21-recomputed-last-hash",
        "partial segment recomputed end mismatch",
    );
}

#[test]
fn n22_payload_event_requires_its_exact_same_delta_identity() {
    for mismatch_descriptor in [false, true] {
        let mut f = Fixture::new(false);
        f.capture(0, vec![], vec![("payloads/p.bin", vec![1, 2, 3])], None);
        if mismatch_descriptor {
            let mut event: Value = serde_json::from_slice(&f.blobs[CHUNK]).unwrap();
            event["event"]["artifact"]["role"] = json!("wrong-role");
            let mut raw = serde_json::to_vec(&event).unwrap();
            raw.push(b'\n');
            replace_chunk(&mut f, raw);
        } else {
            let raw = f.blobs.remove("payloads/p.bin").unwrap();
            f.blobs.insert("payloads/other.bin".into(), raw);
            rename_descriptors(&mut f.snapshots[0], "payloads/p.bin", "payloads/other.bin");
        }
        semantics(
            &f.publish(),
            if mismatch_descriptor {
                "n22-payload-descriptor"
            } else {
                "n22-payload-absent"
            },
            if mismatch_descriptor {
                "partial payload lacks unique matching same-delta artifact"
            } else {
                "partial capture reference lacks same-delta artifact"
            },
        );
    }
}

#[test]
fn n23_additional_capture_members_cannot_be_silently_ignored() {
    let mut f = one_event();
    let mut extra = f.snapshots[0]["delta"]["artifacts"][0].clone();
    extra["path"] = json!("events/000000000001.jsonl");
    f.blobs
        .insert("events/000000000001.jsonl".into(), f.blobs[CHUNK].clone());
    f.snapshots[0]["delta"]["artifacts"]
        .as_array_mut()
        .unwrap()
        .push(extra);
    preflight(
        &f.publish(),
        "n23-unreferenced-chunk",
        "partial capture declared payload/member totals",
    );
    let mut f = Fixture::new(false);
    f.capture(
        0,
        vec![json!({"kind":"checkpoint"})],
        vec![("payloads/p.bin", vec![])],
        None,
    );
    let mut extra = f.snapshots[0]["delta"]["artifacts"][1].clone();
    extra["path"] = json!("payloads/unreferenced.bin");
    f.blobs.insert("payloads/unreferenced.bin".into(), vec![]);
    f.snapshots[0]["delta"]["artifacts"]
        .as_array_mut()
        .unwrap()
        .push(extra);
    f.snapshots[0]["delta"]["cursor"]["payload_count"] = json!(2);
    f.snapshots[0]["delta"]["head"]["payload_count"] = json!(2);
    semantics(
        &f.publish(),
        "n23-unreferenced-payload",
        "partial capture replay membership/cursor mismatch",
    );
}

#[test]
fn n24_fixed_descriptor_payload_corruption_and_bad_namespace_reject() {
    for truncate in [false, true] {
        let mut f = Fixture::new(false);
        f.capture(0, vec![], vec![("payloads/p.bin", b"abc".to_vec())], None);
        let p = f.publish();
        let sha = hash(b"abc");
        fs::write(
            p.mirror.join(format!("sha256/{}/{sha}", &sha[..2])),
            if truncate {
                b"a".as_slice()
            } else {
                b"xyz".as_slice()
            },
        )
        .unwrap();
        rejected(
            &p,
            if truncate {
                "n24-truncated"
            } else {
                "n24-corrupt"
            },
            if truncate {
                "partial source declared size mismatch"
            } else {
                "partial artifact digest mismatch"
            },
            Pins::FixedPin,
            false,
        );
    }
    let mut f = Fixture::new(false);
    f.capture(0, vec![], vec![("payloads/p.bin", b"abc".to_vec())], None);
    let raw = f.blobs.remove("payloads/p.bin").unwrap();
    f.blobs.insert("elsewhere/p.bin".into(), raw);
    rename_descriptors(&mut f.snapshots[0], "payloads/p.bin", "elsewhere/p.bin");
    preflight(
        &f.publish(),
        "n24-rehashed-wrong-namespace",
        "partial payload namespace",
    );
}

#[test]
fn n25_fixed_policy_and_phase_roles_cannot_downgrade() {
    let mut f = Fixture::new(true);
    f.snapshots[0]["context"]
        .as_array_mut()
        .unwrap()
        .retain(|a| a["path"] != "context/measurement-policy.json");
    preflight(
        &f.publish(),
        "n25-policy-missing",
        "partial initial checked policy missing",
    );
    let mut f = Fixture::new(true);
    let mut policy = saved("measurement-policy.json");
    policy.push(b' ');
    f.replace_artifact("context/measurement-policy.json", policy);
    semantics(&f.publish(), "n25-policy-changed", "fixed policy");
    for (field, value, gate) in [
        ("role", json!("opaque"), "partial context role disagreement"),
        ("encoding", json!("binary"), "partial context encoding"),
    ] {
        let mut f = Fixture::new(true);
        f.add_saved_phase(0, FIRST_START);
        f.snapshots[0]["context"]
            .as_array_mut()
            .unwrap()
            .last_mut()
            .unwrap()[field] = value;
        preflight(&f.publish(), &format!("n25-phase-{field}"), gate);
    }
}

#[test]
fn n26_phase_dependencies_and_exact_start_digest_are_required() {
    for (label, phase, first) in [
        ("n26-end-alone", FIRST_END, false),
        ("n26-gap", NEXT_START, false),
        ("n26-after-unfinished", NEXT_START, true),
    ] {
        let mut f = Fixture::new(true);
        if first {
            f.add_saved_phase(0, FIRST_START);
        }
        f.add_saved_phase(0, phase);
        preflight(&f.publish(), label, "partial structural phase prefix gap");
    }
    let mut f = first_phase();
    mutate_json_artifact(&mut f, "context/telemetry/curve-construction.json", |v| {
        v["start_sha256"] = json!("e".repeat(64))
    });
    semantics(
        &f.publish(),
        "n26-start-digest",
        "phase end binding/operation state",
    );
    let mut f = first_phase();
    mutate_json_artifact(&mut f, "context/telemetry/curve-construction.json", |v| {
        v["returned_ok"] = json!(false);
        v["returned_error"] = json!({"message_preview":"","message_bytes":0,"message_sha256":hash(b""),"truncated":false});
    });
    f.add_saved_phase(0, NEXT_START);
    semantics(
        &f.publish(),
        "n26-after-recorded-error",
        "phase dependency prefix gap or continuation after failure",
    );
}

#[test]
fn n27_native_projection_lifecycle_and_admission_claims_are_checked() {
    for (label, mutation, gate) in [
        ("n27-projection", 0, "phase projection/start disagreement"),
        ("n27-lifecycle", 1, "phase native lifecycle order"),
        ("n27-admission", 2, "phase prohibited admission claim"),
    ] {
        let mut f = first_phase();
        mutate_json_artifact(
            &mut f,
            "context/telemetry/curve-construction.json",
            |v| match mutation {
                0 => v["counter_start_succeeded"] = json!(false),
                1 => {
                    v["collection"]["attempts"][0]["events"][0]["lifecycle"][0]["operation"] =
                        json!("close")
                }
                _ => v["training_cost_eligible"] = json!(true),
            },
        );
        semantics(&f.publish(), label, gate);
    }
}

#[test]
fn n28_later_records_do_not_hide_invalid_earlier_phase_bytes() {
    let mut f = Fixture::new(true);
    f.add_saved_phase(0, FIRST_START);
    mutate_json_artifact(
        &mut f,
        "context/telemetry/curve-construction-start.json",
        |v| v["recording_pid"] = json!(0),
    );
    let at = f.push_snapshot(true);
    f.add_saved_phase(at, FIRST_END);
    f.add_saved_phase(at, NEXT_START);
    semantics(
        &f.publish(),
        "n28-invalid-early-prefix",
        "phase start binding/identity",
    );
}

#[test]
fn n29_uncommitted_tail_is_not_restored_as_evidence() {
    let mut f = one_event();
    let mut raw = f.blobs[CHUNK].clone();
    raw.extend_from_slice(b"{\"uncommitted\":");
    replace_chunk(&mut f, raw);
    semantics(
        &f.publish(),
        "n29-admitted-torn-tail",
        "partial segment consumed bytes/hash/final LF mismatch",
    );
    let p = one_event().publish();
    p.restore()
        .expect("n29 control must restore before namespace mutation");
    fs::write(
        p.artifact_path("events/uncommitted-tail.jsonl"),
        b"{\"tail\":",
    )
    .unwrap();
    audit_rejected(
        &p,
        "n29-stray-tail",
        "partial exact restored namespace mismatch",
    );
}

#[test]
fn n30_saved_claims_cannot_upgrade_partial_evidence() {
    let p = Fixture::new(true).publish();
    p.restore()
        .expect("n30 control must restore before report mutation");
    fs::write(p.out.join("partial-consumer/report.json"), wire(&json!({"cell_complete":true,"solver_resumed":true,"scientific_result_verified":true,"training_cost_eligible":true}))).unwrap();
    let fresh = p
        .audit()
        .expect("saved report must not be trusted as an audit input");
    for flag in [
        "cell_complete",
        "solver_resumed",
        "scientific_result_verified",
        "training_cost_eligible",
    ] {
        assert_eq!(
            fresh[flag],
            json!(false),
            "n30 forged report promoted {flag}"
        );
    }
    p.retain(
        "n30-forged-report",
        "fresh report keeps prohibited claims false",
        &fresh,
    );
    fs::write(
        p.out.join("partial-consumer/helper-ack.json"),
        wire(&json!({"cell_complete":true,"solver_state_restorable":true})),
    )
    .unwrap();
    audit_rejected(
        &p,
        "n30-unadmitted-helper-ack",
        "partial exact restored namespace mismatch",
    );
}
