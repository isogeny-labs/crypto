//! Checked external-solver gate for the shared-coefficient Boolean lowering.
//!
//! E1 is deliberately a correctness panel.  It records no solver timings and
//! makes no performance claim.

use crate::boolean::{validate_dimacs, BooleanEncoding};
use crate::{
    build_ir, candidate_m3_models, on_curve, oracle_m3_models, point_add, pow_mod, CanonicalIr,
    InstanceSpec, Point,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::ffi::OsString;
use std::fs::{self, File};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus, Stdio};
use std::thread;
use std::time::{Duration, Instant};

pub const E1_SCHEMA: &str = "prime-shared-coefficient.e1-sat-panel/v1";
const SOLVER_DEADLINE: Duration = Duration::from_secs(300);
const CADICAL_COMMIT: &str = "c60730422e758ef1cebe7aeddf2dda31c996bf04";
const LRAT_TRIM_COMMIT: &str = "b30f400f4ee5c32b77ee566a7c006081b521534f";
const CRYPTOMINISAT_COMMIT: &str = "03954f7f7815ea3984774c95a66e7c224cd25952";
const CAKE_XLRUP_REPOSITORY_COMMIT: &str = "18cd6bc17af10a9891c10a846ce966166f9fe0dc";
const CADICAL_SHA256: &str = "5dba1e331369eb5543b7ee099e33d3ab360c4ae34d1c0cf07c3b878fbca33280";
const LRAT_TRIM_SHA256: &str = "e22e1e33818812e1aa6286578ba9cd11a26225f424c0718612784a7562dec895";
const CRYPTOMINISAT_SHA256: &str =
    "96efc097b956e2d8f1982515de595472b17a47a25a0939fce94d923fb0d162c7";
const CAKE_XLRUP_SHA256: &str = "56e2bc4aeddb362d632bf9f7056cfd604afb2f2e5f86d131d1d1dc554ef5b7bc";

const M3: &str =
    include_str!("../../../prime_one_shot_20261029/prototype/exports/m3_t4_remainder.witness.json");
const M5: &str =
    include_str!("../../../prime_one_shot_20261029/prototype/exports/m5_t4_remainder.witness.json");
const M7: &str =
    include_str!("../../../prime_one_shot_20261029/prototype/exports/m7_t4_remainder.witness.json");

#[derive(Clone, Debug, Serialize)]
pub struct ToolReceipt {
    pub id: String,
    pub source_commit: String,
    pub path: String,
    pub sha256: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct AttemptReceipt {
    pub index: usize,
    pub status: String,
    pub input_sha256: String,
    pub input_bytes: u64,
    pub solver_exit: i32,
    pub solver_stdout_sha256: String,
    pub solver_stderr_sha256: String,
    pub projection: Option<Vec<u64>>,
    pub canonical_replay: Option<bool>,
    pub independent_group_replay: Option<bool>,
    pub proof_sha256: Option<String>,
    pub proof_checked: Option<bool>,
    pub truncated_proof_rejected: Option<bool>,
}

#[derive(Clone, Debug, Serialize)]
pub struct BackendReceipt {
    pub backend: String,
    pub representation: String,
    pub expected_models: usize,
    pub observed_models: Vec<Vec<u64>>,
    pub exact_projected_set_match: bool,
    pub attempts: Vec<AttemptReceipt>,
}

#[derive(Clone, Debug, Serialize)]
pub struct TargetReceipt {
    pub target: [u64; 2],
    pub exact_reference_models: Vec<Vec<u64>>,
    pub candidate_oracle_match: bool,
    pub backends: Vec<BackendReceipt>,
}

#[derive(Clone, Debug, Serialize)]
pub struct FixedControlReceipt {
    pub fixture: String,
    pub arity: usize,
    pub mutation: String,
    pub known_assignment_canonical_replay: bool,
    pub known_assignment_group_replay: bool,
    pub backend: String,
    pub positive: AttemptReceipt,
    pub mutation_negative: AttemptReceipt,
}

#[derive(Clone, Debug, Serialize)]
#[allow(clippy::struct_excessive_bools)]
pub struct ParserControlReceipt {
    pub ordinary_cnf_accepted: bool,
    pub cnf_xor_accepted: bool,
    pub truncated_rejected: bool,
    pub trailing_constraint_rejected: bool,
    pub xor_rejected_as_ordinary_cnf: bool,
}

#[derive(Clone, Debug, Serialize)]
pub struct E1SatPanelReport {
    pub schema: String,
    pub kind: String,
    pub source_revision: String,
    pub runner_sha256: String,
    pub timing_collected: bool,
    pub tools: Vec<ToolReceipt>,
    pub parser_controls: ParserControlReceipt,
    pub targets: Vec<TargetReceipt>,
    pub fixed_controls: Vec<FixedControlReceipt>,
    pub target_count: usize,
    pub sat_target_count: usize,
    pub unsat_target_count: usize,
    pub exact_target_backend_matches: usize,
    pub expected_target_backend_matches: usize,
    pub checked_terminal_unsat_proofs: usize,
    pub rejected_truncated_proofs: usize,
    pub verdict: String,
    pub open_backend_obligations: Vec<String>,
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

#[derive(Clone, Copy, Debug)]
enum Backend {
    Cadical,
    CryptoMiniSat,
}

impl Backend {
    const ALL: [Self; 2] = [Self::Cadical, Self::CryptoMiniSat];

