use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::fs::OpenOptions;
use std::io::Write;
fn sources() -> Value {
    let inputs: [(&str, &[u8]); 8] = [
        (
            "tools/torus-quotient-query/Cargo.toml",
            include_bytes!("../Cargo.toml"),
        ),
        (
            "tools/torus-quotient-query/Cargo.lock",
            include_bytes!("../Cargo.lock"),
        ),
        (
            "tools/torus-quotient-query/src/lib.rs",
            include_bytes!("lib.rs"),
        ),
        (
            "tools/torus-quotient-query/src/arithmetic.rs",
            include_bytes!("arithmetic.rs"),
        ),
        (
            "tools/torus-quotient-query/src/query.rs",
            include_bytes!("query.rs"),
        ),
        (
            "tools/torus-quotient-query/src/controls.rs",
            include_bytes!("controls.rs"),
        ),
        (
            "tools/torus-quotient-query/src/main.rs",
            include_bytes!("main.rs"),
        ),
        (
            "tools/torus-quotient-query/run-control.sh",
            include_bytes!("../run-control.sh"),
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
            json!({"schema":"torus-quotient-query.qualifier/v1","sources":sources(),"scope":"supplied section support/extraction only"})
        );
        return Ok(());
    }
    let [mode, path] = args.as_slice() else {
        return Err("usage: --qualify | --controls NEW_JSONL | --transport NEW_JSONL".into());
    };
    if mode != "--controls" && mode != "--transport" {
        return Err("unknown mode".into());
    }
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|e| e.to_string())?;
    let mut count = 0;
    let mut emit = |value: Value| -> Result<(), String> {
        serde_json::to_writer(&mut file, &value).map_err(|e| e.to_string())?;
        file.write_all(b"\n").map_err(|e| e.to_string())?;
        file.flush().map_err(|e| e.to_string())?;
        count += 1;
        Ok(())
    };
    let passed = if mode == "--transport" {
        emit(json!({"case":"transport-only","passed":true}))?;
        true
    } else {
        torus_quotient_query::controls::run(&mut emit)?
    };
    emit(json!({"case":"completion","passed":passed}))?;
    println!("{}", json!({"passed":passed,"records":count}));
    if passed {
        Ok(())
    } else {
        Err("one or more fixed controls failed; inspect retained per-case records".into())
    }
}
