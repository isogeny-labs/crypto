//! Audit actual paired factor-base supports with the native PR #5153 controls.
//!
//! Usage: `h016_factor_base_audit --input paired.json [--output audit.json]`
//! Input schema: `crypto.factor-base-paired-input/v1`. Coordinates are canonical
//! `{ "x": u64, "y": u64 }` objects; `null` denotes the identity. Both arms use
//! the same ordered `targets` and `anchors`. No points or targets are generated.
//! The original H016 aggregate summary is not an input to this program.
//! The supplied prime `n` annihilates every listed point; this does not certify
//! the ambient group order or membership in one common cyclic subgroup.
//!
//! Counts are unordered index triples with repetition (`i <= j <= k`), with
//! identity and inverse-pair branches retained. Coverage and mean multiplicity
//! are separate statistics. Neither is useful relation rank or a solving cost.

use std::ffi::OsString;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use crypto_lib::cryptanalysis::prime_factor_base_controls::{
    admit_pair, triple_counts, ComparisonArm, IdentityPolicy, Representation, SupportProfile,
    TripleCounts,
};
use crypto_lib::cryptanalysis::prime_fast::{FastCurve, Pt, MAX_BITS};
use crypto_lib::cryptanalysis::residual_walk::is_prime_u64;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

// Qualify the exact shared module and its unit tests alongside this caller,
// without requiring the repository-wide lib-test binary. Both test paths use
// the real production prime_fast arithmetic; the full lib gate remains separate.
#[cfg(test)]
use crypto_lib::cryptanalysis::{self, prime_fast};
#[cfg(test)]
#[path = "../src/cryptanalysis/prime_factor_base_controls.rs"]
mod exact_controls_qualification;

