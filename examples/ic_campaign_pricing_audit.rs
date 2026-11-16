//! Independent accounting reconstruction; never alters sealed records.
use crypto_lib::cryptanalysis::ecbench::{band, methods::PRICED_NATIVE, record::Record};
use crypto_lib::cryptanalysis::gaudry_quartic_bin::{find_bin_curve, fq_muls_per_add, BinCurve};
use crypto_lib::cryptanalysis::ic_boundary::pinned_calibration_ratios;
use rand::{rngs::StdRng, SeedableRng};
use serde_json::{json, Map, Value};
use std::collections::BTreeMap;
use std::path::Path;

fn near(a: f64, b: f64) -> bool {
    (a - b).abs() <= 1e-12 * (1.0 + a.abs().max(b.abs()))
}
fn lift_muls(inst: &BinCurve) -> f64 {
    // The adapter's exact deterministic count recipe, not a wall timer.
    let c = &inst.curve;
    let mut rng = StdRng::seed_from_u64(0x5e);
    let (mut muls, mut n) = (0u64, 0u64);
    while n < 32 {
        let x = c.f.random(&mut rng);
        let before = c.f.f.muls();
        if c.lift_x(&x, &mut rng).is_some() {
            muls += c.f.f.muls() - before;
            n += 1;
        }
    }
    muls as f64 / n as f64
}
fn virtual_row(v: &Value) -> (Map<String, Value>, Value) {
    let spec = &v["workload"]["curve_spec"];
    let m = spec["m"].as_u64().unwrap() as u32;
    let seed = spec["seed"].as_u64().unwrap();
    let h = spec["max_cofactor"].as_u64().unwrap();
    let inst = find_bin_curve(m, seed, h).unwrap();
    let facts = &v["workload"]["curve"];
    assert_eq!(facts["r"], inst.r);
    assert_eq!(facts["group_order"], inst.group_order);
    for (j, coord) in [inst.generator.x, inst.generator.y].into_iter().enumerate() {
        let text = facts["generator"][j].as_str().unwrap();
        assert_eq!(
            u64::from_str_radix(text.trim_start_matches("0x"), 16).unwrap(),
            inst.curve.f.pack(&coord)
        );
    }
    let per_add = fq_muls_per_add(&inst.curve) as f64;
    assert!(per_add > 0.0);
    let lift = lift_muls(&inst);
    let row = json!({"ns_per_double":1.0,"ns_per_row_op":1.0/per_add,
        "ns_per_lookup":1.0/per_add,"ns_per_sqrt":lift/per_add});
    let evidence = json!({"m":m,"seed":seed,"max_cofactor":h,"curve_binding_checked":true,
        "fq_operations_per_add":per_add,"fq_operations_per_successful_lift":lift,
        "ratios_gae_per_native":row,"unit":"deterministic native-count convention; not nanoseconds or hardware calibration"});
    (row.as_object().unwrap().clone(), evidence)
}
#[derive(Default)]
struct Cell {
    planned: u64,
    verified: u64,
    raw: Vec<f64>,
    compat: [Vec<f64>; 3],
    all: [Vec<f64>; 3],
    missing: [bool; 3],
}
fn main() {
    let args: Vec<_> = std::env::args().skip(1).collect();
    assert_eq!(args.len(), 1, "study directory");
    let study = Path::new(&args[0]);
    let mut virtuals = BTreeMap::<String, (Map<String, Value>, Value)>::new();
    let mut cells = BTreeMap::<(String, String), Cell>::new();
    let mut audit = vec![];
    let weights = [0.0, 0.1, 1.0];
    for name in [
        "structured-n7-20261025-01",
        "structured-n13-20261025-01",
        "structured-n23-20261025-01",
        "quartic-smoke-20261025-02",
    ] {
        let bytes =
            std::fs::read(study.join("pipeline-runs").join(name).join("records.jsonl")).unwrap();
        for item in serde_json::Deserializer::from_slice(&bytes).into_iter::<Value>() {
            let v = item.unwrap();
            let r: Record = serde_json::from_value(v.clone()).unwrap();
            if r.warmup {
                continue;
            }
            let key = (r.workload.curve.slug.clone(), r.arm.clone());
            let cell = cells.entry(key).or_default();
            cell.planned += 1;
            let Some(raw) = r.cost.total_gae else {
                continue;
            };
            let ic = r.method.id == "ic.pipeline";
            let quartic = v["workload"]["curve_spec"]["kind"] == "f2m4_torsion";
            let (mut c, mut nc) = (0.0, 0u64);
            let mut solver_charge = 0.0;
            let mut counted_fq = 0u64;
            let solver = &v["detail"]["decomposition"]["solver"];
            let provenance = solver["priced_by"].as_str().unwrap_or("none");
            let mut rebuilt = None;
            if ic {
                let row = if quartic {
                    if !virtuals.contains_key(&r.workload.curve.slug) {
                        virtuals.insert(r.workload.curve.slug.clone(), virtual_row(&v));
                    }
                    virtuals[&r.workload.curve.slug].0.clone()
                } else {
                    pinned_calibration_ratios("koblitz", &r.workload.curve.slug).unwrap_or_default()
                };
                let ratio = |unit: &str| row.get(unit).and_then(Value::as_f64);
                let pinned: Vec<_> = v["detail"]["calibration_pinned_units"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|x| x.as_str().unwrap())
                    .collect();
                let mut native = 0.0;
                for phase in &r.phases {
                    for &(counter, unit) in PRICED_NATIVE {
                        let n = phase.native.get(counter).copied().unwrap_or(0);
                        if let Some(k) = ratio(unit) {
                            native += n as f64 * k;
                            if pinned.contains(&unit) {
                                c += n as f64 * k;
                                nc += n;
                            }
                        }
                    }
                }
                let ops = solver["ops"].as_u64().unwrap_or(0);
                match provenance {
                    "pinned" => {
                        assert!(solver["op_unit"].as_str().unwrap().contains("word XOR"));
                        solver_charge = ops as f64 * ratio("ns_per_word_xor").unwrap();
                        c += solver_charge;
                        nc += ops;
                        assert!(near(solver_charge, solver["gae"].as_f64().unwrap()));
                    }
                    "counted" => {
                        assert!(quartic);
                        assert_eq!(solver["op_unit"], "F_q multiplications");
                        solver_charge = ops as f64 * ratio("ns_per_row_op").unwrap();
                        counted_fq = ops;
                        assert!(near(solver_charge, solver["gae"].as_f64().unwrap()));
                    }
                    "measured" | "none" => {}
                    other => panic!("unhandled solver provenance {other}"),
                }
                let group: f64 = r.phases.iter().map(|p| (p.adds + p.doubles) as f64).sum();
                let expected = group + native + solver_charge;
                assert!(
                    near(expected, raw),
                    "{} raw charge mismatch: {expected} vs {raw}",
                    r.record_id
                );
                rebuilt = Some(expected);
            } else {
                (c, nc) = band::calibrated_native(&r).unwrap();
            }
            let (u, missing) = band::unpriced_total(&r);
            let fixed = rebuilt.unwrap_or(raw);
            let sq = (r.workload.curve.r as f64).sqrt();
            let mut compat = [None; 3];
            let mut all = [None; 3];
            for (j, &w) in weights.iter().enumerate() {
                if !(missing && w > 0.0) {
                    compat[j] = Some((fixed - c + w * (u + nc) as f64) / sq);
                    all[j] = Some(
                        (fixed - c - if counted_fq > 0 { solver_charge } else { 0.0 }
                            + w * (u + nc + counted_fq) as f64)
                            / sq,
                    );
                }
                if let Ok(Some(want)) = band::priced_s(&r, w) {
                    assert!(
                        near(compat[j].unwrap(), want),
                        "compatibility band mismatch"
                    );
                }
            }
            audit.push(json!({"session":name,"record_id":r.record_id,"curve":r.workload.curve.slug,"arm":r.arm,
                "outcome":r.outcome.status,"solver_priced_by":provenance,"solver_ops":solver["ops"],
                "solver_charge_gae_recomputed":solver_charge,"raw_gae":raw,"rebuilt_gae":rebuilt,
                "solver_wall_charge_in_reported_cost":false,"compatibility_s":compat,"all_native_scenario_s":all,
                "counted_fq_solver_ops_varied_only_in_additional_scenario":counted_fq}));
            if r.outcome.status == "verified" {
                cell.verified += 1;
                cell.raw.push(raw / sq);
                for j in 0..3 {
                    if let (Some(a), Some(b)) = (compat[j], all[j]) {
                        cell.compat[j].push(a);
                        cell.all[j].push(b);
                    } else {
                        cell.missing[j] = true;
                    }
                }
            }
        }
    }
    let mean = |xs: &[f64]| xs.iter().sum::<f64>() / xs.len() as f64;
    let rows:Vec<_>=cells.into_iter().map(|((curve,arm),c)| {
        let complete=c.planned==c.verified && c.verified>0;
        let comp:Vec<_>=(0..3).map(|j|(complete&&!c.missing[j]).then(||mean(&c.compat[j]))).collect();
        let all:Vec<_>=(0..3).map(|j|(complete&&!c.missing[j]).then(||mean(&c.all[j]))).collect();
        json!({"curve":curve,"arm":arm,"planned":c.planned,"verified":c.verified,"complete":complete,
            "raw_s_mean":complete.then(||mean(&c.raw)),"compatibility_s_mean":comp,"all_native_scenario_s_mean":all})
    }).collect();
    println!("{}",serde_json::to_string_pretty(&json!({"schema":"ic-campaign-pricing-audit/v1",
        "status":"raw_ic_costs_rebuilt_from_native_counts","weights":weights,"records":audit,"rows":rows,
        "quartic_virtual_calibrations":virtuals.into_iter().map(|(curve,(_,ev))|json!({"curve":curve,"evidence":ev})).collect::<Vec<_>>(),
        "correction":"Completed n13/n23 structured S uses pinned word rates, not solver wall time. The n7 measured solver term is removed. Quartic solver work is counted under its virtual field-operation convention.",
        "scope":"Accounting reconstruction and labeled sensitivity scenarios only; missing construction, splitting, word scans and physical calibration remain missing. No algorithmic improvement or full-price promotion."})).unwrap());
}
