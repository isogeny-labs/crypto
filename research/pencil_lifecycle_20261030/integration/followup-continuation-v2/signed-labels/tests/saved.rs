use serde_json::{json, Value};
use signed_collector_label_check::verify;
use signed_collector_replay::{parse_expected_header, replay_jsonl, Limits};
use std::{fs, path::PathBuf};
fn inputs(i: usize) -> (Vec<u8>, Value) {
    let p = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../signed-replay/fixtures");
    (
        fs::read(p.join(format!("{i}.jsonl"))).unwrap(),
        parse_expected_header(&fs::read(p.join(format!("{i}.header.json"))).unwrap()).unwrap(),
    )
}
fn encode(rows: &[Value]) -> Vec<u8> {
    rows.iter()
        .map(|v| serde_json::to_string(v).unwrap() + "\n")
        .collect::<String>()
        .into_bytes()
}
#[test]
fn saved_stream_labels() {
    for i in 0..6 {
        let (input, h) = inputs(i);
        let v = verify(&input, &h, Limits::default()).unwrap();
        assert_eq!(v.recovered, if i >= 3 { Some(2) } else { None });
        assert!(!v.independent_arithmetic);
        println!("LABEL_CHECK {}", json!({"fixture":i,"result":v}));
    }
}
#[test]
fn coherent_label_poison_requires_group_check() {
    let (input, h) = inputs(0);
    let mut rows: Vec<Value> = std::str::from_utf8(&input)
        .unwrap()
        .lines()
        .map(|s| serde_json::from_str(s).unwrap())
        .collect();
    // Nonsplit candidates never enter the graph: altering a canonical label preserves
    // graph-only replay, but must fail the recorded D=aG'+bQ' equation.
    let i = rows
        .iter()
        .position(|r| r["event"]["kind"] == "nonsplit")
        .unwrap();
    assert_eq!(rows[i - 1]["event"]["kind"], "candidate");
    let a = rows[i - 1]["event"]["a"].as_u64().unwrap();
    let r = h["r"].as_u64().unwrap();
    rows[i - 1]["event"]["a"] = json!((a + 1) % r);
    let poisoned = encode(&rows);
    assert!(replay_jsonl(&poisoned, &h, Limits::default()).is_ok());
    let err = verify(&poisoned, &h, Limits::default()).unwrap_err();
    assert!(err.contains("label equation mismatch"), "{err}");
    println!("REJECTED coherent_nonsplit_label {err}");
    // The stream and supplied expected header agree, but the actual transfer does not.
    let (input, mut h) = inputs(0);
    let mut rows: Vec<Value> = std::str::from_utf8(&input)
        .unwrap()
        .lines()
        .map(|s| serde_json::from_str(s).unwrap())
        .collect();
    h["transferred_target"] = h["transferred_generator"].clone();
    rows[0]["event"] = h.clone();
    let poisoned = encode(&rows);
    assert!(replay_jsonl(&poisoned, &h, Limits::default()).is_ok());
    assert!(verify(&poisoned, &h, Limits::default())
        .unwrap_err()
        .contains("transferred point mismatch"));
    println!("REJECTED coherent_transfer_header");
}
