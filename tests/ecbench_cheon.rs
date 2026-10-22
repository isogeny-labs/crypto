//! Cheon's DLP with auxiliary inputs as an `ecbench` method: the
//! `cheon.*` methods recover the planted logarithm from the target and a
//! static-DH oracle, count the oracle's queries as a resource of their own,
//! replay exactly through the harness, and fit a bound in an oracle domain.

use std::path::{Path, PathBuf};
use std::process::Command;

use crypto_lib::cryptanalysis::cheon_counted::{find_shaped_curve, next_prime, Shaped, Side};
use crypto_lib::cryptanalysis::ecbench::methods::{
    needs_static_dh_oracle, resolve, solve, solve_with_oracle, MethodSpec,
};
use crypto_lib::cryptanalysis::ecbench::oracle::PlantedStaticDh;
use crypto_lib::cryptanalysis::ecbench::workload::{CurveSpec, Instance, TargetKind, Workload};
use serde_json::Value;

fn shaped(side: Side, ratio: f64) -> (CurveSpec, Shaped) {
    let p = next_prime(3 << 17);
    let s = find_shaped_curve(p, 77, side, ratio, 0.06);
    (
        CurveSpec::PrimeExplicit {
            p: s.p,
            a: s.a,
            b: s.b,
            group_order: s.n,
            r: s.n,
            gx: s.gx,
            gy: s.gy,
        },
        s,
    )
}

fn method(id: &str, d: &str) -> MethodSpec {
    MethodSpec {
        id: id.into(),
        params: [("d".to_string(), d.to_string())].into(),
    }
}

#[test]
fn cheon_methods_recover_the_planted_log_and_count_the_oracle() {
    for (side, ratio, id, shape) in [
        (Side::Minus, 0.2, "cheon.pminus1_comb", "shape:0.2:0.06"),
        (Side::Minus, 0.2, "cheon.pminus1_naive", "shape:0.2:0.06"),
        (Side::Plus, 0.12, "cheon.pplus1_comb", "shape:0.12:0.06"),
    ] {
        let (spec, s) = shaped(side, ratio);
        let m = resolve(&method(id, shape)).unwrap();
        assert!(needs_static_dh_oracle(id));
        for index in 0..3 {
            let (w, inst) = Workload::build(&spec, 20261022, index, TargetKind::Planted).unwrap();
            let Instance::Prime(pi) = &inst else {
                panic!("prime instance")
            };
            let alpha = w.planted.unwrap();
            // The method is refused without the oracle: it cannot see alpha.
            let err = solve(&m, &inst, &w.curve, &w.target, 1).unwrap_err();
            assert!(err.contains("static-DH oracle"), "{err}");

            let oracle = PlantedStaticDh::new(&pi.curve, alpha as u64);
            let rep = solve_with_oracle(&m, &inst, &w.curve, &w.target, 1, Some(&oracle)).unwrap();
            assert_eq!(rep.recovered, Some(alpha), "{id} target {index}");
            let d = rep.counters["d"];
            assert_eq!(d, s.d, "the shape rule picks the divisor the search froze");
            let want_queries = match side {
                Side::Minus => d - 1,
                Side::Plus => 2 * d - 1,
            };
            assert_eq!(rep.counters["oracle_queries"], want_queries);
            assert_eq!(oracle.queries(), want_queries);
            assert!(rep.counters["oracle_server_group_ops"] > 0);
            // The unit counts the attacker's group operations only.
            let (a, b): (u64, u64) = rep
                .phases
                .iter()
                .fold((0, 0), |t, p| (t.0 + p.adds, t.1 + p.doubles));
            assert_eq!(rep.total_gae, (a + b) as f64);
            assert!(rep
                .phases
                .iter()
                .all(|p| p.gae == (p.adds + p.doubles) as f64));
            // Unpriced work is listed, queries are not folded into it.
            assert!(rep.unpriced.iter().any(|u| u == "table_insert_uncharged"));
            assert!(!rep.unpriced.iter().any(|u| u.starts_with("oracle")));
            assert!(rep.field_ops.is_some());
            assert!(rep.deterministic);
            // Deterministic: the same inputs give the same counts.
            let oracle2 = PlantedStaticDh::new(&pi.curve, alpha as u64);
            let again =
                solve_with_oracle(&m, &inst, &w.curve, &w.target, 99, Some(&oracle2)).unwrap();
            assert_eq!(again.total_gae, rep.total_gae);
            assert_eq!(again.counters, rep.counters);
        }
    }
}

