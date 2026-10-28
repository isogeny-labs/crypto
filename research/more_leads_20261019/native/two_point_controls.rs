//! Matched-target two-point genus-2 collector control, using the existing v2 solver.
use pencil_relation_gate::three_leads::{solve_genus2_v2, Point, Scholten, StudyInstance};
use serde_json::{json, Value};
use std::fs;

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
    assert_eq!(source.c, [number("b"), number("a"), 0, 1]);
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
    for target_index in 0..20u64 {
        let scalar = (123 + 1777 * target_index) % r;
        let target = source.mul(generator, scalar);
        let seed = 2026102001 + target_index;
        let result = solve_genus2_v2(&inst, target, seed, 128700, false, true);
        assert_eq!(result.recovered, Some(scalar));
        println!(
            "{}",
            json!({"schema":"pencil-matched-two-point/v1",
            "target_index":target_index,"scalar":scalar,"seed":seed,
            "recovered":result.recovered,"counts":result.counts,
            "native":result.native,"phases":result.phases})
        );
    }
}
