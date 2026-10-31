//! Public-interface correctness controls, not performance measurements.
//! The planted scalar constructs Q outside every method invocation.
use crypto_lib::cryptanalysis::{
    ecbench::{
        aps, audit,
        methods::{self, MethodSpec, ResolvedMethod, SolveReport},
        workload::{CurveFacts, CurveSpec, Instance},
    },
    ic_boundary::{CountedGroup, GroupOps},
};
use std::collections::BTreeMap;

fn method(encoding: &str, changes: &[(&str, &str)]) -> ResolvedMethod {
    let mut params: BTreeMap<String, String> = [
        ("encoding", encoding),
        ("roots", "exact"),
        ("stop", "first_informative"),
        ("labels", "guaranteed"),
        ("subset_size", "2"),
        ("attempts", "32"),
        ("degree", "24"),
        ("budget_ms", "10000"),
        ("sampling_draws", "1000"),
        ("orbit", "none"),
        ("beta", "0"),
        ("lambda", "0"),
        ("prune", "false"),
    ]
    .map(|(k, v)| (k.into(), v.into()))
    .into();
    params.extend(changes.iter().map(|(k, v)| ((*k).into(), (*v).into())));
    let resolved = methods::resolve(&MethodSpec {
        id: aps::ID.into(),
        params,
    })
    .unwrap();
    assert_eq!(
        audit::method_determinism(&resolved),
        audit::Determinism::Never
    );
    resolved
}

fn fixture(orbit: bool) -> (Instance, CurveFacts, [String; 2]) {
    let (p, a, b, n, gx, gy, d) = if orbit {
        (103, 0, 5, 97, 2, 42, 17)
    } else {
        (101, 59, 6, 107, 78, 4, 37)
    };
    let spec = CurveSpec::PrimeExplicit {
        p,
        a,
        b,
        group_order: n,
        r: n,
        gx,
        gy,
    };
    let instance = spec.build().unwrap();
    let facts = instance.facts(&spec);
    let Instance::Prime(prime) = &instance else {
        unreachable!()
    };
    let q = prime
        .curve
        .mul(&mut GroupOps::default(), prime.generator_point(), d);
    (
        instance,
        facts,
        [format!("{:x}", q.x), format!("{:x}", q.y)],
    )
}

fn verified(report: &SolveReport, expected: u128) {
    assert_eq!(report.recovered, Some(expected), "{}", report.detail);
    assert!(!report.exhausted);
    assert_eq!(report.detail["outcome"], "verified");
    assert_eq!(report.detail["native_work_accounting_complete"], false);
    assert_eq!(report.detail["costs_attested"], false);
    assert!(!report.deterministic);
    assert!(!report.unpriced.is_empty());
    assert_eq!(
        report
            .phases
            .iter()
            .map(|p| p.wall_ns.unwrap())
            .sum::<u64>(),
        report.solve_wall_ns
    );
    assert_eq!(
        report.phases.iter().map(|p| p.gae).sum::<f64>(),
        report.total_gae
    );
    let online = report.online.as_ref().unwrap();
    assert_eq!(online.phases_ns.values().sum::<u64>(), online.wall_ns);
    assert_eq!(online.phases_ns.len(), 5);
    let attempts = report.detail["attempts"].as_array().unwrap();
    let final_attempt = attempts.last().unwrap();
    assert_eq!(final_attempt["independent_witness_verified"], true);
    assert_eq!(final_attempt["deadline_exhausted"], false);
    assert_eq!(final_attempt["outcome"], "verified");
}

#[test]
fn every_supported_encoding_recovers_through_the_public_dispatcher() {
    let (instance, facts, target) = fixture(false);
    for encoding in [
        "partitioned3",
        "unordered3",
        "line3",
        "expanded4",
        "split4",
        "block_symmetric4",
    ] {
        let m = method(encoding, &[]);
        methods::applicable(&m, &instance, &facts).unwrap();
        let report = methods::solve(&m, &instance, &facts, &target, 20261026).unwrap();
        verified(&report, 37);
    }
}

#[test]
fn certified_orbit_controls_share_the_base_and_recover_the_same_logarithm() {
    let (instance, facts, target) = fixture(true);
    let config = [
        ("orbit", "cubic"),
        ("beta", "56"),
        ("lambda", "35"),
        ("labels", "random"),
        ("subset_size", "1"),
        ("stop", "exhaustive"),
    ];
    let plain = method("line3", &config);
    let mut spec = MethodSpec {
        id: aps::ID.into(),
        params: plain.params.clone(),
    };
    spec.params.insert("prune".into(), "true".into());
    let pruned = methods::resolve(&spec).unwrap();
    let a = methods::solve(&plain, &instance, &facts, &target, 20261026).unwrap();
    let b = methods::solve(&pruned, &instance, &facts, &target, 20261026).unwrap();
    verified(&a, 17);
    verified(&b, 17);
    assert_eq!(
        a.detail["attempts"][0]["base"],
        b.detail["attempts"][0]["base"]
    );
    assert_eq!(a.automorphisms_used, 2);
    assert_eq!(b.automorphisms_used, 2);
}

