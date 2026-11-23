//! Correctness-only msolve cross-check for the shared-coefficient predicate.
//!
//! msolve computes solutions over an algebraic closure. This runner therefore
//! parses its rational parametrization, extracts only base-field points, and
//! replays every recovered assignment against the canonical IR. It records no
//! timing and does not treat an msolve empty result as an independently
//! certified UNSAT proof.

use crate::msolve::{extract_projected_models, ExtractedModels};
use crate::{
    build_ir, candidate_m3_models, on_curve, oracle_m3_models, point_add, pow_mod, CanonicalIr,
    InstanceSpec, Point,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::ffi::OsString;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus, Stdio};
use std::thread;
use std::time::{Duration, Instant};

pub const E1_MSOLVE_SCHEMA: &str = "prime-shared-coefficient.e1-msolve-crosscheck/v1";
const PROCESS_DEADLINE: Duration = Duration::from_secs(300);
const MSOLVE_VERSION: &str = "0.11.0";
const MSOLVE_COMMIT: &str = "cc044551f8e202d00691990fd3fd0b1226a88ea2";
const MSOLVE_SHA256: &str = "7548742eb77d5f235dce7f20ae94343926f36e93edb5f371d4b11c283adb2b3c";

const M3: &str =
    include_str!("../../../prime_one_shot_20261029/prototype/exports/m3_t4_remainder.witness.json");
const M5: &str =
    include_str!("../../../prime_one_shot_20261029/prototype/exports/m5_t4_remainder.witness.json");
const M7: &str =
    include_str!("../../../prime_one_shot_20261029/prototype/exports/m7_t4_remainder.witness.json");

