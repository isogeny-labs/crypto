use five_leads_study::{three_leads::*, three_leads_followup::*};
use serde_json::json;
use signed_collector_replay::Limits;
use signed_collector_stream_controls::Capture;
use std::{fs, path::PathBuf};
#[test]
fn fixed_native_restart_controls() {
    let out = PathBuf::from(
        std::env::var("SIGNED_NATIVE_OUTPUT").expect("private receipt directory required"),
    );
    fs::create_dir(out.join("streams")).unwrap();
    let inst = scholten_instance(739, 23).unwrap();
    let target = inst.curve.mul(inst.generator, 2);
    let mut failures = vec![];
    for seed in 0..32 {
        for (name, projective, rank) in [("affine", false, false), ("target_rank", true, true)] {
            let mut capture = Capture::new(16384, 8_000_000).unwrap();
            let result = solve_genus2_observed(
                &inst,
                target,
                seed,
                256,
                projective,
                rank,
                Some(&mut |v| capture.push(v)),
            )
            .unwrap();
            assert!(capture.complete());
            let input = capture
                .records
                .iter()
                .map(|v| format!("{}\n", v))
                .collect::<String>();
            let id = format!("{seed:02}-{name}");
            fs::write(out.join("streams").join(format!("{id}.jsonl")), &input).unwrap();
            let header = &capture.records[0]["event"];
            fs::write(
                out.join("streams").join(format!("{id}.header.json")),
                format!("{}\n", header),
            )
            .unwrap();
            let checked =
                signed_scheduler_check::verify(input.as_bytes(), header, Limits::default());
            let native_scalar_verified = result
                .recovered
                .map(|k| inst.curve.mul(inst.generator, k) == target);
            let reasons: Vec<_> = capture
                .records
                .iter()
                .filter(|v| v["event"]["kind"] == "restart")
                .map(|v| v["event"].clone())
                .collect();
            let row = json!({"id":id,"seed":seed,"mode":name,"cap":256,"restarts":reasons,"native_recovered":result.recovered,"native_scalar_verified":native_scalar_verified,"scheduler":checked.as_ref().ok(),"error":checked.as_ref().err(),"events":capture.records.len()});
            fs::write(
                out.join("streams").join(format!("{id}.result.json")),
                format!("{}\n", row),
            )
            .unwrap();
            println!("NATIVE_RESTART_CONTROL {}", row);
            if checked.is_err() || native_scalar_verified == Some(false) {
                failures.push(id);
            }
        }
    }
    assert!(failures.is_empty(), "failing controls: {failures:?}");
}
