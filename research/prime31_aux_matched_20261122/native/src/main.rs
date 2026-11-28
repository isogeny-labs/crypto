use std::fs;
use std::io::{self, Write};
use std::time::Instant;

use prime31_aux_matched::bases::{build_base, build_plain_control, prime31_fixture, BaseFamily};
use prime31_aux_matched::collector::{
    collect, group_ledger, to_replay, CollectionLimits, PublicTarget, QueryStream,
};
use prime31_aux_matched::oracle::Presentation;
use prime31_aux_matched::{CountedGroup, GroupOps, SOURCE_MANIFEST};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Cell {
    schema: String,
    cell_id: String,
    public_target: PublicTarget,
    reference_family: BaseFamily,
    depth: u32,
    base_seed: u64,
    max_seed_builds: u32,
    plain_control: bool,
    plain_seed: u64,
    max_plain_x_trials: u64,
    arity: usize,
    presentation: Presentation,
    query_seed: u64,
    collection: CollectionLimits,
    execution_seconds: u64,
    memory_mib: u64,
    rho_max_steps: u64,
    rho_seed: u64,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Panel {
    schema: String,
    protocol_id: String,
    public_target: PublicTarget,
    families: Vec<BaseFamily>,
    depths: Vec<u32>,
    base_seeds: Vec<u64>,
    arities: Vec<usize>,
    max_seed_builds: u32,
    plain_seed: u64,
    max_plain_x_trials: u64,
    query_seed: u64,
    collection: CollectionLimits,
    execution_seconds: u64,
    memory_mib: u64,
    rho_max_steps: u64,
    rho_seed: u64,
}

fn hash(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn emit(v: &Value) -> Result<(), String> {
    let mut out = io::stdout().lock();
    serde_json::to_writer(&mut out, v).map_err(|e| e.to_string())?;
    writeln!(out).map_err(|e| e.to_string())?;
    out.flush().map_err(|e| e.to_string())
}
fn read_cell(path: &str) -> Result<(Cell, Vec<u8>), String> {
    let bytes = fs::read(path).map_err(|e| e.to_string())?;
    let cell: Cell = serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
    if cell.schema != "prime31-aux-matched.cell/v1"
        || cell.cell_id.is_empty()
        || cell.reference_family == BaseFamily::Plain
        || !(1..=10).contains(&cell.depth)
        || !(2..=4).contains(&cell.arity)
        || cell.max_seed_builds == 0
        || cell.max_seed_builds > 32
        || cell.execution_seconds == 0
        || cell.execution_seconds > 3600
        || cell.memory_mib < 128
        || cell.memory_mib > 32768
        || cell.collection.max_seconds > cell.execution_seconds
        || (cell.plain_control && cell.presentation != Presentation::Dense)
        || cell.rho_max_steps == 0
    {
        return Err("invalid or unbounded cell specification".into());
    }
    let (e, n, g) = prime31_fixture();
    if (
        cell.public_target.p,
        cell.public_target.a,
        cell.public_target.b,
        cell.public_target.order,
        cell.public_target.generator,
    ) != (e.p, e.a, e.b, n, to_replay(g))
    {
        return Err("campaign cells must use the frozen ordinary Prime31 fixture".into());
    }
    Ok((cell, bytes))
}

fn run(path: &str, rho: bool) -> Result<i32, String> {
    let (cell, bytes) = read_cell(path)?;
    let all_start = Instant::now();
    let source_revision = std::env::var("SOURCE_REVISION").ok();
    if source_revision
        .as_ref()
        .is_some_and(|s| s.len() != 40 || !s.bytes().all(|b| b.is_ascii_hexdigit()))
    {
        return Err("SOURCE_REVISION must be a full commit SHA when supplied".into());
    }
    emit(
        &json!({"event":"cell_start","cell":cell,"input_sha256":hash(&bytes),
        "source_revision_claim":source_revision,"source_manifest_sha256":hash(SOURCE_MANIFEST.as_bytes()),
        "execution_level":"diagnostic_no_timing_admission","performance_claim":false,
        "native_counts_are_partial":true,"rho_control":rho}),
    )?;
    let mut validation = GroupOps::default();
    cell.public_target.validate(&mut validation)?;
    emit(&json!({"event":"public_target_validation","group_and_field":group_ledger(&validation)}))?;
    if rho {
        let curve = cell.public_target.curve();
        let result = prime31_aux_matched::cryptanalysis::ic_boundary::rho_reference(
            &curve,
            prime31_aux_matched::collector::from_replay(cell.public_target.generator),
            prime31_aux_matched::collector::from_replay(cell.public_target.target),
            cell.public_target.order,
            cell.rho_seed,
            cell.rho_max_steps,
        );
        let independently_replayed = match result.recovered {
            Some(d) => {
                cell.public_target
                    .replay_curve()
                    .scalar(cell.public_target.generator, d)?
                    == cell.public_target.target
            }
            None => false,
        };
        if result.recovered.is_some() && !independently_replayed {
            return Err("rho candidate replay failed".into());
        }
        emit(&json!({"event":"rho_summary","result":result,
            "group_and_field":group_ledger(&result.group_ops),"independent_replay":independently_replayed,
            "reference_scope":"production A=1 reference; not the tuned negation-aware baseline",
            "total_seconds_diagnostic":all_start.elapsed().as_secs_f64()}))?;
        return Ok(if independently_replayed { 0 } else { 2 });
    }
    // Plain controls need the reference cardinality. Preserve the entire
    // matching construction and its work as study-design overhead. It must
    // never silently disappear from the campaign bill or become plain-base
    // algorithm work when a cold algorithm subtotal is reported.
    let reference_start = Instant::now();
    emit(
        &json!({"event":"reference_build_start","family":cell.reference_family,
        "seed":cell.base_seed,"depth":cell.depth,"max_seed_builds":cell.max_seed_builds}),
    )?;
    let reference = match build_base(
        &cell.public_target.curve(),
        cell.public_target.order,
        cell.depth,
        cell.base_seed,
        cell.reference_family,
        cell.max_seed_builds,
    ) {
        Ok(b) => b,
        Err(error) => {
            emit(
                &json!({"event":"setup_failure","stage":"reference_build","failure":error,
                "total_seconds_diagnostic":all_start.elapsed().as_secs_f64()}),
            )?;
            return Ok(3);
        }
    };
    let reference_seconds = reference_start.elapsed().as_secs_f64();
    emit(&json!({"event":"reference_build_result","base":reference,
        "seconds_diagnostic":reference_seconds,"matching_overhead_only":cell.plain_control}))?;
    let plain_start = Instant::now();
    let selected = if cell.plain_control {
        match build_plain_control(
            &cell.public_target.curve(),
            cell.public_target.order,
            reference.columns,
            cell.plain_seed,
            cell.max_plain_x_trials,
        ) {
            Ok(base) => base,
            Err(error) => {
                emit(
                    &json!({"event":"setup_failure","stage":"matched_plain_build","failure":error,
                    "reference":reference,"total_seconds_diagnostic":all_start.elapsed().as_secs_f64()}),
                )?;
                return Ok(3);
            }
        }
    } else {
        reference.clone()
    };
    if selected.columns != reference.columns {
        return Err("matched control cardinality mismatch".into());
    }
    let plain_seconds = if cell.plain_control {
        plain_start.elapsed().as_secs_f64()
    } else {
        0.
    };
    let index_start = Instant::now();
    let fb = selected.factor_base()?;
    let index_seconds = index_start.elapsed().as_secs_f64();
    emit(
        &json!({"event":"selected_base","base":selected,"factor_base_index_seconds_diagnostic":index_seconds,
        "factor_base_index_cost":fb.cost,
        "reference_columns":reference.columns,"matched_columns":fb.columns,
        "plain_build_seconds_diagnostic":plain_seconds}),
    )?;
    let mut limits = cell.collection.clone();
    let remaining = cell
        .execution_seconds
        .saturating_sub(all_start.elapsed().as_secs());
    if remaining == 0 {
        emit(
            &json!({"event":"cell_summary","status":"setup_exhausted_deadline",
            "total_seconds_diagnostic":all_start.elapsed().as_secs_f64(),"recovered":false}),
        )?;
        return Ok(2);
    }
    limits.max_seconds = limits.max_seconds.min(remaining);
    let summary = collect(
        &cell.public_target,
        &fb,
        &selected.v,
        selected.tower.as_ref(),
        cell.arity,
        cell.presentation,
        cell.query_seed,
        &limits,
        &mut emit,
    )?;
    emit(
        &json!({"event":"cell_summary","status":summary.status,"recovered":summary.recovery_replayed,
        "collection":summary,"reference_setup_seconds_diagnostic":reference_seconds,
        "plain_setup_seconds_diagnostic":plain_seconds,"index_seconds_diagnostic":index_seconds,
        "matching_overhead_seconds_diagnostic":if cell.plain_control {reference_seconds}else{0.},
        "total_seconds_diagnostic":all_start.elapsed().as_secs_f64(),
        "scope":"observed diagnostic elapsed time includes matching, all failed construction attempts, all queries, extraction, signed replay and rank work; no admitted speed ratio"}),
    )?;
    Ok(if summary.recovery_replayed { 0 } else { 2 })
}

fn panel(path: &str, directory: &str) -> Result<i32, String> {
    let bytes = fs::read(path).map_err(|e| e.to_string())?;
    let p: Panel = serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
    if p.schema != "prime31-aux-matched.panel/v1" {
        return Err("unknown panel schema".into());
    }
    fs::create_dir_all(directory).map_err(|e| e.to_string())?;
    let mut cells = Vec::new();
    for &family in &p.families {
        for &depth in &p.depths {
            for &seed in &p.base_seeds {
                for &arity in &p.arities {
                    for (plain, presentation, arm) in [
                        (false, Presentation::Dense, "dense"),
                        (false, Presentation::Tower, "tower"),
                        (true, Presentation::Dense, "plain"),
                    ] {
                        let family_name = serde_json::to_value(family)
                            .map_err(|e| e.to_string())?
                            .as_str()
                            .unwrap()
                            .to_string();
                        let id = format!(
                            "{}-{}-t{}-s{}-m{}-{}",
                            p.protocol_id, family_name, depth, seed, arity, arm
                        );
                        if !id
                            .bytes()
                            .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_')
                        {
                            return Err("unsafe protocol identifier".into());
                        }
                        let cell = Cell {
                            schema: "prime31-aux-matched.cell/v1".into(),
                            cell_id: id.clone(),
                            public_target: p.public_target.clone(),
                            reference_family: family,
                            depth,
                            base_seed: seed,
                            max_seed_builds: p.max_seed_builds,
                            plain_control: plain,
                            plain_seed: p.plain_seed,
                            max_plain_x_trials: p.max_plain_x_trials,
                            arity,
                            presentation,
                            query_seed: p.query_seed,
                            collection: p.collection.clone(),
                            execution_seconds: p.execution_seconds,
                            memory_mib: p.memory_mib,
                            rho_max_steps: p.rho_max_steps,
                            rho_seed: p.rho_seed,
                        };
                        let destination =
                            std::path::Path::new(directory).join(format!("{id}.json"));
                        let mut file = fs::OpenOptions::new()
                            .write(true)
                            .create_new(true)
                            .open(&destination)
                            .map_err(|e| e.to_string())?;
                        let serialized =
                            serde_json::to_vec_pretty(&cell).map_err(|e| e.to_string())?;
                        file.write_all(&serialized).map_err(|e| e.to_string())?;
                        cells.push(json!({"cell_id":id,"input_sha256":hash(&serialized),"file":destination}));
                    }
                }
            }
        }
    }
    emit(
        &json!({"event":"panel_materialized","protocol_sha256":hash(&bytes),"cells":cells,
        "experiment_executed":false}),
    )?;
    Ok(0)
}

fn make_target(seed: u64) -> Result<i32, String> {
    let (curve, order, generator) = prime31_fixture();
    let mut stream = QueryStream::new(seed);
    let mut d = stream.scalar(order);
    while d == 0 {
        d = stream.scalar(order);
    }
    let mut ops = GroupOps::default();
    let q = curve.mul(&mut ops, generator, d);
    let target = PublicTarget {
        p: curve.p,
        a: curve.a,
        b: curve.b,
        order,
        generator: to_replay(generator),
        target: to_replay(q),
    };
    if target.replay_curve().scalar(target.generator, d)? != target.target {
        return Err("target fixture generation replay failed".into());
    }
    emit(
        &json!({"event":"target_fixture_preparation","fixture_seed":seed,"public_target":target,
        "generation_group_and_field":group_ledger(&ops),"generation_random_words":stream.words,
        "target_scalar_passed_to_solver":false,"experiment_executed":false}),
    )?;
    Ok(0)
}

fn command() -> Result<i32, String> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    match args.iter().map(String::as_str).collect::<Vec<_>>().as_slice() {
        ["source-manifest"] => { println!("{SOURCE_MANIFEST}"); Ok(0) }
        ["source-check", root] => {
            emit(&prime31_aux_matched::provenance::check_sources(std::path::Path::new(root))?)?;
            Ok(0)
        }
        ["replay", path] => {
            let file = fs::File::open(path).map_err(|e| e.to_string())?;
            emit(&prime31_aux_matched::audit::audit_jsonl(std::io::BufReader::new(file))?)?;
            Ok(0)
        }
        ["make-target", seed] => make_target(seed.parse::<u64>().map_err(|e|e.to_string())?),
        ["panel", path, directory] => panel(path,directory),
        ["run", path] => run(path,false),
        ["rho", path] => run(path,true),
        _ => Err("usage: prime31-aux-matched source-manifest | source-check ROOT | replay EVENTS.jsonl | make-target SEED | panel PANEL.json OUTDIR | run CELL.json | rho CELL.json".into()),
    }
}
fn main() {
    match command() {
        Ok(code) => std::process::exit(code),
        Err(error) => {
            let _ = emit(&json!({"event":"fatal_error","error":error}));
            eprintln!("{error}");
            std::process::exit(1);
        }
    }
}
