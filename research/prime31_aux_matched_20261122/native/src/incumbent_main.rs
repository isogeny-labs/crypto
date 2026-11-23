//! Source-bound complete-IC control. Panel/validation commands perform no solve.
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::time::Instant;

use prime31_aux_matched::bases::prime31_fixture;
use prime31_aux_matched::collector::{from_replay, PublicTarget};
use prime31_aux_matched::incumbent_control::{
    solve_fast_cold, FastIcConfig, FAST_IC_SOURCE_REVISION, FAST_IC_SOURCE_SHA256,
};
use prime31_aux_matched::replay;
use prime31_aux_matched::SOURCE_MANIFEST;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct IncumbentCell {
    schema: String,
    cell_id: String,
    public_target: PublicTarget,
    config: FastIcConfig,
    stage: String,
    round: u32,
    execution_seconds: u64,
    memory_mib: u64,
}
fn digest(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
fn emit(v: &Value) -> Result<(), String> {
    let mut out = io::stdout().lock();
    serde_json::to_writer(&mut out, v).map_err(|e| e.to_string())?;
    writeln!(out).map_err(|e| e.to_string())?;
    out.flush().map_err(|e| e.to_string())
}
fn target() -> PublicTarget {
    let (c, n, g) = prime31_fixture();
    PublicTarget {
        p: c.p,
        a: c.a,
        b: c.b,
        order: n,
        generator: replay::Point::affine(g.x, g.y),
        target: replay::Point::affine(984_837_385, 1_142_774_582),
    }
}
pub(crate) fn read_cell(path: &str) -> Result<(IncumbentCell, Vec<u8>), String> {
    let bytes = fs::read(path).map_err(|e| e.to_string())?;
    let c: IncumbentCell = serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
    let (e, n, g) = prime31_fixture();
    if c.schema != "prime31-complete-ic-incumbent.cell/v1"
        || c.cell_id.is_empty()
        || (
            c.public_target.p,
            c.public_target.a,
            c.public_target.b,
            c.public_target.order,
            from_replay(c.public_target.generator),
        ) != (e.p, e.a, e.b, n, g)
        || c.public_target.target != target().target
        || !matches!(c.stage.as_str(), "screening" | "complete_recovery")
        || c.execution_seconds != 120
        || c.memory_mib != 4096
        || c.config.extra_relations != 16
        || c.config.max_target_walk != 65_536
    {
        return Err("unbounded or mismatched complete-IC incumbent cell".into());
    }
    c.config.validate_limits(&c.public_target)?;
    Ok((c, bytes))
}
pub(crate) fn panel(directory: &str) -> Result<(), String> {
    let root = Path::new(directory);
    if !root.is_absolute() || root.exists() {
        return Err("incumbent panel requires a new absolute directory".into());
    }
    fs::create_dir_all(root).map_err(|e| e.to_string())?;
    let seeds = [4188, 104_729, 130_363, 169_087, 224_737];
    for (label, stage, seeds) in [
        ("screening", "screening", &seeds[..1]),
        ("recovery-candidates", "complete_recovery", &seeds[..]),
    ] {
        let dir = root.join(label);
        fs::create_dir(&dir).map_err(|e| e.to_string())?;
        let mut cells = Vec::new();
        for (round, &seed) in seeds.iter().enumerate() {
            for (position, fb_size) in [None, Some(515), Some(677)].into_iter().enumerate() {
                let c = IncumbentCell {
                    schema: "prime31-complete-ic-incumbent.cell/v1".into(),
                    cell_id: format!("p31-ic-incumbent-{label}-s{round}-{position}"),
                    public_target: target(),
                    config: FastIcConfig {
                        fb_size,
                        extra_relations: 16,
                        max_target_walk: 65_536,
                        seed,
                    },
                    stage: stage.into(),
                    round: round as u32,
                    execution_seconds: 120,
                    memory_mib: 4096,
                };
                let file = format!("{}.json", c.cell_id);
                let bytes = serde_json::to_vec_pretty(&c).map_err(|e| e.to_string())?;
                let path = dir.join(&file);
                fs::write(&path, &bytes).map_err(|e| e.to_string())?;
                read_cell(path.to_str().ok_or("non-UTF8 input path")?)?;
                cells.push(json!({"file":file,"sha256":digest(&bytes),"seed":seed,"requested_factor_base":fb_size,"round":round}));
            }
        }
        fs::write(dir.join("index.json"),serde_json::to_vec_pretty(&json!({
            "schema":"prime31-complete-ic-incumbent.panel/v1","stage":stage,
            "status":"prospective_unselected_controls","prepared_cells":cells.len(),"attempted":0,"selected_for_execution":0,
            "backend":"prime_fast.ic_solve_fast","source_revision":FAST_IC_SOURCE_REVISION,"source_sha256":FAST_IC_SOURCE_SHA256,
            "target_scope":"same fixed original ordinary Prime31 public target; cold single-target calls",
            "setup_reuse":false,"target_walk_cap_is_not_setup_cap":true,
            "source_bound_outer_envelope_required":true,"admission_granted":false,"cells":cells
        })).unwrap()).map_err(|e| e.to_string())?;
    }
    emit(
        &json!({"event":"incumbent_panel_prepared","screening_cells":3,"recovery_candidates":15,"experiment_executed":false}),
    )
}
fn run(path: &str) -> Result<i32, String> {
    let cold = Instant::now();
    let (cell, bytes) = read_cell(path)?;
    emit(
        &json!({"event":"incumbent_start","cell":cell,"input_sha256":digest(&bytes),
        "source_manifest_sha256":digest(SOURCE_MANIFEST.as_bytes()),
        "source_revision_claim":std::env::var("SOURCE_REVISION").ok(),"timing_admitted":false}),
    )?;
    let result = match solve_fast_cold(&cell.public_target, &cell.config) {
        Ok(r) => r,
        Err(e) => {
            emit(
                &json!({"event":"incumbent_failure","error":e,"cold_seconds_diagnostic":cold.elapsed().as_secs_f64()}),
            )?;
            return Ok(1);
        }
    };
    let recovered = result.independent_replay;
    emit(
        &json!({"event":"incumbent_summary","result":result,"recovered":recovered,
        "cold_seconds_diagnostic":cold.elapsed().as_secs_f64(),"timing_admitted":false,
        "clock_scope":"input read/validation, start record, complete native cold setup/target/replay; excludes process startup and final summary serialization/shutdown",
        "outer_process_envelope_required":true}),
    )?;
    Ok(if recovered { 0 } else { 2 })
}

/// Independent scalar-only replay; it does not reconstruct the baseline's
/// internal relations, counters, operation history, timing or execution source.
pub(crate) fn audit_records(records: &[Value]) -> Result<Value, String> {
    if records.is_empty() || records[0]["event"] != "incumbent_start" {
        return Err("missing incumbent start".into());
    }
    let c: IncumbentCell =
        serde_json::from_value(records[0]["cell"].clone()).map_err(|e| e.to_string())?;
    if c.schema != "prime31-complete-ic-incumbent.cell/v1" {
        return Err("unsupported incumbent schema".into());
    }
    c.config.validate_limits(&c.public_target)?;
    let t = &c.public_target;
    let e = t.replay_curve();
    if !e.valid_parameters()
        || !replay::prime(t.p)
        || !replay::prime(t.order)
        || t.generator.infinity
        || t.target.infinity
        || !e.on_curve(t.generator)
        || !e.on_curve(t.target)
        || e.scalar(t.generator, t.order)? != replay::Point::IDENTITY
        || e.scalar(t.target, t.order)? != replay::Point::IDENTITY
    {
        return Err("invalid independently replayed public target".into());
    }
    let mut recovered = None;
    let mut terminal = false;
    for record in &records[1..] {
        if terminal {
            return Err("records after incumbent terminal".into());
        }
        match record["event"].as_str() {
            Some("incumbent_failure" | "driver_error") => {
                terminal = true;
            }
            Some("incumbent_summary") => {
                let result = &record["result"];
                if result["schema"] != "prime31-complete-ic-incumbent/v1"
                    || result["backend"] != "prime_fast.ic_solve_fast"
                    || result["source_revision"] != FAST_IC_SOURCE_REVISION
                    || result["source_sha256"] != FAST_IC_SOURCE_SHA256
                    || result["config"] != serde_json::to_value(&c.config).unwrap()
                {
                    return Err("incumbent result changes its input/backend/source claim".into());
                }
                recovered = match result.get("recovered_scalar") {
                    Some(Value::Null) => None,
                    Some(v) => Some(v.as_u64().ok_or("noncanonical recovered scalar")?),
                    None => return Err("missing recovered scalar".into()),
                };
                if let Some(d) = recovered {
                    if d >= t.order || e.scalar(t.generator, d)? != t.target {
                        return Err("incumbent scalar fails independent public-point replay".into());
                    }
                }
                if result["independent_replay"].as_bool() != Some(recovered.is_some())
                    || record["recovered"].as_bool() != Some(recovered.is_some())
                    || result["status"]
                        != if recovered.is_some() {
                            "recovered"
                        } else {
                            "unrecovered_unspecified"
                        }
                {
                    return Err("incumbent result status contradicts scalar replay".into());
                }
                terminal = true;
            }
            _ => return Err("unsupported incumbent event".into()),
        }
    }
    Ok(
        json!({"schema":"prime31-complete-ic-incumbent.audit/v1","audit_valid":true,
        "stream_complete":terminal,"recovered_scalar":recovered,"complete":terminal&&recovered.is_some(),
        "public_scalar_equation_verified":recovered.is_some(),"internal_relations_replayed":false,
        "negative_exhaustion_replayed":false,"performance_independently_validated":false,
        "execution_source_independently_verified":false,
        "frozen_input_scope_verified":false,"input_file_hash_verified":false,
        "source_manifest_hash_verified":false,"timing_admission_verified":false,
        "scope":"scalar equation and transcript consistency for the stated public target; frozen Prime31 input selection, input bytes, source execution and timing admission require separate evidence"}),
    )
}
fn main_result() -> Result<i32, String> {
    let args: Vec<_> = std::env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("run") if args.len()==3=>run(&args[2]),
        Some("panel") if args.len()==3=>{panel(&args[2])?;Ok(0)},
        Some("validate-cell") if args.len()==3=>{let(c,bytes)=read_cell(&args[2])?;emit(&json!({"valid":true,"cell_id":c.cell_id,"input_sha256":digest(&bytes),"experiment_executed":false}))?;Ok(0)},
        Some("replay") if args.len()==3=>{let text=fs::read_to_string(&args[2]).map_err(|e|e.to_string())?;
            let records=text.lines().filter(|s|!s.trim().is_empty()).map(serde_json::from_str).collect::<Result<Vec<Value>,_>>().map_err(|e|e.to_string())?;
            emit(&audit_records(&records)?)?;Ok(0)},
        Some("source-manifest")=>{println!("{SOURCE_MANIFEST}");Ok(0)},
        Some("source-check") if args.len()==3=>{emit(&prime31_aux_matched::provenance::check_sources(Path::new(&args[2]))?)?;Ok(0)},
        _=>Err("usage: prime31-ic-incumbent run|panel|validate-cell|replay|source-check PATH | source-manifest".into()),
    }
}
fn main() {
    let code = match main_result() {
        Ok(code) => code,
        Err(e) => {
            let _ = emit(&json!({"event":"driver_error","error":e}));
            1
        }
    };
    std::process::exit(code);
}
