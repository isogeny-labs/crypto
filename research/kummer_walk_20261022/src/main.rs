//! Reference quotient/differential/sign-recovery invariant gate.
//! Uses Jacobian addition as an oracle; this is not a fast Kummer law.
use five_leads_study::three_leads::{Divisor, Point, Scholten};
use serde_json::{json, Value};
use std::collections::BTreeSet;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::process::Command;

fn k(s: &Scholten, d: Divisor) -> Divisor {
    d.min(s.j.neg(d))
}

fn lifts(s: &Scholten, c: Divisor) -> BTreeSet<Divisor> {
    [c, s.j.neg(c)].into_iter().collect()
}

fn differential_reference(
    s: &Scholten,
    ka: Divisor,
    kb: Divisor,
    k_difference: Divisor,
) -> (BTreeSet<Divisor>, usize) {
    let mut outputs = BTreeSet::new();
    let mut compatible = 0;
    for a in lifts(s, ka) {
        for b in lifts(s, kb) {
            if k(s, s.j.add(a, s.j.neg(b))) == k_difference {
                compatible += 1;
                outputs.insert(k(s, s.j.add(a, b)));
            }
        }
    }
    (outputs, compatible)
}

fn recover_signed(
    s: &Scholten,
    sum_class: Divisor,
    next_class: Divisor,
    anchor: Divisor,
) -> BTreeSet<Divisor> {
    lifts(s, sum_class)
        .into_iter()
        .filter(|&candidate| k(s, s.j.add(candidate, anchor)) == next_class)
        .collect()
}

fn case(s: &Scholten, a: Divisor, b: Divisor, anchor: Divisor) -> Value {
    let expected = s.j.add(a, b);
    let difference = k(s, s.j.add(a, s.j.neg(b)));
    let (outputs, compatible_lifts) = differential_reference(s, k(s, a), k(s, b), difference);
    let next_class = k(s, s.j.add(expected, anchor));
    let recovered = recover_signed(s, k(s, expected), next_class, anchor);
    json!({
        "compatible_lifts": compatible_lifts,
        "differential_output_count": outputs.len(),
        "differential_correct": outputs.len() == 1 && outputs.contains(&k(s, expected)),
        "signed_lift_count": recovered.len(),
        "signed_recovery_correct": recovered.len() == 1 && recovered.contains(&expected),
        "a_degree": a.degree,
        "b_degree": b.degree,
        "sum_degree": expected.degree
    })
}

fn source_point(v: &Value) -> Point {
    Point::affine(v[0].as_u64().unwrap(), v[1].as_u64().unwrap())
}

fn target_point(v: &Value) -> Point {
    let coordinate =
        |i| u64::from_str_radix(v[i].as_str().unwrap().trim_start_matches("0x"), 16).unwrap();
    Point::affine(coordinate(0), coordinate(1))
}

fn archive_rows(bits: u64) -> Vec<Value> {
    let archive = "research/five_leads_e2e_20261019/sessions_projective_chart_v2.tar.zst";
    let member = format!("sessions_projective_chart_v2/order-genus-{bits}/records.jsonl");
    let out = Command::new("tar")
        .args(["--zstd", "-xOf", archive, &member])
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "archive read failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8(out.stdout)
        .unwrap()
        .lines()
        .map(|line| serde_json::from_str::<Value>(line).unwrap())
        .filter(|row| row["arm"] == "rho_neg")
        .collect()
}

fn roots(p: u64) -> Vec<u64> {
    let path = "research/four_followups_20261021/results/kummer-model-gate.jsonl";
    let data = std::fs::read_to_string(path).unwrap();
    let row: Value = data
        .lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .find(|row: &Value| row["p"].as_u64() == Some(p))
        .unwrap();
    assert_eq!(row["factorization_verified"], true);
    row["finite_rational_weierstrass_roots"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_u64().unwrap())
        .collect()
}

