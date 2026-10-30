//! Independently replay recorded projective-coordinate witnesses and source receipts.
use serde_json::Value;
use std::{collections::BTreeMap, fs};
fn mul(x: u64, y: u64, p: u64) -> u64 {
    x * y % p
}
fn sum(x: u64, y: u64, p: u64) -> u64 {
    (x + y) % p
}
fn sub(x: u64, y: u64, p: u64) -> u64 {
    (x + p - y) % p
}
fn exp(mut x: u64, mut n: u64, p: u64) -> u64 {
    let mut z = 1;
    while n > 0 {
        if n % 2 == 1 {
            z = mul(z, x, p)
        }
        x = mul(x, x, p);
        n /= 2
    }
    z
}
fn hadamard(x: [u64; 4], p: u64) -> [u64; 4] {
    [
        sum(sum(x[0], x[1], p), sum(x[2], x[3], p), p),
        sub(sum(x[0], x[1], p), sum(x[2], x[3], p), p),
        sum(sub(x[0], x[1], p), sub(x[2], x[3], p), p),
        sub(sub(x[0], x[1], p), sub(x[2], x[3], p), p),
    ]
}
fn vec4(v: &Value) -> Option<[u64; 4]> {
    if v.is_null() {
        None
    } else {
        Some(std::array::from_fn(|i| v[i].as_u64().unwrap()))
    }
}
fn equivalent(x: [u64; 4], y: [u64; 4], p: u64) -> bool {
    (0..4).all(|i| (0..4).all(|j| mul(x[i], y[j], p) == mul(x[j], y[i], p)))
}
fn replay(x: [u64; 4], y: [u64; 4], d: [u64; 4], theta: [u64; 4], p: u64) -> Option<[u64; 4]> {
    let hx = hadamard(x, p);
    let hy = hadamard(y, p);
    let q = std::array::from_fn(|i| mul(mul(hx[i], hy[i], p), exp(theta[i], p - 2, p), p));
    let z = hadamard(q, p);
    let out = std::array::from_fn(|i| {
        (0..4)
            .filter(|&j| j != i)
            .fold(mul(z[i], z[i], p), |v, j| mul(v, d[j], p))
    });
    if out == [0; 4] {
        None
    } else {
        Some(out)
    }
}
fn check_case(case: &Value, field: &str, theta: [u64; 4], p: u64) -> &'static str {
    let w = &case["coordinate_witness"];
    let x = vec4(&w["k_left"]);
    let y = vec4(&w["k_right"]);
    let d = vec4(&w["k_difference"]);
    let expected = vec4(&w["k_expected"]);
    let observed = match (x, y, d) {
        (Some(x), Some(y), Some(d)) => replay(x, y, d, theta, p),
        _ => None,
    };
    assert_eq!(observed, vec4(&w["k_observed"]));
    assert_eq!(
        w["jacobian_expected"]["degree"].as_u64().unwrap(),
        case["sum_degree"].as_u64().unwrap()
    );
    let status = if x.is_none() || y.is_none() || d.is_none() || expected.is_none() {
        "projection_inapplicable"
    } else if observed.is_none() {
        "zero_output"
    } else if equivalent(observed.unwrap(), expected.unwrap(), p) {
        "pass"
    } else {
        "fail"
    };
    assert_eq!(case[field].as_str().unwrap(), status);
    status
}
fn main() {
    let theta_raw =
        fs::read_to_string("research/kummer_theta_20261022/results/theta-models.jsonl").unwrap();
    let mut theta = BTreeMap::new();
    for line in theta_raw.lines() {
        let v: Value = serde_json::from_str(line).unwrap();
        let p = v["p"].as_u64().unwrap();
        let t = &v["selected"]["constants"];
        theta.insert(
            p,
            ["A", "B", "C", "D"].map(|k| {
                let z = t[k].as_u64().unwrap();
                assert_ne!(z, 0);
                z
            }),
        );
    }
    let models =
        fs::read_to_string("research/kummer_coordinates_20261022/results/generic-models.jsonl")
            .unwrap();
    let mut model_pass = 0;
    let mut model_zero = 0;
    let mut model_fail = 0;
    let mut map_fail = 0;
    let mut model_cases = 0;
    let mut generic_doubles = 0;
    let mut generic_additions = 0;
    let mut actions = 0;
    let mut map_checks = 0;
    for line in models.lines() {
        let v: Value = serde_json::from_str(line).unwrap();
        let p = v["p"].as_u64().unwrap();
        let th = theta[&p];
        map_fail += v["map_fail"].as_u64().unwrap();
        map_checks += v["map_checks"].as_u64().unwrap();
        generic_doubles += v["xDBL_pass"].as_u64().unwrap();
        generic_additions += v["xADD_pass"].as_u64().unwrap();
        assert_eq!(v["xDBL_fail"], 0);
        assert_eq!(v["xADD_fail"], 0);
        assert_eq!(v["xDBL_inapplicable"], 0);
        assert_eq!(v["xADD_inapplicable"], 0);
        actions += v["torsion_actions"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|a| !a["action"].is_null())
            .count();
        for c in v["exceptional_cases"].as_array().unwrap() {
            model_cases += 1;
            match check_case(c, "status", th, p) {
                "pass" => model_pass += 1,
                "zero_output" => model_zero += 1,
                _ => model_fail += 1,
            }
        }
    }
    assert_eq!(theta.len(), 21);
    assert_eq!(models.lines().count(), 21);
    assert_eq!(
        (generic_doubles, generic_additions, actions, map_checks),
        (84, 126, 105, 336)
    );
    assert_eq!(
        (model_cases, model_pass, model_zero, model_fail, map_fail),
        (1176, 1008, 168, 0, 0)
    );
    let mut targets = 0;
    let mut cases = 0;
    let mut pass = 0;
    let mut zero = 0;
    let mut fail = 0;
    let mut source_fail = 0;
    let mut signed_fail = 0;
    for bits in [16, 20, 24, 28, 32, 36, 40] {
        let path = format!("research/kummer_coordinates_20261022/results/target-gate-{bits}.jsonl");
        let prior = format!("research/kummer_walk_20261022/results/target-gate-{bits}.jsonl");
        let rows = fs::read_to_string(path).unwrap();
        let controls = fs::read_to_string(prior).unwrap();
        assert_eq!(rows.lines().count(), 180);
        assert_eq!(controls.lines().count(), 180);
        for (raw, ctl) in rows.lines().zip(controls.lines()) {
            let v: Value = serde_json::from_str(raw).unwrap();
            let c: Value = serde_json::from_str(ctl).unwrap();
            let p = v["p"].as_u64().unwrap();
            let th = theta[&p];
            assert_eq!(v["workload_id"], c["workload_id"]);
            assert_eq!(v["workload_sha256"], c["workload_sha256"]);
            assert_eq!(v["algorithm_seed"], c["algorithm_seed"]);
            assert_eq!(v["planted"], c["planted"]);
            assert_eq!(v["source_scalar_verified"], true);
            assert_eq!(c["all_correct"], true);
            if v["all_source_checks"] != true {
                source_fail += 1
            }
            let cs = v["cases"].as_array().unwrap();
            let prior_cases = c["cases"].as_array().unwrap();
            assert_eq!(cs.len(), 7);
            for (z, pc) in cs.iter().zip(prior_cases) {
                assert_eq!(z["a"], pc["a"]);
                assert_eq!(z["b"], pc["b"]);
                assert_eq!(z["case_index"], pc["case_index"]);
                assert_eq!(z["source_label_correct"], true);
                assert_eq!(z["affine_map_correct"], true);
                if z["signed_lift_count"] != 1 {
                    signed_fail += 1
                }
                cases += 1;
                match check_case(z, "coordinate_status", th, p) {
                    "pass" => pass += 1,
                    "zero_output" => zero += 1,
                    _ => fail += 1,
                }
            }
            targets += 1;
        }
    }
    assert_eq!(
        (targets, cases, pass, zero, fail, source_fail, signed_fail),
        (1260, 8820, 8813, 7, 0, 0, 0)
    );
    let exceptions = fs::read_to_string(
        "research/kummer_coordinates_20261022/results/coordinate-exceptions.jsonl",
    )
    .unwrap();
    let mut model_exception_rows = 0;
    let mut target_exception_rows = 0;
    for line in exceptions.lines() {
        let v: Value = serde_json::from_str(line).unwrap();
        assert_eq!(v["status"], "zero_output");
        if v["source"] == "model" {
            model_exception_rows += 1
        } else if v["source"] == "target" {
            target_exception_rows += 1
        } else {
            panic!("unexpected exception source")
        }
    }
    assert_eq!((model_exception_rows, target_exception_rows), (168, 7));
    println!("validated_models=21 model_exception_pass={model_pass} model_exception_zero={model_zero} validated_targets={targets} coordinate_pass={pass} coordinate_zero={zero} coordinate_fail={fail} source_fail={source_fail} signed_oracle_fail={signed_fail}");
}
