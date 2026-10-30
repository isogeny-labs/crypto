//! Administrative narrowing for local work/correctness screening, never timing admission.
use serde_json::Value;
use std::{fs, path::Path};
fn main() {
    let root = std::env::args().nth(1).expect("screen-specs STUDY");
    let root = Path::new(&root);
    for n in [17, 23] {
        let mut spec: Value = serde_json::from_str(
            &fs::read_to_string(root.join(format!("ecbench-n{n}.json"))).unwrap(),
        )
        .unwrap();
        spec["label"] =
            format!("Adaptive degree ladder: n{n} local correctness/work screen").into();
        spec["workloads"]["targets_per_curve"] = 1.into();
        spec["measurement"]["rounds"] = 1.into();
        spec["measurement"]["warmup"] = 1.into();
        spec["measurement"]["isolation_required"] = "L0".into();
        spec["arms"].as_array_mut().unwrap().retain(|arm| {
            matches!(
                arm["name"].as_str(),
                Some("f4-baseline" | "f4-tail" | "f5-baseline" | "f5-tail")
            )
        });
        fs::write(
            root.join(format!("adaptive-screen-n{n}.json")),
            serde_json::to_string_pretty(&spec).unwrap() + "\n",
        )
        .unwrap();
    }
}
