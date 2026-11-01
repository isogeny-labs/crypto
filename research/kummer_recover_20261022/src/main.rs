//! Two-torsion-shift repair of published fast-Kummer differential addition.
use serde_json::{json, Value};
use std::{collections::BTreeMap, fs};
fn mul(x: u64, y: u64, p: u64) -> u64 {
    x * y % p
}
fn add(x: u64, y: u64, p: u64) -> u64 {
    (x + y) % p
}
fn sub(x: u64, y: u64, p: u64) -> u64 {
    (x + p - y) % p
}
fn pow(mut x: u64, mut n: u64, p: u64) -> u64 {
    let mut z = 1;
    while n > 0 {
        if n & 1 == 1 {
            z = mul(z, x, p)
        }
        x = mul(x, x, p);
        n >>= 1
    }
    z
}
fn inv(x: u64, p: u64) -> u64 {
    assert_ne!(x, 0);
    pow(x, p - 2, p)
}
fn h(x: [u64; 4], p: u64) -> [u64; 4] {
    let [a, b, c, d] = x;
    [
        add(add(a, b, p), add(c, d, p), p),
        sub(add(a, b, p), add(c, d, p), p),
        add(sub(a, b, p), sub(c, d, p), p),
        sub(sub(a, b, p), sub(c, d, p), p),
    ]
}
fn same(x: [u64; 4], y: [u64; 4], p: u64) -> bool {
    (0..4).all(|i| (0..4).all(|j| mul(x[i], y[j], p) == mul(x[j], y[i], p)))
}
fn vec4(v: &Value) -> [u64; 4] {
    std::array::from_fn(|i| v[i].as_u64().unwrap())
}
fn matrix(v: &Value) -> [[u64; 4]; 4] {
    std::array::from_fn(|i| vec4(&v[i]))
}
fn linear(x: [u64; 4], m: [[u64; 4]; 4], p: u64) -> [u64; 4] {
    std::array::from_fn(|i| (0..4).fold(0, |z, j| add(z, mul(m[i][j], x[j], p), p)))
}
fn xadd(x: [u64; 4], y: [u64; 4], diff: [u64; 4], theta: [u64; 4], p: u64) -> Option<[u64; 4]> {
    let hx = h(x, p);
    let hy = h(y, p);
    let z = h(
        std::array::from_fn(|i| mul(mul(hx[i], hy[i], p), inv(theta[i], p), p)),
        p,
    );
    let out = std::array::from_fn(|i| {
        (0..4)
            .filter(|&j| j != i)
            .fold(mul(z[i], z[i], p), |v, j| mul(v, diff[j], p))
    });
    if out == [0; 4] {
        None
    } else {
        Some(out)
    }
}
fn main() {
    let models =
        fs::read_to_string("research/kummer_coordinates_20261022/results/generic-models.jsonl")
            .unwrap();
    let constants =
        fs::read_to_string("research/kummer_theta_20261022/results/theta-models.jsonl").unwrap();
    let mut maps = BTreeMap::new();
    let mut theta = BTreeMap::new();
    for line in models.lines() {
        let v: Value = serde_json::from_str(line).unwrap();
        let p = v["p"].as_u64().unwrap();
        let branches: Vec<_> = v["torsion_actions"]
            .as_array()
            .unwrap()
            .iter()
            .map(|a| (a["root"].as_u64().unwrap(), matrix(&a["action"]["matrix"])))
            .collect();
        assert_eq!(branches.len(), 5);
        maps.insert(p, branches);
    }
    for line in constants.lines() {
        let v: Value = serde_json::from_str(line).unwrap();
        let p = v["p"].as_u64().unwrap();
        let c = &v["selected"]["constants"];
        theta.insert(p, ["A", "B", "C", "D"].map(|k| c[k].as_u64().unwrap()));
    }
    let failures = fs::read_to_string(
        "research/kummer_coordinates_20261022/results/coordinate-exceptions.jsonl",
    )
    .unwrap();
    let mut out = String::new();
    let mut solved = 0;
    let mut failed = 0;
    for line in failures.lines() {
        let v: Value = serde_json::from_str(line).unwrap();
        let p = v["p"].as_u64().unwrap();
        let w = &v["case"]["coordinate_witness"];
        let (x, y, d, expected) = (
            vec4(&w["k_left"]),
            vec4(&w["k_right"]),
            vec4(&w["k_difference"]),
            vec4(&w["k_expected"]),
        );
        assert!(xadd(x, y, d, theta[&p], p).is_none());
        let mut attempts = Vec::new();
        let mut selected = None;
        for &(root, m) in &maps[&p] {
            let tx = linear(x, m, p);
            let td = linear(d, m, p);
            let shifted = xadd(tx, y, td, theta[&p], p);
            let recovered = shifted.map(|z| linear(z, m, p));
            let matches = recovered.is_some_and(|z| same(z, expected, p));
            attempts.push(json!({"root":root,"k_left_shifted":tx,"k_difference_shifted":td,"shifted_output":shifted,"recovered_output":recovered,"matches_expected":matches}));
            if selected.is_none() && matches {
                selected = Some(root)
            }
        }
        if selected.is_some() {
            solved += 1
        } else {
            failed += 1
        }
        out.push_str(&format!("{}\n",json!({"schema":"kummer-zero-shift/v1","source":v["source"],"bits":v["bits"],"p":p,"target_index":v["target_index"],"case_index":v["case_index"],"original_status":"zero_output","selected_root":selected,"repair_verified":selected.is_some(),"expected_kummer":expected,"expected_jacobian":w["jacobian_expected"],"attempts":attempts})));
    }
    assert_eq!(solved + failed, 175);
    fs::create_dir_all("research/kummer_recover_20261022/results").unwrap();
    fs::write(
        "research/kummer_recover_20261022/results/zero-shift.jsonl",
        out,
    )
    .unwrap();
    println!("zero_shift_cases=175 solved={solved} failed={failed}");
}