    const fn id(self) -> &'static str {
        match self {
            Self::Cadical => "cadical-cnf",
            Self::CryptoMiniSat => "cryptominisat-cnf-xor",
        }
    }

    const fn representation(self) -> &'static str {
        match self {
            Self::Cadical => "ordinary-cnf",
            Self::CryptoMiniSat => "cnf-xor",
        }
    }

    const fn native_xor(self) -> bool {
        matches!(self, Self::CryptoMiniSat)
    }
}

#[derive(Clone, Copy)]
struct ToolPaths<'a> {
    cadical: &'a Path,
    lrat_trim: &'a Path,
    cryptominisat: &'a Path,
    cake_xlrup: &'a Path,
}

#[derive(Debug)]
struct CommandReceipt {
    status: ExitStatus,
    timed_out: bool,
    stdout: String,
    stderr: String,
}

impl<'a> ToolPaths<'a> {
    const fn solver(self, backend: Backend) -> &'a Path {
        match backend {
            Backend::Cadical => self.cadical,
            Backend::CryptoMiniSat => self.cryptominisat,
        }
    }

    const fn checker(self, backend: Backend) -> &'a Path {
        match backend {
            Backend::Cadical => self.lrat_trim,
            Backend::CryptoMiniSat => self.cake_xlrup,
        }
    }
}

fn sha256_bytes(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
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
    if path.exists() {
        return Err(format!("refusing to overwrite {}", path.display()));
    }
    fs::write(path, bytes).map_err(|error| format!("write {}: {error}", path.display()))
}

fn execute(
    program: &Path,
    arguments: &[OsString],
    stdout_path: &Path,
    stderr_path: &Path,
) -> Result<CommandReceipt, String> {
    let stdout_file = File::create(stdout_path)
        .map_err(|error| format!("create {}: {error}", stdout_path.display()))?;
    let stderr_file = File::create(stderr_path)
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
        if started.elapsed() >= SOLVER_DEADLINE {
            child
                .kill()
                .map_err(|error| format!("kill timed-out {}: {error}", program.display()))?;
            let status = child
                .wait()
                .map_err(|error| format!("reap timed-out {}: {error}", program.display()))?;
            break (status, true);
        }
        thread::sleep(Duration::from_millis(20));
    };
    let stdout = fs::read_to_string(stdout_path)
        .map_err(|error| format!("read {}: {error}", stdout_path.display()))?;
    let stderr = fs::read_to_string(stderr_path)
        .map_err(|error| format!("read {}: {error}", stderr_path.display()))?;
    Ok(CommandReceipt {
        status,
        timed_out,
        stdout,
        stderr,
    })
}

