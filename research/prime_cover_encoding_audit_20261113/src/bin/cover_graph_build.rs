//! Paired polynomial export only. No F4/GB solver is linked or executed.
#[path = "../graph_builder.rs"]
mod graph_builder;
#[allow(dead_code)]
#[path = "../../../../src/hash/sha256.rs"]
mod hash_sha256;
#[path = "../polynomial.rs"]
mod polynomial;
use serde_json::json;
use std::{fs, path::Path};
fn digest(bytes: &[u8]) -> String {
    hash_sha256::sha256(bytes)
        .iter()
        .map(|x| format!("{x:02x}"))
        .collect()
}
fn write(path: &Path, bytes: &[u8]) -> Result<String, String> {
    fs::write(path, bytes).map_err(|e| e.to_string())?;
    Ok(digest(bytes))
}
fn main() -> Result<(), String> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 3 {
        return Err("usage: cover-graph-build SAVED_JSONL OUTPUT_DIRECTORY".into());
    }
    let bytes = fs::read(&args[1]).map_err(|e| e.to_string())?;
    let input_sha = digest(&bytes);
    if ![
        "88f5ad4b1d972c3d78f2d64cad5934788ff93e7f203c44528c833a185ac7ba7e",
        "f9eeef35b50b7dd5e0f282f1c700e1d3f762f21e8cc0c31f074ae761e31bf895",
    ]
    .contains(&input_sha.as_str())
    {
        return Err("input is not one of the frozen saved cases".into());
    }
    let case = graph_builder::Case::parse(&bytes)?;
    let direct = graph_builder::build(&case, false);
    let graph = graph_builder::build(&case, true);
    graph_builder::verify_saved_dimensions(&case, &direct)?;
    graph_builder::verify_pair(&case, &direct, &graph)?;
    let out = Path::new(&args[2]);
    // Exclusive creation: never overwrite a previous export or partial attempt.
    fs::create_dir(out).map_err(|e| format!("cannot create fresh output directory: {e}"))?;
    let mut artifacts = Vec::new();
    for (name, system) in [("direct", &direct), ("graph", &graph)] {
        let data = serde_json::to_vec_pretty(&system.json(&case, &input_sha))
            .map_err(|e| e.to_string())?;
        let json_sha = write(&out.join(format!("{name}.json")), &data)?;
        let text = system.msolve_text(case.p);
        let text_sha = write(&out.join(format!("{name}.ms")), text.as_bytes())?;
        artifacts.push(json!({"representation":name,"n_vars":system.n,"n_equations":system.equations.len(),
            "max_input_degree":system.degrees().into_iter().max(),"json_sha256":json_sha,"msolve_text_sha256":text_sha}));
    }
    let manifest = json!({"schema":"cover.paired-builder/v1","input_sha256":input_sha,"case":case.metadata(),
        "artifacts":artifacts,"coefficient_projection_check":"completed for every equation",
        "solver_executed":false,"solver_consumer_validation":"not performed",
        "scope":"native construction/substitution correctness only; no speed or asymptotic inference",
        "standing":"unsigned producer evidence"});
    write(
        &out.join("manifest.json"),
        &serde_json::to_vec_pretty(&manifest).map_err(|e| e.to_string())?,
    )?;
    for item in &artifacts {
        println!(
            "{}: variables={}, equations={}, max_input_degree={}",
            item["representation"], item["n_vars"], item["n_equations"], item["max_input_degree"]
        );
    }
    Ok(())
}