fn run_model(bits: u64, p: u64, row: &Value) -> Value {
    let spec = &row["workload"]["curve_spec"];
    let s = Scholten::new(p);
    let g = source_point(&json!([spec["gx"], spec["gy"]]));
    let dg = s.tau(s.from_short(g));
    let mut cases = Vec::new();
    for root in roots(p) {
        let t = s.j.point(root, 0);
        let minus_g = s.j.neg(dg);
        let checks = [
            (Divisor::O, dg),
            (dg, dg),
            (dg, minus_g),
            (t, dg),
            (t, t),
            (t, s.j.neg(t)),
        ];
        for (index, (a, b)) in checks.into_iter().enumerate() {
            let out = case(&s, a, b, dg);
            cases.push(json!({"root":root,"case_index":index,"result":out}));
        }
    }
    let all_correct = cases.iter().all(|c| {
        c["result"]["differential_correct"] == true
            && c["result"]["signed_recovery_correct"] == true
    });
    json!({"schema":"kummer-differential-model/v1","bits":bits,"p":p,
        "r":spec["r"],"root_count":5,"exception_cases":cases.len(),
        "all_correct":all_correct,"cases":cases})
}

fn run_target(bits: u64, row: &Value) -> Value {
    let w = &row["workload"];
    let spec = &w["curve_spec"];
    let p = spec["p"].as_u64().unwrap();
    let r = spec["r"].as_u64().unwrap();
    let s = Scholten::new(p);
    let source = s.short_curve();
    let g = source_point(&json!([spec["gx"], spec["gy"]]));
    let q = target_point(&w["target"]);
    let planted = w["planted"].as_u64().unwrap();
    let source_scalar_verified = source.valid(g) && source.valid(q) && source.mul(g, planted) == q;
    let dg = s.tau(s.from_short(g));
    let dq = s.tau(s.from_short(q));
    let seed: u64 = row["algorithm_seed"].as_str().unwrap().parse().unwrap();
    let a = (seed ^ 0x9e37_79b9_7f4a_7c15) % r;
    let b = ((seed.rotate_left(17) ^ 0xd1b5_4a32_d192_ed03) % (r - 1)) + 1;
    let pairs = [
        (0, 0),
        (1, 0),
        (0, 1),
        (1, 1),
        (1, r - 1),
        (a, b),
        (r - 1, r - 1),
    ];
    let mut cases = Vec::new();
    for (index, (aa, bb)) in pairs.into_iter().enumerate() {
        let left = s.j.mul(dg, aa);
        let right = s.j.mul(dq, bb);
        let oracle = s.tau(s.from_short(source.add(source.mul(g, aa), source.mul(q, bb))));
        let sum = s.j.add(left, right);
        let mut out = case(&s, left, right, dg);
        out["label_transfer_correct"] = json!(oracle == sum);
        cases.push(json!({"case_index":index,"a":aa,"b":bb,"result":out}));
    }
    let all_correct = source_scalar_verified
        && cases.iter().all(|c| {
            c["result"]["differential_correct"] == true
                && c["result"]["signed_recovery_correct"] == true
                && c["result"]["label_transfer_correct"] == true
        });
    json!({"schema":"kummer-differential-target/v1","bits":bits,"p":p,"r":r,
        "target_index":w["target_index"],"workload_id":w["workload_id"],
        "workload_sha256":w["workload_sha256"],"algorithm_seed":row["algorithm_seed"],
        "planted":planted,"source_scalar_verified":source_scalar_verified,
        "case_count":cases.len(),"all_correct":all_correct,"cases":cases})
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    assert_eq!(args.len(), 3, "usage: kummer-walk-gate BITS OUTDIR");
    let bits: u64 = args[1].parse().unwrap();
    assert!([16, 20, 24, 28, 32, 36, 40].contains(&bits));
    let rows = archive_rows(bits);
    assert_eq!(rows.len(), 180);
    let output_dir = &args[2];
    std::fs::create_dir_all(output_dir).unwrap();
    let mut targets =
        BufWriter::new(File::create(format!("{output_dir}/target-gate-{bits}.jsonl")).unwrap());
    let mut models = BufWriter::new(
        File::create(format!("{output_dir}/model-exceptions-{bits}.jsonl")).unwrap(),
    );
    let mut seen = BTreeSet::new();
    for row in rows {
        let p = row["workload"]["curve_spec"]["p"].as_u64().unwrap();
        if seen.insert(p) {
            writeln!(models, "{}", run_model(bits, p, &row)).unwrap();
        }
        writeln!(targets, "{}", run_target(bits, &row)).unwrap();
    }
    assert_eq!(seen.len(), 3);
    targets.flush().unwrap();
    models.flush().unwrap();
}
