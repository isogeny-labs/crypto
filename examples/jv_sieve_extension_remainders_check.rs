//! Independent copied qualifier; no scientific work is executed locally.
#[path = "../research/extension_ic_graph_20261115/extension_remainder_check.rs"]
mod check;
#[allow(dead_code)]
#[path = "../research/extension_ic_graph_20261115/extension_remainder_driver.rs"]
mod driver;
#[allow(dead_code)]
#[path = "../research/extension_ic_graph_20261115/extension_remainder.rs"]
mod remainder;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args.len() != 2 {
        return Err("usage: CHECKER RUN_DIR NEW_SUMMARY".into());
    }
    let dir = std::path::Path::new(&args[0]);
    let a: driver::Record =
        serde_json::from_slice(&std::fs::read(dir.join("full-smooth.raw.json"))?)?;
    let b: driver::Record =
        serde_json::from_slice(&std::fs::read(dir.join("quadratic-remainders.raw.json"))?)?;
    let plan: serde_json::Value =
        serde_json::from_slice(&std::fs::read(dir.join("runtime-plan.json"))?)?;
    if a.source_revision != plan["source_revision"] || b.source_revision != plan["source_revision"]
    {
        return Err("runtime source mismatch".into());
    }
    for mode in ["full-smooth", "quadratic-remainders"] {
        if std::fs::read_to_string(dir.join(format!("{mode}.exit-code.txt")))?.trim() != "0" {
            return Err("failed scientific worker; originals retained".into());
        }
    }
    let result = check::pair(&a, &b)?;
    let flow: Vec<serde_json::Value> = std::fs::read_to_string(dir.join("flow.jsonl"))?
        .lines()
        .map(serde_json::from_str)
        .collect::<Result<_, _>>()?;
    if flow.len() != 4 {
        return Err("planned arm flow mismatch".into());
    }
    for (i, mode) in ["full-smooth", "quadratic-remainders"].iter().enumerate() {
        if flow[2 * i]["record_type"] != "execution_start"
            || flow[2 * i + 1]["record_type"] != "execution"
            || flow[2 * i]["mode"] != *mode
            || flow[2 * i + 1]["mode"] != *mode
            || flow[2 * i + 1]["exit_code"] != 0
            || flow[2 * i]["source_revision"] != plan["source_revision"]
            || flow[2 * i + 1]["source_revision"] != plan["source_revision"]
        {
            return Err("execution flow binding".into());
        }
    }
    let export = serde_json::json!({"schema":"experiment.export/v1","canonical_experiment_export_schema_available":false,"canonical_schema_conformance_claimed":false,"protocol":plan,"fixture":{"public_only":a.public,"input_sha256":driver::INPUT_SHA},"workers":[a,b],"analysis":result,"flow":flow});
    let exported = std::fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(dir.join("raw-export.json"))?;
    serde_json::to_writer_pretty(exported, &export)?;
    let out = std::fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&args[1])?;
    serde_json::to_writer_pretty(out, &result)?;
    println!("Independent source/group/graph/rank/count qualification passed.");
    Ok(())
}
