use super::*;
use std::time::Duration;

fn binding() -> TraceBinding {
    TraceBinding {
        source_commit: "0".repeat(40),
        producer_sha256: "1".repeat(64),
        configuration_sha256: "2".repeat(64),
        context_sha256: "3".repeat(64),
        call_id: "4".repeat(64),
    }
}

fn poly(n_vars: usize, terms: &[u8]) -> F2BoolPoly {
    F2BoolPoly::from_monos(
        terms
            .iter()
            .map(|&m| F2BoolMono::from_mask(Mask::from(m)))
            .collect(),
        n_vars,
    )
}

fn capture(
    polys: &[F2BoolPoly],
    n_vars: usize,
    degree: u32,
    limits: TraceLimits,
) -> (Result<TracedMatrixOutcome, TraceFailure>, Vec<Vec<u8>>) {
    let mut lines = Vec::new();
    let result = matrix_f4_f2_traced(
        polys,
        n_vars,
        degree,
        &binding(),
        limits,
        Instant::now() + Duration::from_secs(30),
        &mut || false,
        &mut |line: &[u8]| {
            lines.push(line.to_vec());
            Ok(())
        },
    );
    (result, lines)
}

fn records(lines: &[Vec<u8>]) -> Vec<serde_json::Value> {
    lines
        .iter()
        .map(|line| serde_json::from_slice(line).unwrap())
        .collect()
}

fn assert_prefix(lines: &[Vec<u8>], prefix: &TracePrefix) {
    let mut previous = "0".repeat(64);
    let mut bytes = 0;
    for (index, line) in lines.iter().enumerate() {
        assert_eq!(line.last(), Some(&b'\n'));
        assert!(!line[..line.len() - 1].contains(&b'\n'));
        let value: serde_json::Value = serde_json::from_slice(line).unwrap();
        assert_eq!(value["schema"], SCHEMA);
        assert_eq!(value["sequence"], (index + 1) as u64);
        assert_eq!(value["previous_sha256"], previous);
        previous = format!("{:x}", Sha256::digest(&line[..line.len() - 1]));
        bytes += line.len() as u64;
    }
    assert_eq!(prefix.acknowledged_events, lines.len() as u64);
    assert_eq!(prefix.acknowledged_bytes, bytes);
    assert_eq!(prefix.last_sha256, previous);
}

fn assert_no_claims(claims: &TraceClaims) {
    let value = serde_json::to_value(claims).unwrap();
    for key in [
        "input_encoding_checked",
        "matrix_rank_independently_verified",
        "execution_stream_complete",
        "total_operation_count_verified",
        "unsat_verified",
        "groebner_basis_complete",
    ] {
        assert_eq!(value[key], false);
    }
    assert!(value["first_fall_degree"].is_null());
    assert!(value["last_fall_degree"].is_null());
}

#[test]
fn empty_zero_variable_input_completes_without_claims() {
    let (result, lines) = capture(&[], 0, 0, TraceLimits::default());
    let result = result.unwrap();
    assert_eq!(result.rank_native, 0);
    assert_eq!(result.word_xors_native, 0);
    assert!(result.reduced_rows.is_empty());
    assert_prefix(&lines, &result.prefix);
    assert_no_claims(&result.claims);
    let events = records(&lines);
    assert_eq!(events.last().unwrap()["event"]["admission_units"], 0);
    assert_eq!(events.last().unwrap()["event"]["kind"], "end");
}

#[test]
fn constant_zero_variable_input_uses_self_swap_and_low_bit() {
    let (result, lines) = capture(&[F2BoolPoly::one(0)], 0, 0, TraceLimits::default());
    let result = result.unwrap();
    assert_eq!(result.rank_native, 1);
    assert_eq!(result.reduced_rows, [F2BoolPoly::one(0)]);
    assert_no_claims(&result.claims);
    let events = records(&lines);
    let swap = &events
        .iter()
        .find(|r| r["event"]["kind"] == "swap")
        .unwrap()["event"];
    assert_eq!(swap["left"], 0);
    assert_eq!(swap["right"], 0);
    assert_eq!(swap["left_after_version"], 1);
    assert_eq!(swap["right_after_version"], 1);
    let row = &events
        .iter()
        .find(|r| r["event"]["kind"] == "matrix_row")
        .unwrap()["event"];
    assert_eq!(row["words"], serde_json::json!(["0000000000000001"]));
    // degree scan1 + candidate4 + packing2 + search1 + swap1 + output1.
    assert_eq!(events.last().unwrap()["event"]["admission_units"], 10);
}

