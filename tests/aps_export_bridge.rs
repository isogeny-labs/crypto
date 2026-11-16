//! Development integration between the actual APS producer and separate reader.
//! No measured sample or wall-clock qualification is produced by these tests.
#[path = "../tools/aps-export/src/value.rs"]
mod value;
pub use value::*;
#[path = "../tools/aps-export/src/audit.rs"]
mod audit;

use crypto_lib::cryptanalysis::ecbench::exact_scalar::ExactScalar;
use crypto_lib::cryptanalysis::{
    ecbench::{
        methods::{self, MethodSpec},
        workload::{CurveSpec, Instance},
    },
    ic_boundary::{CountedGroup, GroupOps},
};
use serde_json::{json, Value};
use std::collections::BTreeMap;

fn produced(encoding: &str, orbit: bool, prune: bool, first: bool) -> Value {
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
        ("encoding", encoding),
        ("roots", "exact"),
        (
            "stop",
            if first {
                "first_informative"
            } else {
                "exhaustive"
            },
        ),
        ("labels", if orbit { "random" } else { "guaranteed" }),
        ("subset_size", if orbit { "1" } else { "2" }),
        ("attempts", "32"),
        ("degree", "24"),
        ("budget_ms", "10000"),
        ("sampling_draws", "1000"),
        ("orbit", if orbit { "cubic" } else { "none" }),
        ("beta", if orbit { "56" } else { "0" }),
        ("lambda", if orbit { "35" } else { "0" }),
        ("prune", if prune { "true" } else { "false" }),
    ]
    .into_iter()
    .map(|(a, b)| (a.into(), b.into()))
    .collect();
    let method = methods::resolve(&MethodSpec {
        id: "ic.aps_v1".into(),
        params: std::mem::take(&mut params),
    })
    .unwrap();
    let report = methods::solve(&method, &instance, &facts, &target, 20261026).unwrap();
    assert_eq!(
        report.recovered,
        Some(ExactScalar::from(u128::from(k))),
        "{}",
        report.detail
    );
    let record = json!({"schema":"ecbench.record/v1","record_id":"","workload":{"curve_spec":spec,"curve":facts,"target":target},"method":method,"algorithm_seed":"20261026","outcome":{"status":"verified","recovered":k.to_string()},"detail":report.detail,"phases":report.phases,"online":report.online,"counters":report.counters,"time":{"solve_wall_ns":report.solve_wall_ns,"method_call_wall_ns":null}});
    // This literal seal is an integrity check, not an execution certificate.
    let mut record = record;
    let bytes = serde_json::to_vec(&record).unwrap();
    record["record_id"] = json!(format!("ECR1h{}", &hash(&bytes)[..12]));
    audit::record_line(&serde_json::to_string(&record).unwrap()).unwrap()
}

#[test]
fn six_actual_producer_encodings_pass_separate_arithmetic_and_clock_checks() {
    for enc in [
        "partitioned3",
        "unordered3",
        "line3",
        "expanded4",
        "split4",
        "block_symmetric4",
    ] {
        let r = produced(enc, false, false, false);
        audit::metrics(&r).unwrap();
        let checked = audit::mathematical(&r, audit::MAX_ORACLE_CANDIDATES).unwrap();
        assert!(
            checked["witnesses_checked"].as_u64().unwrap() > 0,
            "{enc}: {checked}"
        );
    }
}
#[test]
fn actual_cubic_orbit_labels_and_horizontal_pruning_are_checked() {
    for prune in [false, true] {
        let r = produced("line3", true, prune, false);
        audit::metrics(&r).unwrap();
        audit::mathematical(&r, audit::MAX_ORACLE_CANDIDATES).unwrap();
        let mut bad = r;
        bad["method"]["params"]["lambda"] = json!("34");
        assert!(audit::mathematical(&bad, audit::MAX_ORACLE_CANDIDATES).is_err());
    }
}
#[test]
fn actual_first_informative_output_keeps_its_partial_status() {
    let r = produced("unordered3", false, false, true);
    let checked = audit::mathematical(&r, audit::MAX_ORACLE_CANDIDATES).unwrap();
    assert!(checked["witnesses_checked"].as_u64().unwrap() > 0);
    assert!(checked["partial_attempts_retained"].as_u64().unwrap() > 0);
}
