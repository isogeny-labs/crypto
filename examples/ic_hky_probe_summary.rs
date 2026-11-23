//! Independently bind finite large-variable probe outcomes to frozen inputs.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::path::Path;

fn hash(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
fn main() {
    let args: Vec<_> = std::env::args().skip(1).collect();
    assert_eq!(args.len(), 1, "STUDY");
    let study = Path::new(&args[0]);
    let mut cells = vec![];
    for (name, input, draw, nv) in [
        (
            "hky-N58-random-d2-20261025-02",
            "m6-n10-ell3-random.jsonl",
            2,
            58,
        ),
        (
            "hky-N60-random-d0-20261025-02",
            "m5-n15-ell3-random.jsonl",
            0,
            60,
        ),
        (
            "hky-N100-random-d0-20261025-02",
            "m10-n10-ell2-random.jsonl",
            0,
            100,
        ),
    ] {
        let p = study.join("runs").join(name).join("cell-0000");
        let input_bytes = std::fs::read(study.join("inputs/fresh").join(input)).unwrap();
        let input_hash = hash(&input_bytes);
        let fixture: Value = std::str::from_utf8(&input_bytes)
            .unwrap()
            .lines()
            .map(|l| serde_json::from_str::<Value>(l).unwrap())
            .find(|r| r["draw"] == draw)
            .unwrap();
        assert_eq!(fixture["solutions"], 0);
        let bytes = std::fs::read(p.join("rows.jsonl")).unwrap();
        let rows: Vec<Value> = std::str::from_utf8(&bytes)
            .unwrap()
            .lines()
            .map(|l| serde_json::from_str(l).unwrap())
            .collect();
        let status: Value =
            serde_json::from_slice(&std::fs::read(p.join("status.json")).unwrap()).unwrap();
        let stderr = std::fs::read_to_string(p.join("stderr.txt")).unwrap();
        assert!(rows
            .iter()
            .all(|r| r["input_sha256"] == input_hash && r["variables"] == nv && r["draw"] == draw));
        let mut degrees = vec![];
        for degree in 2..=4 {
            let first = rows.iter().find(|r| r["degree"] == degree).unwrap();
            assert_eq!(first["status"], "started");
            let terminal = rows
                .iter()
                .find(|r| r["degree"] == degree && r["status"] != "started");
            let outcome = match terminal {
                Some(r) => r["status"].as_str().unwrap(),
                None => {
                    assert_eq!(status["exit_code"], 134);
                    assert!(stderr.contains("memory allocation") && stderr.contains("failed"));
                    "allocation_failed_no_degree_verdict"
                }
            };
            if let Some(r) = terminal {
                if outcome == "saturated_not_refuted" {
                    if r["engine"] == "dense" {
                        assert!(
                            r["closure_dimension"].as_u64().unwrap()
                                >= r["plain_dimension"].as_u64().unwrap()
                        );
                    } else {
                        assert_eq!(r["sparse"]["status"], r["status"]);
                        assert!(
                            r["sparse"]["rank"].as_u64().unwrap()
                                <= r["columns"].as_str().unwrap().parse::<u64>().unwrap()
                        );
                    }
                }
            }
            degrees.push(
                json!({"degree":degree,"columns":first["columns"],"outcome":outcome,
                "complete_fixed_point":outcome=="saturated_not_refuted"||outcome=="refuted",
                "plain_dimension":terminal.map(|r|r["plain_dimension"].clone()),
                "closure_dimension":terminal.map(|r| if r["engine"]=="dense" {r["closure_dimension"].clone()}else{r["sparse"]["rank"].clone()}),
                "sparse_counters":terminal.map(|r|r["sparse"].clone()),
                "rounds":terminal.map(|r|r["rounds"].clone())}),
            );
        }
        cells.push(json!({"run":name,"variables":nv,"input_sha256":input_hash,"input":input,
            "draw":draw,"fixture":fixture,"process":status,"rows_sha256":hash(&bytes),"degrees":degrees,
            "subgroup_query_relevance":null}));
    }
    println!("{}",serde_json::to_string_pretty(&json!({"schema":"ic-hky-probe-summary/v1",
        "status":"native_rows_and_input_hashes_checked","cells":cells,
        "scope":"Three fixed Boolean fixtures only. Degree-two/three fixed points are observations, not independent certificates. All degree-four verdicts remain unknown; no larger-field degree law or DLP gain."})).unwrap());
}