/// Negative control: the auxiliary input is used.  An oracle holding another
/// scalar leaks the powers of another scalar, and the planted logarithm is
/// not returned.
#[test]
fn a_wrong_oracle_does_not_return_the_planted_log() {
    for (side, ratio, id, shape) in [
        (Side::Minus, 0.2, "cheon.pminus1_comb", "shape:0.2:0.06"),
        (Side::Plus, 0.12, "cheon.pplus1_comb", "shape:0.12:0.06"),
    ] {
        let (spec, _) = shaped(side, ratio);
        let m = resolve(&method(id, shape)).unwrap();
        let (w, inst) = Workload::build(&spec, 5, 0, TargetKind::Planted).unwrap();
        let Instance::Prime(pi) = &inst else { panic!() };
        let alpha = w.planted.unwrap() as u64;
        let wrong = PlantedStaticDh::new(&pi.curve, alpha % (pi.r - 3) + 1);
        let rep = solve_with_oracle(&m, &inst, &w.curve, &w.target, 1, Some(&wrong)).unwrap();
        assert_ne!(rep.recovered, w.planted, "{id}");
    }
}

#[test]
fn an_integer_d_and_a_public_target_are_handled() {
    let (spec, s) = shaped(Side::Minus, 0.2);
    // An explicit d that is not a divisor of n - 1 is reported, not run.
    let m = resolve(&method("cheon.pminus1_comb", &(s.d + 1).to_string())).unwrap();
    let (w, inst) = Workload::build(&spec, 5, 0, TargetKind::Planted).unwrap();
    let Instance::Prime(pi) = &inst else { panic!() };
    let o = PlantedStaticDh::new(&pi.curve, w.planted.unwrap() as u64);
    let rep = solve_with_oracle(&m, &inst, &w.curve, &w.target, 1, Some(&o)).unwrap();
    assert!(rep.exhausted && rep.recovered.is_none());
    // The registered method has no default for d.
    assert!(resolve(&MethodSpec {
        id: "cheon.pminus1_comb".into(),
        params: Default::default()
    })
    .is_err());
}

fn exe() -> &'static str {
    env!("CARGO_BIN_EXE_ecbench")
}

fn scratch(name: &str) -> PathBuf {
    let d = std::env::temp_dir().join(format!("ecbench-cheon-{}-{name}", std::process::id()));
    let _ = std::fs::remove_dir_all(&d);
    std::fs::create_dir_all(&d).unwrap();
    d
}

