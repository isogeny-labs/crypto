//! Parser and independent output replay only; no full solver/campaign calls.
#[allow(dead_code)]
#[path = "../src/incumbent_main.rs"]
mod driver;

use prime31_aux_matched::collector::PublicTarget;
use prime31_aux_matched::incumbent_control::{
    FastIcConfig, FAST_IC_SOURCE_REVISION, FAST_IC_SOURCE_SHA256,
};
use prime31_aux_matched::replay::{Curve, Point};
use serde_json::{json, Value};
use std::fs;
use std::sync::atomic::{AtomicU64, Ordering};
static NEXT: AtomicU64 = AtomicU64::new(0);

#[test]
fn complete_ic_control_inputs_are_bounded_and_unselected() {
    let root = std::env::temp_dir().join(format!(
        "p31-incumbent-input-{}-{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    driver::panel(root.to_str().unwrap()).unwrap();
    for (label, expected) in [("screening", 3), ("recovery-candidates", 15)] {
        let index: Value =
            serde_json::from_slice(&fs::read(root.join(label).join("index.json")).unwrap())
                .unwrap();
        assert_eq!(index["prepared_cells"], expected);
        assert_eq!(index["attempted"], 0);
        assert_eq!(index["admission_granted"], false);
        for entry in index["cells"].as_array().unwrap() {
            let p = root.join(label).join(entry["file"].as_str().unwrap());
            driver::read_cell(p.to_str().unwrap()).unwrap();
            let mut bad: Value = serde_json::from_slice(&fs::read(&p).unwrap()).unwrap();
            bad["config"]["max_target_walk"] = json!(0);
            let bad_path = root.join("bad.json");
            fs::write(&bad_path, bad.to_string()).unwrap();
            assert!(driver::read_cell(bad_path.to_str().unwrap()).is_err());
            bad["config"]["max_target_walk"] = json!(65536);
            bad["public_target"]["target"]["x"] = json!(0);
            fs::write(&bad_path, bad.to_string()).unwrap();
            assert!(driver::read_cell(bad_path.to_str().unwrap()).is_err());
        }
    }
    assert!(driver::panel(root.to_str().unwrap()).is_err());
    fs::remove_dir_all(root).unwrap();
}

fn records() -> Vec<Value> {
    // A bounded independent replay fixture, intentionally not the Prime31
    // campaign input. No target logarithm is passed to the producer API.
    let e = Curve { p: 19, a: 0, b: 2 };
    let g = Point::affine(4, 3);
    let target = PublicTarget {
        p: 19,
        a: 0,
        b: 2,
        order: 13,
        generator: g,
        target: e.scalar(g, 7).unwrap(),
    };
    let config = FastIcConfig {
        fb_size: Some(3),
        extra_relations: 16,
        max_target_walk: 65536,
        seed: 4188,
    };
    vec![
        json!({"event":"incumbent_start","cell":{
            "schema":"prime31-complete-ic-incumbent.cell/v1","cell_id":"synthetic-replay-only",
            "public_target":target,"config":config,"stage":"screening","round":0,
            "execution_seconds":120,"memory_mib":4096}}),
        json!({"event":"incumbent_summary","recovered":true,"result":{
            "schema":"prime31-complete-ic-incumbent/v1","backend":"prime_fast.ic_solve_fast",
            "source_revision":FAST_IC_SOURCE_REVISION,"source_sha256":FAST_IC_SOURCE_SHA256,
            "config":config,"status":"recovered","recovered_scalar":7,"independent_replay":true}}),
    ]
}

#[test]
fn independent_baseline_replay_rejects_wrong_scalar_and_changed_input() {
    let original = records();
    let checked = driver::audit_records(&original).unwrap();
    assert_eq!(checked["complete"], true);
    assert_eq!(checked["performance_independently_validated"], false);
    assert_eq!(checked["execution_source_independently_verified"], false);
    assert_eq!(checked["frozen_input_scope_verified"], false);
    assert_eq!(checked["input_file_hash_verified"], false);
    assert_eq!(checked["source_manifest_hash_verified"], false);
    assert_eq!(checked["timing_admission_verified"], false);
    for (index, pointer, value) in [
        (1, "/result/recovered_scalar", json!(8)),
        (1, "/result/recovered_scalar", json!(20)),
        (1, "/result/source_sha256", json!("wrong")),
        (1, "/result/config/seed", json!(1)),
        (1, "/result/independent_replay", json!(false)),
        (1, "/result/status", json!("unrecovered_unspecified")),
        (0, "/cell/public_target/target/x", json!(0)),
    ] {
        let mut bad = original.clone();
        *bad[index].pointer_mut(pointer).unwrap() = value;
        assert!(driver::audit_records(&bad).is_err(), "accepted {pointer}");
    }
    let partial = driver::audit_records(&original[..1]).unwrap();
    assert_eq!(partial["complete"], false);
    assert_eq!(partial["stream_complete"], false);
    let mut nonrecovery = original.clone();
    nonrecovery[1]["result"]["recovered_scalar"] = Value::Null;
    nonrecovery[1]["result"]["status"] = json!("unrecovered_unspecified");
    nonrecovery[1]["result"]["independent_replay"] = json!(false);
    nonrecovery[1]["recovered"] = json!(false);
    let checked = driver::audit_records(&nonrecovery).unwrap();
    assert_eq!(checked["complete"], false);
    assert_eq!(checked["stream_complete"], true);
    let mut duplicate = original.clone();
    duplicate.push(original[1].clone());
    assert!(driver::audit_records(&duplicate).is_err());
}
