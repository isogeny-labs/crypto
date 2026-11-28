use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{fs::OpenOptions, io::Write};
fn sources() -> Value {
    let inputs: [(&str, &[u8]); 8] = [
        (
            "tools/torus-section-radical/Cargo.toml",
            include_bytes!("../Cargo.toml"),
        ),
        (
            "tools/torus-section-radical/Cargo.lock",
            include_bytes!("../Cargo.lock"),
        ),
        (
            "tools/torus-section-radical/.gitignore",
            include_bytes!("../.gitignore"),
        ),
        (
            "tools/torus-section-radical/src/lib.rs",
            include_bytes!("lib.rs"),
        ),
        (
            "tools/torus-section-radical/src/main.rs",
            include_bytes!("main.rs"),
        ),
        (
            "tools/torus-section-radical/src/symbolic.rs",
            include_bytes!("symbolic.rs"),
        ),
        (
            "tools/torus-section-radical/src/controls.rs",
            include_bytes!("controls.rs"),
        ),
        (
            "tools/torus-section-radical/run-control.sh",
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
            json!({"schema":"torus-section-radical.qualifier/v1","sources":sources(),"scope":"fixed support equivalence/degree controls, not coefficient discovery"})
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
        .create_new(true)
        .write(true)
        .open(path)
        .map_err(|e| e.to_string())?;
    let mut count = 0;
    let mut emit = |v: Value| -> Result<(), String> {
        serde_json::to_writer(&mut file, &v).map_err(|e| e.to_string())?;
        file.write_all(b"\n").map_err(|e| e.to_string())?;
        file.flush().map_err(|e| e.to_string())?;
        count += 1;
        Ok(())
    };
    let passed = if mode == "--transport" {
        emit(json!({"case":"transport-only","passed":true}))?;
        true
    } else {
        torus_section_radical::controls::run(&mut emit)?
    };
    emit(json!({"case":"completion","passed":passed}))?;
    println!("{}", json!({"passed":passed,"records":count}));
    if passed {
        Ok(())
    } else {
        Err("one or more fixed controls failed".into())
    }
}