#[derive(Clone, Debug, Serialize)]
pub struct MsolveToolReceipt {
    pub id: String,
    pub version: String,
    pub source_commit: String,
    pub path: String,
    pub sha256: String,
    pub argv_template: Vec<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct MsolveAttemptReceipt {
    pub status: String,
    pub canonical_ir_sha256: String,
    pub input_sha256: String,
    pub input_bytes: u64,
    pub solver_exit: i32,
    pub solver_stdout_sha256: String,
    pub solver_stderr_sha256: String,
    pub output_sha256: String,
    pub output_bytes: u64,
    pub expected_projected_models: Vec<Vec<u64>>,
    pub extraction: ExtractedModels,
    pub exact_projected_set_match: bool,
    pub unsat_certificate_checked: bool,
}

#[derive(Clone, Debug, Serialize)]
pub struct MsolveTargetReceipt {
    pub target: [u64; 2],
    pub candidate_oracle_match: bool,
    pub reference_models: Vec<Vec<u64>>,
    pub attempt: MsolveAttemptReceipt,
}

#[derive(Clone, Debug, Serialize)]
#[allow(clippy::struct_excessive_bools)]
pub struct MsolveFixedControlReceipt {
    pub fixture: String,
    pub arity: usize,
    pub mutation: String,
    pub known_assignment_canonical_replay: bool,
    pub known_assignment_group_replay: bool,
    pub positive_exact_full_assignment: bool,
    pub mutation_has_no_base_field_assignment: bool,
    pub positive: MsolveAttemptReceipt,
    pub mutation_negative: MsolveAttemptReceipt,
}

#[derive(Clone, Debug, Serialize)]
#[allow(clippy::struct_excessive_bools)]
pub struct MsolveParserControlReceipt {
    pub valid_finite_result_accepted: bool,
    pub missing_terminator_rejected: bool,
    pub trailing_token_rejected: bool,
    pub duplicate_name_rejected: bool,
    pub wrong_characteristic_rejected: bool,
    pub zero_denominator_rejected: bool,
    pub unexpected_extra_variable_rejected: bool,
    pub positive_dimensional_rejected: bool,
}

#[derive(Clone, Debug, Serialize)]
pub struct E1MsolveReport {
    pub schema: String,
    pub kind: String,
    pub source_revision: String,
    pub runner_sha256: String,
    pub timing_collected: bool,
    pub independent_unsat_certificate_available: bool,
    pub closes_e1_unsat_gate: bool,
    pub tool: MsolveToolReceipt,
    pub parser_controls: MsolveParserControlReceipt,
    pub targets: Vec<MsolveTargetReceipt>,
    pub fixed_controls: Vec<MsolveFixedControlReceipt>,
    pub target_count: usize,
    pub sat_target_count: usize,
    pub empty_target_count: usize,
    pub exact_target_matches: usize,
    pub expected_target_matches: usize,
    pub fixed_control_matches: usize,
    pub expected_fixed_control_matches: usize,
    pub verdict: String,
    pub open_obligations: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
struct LegacyWitness {
    p: u64,
    a: u64,
    b: u64,
    m: usize,
    t: usize,
    terminal_c: u64,
    target: [u64; 2],
    summands: Vec<[u64; 2]>,
    assignment: BTreeMap<String, u64>,
}

#[derive(Debug)]
struct CommandReceipt {
    status: ExitStatus,
    timed_out: bool,
}

fn sha256_file(path: &Path) -> Result<String, String> {
    let mut file = File::open(path).map_err(|error| format!("open {}: {error}", path.display()))?;
    let mut digest = Sha256::new();
    let mut buffer = vec![0_u8; 64 * 1024].into_boxed_slice();
    loop {
        let read = file
            .read(&mut buffer)
            .map_err(|error| format!("read {}: {error}", path.display()))?;
        if read == 0 {
            break;
        }
        digest.update(&buffer[..read]);
    }
    Ok(format!("{:x}", digest.finalize()))
}

fn write_new(path: &Path, bytes: &[u8]) -> Result<(), String> {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| format!("create {}: {error}", path.display()))?;
    file.write_all(bytes)
        .map_err(|error| format!("write {}: {error}", path.display()))
}

fn execute(
    program: &Path,
    arguments: &[OsString],
    stdout_path: &Path,
    stderr_path: &Path,
) -> Result<CommandReceipt, String> {
    let stdout_file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(stdout_path)
        .map_err(|error| format!("create {}: {error}", stdout_path.display()))?;
    let stderr_file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(stderr_path)
        .map_err(|error| format!("create {}: {error}", stderr_path.display()))?;
    let mut child = Command::new(program)
        .args(arguments)
        .stdin(Stdio::null())
        .stdout(Stdio::from(stdout_file))
        .stderr(Stdio::from(stderr_file))
        .spawn()
        .map_err(|error| format!("spawn {}: {error}", program.display()))?;
    let started = Instant::now();
    let (status, timed_out) = loop {
        if let Some(status) = child
            .try_wait()
            .map_err(|error| format!("wait for {}: {error}", program.display()))?
        {
            break (status, false);
        }
        if started.elapsed() >= PROCESS_DEADLINE {
            child
                .kill()
                .map_err(|error| format!("kill timed-out {}: {error}", program.display()))?;
            let status = child
                .wait()
                .map_err(|error| format!("reap timed-out {}: {error}", program.display()))?;
            break (status, true);
        }
        thread::sleep(Duration::from_millis(10));
    };
    Ok(CommandReceipt { status, timed_out })
}

fn exit_code(status: ExitStatus) -> i32 {
    status.code().unwrap_or(-1)
}

fn require_source_revision(revision: &str) -> Result<(), String> {
    if revision.len() != 40
        || !revision
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Err(
            "source revision must be exactly 40 lowercase hexadecimal characters".to_owned(),
        );
    }
    Ok(())
}

fn tool_receipt(path: &Path) -> Result<(PathBuf, MsolveToolReceipt), String> {
    let canonical = fs::canonicalize(path)
        .map_err(|error| format!("canonicalize msolve {}: {error}", path.display()))?;
    let observed = sha256_file(&canonical)?;
    if observed != MSOLVE_SHA256 {
        return Err(format!(
            "msolve binary hash mismatch: expected {MSOLVE_SHA256}, observed {observed}"
        ));
    }
    let receipt = MsolveToolReceipt {
        id: "msolve-f4-base-field-crosscheck".to_owned(),
        version: MSOLVE_VERSION.to_owned(),
        source_commit: MSOLVE_COMMIT.to_owned(),
        path: canonical.display().to_string(),
        sha256: observed,
        argv_template: vec![
            "-t".to_owned(),
            "1".to_owned(),
            "-c".to_owned(),
            "2".to_owned(),
            "-d".to_owned(),
            "4".to_owned(),
            "--random-seed".to_owned(),
            "0".to_owned(),
            "-f".to_owned(),
            "INPUT".to_owned(),
            "-o".to_owned(),
            "OUTPUT".to_owned(),
        ],
    };
    Ok((canonical, receipt))
}