fn exit_code(status: ExitStatus) -> i32 {
    status.code().unwrap_or(-1)
}

fn require_source_revision(source_revision: &str) -> Result<(), String> {
    if source_revision.len() != 40
        || !source_revision
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Err("source revision must be 40 lowercase hexadecimal characters".to_owned());
    }
    Ok(())
}

fn tool_receipts(paths: ToolPaths<'_>) -> Result<Vec<ToolReceipt>, String> {
    [
        ("cadical", CADICAL_COMMIT, CADICAL_SHA256, paths.cadical),
        (
            "lrat-trim",
            LRAT_TRIM_COMMIT,
            LRAT_TRIM_SHA256,
            paths.lrat_trim,
        ),
        (
            "cryptominisat",
            CRYPTOMINISAT_COMMIT,
            CRYPTOMINISAT_SHA256,
            paths.cryptominisat,
        ),
        (
            "cake-xlrup",
            CAKE_XLRUP_REPOSITORY_COMMIT,
            CAKE_XLRUP_SHA256,
            paths.cake_xlrup,
        ),
    ]
    .into_iter()
    .map(|(id, commit, expected_sha256, path)| {
        let canonical = fs::canonicalize(path)
            .map_err(|error| format!("canonicalize {}: {error}", path.display()))?;
        let observed_sha256 = sha256_file(&canonical)?;
        if observed_sha256 != expected_sha256 {
            return Err(format!(
                "{id} binary hash mismatch: expected {expected_sha256}, observed {observed_sha256}"
            ));
        }
        Ok(ToolReceipt {
            id: id.to_owned(),
            source_commit: commit.to_owned(),
            path: canonical.display().to_string(),
            sha256: observed_sha256,
        })
    })
    .collect()
}

fn solver_arguments(backend: Backend, input: &Path, proof: &Path) -> Vec<OsString> {
    match backend {
        Backend::Cadical => vec![
            "--lrat".into(),
            "--no-binary".into(),
            input.as_os_str().to_owned(),
            proof.as_os_str().to_owned(),
        ],
        Backend::CryptoMiniSat => vec![
            "--threads".into(),
            "1".into(),
            "--verb".into(),
            "0".into(),
            "--xlrup".into(),
            "1".into(),
            input.as_os_str().to_owned(),
            proof.as_os_str().to_owned(),
        ],
    }
}

fn checker_arguments(backend: Backend, input: &Path, proof: &Path) -> Vec<OsString> {
    match backend {
        Backend::Cadical => vec![input.as_os_str().to_owned(), proof.as_os_str().to_owned()],
        Backend::CryptoMiniSat => vec![
            "--no-binary".into(),
            input.as_os_str().to_owned(),
            proof.as_os_str().to_owned(),
        ],
    }
}

fn solver_status(receipt: &CommandReceipt) -> Result<&'static str, String> {
    if receipt.timed_out {
        return Err("solver exceeded the 300-second correctness deadline".to_owned());
    }
    let code = exit_code(receipt.status);
    if code == 10
        && receipt
            .stdout
            .lines()
            .any(|line| line.trim() == "s SATISFIABLE")
    {
        Ok("sat")
    } else if code == 20
        && receipt
            .stdout
            .lines()
            .any(|line| line.trim() == "s UNSATISFIABLE")
    {
        Ok("unsat")
    } else {
        Err(format!(
            "unaccepted solver result: exit={code}, stdout={}, stderr={}",
            sha256_bytes(receipt.stdout.as_bytes()),
            sha256_bytes(receipt.stderr.as_bytes())
        ))
    }
}

fn checker_accepted(backend: Backend, receipt: &CommandReceipt) -> bool {
    if receipt.timed_out {
        return false;
    }
    match backend {
        Backend::Cadical => {
            exit_code(receipt.status) == 20 && receipt.stdout.contains("s VERIFIED")
        }
        Backend::CryptoMiniSat => {
            receipt.status.success() && receipt.stdout.contains("s VERIFIED UNSAT")
        }
    }
}

