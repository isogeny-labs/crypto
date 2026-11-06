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
    let spec = CurveSpec::PrimeExplicit {
        p: 101,
        a: 59,
        b: 6,
        group_order: 107,
        r: 107,
        gx: 78,
        gy: 4,
    };
    let instance = spec.build().unwrap();
    let facts = instance.facts(&spec);
    let Instance::Prime(inst) = &instance else {
        unreachable!()
    };
    let q = inst
        .curve
        .mul(&mut GroupOps::default(), inst.generator_point(), 37);
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
        Some("37".into()),
        "{}",
        report.detail
    );
    let r = json!({"schema":"ecbench.record/v1","record_id":"", "workload":{"curve_spec":spec,"curve":facts,"target":target},
        "method":method,"algorithm_seed":seed.to_string(),"outcome":{"status":"verified","recovered":"37"},
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
