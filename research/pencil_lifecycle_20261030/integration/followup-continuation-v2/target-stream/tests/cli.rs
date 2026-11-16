use serde_json::json;
use std::{fs, process::Command};
#[test]
fn cli_reports_only_a_syntactic_complete_stream_and_rejects_truncation() {
    let dir = std::env::temp_dir().join(format!("pencil-stream-cli-{}", std::process::id()));
    fs::create_dir_all(&dir).unwrap();
    let header = json!({"schema":"pencil-relation-stream/v1","identity":{"source_revision":"a".repeat(40),"source_sha256":"b".repeat(64),"workload_id":"cli-fixture","run_id":"one"},"convention":"PencilAugmentedRhs","p":19,"r":7,"order":7,"cofactor":1,"seed":0,"source_coefficients":[1,0,0,1],"polynomial":[1,0,0,0,0,1],"generator":{"x":1,"y":2,"infinity":false},"target":{"x":3,"y":4,"infinity":false},"x_to_column":vec![None::<usize>;19],"nuisance_columns":0,"small_columns":[0,1],"column_map_version":0,"denominator":2,"coefficient_cap":100});
    let records = json!([{"sequence":0,"event":{"kind":"Header","data":header}}, {"sequence":1,"event":{"kind":"Terminal","data":{"walks":16,"candidate_lambdas":0,"verified_relations":0,"rejected_candidates":0,"submitted_rows":0,"admitted_rows":0,"recovered":null,"source_verified":false,"failure":"walk_cap"}}}]);
    let expected = dir.join("expected.json");
    let stream = dir.join("stream.json");
    fs::write(&expected, header.to_string()).unwrap();
    fs::write(&stream, records.to_string()).unwrap();
    let invoke = || {
        Command::new(env!("CARGO_BIN_EXE_pencil-target-stream-offline"))
            .arg("--expected-header")
            .arg(&expected)
            .arg(&stream)
            .output()
            .unwrap()
    };
    let ok = invoke();
    assert!(
        ok.status.success(),
        "{}",
        String::from_utf8_lossy(&ok.stderr)
    );
    let out: serde_json::Value = serde_json::from_slice(&ok.stdout).unwrap();
    assert_eq!(out["syntactically_complete"], true);
    assert_eq!(out["scalar_independently_verified"], false);
    fs::write(&stream, "[").unwrap();
    let bad = invoke();
    assert_eq!(bad.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&bad.stderr).contains("JSON packet"));
    fs::remove_dir_all(dir).unwrap();
}
