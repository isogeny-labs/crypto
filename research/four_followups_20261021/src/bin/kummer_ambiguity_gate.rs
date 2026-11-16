//! Exact sign-ambiguity witness on each transferred exact-order subgroup.
use five_leads_study::three_leads::{Divisor, Point, Scholten};
use serde_json::{json, Value};
use std::process::Command;

fn canonical(s: &Scholten, d: Divisor) -> Divisor {
    d.min(s.j.neg(d))
}
fn descriptor(d: Divisor) -> Value {
    json!({"degree":d.degree,"u":d.u,"v":d.v})
}
fn main() {
    let archive = "research/five_leads_e2e_20261019/sessions_projective_chart_v2.tar.zst";
    for bits in [16, 20, 24, 28, 32, 36, 40] {
        let member = format!("sessions_projective_chart_v2/order-genus-{bits}/records.jsonl");
        let out = Command::new("tar")
            .args(["--zstd", "-xOf", archive, &member])
            .output()
            .unwrap();
        assert!(out.status.success());
        let content = String::from_utf8(out.stdout).unwrap();
        let mut seen = std::collections::BTreeSet::new();
        for row in content
            .lines()
            .map(|line| serde_json::from_str::<Value>(line).unwrap())
        {
            if row["arm"] != "rho_neg" {
                continue;
            }
            let spec = &row["workload"]["curve_spec"];
            let p = spec["p"].as_u64().unwrap();
            if !seen.insert(p) {
                continue;
            }
            let r = spec["r"].as_u64().unwrap();
            let g = Point::affine(spec["gx"].as_u64().unwrap(), spec["gy"].as_u64().unwrap());
            let s = Scholten::new(p);
            assert_eq!(s.short_curve().c[0], spec["b"].as_u64().unwrap());
            assert_eq!(s.short_curve().c[1], spec["a"].as_u64().unwrap());
            assert!(s.short_curve().valid(g));
            let d = s.tau(s.from_short(g));
            let order_check = d != Divisor::O && s.j.mul(d, r) == Divisor::O;
            let two = s.j.mul(d, 2);
            let plus = s.j.add(d, two);
            let minus = s.j.add(d, s.j.neg(two));
            let arithmetic_check = plus == s.j.mul(d, 3) && minus == s.j.neg(d);
            let same_inputs = canonical(&s, two) == canonical(&s, s.j.neg(two));
            let different_outputs = canonical(&s, plus) != canonical(&s, minus);
            println!(
                "{}",
                json!({
                    "schema":"four-followups-kummer-ambiguity/v1", "bits":bits, "p":p, "r":r,
                    "source_generator":[spec["gx"],spec["gy"]],
                    "order_check":order_check,"arithmetic_check":arithmetic_check,
                    "same_kummer_inputs":same_inputs,"different_kummer_outputs":different_outputs,
                    "K(P)":descriptor(canonical(&s,d)),"K(2P)":descriptor(canonical(&s,two)),
                    "K(P+2P)":descriptor(canonical(&s,plus)),
                    "K(P-2P)":descriptor(canonical(&s,minus))
                })
            );
        }
        assert_eq!(seen.len(), 3);
    }
}