#[test]
fn quotient_collision_keeps_zero_candidate_in_native_order() {
    let (result, lines) = capture(&[poly(1, &[1, 0])], 1, 2, TraceLimits::default());
    let result = result.unwrap();
    assert_eq!(result.rank_native, 1);
    let events = records(&lines);
    let candidates: Vec<_> = events
        .iter()
        .filter(|r| r["event"]["kind"] == "candidate")
        .collect();
    assert_eq!(candidates.len(), 2);
    assert_eq!(candidates[0]["event"]["multiplier"], 0);
    assert_eq!(candidates[0]["event"]["terms"], serde_json::json!([0, 1]));
    assert_eq!(candidates[1]["event"]["multiplier"], 1);
    assert_eq!(candidates[1]["event"]["terms"], serde_json::json!([]));
    assert!(candidates[1]["event"]["matrix_row"].is_null());
    assert_prefix(&lines, &result.prefix);
}

#[test]
fn zero_generator_enumerates_combination_order() {
    let (result, lines) = capture(&[F2BoolPoly::zero(4)], 4, 2, TraceLimits::default());
    assert_eq!(result.unwrap().rank_native, 0);
    let events = records(&lines);
    let multipliers: Vec<_> = events
        .iter()
        .filter(|r| r["event"]["kind"] == "candidate")
        .map(|r| r["event"]["multiplier"].as_u64().unwrap())
        .collect();
    assert_eq!(multipliers, [0, 1, 2, 4, 8, 3, 5, 9, 6, 10, 12]);
    assert_eq!(events.last().unwrap()["event"]["admission_units"], 11);
}

#[test]
fn traced_native_result_and_xor_count_match_untraced_wrapper() {
    // f=x+y, g=y+1 give the two independent rows x+1 and y+1.
    let polys = [poly(2, &[1, 2]), poly(2, &[2, 0])];
    let expected = super::super::matrix_f4_f2_counted(&polys, 2, 1).unwrap();
    let (result, lines) = capture(&polys, 2, 1, TraceLimits::default());
    let result = result.unwrap();
    assert_eq!(result.rank_native, 2);
    assert_eq!(result.reduced_rows, [poly(2, &[1, 0]), poly(2, &[2, 0])]);
    assert_eq!(result.reduced_rows, expected.0);
    assert_eq!(result.word_xors_native, expected.1);
    assert_eq!(result.word_xors_native, 1);
    assert_prefix(&lines, &result.prefix);
}

#[test]
fn nontrivial_swap_tracks_slot_versions_and_null_pivot() {
    let polys = [poly(2, &[2]), poly(2, &[1, 2]), poly(2, &[1, 2])];
    let (result, lines) = capture(&polys, 2, 1, TraceLimits::default());
    assert_eq!(result.unwrap().rank_native, 2);
    let events = records(&lines);
    let swap = &events
        .iter()
        .find(|r| r["event"]["kind"] == "swap")
        .unwrap()["event"];
    assert_eq!(swap["left"], 0);
    assert_eq!(swap["right"], 1);
    assert_eq!(swap["left_version"], 0);
    assert_eq!(swap["right_version"], 0);
    assert_eq!(swap["left_after_version"], 1);
    assert_eq!(swap["right_after_version"], 1);
    let (result, lines) = capture(
        &[poly(1, &[1, 0]), poly(1, &[1, 0])],
        1,
        1,
        TraceLimits::default(),
    );
    assert_eq!(result.unwrap().rank_native, 1);
    assert!(records(&lines)
        .iter()
        .any(|r| r["event"]["kind"] == "pivot" && r["event"]["selected_row"].is_null()));
}

