use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::fs::OpenOptions;
use std::io::Write;

fn sources() -> Value {
    let inputs: [(&str, &[u8]); 7] = [
        (
            "tools/torus-section-operator/Cargo.toml",
            include_bytes!("../Cargo.toml"),
        ),
        (
            "tools/torus-section-operator/Cargo.lock",
            include_bytes!("../Cargo.lock"),
        ),
        (
            "tools/torus-section-operator/src/lib.rs",
            include_bytes!("lib.rs"),
        ),
        (
            "tools/torus-section-operator/src/main.rs",
            include_bytes!("main.rs"),
        ),
        (
            "tools/torus-section-operator/src/controls.rs",
            include_bytes!("controls.rs"),
        ),
        (
            "tools/torus-s4/Cargo.toml",
            include_bytes!("../../torus-s4/Cargo.toml"),
        ),
        (
            "tools/torus-s4/src/lib.rs",
            include_bytes!("../../torus-s4/src/lib.rs"),
        ),
    ];
    json!(inputs
        .iter()
        .map(|(p, b)| json!({"path":p,"sha256":format!("{:x}",Sha256::digest(b))}))
        .collect::<Vec<_>>())
}

fn main() -> Result<(), String> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args == ["--qualify"] {
        println!(
            "{}",
            json!({"schema":"torus-section-operator.qualifier/v1","sources":sources(),"scope":"candidate predicate; no root discovery"})
        );
        return Ok(());
    }
    let [mode, path] = args.as_slice() else {
        return Err("usage: --qualify | --controls NEW_OUTPUT".into());
    };
    if mode != "--controls" {
        return Err("unknown mode".into());
    }
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|e| e.to_string())?;
    let value = match torus_section_operator::controls::run() {
        Ok(v) => v,
        Err(e) => json!({"schema":"torus-section-operator.controls/v1","passed":false,"fatal":e}),
    };
    let bytes = serde_json::to_vec_pretty(&value).map_err(|e| e.to_string())?;
    if bytes.len() > 16 * 1024 * 1024 {
        return Err("development output cap; not an absence result".into());
    }
    file.write_all(&bytes).map_err(|e| e.to_string())?;
    file.write_all(b"\n").map_err(|e| e.to_string())?;
    println!(
        "{}",
        json!({"passed":value["passed"],"matched":value["matched"],"positive_sections":value["positive_sections"],"output_bytes":bytes.len()+1})
    );
    if value["passed"] != true {
        return Err("fixed controls did not all pass".into());
    }
    Ok(())
}
