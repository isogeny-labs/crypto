//! Bind corrected scheduling into new method identities; preserve old specs.
use serde_json::Value;
use std::{fs, path::Path};
fn main() {
    let root = std::env::args().nth(1).expect("adaptive-specs STUDY");
    let root = Path::new(&root);
    for n in [17, 23] {
        let original: Value = serde_json::from_str(
            &fs::read_to_string(root.join(format!("ecbench-n{n}.json"))).unwrap(),
        )
        .unwrap();
        let mut full = original.clone();
        full["label"] = format!("Adaptive F4/F5 complete cold recovery n{n}").into();
        for arm in full["arms"].as_array_mut().unwrap() {
            if matches!(arm["name"].as_str(), Some("f4-tail" | "f5-tail")) {
                let oracle = arm["method"]["params"]["oracle"].as_str().unwrap().replace(
                    "matrix_runtime=quadratic-tail",
                    "matrix_runtime=adaptive-quadratic-tail",
                );
                arm["method"]["params"]["oracle"] = oracle.into();
            }
        }
        assert_eq!(original["workloads"], full["workloads"]);
        assert_eq!(original["measurement"], full["measurement"]);
        fs::write(
            root.join(format!("ecbench-adaptive-n{n}.json")),
            serde_json::to_string_pretty(&full).unwrap() + "\n",
        )
        .unwrap();
        let mut screen = full;
        screen["label"] = format!("Versioned adaptive policy n{n}: local work/correctness").into();
        screen["workloads"]["targets_per_curve"] = 1.into();
        screen["measurement"]["rounds"] = 1.into();
        screen["measurement"]["warmup"] = 1.into();
        screen["measurement"]["isolation_required"] = "L0".into();
        screen["arms"].as_array_mut().unwrap().retain(|a| {
            matches!(
                a["name"].as_str(),
                Some("f4-baseline" | "f4-tail" | "f5-baseline" | "f5-tail")
            )
        });
        fs::write(
            root.join(format!("versioned-screen-n{n}.json")),
            serde_json::to_string_pretty(&screen).unwrap() + "\n",
        )
        .unwrap();
    }
}