fn ecbench(args: &[&str]) -> (bool, String, String) {
    let out = Command::new(exe()).args(args).output().expect("ecbench");
    (
        out.status.success(),
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}

fn read_json(p: &Path) -> Value {
    serde_json::from_str(&std::fs::read_to_string(p).unwrap()).unwrap()
}

/// Through the harness: a session replays exactly, a bound for an arm that
/// queries an oracle needs the oracle named, lands in a domain of its own,
/// and the frontier keeps the querying method and a reference that does not
/// query side by side.
#[test]
fn a_session_replays_and_the_bound_sits_in_an_oracle_domain() {
    let (cs, s) = shaped(Side::Minus, 0.2);
    let CurveSpec::PrimeExplicit {
        p,
        a,
        b,
        group_order,
        r,
        gx,
        gy,
    } = cs
    else {
        unreachable!()
    };
    let dir = scratch("session");
    let spec = serde_json::json!({
        "schema": "ecbench.spec/v1",
        "label": "cheon integration test",
        "workloads": {
            "curves": [{"kind": "prime_explicit", "p": p, "a": a, "b": b,
                        "group_order": group_order, "r": r, "gx": gx, "gy": gy}],
            "targets_per_curve": 3,
            "target_seed": 20261022
        },
        "arms": [
            {"name": "rho-neg", "role": "reference", "method": {"id": "rho.negation"}},
            {"name": "cheon-comb", "role": "candidate",
             "method": {"id": "cheon.pminus1_comb", "params": {"d": "shape:0.2:0.06"}}}
        ],
        "measurement": {"rounds": 1, "warmup": 0, "seed": 1, "isolation_required": "L0",
                        "timeout_seconds": 120}
    });
    let spec_path = dir.join("spec.json");
    std::fs::write(&spec_path, spec.to_string()).unwrap();
    let out = dir.join("s");
    let (ok, _, err) = ecbench(&[
        "run",
        "--spec",
        spec_path.to_str().unwrap(),
        "--out",
        out.to_str().unwrap(),
        "--cpus",
        "none",
        "--lock",
        dir.join("bench.lock").to_str().unwrap(),
        "--quiet",
    ]);
    assert!(ok, "run failed: {err}");
    let session = read_json(&out.join("session.json"));
    assert_eq!(session["status"], "complete");
    assert_eq!(session["status_counts"]["verified"], 6);

    let (ok, stdout, err) = ecbench(&[
        "verify",
        "--dir",
        out.to_str().unwrap(),
        "--replay-all",
        "--exit-code",
    ]);
    assert!(ok, "verify failed: {err}");
    let audit: Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(audit["ok"], true);
    let audit_path = dir.join("audit.json");
    std::fs::write(&audit_path, &stdout).unwrap();

    // The queries are in the record, apart from the unit.
    let records = std::fs::read_to_string(out.join("records.jsonl")).unwrap();
    let cheon: Vec<Value> = records
        .lines()
        .map(|l| serde_json::from_str::<Value>(l).unwrap())
        .filter(|v| v["arm"] == "cheon-comb")
        .collect();
    assert_eq!(cheon.len(), 3);
    for rec in &cheon {
        assert_eq!(rec["counters"]["oracle_queries"], s.d - 1);
        assert_eq!(rec["detail"]["oracle"]["model"], "static_dh");
    }

    // A bound for the querying arm must name the oracle.
    let fit = |arm: &str, oracle: Option<&str>, to: &str| {
        let mut args = vec![
            "bound",
            "fit",
            "--dir",
            out.to_str().unwrap(),
            "--arm",
            arm,
            "--audit",
            audit_path.to_str().unwrap(),
            "--root",
            dir.to_str().unwrap(),
            "--out",
            to,
        ];
        if let Some(o) = oracle {
            args.extend(["--oracle", o]);
        }
        ecbench(&args)
    };
    let c_path = dir.join("cheon.json");
    let r_path = dir.join("rho.json");
    let (ok, _, err) = fit("cheon-comb", None, c_path.to_str().unwrap());
    assert!(!ok && err.contains("--oracle"), "{err}");
    let label = "static_dh:pminus1:test";
    let (ok, _, err) = fit("cheon-comb", Some(label), c_path.to_str().unwrap());
    assert!(ok, "{err}");
    let (ok, _, err) = fit("rho-neg", Some(label), r_path.to_str().unwrap());
    assert!(ok, "{err}");
    let cb = read_json(&c_path);
    let rb = read_json(&r_path);
    assert_eq!(cb["domain"]["envelope"]["oracle"], label);
    assert_eq!(cb["domain_id"], rb["domain_id"]);
    assert!(
        cb["dimensions"]["oracle_queries"]["value"]
            .as_f64()
            .unwrap()
            > 0.0
    );
    assert_eq!(
        rb["dimensions"]["oracle_queries"]["value"]
            .as_f64()
            .unwrap(),
        0.0
    );
    assert!(cb["oracle"].is_null() || cb["oracle"]["model"] == "static_dh");
    // A plain-domain bound of the same rho arm is a different domain.
    let p_path = dir.join("rho-plain.json");
    let (ok, _, err) = fit("rho-neg", None, p_path.to_str().unwrap());
    assert!(ok, "{err}");
    assert_ne!(read_json(&p_path)["domain_id"], rb["domain_id"]);

    // Bounds re-derive.
    let (ok, _, err) = ecbench(&[
        "bound",
        "check",
        "--record",
        c_path.to_str().unwrap(),
        r_path.to_str().unwrap(),
        "--root",
        dir.to_str().unwrap(),
    ]);
    assert!(ok, "{err}");

    // Frontier: the rho reference queries nothing, so Cheon's lower ops do
    // not displace it.
    let recs = dir.join("recs");
    std::fs::create_dir_all(&recs).unwrap();
    std::fs::copy(&c_path, recs.join("cheon.json")).unwrap();
    std::fs::copy(&r_path, recs.join("rho.json")).unwrap();
    let fj = dir.join("frontier.json");
    let (ok, _, err) = ecbench(&[
        "frontier",
        "build",
        "--bounds",
        recs.to_str().unwrap(),
        "--out",
        fj.to_str().unwrap(),
    ]);
    assert!(ok, "{err}");
    let f = read_json(&fj);
    let d = &f["domains"][0];
    assert!(d["axes"]
        .as_array()
        .unwrap()
        .iter()
        .any(|a| a == "oracle_queries"));
    for e in d["entries"].as_array().unwrap() {
        // Cheon trades oracle queries for ops; neither dominates here
        // unless the intervals say so, and the reference is never displaced
        // by a method that was handed the answers.
        if e["method"] == "rho.negation" {
            assert_eq!(e["is_frontier"], true, "{e}");
        }
    }
}
