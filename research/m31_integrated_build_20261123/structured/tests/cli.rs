use m31_structured_f4::{replay_witness, Limits, Problem, SignedWitness};
use serde_json::{json, Value};
use std::io::Write;
use std::process::{Command, Stdio};

fn request() -> Value {
    json!({
        "problem": {
            "p": 2147483647u64, "a": 847514846u64, "b": 2087444724u64,
            "target": [625037715u64, 2100627251u64],
            "selectors": [[1522445932u64, 1], [1522445932u64, 1], [1522445932u64, 1]]
        },
        "formulation": "chain_left", "preprocessing": "require_selector_quotient"
    })
}

fn execute(value: &Value) -> (i32, Value) {
    let mut child = Command::new(env!("CARGO_BIN_EXE_m31-structured-f4"))
        .env("RAYON_NUM_THREADS", "1")
        .stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped())
        .spawn().unwrap();
    child.stdin.take().unwrap().write_all(&serde_json::to_vec(value).unwrap()).unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(output.stderr.is_empty(), "{}", String::from_utf8_lossy(&output.stderr));
    (output.status.code().unwrap(), serde_json::from_slice(&output.stdout).unwrap())
}

#[test]
fn public_cli_complete_witnesses_replay() {
    let input = request();
    let (code, output) = execute(&input);
    assert_eq!(code, 0);
    assert_eq!(output["status"], "complete");
    let problem: Problem = serde_json::from_value(input["problem"].clone()).unwrap();
    let witnesses = output["witnesses"].as_array().unwrap();
    assert_eq!(witnesses.len(), 3);
    for witness in witnesses {
        let points = serde_json::from_value(witness["points"].clone()).unwrap();
        assert!(replay_witness(&problem, &SignedWitness { points }));
    }
}

#[test]
fn public_cli_output_limit_has_distinct_exit() {
    let mut input = request();
    input["limits"] = serde_json::to_value(Limits { max_witnesses: 1, ..Limits::default() }).unwrap();
    let (code, output) = execute(&input);
    assert_eq!(code, 3);
    assert_eq!(output["status"], "incomplete");
    assert_eq!(output["stop_reason"], "signed witness cap");
}

#[test]
fn public_cli_refuses_wrong_field_and_unknown_limit() {
    let mut input = request(); input["problem"]["p"] = json!(31);
    let (code, output) = execute(&input);
    assert_eq!(code, 2);
    assert_eq!(output["status"], "refused");
    let mut input = request();
    input["limits"] = serde_json::to_value(Limits::default()).unwrap();
    input["limits"]["unchecked_memory"] = json!(1_000_000_000u64);
    assert_eq!(execute(&input).0, 2);
}