#[test]
fn invalid_orbit_certificates_and_unsupported_encodings_are_planning_refusals() {
    let (instance, facts, _) = fixture(true);
    let wrong = method(
        "line3",
        &[
            ("orbit", "cubic"),
            ("beta", "56"),
            ("lambda", "34"),
            ("prune", "true"),
            ("subset_size", "1"),
        ],
    );
    assert!(methods::applicable(&wrong, &instance, &facts).is_err());
    let unsupported = method("hybrid3", &[]);
    assert!(methods::applicable(&unsupported, &instance, &facts).is_err());
}

#[test]
fn native_scratch_session_audits_aps_without_claiming_count_replay() {
    use serde_json::{json, Value};
    use std::process::Command;
    use std::time::{SystemTime, UNIX_EPOCH};

    let dir = std::env::temp_dir().join(format!(
        "aps-ecbench-audit-{}-{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    std::fs::create_dir(&dir).unwrap();
    let mut spec: Value = serde_json::from_str(include_str!(
        "../research/aps_measurement_20261029/spec-controls.json"
    ))
    .unwrap();
    let arms: Vec<_> = spec["arms"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|a| matches!(a["name"].as_str(), Some("rho-neg" | "line")))
        .cloned()
        .collect();
    assert_eq!(arms.len(), 2);
    spec["arms"] = json!(arms);
    spec["workloads"]["targets_per_curve"] = json!(1);
    spec["workloads"]["target_kind"] = json!("public");
    spec["measurement"]["rounds"] = json!(1);
    spec["measurement"]["warmup"] = json!(0);
    spec["measurement"]["isolation_required"] = json!("L0");
    for arm in spec["arms"].as_array_mut().unwrap() {
        if arm["name"] == "line" {
            arm["method"]["params"]["attempts"] = json!("32");
        }
    }
    let spec_path = dir.join("spec.json");
    let session = dir.join("session");
    std::fs::write(&spec_path, serde_json::to_vec_pretty(&spec).unwrap()).unwrap();
    let exe = env!("CARGO_BIN_EXE_ecbench");
    let run = Command::new(exe)
        .args(["run", "--spec"])
        .arg(&spec_path)
        .arg("--out")
        .arg(&session)
        .args(["--cpus", "none", "--lock"])
        .arg(dir.join("bench.lock"))
        .arg("--quiet")
        .output()
        .unwrap();
    assert!(
        run.status.success(),
        "scratch run failed; retained at {}: {}",
        dir.display(),
        String::from_utf8_lossy(&run.stderr)
    );
    let text = std::fs::read_to_string(session.join("records.jsonl")).unwrap();
    let records: Vec<Value> = text
        .lines()
        .map(|s| serde_json::from_str(s).unwrap())
        .collect();
    assert_eq!(records.len(), 2);
    for record in &records {
        assert_eq!(
            record["outcome"]["status"],
            "verified",
            "scratch record failed; retained at {}",
            dir.display()
        );
        assert!(record["workload"]["planted"].is_null());
    }
    let aps_record = records
        .iter()
        .find(|r| r["method"]["id"] == aps::ID)
        .unwrap();
    assert_eq!(aps_record["cost"]["deterministic"], false);
    assert_eq!(aps_record["cost"]["lower_bound"], true);
    assert_eq!(
        aps_record["detail"]["native_work_accounting_complete"],
        false
    );
    let audit = Command::new(exe)
        .args(["verify", "--dir"])
        .arg(&session)
        .args(["--replay-all", "--exit-code"])
        .output()
        .unwrap();
    assert!(
        audit.status.success(),
        "audit failed; retained at {}: {} {}",
        dir.display(),
        String::from_utf8_lossy(&audit.stdout),
        String::from_utf8_lossy(&audit.stderr)
    );
    let receipt: Value = serde_json::from_slice(&audit.stdout).unwrap();
    assert_eq!(receipt["ok"], true);
    let replays = receipt["replays"].as_array().unwrap();
    assert_eq!(replays.len(), 1); // The deterministic rho control alone replays.
    assert!(replays
        .iter()
        .all(|r| r["record_id"] != aps_record["record_id"]));
    std::fs::remove_dir_all(&dir).unwrap();
}