fn solver_arguments(input: &Path, output: &Path) -> Vec<OsString> {
    vec![
        "-t".into(),
        "1".into(),
        "-c".into(),
        "2".into(),
        "-d".into(),
        "4".into(),
        "--random-seed".into(),
        "0".into(),
        "-f".into(),
        input.as_os_str().to_owned(),
        "-o".into(),
        output.as_os_str().to_owned(),
    ]
}

fn attempt_status(extraction: &ExtractedModels) -> &'static str {
    if extraction.algebraic_closure_empty {
        "algebraic_closure_empty_without_certificate"
    } else if extraction.projected_models.is_empty() {
        "finite_algebraic_closure_no_base_field_points"
    } else {
        "base_field_models_extracted"
    }
}

fn write_receipt(path: &Path, receipt: &MsolveAttemptReceipt) -> Result<(), String> {
    let bytes = serde_json::to_vec_pretty(receipt)
        .map_err(|error| format!("serialize msolve attempt: {error}"))?;
    write_new(path, &bytes)
}

fn run_attempt(
    msolve: &Path,
    directory: &Path,
    ir: &CanonicalIr,
    input_text: &str,
    expected_models: &[Vec<u64>],
) -> Result<MsolveAttemptReceipt, String> {
    fs::create_dir_all(directory)
        .map_err(|error| format!("create {}: {error}", directory.display()))?;
    let input = directory.join("input.ms");
    let output = directory.join("output.ms");
    let stdout = directory.join("solver.stdout");
    let stderr = directory.join("solver.stderr");
    write_new(&input, input_text.as_bytes())?;
    let command = execute(msolve, &solver_arguments(&input, &output), &stdout, &stderr)?;
    if command.timed_out {
        return Err(format!(
            "msolve exceeded the 300-second correctness deadline for {}",
            input.display()
        ));
    }
    if !command.status.success() {
        return Err(format!(
            "msolve failed for {}: exit={}, stdout={}, stderr={}",
            input.display(),
            exit_code(command.status),
            sha256_file(&stdout)?,
            sha256_file(&stderr)?
        ));
    }
    let output_text = fs::read_to_string(&output)
        .map_err(|error| format!("read {} as UTF-8: {error}", output.display()))?;
    let extraction = extract_projected_models(&output_text, ir)?;
    let exact_projected_set_match = extraction.projected_models == expected_models;
    let receipt = MsolveAttemptReceipt {
        status: attempt_status(&extraction).to_owned(),
        canonical_ir_sha256: ir.sha256()?,
        input_sha256: sha256_file(&input)?,
        input_bytes: fs::metadata(&input)
            .map_err(|error| format!("stat {}: {error}", input.display()))?
            .len(),
        solver_exit: exit_code(command.status),
        solver_stdout_sha256: sha256_file(&stdout)?,
        solver_stderr_sha256: sha256_file(&stderr)?,
        output_sha256: sha256_file(&output)?,
        output_bytes: fs::metadata(&output)
            .map_err(|error| format!("stat {}: {error}", output.display()))?
            .len(),
        expected_projected_models: expected_models.to_vec(),
        extraction,
        exact_projected_set_match,
        unsat_certificate_checked: false,
    };
    write_receipt(&directory.join("attempt.json"), &receipt)?;
    if !receipt.exact_projected_set_match {
        return Err(format!(
            "msolve projected model set mismatch for {}",
            input.display()
        ));
    }
    Ok(receipt)
}

fn f17_targets() -> Vec<Point> {
    let base = InstanceSpec {
        modulus: 17,
        curve_a: 2,
        curve_b: 2,
        arity: 3,
        depth: 3,
        terminal_c: 1,
        target_x: 0,
        target_y: 0,
    };
    (0..17)
        .flat_map(|x| (0..17).map(move |y| Point { x, y }))
        .filter(|point| on_curve(*point, base))
        .collect()
}