fn verify_proof(
    backend: Backend,
    tools: ToolPaths<'_>,
    input: &Path,
    proof: &Path,
    directory: &Path,
) -> Result<(String, bool, bool), String> {
    if !proof.is_file() {
        return Err(format!("solver omitted proof {}", proof.display()));
    }
    let proof_sha256 = sha256_file(proof)?;
    let checker_stdout = directory.join("checker.stdout");
    let checker_stderr = directory.join("checker.stderr");
    let proof_check = execute(
        tools.checker(backend),
        &checker_arguments(backend, input, proof),
        &checker_stdout,
        &checker_stderr,
    )?;
    let proof_valid = checker_accepted(backend, &proof_check);
    if !proof_valid {
        return Err(format!(
            "{} rejected terminal proof {}: exit={}, stdout={}, stderr={}",
            backend.id(),
            proof.display(),
            exit_code(proof_check.status),
            sha256_bytes(proof_check.stdout.as_bytes()),
            sha256_bytes(proof_check.stderr.as_bytes())
        ));
    }

    let truncated = directory.join("proof.truncated");
    write_new(&truncated, &[])?;
    let mutation_stdout = directory.join("checker-truncated.stdout");
    let mutation_stderr = directory.join("checker-truncated.stderr");
    let mutation = execute(
        tools.checker(backend),
        &checker_arguments(backend, input, &truncated),
        &mutation_stdout,
        &mutation_stderr,
    )?;
    let mutation_rejected = !checker_accepted(backend, &mutation);
    if !mutation_rejected {
        return Err(format!(
            "{} accepted an empty truncated proof for {}",
            backend.id(),
            input.display()
        ));
    }
    Ok((proof_sha256, proof_valid, mutation_rejected))
}

fn run_solver(
    backend: Backend,
    tools: ToolPaths<'_>,
    input: &Path,
    directory: &Path,
) -> Result<(CommandReceipt, PathBuf), String> {
    let proof = directory.join(match backend {
        Backend::Cadical => "proof.lrat",
        Backend::CryptoMiniSat => "proof.xlrup",
    });
    let stdout = directory.join("solver.stdout");
    let stderr = directory.join("solver.stderr");
    let receipt = execute(
        tools.solver(backend),
        &solver_arguments(backend, input, &proof),
        &stdout,
        &stderr,
    )?;
    Ok((receipt, proof))
}

fn base_attempt_receipt(
    index: usize,
    status: &str,
    input: &Path,
    solver: &CommandReceipt,
) -> Result<AttemptReceipt, String> {
    Ok(AttemptReceipt {
        index,
        status: status.to_owned(),
        input_sha256: sha256_file(input)?,
        input_bytes: fs::metadata(input)
            .map_err(|error| format!("stat {}: {error}", input.display()))?
            .len(),
        solver_exit: exit_code(solver.status),
        solver_stdout_sha256: sha256_bytes(solver.stdout.as_bytes()),
        solver_stderr_sha256: sha256_bytes(solver.stderr.as_bytes()),
        projection: None,
        canonical_replay: None,
        independent_group_replay: None,
        proof_sha256: None,
        proof_checked: None,
        truncated_proof_rejected: None,
    })
}

