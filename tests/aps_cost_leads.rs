//! Correctness bridge to the ordinary-residue reader; no benchmark is run.
#[path = "../tools/aps-export/src/value.rs"]
mod value;
pub use value::*;
#[path = "../tools/aps-export/src/audit.rs"]
mod audit;
use crypto_lib::cryptanalysis::{
    ecbench::{
        methods::{self, MethodSpec},
        workload::{CurveSpec, Instance},
    },
    ic_boundary::{CountedGroup, GroupOps},
};
use serde_json::{json, Value};
use std::collections::BTreeMap;

fn produced(changes: &[(&str, &str)], seed: u64) -> Value {
    let orbit = changes
        .iter()
        .any(|(key, value)| *key == "orbit" && *value == "cubic");
    let (p, a, b, n, gx, gy, k) = if orbit {
        (103, 0, 5, 97, 2, 42, 17)
    } else {
        (101, 59, 6, 107, 78, 4, 37)
    };
    let spec = CurveSpec::PrimeExplicit {
        p,
        a,
        b,
        group_order: n,
        r: n,
        gx,
        gy,
    };
    let instance = spec.build().unwrap();
    let facts = instance.facts(&spec);
    let Instance::Prime(inst) = &instance else {
        unreachable!()
    };
    let q = inst
        .curve
        .mul(&mut GroupOps::default(), inst.generator_point(), k);
    let target = [format!("{:x}", q.x), format!("{:x}", q.y)];
    let mut params: BTreeMap<String, String> = [
        ("encoding", "partitioned3"),
        ("roots", "exact"),
        ("stop", "exhaustive"),
        ("labels", "guaranteed"),
        ("subset_size", "0"),
        ("subset_profile", "2,2,2"),
        ("attempts", "32"),
        ("degree", "24"),
        ("budget_ms", "10000"),
        ("sampling_draws", "1000"),
        ("orbit", "none"),
        ("beta", "0"),
        ("lambda", "0"),
        ("prune", "false"),
        ("guess", "none"),
        ("trace", "certified"),
        ("trace_entries", "8"),
        ("trace_words", "100000"),
        ("growth", "none"),
        ("growth_ceiling", "4"),
    ]
    .map(|(a, b)| (a.into(), b.into()))
    .into();
    params.extend(changes.iter().map(|(a, b)| ((*a).into(), (*b).into())));
    let method = methods::resolve(&MethodSpec {
        id: "ic.aps_v3".into(),
        params,
    })
    .unwrap();
    let report = methods::solve(&method, &instance, &facts, &target, seed).unwrap();
    assert_eq!(
        report.recovered.as_ref().map(ToString::to_string),
        Some(k.to_string()),
        "{}",
        report.detail
    );
    let r = json!({"schema":"ecbench.record/v1","record_id":"", "workload":{"curve_spec":spec,"curve":facts,"target":target},
        "method":method,"algorithm_seed":seed.to_string(),"outcome":{"status":"verified","recovered":k.to_string()},
        "detail":report.detail,"phases":report.phases,"online":report.online,"counters":report.counters,
        "time":{"solve_wall_ns":report.solve_wall_ns,"method_call_wall_ns":null}});
    let mut r = r;
    let raw = serde_json::to_vec(&r).unwrap();
    r["record_id"] = json!(format!("ECR1h{}", &hash(&raw)[..12]));
    audit::record_line(&serde_json::to_string(&r).unwrap()).unwrap()
}
fn check(r: &Value) {
    assert_eq!(audit::metrics(r).unwrap()["aps"], true);
    let v = audit::mathematical(r, audit::MAX_ORACLE_CANDIDATES).unwrap();
    assert_eq!(v["in_budget_witnesses_checked"], 1);
    assert_eq!(v["execution_attested"], false);
}
#[test]
fn actual_chart_and_unequal_hybrids_pass_independent_full_set_reader() {
    check(&produced(&[("encoding", "line_chart3")], 20261026));
    for guess in ["0", "1", "2"] {
        check(&produced(
            &[("subset_profile", "1,3,2"), ("guess", guess)],
            20261026,
        ));
    }
}
#[test]
fn unequal_four_point_profiles_restore_each_original_guessed_coordinate() {
    for encoding in ["expanded4", "split4"] {
        for guess in ["0", "1", "2", "3"] {
            check(&produced(
                &[
                    ("encoding", encoding),
                    ("subset_profile", "1,2,1,2"),
                    ("guess", guess),
                ],
                20261026,
            ));
        }
    }
}
#[test]
fn actual_certified_orbit_chart_matches_the_saturated_producer_and_reader() {
    let mut changes = vec![
        ("labels", "random"),
        ("subset_profile", "1,1,1"),
        ("orbit", "cubic"),
        ("beta", "56"),
        ("lambda", "35"),
        ("prune", "true"),
        ("encoding", "line3"),
    ];
    let original = produced(&changes, 20261026);
    changes.last_mut().unwrap().1 = "line_chart3";
    let chart = produced(&changes, 20261026);
    check(&original);
    check(&chart);
    assert_eq!(
        original["detail"]["attempts"][0]["base"],
        chart["detail"]["attempts"][0]["base"]
    );
    assert_eq!(
        original["detail"]["attempts"][0]["algebra"]["coordinate_sets"],
        chart["detail"]["attempts"][0]["algebra"]["coordinate_sets"]
    );
    assert_eq!(
        original["detail"]["attempts"][0]["system"]["variables"]
            .as_array()
            .unwrap()
            .len(),
        3
    );
    assert_eq!(
        chart["detail"]["attempts"][0]["system"]["variables"]
            .as_array()
            .unwrap()
            .len(),
        2
    );
}
#[test]
fn growth_keeps_coverage_chain_and_rejects_partial_old_certificate() {
    let r = produced(
        &[
            ("subset_profile", "1,1,1"),
            ("growth", "frontier"),
            ("growth_ceiling", "3"),
            ("guess", "1"),
        ],
        17,
    );
    check(&r);
    let full = produced(
        &[
            ("subset_profile", "1,1,1"),
            ("growth", "retained_full"),
            ("growth_ceiling", "3"),
            ("guess", "1"),
        ],
        17,
    );
    check(&full);
    assert_eq!(
        r["detail"]["attempts"].as_array().unwrap().len(),
        full["detail"]["attempts"].as_array().unwrap().len()
    );
    for (frontier, whole) in r["detail"]["attempts"]
        .as_array()
        .unwrap()
        .iter()
        .zip(full["detail"]["attempts"].as_array().unwrap())
    {
        assert_eq!(
            frontier["base"], whole["base"],
            "one-factor growth control must retain identical points, labels and cumulative draws"
        );
        assert!(whole["lead"]["skipped_parts"].is_null());
    }
    let rows = r["detail"]["attempts"].as_array().unwrap();
    let i = rows
        .iter()
        .position(|a| !a["lead"]["skipped_parts"].is_null())
        .expect("fixture exercises retained complete coverage");
    assert!(i > 0);
    let mut bad = r.clone();
    bad["detail"]["attempts"][i - 1]["algebra"]["complete"] = json!(false);
    assert!(audit::mathematical(&bad, audit::MAX_ORACLE_CANDIDATES).is_err());
}
#[test]
fn plan_and_native_counter_tampering_are_rejected() {
    let r = produced(&[("guess", "1"), ("subset_profile", "1,3,2")], 20261026);
    check(&r);
    let mut bad = r.clone();
    bad["detail"]["attempts"][0]["lead"]["cells"][0]["fixed_coordinates"] = json!([999]);
    assert!(audit::mathematical(&bad, audit::MAX_ORACLE_CANDIDATES).is_err());
    let mut bad = r.clone();
    bad["counters"]["aps_trace_certification_term_steps_uncharged"] = json!(u64::MAX);
    assert!(audit::metrics(&bad).is_err());
    let mut bad = r;
    bad["detail"]["costs_attested"] = json!(true);
    assert!(audit::metrics(&bad).is_err());
}
#[test]
fn first_informative_chart_retains_partial_coverage() {
    let r = produced(
        &[("encoding", "line_chart3"), ("stop", "first_informative")],
        20261026,
    );
    check(&r);
    assert!(r["detail"]["attempts"]
        .as_array()
        .unwrap()
        .iter()
        .any(|a| a["algebra"]["stopped_early"] == true));
}
