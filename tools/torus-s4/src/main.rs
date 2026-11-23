use std::{env, fs, path::Path};
use torus_s4::{sha256, Case, System};
fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
fn run() -> Result<(), Box<dyn std::error::Error>> {
    let a: Vec<_> = env::args().collect();
    if a.len() != 4 || a[1] != "compile" {
        return Err("usage: torus-s4 compile CASE.json NEW_OUTPUT_DIRECTORY".into());
    }
    let input = fs::read(&a[2])?;
    let case: Case = serde_json::from_slice(&input)?;
    let dir = Path::new(&a[3]);
    fs::create_dir(dir)?;
    fs::write(
        dir.join("compile-pre-call.json"),
        serde_json::to_vec_pretty(
            &serde_json::json!({"schema":"torus-s4.compile-pre-call/v1","input_sha256":sha256(&input),"binary_sha256":sha256(&fs::read(env::current_exe()?)?)}),
        )?,
    )?;
    let s = System::compile(&case)?;
    let system = serde_json::to_vec_pretty(&s.canonical_json())?;
    fs::write(dir.join("system.json"), &system)?;
    fs::write(dir.join("input.sing"), s.singular())?;
    fs::write(
        dir.join("compile-post-call.json"),
        serde_json::to_vec_pretty(
            &serde_json::json!({"schema":"torus-s4.compile-post-call/v1","system_sha256":sha256(&system),"variables":s.variables.len(),"equations":s.equations.len(),"max_degree":s.equations.iter().map(|e|e.degree()).max(),"pcc_standing":"unverified","solved":false}),
        )?,
    )?;
    Ok(())
}
