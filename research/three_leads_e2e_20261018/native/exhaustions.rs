use super::*;
use std::process::{Command, Stdio};
fn record_files(dir: &Path, out: &mut Vec<std::path::PathBuf>) {
    for item in fs::read_dir(dir).unwrap() {
        let p = item.unwrap().path();
        if p.is_dir() {
            record_files(&p, out)
        } else if p.file_name().is_some_and(|s| s == "records.jsonl") {
            out.push(p);
        }
    }
}
pub fn replay(study: &str, binary: &str) {
    let study = Path::new(study);
    let out = study.join("exhaustion-replays");
    fs::create_dir(&out).unwrap();
    let mut paths = vec![];
    record_files(&study.join("sessions"), &mut paths);
    paths.sort();
    let mut jobs = vec![];
    for p in paths {
        for line in fs::read_to_string(p).unwrap().lines() {
            let row: Value = serde_json::from_str(line).unwrap();
            if row["outcome"]["status"] == "exhausted" {
                jobs.push(row);
            }
        }
    }
    let receipts = std::thread::scope(|scope| {
        let mut threads = vec![];
        for lane in 0..4 {
            let jobs = &jobs;
            let out = &out;
            threads.push(scope.spawn(move||{let mut results=vec![];for(index,row)in jobs.iter().enumerate(){if index%4!=lane{continue;}let id=row["record_id"].as_str().unwrap();let w=&row["workload"];let m=&row["method"];let seed=row["algorithm_seed"].as_str().unwrap().parse::<u64>().unwrap();let input=json!({"schema":"ecbench.child_input/v1","curve":w["curve_spec"],"target_seed":w["target_seed"],"target_index":w["target_index"],"expected_workload_id":w["workload_id"],"method":{"id":m["id"],"params":m["params"]},"expected_method_id":m["method_id"],"algorithm_seed":seed});write(out.join(format!("{id}.input.json")),&input);let mut child=Command::new(binary).arg("exec").stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(serde_json::to_string(&input).unwrap().as_bytes()).unwrap();let output=child.wait_with_output().unwrap();fs::write(out.join(format!("{id}.output.json")),&output.stdout).unwrap();if !output.stderr.is_empty(){fs::write(out.join(format!("{id}.stderr")),&output.stderr).unwrap();}let actual:Value=serde_json::from_slice(&output.stdout).unwrap();let report=&actual["report"];let counters=report["counters"]==row["counters"];let fields=report["field_ops"]==row["field_ops"];let phases=report["phases"]==row["phases"];let ok=output.status.success()&&actual["error"].is_null()&&report["exhausted"]==true&&report["recovered"].is_null()&&counters&&fields&&phases;let receipt=json!({"schema":"three-leads.exhaustion-replay/v1","record_id":id,"session_id":row["session_id"],"seq":row["seq"],"expected_status":"exhausted","observed_exhausted":report["exhausted"],"counters_identical":counters,"field_ops_identical":fields,"phases_identical":phases,"ok":ok});write(out.join(format!("{id}.receipt.json")),&receipt);assert!(ok,"exhaustion did not reproduce: {id}");results.push(receipt);}results}));
        }
        threads
            .into_iter()
            .flat_map(|h| h.join().unwrap())
            .collect::<Vec<_>>()
    });
    assert_eq!(receipts.len(), jobs.len());
    write(
        out.join("summary.json"),
        &json!({"exhausted_records":jobs.len(),"reproduced":receipts.len(),"all_identical":true,"binary":binary,"meaning":"ecbench replay-all audits successful measured records; this separately re-executes every retained capped diagnostic and compares exact primitive/native counts and phases","receipts":receipts}),
    );
}
