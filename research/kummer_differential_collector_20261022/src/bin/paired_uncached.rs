//! Run the frozen uncached Kummer collector and native strong rho in one process.
#[path = "../main_uncached.rs"]
mod candidate;
use five_leads_study::three_leads::{Elliptic, Field, Point, StudyInstance, solve_rho};
use serde_json::{Value, json};
use std::{fs, process::Command};
fn n(v: &Value, k: &str) -> u64 {
    v[k].as_u64().unwrap()
}
fn s<'a>(v: &'a Value, k: &str) -> &'a str {
    v[k].as_str().unwrap()
}
fn point(v: &Value) -> Point {
    Point::affine(
        u64::from_str_radix(v[0].as_str().unwrap().trim_start_matches("0x"), 16).unwrap(),
        u64::from_str_radix(v[1].as_str().unwrap().trim_start_matches("0x"), 16).unwrap(),
    )
}
fn main() {
    let outdir = "research/kummer_differential_collector_20261022/results/paired/uncached";
    candidate::run(outdir);
    let cap_targets = std::env::var("KUMMER_TARGET_CAP")
        .ok()
        .and_then(|x| x.parse::<usize>().ok())
        .unwrap_or(1260);
    let start_index = std::env::var("KUMMER_START_INDEX")
        .ok()
        .and_then(|x| x.parse::<usize>().ok())
        .unwrap_or(0);
    let mut completed = 0;
    let mut seen = 0;
    for bits in [16, 20, 24, 28, 32, 36, 40] {
        let member = format!("sessions_projective_chart_v2/order-genus-{bits}/records.jsonl");
        let archive = "research/five_leads_e2e_20261019/sessions_projective_chart_v2.tar.zst";
        let raw = Command::new("tar")
            .args(["--zstd", "-xOf", archive, &member])
            .output()
            .unwrap();
        assert!(raw.status.success());
        let mut lines = String::new();
        for line in String::from_utf8(raw.stdout).unwrap().lines() {
            let record: Value = serde_json::from_str(line).unwrap();
            if record["arm"] != "rho_neg" {
                continue;
            }
            let idx = seen;
            seen += 1;
            if idx < start_index || completed >= cap_targets {
                continue;
            }
            let w = &record["workload"];
            let sp = &w["curve_spec"];
            let p = n(sp, "p");
            let r = n(sp, "r");
            let order = n(sp, "group_order");
            let e = Elliptic {
                f: Field::new(p, 2, n(sp, "modulus")),
                c: [n(sp, "b"), n(sp, "a"), 0, 1],
                binary: false,
            };
            let g = Point::affine(n(sp, "gx"), n(sp, "gy"));
            let q = point(&w["target"]);
            let inst = StudyInstance {
                curve: e.clone(),
                order,
                r,
                h: order / r,
                generator: g,
                family: s(sp, "family").to_string(),
                parameter: n(sp, "parameter"),
                evidence: json!({"paired":"kummer"}),
            };
            assert!(e.valid(g) && e.valid(q));
            assert_eq!(e.mul(g, n(w, "planted")), q);
            let two = e.mul(g, 2);
            e.f.reset();
            let _sum = e.add(g, two);
            let unit = e.f.counts();
            let archived = &record["counters"];
            assert_eq!(unit.m, n(archived, "source_add_multiplications"));
            assert_eq!(unit.s, n(archived, "source_add_squarings"));
            assert_eq!(unit.i, n(archived, "source_add_inversions"));
            let seed = s(&record, "algorithm_seed").parse::<u64>().unwrap();
            let cap = s(&record["method"]["params"], "trial_cap")
                .parse::<u64>()
                .unwrap();
            let outcome = solve_rho(&inst, q, seed, cap, true, false);
            let k = outcome.recovered.expect("rho exhausted");
            assert_eq!(k, n(w, "planted"));
            e.f.reset();
            assert_eq!(e.mul(g, k), q);
            let c = outcome.counts + e.f.counts();
            let mut counters = json!({"field_multiplications":c.m,"field_squarings":c.s,"field_inversions":c.i,
          "field_additions_uncharged":c.field_adds,"source_add_multiplications":unit.m,
          "source_add_squarings":unit.s,"source_add_inversions":unit.i,"source_ec_additions":c.ec_adds,
          "jacobian_additions":c.jac_adds});
            for (key, value) in &outcome.native {
                counters[key] = json!(value);
            }
            let match_archived = counters["field_multiplications"]
                == archived["field_multiplications"]
                && counters["field_squarings"] == archived["field_squarings"]
                && counters["field_inversions"] == archived["field_inversions"];
            lines.push_str(&format!("{}\n",json!({"schema":"kummer-paired-rho/v1","bits":bits,
          "workload_id":w["workload_id"],"p":p,"r":r,"target_index":w["target_index"],
          "planted":w["planted"],"recovered":k,"status":"verified","source_verified":true,
          "algorithm_seed":seed,"trial_cap":cap,"counters":counters,"archived_arithmetic_match":match_archived,
          "same_process_as_candidate":true,"binary":"paired_uncached"})));
            completed += 1;
        }
        if cap_targets == 1260 {
            assert_eq!(lines.lines().count(), 180);
        }
        let rho_dir = "research/kummer_differential_collector_20261022/results/paired/rho";
        fs::create_dir_all(rho_dir).unwrap();
        fs::write(format!("{rho_dir}/rho-{bits}.jsonl"), lines).unwrap();
    }
    assert_eq!(completed, cap_targets);
    println!("paired_rho={completed}");
}
