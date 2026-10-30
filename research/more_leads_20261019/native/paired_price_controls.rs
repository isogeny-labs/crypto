//! Repeated same-process wall diagnostics for the two existing controls.
use pencil_relation_gate::three_leads::{
    solve_genus2_v2, solve_rho, Point, Scholten, StudyInstance,
};
use serde_json::{json, Value};
use std::fs;
use std::time::Instant;

fn main() {
    let path = "research/three_leads_e2e_20261018/genus-order-fixtures-v1/order-genus-16.json";
    let spec: Value = serde_json::from_str(&fs::read_to_string(path).unwrap()).unwrap();
    let row = &spec["workloads"]["curves"][0];
    let number = |key: &str| row[key].as_u64().unwrap();
    let p = number("p");
    let r = number("r");
    let s = Scholten::new(p);
    let source = s.short_curve();
    let generator = Point::affine(number("gx"), number("gy"));
    let inst = StudyInstance {
        curve: source.clone(),
        order: number("group_order"),
        r,
        h: number("group_order") / r,
        generator,
        family: "scholten".into(),
        parameter: 0,
        evidence: json!({}),
    };
    inst.validate().unwrap();
    for round in 0..3u64 {
        for target_index in 0..20u64 {
            let scalar = (123 + 1777 * target_index) % r;
            let target = source.mul(generator, scalar);
            let seed = 2026102001 + target_index;
            let order = if (round + target_index) % 2 == 0 {
                ["rho", "two_point"]
            } else {
                ["two_point", "rho"]
            };
            for method in order {
                let start = Instant::now();
                let result = if method == "rho" {
                    solve_rho(&inst, target, seed, 128700, true, false)
                } else {
                    solve_genus2_v2(&inst, target, seed, 128700, false, true)
                };
                let recovered = result.recovered.expect("control must solve");
                source.f.reset();
                assert_eq!(source.mul(generator, recovered), target);
                let verification_counts = source.f.counts();
                let elapsed_ns = start.elapsed().as_nanos();
                assert_eq!(recovered, scalar);
                println!(
                    "{}",
                    json!({"schema":"pencil-paired-control/v1",
                    "round":round,"target_index":target_index,"method":method,
                    "scalar":scalar,"seed":seed,"recovered":recovered,
                    "elapsed_ns":elapsed_ns,"solver_counts":result.counts,
                    "verification_counts":verification_counts,
                    "native":result.native,"phases":result.phases})
                );
            }
        }
    }
}
