mod arithmetic;
mod evaluate;
mod recipe;
mod store;
#[cfg(test)]
mod tests;
use crate::{recipe::Config, store::*};
use anyhow::{bail, Context, Result};
use serde_json::{json, Value};
use std::{path::Path, time::Instant};

const HELP: &str = r#"factor-base: deterministic native precomputation factory

  factor-base requirements CONFIG
  factor-base build CONFIG STORE
  factor-base batch CONFIG_ARRAY STORE RECEIPT
  factor-base verify STORE MANIFEST_SHA256
  factor-base evaluate STORE MANIFEST_SHA256 WORKLOAD RECEIPT
  factor-base replay STORE MANIFEST_SHA256 RECEIPT
  factor-base compare RECEIPT [RECEIPT ...]
  factor-base upload-plan STORE MANIFEST_SHA256 PREFIX
  factor-base publish STORE MANIFEST_SHA256 BUCKET PREFIX RECEIPT
  factor-base fetch STORE MANIFEST_SHA256 BUCKET PREFIX

All JSON file arguments are explicit. build/verify/evaluate/batch are offline.
publish uses AWS CLI v2 on the trusted controller, objects first, manifest last.
Every output file is immutable. Use a fresh receipt path for each execution.
Binary fields: degrees 2..571. Prime fields: p > 3, at most 576 bits.
Primality above 64 bits is labelled probable, not certified.
Relation search: inverse-pair-free multisets, arities 2..8, explicit state cap.
Exact leading-form regularity: binary S3 chains, one affine subspace, <=14 variables.
Unions and prime-field regularity retain explicit unsupported status.
"#;
fn emit(v: &Value) -> Result<()> {
    println!("{}", serde_json::to_string_pretty(v)?);
    Ok(())
}
fn save(path: &str, v: &Value) -> Result<()> {
    write_once(Path::new(path), &canonical(v)?)?;
    emit(&json!({"receipt":path,"sha256":digest(&canonical(v)?)}))
}
fn run() -> Result<()> {
    let a = std::env::args().skip(1).collect::<Vec<_>>();
    if a.is_empty() || a[0] == "--help" || a[0] == "help" {
        print!("{HELP}");
        return Ok(());
    }
    match (a[0].as_str(), a.len()) {
        ("requirements", 2) => {
            let c: Config = read_json(Path::new(&a[1]))?;
            let curve = arithmetic::Curve::new(c.curve)?;
            emit(&curve.requirements())
        }
        ("build", 3) => {
            let c: Config = read_json(Path::new(&a[1]))?;
            let t = Instant::now();
            let id = build(&c, Path::new(&a[2]))?;
            emit(
                &json!({"manifest_sha256":id,"manifest_path":manifest_path(Path::new(&a[2]),&id)?,"construction_and_storage_ms":t.elapsed().as_secs_f64()*1000.,"wall_time_qualification":"unqualified_diagnostic"}),
            )
        }
        ("batch", 4) => {
            let configs: Vec<Config> = read_json(Path::new(&a[1]))?;
            anyhow::ensure!(configs.len() <= 4096, "too many configurations");
            let mut rows = Vec::new();
            for (i, c) in configs.iter().enumerate() {
                let t = Instant::now();
                let result = match build(c, Path::new(&a[2])) {
                    Ok(id) => json!({"index":i,"status":"built","manifest_sha256":id}),
                    Err(e) => json!({"index":i,"status":"failed","reason":e.to_string()}),
                };
                rows.push(json!({"config_sha256":digest(&canonical(c)?),"result":result,"elapsed_ms":t.elapsed().as_secs_f64()*1000.}));
            }
            save(
                &a[3],
                &json!({"schema":"factor-base-batch/v1","input_sha256":digest(&canonical(&configs)?),"source_sha256":source_digest(),"rows":rows,"wall_time_qualification":"unqualified_diagnostic"}),
            )
        }
        ("verify", 3) => {
            let (m, g) = verify(Path::new(&a[1]), &a[2])?;
            emit(
                &json!({"status":"replayed","manifest_sha256":a[2],"points":g.points.len(),"requirements":m.requirements,"replay_source_sha256":source_digest(),"independence":"same_native_implementation"}),
            )
        }
        ("evaluate", 5) => {
            let w = read_json(Path::new(&a[3]))?;
            let result = evaluate::evaluate(Path::new(&a[1]), &a[2], &w)?;
            save(&a[4], &result)
        }
        ("replay", 4) => {
            let r = read_json(Path::new(&a[3]))?;
            emit(&evaluate::replay(Path::new(&a[1]), &a[2], &r)?)
        }
        ("compare", len) if len >= 2 => {
            let receipts = a[1..]
                .iter()
                .map(|p| read_json::<Value>(Path::new(p)))
                .collect::<Result<Vec<_>>>()?;
            emit(&compare(&receipts)?)
        }
        ("upload-plan", 4) => emit(&plan(Path::new(&a[1]), &a[2], &a[3])?),
        ("fetch", 5) => emit(&fetch(Path::new(&a[1]), &a[2], &a[3], &a[4])?),
        ("publish", 6) => {
            let receipt = publish(Path::new(&a[1]), &a[2], &a[3], &a[4])?;
            save(&a[5], &receipt)
        }
        _ => bail!("invalid command or argument count\n{HELP}"),
    }
}
fn compare(receipts: &[Value]) -> Result<Value> {
    let first = receipts.first().context("no receipts")?;
    let mut eligible = Vec::new();
    let mut excluded = Vec::new();
    for r in receipts {
        let s = &r["semantic"];
        anyhow::ensure!(
            r["semantic_sha256"] == digest(&canonical(s)?),
            "receipt semantic digest mismatch"
        );
        anyhow::ensure!(
            s["schema"] == "factor-base-evaluation/v1"
                && s["curve_sha256"] == first["semantic"]["curve_sha256"]
                && s["workload_sha256"] == first["semantic"]["workload_sha256"]
                && s["targets"] == first["semantic"]["targets"],
            "comparison requires identical curve, workload, and sampled points"
        );
        let regs = s["regularity"]
            .as_array()
            .context("missing regularity array")?;
        if s["relation_yield"].is_null()
            || regs.is_empty()
            || regs.iter().any(|r| r["status"] != "measured")
        {
            excluded.push(json!({"manifest_sha256":s["manifest_sha256"],"reason":"yield_or_regularity_unmeasured","relation_yield":s["relation_yield"]}));
            continue;
        }
        let d = regs
            .iter()
            .map(|r| r["degree_of_regularity"].as_u64().context("invalid degree"))
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .max()
            .unwrap();
        anyhow::ensure!(
            s["relation_yield"]
                .as_f64()
                .is_some_and(|y| (0.0..=1.0).contains(&y))
                && s["base_points"].as_u64().is_some_and(|n| n > 0),
            "invalid yield or base size in receipt"
        );
        eligible.push(json!({"manifest_sha256":s["manifest_sha256"],"yield":s["relation_yield"],"max_measured_regularity":d,"base_points":s["base_points"]}));
    }
    let frontier = eligible
        .iter()
        .filter(|a| {
            !eligible.iter().any(|b| {
                let ya = a["yield"].as_f64().unwrap();
                let yb = b["yield"].as_f64().unwrap();
                let da = a["max_measured_regularity"].as_u64().unwrap();
                let db = b["max_measured_regularity"].as_u64().unwrap();
                let ba = a["base_points"].as_u64().unwrap();
                let bb = b["base_points"].as_u64().unwrap();
                yb >= ya && db <= da && bb <= ba && (yb > ya || db < da || bb < ba)
            })
        })
        .cloned()
        .collect::<Vec<_>>();
    Ok(
        json!({"schema":"factor-base-screen/v1","frontier":frontier,"excluded":excluded,"candidate_id":null,"selected_best_total_runtime":null,"interpretation":"descriptive_pareto_screen_not_statistical_or_total_cost_selection","split":first["semantic"]["workload"]["split"]}),
    )
}
fn main() {
    if let Err(e) = run() {
        eprintln!("factor-base: {e:#}");
        std::process::exit(1);
    }
}
