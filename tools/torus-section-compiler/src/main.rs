use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{fs::OpenOptions, io::Write, path::Path};
fn sources() -> Value {
    let inputs: [(&str, &[u8]); 10] = [
        (
            "tools/torus-section-compiler/Cargo.toml",
            include_bytes!("../Cargo.toml"),
        ),
        (
            "tools/torus-section-compiler/Cargo.lock",
            include_bytes!("../Cargo.lock"),
        ),
        (
            "tools/torus-section-compiler/.gitignore",
            include_bytes!("../.gitignore"),
        ),
        (
            "tools/torus-section-compiler/FIXTURES.json",
            include_bytes!("../FIXTURES.json"),
        ),
        (
            "tools/torus-section-compiler/src/main.rs",
            include_bytes!("main.rs"),
        ),
        (
            "tools/torus-section-compiler/src/lib.rs",
            include_bytes!("lib.rs"),
        ),
        (
            "tools/torus-section-compiler/src/gates.rs",
            include_bytes!("gates.rs"),
        ),
        (
            "tools/torus-section-compiler/src/arithmetic.rs",
            include_bytes!("arithmetic.rs"),
        ),
        (
            "tools/torus-section-compiler/src/controls.rs",
            include_bytes!("controls.rs"),
        ),
        (
            "tools/torus-section-compiler/run-control.sh",
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
            json!({"schema":"section-compiler.qualifier/v1","sources":sources(),"scope":"known-coefficient circuit extension/projection controls; not root discovery"})
        );
        return Ok(());
    }
    let [mode, path, dir] = args.as_slice() else {
        return Err(
            "usage: --qualify | --controls NEW_JSONL NEW_EXPORT_DIR | --transport NEW_JSONL UNUSED"
                .into(),
        );
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
        torus_section_compiler::controls::run(&mut emit, Path::new(dir))?
    };
    emit(json!({"case":"completion","passed":passed}))?;
    println!("{}", json!({"records":count,"passed":passed}));
    if passed {
        Ok(())
    } else {
        Err("one or more fixed controls failed".into())
    }
}