#[test]
fn suffix_word_boundary_counts_zero_result_words() {
    let polys: Vec<_> = (0..8).map(|v| poly(8, &[1 << v])).collect();
    let expected = super::super::matrix_f4_f2_counted(&polys, 8, 3).unwrap();
    let (result, lines) = capture(&polys, 8, 3, TraceLimits::default());
    let result = result.unwrap();
    assert_eq!(result.rank_native, 92);
    assert_eq!(result.reduced_rows, expected.0);
    assert_eq!(result.word_xors_native, expected.1);
    let events = records(&lines);
    let mut counted = 0;
    let mut second_word = false;
    for record in &events {
        let event = &record["event"];
        if event["kind"] == "row_xor" {
            let start = event["start_word"].as_u64().unwrap();
            counted += 2 - start;
            second_word |= start == 1;
            assert_eq!(
                event["words_after"],
                serde_json::json!(["0000000000000000", "0000000000000000"])
            );
        }
    }
    assert!(second_word);
    assert_eq!(counted, result.word_xors_native);
}

#[test]
fn malformed_or_degree_overflow_inputs_fail_before_sink() {
    let cases = [
        (vec![poly(5, &[31])], 5, 4),
        (vec![poly(2, &[1])], 1, 1),
        (vec![poly(1, &[2])], 1, 1),
        (
            vec![F2BoolPoly {
                n_vars: 2,
                terms: vec![F2BoolMono::from_mask(1), F2BoolMono::from_mask(1)],
            }],
            2,
            1,
        ),
        (
            vec![F2BoolPoly {
                n_vars: 2,
                terms: vec![F2BoolMono::from_mask(2), F2BoolMono::from_mask(1)],
            }],
            2,
            1,
        ),
    ];
    for (polys, n_vars, degree) in cases {
        let (result, lines) = capture(&polys, n_vars, degree, TraceLimits::default());
        let failure = result.unwrap_err();
        assert!(matches!(failure.reason, TraceStop::InvalidInput(_)));
        assert!(lines.is_empty());
        assert_prefix(&lines, &failure.prefix);
        assert_no_claims(&failure.claims);
    }
    let (result, lines) = capture(&[], 9, 1, TraceLimits::default());
    assert!(matches!(
        result.unwrap_err().reason,
        TraceStop::UnsupportedProfile(_)
    ));
    assert!(lines.is_empty());
}

#[test]
fn noncanonical_binding_fails_before_sink() {
    let mut declared = binding();
    declared.producer_sha256 = "A".repeat(64);
    let mut called = false;
    let failure = matrix_f4_f2_traced(
        &[],
        0,
        0,
        &declared,
        TraceLimits::default(),
        Instant::now() + Duration::from_secs(30),
        &mut || false,
        &mut |_: &[u8]| {
            called = true;
            Ok(())
        },
    )
    .unwrap_err();
    assert!(matches!(failure.reason, TraceStop::InvalidInput(_)));
    assert!(!called);
}