const fn fixture_spec(witness: &LegacyWitness) -> InstanceSpec {
    InstanceSpec {
        modulus: witness.p,
        curve_a: witness.a,
        curve_b: witness.b,
        arity: witness.m,
        depth: witness.t,
        terminal_c: witness.terminal_c,
        target_x: witness.target[0],
        target_y: witness.target[1],
    }
}

fn fixture_group_replay(witness: &LegacyWitness) -> bool {
    let spec = fixture_spec(witness);
    let target = Point {
        x: witness.target[0],
        y: witness.target[1],
    };
    if witness.summands.len() != witness.m || !on_curve(target, spec) {
        return false;
    }
    let mut x_values = BTreeSet::new();
    let mut sum = None;
    for coordinates in &witness.summands {
        let point = Point {
            x: coordinates[0],
            y: coordinates[1],
        };
        if point.x == target.x
            || !x_values.insert(point.x)
            || !on_curve(point, spec)
            || pow_mod(point.x, 1_u64 << witness.t, witness.p) != witness.terminal_c
        {
            return false;
        }
        sum = point_add(sum, Some(point), spec);
    }
    sum == Some(target)
}

fn project_assignment(
    ir: &CanonicalIr,
    assignment: &BTreeMap<String, u64>,
) -> Result<Vec<u64>, String> {
    ir.projected_variables
        .iter()
        .map(|id| {
            let name = &ir.variables[*id as usize].name;
            assignment
                .get(name)
                .copied()
                .ok_or_else(|| format!("assignment omits projected variable {name}"))
        })
        .collect()
}

fn fixed_input(ir: &CanonicalIr, assignment: &BTreeMap<String, u64>) -> Result<String, String> {
    ir.validate()?;
    if assignment.len() != ir.variables.len() {
        return Err(format!(
            "fixed assignment has {} entries; expected {}",
            assignment.len(),
            ir.variables.len()
        ));
    }
    let modulus = ir
        .spec
        .modulus
        .parse::<u64>()
        .map_err(|_| "invalid canonical IR modulus".to_owned())?;
    let mut output = ir.to_msolve()?;
    if output.pop() != Some('\n') {
        return Err("canonical msolve export must end with a newline".to_owned());
    }
    output.push_str(",\n");
    for (index, variable) in ir.variables.iter().enumerate() {
        let value = assignment
            .get(&variable.name)
            .copied()
            .ok_or_else(|| format!("fixed assignment omits {}", variable.name))?;
        if value >= modulus {
            return Err(format!("noncanonical fixed value for {}", variable.name));
        }
        let constant = if value == 0 { 0 } else { modulus - value };
        output.push_str(&variable.name);
        if constant != 0 {
            output.push('+');
            output.push_str(&constant.to_string());
        }
        output.push_str(if index + 1 == ir.variables.len() {
            "\n"
        } else {
            ",\n"
        });
    }
    Ok(output)
}

fn synthetic_fixed_output(
    ir: &CanonicalIr,
    assignment: &BTreeMap<String, u64>,
) -> Result<String, String> {
    let order = ir
        .variables
        .iter()
        .map(|variable| variable.name.clone())
        .collect::<Vec<_>>();
    synthetic_fixed_output_in_order(ir, assignment, &order)
}

fn synthetic_fixed_output_in_order(
    ir: &CanonicalIr,
    assignment: &BTreeMap<String, u64>,
    order: &[String],
) -> Result<String, String> {
    let modulus = ir
        .spec
        .modulus
        .parse::<u64>()
        .map_err(|_| "invalid canonical IR modulus".to_owned())?;
    let mut names = order
        .iter()
        .map(|name| format!("'{name}'"))
        .collect::<Vec<_>>();
    names.push("'A'".to_owned());
    let mut linear = vec!["0"; order.len()];
    linear.push("1");
    let coordinates = order
        .iter()
        .map(|name| {
            let value = assignment
                .get(name)
                .copied()
                .ok_or_else(|| format!("synthetic assignment omits {name}"))?;
            let negated = if value == 0 { 0 } else { modulus - value };
            Ok(if negated == 0 {
                "[[-1,[0]]]".to_owned()
            } else {
                format!("[[0,[{negated}]]]")
            })
        })
        .collect::<Result<Vec<_>, String>>()?
        .join(",");
    Ok(format!(
        "[0,[{modulus},{},1,[{}],[{}],[1,[[1,[{},1]],[0,[1]],[{}]]]]]:\n",
        order.len() + 1,
        names.join(","),
        linear.join(","),
        modulus - 1,
        coordinates
    ))
}

