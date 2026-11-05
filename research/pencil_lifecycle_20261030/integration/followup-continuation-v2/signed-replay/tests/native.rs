use five_leads_study::{three_leads::*, three_leads_followup::*};
use serde_json::{json, Value};
use signed_collector_replay::{replay_jsonl, Limits};
use signed_collector_stream_controls::Capture;
fn encode(rows: &[Value]) -> Vec<u8> {
    rows.iter()
        .map(|r| serde_json::to_string(r).unwrap() + "\n")
        .collect::<String>()
        .into_bytes()
}
fn fixture(projective: bool, rank: bool, cap: u64) -> (Vec<Value>, Value) {
    let inst = scholten_instance(739, 23).unwrap();
    let target = inst.curve.mul(inst.generator, 2);
    let mut c = Capture::new(16384, 8_000_000).unwrap();
    let out = solve_genus2_observed(
        &inst,
        target,
        47,
        cap,
        projective,
        rank,
        Some(&mut |v| c.push(v)),
    )
    .unwrap();
    assert!(c.complete());
    if let Some(k) = out.recovered {
        assert_eq!(inst.curve.mul(inst.generator, k), target);
    }
    // In software controls, reconstruct the expected header from a separate invocation.
    // Production callers must use independently pinned input/custody, not this fixture convention.
    let mut expected = None;
    let _ = solve_genus2_observed(
        &inst,
        target,
        47,
        cap,
        projective,
        rank,
        Some(&mut |v| {
            if v["kind"] == "header" {
                expected = Some(v);
            }
            Err("header only".into())
        }),
    );
    (c.records, expected.unwrap())
}
#[test]
fn six_native_streams() {
    for (projective, rank, cap) in [
        (false, false, 64),
        (true, false, 64),
        (true, true, 64),
        (false, false, 2048),
        (true, false, 2048),
        (true, true, 2048),
    ] {
        let (rows, h) = fixture(projective, rank, cap);
        println!(
            "SIGNED_STREAM {}",
            json!({"expected_header":h,"records":rows})
        );
        let replay = replay_jsonl(&encode(&rows), &h, Limits::default()).unwrap();
        assert_eq!(replay.recovered, if cap == 2048 { Some(2) } else { None });
        println!(
            "SIGNED_REPLAY {}",
            json!({"projective":projective,"rank":rank,"cap":cap,"event_count":rows.len(),"replay":replay})
        );
    }
}
#[test]
fn corruption_and_limits() {
    let (rows, h) = fixture(true, true, 2048);
    let wire = encode(&rows);
    let mut cases = vec![];
    let mut m = rows.clone();
    m[1]["sequence"] = json!(0);
    cases.push(("sequence", m));
    let mut m = rows.clone();
    m[0]["event"]["target"]["inf"] = json!(true);
    cases.push(("target", m));
    let mut m = rows.clone();
    m.pop();
    cases.push(("truncated", m));
    let mut m = rows.clone();
    m.push(json!({"sequence":m.len(),"event":{"kind":"nonsplit","attempt":1}}));
    cases.push(("post_terminal", m));
    let edge = rows
        .iter()
        .position(|r| r["event"]["kind"] == "edge")
        .unwrap();
    for (field, value) in [
        ("a", json!(34147)),
        ("signs", json!([0, 1])),
        ("nodes", json!([99999, 0])),
        ("rank_score", json!(9)),
        ("cycles", json!(9999)),
        ("extra", json!(0)),
    ] {
        let mut m = rows.clone();
        m[edge]["event"][field] = value;
        cases.push((field, m));
    }
    let mut m = rows.clone();
    let last = m.len() - 1;
    m[last]["event"]["attempts"] = json!(0);
    cases.push(("terminal_counts", m));
    let node = rows
        .iter()
        .position(|r| r["event"]["kind"] == "node")
        .unwrap();
    let mut m = rows.clone();
    m[node]["event"]["x"] = json!(739);
    cases.push(("node_domain", m));
    let mut m = rows.clone();
    let sign = m[edge]["event"]["signs"][0].as_i64().unwrap();
    m[edge]["event"]["signs"][0] = json!(-sign);
    cases.push(("wrong_point_sign", m));
    let mut m = rows.clone();
    let x = m[node]["event"]["x"].as_u64().unwrap();
    m[node]["event"]["x"] = json!((x + 1) % 739);
    cases.push(("wrong_root", m));
    let non = rows
        .iter()
        .position(|r| r["event"]["kind"] == "nonsplit")
        .unwrap();
    let mut m = rows.clone();
    m[non - 1]["event"]["divisor"]["u"] = json!([0, 0, 1]);
    cases.push(("false_nonsplit", m));
    for (name, m) in cases {
        assert!(
            replay_jsonl(&encode(&m), &h, Limits::default()).is_err(),
            "accepted {name}"
        );
        println!("REJECTED {name}");
    }
    let bad = String::from_utf8(wire.clone()).unwrap().replacen(
        "\"sequence\":0",
        "\"sequence\":0,\"sequence\":0",
        1,
    );
    assert!(replay_jsonl(bad.as_bytes(), &h, Limits::default())
        .unwrap_err()
        .contains("duplicate JSON key"));
    let nested = String::from_utf8(wire.clone()).unwrap().replacen(
        "\"kind\":\"header\"",
        "\"kind\":\"header\",\"kind\":\"header\"",
        1,
    );
    assert!(replay_jsonl(nested.as_bytes(), &h, Limits::default())
        .unwrap_err()
        .contains("duplicate JSON key"));
    println!("REJECTED nested_duplicate_key");
    let mut budget = Limits::default();
    budget.prefix_cells = 1;
    assert!(replay_jsonl(&wire, &h, budget).is_err());
    let mut budget = Limits::default();
    budget.bytes = 1;
    assert!(replay_jsonl(&wire, &h, budget).is_err());
    let mut budget = Limits::default();
    budget.events = 1;
    assert!(replay_jsonl(&wire, &h, budget).is_err());
    let mut budget = Limits::default();
    budget.nodes = 1;
    assert!(replay_jsonl(&wire, &h, budget).is_err());
    println!("REJECTED duplicate_json_key prefix_cells bytes events nodes");
}