fn run_target_backend(
    ir: &CanonicalIr,
    encoding: &BooleanEncoding,
    expected: &BTreeSet<Vec<u64>>,
    backend: Backend,
    tools: ToolPaths<'_>,
    directory: &Path,
) -> Result<BackendReceipt, String> {
    fs::create_dir_all(directory)
        .map_err(|error| format!("create {}: {error}", directory.display()))?;
    let mut blocks = Vec::<Vec<i64>>::new();
    let mut observed = BTreeSet::<Vec<u64>>::new();
    let mut attempts = Vec::new();
    let mut terminal_unsat = false;

    for index in 0..=expected.len() {
        let attempt_directory = directory.join(format!("attempt-{index:02}"));
        fs::create_dir(&attempt_directory)
            .map_err(|error| format!("create {}: {error}", attempt_directory.display()))?;
        let input = attempt_directory.join(if backend.native_xor() {
            "input.xnf"
        } else {
            "input.cnf"
        });
        let rendered = encoding.with_additional_clauses(backend.native_xor(), &blocks)?;
        validate_dimacs(&rendered, backend.native_xor())?;
        write_new(&input, rendered.as_bytes())?;
        let (solver, proof) = run_solver(backend, tools, &input, &attempt_directory)?;
        let status = solver_status(&solver)?;
        let mut receipt = base_attempt_receipt(index, status, &input, &solver)?;
        match status {
            "sat" => {
                let assignment = encoding.decode_solver_model(&solver.stdout)?;
                let canonical_replay = ir.failed_equations(&assignment)?.is_empty();
                if !canonical_replay {
                    return Err(format!(
                        "{} model fails canonical replay for {}",
                        backend.id(),
                        input.display()
                    ));
                }
                let projection = encoding.projected_values(&assignment)?;
                let independent_group_replay = expected.contains(&projection);
                if !independent_group_replay {
                    return Err(format!(
                        "{} produced unexpected projection {:?}",
                        backend.id(),
                        projection
                    ));
                }
                if !observed.insert(projection.clone()) {
                    return Err(format!(
                        "{} repeated blocked projection {:?}",
                        backend.id(),
                        projection
                    ));
                }
                if observed.len() > expected.len() {
                    return Err(format!(
                        "{} produced too many projected models",
                        backend.id()
                    ));
                }
                blocks.push(encoding.projected_blocking_clause(&assignment)?);
                receipt.projection = Some(projection);
                receipt.canonical_replay = Some(canonical_replay);
                receipt.independent_group_replay = Some(independent_group_replay);
            }
            "unsat" => {
                let (proof_sha256, checked, mutation_rejected) =
                    verify_proof(backend, tools, &input, &proof, &attempt_directory)?;
                receipt.proof_sha256 = Some(proof_sha256);
                receipt.proof_checked = Some(checked);
                receipt.truncated_proof_rejected = Some(mutation_rejected);
                terminal_unsat = true;
                attempts.push(receipt);
                break;
            }
            _ => return Err("internal unrecognized solver status".to_owned()),
        }
        attempts.push(receipt);
    }
    if !terminal_unsat {
        return Err(format!(
            "{} did not reach terminal UNSAT within the frozen bound",
            backend.id()
        ));
    }
    let exact = &observed == expected;
    if !exact {
        return Err(format!(
            "{} projected set mismatch: expected {}, observed {}",
            backend.id(),
            expected.len(),
            observed.len()
        ));
    }
    Ok(BackendReceipt {
        backend: backend.id().to_owned(),
        representation: backend.representation().to_owned(),
        expected_models: expected.len(),
        observed_models: observed.into_iter().collect(),
        exact_projected_set_match: exact,
        attempts,
    })
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

fn fixed_formula(
    encoding: &BooleanEncoding,
    assignment: &BTreeMap<String, u64>,
    backend: Backend,
) -> Result<String, String> {
    let units = encoding.assignment_unit_clauses(assignment)?;
    encoding.with_additional_clauses(backend.native_xor(), &units)
}

fn run_fixed_attempt(
    ir: &CanonicalIr,
    encoding: &BooleanEncoding,
    assignment: &BTreeMap<String, u64>,
    expected_sat: bool,
    backend: Backend,
    tools: ToolPaths<'_>,
    directory: &Path,
) -> Result<AttemptReceipt, String> {
    fs::create_dir_all(directory)
        .map_err(|error| format!("create {}: {error}", directory.display()))?;
    let input = directory.join(if backend.native_xor() {
        "input.xnf"
    } else {
        "input.cnf"
    });
    let formula = fixed_formula(encoding, assignment, backend)?;
    validate_dimacs(&formula, backend.native_xor())?;
    write_new(&input, formula.as_bytes())?;
    let (solver, proof) = run_solver(backend, tools, &input, directory)?;
    let status = solver_status(&solver)?;
    let mut receipt = base_attempt_receipt(0, status, &input, &solver)?;
    match (expected_sat, status) {
        (true, "sat") => {
            let decoded = encoding.decode_solver_model(&solver.stdout)?;
            if decoded != *assignment {
                return Err(format!(
                    "{} fixed model differs from the requested assignment",
                    backend.id()
                ));
            }
            let canonical_replay = ir.failed_equations(&decoded)?.is_empty();
            if !canonical_replay {
                return Err(format!(
                    "{} fixed positive fails canonical replay",
                    backend.id()
                ));
            }
            receipt.projection = Some(encoding.projected_values(&decoded)?);
            receipt.canonical_replay = Some(canonical_replay);
        }
        (false, "unsat") => {
            let (proof_sha256, checked, mutation_rejected) =
                verify_proof(backend, tools, &input, &proof, directory)?;
            receipt.proof_sha256 = Some(proof_sha256);
            receipt.proof_checked = Some(checked);
            receipt.truncated_proof_rejected = Some(mutation_rejected);
        }
        _ => {
            return Err(format!(
                "{} fixed control expected {}, observed {status}",
                backend.id(),
                if expected_sat { "sat" } else { "unsat" }
            ));
        }
    }
    Ok(receipt)
}

fn run_fixed_controls(
    root: &Path,
    tools: ToolPaths<'_>,
) -> Result<Vec<FixedControlReceipt>, String> {
    let fixtures = [("m3-t4", M3), ("m5-t4", M5), ("m7-t4", M7)];
    let mut receipts = Vec::new();
    for (name, input) in fixtures {
        let witness: LegacyWitness = serde_json::from_str(input)
            .map_err(|error| format!("parse {name} witness: {error}"))?;
        let ir = build_ir(fixture_spec(&witness))?;
        let encoding = BooleanEncoding::from_ir(&ir)?;
        let canonical_replay = ir.failed_equations(&witness.assignment)?.is_empty();
        let group_replay = fixture_group_replay(&witness);
        if !canonical_replay || !group_replay {
            return Err(format!("{name} source fixture fails replay"));
        }
        let mut mutation = witness.assignment.clone();
        let original_a0 = mutation
            .get("a0")
            .copied()
            .ok_or_else(|| format!("{name} witness omits a0"))?;
        mutation.insert("a0".to_owned(), (original_a0 + 1) % witness.p);
        if ir.failed_equations(&mutation)?.is_empty() {
            return Err(format!(
                "{name} deterministic mutation unexpectedly satisfies IR"
            ));
        }
        for backend in Backend::ALL {
            let backend_root = root.join(name).join(backend.id());
            let positive = run_fixed_attempt(
                &ir,
                &encoding,
                &witness.assignment,
                true,
                backend,
                tools,
                &backend_root.join("positive"),
            )?;
            let mutation_negative = run_fixed_attempt(
                &ir,
                &encoding,
                &mutation,
                false,
                backend,
                tools,
                &backend_root.join("a0-plus-one"),
            )?;
            receipts.push(FixedControlReceipt {
                fixture: name.to_owned(),
                arity: witness.m,
                mutation: "a0 := (a0 + 1) mod 17; all semantic fields fixed".to_owned(),
                known_assignment_canonical_replay: canonical_replay,
                known_assignment_group_replay: group_replay,
                backend: backend.id().to_owned(),
                positive,
                mutation_negative,
            });
        }
    }
    Ok(receipts)
}

fn parser_controls() -> Result<ParserControlReceipt, String> {
    let ir = build_ir(InstanceSpec {
        modulus: 17,
        curve_a: 2,
        curve_b: 2,
        arity: 3,
        depth: 3,
        terminal_c: 1,
        target_x: 0,
        target_y: 6,
    })?;
    let encoding = BooleanEncoding::from_ir(&ir)?;
    let cnf = encoding.to_cnf();
    let cnf_xor = encoding.to_cnf_xor();
    let mut truncated = cnf.clone();
    let last_terminator = truncated
        .rfind(" 0\n")
        .ok_or_else(|| "generated CNF has no terminated constraint".to_owned())?;
    truncated.truncate(last_terminator + 1);
    let trailing = format!("{cnf}1 0\n");
    Ok(ParserControlReceipt {
        ordinary_cnf_accepted: validate_dimacs(&cnf, false).is_ok(),
        cnf_xor_accepted: validate_dimacs(&cnf_xor, true).is_ok(),
        truncated_rejected: validate_dimacs(&truncated, false).is_err(),
        trailing_constraint_rejected: validate_dimacs(&trailing, false).is_err(),
        xor_rejected_as_ordinary_cnf: validate_dimacs(&cnf_xor, false).is_err(),
    })
}

const fn all_parser_controls_pass(receipt: &ParserControlReceipt) -> bool {
    receipt.ordinary_cnf_accepted
        && receipt.cnf_xor_accepted
        && receipt.truncated_rejected
        && receipt.trailing_constraint_rejected
        && receipt.xor_rejected_as_ordinary_cnf
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

fn count_checked_proofs(report: &E1SatPanelReport) -> (usize, usize) {
    let target_attempts = report
        .targets
        .iter()
        .flat_map(|target| &target.backends)
        .flat_map(|backend| &backend.attempts);
    let fixed_attempts = report
        .fixed_controls
        .iter()
        .flat_map(|control| [&control.positive, &control.mutation_negative]);
    target_attempts.chain(fixed_attempts).fold(
        (0_usize, 0_usize),
        |(checked, rejected), attempt| {
            (
                checked + usize::from(attempt.proof_checked == Some(true)),
                rejected + usize::from(attempt.truncated_proof_rejected == Some(true)),
            )
        },
    )
}

/// Execute the fixed E1 SAT/CNF-XOR representation-equivalence panel.
///
/// # Errors
///
/// Fails closed on any parser, model, oracle, proof, mutation, tool, or
/// write-once-artifact mismatch.
#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
pub fn run_e1_sat_panel(
    cadical: &Path,
    lrat_trim: &Path,
    cryptominisat: &Path,
    cake_xlrup: &Path,
    output: &Path,
    source_revision: &str,
) -> Result<E1SatPanelReport, String> {
    require_source_revision(source_revision)?;
    if output.exists() {
        return Err(format!(
            "E1 output directory already exists: {}",
            output.display()
        ));
    }
    fs::create_dir_all(output).map_err(|error| format!("create {}: {error}", output.display()))?;
    let tools = ToolPaths {
        cadical,
        lrat_trim,
        cryptominisat,
        cake_xlrup,
    };
    let tool_receipts = tool_receipts(tools)?;
    let parser_controls = parser_controls()?;
    if !all_parser_controls_pass(&parser_controls) {
        return Err("one or more strict parser controls failed".to_owned());
    }

    let targets_root = output.join("targets");
    fs::create_dir(&targets_root)
        .map_err(|error| format!("create {}: {error}", targets_root.display()))?;
    let mut targets = Vec::new();
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
        let encoding = BooleanEncoding::from_ir(&ir)?;
        let case_directory = targets_root.join(format!("x{:02}-y{:02}", target.x, target.y));
        let mut backends = Vec::new();
        for backend in Backend::ALL {
            backends.push(run_target_backend(
                &ir,
                &encoding,
                &oracle,
                backend,
                tools,
                &case_directory.join(backend.id()),
            )?);
        }
        targets.push(TargetReceipt {
            target: [target.x, target.y],
            exact_reference_models: oracle.into_iter().collect(),
            candidate_oracle_match: true,
            backends,
        });
    }

    let fixed_root = output.join("fixed-controls");
    fs::create_dir(&fixed_root)
        .map_err(|error| format!("create {}: {error}", fixed_root.display()))?;
    let fixed_controls = run_fixed_controls(&fixed_root, tools)?;
    let target_count = targets.len();
    let sat_target_count = targets
        .iter()
        .filter(|target| !target.exact_reference_models.is_empty())
        .count();
    let exact_target_backend_matches = targets
        .iter()
        .flat_map(|target| &target.backends)
        .filter(|backend| backend.exact_projected_set_match)
        .count();
    let expected_target_backend_matches = target_count * Backend::ALL.len();
    let runner = std::env::current_exe().map_err(|error| format!("resolve runner: {error}"))?;
    let mut report = E1SatPanelReport {
        schema: E1_SCHEMA.to_owned(),
        kind: "correctness-only representation equivalence; no timing or speed claim".to_owned(),
        source_revision: source_revision.to_owned(),
        runner_sha256: sha256_file(&runner)?,
        timing_collected: false,
        tools: tool_receipts,
        parser_controls,
        targets,
        fixed_controls,
        target_count,
        sat_target_count,
        unsat_target_count: target_count - sat_target_count,
        exact_target_backend_matches,
        expected_target_backend_matches,
        checked_terminal_unsat_proofs: 0,
        rejected_truncated_proofs: 0,
        verdict: "pending_aggregate_check".to_owned(),
        open_backend_obligations: vec![
            "msolve projected-model equivalence".to_owned(),
            "cvc5 QF_FF projected-model equivalence".to_owned(),
        ],
    };
    let (checked, rejected) = count_checked_proofs(&report);
    report.checked_terminal_unsat_proofs = checked;
    report.rejected_truncated_proofs = rejected;
    let fixed_pass = report.fixed_controls.iter().all(|control| {
        control.known_assignment_canonical_replay
            && control.known_assignment_group_replay
            && control.positive.status == "sat"
            && control.positive.canonical_replay == Some(true)
            && control.mutation_negative.status == "unsat"
            && control.mutation_negative.proof_checked == Some(true)
            && control.mutation_negative.truncated_proof_rejected == Some(true)
    });
    let target_pass = report.exact_target_backend_matches == report.expected_target_backend_matches
        && report
            .targets
            .iter()
            .flat_map(|target| &target.backends)
            .all(|backend| {
                backend.attempts.last().is_some_and(|attempt| {
                    attempt.status == "unsat"
                        && attempt.proof_checked == Some(true)
                        && attempt.truncated_proof_rejected == Some(true)
                })
            });
    if !fixed_pass || !target_pass {
        return Err("E1 aggregate correctness invariant failed".to_owned());
    }
    "sat_boolean_subgate_passed_msolve_cvc5_open".clone_into(&mut report.verdict);
    let encoded = serde_json::to_vec_pretty(&report)
        .map_err(|error| format!("serialize E1 report: {error}"))?;
    write_new(&output.join("e1-sat-panel.json"), &encoded)?;
    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixture_group_replay_and_mutation_controls_are_well_formed() {
        for input in [M3, M5, M7] {
            let witness: LegacyWitness = serde_json::from_str(input).unwrap();
            assert!(fixture_group_replay(&witness));
            let ir = build_ir(fixture_spec(&witness)).unwrap();
            let encoding = BooleanEncoding::from_ir(&ir).unwrap();
            assert!(ir.failed_equations(&witness.assignment).unwrap().is_empty());
            for backend in Backend::ALL {
                let formula = fixed_formula(&encoding, &witness.assignment, backend).unwrap();
                validate_dimacs(&formula, backend.native_xor()).unwrap();
            }
            let mut mutation = witness.assignment;
            mutation.insert("a0".to_owned(), (mutation["a0"] + 1) % witness.p);
            assert!(!ir.failed_equations(&mutation).unwrap().is_empty());
        }
    }

    #[test]
    fn parser_mutations_are_rejected() {
        assert!(all_parser_controls_pass(&parser_controls().unwrap()));
    }

    #[test]
    fn source_revision_is_strict() {
        assert!(require_source_revision(&"a".repeat(40)).is_ok());
        assert!(require_source_revision(&"A".repeat(40)).is_err());
        assert!(require_source_revision(&"a".repeat(39)).is_err());
        assert!(require_source_revision(&"g".repeat(40)).is_err());
    }
}