fn replace_required(input: &str, from: &str, to: &str) -> Result<String, String> {
    if !input.contains(from) {
        return Err(format!("parser-control source omits required token {from}"));
    }
    Ok(input.replacen(from, to, 1))
}

fn parser_controls() -> Result<MsolveParserControlReceipt, String> {
    let witness: LegacyWitness =
        serde_json::from_str(M3).map_err(|error| format!("parse m3 witness: {error}"))?;
    let ir = build_ir(fixture_spec(&witness))?;
    let valid = synthetic_fixed_output(&ir, &witness.assignment)?;
    let expected = vec![witness.assignment];
    let valid_finite_result_accepted = extract_projected_models(&valid, &ir)
        .is_ok_and(|models| models.canonical_assignments == expected);
    let missing = valid.trim().trim_end_matches(':');
    let trailing = format!("{valid}trailing");
    let duplicate = replace_required(&valid, "'a0'", "'a1'")?;
    let wrong_characteristic = replace_required(&valid, "[0,[17,", "[0,[19,")?;
    let zero_denominator = replace_required(&valid, "[0,[1]]", "[-1,[0]]")?;
    let unexpected_extra = replace_required(&valid, "'A'", "'B'")?;
    Ok(MsolveParserControlReceipt {
        valid_finite_result_accepted,
        missing_terminator_rejected: extract_projected_models(missing, &ir).is_err(),
        trailing_token_rejected: extract_projected_models(&trailing, &ir).is_err(),
        duplicate_name_rejected: extract_projected_models(&duplicate, &ir).is_err(),
        wrong_characteristic_rejected: extract_projected_models(&wrong_characteristic, &ir)
            .is_err(),
        zero_denominator_rejected: extract_projected_models(&zero_denominator, &ir).is_err(),
        unexpected_extra_variable_rejected: extract_projected_models(&unexpected_extra, &ir)
            .is_err(),
        positive_dimensional_rejected: extract_projected_models("[1,3,-1,[]]:\n", &ir).is_err(),
    })
}

const fn parser_controls_pass(receipt: &MsolveParserControlReceipt) -> bool {
    receipt.valid_finite_result_accepted
        && receipt.missing_terminator_rejected
        && receipt.trailing_token_rejected
        && receipt.duplicate_name_rejected
        && receipt.wrong_characteristic_rejected
        && receipt.zero_denominator_rejected
        && receipt.unexpected_extra_variable_rejected
        && receipt.positive_dimensional_rejected
}

fn run_targets(msolve: &Path, root: &Path) -> Result<Vec<MsolveTargetReceipt>, String> {
    let mut receipts = Vec::new();
    for target in f17_targets() {
        let spec = InstanceSpec {
            modulus: 17,
            curve_a: 2,
            curve_b: 2,
            arity: 3,
            depth: 3,
            terminal_c: 1,
            target_x: target.x,
            target_y: target.y,
        };
        let candidate = candidate_m3_models(spec)?;
        let oracle = oracle_m3_models(spec)?;
        if candidate != oracle {
            return Err(format!(
                "pre-solver candidate/oracle mismatch at ({}, {})",
                target.x, target.y
            ));
        }
        let ir = build_ir(spec)?;
        let reference_models = oracle.into_iter().collect::<Vec<_>>();
        let directory = root.join(format!("x{:02}-y{:02}", target.x, target.y));
        let attempt = run_attempt(msolve, &directory, &ir, &ir.to_msolve()?, &reference_models)?;
        receipts.push(MsolveTargetReceipt {
            target: [target.x, target.y],
            candidate_oracle_match: true,
            reference_models,
            attempt,
        });
    }
    Ok(receipts)
}