const INPUT_SCHEMA: &str = "crypto.factor-base-paired-input/v1";
const OUTPUT_SCHEMA: &str = "crypto.factor-base-paired-audit/v1";
const USAGE: &str = "Usage: h016_factor_base_audit --input paired.json [--output audit.json]\n\
Input: crypto.factor-base-paired-input/v1; canonical {x,y} points or null for O.\n\
An existing output file is never replaced. Without --output, JSON goes to stdout.";

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Coordinate {
    x: u64,
    y: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct CurveInput {
    p: u64,
    a: u64,
    b: u64,
    n: u64,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
enum WireRepresentation {
    ExplicitPoints,
    NegationClosedPoints,
}

impl From<WireRepresentation> for Representation {
    fn from(value: WireRepresentation) -> Self {
        match value {
            WireRepresentation::ExplicitPoints => Self::ExplicitPoints,
            WireRepresentation::NegationClosedPoints => Self::NegationClosedPoints,
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
enum WireIdentityPolicy {
    Allow,
    Exclude,
}

impl From<WireIdentityPolicy> for IdentityPolicy {
    fn from(value: WireIdentityPolicy) -> Self {
        match value {
            WireIdentityPolicy::Allow => Self::Allow,
            WireIdentityPolicy::Exclude => Self::Exclude,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct ArmInput {
    name: String,
    declared_points: usize,
    representation: WireRepresentation,
    identity_policy: WireIdentityPolicy,
    points: Vec<Option<Coordinate>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Provenance {
    source: Option<String>,
    seed: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct AuditInput {
    schema: String,
    curve: CurveInput,
    anchors: Vec<Option<Coordinate>>,
    targets: Vec<Option<Coordinate>>,
    reference: ArmInput,
    control: ArmInput,
    provenance: Option<Provenance>,
}

fn checked_curve(input: &CurveInput) -> Result<FastCurve, String> {
    if input.p <= 3 || input.p >= (1_u64 << MAX_BITS) || !is_prime_u64(input.p) {
        return Err(format!(
            "curve.p must be an odd prime > 3 and < 2^{MAX_BITS}"
        ));
    }
    if input.n <= 2 || input.n >= (1_u64 << MAX_BITS) || !is_prime_u64(input.n) {
        return Err(format!(
            "curve.n must be an odd prime > 2 and < 2^{MAX_BITS}"
        ));
    }
    if input.a >= input.p || input.b >= input.p {
        return Err("curve.a and curve.b must be canonical residues less than p".into());
    }
    let curve = FastCurve::new(
        "paired-factor-base-input",
        input.p,
        input.a,
        input.b,
        input.n,
    );
    let f = &curve.f;
    let a_cubed = f.mul(f.sqr(curve.a), curve.a);
    let discriminant = f.add(
        f.mul(f.to_mont(4), a_cubed),
        f.mul(f.to_mont(27), f.sqr(curve.b)),
    );
    if discriminant == 0 {
        return Err("curve is singular: 4*a^3 + 27*b^2 vanishes modulo p".into());
    }
    Ok(curve)
}

fn checked_points(
    curve: &FastCurve,
    points: &[Option<Coordinate>],
    label: &str,
) -> Result<Vec<Option<Pt>>, String> {
    points
        .iter()
        .enumerate()
        .map(|(index, point)| {
            let Some(point) = point else {
                return Ok(None);
            };
            // FastCurve::point reduces residues, so this check must precede it.
            if point.x >= curve.f.p || point.y >= curve.f.p {
                return Err(format!("{label}[{index}] has a noncanonical coordinate"));
            }
            let point = curve.point(point.x, point.y);
            if !curve.is_on_curve(point) {
                return Err(format!("{label}[{index}] is not on the supplied curve"));
            }
            // FastCurve::mul uses the unreduced integer scalar. Reducing n
            // modulo n here would make this check vacuous.
            if curve.mul(Some(point), curve.n).is_some() {
                return Err(format!("{label}[{index}] is not killed by curve.n"));
            }
            Ok(Some(point))
        })
        .collect()
}

fn arm_output(input: &ArmInput, counts: TripleCounts, profile: &SupportProfile) -> Value {
    let mean_multiplicity = counts.stats.mean_multiplicity();
    let coverage = counts.stats.coverage();
    json!({
        "name": input.name,
        "declared_points": input.declared_points,
        "representation": input.representation,
        "identity_policy": input.identity_policy,
        "points": input.points,
        "support_profile": profile,
        "per_query_multiplicity": counts.per_target,
        "mean_multiplicity": mean_multiplicity,
        "coverage": coverage,
        "counting_stats": counts.stats,
    })
}

fn audit(input_bytes: &[u8]) -> Result<Value, String> {
    let input: AuditInput = serde_json::from_slice(input_bytes)
        .map_err(|error| format!("invalid strict input JSON: {error}"))?;
    if input.schema != INPUT_SCHEMA {
        return Err(format!("input schema must be {INPUT_SCHEMA}"));
    }
    if input.targets.is_empty() {
        return Err("the ordered target list must be nonempty".into());
    }
    if input.reference.name.trim().is_empty() || input.control.name.trim().is_empty() {
        return Err("both arm names must be nonempty".into());
    }
    let curve = checked_curve(&input.curve)?;
    let anchors = checked_points(&curve, &input.anchors, "anchors")?;
    let targets = checked_points(&curve, &input.targets, "targets")?;
    let reference_points = checked_points(&curve, &input.reference.points, "reference.points")?;
    let control_points = checked_points(&curve, &input.control.points, "control.points")?;
    let arm = |definition: &ArmInput, points| ComparisonArm {
        points,
        targets: &targets,
        declared_points: definition.declared_points,
        representation: definition.representation.into(),
        identity_policy: definition.identity_policy.into(),
    };
    let admission = admit_pair(
        &curve,
        &anchors,
        arm(&input.reference, reference_points.as_slice()),
        arm(&input.control, control_points.as_slice()),
    )
    .map_err(|error| format!("paired support admission failed: {error}"))?;

    let reference_counts = triple_counts(&curve, &reference_points, &targets)
        .map_err(|error| format!("reference counting failed: {error}"))?;
    let control_counts = triple_counts(&curve, &control_points, &targets)
        .map_err(|error| format!("control counting failed: {error}"))?;
    let reference = arm_output(&input.reference, reference_counts, &admission.profile);
    let control = arm_output(&input.control, control_counts, &admission.profile);
    Ok(json!({
        "schema": OUTPUT_SCHEMA,
        "status": "component_audit",
        "pcc_status": "unverified",
        "input_sha256": format!("{:x}", Sha256::digest(input_bytes)),
        "source_sha256": {
            "example": format!("{:x}", Sha256::digest(include_bytes!("h016_factor_base_audit.rs"))),
            "controls": format!("{:x}", Sha256::digest(include_bytes!("../src/cryptanalysis/prime_factor_base_controls.rs"))),
            "prime_fast": format!("{:x}", Sha256::digest(include_bytes!("../src/cryptanalysis/prime_fast.rs"))),
            "primality": format!("{:x}", Sha256::digest(include_bytes!("../src/cryptanalysis/residual_walk.rs"))),
        },
        "provided_provenance": input.provenance,
        "curve": input.curve,
        "parameter_semantics": "n is the supplied prime annihilator used to check [n]P = O on every input point; no ambient curve order or membership in one common cyclic subgroup is certified.",
        "anchors": input.anchors,
        "targets": input.targets,
        "counting_convention": "unordered_index_triples_i_le_j_le_k_with_repetition",
        "paired_admission": admission,
        "reference": reference,
        "control": control,
        "unmeasured": {
            "useful_independent_relation_rank": null,
            "scalar_recovery": null,
            "complete_solving_cost": null,
            "runtime": null,
        },
        "scope": "Actual support admission and exact tuple counts only; counting_stats excludes validation, hashing, allocation, relation filtering, rank and recovery.",
    }))
}

fn output_bytes(report: &Value) -> Result<Vec<u8>, String> {
    let mut bytes = serde_json::to_vec_pretty(report)
        .map_err(|error| format!("cannot encode audit output: {error}"))?;
    bytes.push(b'\n');
    Ok(bytes)
}

fn write_new_output(path: &Path, bytes: &[u8]) -> Result<(), String> {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| format!("cannot create output {}: {error}", path.display()))?;
    file.write_all(bytes)
        .and_then(|()| file.sync_all())
        .map_err(|error| format!("cannot finish output {}: {error}", path.display()))
}

fn run_audit(input_path: &Path, output_path: Option<&Path>) -> Result<(), String> {
    let input = fs::read(input_path)
        .map_err(|error| format!("cannot read input {}: {error}", input_path.display()))?;
    // No output is opened or emitted until every validation and count succeeds.
    let bytes = output_bytes(&audit(&input)?)?;
    if let Some(path) = output_path {
        write_new_output(path, &bytes)
    } else {
        let mut stdout = io::stdout().lock();
        stdout
            .write_all(&bytes)
            .and_then(|()| stdout.flush())
            .map_err(|error| format!("cannot write audit to stdout: {error}"))
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Command {
    Help,
    Audit {
        input: PathBuf,
        output: Option<PathBuf>,
    },
}

fn parse_args(args: impl IntoIterator<Item = OsString>) -> Result<Command, String> {
    let mut args = args.into_iter().peekable();
    let mut input = None;
    let mut output = None;
    while let Some(arg) = args.next() {
        match arg.to_str() {
            Some("--help" | "-h")
                if input.is_none() && output.is_none() && args.peek().is_none() =>
            {
                return Ok(Command::Help);
            }
            Some("--input") => {
                if input.is_some() {
                    return Err("--input may occur only once".into());
                }
                input = Some(PathBuf::from(args.next().ok_or("--input needs a path")?));
            }
            Some("--output") => {
                if output.is_some() {
                    return Err("--output may occur only once".into());
                }
                output = Some(PathBuf::from(args.next().ok_or("--output needs a path")?));
            }
            _ => return Err(format!("unknown argument: {}", arg.to_string_lossy())),
        }
    }
    Ok(Command::Audit {
        input: input.ok_or("--input is required")?,
        output,
    })
}

fn main() {
    let result = parse_args(std::env::args_os().skip(1)).and_then(|command| match command {
        Command::Help => {
            println!("{USAGE}");
            Ok(())
        }
        Command::Audit { input, output } => run_audit(&input, output.as_deref()),
    });
    if let Err(error) = result {
        eprintln!("h016_factor_base_audit: {error}");
        std::process::exit(2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};

    const P: u64 = 2_147_483_647;
    const N: u64 = 2_147_446_831;

    fn curve() -> FastCurve {
        FastCurve::new("M31 component regression", P, 847_514_846, 2_087_444_724, N)
    }

    fn point(curve: &FastCurve, point: Option<Pt>) -> Value {
        point.map_or(Value::Null, |point| {
            let (x, y) = curve.canonical(point);
            json!({ "x": x, "y": y })
        })
    }

    fn fixture() -> Value {
        let curve = curve();
        let g = curve.point(625_037_715, 2_100_627_251);
        let g_json = point(&curve, Some(g));
        let minus_g = point(&curve, Some(curve.neg(g)));
        let two_g = point(&curve, curve.mul(Some(g), 2));
        let three_g = point(&curve, curve.mul(Some(g), 3));
        json!({
            "schema": INPUT_SCHEMA,
            "curve": { "p": P, "a": 847_514_846, "b": 2_087_444_724_u64, "n": N },
            "anchors": [g_json.clone()],
            "targets": [two_g.clone(), two_g.clone(), null, three_g],
            "reference": {
                "name": "ordered", "declared_points": 3,
                "representation": "explicit_points", "identity_policy": "exclude",
                "points": [g_json.clone(), minus_g.clone(), two_g.clone()],
            },
            "control": {
                "name": "permuted", "declared_points": 3,
                "representation": "explicit_points", "identity_policy": "exclude",
                "points": [g_json, two_g, minus_g],
            },
        })
    }

    fn audit_value(input: &Value) -> Result<Value, String> {
        audit(&serde_json::to_vec(input).unwrap())
    }

    #[test]
    fn m31_identity_cancellation_duplicate_queries_and_metrics() {
        let input = serde_json::to_vec(&fixture()).unwrap();
        let report = audit(&input).unwrap();
        assert_eq!(report["schema"], OUTPUT_SCHEMA);
        assert_eq!(
            report["input_sha256"],
            format!("{:x}", Sha256::digest(&input))
        );
        for arm in ["reference", "control"] {
            assert_eq!(report[arm]["per_query_multiplicity"], json!([1, 1, 1, 2]));
            assert_eq!(report[arm]["mean_multiplicity"], 1.25);
            assert_eq!(report[arm]["coverage"], 1.0);
            assert_eq!(report[arm]["counting_stats"]["queries"], 4);
            assert_eq!(report[arm]["support_profile"]["inverse_pairs"], 1);
            assert_eq!(report[arm]["support_profile"]["singleton_orbits"], 1);
        }
        assert!(report["unmeasured"]["useful_independent_relation_rank"].is_null());
        assert!(report["unmeasured"]["scalar_recovery"].is_null());
        assert!(report["unmeasured"]["complete_solving_cost"].is_null());
        assert!(report["provided_provenance"].is_null());
    }

    #[test]
    fn strict_schema_and_modes_reject_unknown_or_malformed_data() {
        assert!(audit(b"{").is_err());
        for (pointer, replacement) in [
            ("/schema", json!("h016.aggregate/v1")),
            ("/reference/representation", json!("x_only")),
            ("/control/identity_policy", json!("silently_drop")),
            ("/targets", json!([])),
            ("/reference/name", json!(" ")),
            ("/curve/p", json!(-1)),
            ("/curve/p", json!(2_147_483_647.0)),
        ] {
            let mut input = fixture();
            *input.pointer_mut(pointer).unwrap() = replacement;
            assert!(
                audit_value(&input).is_err(),
                "accepted mutation at {pointer}"
            );
        }
        for pointer in [
            "",
            "/curve",
            "/reference",
            "/control",
            "/reference/points/0",
        ] {
            let mut input = fixture();
            input
                .pointer_mut(pointer)
                .unwrap()
                .as_object_mut()
                .unwrap()
                .insert("unexpected".into(), json!(true));
            assert!(
                audit_value(&input).is_err(),
                "accepted extra field at {pointer}"
            );
        }
    }

    #[test]
    fn field_subgroup_and_curve_validation_precede_arithmetic() {
        for (field, value) in [
            ("p", 2),
            ("p", 9),
            ("p", 1_u64 << 62),
            ("n", 1),
            ("n", 2),
            ("n", 9),
            ("n", 1_u64 << 62),
            ("a", P),
            ("b", P),
        ] {
            let mut input = fixture();
            input["curve"][field] = json!(value);
            assert!(
                audit_value(&input).is_err(),
                "accepted invalid curve.{field}={value}"
            );
        }
        let mut input = fixture();
        input["curve"]["a"] = json!(0);
        input["curve"]["b"] = json!(0);
        assert!(audit_value(&input).unwrap_err().contains("singular"));
        let mut input = fixture();
        input["curve"]["n"] = json!(3);
        assert!(audit_value(&input).unwrap_err().contains("not killed"));
    }

    #[test]
    fn all_coordinate_lists_reject_noncanonical_and_off_curve_points() {
        for pointer in [
            "/anchors/0",
            "/targets/0",
            "/reference/points/0",
            "/control/points/0",
        ] {
            for replacement in [
                json!({ "x": P, "y": 1 }),
                json!({ "x": 1, "y": P }),
                json!({ "x": 0, "y": 0 }),
            ] {
                let mut input = fixture();
                *input.pointer_mut(pointer).unwrap() = replacement;
                assert!(
                    audit_value(&input).is_err(),
                    "accepted bad point at {pointer}"
                );
            }
        }
    }

    #[test]
    fn mismatched_declared_size_duplicates_and_orbit_profiles_reject() {
        let mut input = fixture();
        input["control"]["declared_points"] = json!(4);
        assert!(audit_value(&input).unwrap_err().contains("declares 4"));
        let mut input = fixture();
        input["control"]["points"][1] = input["control"]["points"][0].clone();
        assert!(audit_value(&input).unwrap_err().contains("duplicate"));
        let mut input = fixture();
        input["anchors"] = json!([]);
        input["control"]["points"][2] = input["targets"][3].clone();
        assert!(audit_value(&input).unwrap_err().contains("profiles"));
    }

    #[test]
    fn matching_orbit_counts_do_not_hide_changed_anchor_signs() {
        let mut input = fixture();
        let g = input["reference"]["points"][0].clone();
        let minus_g = input["reference"]["points"][1].clone();
        let two_g = input["targets"][0].clone();
        let three_g = input["targets"][3].clone();
        input["reference"]["points"] = json!([g, two_g.clone(), three_g.clone()]);
        input["control"]["points"] = json!([minus_g, two_g, three_g]);
        assert!(audit_value(&input).unwrap_err().contains("anchor"));
    }

    #[test]
    fn odd_mixed_support_is_explicit_and_identity_closed_support_is_allowed() {
        assert!(audit_value(&fixture()).is_ok());
        let mut input = fixture();
        for arm in ["reference", "control"] {
            input[arm]["representation"] = json!("negation_closed_points");
        }
        assert!(audit_value(&input).is_err());
        let g = input["reference"]["points"][0].clone();
        let minus_g = input["reference"]["points"][1].clone();
        input["anchors"] = json!([null, g.clone()]);
        for arm in ["reference", "control"] {
            input[arm]["points"] = json!([null, g.clone(), minus_g.clone()]);
            input[arm]["identity_policy"] = json!("allow");
        }
        let report = audit_value(&input).unwrap();
        assert_eq!(report["reference"]["support_profile"]["identity_points"], 1);
        assert_eq!(report["reference"]["support_profile"]["inverse_pairs"], 1);
        input["control"]["identity_policy"] = json!("exclude");
        assert!(audit_value(&input).is_err());
    }

    #[test]
    fn identity_only_support_does_not_certify_an_ambient_group_order() {
        let mut input = fixture();
        input["curve"]["n"] = json!(3);
        input["anchors"] = json!([null]);
        input["targets"] = json!([null, null]);
        for arm in ["reference", "control"] {
            input[arm]["declared_points"] = json!(1);
            input[arm]["points"] = json!([null]);
            input[arm]["representation"] = json!("negation_closed_points");
            input[arm]["identity_policy"] = json!("allow");
        }
        let report = audit_value(&input).unwrap();
        assert_eq!(report["reference"]["per_query_multiplicity"], json!([1, 1]));
        assert_eq!(report["reference"]["support_profile"]["orbit_columns"], 0);
        assert!(report["parameter_semantics"]
            .as_str()
            .unwrap()
            .contains("no ambient curve order"));
    }

    struct TemporaryDirectory(PathBuf);

    impl TemporaryDirectory {
        fn new() -> Self {
            static NEXT: AtomicU64 = AtomicU64::new(0);
            for _ in 0..128 {
                let path = std::env::temp_dir().join(format!(
                    "h016-native-output-{}-{}",
                    std::process::id(),
                    NEXT.fetch_add(1, Ordering::Relaxed)
                ));
                match fs::create_dir(&path) {
                    Ok(()) => return Self(path),
                    Err(error) if error.kind() == io::ErrorKind::AlreadyExists => continue,
                    Err(error) => panic!("cannot create test directory: {error}"),
                }
            }
            panic!("cannot allocate a unique test directory");
        }
    }

    impl Drop for TemporaryDirectory {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn output_refuses_overwrite_and_invalid_input_creates_no_report() {
        let temp = TemporaryDirectory::new();
        let input_path = temp.0.join("paired.json");
        let output_path = temp.0.join("audit.json");
        fs::write(&input_path, serde_json::to_vec(&fixture()).unwrap()).unwrap();
        run_audit(&input_path, Some(&output_path)).unwrap();
        let retained = fs::read(&output_path).unwrap();
        assert!(run_audit(&input_path, Some(&output_path)).is_err());
        assert_eq!(fs::read(&output_path).unwrap(), retained);
        assert!(run_audit(&input_path, Some(&input_path)).is_err());

        fs::write(&input_path, b"{ malformed").unwrap();
        let fresh_output = temp.0.join("must-not-exist.json");
        assert!(run_audit(&input_path, Some(&fresh_output)).is_err());
        assert!(!fresh_output.exists());
    }

    #[test]
    fn argument_parser_requires_explicit_paths_and_rejects_unknown_flags() {
        let args = |items: &[&str]| items.iter().map(OsString::from).collect::<Vec<_>>();
        assert_eq!(parse_args(args(&["--help"])).unwrap(), Command::Help);
        assert_eq!(
            parse_args(args(&["--input", "paired.json"])).unwrap(),
            Command::Audit {
                input: PathBuf::from("paired.json"),
                output: None,
            }
        );
        for invalid in [
            vec![],
            vec!["--input"],
            vec!["--unknown"],
            vec!["--input", "a", "--input", "b"],
            vec!["--input", "a", "--output", "b", "--output", "c"],
        ] {
            assert!(parse_args(args(&invalid)).is_err());
        }
    }
}
