//! Bounded comparison driver. Actual campaign execution requires the existing
//! checkpoint/publication/budget/admission wrapper; panel generation is not a run.
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::time::Instant;

use prime31_aux_matched::bases::{build_plain_control, prime31_fixture};
use prime31_aux_matched::collector::{self, CollectionLimits, PublicTarget};
use prime31_aux_matched::cryptanalysis::{ic_boundary, prime_fast};
use prime31_aux_matched::kernel_fibers::{build_fiber, KernelChoice};
use prime31_aux_matched::oracle::{DomainRoots, OracleLimits, Presentation};
use prime31_aux_matched::replay::Point;
use prime31_aux_matched::target_transport::{self as transfer, TargetArm, TransferWork};
use prime31_aux_matched::{GroupOps, SOURCE_MANIFEST};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Method {
    Ic,
    RhoNegation,
    RhoV6,
    RhoFast,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ChoiceCell {
    schema: String,
    cell_id: String,
    method: Method,
    source_target: PublicTarget,
    kernel: KernelChoice,
    coset_scalar: u64,
    target_arm: TargetArm,
    arity: usize,
    presentation: Presentation,
    #[serde(default)]
    plain_control: bool,
    #[serde(default = "plain_seed")]
    plain_seed: u64,
    #[serde(default = "plain_max_trials")]
    max_plain_x_trials: u64,
    query_seed: u64,
    collection: CollectionLimits,
    execution_seconds: u64,
    memory_mib: u64,
    round: u32,
    rho_max_steps: u64,
    rho_seed: u64,
}

fn plain_seed() -> u64 {
    1
}
fn plain_max_trials() -> u64 {
    1_000_000
}

fn hash(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
fn emit(value: &Value) -> Result<(), String> {
    let mut output = io::stdout().lock();
    serde_json::to_writer(&mut output, value).map_err(|e| e.to_string())?;
    writeln!(output).map_err(|e| e.to_string())?;
    output.flush().map_err(|e| e.to_string())
}
fn read_cell(path: &str) -> Result<(ChoiceCell, Vec<u8>), String> {
    let bytes = fs::read(path).map_err(|e| e.to_string())?;
    let cell: ChoiceCell = serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
    let (e, n, g) = prime31_fixture();
    if cell.schema != "prime31-isogeny-choice.cell/v1"
        || cell.cell_id.is_empty()
        || (
            cell.source_target.p,
            cell.source_target.a,
            cell.source_target.b,
            cell.source_target.order,
            collector::from_replay(cell.source_target.generator),
        ) != (e.p, e.a, e.b, n, g)
        || cell.source_target.target != source_target().target
        || cell.arity != 3
        || cell.coset_scalar == 0
        || cell.coset_scalar & 1 == 0
        || cell.coset_scalar >= 1 << 19
        || cell.execution_seconds != 120
        || cell.memory_mib != 4096
        || cell.collection.max_seconds > cell.execution_seconds
        || cell.collection.max_queries == 0
        || cell.collection.max_queries > 4096
        || cell.collection.oracle.max_variables > 64
        || cell.collection.oracle.max_input_terms > 100_000
        || cell.collection.oracle.max_degree < 1024
        || cell.collection.oracle.max_degree > 2048
        || cell.collection.oracle.timeout_ms == 0
        || cell.collection.oracle.timeout_ms > 30_000
        || cell.rho_max_steps == 0
        || cell.rho_max_steps > 10_000_000
        || cell.plain_seed != plain_seed()
        || cell.max_plain_x_trials != plain_max_trials()
        || (cell.plain_control
            && (cell.method != Method::Ic || cell.presentation != Presentation::Dense))
        || (cell.target_arm == TargetArm::IsogenousTransported
            && cell.presentation != Presentation::Dense)
    {
        return Err("invalid, mismatched, or unbounded isogeny-choice cell".into());
    }
    Ok((cell, bytes))
}

fn source_target() -> PublicTarget {
    let (e, order, g) = prime31_fixture();
    PublicTarget {
        p: e.p,
        a: e.a,
        b: e.b,
        order,
        generator: collector::to_replay(g),
        target: Point::affine(984_837_385, 1_142_774_582),
    }
}

fn rho_target(cell: &ChoiceCell, work: &mut TransferWork) -> Result<PublicTarget, String> {
    let mut target = cell.source_target.clone();
    if cell.target_arm != TargetArm::Original {
        let e1 = transfer::codomain();
        target.a = e1.a;
        target.b = e1.b;
        target.generator = collector::to_replay(transfer::image_point(
            collector::from_replay(target.generator),
            work,
        )?);
        target.target = collector::to_replay(transfer::image_point(
            collector::from_replay(target.target),
            work,
        )?);
    }
    Ok(target)
}

fn run_rho(cell: &ChoiceCell, cold: Instant) -> Result<i32, String> {
    let setup_start = Instant::now();
    let mut work = TransferWork {
        costs_partial: true,
        ..Default::default()
    };
    let target = rho_target(cell, &mut work)?;
    let mut validation = GroupOps::default();
    cell.source_target.validate(&mut validation)?;
    target.validate(&mut validation)?;
    let curve = target.curve();
    let g = collector::from_replay(target.generator);
    let q = collector::from_replay(target.target);
    let setup_seconds = setup_start.elapsed().as_secs_f64();
    let online = Instant::now();
    let (d, result) = match cell.method {
        Method::RhoFast => {
            let c = prime_fast::FastCurve::new(
                "prime31-isogeny-choice",
                curve.p,
                curve.a,
                curve.b,
                target.order,
            );
            let mut config = prime_fast::RhoConfig::tuned(target.order);
            config.seed = cell.rho_seed;
            config.max_steps = cell.rho_max_steps;
            config.threads = 1;
            let result =
                prime_fast::rho_parallel(&c, c.point(g.x, g.y), c.point(q.x, q.y), &config);
            match result {
                Some(r) => (
                    Some(r.log),
                    json!({"recovered":r.log,"backend":"rho.prime_fast",
                    "total_steps":r.total_steps,"distinguished_points":r.distinguished_points,
                    "threads":r.threads,"walkers_per_thread":r.walkers_per_thread,"dp_bits":r.dp_bits,
                    "ops":{"table_adds":r.ops.table_adds,"table_doubles":r.ops.table_doubles,
                        "initial_start_adds":r.ops.initial_start_adds,"initial_start_doubles":r.ops.initial_start_doubles,
                        "restart_adds":r.ops.restart_adds,"restart_doubles":r.ops.restart_doubles,
                        "escape_doubles":r.ops.escape_doubles,"collision_checks":r.ops.collision_checks,
                        "look_ahead_adds":r.ops.look_ahead_adds,"cycles_2":r.ops.cycles_2,
                        "cycles_4":r.ops.cycles_4,"cycles_other":r.ops.cycles_other,
                        "abandoned":r.ops.abandoned,"capped":r.ops.capped,"restarts":r.ops.restarts,
                        "dp_trail_steps":r.ops.dp_trail_steps,"lost_steps":r.ops.lost_steps,
                        "in_flight_steps":r.ops.in_flight_steps},
                    "wall_seconds_diagnostic":r.wall_secs}),
                ),
                None => (
                    None,
                    json!({"recovered":null,"backend":"rho.prime_fast",
                    "status":"capped_no_recovery","failure_operation_counters":null}),
                ),
            }
        }
        Method::RhoNegation | Method::RhoV6 => {
            let result = if cell.method == Method::RhoNegation {
                ic_boundary::rho_reference_negation(
                    &curve,
                    g,
                    q,
                    target.order,
                    cell.rho_seed,
                    cell.rho_max_steps,
                )
            } else {
                ic_boundary::rho_walk_v3(
                    &curve,
                    &ic_boundary::NegationClasses { r: target.order },
                    g,
                    q,
                    target.order,
                    cell.rho_seed,
                    cell.rho_max_steps,
                    ic_boundary::RhoV3::v6(true),
                )
            };
            (
                result.recovered,
                serde_json::to_value(result).map_err(|e| e.to_string())?,
            )
        }
        Method::Ic => return Err("IC cell sent to rho".into()),
    };
    let independent_replay = match d {
        Some(d) => {
            target.replay_curve().scalar(target.generator, d)? == target.target
                && cell
                    .source_target
                    .replay_curve()
                    .scalar(cell.source_target.generator, d)?
                    == cell.source_target.target
        }
        None => false,
    };
    if d.is_some() && !independent_replay {
        return Err("rho scalar failed original/destination replay".into());
    }
    let online_seconds = online.elapsed().as_secs_f64();
    emit(
        &json!({"event":"rho_summary","result":result,"independent_replay":independent_replay,
        "public_target":target,"target_setup_seconds_diagnostic":setup_seconds,
        "target_setup_transfer_work":work,"validation_work":collector::group_ledger(&validation),
        "online_including_independent_replay_seconds_diagnostic":online_seconds,
        "cold_seconds_diagnostic":cold.elapsed().as_secs_f64(),"timing_admitted":false}),
    )?;
    Ok(if independent_replay { 0 } else { 2 })
}

fn run(path: &str) -> Result<i32, String> {
    let (cell, bytes) = read_cell(path)?;
    let cold = Instant::now();
    emit(
        &json!({"event":"choice_start","cell":cell,"input_sha256":hash(&bytes),
        "source_manifest_sha256":hash(SOURCE_MANIFEST.as_bytes()),
        "source_revision_claim":std::env::var("SOURCE_REVISION").ok(),
        "timing_admitted":false,"native_counts_are_partial":true}),
    )?;
    if cell.method != Method::Ic {
        return run_rho(&cell, cold);
    }
    let setup = Instant::now();
    let mut ops = GroupOps::default();
    let fiber = match build_fiber(cell.kernel, cell.coset_scalar, &mut ops) {
        Ok(fiber) => fiber,
        Err(error) => {
            emit(
                &json!({"event":"setup_failure","stage":"kernel","error":error,
                "partial_group_work":collector::group_ledger(&ops),"map_work":null,
                "cold_seconds_diagnostic":cold.elapsed().as_secs_f64()}),
            )?;
            return Ok(3);
        }
    };
    emit(&json!({"event":"choice_kernel","fiber":fiber}))?;
    let prepared = match transfer::prepare(&fiber, &cell.source_target, cell.target_arm) {
        Ok(base) => base,
        Err(error) => {
            emit(
                &json!({"event":"setup_failure","stage":"target_base","error":error,
                "cold_seconds_diagnostic":cold.elapsed().as_secs_f64()}),
            )?;
            return Ok(3);
        }
    };
    let reference_fb = prepared.factor_base()?;
    let reference_setup_seconds = setup.elapsed().as_secs_f64();
    emit(
        &json!({"event":"choice_prepared","base":prepared,"factor_base_index_work":reference_fb.cost,
        "setup_seconds_diagnostic":reference_setup_seconds}),
    )?;
    let (fb, selected_v, selected_tower) = if cell.plain_control {
        let plain_setup = Instant::now();
        let plain = match build_plain_control(
            &prepared.public_target.curve(),
            prepared.public_target.order,
            reference_fb.columns,
            cell.plain_seed,
            cell.max_plain_x_trials,
        ) {
            Ok(base) => base,
            Err(error) => {
                emit(
                    &json!({"event":"setup_failure","stage":"plain_base","error":error.message,
                    "partial_cost":error.cost,"reference_columns":reference_fb.columns,
                    "reference_setup_seconds_diagnostic":reference_setup_seconds,
                    "cold_seconds_diagnostic":cold.elapsed().as_secs_f64()}),
                )?;
                return Ok(3);
            }
        };
        let plain_fb = plain.factor_base()?;
        if plain_fb.columns != reference_fb.columns || plain.tower.is_some() {
            return Err("plain control lost its exact usable-column match".into());
        }
        emit(
            &json!({"event":"choice_plain","base":plain,"seed":cell.plain_seed,
            "max_x_trials":cell.max_plain_x_trials,"reference_columns":reference_fb.columns,
            "reference_setup_seconds_diagnostic":reference_setup_seconds,
            "plain_setup_seconds_diagnostic":plain_setup.elapsed().as_secs_f64(),
            "factor_base_index_work":plain_fb.cost}),
        )?;
        (plain_fb, plain.v, None)
    } else {
        (reference_fb, prepared.v.clone(), prepared.tower.clone())
    };
    let setup_seconds = setup.elapsed().as_secs_f64();
    let remaining = cell
        .execution_seconds
        .saturating_sub(cold.elapsed().as_secs());
    if remaining == 0 {
        emit(
            &json!({"event":"setup_failure","stage":"cold_deadline","error":"setup exhausted cell deadline",
            "cold_seconds_diagnostic":cold.elapsed().as_secs_f64()}),
        )?;
        return Ok(2);
    }
    let mut limits = cell.collection.clone();
    limits.max_seconds = limits.max_seconds.min(remaining);
    let result = collector::collect(
        &prepared.public_target,
        &fb,
        &selected_v,
        selected_tower.as_ref(),
        cell.arity,
        cell.presentation,
        cell.query_seed,
        &limits,
        &mut emit,
    )?;
    let original_scalar_replay = match result.recovered_scalar {
        Some(d) => {
            cell.source_target
                .replay_curve()
                .scalar(cell.source_target.generator, d)?
                == cell.source_target.target
        }
        None => false,
    };
    if result.recovery_replayed != original_scalar_replay {
        return Err("IC scalar failed original-target independent replay".into());
    }
    emit(
        &json!({"event":"cell_summary","status":result.status,"recovered":result.recovery_replayed,
        "collection":result,"setup_seconds_diagnostic":setup_seconds,
        "reference_setup_seconds_diagnostic":reference_setup_seconds,
        "plain_control":cell.plain_control,"selected_columns":fb.columns,
        "original_scalar_replay":original_scalar_replay,
        "cold_seconds_diagnostic":cold.elapsed().as_secs_f64(),"timing_admitted":false}),
    )?;
    Ok(if result.status == "oracle_error" {
        1
    } else if result.recovery_replayed {
        0
    } else {
        2
    })
}

fn panel(directory: &str) -> Result<(), String> {
    let directory = Path::new(directory);
    if directory.exists() {
        return Err("panel destination must not already exist".into());
    }
    fs::create_dir_all(directory).map_err(|e| e.to_string())?;
    let mut cells = Vec::new();
    for round in 0..5 {
        let base = ChoiceCell {
            schema: "prime31-isogeny-choice.cell/v1".into(),
            cell_id: String::new(),
            method: Method::Ic,
            source_target: source_target(),
            kernel: KernelChoice::AplusCyclic,
            coset_scalar: 1,
            target_arm: TargetArm::Original,
            arity: 3,
            presentation: Presentation::Dense,
            plain_control: false,
            plain_seed: plain_seed(),
            max_plain_x_trials: plain_max_trials(),
            query_seed: 4188,
            round,
            collection: CollectionLimits {
                max_queries: 64,
                max_seconds: 110,
                oracle: OracleLimits {
                    max_degree: 1024,
                    max_pairs_per_step: 32,
                    timeout_ms: 2000,
                    max_solutions: 4096,
                    max_signed_lift_attempts: 1_000_000,
                    stop_after_first: true,
                    max_variables: 48,
                    max_input_terms: 100_000,
                    domain_roots: DomainRoots::Auto,
                    sibling_batch: true,
                },
            },
            execution_seconds: 120,
            memory_mib: 4096,
            rho_max_steps: 2_000_000,
            rho_seed: 4188,
        };
        let mut ic_cells = Vec::new();
        for kernel in [
            KernelChoice::AplusCyclic,
            KernelChoice::AminusH0,
            KernelChoice::AminusH1,
            KernelChoice::AminusH2,
        ] {
            for target_arm in [
                TargetArm::Original,
                TargetArm::IsogenousNative,
                TargetArm::IsogenousTransported,
            ] {
                for presentation in [
                    Presentation::Dense,
                    Presentation::Tower,
                    Presentation::Tower4,
                ] {
                    if target_arm == TargetArm::IsogenousTransported
                        && presentation != Presentation::Dense
                    {
                        continue;
                    }
                    let mut cell = base.clone();
                    cell.kernel = kernel;
                    cell.target_arm = target_arm;
                    cell.presentation = presentation;
                    ic_cells.push(cell);
                }
                let mut plain = base.clone();
                plain.kernel = kernel;
                plain.target_arm = target_arm;
                plain.presentation = Presentation::Dense;
                plain.plain_control = true;
                ic_cells.push(plain);
            }
        }
        let mut rho_cells = Vec::new();
        for target_arm in [TargetArm::Original, TargetArm::IsogenousNative] {
            for method in [Method::RhoNegation, Method::RhoV6, Method::RhoFast] {
                let mut cell = base.clone();
                cell.method = method;
                cell.target_arm = target_arm;
                rho_cells.push(cell);
            }
        }
        if ic_cells.len() != 40 || rho_cells.len() != 6 {
            return Err("internal registered panel cardinality changed".into());
        }
        // Distribute rho controls through the IC configurations, then rotate
        // each round by a fixed offset and reverse odd rounds. The exact order
        // is recorded; repeated rounds still use identical workloads/seeds.
        let mut round_cells = Vec::new();
        let mut rho = rho_cells.into_iter();
        for (i, cell) in ic_cells.into_iter().enumerate() {
            round_cells.push(cell);
            if (i + 1) % 6 == 0 {
                if let Some(reference) = rho.next() {
                    round_cells.push(reference);
                }
            }
        }
        round_cells.extend(rho);
        round_cells.rotate_left((9 * round as usize) % 46);
        if round & 1 == 1 {
            round_cells.reverse();
        }
        cells.extend(round_cells);
    }
    let mut index = Vec::new();
    for (i, mut cell) in cells.into_iter().enumerate() {
        cell.cell_id = format!("p31-isogeny-v1-{i:03}");
        let name = format!("{}.json", cell.cell_id);
        let bytes = serde_json::to_vec_pretty(&cell).map_err(|e| e.to_string())?;
        fs::write(directory.join(&name), &bytes).map_err(|e| e.to_string())?;
        index.push(json!({"file":name,"sha256":hash(&bytes),"position":i,"round_position":i%46,"round":cell.round,
            "method":cell.method,"kernel":cell.kernel,"target_arm":cell.target_arm,"presentation":cell.presentation,
            "plain_control":cell.plain_control,"plain_seed":cell.plain_seed,"max_plain_x_trials":cell.max_plain_x_trials}));
    }
    let manifest = json!({"schema":"prime31-isogeny-choice.panel/v1","status":"draft_unadmitted",
        "planned":index.len(),"attempted":0,"completed":0,"rounds":5,
        "planned_ic":200,"planned_auxiliary_ic":140,"planned_plain_ic":60,"planned_rho":30,
        "order_rule":"40 IC cells in kernel/target/presentation order with a matched plain after each target group; insert successive rho after each 6 IC; rotate left by9*round mod46; reverse odd rounds",
        "execute_in_recorded_cells_order":true,
        "same_workload_rounds_are_timing_replicates_not_independent_samples":true,
        "warmup_and_AA_admission":"required separately before any timing claim",
        "cells":index});
    fs::write(
        directory.join("index.json"),
        serde_json::to_vec_pretty(&manifest).unwrap(),
    )
    .map_err(|e| e.to_string())?;
    emit(&json!({"event":"panel_prepared","cells":manifest["planned"],"experiment_executed":false}))
}

fn main_result() -> Result<i32, String> {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("run") if args.len() == 3 => run(&args[2]),
        Some("panel") if args.len() == 3 => { panel(&args[2])?; Ok(0) },
        Some("validate-cell") if args.len() == 3 => { let (c, _) = read_cell(&args[2])?;
            emit(&json!({"cell_id":c.cell_id,"input_valid":true,"experiment_executed":false}))?; Ok(0) },
        Some("source-manifest") => { println!("{SOURCE_MANIFEST}"); Ok(0) },
        Some("source-check") if args.len() == 3 => {
            emit(&prime31_aux_matched::provenance::check_sources(Path::new(&args[2]))?)?; Ok(0)
        }
        Some("replay") if args.len() == 3 => {
            let file = fs::File::open(&args[2]).map_err(|e| e.to_string())?;
            emit(&prime31_aux_matched::audit::audit_jsonl(io::BufReader::new(file))?)?; Ok(0)
        }
        _ => Err("usage: prime31-isogeny-choices run|panel|validate-cell|source-check|replay PATH | source-manifest".into()),
    }
}
fn main() {
    match main_result() {
        Ok(code) => std::process::exit(code),
        Err(error) => {
            let _ = emit(&json!({"event":"driver_error","error":error}));
            std::process::exit(1);
        }
    }
}
