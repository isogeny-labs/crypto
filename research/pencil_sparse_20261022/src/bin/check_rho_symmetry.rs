//! Check whether the source curves admit an extra Frobenius or geometric automorphism fold.
use five_leads_study::three_leads::Field;
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::fs;
use std::process::Command;

fn main() {
    let mut specs = BTreeMap::<u64, (u64, u64)>::new();
    for bits in [16, 20, 24, 28, 32] {
        let member = format!("sessions_projective_chart_v2/order-genus-{bits}/records.jsonl");
        let archive = "research/five_leads_e2e_20261019/sessions_projective_chart_v2.tar.zst";
        let output = Command::new("tar")
            .args(["--zstd", "-xOf", archive, &member])
            .output()
            .expect("frozen archive available");
        assert!(output.status.success());
        for line in String::from_utf8(output.stdout).unwrap().lines() {
            let row: Value = serde_json::from_str(line).unwrap();
            if row["arm"] != "genus_projective" {
                continue;
            }
            let spec = &row["workload"]["curve_spec"];
            let p = spec["p"].as_u64().unwrap();
            let a = spec["a"].as_u64().unwrap();
            let b = spec["b"].as_u64().unwrap();
            if let Some(previous) = specs.insert(p, (a, b)) {
                assert_eq!(previous, (a, b));
            }
        }
    }
    let mut checks = Vec::new();
    for (p, (a, b)) in specs {
        let f = Field::new(p, 2, 1);
        f.validate().unwrap();
        assert_ne!(a, 0);
        assert_ne!(b, 0);
        let a3 = f.mul(f.sq(a), a);
        let b2 = f.sq(b);
        let four_a3 = f.mul(4, a3);
        let denominator = f.add(four_a3, f.mul(27, b2));
        assert_ne!(denominator, 0, "source curve nonsingular");
        let j = f.div(f.mul(1728 % p, four_a3), denominator);
        let frobenius_fixes_j = f.conjugate(j) == j;
        checks.push(json!({
            "p": p, "a": a, "b": b, "j": j,
            "a_in_base_field": a < p, "b_in_base_field": b < p,
            "frobenius_fixes_j": frobenius_fixes_j,
            "extra_geometric_automorphism": j == 0 || j == 1728 % p
        }));
    }
    assert_eq!(checks.len(), 15);
    assert!(checks.iter().all(|check| {
        check["frobenius_fixes_j"] == false && check["extra_geometric_automorphism"] == false
    }));
    let result = json!({
        "schema": "pencil-rho-symmetry-check/v1",
        "reference": "source-negation-rho",
        "field": "F_(p^2) with basis 1,w and w^2=-1",
        "test": "j=1728*4a^3/(4a^3+27b^2); j^p != j and j != 0,1728",
        "conclusion": "the source curves have no Frobenius self-map or extra geometric automorphism fold; negation is the strongest available symmetry",
        "checks": checks
    });
    let path = "research/pencil_sparse_20261022/results/rho-symmetry-check.json";
    fs::write(path, serde_json::to_vec_pretty(&result).unwrap()).unwrap();
    println!(
        "{path}: {} source curves checked",
        result["checks"].as_array().unwrap().len()
    );
}
