//! Public-fixture integration checks for the actual supervisor/worker CLI.
use serde_json::{json, Value};
use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, Output},
    sync::atomic::{AtomicU64, Ordering},
    time::{Duration, Instant},
};
static NEXT: AtomicU64 = AtomicU64::new(0);
struct Temp(PathBuf);
impl Temp {
    fn new() -> Self {
        let p = std::env::temp_dir().join(format!(
            "factor-base-sweep-test-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&p).unwrap();
        Self(p)
    }
}
impl Drop for Temp {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
fn exe() -> &'static str {
    env!("CARGO_BIN_EXE_factor-base-sweep")
}
fn call(args: &[&str]) -> Output {
    Command::new(exe()).args(args).output().unwrap()
}
fn value(path: &Path) -> Value {
    serde_json::from_slice(&fs::read(path).unwrap()).unwrap()
}
fn complete_event_recorded(path: &Path, expected: &str) -> bool {
    fs::read_to_string(path)
        .map(|contents| {
            contents.split_inclusive('\n').any(|line| {
                line.strip_suffix('\n')
                    .and_then(|record| serde_json::from_str::<Value>(record).ok())
                    .is_some_and(|event| event["event"].as_str() == Some(expected))
            })
        })
        .unwrap_or(false)
}
fn plan(temp: &Temp, prime: bool) -> PathBuf {
    let p = temp.0.join("plan.json");
    assert!(call(&[
        "plan",
        "--out",
        p.to_str().unwrap(),
        "--preset",
        if prime { "prime" } else { "binary" }
    ])
    .status
    .success());
    let mut v = value(&p);
    v["candidates"] = json!([v["candidates"][0].clone()]);
    if !prime {
        v["candidates"][0]["dimension"] = json!(2);
    }
    v["train"] = json!(4);
    v["holdout"] = json!(2);
    v["candidate_seconds"] = json!(20);
    fs::write(&p, serde_json::to_vec_pretty(&v).unwrap()).unwrap();
    p
}
fn launch(p: &Path, dir: &Path) -> Output {
    call(&[
        "run",
        "--plan",
        p.to_str().unwrap(),
        "--out",
        dir.to_str().unwrap(),
        "--heartbeat-seconds",
        "0.05",
    ])
}
fn attempt(dir: &Path) -> PathBuf {
    fs::read_dir(dir.join("candidates"))
        .unwrap()
        .next()
        .unwrap()
        .unwrap()
        .path()
        .join("attempt-0001")
}
#[test]
fn binary_and_prime_probes_replay_and_resume_without_overwriting() {
    for prime in [false, true] {
        let t = Temp::new();
        let p = plan(&t, prime);
        let dir = t.0.join("nested/run");
        let output = launch(&p, &dir);
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        let summary = value(&dir.join("summary-0001.json"));
        assert_eq!(
            summary["candidates"][0]["outcome"], "completed",
            "{summary}"
        );
        let a = attempt(&dir);
        let original = fs::read(a.join("base.json")).unwrap();
        let complete = fs::read(a.join("complete.json")).unwrap();
        let result = value(&a.join("result.json"));
        assert_eq!(result["trials"].as_array().unwrap().len(), 7);
        assert_eq!(result["trials"][0]["witness_verified"], true);
        assert!(result["certified_degree_of_regularity"].is_null());
        assert!(result["full_pipeline_speedup"].is_null());
        // The advisory degree-of-regularity forecast travels with the result,
        // every trial record carries its running view, the supervisor's
        // independent replay from the records matches the worker's copy, and
        // the run summary ranks candidates without having stopped any.
        let forecast = &result["degree_of_regularity_forecast"];
        assert_eq!(forecast["schema"], "dreg_forecast/v1", "{result}");
        assert_eq!(forecast["solves"], 6);
        assert!(forecast["certified_degree_of_regularity"].is_null());
        assert!(forecast["all_targets"]["predicted_degree_label"].is_string());
        assert!(
            forecast["cost"]["hope_ratio_band"]
                .as_array()
                .unwrap()
                .len()
                == 3
        );
        assert!(result["trials"][6]["forecast"]["advice"].is_string());
        assert_eq!(
            result["trials"][6]["independent_rank"],
            forecast["cost"]["rank"]
        );
        let candidate = &summary["candidates"][0];
        assert_eq!(
            candidate["forecast_replay_matches_worker"], true,
            "{candidate}"
        );
        assert_eq!(candidate["forecast"]["solves"], 6);
        assert_eq!(summary["triage"]["rows"].as_array().unwrap().len(), 1);
        assert!(summary["triage"]["certified_degree_of_regularity"].is_null());
        let triage_out = t.0.join(format!("triage-{prime}.json"));
        let triaged = call(&[
            "triage",
            "--run",
            dir.to_str().unwrap(),
            "--out",
            triage_out.to_str().unwrap(),
        ]);
        assert!(
            triaged.status.success(),
            "{}",
            String::from_utf8_lossy(&triaged.stderr)
        );
        let triage = value(&triage_out);
        assert_eq!(triage["schema"], "factor_base_sweep.triage/v1");
        assert_eq!(triage["rows"][0]["solves"], 6);
        assert_eq!(triage["rows"][0]["advice"], candidate["forecast"]["advice"]);
        // Linear algebra is charged beside the solver work on the training
        // rows, and the next plan comes from the triage without running.
        if forecast["relations"].as_u64().unwrap() > 0 {
            assert!(
                forecast["cost"]["linear_algebra"]["observed_rows"]
                    .as_u64()
                    .unwrap()
                    >= 1
            );
            assert!(
                forecast["cost"]["projected_relation_phase_work_with_linear_algebra"].is_number()
            );
        } else {
            assert!(forecast["cost"]["linear_algebra"].is_null());
        }
        let next_plan = t.0.join(format!("next-{prime}.json"));
        let refined = call(&[
            "refine",
            "--run",
            dir.to_str().unwrap(),
            "--out",
            next_plan.to_str().unwrap(),
            "--drop",
            "hopeless_at_bound",
            "--seed-count",
            "2",
        ]);
        assert!(
            refined.status.success(),
            "{}",
            String::from_utf8_lossy(&refined.stderr)
        );
        let next = value(&next_plan);
        assert_eq!(next["schema"], "factor_base_sweep/v1");
        assert_eq!(
            next["candidates"].as_array().unwrap().len(),
            if prime { 1 } else { 2 }
        );
        assert_eq!(next["train"], 4, "budgets are inherited");
        let provenance = value(&t.0.join(format!("next-{prime}.refine.json")));
        assert_eq!(provenance["schema"], "factor_base_sweep.refine/v1");
        assert_eq!(provenance["kept"].as_array().unwrap().len(), 1);
        assert!(provenance["advisory"]
            .as_str()
            .unwrap()
            .contains("nothing was run"));
        // The refined plan is a valid plan for a new run directory.
        assert!(call(&[
            "plan",
            "--out",
            t.0.join(format!("unused-{prime}.json")).to_str().unwrap()
        ])
        .status
        .success());
        let bad = call(&[
            "refine",
            "--run",
            dir.to_str().unwrap(),
            "--out",
            next_plan.to_str().unwrap(),
        ]);
        assert!(!bad.status.success(), "write-once output");
        let checked = call(&["verify", "--run", dir.to_str().unwrap()]);
        assert!(
            checked.status.success(),
            "{}",
            String::from_utf8_lossy(&checked.stderr)
        );
        let events = fs::read_to_string(dir.join("events.jsonl")).unwrap();
        assert!(events.contains("solve_begin"));
        assert!(events.contains("rolling_verified_per_second"));
        assert!(events.contains("observed_solving_degree"));
        assert!(events.contains("\"event\":\"forecast\""));
        assert!(events.contains("predicted_degree_of_regularity"));
        let resumed = call(&["run", "--out", dir.to_str().unwrap(), "--resume"]);
        assert!(
            resumed.status.success(),
            "{}",
            String::from_utf8_lossy(&resumed.stderr)
        );
        assert_eq!(original, fs::read(a.join("base.json")).unwrap());
        assert_eq!(complete, fs::read(a.join("complete.json")).unwrap());
        assert!(dir.join("summary-0002.json").exists());
        assert!(!a.parent().unwrap().join("attempt-0002").exists());
        let changed_environment = Command::new(exe())
            .env("F4_SWEEP_CONFIG_BINDING_TEST", "changed")
            .args(["run", "--out", dir.to_str().unwrap(), "--resume"])
            .output()
            .unwrap();
        assert!(!changed_environment.status.success());
        assert!(String::from_utf8_lossy(&changed_environment.stderr).contains("F4 environment"));
        let mut other = value(&p);
        other["train"] = json!(5);
        fs::write(&p, serde_json::to_vec(&other).unwrap()).unwrap();
        assert!(!call(&[
            "run",
            "--plan",
            p.to_str().unwrap(),
            "--out",
            dir.to_str().unwrap(),
            "--resume"
        ])
        .status
        .success());
        fs::write(a.join("base.json"), b"corrupt inventory").unwrap();
        let bad = call(&["verify", "--run", dir.to_str().unwrap()]);
        assert!(!bad.status.success());
        assert!(String::from_utf8_lossy(&bad.stderr).contains("hash/member mismatch"));
    }
}
#[test]
fn matrix_caps_are_retained_as_exhaustion_not_refutation() {
    let t = Temp::new();
    let p = plan(&t, true);
    let mut v = value(&p);
    v["max_matrix_cells"] = json!(1);
    fs::write(&p, serde_json::to_vec(&v).unwrap()).unwrap();
    let dir = t.0.join("run");
    assert!(launch(&p, &dir).status.success());
    let done = value(&attempt(&dir).join("complete.json"));
    assert_eq!(done["outcome"], "matrix_budget_exhausted", "{done}");
    assert!(call(&["verify", "--run", dir.to_str().unwrap()])
        .status
        .success());
}
#[cfg(target_os = "linux")]
#[test]
fn interrupted_attempt_is_retained_and_resume_obeys_the_hard_deadline() {
    let t = Temp::new();
    let p = plan(&t, true);
    let mut v = value(&p);
    // Keep direct root extraction inside its supported p <= 1_000_000 range.
    // The large trial count keeps the resumed worker active through the cap.
    v["candidates"][0]["curve"] = json!({"kind":"prime_search","bits":19,"seed":19});
    v["train"] = json!(4096);
    v["holdout"] = json!(4096);
    v["candidate_seconds"] = json!(1);
    fs::write(&p, serde_json::to_vec(&v).unwrap()).unwrap();
    let dir = t.0.join("run");
    let mut child = Command::new(exe())
        .args([
            "run",
            "--plan",
            p.to_str().unwrap(),
            "--out",
            dir.to_str().unwrap(),
            "--heartbeat-seconds",
            "0.05",
        ])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .unwrap();
    let began = Instant::now();
    while began.elapsed() < Duration::from_secs(10) {
        if complete_event_recorded(&dir.join("events.jsonl"), "worker_started") {
            break;
        }
        std::thread::sleep(Duration::from_millis(10));
    }
    assert!(complete_event_recorded(
        &dir.join("events.jsonl"),
        "worker_started"
    ));
    child.kill().unwrap();
    child.wait().unwrap();
    let first = attempt(&dir);
    assert!(!first.join("complete.json").exists());
    assert!(first.join("request.json").exists());
    let resumed = call(&[
        "run",
        "--out",
        dir.to_str().unwrap(),
        "--resume",
        "--heartbeat-seconds",
        "0.05",
    ]);
    assert!(
        resumed.status.success(),
        "{}",
        String::from_utf8_lossy(&resumed.stderr)
    );
    let second = first.parent().unwrap().join("attempt-0002");
    let done = value(&second.join("complete.json"));
    assert_eq!(done["outcome"], "timed_out", "{done}");
    assert!(first.join("request.json").exists());
    assert!(fs::read_to_string(dir.join("events.jsonl"))
        .unwrap()
        .contains("heartbeat"));
    assert!(call(&["verify", "--run", dir.to_str().unwrap()])
        .status
        .success());
}
#[test]
fn malformed_plans_and_nonfinite_heartbeat_are_rejected_before_execution() {
    let t = Temp::new();
    let p = plan(&t, false);
    let mut v = value(&p);
    v["candidates"][0]["dimension"] = json!(0);
    fs::write(&p, serde_json::to_vec(&v).unwrap()).unwrap();
    let dir = t.0.join("run");
    assert!(!launch(&p, &dir).status.success());
    assert!(!dir.exists());
    assert!(!call(&[
        "run",
        "--out",
        dir.to_str().unwrap(),
        "--heartbeat-seconds",
        "NaN"
    ])
    .status
    .success());
    assert!(!dir.exists());
}

#[test]
fn grid_options_freeze_all_requested_seed_dimension_combinations() {
    let t = Temp::new();
    let path = t.0.join("grid.json");
    let output = call(&[
        "plan",
        "--out",
        path.to_str().unwrap(),
        "--preset",
        "binary",
        "--seed",
        "42",
        "--seed-count",
        "2",
        "--dimensions",
        "2,4",
    ]);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let p = value(&path);
    let candidates = p["candidates"].as_array().unwrap();
    assert_eq!(candidates.len(), 52);
    for seed in [42, 43] {
        for dimension in [2, 4] {
            assert_eq!(
                candidates
                    .iter()
                    .filter(|c| c["seed"] == seed && c["dimension"] == dimension)
                    .count(),
                13
            );
        }
    }
    assert!(!call(&[
        "plan",
        "--out",
        t.0.join("overflow.json").to_str().unwrap(),
        "--seed-count",
        "0"
    ])
    .status
    .success());
}

#[cfg(target_os = "linux")]
#[test]
fn a_second_supervisor_cannot_write_into_an_active_run() {
    struct Guard(std::process::Child);
    impl Drop for Guard {
        fn drop(&mut self) {
            let _ = self.0.kill();
            let _ = self.0.wait();
        }
    }
    let t = Temp::new();
    let p = plan(&t, true);
    let mut v = value(&p);
    v["candidates"][0]["curve"] = json!({"kind":"prime_search","bits":19,"seed":19});
    v["train"] = json!(4096);
    v["holdout"] = json!(4096);
    v["candidate_seconds"] = json!(30);
    fs::write(&p, serde_json::to_vec(&v).unwrap()).unwrap();
    let dir = t.0.join("active");
    let _first = Guard(
        Command::new(exe())
            .args([
                "run",
                "--plan",
                p.to_str().unwrap(),
                "--out",
                dir.to_str().unwrap(),
            ])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn()
            .unwrap(),
    );
    let began = Instant::now();
    while began.elapsed() < Duration::from_secs(10) {
        if complete_event_recorded(&dir.join("events.jsonl"), "worker_started") {
            break;
        }
        std::thread::sleep(Duration::from_millis(10));
    }
    assert!(complete_event_recorded(
        &dir.join("events.jsonl"),
        "worker_started"
    ));
    let manifest = fs::read(dir.join("manifest.json")).unwrap();
    let second = call(&["run", "--out", dir.to_str().unwrap(), "--resume"]);
    assert!(!second.status.success());
    assert!(String::from_utf8_lossy(&second.stderr).contains("holds the sweep run lock"));
    assert_eq!(manifest, fs::read(dir.join("manifest.json")).unwrap());
}
