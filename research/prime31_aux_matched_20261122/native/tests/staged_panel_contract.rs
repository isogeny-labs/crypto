//! Parser and prospective-input contracts only. No run command is invoked.
#[allow(dead_code)]
#[path = "../src/choice_main.rs"]
mod driver;

use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT: AtomicU64 = AtomicU64::new(0);
struct Scratch(PathBuf);
impl Scratch {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "p31-stage-contract-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&path).unwrap();
        Self(path)
    }
}
impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
fn read(path: &Path) -> Value {
    serde_json::from_slice(&fs::read(path).unwrap()).unwrap()
}
fn contract_key(mut v: Value) -> String {
    for key in ["cell_id", "query_seed", "rho_seed", "round", "stage"] {
        v.as_object_mut().unwrap().remove(key);
    }
    v["collection"]["max_queries"] = json!(0);
    serde_json::to_string(&v).unwrap()
}

#[test]
fn legacy_panel_stays_byte_equal_and_v1_rejects_new_semantics() {
    let scratch = Scratch::new();
    let generated = scratch.0.join("legacy");
    driver::panel(generated.to_str().unwrap()).unwrap();
    let frozen = Path::new(env!("CARGO_MANIFEST_DIR")).join("../isogeny-choice/panel");
    let index = read(&frozen.join("index.json"));
    assert_eq!(
        fs::read(generated.join("index.json")).unwrap(),
        fs::read(frozen.join("index.json")).unwrap()
    );
    for entry in index["cells"].as_array().unwrap() {
        let name = entry["file"].as_str().unwrap();
        assert_eq!(
            fs::read(generated.join(name)).unwrap(),
            fs::read(frozen.join(name)).unwrap()
        );
    }
    let first = index["cells"][0]["file"].as_str().unwrap();
    let mut v = read(&generated.join(first));
    v["stage"] = json!("screening");
    let altered = scratch.0.join("altered.json");
    fs::write(&altered, v.to_string()).unwrap();
    assert!(driver::read_cell(altered.to_str().unwrap()).is_err());
}

#[test]
fn stages_have_paired_streams_and_explicit_unselected_recovery_pool() {
    let scratch = Scratch::new();
    let root = scratch.0.join("staged");
    driver::staged_panel(root.to_str().unwrap()).unwrap();
    let screen = read(&root.join("screening/index.json"));
    let recovery = read(&root.join("recovery-candidates/index.json"));
    assert_eq!(screen["prepared_cells"], 77);
    assert_eq!(recovery["prepared_cells"], 385);
    assert_eq!(screen["attempted"], 0);
    assert_eq!(recovery["selected_for_execution"], 0);
    let expected: BTreeSet<_> = screen["cells"]
        .as_array()
        .unwrap()
        .iter()
        .map(|entry| {
            let v = read(&root.join("screening").join(entry["file"].as_str().unwrap()));
            assert_eq!(v["collection"]["max_queries"], 64);
            contract_key(v)
        })
        .collect();
    assert_eq!(expected.len(), 77);
    let mut cohorts = BTreeMap::<u64, BTreeSet<String>>::new();
    for entry in recovery["cells"].as_array().unwrap() {
        let path = root
            .join("recovery-candidates")
            .join(entry["file"].as_str().unwrap());
        driver::read_cell(path.to_str().unwrap()).unwrap();
        let v = read(&path);
        assert_eq!(v["collection"]["max_queries"], 65_536);
        assert_eq!(v["collection"]["max_seconds"], 110);
        assert_eq!(v["execution_seconds"], 120);
        assert_eq!(v["memory_mib"], 4096);
        assert_eq!(v["collection"]["oracle"]["stop_after_first"], true);
        assert_eq!(v["coset_scalar"], 1);
        cohorts
            .entry(v["query_seed"].as_u64().unwrap())
            .or_default()
            .insert(contract_key(v));
    }
    assert_eq!(cohorts.len(), 5);
    for cohort in cohorts.values() {
        assert_eq!(cohort, &expected);
    }
}

#[test]
fn staged_parser_rejects_changed_caps_missing_reference_and_backend_substitution() {
    let scratch = Scratch::new();
    let root = scratch.0.join("staged");
    driver::staged_panel(root.to_str().unwrap()).unwrap();
    let index = read(&root.join("recovery-candidates/index.json"));
    let plain = index["cells"]
        .as_array()
        .unwrap()
        .iter()
        .find(|v| v["plain_control"] == true && v["oracle_backend"] == "polynomial")
        .unwrap();
    let original = read(
        &root
            .join("recovery-candidates")
            .join(plain["file"].as_str().unwrap()),
    );
    let path = scratch.0.join("mutant.json");
    let rejected = |v: Value| {
        fs::write(&path, v.to_string()).unwrap();
        assert!(driver::read_cell(path.to_str().unwrap()).is_err());
    };
    let mut v = original.clone();
    v["collection"]["max_queries"] = json!(64);
    rejected(v);
    let mut v = original.clone();
    v["collection"]["max_queries"] = json!(65_537);
    rejected(v);
    let mut v = original.clone();
    v["collection"]["max_seconds"] = json!(111);
    rejected(v);
    let mut v = original.clone();
    v["collection"]["oracle"]["timeout_ms"] = json!(2001);
    rejected(v);
    let mut v = original.clone();
    v["collection"]["oracle"]["stop_after_first"] = json!(false);
    rejected(v);
    let mut v = original.clone();
    v.as_object_mut().unwrap().remove("stage");
    rejected(v);
    let mut v = original.clone();
    v.as_object_mut().unwrap().remove("plain_reference");
    rejected(v);
    let mut v = original.clone();
    v["plain_reference"]["columns"] = json!(1);
    rejected(v);
    let mut v = original.clone();
    v["coset_scalar"] = json!(3);
    rejected(v);
    let mut v = original.clone();
    v["oracle_backend"] = json!("pair_table");
    v["presentation"] = json!("tower");
    rejected(v);
    let mut v = original;
    v["oracle_backend"] = json!("pair_table");
    v["method"] = json!("rho_fast");
    rejected(v);
}
