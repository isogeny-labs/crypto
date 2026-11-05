use serde_json::{json, Value};
use signed_collector_replay::Limits;
use std::{fs, path::PathBuf};
fn fixture(i: usize) -> (Vec<u8>, Value) {
    let p = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../signed-replay/fixtures");
    (
        fs::read(p.join(format!("{i}.jsonl"))).unwrap(),
        serde_json::from_slice(&fs::read(p.join(format!("{i}.header.json"))).unwrap()).unwrap(),
    )
}
fn encode(rows: &[Value]) -> Vec<u8> {
    rows.iter()
        .map(|v| format!("{}\n", v))
        .collect::<String>()
        .into_bytes()
}
#[test]
fn six_saved_streams() {
    let mut total = 0;
    for i in 0..6 {
        let (input, h) = fixture(i);
        let result = signed_scheduler_check::verify(&input, &h, Limits::default()).unwrap();
        total += result["candidates"].as_u64().unwrap();
        println!("SCHEDULER_RESULT {}", json!({"fixture":i,"result":result}));
    }
    assert_eq!(total, 731);
}
#[test]
fn coherent_schedule_corruptions() {
    let (input, h) = fixture(0);
    let original: Vec<Value> = std::str::from_utf8(&input)
        .unwrap()
        .lines()
        .map(|l| serde_json::from_str(l).unwrap())
        .collect();
    for kind in ["seed", "jump_index", "choice"] {
        let mut rows = original.clone();
        let mut header = h.clone();
        match kind {
            "seed" => {
                header["seed"] = json!(h["seed"].as_u64().unwrap() + 1);
                rows[0]["event"] = header.clone();
            }
            "jump_index" => {
                let ids: Vec<_> = rows
                    .iter()
                    .enumerate()
                    .filter(|(_, v)| {
                        v["event"]["kind"] == "candidate" && v["event"]["continuation"] == 1
                    })
                    .map(|(i, v)| (i, v["event"]["jump_index"].as_u64().unwrap()))
                    .collect();
                let new = (0..16)
                    .find(|x| ids.iter().all(|(_, old)| old != x))
                    .unwrap();
                rows[ids[0].0]["event"]["jump_index"] = json!(new);
            }
            "choice" => {
                let pos = rows
                    .iter()
                    .position(|v| v["event"]["kind"] == "choice")
                    .unwrap();
                let old = rows[pos]["event"]["selected_attempt"].as_u64().unwrap();
                rows[pos]["event"]["selected_attempt"] = json!(if old == 1 { 2 } else { 1 });
            }
            _ => unreachable!(),
        }
        let poisoned = encode(&rows);
        signed_collector_label_check::verify(&poisoned, &header, Limits::default()).unwrap();
        let err =
            signed_scheduler_check::verify(&poisoned, &header, Limits::default()).unwrap_err();
        println!(
            "SCHEDULER_NEGATIVE {}",
            json!({"mutation":kind,"graph_and_labels_pass":true,"scheduler_error":err})
        );
    }
}
