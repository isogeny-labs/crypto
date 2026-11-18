//! Copy already prepared native values into the dashboard component. No pooling.
use serde_json::{json, Value};
use std::{fs, path::Path};
fn main() {
    let root = std::env::args().nth(1).expect("component STUDY_DIRECTORY");
    let root = Path::new(&root);
    let read = |name: &str| {
        serde_json::from_str::<Value>(&fs::read_to_string(root.join(name)).unwrap()).unwrap()
    };
    let initial = read("initial-summary.json");
    let optimized = read("optimization-summary.json");
    let final_run = read("final-summary.json");
    let current = read("policy-comparison-summary.json");
    let policy = read("policy-summary.json");
    let serial = root
        .join("serial-clear-summary.json")
        .exists()
        .then(|| read("serial-clear-summary.json"));
    let guarded = root
        .join("serial-guard-summary.json")
        .exists()
        .then(|| read("serial-guard-summary.json"));
    assert_eq!(final_run["final_run"], true);
    assert_eq!(current["policy_run"], true);
    let rows = current["costs"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|r| r["n"] == 23)
        .cloned()
        .collect::<Vec<_>>();
    assert!(rows.iter().all(|r| r["cold"]["n"] == 5));
    let mut improvements = Vec::new();
    for row in initial["matrices"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|r| r["n"] == 23 && r["seed"] == 230)
    {
        improvements.push(json!({"change":format!("{} single-write",row["arm"].as_str().unwrap()),"target":"planted",
            "before":row["reference"],"after":row["candidate"],"comparison":row["comparison"],"source":"initial-summary.json"}));
    }
    for row in final_run["batching"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|r| r["n"] == 23)
    {
        improvements.push(json!({"change":"Semaev batched cold setup", "target":if row["random"]==1 {"generator"}else{"planted"},
            "before":row["before"],"after":row["after"],"comparison":row["comparison"],"source":"final-summary.json"}));
    }
    for row in policy["changes"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|r| r["seed"] == 230 && r["arm"] == "decomp-generalized")
    {
        improvements.push(json!({"change":format!("n{} generalized typed entry",row["n"]),"target":if row["random"]==1 {"generator"}else{"planted"},
            "before":row["before"],"after":row["typed"],"comparison":row["before_over_typed"],"source":"policy-summary.json"}));
    }
    if let Some(serial) = &serial {
        for row in serial["matrices"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|r| r["workers"] == 4 && r["n"] == 17 && r["arm"] == "matrix-f4")
        {
            improvements.push(json!({"change":"initial n17 F4 four-worker control", "target":"planted",
                "before":row["before"],"after":row["after"],"comparison":row["comparison"],"source":"serial-clear-summary.json"}));
        }
    }
    if let Some(guarded) = &guarded {
        for row in guarded["matrices"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|r| r["workers"] == 1 && r["n"] == 23 && r["seed"] == 230)
        {
            improvements.push(json!({"change":format!("{} guarded serial clearing",row["arm"].as_str().unwrap()),"target":"planted",
                "before":row["before"],"after":row["after"],"comparison":row["comparison"],"source":"serial-guard-summary.json"}));
            improvements.push(json!({"change":format!("{} legacy to guarded candidate",row["arm"].as_str().unwrap()),"target":"planted",
                "before":row["legacy"],"after":row["after"],"comparison":row["legacy_over_candidate"],"source":"serial-guard-summary.json"}));
        }
    }
    let changes = optimized["changes"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|r| r["n"] == 23 && r["arm"].as_str().unwrap().contains("generalized"))
        .cloned()
        .collect::<Vec<_>>();
    println!("{}",serde_json::to_string_pretty(&json!({"schema":"boolean-native-component/v1","scope":"Native matched solver boundaries, one worker, dedicated x86-64 VMs; exact outputs; no cross-host pooling or whole-IC/rho ratio",
        "study":"research/f4_f5_boolean_runtime_20261019/","report":"research/f4_f5_boolean_runtime_20261019/REPORT.pdf",
        "curves":read("fixture-identities.json"),"initial_matrices":initial["matrices"],"optimized_matrices":optimized["matrices"],"holdouts":optimized["holdouts"],
        "generalized_changes":changes,"semaev_changes":final_run["batching"],"policy_confirmation":policy,"serial_clear_initial":serial,"serial_guard":guarded,"costs_n23":rows,"improvements":improvements,
        "target_labels":"random=true in raw records means the unplanted generator control, not a random sample",
        "instruction_component":"research/f4_f5_boolean_perf_20261018/component.json",
        "defaults":"Single-write, SIMD tables, serial row clearing and batched Semaev setup remain opt-in; extra SIMD tables were neutral. The first serial guard regressed one four-worker control; the refined guard requires explicit RAYON_NUM_THREADS=1 and a current one-worker pool. Generalized auto uses scalar below22 variables and wide Gray when available at22 or more. Typed input avoids JSON; exact counts and original-root verification remain enforced. The18-arm costs precede serial-clear experiments and do not claim their complete-root speedup."})).unwrap());
}
