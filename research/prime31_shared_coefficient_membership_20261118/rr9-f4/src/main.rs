use prime31_rr9_f4::{f4_fp, jv_bitops};
use prime31_selector_sat::prime31_curve;
use prime31_selector_sat::rr9::{
    build_frozen_tower, decode_and_replay_chart, derive_tower_witness, verify_assignment,
    FrozenTower, SignedFactor,
};
use prime31_selector_sat::rr9_panel::SharedRr9Case;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::env;
use std::fs;
use std::path::Path;
use std::time::{Duration, Instant};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct ModelRecord {
    schema: String,
    case_id: String,
    assignment: Vec<u64>,
}

#[derive(Clone, Debug, Serialize)]
struct VerifiedWitness {
    chart: [u64; 7],
    signed_factors: Vec<SignedFactor>,
}

#[derive(Debug, Serialize)]
struct SolveRecord {
    schema: &'static str,
    case_id: String,
    status: String,
    case_sha256: String,
    tower_sha256: String,
    f4_source_sha256: String,
    executable_sha256: String,
    degree_cap: u32,
    timeout_seconds: u64,
    rayon_threads: usize,
    tower: prime31_selector_sat::rr9::TowerSystemSpec,
    report: f4_fp::VisitReport,
    f4_native_operations: u64,
    f4_native_stair_operations: u64,
    supplemental_operations: jv_bitops::Snapshot,
    verified_witness: Option<VerifiedWitness>,
    verification_error: Option<String>,
    wall_ns: String,
    peak_rss_bytes_at_end: Option<u64>,
    forbidden: ForbiddenCounters,
    pcc: &'static str,
}

#[derive(Clone, Copy, Debug, Default, Serialize)]
struct ForbiddenCounters {
    secret_scalars_consumed: u64,
    supplied_chart_or_auxiliaries_consumed: u64,
    signed_multiset_table_entries_consumed: u64,
    complete_signed_multisets_enumerated: u64,
    grader_records_consumed_after_case_extraction: u64,
}

