//! Explicit software-fixture replay from a separately compiled native producer.
use super::*;
use serde::Deserialize;
use std::{
    fs::{File, OpenOptions},
    io::{Cursor, Read, Write},
    path::Path,
};

#[derive(Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
struct Prefix {
    acknowledged_events: u64,
    acknowledged_bytes: u64,
    last_sha256: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct NativeFailure {
    reason: StopReason,
    prefix: Prefix,
    claims: Claims,
}

fn bounded_file(path: &Path, cap: usize) -> Vec<u8> {
    let mut bytes = Vec::new();
    File::open(path)
        .unwrap()
        .take(cap as u64 + 1)
        .read_to_end(&mut bytes)
        .unwrap();
    assert!(bytes.len() <= cap, "oversized fixture {}", path.display());
    bytes
}

fn rehash(events: &[Event]) -> Vec<u8> {
    let mut out = Vec::new();
    let mut previous = "0".repeat(64);
    for (i, event) in events.iter().enumerate() {
        let r = Record {
            schema: SCHEMA.into(),
            sequence: i as u64 + 1,
            previous_sha256: previous,
            event: event.clone(),
        };
        let bytes = serde_json::to_vec(&r).unwrap();
        previous = digest(&bytes);
        out.extend(bytes);
        out.push(b'\n');
    }
    out
}

fn save(path: &Path, value: &impl Serialize) {
    let mut f = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(path)
        .unwrap();
    serde_json::to_writer_pretty(&mut f, value).unwrap();
    f.write_all(b"\n").unwrap();
    f.sync_all().unwrap();
}

#[test]
#[ignore = "requires separately compiled native fixtures and externally supplied source/binary identities"]
fn replay_native_fixtures_and_rehashed_multiword_mutations() {
    let fixture_dir =
        std::path::PathBuf::from(std::env::var("BOOLEAN_NATIVE_FIXTURE_DIR").unwrap());
    let output_dir = std::path::PathBuf::from(std::env::var("BOOLEAN_INTEROP_OUTPUT_DIR").unwrap());
    assert!(fixture_dir.is_absolute() && output_dir.is_absolute());
    std::fs::create_dir(&output_dir).expect("output directory must be new");
    // These fixture-only hashes are frozen by the independent native test helper.
    // Source and actual executing native binary identities come from the runner,
    // not from fields copied out of the supplied trace.
    let expected = Binding {
        source_commit: std::env::var("BOOLEAN_NATIVE_SOURCE_COMMIT").unwrap(),
        producer_sha256: std::env::var("BOOLEAN_NATIVE_BINARY_SHA256").unwrap(),
        configuration_sha256: "2".repeat(64),
        context_sha256: "3".repeat(64),
        call_id: "4".repeat(64),
    };
    validate_binding(&expected).unwrap();
    save(&output_dir.join("expected-binding.json"), &expected);
    let mut counts = Vec::new();
    for (name, rank) in [
        ("empty", 0),
        ("constant", 1),
        ("quotient_collision", 1),
        ("swap", 2),
        ("suffix_boundary", 92),
    ] {
        let raw = bounded_file(&fixture_dir.join(format!("{name}.jsonl")), STREAM_CAP);
        let report = verify(Cursor::new(&raw), &expected).unwrap();
        assert!(report.matrix_trace_complete);
        assert_eq!(report.independent_rank, Some(rank));
        let prefix: Prefix = serde_json::from_slice(&bounded_file(
            &fixture_dir.join(format!("{name}.prefix.json")),
            4096,
        ))
        .unwrap();
        assert_eq!(prefix.acknowledged_events, report.acknowledged_records);
        assert_eq!(prefix.acknowledged_bytes, report.acknowledged_bytes as u64);
        assert_eq!(prefix.last_sha256, report.last_record_sha256);
        save(&output_dir.join(format!("{name}.review.json")), &report);
        let events: Vec<Event> = raw
            .split(|c| *c == b'\n')
            .filter(|s| !s.is_empty())
            .map(|s| serde_json::from_slice::<Record>(s).unwrap().event)
            .collect();
        let multiword: Vec<usize> = events
            .iter()
            .enumerate()
            .filter_map(|(i, event)| {
                matches!(event, Event::RowXor { start_word, .. } if *start_word > 0).then_some(i)
            })
            .collect();
        if name == "suffix_boundary" {
            assert!(
                !multiword.is_empty(),
                "fixture must exercise a nonzero suffix offset"
            );
            let i = multiword[0];
            // Corrupt the omitted word, a suffix word, offset, and source version;
            // all records are rehashed so hash-chain checking alone cannot pass.
            for variant in 0..4 {
                let mut mutated = events.clone();
                if let Event::RowXor {
                    start_word,
                    words_after,
                    pivot_version,
                    ..
                } = &mut mutated[i]
                {
                    match variant {
                        0 => {
                            let w = u64::from_str_radix(&words_after[0], 16).unwrap() ^ 1;
                            words_after[0] = format!("{w:016x}");
                        }
                        1 => {
                            let slot = *start_word as usize;
                            let w = u64::from_str_radix(&words_after[slot], 16).unwrap() ^ 1;
                            words_after[slot] = format!("{w:016x}");
                        }
                        2 => *start_word = 0,
                        _ => *pivot_version += 1,
                    }
                }
                let changed = rehash(&mutated);
                assert!(verify(Cursor::new(&changed), &expected).is_err());
                let mut f = OpenOptions::new()
                    .write(true)
                    .create_new(true)
                    .open(output_dir.join(format!("suffix-tamper-{variant}.jsonl")))
                    .unwrap();
                f.write_all(&changed).unwrap();
                f.sync_all().unwrap();
            }
            for n in [i, i + 1] {
                let prefix = verify(Cursor::new(rehash(&events[..n])), &expected).unwrap();
                assert!(!prefix.matrix_trace_complete && prefix.independent_rank.is_none());
            }
        }
        counts.push(
            serde_json::json!({"name":name,"raw_sha256":digest(&raw),"rank":rank,
            "records":events.len(),"actual_nonzero_suffix_offset_xors":multiword.len()}),
        );
    }
    let censored = bounded_file(&fixture_dir.join("native_row_cap0.jsonl"), STREAM_CAP);
    let report = verify(Cursor::new(&censored), &expected).unwrap();
    assert!(!report.matrix_trace_complete && report.independent_rank.is_none());
    assert_eq!(report.stop_reason, Some(StopReason::NativeSizeLimit));
    assert!(!report.stop_reason_independently_verified);
    let failure: NativeFailure = serde_json::from_slice(&bounded_file(
        &fixture_dir.join("native_row_cap0.failure.json"),
        4096,
    ))
    .unwrap();
    assert_eq!(failure.reason, StopReason::NativeSizeLimit);
    assert_eq!(failure.claims, Claims::unclaimed());
    assert_eq!(
        failure.prefix.acknowledged_events,
        report.acknowledged_records
    );
    assert_eq!(
        failure.prefix.acknowledged_bytes,
        report.acknowledged_bytes as u64
    );
    assert_eq!(failure.prefix.last_sha256, report.last_record_sha256);
    let events: Vec<Event> = censored
        .split(|c| *c == b'\n')
        .filter(|s| !s.is_empty())
        .map(|s| serde_json::from_slice::<Record>(s).unwrap().event)
        .collect();
    assert_eq!(events.len(), 4);
    assert!(matches!(
        events[0],
        Event::Begin {
            native_row_cap: 0,
            ..
        }
    ));
    assert!(
        matches!(&events[2], Event::Candidate { terms, matrix_row: Some(0), .. } if terms == &[0])
    );
    let just_candidate = verify(Cursor::new(rehash(&events[..3])), &expected).unwrap();
    assert!(!just_candidate.matrix_trace_complete && just_candidate.independent_rank.is_none());
    save(&output_dir.join("native_row_cap0.review.json"), &report);
    save(
        &output_dir.join("interop.json"),
        &serde_json::json!({
            "schema":"crypto.boolean-matrix-software-interop/v1", "fixtures":counts,
            "rehashed_multiword_mutations_rejected":4, "independent_rank_checks":5,
            "native_censored_row_cap_prefix_checked":true,
            "censored_raw_sha256":digest(&censored),
            "producer_execution_attested":false,"scientific_dataset_generated":false,
            "first_or_last_fall_degree_claim":false
        }),
    );
}
