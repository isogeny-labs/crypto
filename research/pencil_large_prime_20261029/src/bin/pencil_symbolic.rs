//! Frozen cohort driver; the reusable solver accepts no planted scalar.
use five_leads_study::three_leads::{solve_rho, Elliptic, Field, Point, StudyInstance};
use pencil_symbolic_graph::{
    allocation::ObservedSystem,
    native_solver::{solve_one, Config},
};
use serde_json::{json, Value};
use std::{process::Command, time::Instant};
#[global_allocator]
static ALLOCATOR: ObservedSystem = ObservedSystem::new();
fn main() {
    let args: Vec<String> = std::env::args().collect();
    assert!(
        args.len() == 4 && args[1] == "--cohort",
        "usage: pencil-symbolic --cohort BITS P; freeze protocol before use"
    );
    cohort_main(args[2].parse().unwrap(), args[3].parse().unwrap());
}
fn cohort_main(bits: u64, selected_p: u64) {
    assert!([16, 20, 24, 28, 32, 36, 40].contains(&bits));
    let matrix_coefficient_cap: usize = std::env::var("PENCIL_MATRIX_CAP")
        .ok()
        .map(|value| value.parse().expect("PENCIL_MATRIX_CAP integer"))
        .unwrap_or(10_000_000);
    assert!((1_000_000..=25_000_000).contains(&matrix_coefficient_cap));
    let member = format!("sessions_projective_chart_v2/order-genus-{bits}/records.jsonl");
    let archive = "research/five_leads_e2e_20261019/sessions_projective_chart_v2.tar.zst";
    let output = Command::new("tar")
        .args(["--zstd", "-xOf", archive, &member])
        .output()
        .expect("open frozen input archive");
    assert!(
        output.status.success(),
        "archive input: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let input = String::from_utf8(output.stdout).unwrap();
    let rows: Vec<Value> = input
        .lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .filter(|row: &Value| {
            row["arm"] == "genus_projective"
                && row["workload"]["curve_spec"]["p"].as_u64() == Some(selected_p)
        })
        .collect();
    assert_eq!(rows.len(), 60, "one curve has 60 frozen targets");
    let spec = &rows[0]["workload"]["curve_spec"];
    let p = spec["p"].as_u64().unwrap();
    let r = spec["r"].as_u64().unwrap();
    let order = spec["group_order"].as_u64().unwrap();
    let generator = Point::affine(spec["gx"].as_u64().unwrap(), spec["gy"].as_u64().unwrap());
    let inst = StudyInstance {
        curve: Elliptic {
            f: Field::new(p, 2, 1),
            c: [
                spec["b"].as_u64().unwrap(),
                spec["a"].as_u64().unwrap(),
                0,
                1,
            ],
            binary: false,
        },
        order,
        r,
        h: order / r,
        generator,
        family: "scholten".into(),
        parameter: 0,
        evidence: Value::Null,
    };
    let unit = inst.fresh();
    let twice = unit.curve.mul(unit.generator, 2);
    unit.curve.f.reset();
    let _ = unit.curve.add(unit.generator, twice);
    let source_add = unit.curve.f.counts();
    let target_limit: usize = std::env::var("PENCIL_MAX_TARGETS")
        .ok()
        .map(|value| value.parse().expect("PENCIL_MAX_TARGETS integer"))
        .unwrap_or(60);
    assert!((1..=60).contains(&target_limit));
    let target_start: usize = std::env::var("PENCIL_TARGET_START")
        .ok()
        .map(|value| value.parse().expect("PENCIL_TARGET_START integer"))
        .unwrap_or(0);
    assert!(target_start + target_limit <= 60);
    let denominator: usize = std::env::var("PENCIL_SMALL_DENOMINATOR")
        .expect("explicit frozen small-base denominator required")
        .parse()
        .unwrap();
    assert!([2, 4, 8, 16].contains(&denominator));
    for row in rows.into_iter().skip(target_start).take(target_limit) {
        let workload = &row["workload"];
        let scalar = workload["planted"].as_u64().unwrap();
        let hx = |v: &Value| {
            u64::from_str_radix(v.as_str().unwrap().trim_start_matches("0x"), 16).unwrap()
        };
        let target = Point::affine(hx(&workload["target"][0]), hx(&workload["target"][1]));
        let seed: u64 = row["algorithm_seed"].as_str().unwrap().parse().unwrap();
        let cap: u64 = row["method"]["params"]["trial_cap"]
            .as_str()
            .unwrap()
            .parse()
            .unwrap();
        let result = solve_one(
            &inst,
            target,
            seed,
            Config {
                denominator,
                matrix_coefficient_cap,
            },
            Some(&ALLOCATOR),
        )
        .expect("valid pencil configuration");
        let rho_allocation_start = ALLOCATOR.begin();
        let rho_clock = Instant::now();
        let mut rho = solve_rho(&inst, target, seed, cap, true, false);
        let rho_verified = if let Some(k) = rho.recovered {
            let verifier = inst.fresh();
            let valid = verifier.curve.mul(generator, k) == target && k == scalar;
            rho.counts = rho.counts + verifier.curve.f.counts();
            valid
        } else {
            false
        };
        let rho_elapsed_ns = rho_clock.elapsed().as_nanos();
        let rho_allocation = ALLOCATOR.finish(rho_allocation_start);
        let mut output = result.detail.clone();
        output["schema"] = json!("pencil-symbolic-cohort-target/v2");
        output["bits"] = json!(bits);
        output["workload_id"] = workload["workload_id"].clone();
        output["workload_sha256"] = workload["workload_sha256"].clone();
        output["target_index"] = workload["target_index"].clone();
        output["algorithm_seed"] = json!(seed.to_string());
        output["cap"] = json!(cap);
        output["planted"] = json!(scalar);
        output["verified"] = json!(result.source_verified && result.recovered == Some(scalar));
        output["source_add"] = json!(source_add);
        output["cold_setup"] = result.setup.clone();
        output["rho"] = json!({"recovered":rho.recovered,"verified":rho_verified,"counts":rho.counts,
            "allocation":rho_allocation,"elapsed_ns":rho_elapsed_ns,
            "native":rho.native,"phases":rho.phases,"detail":rho.detail});
        println!("{output}");
    }
}