fn run_fixed_controls(
    msolve: &Path,
    root: &Path,
) -> Result<Vec<MsolveFixedControlReceipt>, String> {
    let fixtures = [("m3-t4", M3), ("m5-t4", M5), ("m7-t4", M7)];
    let mut receipts = Vec::new();
    for (name, input) in fixtures {
        let witness: LegacyWitness = serde_json::from_str(input)
            .map_err(|error| format!("parse {name} witness: {error}"))?;
        let ir = build_ir(fixture_spec(&witness))?;
        let canonical_replay = ir.failed_equations(&witness.assignment)?.is_empty();
        let group_replay = fixture_group_replay(&witness);
        if !canonical_replay || !group_replay {
            return Err(format!("{name} source fixture fails replay"));
        }
        let expected_positive = vec![project_assignment(&ir, &witness.assignment)?];
        let positive = run_attempt(
            msolve,
            &root.join(name).join("positive"),
            &ir,
            &fixed_input(&ir, &witness.assignment)?,
            &expected_positive,
        )?;
        let positive_exact_full_assignment =
            positive.extraction.canonical_assignments == vec![witness.assignment.clone()];
        if !positive_exact_full_assignment {
            return Err(format!(
                "{name} positive did not recover the exact fixed assignment"
            ));
        }

        let mut mutation = witness.assignment.clone();
        let a0 = mutation
            .get("a0")
            .copied()
            .ok_or_else(|| format!("{name} witness omits a0"))?;
        mutation.insert("a0".to_owned(), (a0 + 1) % witness.p);
        if ir.failed_equations(&mutation)?.is_empty() {
            return Err(format!(
                "{name} deterministic mutation unexpectedly satisfies the IR"
            ));
        }
        let mutation_negative = run_attempt(
            msolve,
            &root.join(name).join("a0-plus-one"),
            &ir,
            &fixed_input(&ir, &mutation)?,
            &[],
        )?;
        let mutation_has_no_base_field_assignment = mutation_negative
            .extraction
            .canonical_assignments
            .is_empty();
        if !mutation_has_no_base_field_assignment {
            return Err(format!("{name} mutation produced a base-field assignment"));
        }
        receipts.push(MsolveFixedControlReceipt {
            fixture: name.to_owned(),
            arity: witness.m,
            mutation: "a0 := (a0 + 1) mod 17; all canonical fields fixed".to_owned(),
            known_assignment_canonical_replay: canonical_replay,
            known_assignment_group_replay: group_replay,
            positive_exact_full_assignment,
            mutation_has_no_base_field_assignment,
            positive,
            mutation_negative,
        });
    }
    Ok(receipts)
}