#[test]
fn selected_unsupported_profiles_and_native_caps_are_checked_in_child_processes() {
    // Per-process environment avoids changing layout selection under concurrent tests.
    let test_name = format!(
        "{}::selected_profile_child",
        module_path!().split_once("::").unwrap().1
    );
    for (key, value, expected) in [
        ("KIC_F4_ACTIVE_MULTIPLIERS", "1", "unsupported"),
        ("KIC_F4_SOLVER_LINEAR_TAIL", "1", "unsupported"),
        ("KIC_F4_FLAT_MATRIX", "1", "unsupported"),
        ("KIC_F4_STD_COLUMN_HASH", "1", "unsupported"),
        ("KIC_F4_HASH_COLUMN_INDEX", "1", "unsupported"),
        ("KIC_F4_CRITERION", "f5", "unsupported"),
        ("F4_F2_MAX_ROWS", "0", "native_size"),
        ("F4_F2_MAX_COLS", "0", "native_size"),
    ] {
        let output = std::process::Command::new(std::env::current_exe().unwrap())
            .args([&test_name, "--ignored", "--exact", "--nocapture"])
            .env(key, value)
            .env("F4_TRACE_TEST_EXPECTED", expected)
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{key}: {}{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(String::from_utf8_lossy(&output.stdout).contains("1 passed"));
    }
}

#[test]
#[ignore = "isolated environment helper, invoked by its parent test"]
fn selected_profile_child() {
    let expected = std::env::var("F4_TRACE_TEST_EXPECTED").expect("child test authorization");
    let (result, lines) = capture(&[F2BoolPoly::one(0)], 0, 0, TraceLimits::default());
    let failure = result.unwrap_err();
    if expected == "unsupported" {
        assert!(matches!(failure.reason, TraceStop::UnsupportedProfile(_)));
        assert!(lines.is_empty());
    } else {
        assert_eq!(expected, "native_size");
        assert_eq!(failure.reason, TraceStop::NativeSizeLimit);
        assert!(!lines.is_empty());
        assert_prefix(&lines, &failure.prefix);
    }
}

#[test]
fn sink_failure_at_every_boundary_preserves_acknowledged_prefix() {
    let polys = [poly(2, &[1, 2]), poly(2, &[2, 0])];
    let (baseline, complete) = capture(&polys, 2, 1, TraceLimits::default());
    baseline.unwrap();
    for fail_at in 0..complete.len() {
        let mut acknowledged = Vec::new();
        let mut calls = 0;
        let failure = matrix_f4_f2_traced(
            &polys,
            2,
            1,
            &binding(),
            TraceLimits::default(),
            Instant::now() + Duration::from_secs(30),
            &mut || false,
            &mut |line: &[u8]| {
                let current = calls;
                calls += 1;
                if current == fail_at {
                    return Err("ambiguous partial write".into());
                }
                acknowledged.push(line.to_vec());
                Ok(())
            },
        )
        .unwrap_err();
        assert!(matches!(failure.reason, TraceStop::SinkFailure(_)));
        assert_eq!(
            calls,
            fail_at + 1,
            "must never retry or emit stopped after failed append"
        );
        assert_eq!(acknowledged, complete[..fail_at]);
        assert_prefix(&acknowledged, &failure.prefix);
        assert_no_claims(&failure.claims);
    }
}

#[test]
fn cumulative_limits_censor_without_completion_claims() {
    let cases = [
        (
            TraceLimits {
                max_candidates: 0,
                ..TraceLimits::default()
            },
            TraceStop::CandidateLimit,
        ),
        (
            TraceLimits {
                max_events: 2,
                ..TraceLimits::default()
            },
            TraceStop::EventLimit,
        ),
        (
            TraceLimits {
                max_stream_bytes: 2048,
                ..TraceLimits::default()
            },
            TraceStop::StreamByteLimit,
        ),
        (
            TraceLimits {
                max_event_bytes: 1024,
                ..TraceLimits::default()
            },
            TraceStop::EventByteLimit,
        ),
        (
            TraceLimits {
                max_workspace_bytes: 128 * 1024,
                ..TraceLimits::default()
            },
            TraceStop::WorkspaceLimit,
        ),
        (
            TraceLimits {
                max_memory_bytes: 0,
                ..TraceLimits::default()
            },
            TraceStop::MemoryLimit,
        ),
        (
            TraceLimits {
                max_admission_units: 0,
                ..TraceLimits::default()
            },
            TraceStop::AdmissionLimit,
        ),
    ];
    for (limits, expected) in cases {
        let max_events = limits.max_events;
        let max_bytes = limits.max_stream_bytes;
        let (result, lines) = capture(&[poly(1, &[1, 0])], 1, 2, limits);
        let failure = result.unwrap_err();
        assert_eq!(failure.reason, expected);
        assert_prefix(&lines, &failure.prefix);
        assert!(failure.prefix.acknowledged_events <= max_events);
        assert!(failure.prefix.acknowledged_bytes <= max_bytes);
        assert!(!records(&lines).iter().any(|r| r["event"]["kind"] == "end"));
        assert_no_claims(&failure.claims);
    }
}

#[test]
fn failed_terminal_sink_is_sticky_even_after_limit_stop() {
    let limits = TraceLimits {
        max_events: 2,
        ..TraceLimits::default()
    };
    let mut calls = 0;
    let mut lines = Vec::new();
    let failure = matrix_f4_f2_traced(
        &[poly(1, &[1])],
        1,
        1,
        &binding(),
        limits,
        Instant::now() + Duration::from_secs(30),
        &mut || false,
        &mut |line: &[u8]| {
            calls += 1;
            if calls == 2 {
                return Err("terminal write failed".into());
            }
            lines.push(line.to_vec());
            Ok(())
        },
    )
    .unwrap_err();
    assert!(matches!(failure.reason, TraceStop::SinkFailure(_)));
    assert_eq!(calls, 2);
    assert_prefix(&lines, &failure.prefix);
}

#[test]
fn cancellation_and_deadline_preserve_prefix_without_end() {
    for expired in [false, true] {
        let mut calls = 0;
        let mut lines = Vec::new();
        let deadline = if expired {
            Instant::now()
        } else {
            Instant::now() + Duration::from_secs(30)
        };
        let failure = matrix_f4_f2_traced(
            &[poly(2, &[1, 2])],
            2,
            2,
            &binding(),
            TraceLimits::default(),
            deadline,
            &mut || {
                calls += 1;
                calls >= 5
            },
            &mut |line: &[u8]| {
                lines.push(line.to_vec());
                Ok(())
            },
        )
        .unwrap_err();
        assert_eq!(
            failure.reason,
            if expired {
                TraceStop::Deadline
            } else {
                TraceStop::Cancelled
            }
        );
        assert_prefix(&lines, &failure.prefix);
        assert!(!records(&lines).iter().any(|r| r["event"]["kind"] == "end"));
        if !expired {
            assert!(!lines.is_empty());
        }
    }
}

/// Explicit local software fixtures only; never runs in the ordinary suite.
/// The caller supplies source/binary metadata and a new external output directory.
#[test]
#[ignore = "explicit bounded fixture export for the independent checker"]
fn write_checker_fixtures() {
    use std::io::Write;
    let destination = std::path::PathBuf::from(
        std::env::var("F4_TRACE_FIXTURE_DIR").expect("F4_TRACE_FIXTURE_DIR"),
    );
    assert!(destination.is_absolute());
    std::fs::create_dir(&destination).expect("fixture destination must be new");
    let mut declared = binding();
    declared.source_commit =
        std::env::var("F4_TRACE_SOURCE_COMMIT").expect("F4_TRACE_SOURCE_COMMIT");
    declared.producer_sha256 =
        std::env::var("F4_TRACE_PRODUCER_SHA256").expect("F4_TRACE_PRODUCER_SHA256");
    let cases = [
        ("empty", vec![], 0, 0),
        ("constant", vec![F2BoolPoly::one(0)], 0, 0),
        ("quotient_collision", vec![poly(1, &[1, 0])], 1, 2),
        (
            "swap",
            vec![poly(2, &[2]), poly(2, &[1, 2]), poly(2, &[1, 2])],
            2,
            1,
        ),
        (
            "suffix_boundary",
            (0..8).map(|v| poly(8, &[1 << v])).collect(),
            8,
            3,
        ),
    ];
    for (name, polys, n_vars, degree) in cases {
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(destination.join(format!("{name}.jsonl")))
            .unwrap();
        let result = matrix_f4_f2_traced(
            &polys,
            n_vars,
            degree,
            &declared,
            TraceLimits::default(),
            Instant::now() + Duration::from_secs(30),
            &mut || false,
            &mut |line: &[u8]| file.write_all(line).map_err(|e| e.to_string()),
        )
        .unwrap();
        file.sync_all().unwrap();
        assert_no_claims(&result.claims);
        let receipt = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(destination.join(format!("{name}.prefix.json")))
            .unwrap();
        serde_json::to_writer(receipt, &result.prefix).unwrap();
    }
}