fn digest(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn peak_rss_bytes() -> Option<u64> {
    // SAFETY: `rusage` is plain old data and `getrusage` initializes it.
    let mut usage: libc::rusage = unsafe { std::mem::zeroed() };
    if unsafe { libc::getrusage(libc::RUSAGE_SELF, &mut usage) } != 0 {
        return None;
    }
    let raw = u64::try_from(usage.ru_maxrss).ok()?;
    #[cfg(target_os = "macos")]
    return Some(raw);
    #[cfg(not(target_os = "macos"))]
    raw.checked_mul(1024)
}

fn read_case(path: &str) -> Result<(SharedRr9Case, Vec<u8>), String> {
    let bytes = fs::read(path).map_err(|error| format!("read {path}: {error}"))?;
    let case = serde_json::from_slice::<SharedRr9Case>(&bytes)
        .map_err(|error| format!("parse {path}: {error}"))?;
    case.validate()?;
    if case.curve != prime31_curve() || case.curve.canonical_base(case.base.len())? != case.base {
        return Err("RR9 F4 runner accepts only the canonical Prime31 support".into());
    }
    Ok((case, bytes))
}

fn build_tower(case: &SharedRr9Case) -> Result<FrozenTower, String> {
    build_frozen_tower(
        case.curve.p,
        case.curve.a,
        (case.residual.x, case.residual.y),
        &case.support_polynomial,
    )
}

fn verify_model(
    case: &SharedRr9Case,
    tower: &FrozenTower,
    model: &ModelRecord,
) -> Result<VerifiedWitness, String> {
    if model.schema != "prime31.rr9-f4-model/v1" || model.case_id != case.case_id {
        return Err("RR9 model metadata does not match the public case".into());
    }
    if !verify_assignment(tower, &model.assignment, case.curve.p) {
        return Err("RR9 model fails direct evaluation of every tower equation".into());
    }
    let chart: [u64; 7] = model.assignment[..7]
        .try_into()
        .map_err(|_| "RR9 model omits chart variables".to_owned())?;
    let derived = derive_tower_witness(
        tower,
        case.curve.p,
        case.curve.a,
        (case.residual.x, case.residual.y),
        &case.support_polynomial,
        chart,
    )?;
    if derived != model.assignment {
        return Err("RR9 model auxiliaries differ from unique polynomial-division witness".into());
    }
    let signed_factors = decode_and_replay_chart(case.curve, &case.base, case.residual, &chart)?;
    Ok(VerifiedWitness {
        chart,
        signed_factors,
    })
}

fn write_json(path: &str, value: &impl Serialize) -> Result<(), String> {
    let mut bytes = serde_json::to_vec_pretty(value).map_err(|error| error.to_string())?;
    bytes.push(b'\n');
    fs::write(path, bytes).map_err(|error| format!("write {path}: {error}"))
}

fn solve(arguments: &[String]) -> Result<(), String> {
    if arguments.len() != 6 {
        return Err(
            "usage: prime31-rr9-f4 solve <case.json> <record.json> <model.json> <degree-cap> <timeout-seconds>"
                .into(),
        );
    }
    if rayon::current_num_threads() != 1 {
        return Err("registered RR9 F4 runs require RAYON_NUM_THREADS=1".into());
    }
    let degree_cap = arguments[4]
        .parse::<u32>()
        .map_err(|_| format!("invalid degree cap {:?}", arguments[4]))?;
    let timeout_seconds = arguments[5]
        .parse::<u64>()
        .map_err(|_| format!("invalid timeout {:?}", arguments[5]))?;
    if degree_cap < 3 || timeout_seconds == 0 {
        return Err("degree cap must be at least three and timeout must be positive".into());
    }
    let (case, case_bytes) = read_case(&arguments[1])?;
    let tower = build_tower(&case)?;
    let tower_bytes = serde_json::to_vec(&tower).map_err(|error| error.to_string())?;
    let polynomials = tower
        .equations
        .iter()
        .map(|equation| {
            let terms = equation
                .iter()
                .map(|term| (term.exponents.clone(), term.coefficient))
                .collect::<Vec<_>>();
            f4_fp::normalise(&terms, case.curve.p, f4_fp::Ordering::Grevlex)
        })
        .collect::<Vec<_>>();
    let options = f4_fp::F4Options::new(f4_fp::Ordering::Grevlex, degree_cap)
        .with_budget(Duration::from_secs(timeout_seconds));
    let domains = vec![None; tower.spec.variables];
    let before_supplemental = jv_bitops::snapshot();
    let before_native = f4_fp::native_ops_total();
    let before_stair = f4_fp::native_stair_ops_total();
    let started = Instant::now();
    let mut model = None;
    let mut verified_witness = None;
    let mut verification_error = None;
    let report = f4_fp::solve_visit(
        &polynomials,
        tower.spec.variables,
        case.curve.p,
        &options,
        &domains,
        |assignment| {
            let candidate = ModelRecord {
                schema: "prime31.rr9-f4-model/v1".into(),
                case_id: case.case_id.clone(),
                assignment: assignment.to_vec(),
            };
            match verify_model(&case, &tower, &candidate) {
                Ok(witness) => {
                    model = Some(candidate);
                    verified_witness = Some(witness);
                }
                Err(error) => verification_error = Some(error),
            }
            true
        },
    )?;
    let wall_ns = started.elapsed().as_nanos();
    let status = if verification_error.is_some() {
        "implementation_failure_invalid_model"
    } else if model.is_some() {
        "verified_sat"
    } else if report.complete {
        "complete_unsat"
    } else if report.timed_out {
        "timeout_unknown"
    } else {
        "incomplete_unknown"
    }
    .to_owned();
    if let Some(model) = &model {
        write_json(&arguments[3], model)?;
    }

    let f4_source =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../src/cryptanalysis/f4_fp.rs");
    let f4_source_bytes =
        fs::read(&f4_source).map_err(|error| format!("read {}: {error}", f4_source.display()))?;
    let executable =
        fs::read("/proc/self/exe").map_err(|error| format!("read running executable: {error}"))?;
    let record = SolveRecord {
        schema: "prime31.rr9-f4-solve-record/v1",
        case_id: case.case_id.clone(),
        status,
        case_sha256: digest(&case_bytes),
        tower_sha256: digest(&tower_bytes),
        f4_source_sha256: digest(&f4_source_bytes),
        executable_sha256: digest(&executable),
        degree_cap,
        timeout_seconds,
        rayon_threads: rayon::current_num_threads(),
        tower: tower.spec.clone(),
        report,
        f4_native_operations: f4_fp::native_ops_total().saturating_sub(before_native),
        f4_native_stair_operations: f4_fp::native_stair_ops_total().saturating_sub(before_stair),
        supplemental_operations: jv_bitops::snapshot().difference(before_supplemental),
        verified_witness,
        verification_error,
        wall_ns: wall_ns.to_string(),
        peak_rss_bytes_at_end: peak_rss_bytes(),
        forbidden: ForbiddenCounters::default(),
        pcc: "unverified",
    };
    write_json(&arguments[2], &record)?;
    println!("{}", record.status);
    Ok(())
}

fn verify(arguments: &[String]) -> Result<(), String> {
    if arguments.len() != 3 {
        return Err("usage: prime31-rr9-f4 verify <case.json> <model.json>".into());
    }
    let (case, _) = read_case(&arguments[1])?;
    let tower = build_tower(&case)?;
    let bytes =
        fs::read(&arguments[2]).map_err(|error| format!("read {}: {error}", arguments[2]))?;
    let model = serde_json::from_slice::<ModelRecord>(&bytes)
        .map_err(|error| format!("parse {}: {error}", arguments[2]))?;
    println!(
        "{}",
        serde_json::to_string_pretty(&verify_model(&case, &tower, &model)?)
            .map_err(|error| error.to_string())?
    );
    Ok(())
}

fn main() -> Result<(), String> {
    let arguments = env::args().skip(1).collect::<Vec<_>>();
    match arguments.first().map(String::as_str) {
        Some("solve") => solve(&arguments),
        Some("verify") => verify(&arguments),
        _ => Err("usage: prime31-rr9-f4 <solve|verify> ...".into()),
    }
}