/// Execute the fixed, correctness-only msolve base-field cross-check.
///
/// # Errors
///
/// Fails closed on any tool hash, process, parser, base-field extraction,
/// canonical replay, exact model-set, fixed-control, or write-once mismatch.
#[allow(clippy::too_many_lines)]
pub fn run_e1_msolve_panel(
    msolve: &Path,
    output: &Path,
    source_revision: &str,
) -> Result<E1MsolveReport, String> {
    require_source_revision(source_revision)?;
    if output.exists() {
        return Err(format!(
            "E1 msolve output directory already exists: {}",
            output.display()
        ));
    }
    fs::create_dir_all(output).map_err(|error| format!("create {}: {error}", output.display()))?;
    let (msolve, tool) = tool_receipt(msolve)?;
    let parser_controls = parser_controls()?;
    if !parser_controls_pass(&parser_controls) {
        return Err("one or more strict msolve parser controls failed".to_owned());
    }
    let targets_root = output.join("targets");
    fs::create_dir(&targets_root)
        .map_err(|error| format!("create {}: {error}", targets_root.display()))?;
    let targets = run_targets(&msolve, &targets_root)?;
    if targets.len() != 18 {
        return Err(format!(
            "expected 18 F17 affine targets, found {}",
            targets.len()
        ));
    }
    let fixed_root = output.join("fixed-controls");
    fs::create_dir(&fixed_root)
        .map_err(|error| format!("create {}: {error}", fixed_root.display()))?;
    let fixed_controls = run_fixed_controls(&msolve, &fixed_root)?;
    let target_count = targets.len();
    let sat_target_count = targets
        .iter()
        .filter(|target| !target.reference_models.is_empty())
        .count();
    let exact_target_matches = targets
        .iter()
        .filter(|target| target.attempt.exact_projected_set_match)
        .count();
    let fixed_control_matches = fixed_controls
        .iter()
        .filter(|control| {
            control.known_assignment_canonical_replay
                && control.known_assignment_group_replay
                && control.positive_exact_full_assignment
                && control.mutation_has_no_base_field_assignment
                && control.positive.exact_projected_set_match
                && control.mutation_negative.exact_projected_set_match
        })
        .count();
    if exact_target_matches != target_count || fixed_control_matches != fixed_controls.len() {
        return Err("E1 msolve aggregate correctness invariant failed".to_owned());
    }
    let runner = std::env::current_exe().map_err(|error| format!("resolve runner: {error}"))?;
    let report = E1MsolveReport {
        schema: E1_MSOLVE_SCHEMA.to_owned(),
        kind: "external representation/model cross-check; no timing or speed claim".to_owned(),
        source_revision: source_revision.to_owned(),
        runner_sha256: sha256_file(&runner)?,
        timing_collected: false,
        independent_unsat_certificate_available: false,
        closes_e1_unsat_gate: false,
        tool,
        parser_controls,
        targets,
        fixed_controls,
        target_count,
        sat_target_count,
        empty_target_count: target_count - sat_target_count,
        exact_target_matches,
        expected_target_matches: target_count,
        fixed_control_matches,
        expected_fixed_control_matches: 3,
        verdict: "msolve_base_field_crosscheck_passed_no_unsat_certificate".to_owned(),
        open_obligations: vec![
            "independently checked UNSAT certificates for every empty target".to_owned(),
            "frozen CaDiCaL/CNF and CryptoMiniSat/CNF-XOR E1 panel".to_owned(),
            "E2 disjoint preflight before any performance claim".to_owned(),
        ],
    };
    let encoded = serde_json::to_vec_pretty(&report)
        .map_err(|error| format!("serialize E1 msolve report: {error}"))?;
    write_new(&output.join("e1-msolve-crosscheck.json"), &encoded)?;
    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser_control_bundle_fails_closed() {
        let controls = parser_controls().unwrap();
        assert!(parser_controls_pass(&controls));
    }

    #[test]
    fn fixed_input_adds_every_canonical_assignment_constraint() {
        let witness: LegacyWitness = serde_json::from_str(M3).unwrap();
        let ir = build_ir(fixture_spec(&witness)).unwrap();
        let base = ir.to_msolve().unwrap();
        let fixed = fixed_input(&ir, &witness.assignment).unwrap();
        assert!(fixed.starts_with(&base[..base.len() - 1]));
        assert_eq!(
            fixed.lines().count(),
            base.lines().count() + ir.variables.len()
        );
        assert!(fixed.ends_with('\n'));
    }

    #[test]
    fn source_revision_is_strict() {
        assert!(require_source_revision("0123456789abcdef0123456789abcdef01234567").is_ok());
        assert!(require_source_revision("0123456789ABCDEF0123456789ABCDEF01234567").is_err());
        assert!(require_source_revision("0123").is_err());
    }

    #[test]
    fn reordered_msolve_names_recover_the_same_canonical_assignment() {
        let witness: LegacyWitness = serde_json::from_str(M3).unwrap();
        let ir = build_ir(fixture_spec(&witness)).unwrap();
        let mut order = ir
            .variables
            .iter()
            .map(|variable| variable.name.clone())
            .collect::<Vec<_>>();
        order.reverse();
        let output = synthetic_fixed_output_in_order(&ir, &witness.assignment, &order).unwrap();
        let extracted = extract_projected_models(&output, &ir).unwrap();
        assert_eq!(extracted.canonical_assignments, [witness.assignment]);
    }
}
