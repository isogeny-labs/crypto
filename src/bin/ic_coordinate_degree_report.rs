//! Pre-registered reducer for the matched coordinate degree study.
//!
//! The measurement binary emits one deterministic report containing all arms.
//! This reducer checks the frozen controls, preserves the three distinct degree
//! conventions, and emits a compact JSON summary, Markdown table, and SVG.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

const EXPECTED_SYSTEMS: [&str; 9] = [
    "montgomery-u-semaev",
    "montgomery-u-aa",
    "weierstrass-x-semaev",
    "montgomery-affine-slope",
    "quotient-w-squarefree",
    "quotient-w-raw-norm",
    "edwards-y-semaev",
    "edwards-affine-bilinear",
    "montgomery-u-unsatisfiable-control",
];

#[derive(Clone, Debug, Deserialize)]
struct DegreeStudyReport {
    schema: String,
    evidence_status: String,
    config: StudyConfig,
    fixture: Fixture,
    observations: Vec<Observation>,
    controls: Controls,
    solving_degree_convention: String,
    regularity_convention: String,
    limitations: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
struct StudyConfig {
    protocol_sha256: String,
    instrument_commit: String,
    monomial_order: String,
    max_degree: u32,
    wall_time_admissible: bool,
}

#[derive(Clone, Debug, Deserialize)]
struct Fixture {
    fixture_id: String,
    source_curve_id: String,
    source_curve_uid: String,
    destination_curve_id: String,
    destination_curve_uid: String,
    source_group_order: u64,
    subgroup_order: u64,
    subgroup_map_passed: bool,
    raw_norm_identity_passed: bool,
    hessian_included: bool,
    hessian_exclusion: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Observation {
    name: String,
    coordinate_model: String,
    control_role: String,
    expected_outcome: String,
    variables: usize,
    equations: usize,
    input_monomials: usize,
    max_input_degree: u32,
    semi_regular_degree: Option<usize>,
    known_solution_valid: bool,
    solver_complete: bool,
    expected_outcome_passed: bool,
    inconsistent: bool,
    pairs_above_bound: usize,
    selected_pairs: usize,
    f4_steps: usize,
    productive_solving_degree: u32,
    total_solving_degree: u32,
    raw_max_step_degree: u32,
    affine_regularity_index: Option<usize>,
    basis_max_degree: u32,
    basis_polynomials: usize,
    basis_monomials: usize,
    max_rows: usize,
    max_columns: usize,
    field_multiplications: u64,
}

#[derive(Clone, Debug, Deserialize)]
struct Controls {
    inventory_passed: bool,
    aa_observation_identical: bool,
    positive_controls_passed: bool,
    negative_control_passed: bool,
    all_solver_runs_complete: bool,
}

impl Controls {
    fn passed(&self) -> bool {
        self.inventory_passed
            && self.aa_observation_identical
            && self.positive_controls_passed
            && self.negative_control_passed
            && self.all_solver_runs_complete
    }
}

#[derive(Clone, Debug, Serialize)]
struct FileHash {
    path: String,
    sha256: String,
    bytes: u64,
}

#[derive(Clone, Debug, Serialize)]
struct Findings {
    squarefree_w_matches_u_degree_profile: bool,
    raw_norm_preserves_affine_regularity: bool,
    raw_norm_total_degree_increase: i64,
    edwards_bilinear_input_degree_reduction: i64,
    edwards_bilinear_total_degree_difference: i64,
    edwards_bilinear_selected_pair_difference: i64,
    montgomery_affine_is_unique_minimum_total_degree: bool,
    hessian_excluded: bool,
}

#[derive(Clone, Debug, Serialize)]
struct Summary {
    schema: &'static str,
    evidence_status: String,
    raw_report: FileHash,
    analysis_source: FileHash,
    protocol_sha256: String,
    instrument_commit: String,
    monomial_order: String,
    max_degree: u32,
    fixture_id: String,
    source_curve_id: String,
    source_curve_uid: String,
    destination_curve_id: String,
    destination_curve_uid: String,
    controls_passed: bool,
    findings: Findings,
    observations: Vec<Observation>,
    solving_degree_convention: String,
    regularity_convention: String,
    limitations: Vec<String>,
}

fn option_value(args: &[String], name: &str) -> Option<PathBuf> {
    args.windows(2)
        .find(|window| window[0] == name)
        .map(|window| PathBuf::from(&window[1]))
}

fn usage() -> ! {
    eprintln!(
        "usage: ic_coordinate_degree_report --input FILE --analysis-source FILE --out-dir DIR"
    );
    std::process::exit(2);
}

fn read_json<T: for<'de> Deserialize<'de>>(path: &Path) -> Result<T, String> {
    let bytes = fs::read(path).map_err(|error| format!("{}: {error}", path.display()))?;
    serde_json::from_slice(&bytes).map_err(|error| format!("{}: {error}", path.display()))
}

fn file_hash(path: &Path) -> Result<FileHash, String> {
    let bytes = fs::read(path).map_err(|error| format!("{}: {error}", path.display()))?;
    Ok(FileHash {
        path: path.display().to_string(),
        sha256: format!("{:x}", Sha256::digest(&bytes)),
        bytes: bytes.len() as u64,
    })
}

fn difference(left: usize, right: usize) -> i64 {
    left as i64 - right as i64
}

fn degree_difference(left: u32, right: u32) -> i64 {
    i64::from(left) - i64::from(right)
}

fn same_degree_profile(left: &Observation, right: &Observation) -> bool {
    left.max_input_degree == right.max_input_degree
        && left.semi_regular_degree == right.semi_regular_degree
        && left.affine_regularity_index == right.affine_regularity_index
        && left.productive_solving_degree == right.productive_solving_degree
        && left.total_solving_degree == right.total_solving_degree
        && left.raw_max_step_degree == right.raw_max_step_degree
        && left.basis_max_degree == right.basis_max_degree
}

fn validate(report: &DegreeStudyReport) -> Result<BTreeMap<&str, &Observation>, String> {
    if report.schema != "ic.coordinate_rosetta.degree_study/v1" {
        return Err(format!("unexpected report schema: {}", report.schema));
    }
    if report.evidence_status != "pcc:unverified" {
        return Err("degree report must remain pcc:unverified".into());
    }
    if report.config.wall_time_admissible {
        return Err("degree report incorrectly marks local wall time admissible".into());
    }
    if report.config.protocol_sha256.len() != 64
        || !report
            .config
            .protocol_sha256
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit())
    {
        return Err("invalid protocol SHA-256 in raw report".into());
    }
    if report.config.instrument_commit.len() != 40
        || !report
            .config
            .instrument_commit
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit())
    {
        return Err("raw report must carry a full 40-hex instrument commit".into());
    }
    if !report.controls.passed() {
        return Err("one or more hard study controls failed".into());
    }
    if report.fixture.source_group_order != 1_004
        || report.fixture.subgroup_order != 251
        || !report.fixture.subgroup_map_passed
        || !report.fixture.raw_norm_identity_passed
    {
        return Err("fixture arithmetic certificate failed".into());
    }
    if report.fixture.hessian_included {
        return Err("Hessian arm must be excluded for the fixed order-1004 fixture".into());
    }
    let expected = EXPECTED_SYSTEMS.into_iter().collect::<BTreeSet<_>>();
    let actual = report
        .observations
        .iter()
        .map(|observation| observation.name.as_str())
        .collect::<BTreeSet<_>>();
    if actual != expected || report.observations.len() != EXPECTED_SYSTEMS.len() {
        return Err(format!("unexpected system set: {actual:?}"));
    }
    if report.observations.iter().any(|observation| {
        !observation.solver_complete
            || !observation.expected_outcome_passed
            || observation.pairs_above_bound != 0
    }) {
        return Err("an observation is incomplete, over-bound, or failed its oracle".into());
    }
    Ok(report
        .observations
        .iter()
        .map(|observation| (observation.name.as_str(), observation))
        .collect())
}

fn findings(rows: &BTreeMap<&str, &Observation>) -> Findings {
    let u = rows["montgomery-u-semaev"];
    let w = rows["quotient-w-squarefree"];
    let raw = rows["quotient-w-raw-norm"];
    let edwards = rows["edwards-y-semaev"];
    let bilinear = rows["edwards-affine-bilinear"];
    let affine = rows["montgomery-affine-slope"];
    let minimum = rows
        .values()
        .filter(|row| row.expected_outcome == "planted_solution")
        .map(|row| row.total_solving_degree)
        .min()
        .unwrap_or(0);
    let minimum_count = rows
        .values()
        .filter(|row| {
            row.expected_outcome == "planted_solution" && row.total_solving_degree == minimum
        })
        .count();
    Findings {
        squarefree_w_matches_u_degree_profile: same_degree_profile(u, w),
        raw_norm_preserves_affine_regularity: raw.affine_regularity_index
            == w.affine_regularity_index,
        raw_norm_total_degree_increase: degree_difference(
            raw.total_solving_degree,
            w.total_solving_degree,
        ),
        edwards_bilinear_input_degree_reduction: degree_difference(
            edwards.max_input_degree,
            bilinear.max_input_degree,
        ),
        edwards_bilinear_total_degree_difference: degree_difference(
            bilinear.total_solving_degree,
            edwards.total_solving_degree,
        ),
        edwards_bilinear_selected_pair_difference: difference(
            bilinear.selected_pairs,
            edwards.selected_pairs,
        ),
        montgomery_affine_is_unique_minimum_total_degree: affine.total_solving_degree == minimum
            && minimum_count == 1,
        hessian_excluded: true,
    }
}

fn optional(value: Option<usize>) -> String {
    value.map_or_else(|| "—".into(), |value| value.to_string())
}

fn markdown(summary: &Summary, hessian_exclusion: &str) -> String {
    let mut report =
        String::from("# Matched coordinate degree study\n\n[pcc:unverified]\n\n## Outcome\n\n");
    report.push_str(if summary.controls_passed {
        "All pre-registered arithmetic, A/A, planted, negative, and solver-completion controls passed.\n\n"
    } else {
        "At least one hard control failed; no comparison is admissible.\n\n"
    });
    report.push_str("| system | vars | eqs | terms | input degree | semi-regular estimate | affine regularity | productive F4 degree | total solving degree | selected pair rows | max matrix |\n| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |\n");
    for row in &summary.observations {
        report.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {}×{} |\n",
            row.name,
            row.variables,
            row.equations,
            row.input_monomials,
            row.max_input_degree,
            optional(row.semi_regular_degree),
            optional(row.affine_regularity_index),
            row.productive_solving_degree,
            row.total_solving_degree,
            row.selected_pairs,
            row.max_rows,
            row.max_columns
        ));
    }
    let f = &summary.findings;
    report.push_str("\n## Frozen comparisons\n\n");
    report.push_str(&format!(
        "- Square-free `w` matches the Montgomery `u` degree profile: `{}`.\n- Raw `h_3^4` preserves the affine regularity index and changes total solving degree by `{:+}`: `{}`.\n- Edwards bilinear changes input degree by `-{}` and total solving degree by `{:+}`; selected F4 pair rows change by `{:+}`.\n- Montgomery affine-with-slope is the unique minimum-total-degree arm: `{}`.\n- Hessian exclusion: {}\n",
        f.squarefree_w_matches_u_degree_profile,
        f.raw_norm_total_degree_increase,
        f.raw_norm_preserves_affine_regularity,
        f.edwards_bilinear_input_degree_reduction,
        f.edwards_bilinear_total_degree_difference,
        f.edwards_bilinear_selected_pair_difference,
        f.montgomery_affine_is_unique_minimum_total_degree,
        hessian_exclusion
    ));
    report.push_str(
        "\n![Degree comparison](figures/degree_comparison.svg)\n\n## Conventions and limits\n\n",
    );
    report.push_str(&format!(
        "- Total solving degree: {}\n- Regularity: {}\n- Monomial order: `{}`; maximum admitted degree: `{}`.\n- Source curve: `{}` (`{}`).\n- Isogenous destination: `{}` (`{}`).\n- No local wall time is reported. Selected pair rows are native F4 rows, not sequential Buchberger S-pair counts.\n- This one public `F_1019` fixture is a symbolic qualification, not a Curve25519 attack or scaling result.\n",
        summary.solving_degree_convention,
        summary.regularity_convention,
        summary.monomial_order,
        summary.max_degree,
        summary.source_curve_id,
        summary.source_curve_uid,
        summary.destination_curve_id,
        summary.destination_curve_uid
    ));
    for limitation in &summary.limitations {
        report.push_str(&format!("- {limitation}\n"));
    }
    report.push_str(&format!(
        "\nProtocol SHA-256: `{}`  \nInstrument commit: `{}`  \nRaw report SHA-256: `{}`  \nReducer source SHA-256: `{}`\n",
        summary.protocol_sha256,
        summary.instrument_commit,
        summary.raw_report.sha256,
        summary.analysis_source.sha256
    ));
    report
}

fn svg(observations: &[Observation]) -> String {
    let width = 1_300.0;
    let height = 700.0;
    let left = 80.0;
    let top = 90.0;
    let plot_height = 430.0;
    let step = 132.0;
    let bar_width = 34.0;
    let maximum = observations
        .iter()
        .map(|row| row.max_input_degree.max(row.total_solving_degree))
        .max()
        .unwrap_or(1) as f64;
    let mut output = format!(
        "<svg xmlns='http://www.w3.org/2000/svg' width='{width}' height='{height}' viewBox='0 0 {width} {height}'>\n<rect width='100%' height='100%' fill='white'/>\n<text x='40' y='38' font-family='sans-serif' font-size='24' font-weight='bold'>Matched coordinate degree study</text>\n<text x='40' y='63' font-family='sans-serif' font-size='14' fill='#444'>Input degree (blue) and total solving degree (purple); labels give selected F4 pair rows. [pcc:unverified]</text>\n"
    );
    for tick in 0..=8 {
        let value = maximum * tick as f64 / 8.0;
        let y = top + plot_height - plot_height * tick as f64 / 8.0;
        output.push_str(&format!(
            "<line x1='{left}' y1='{y:.1}' x2='1260' y2='{y:.1}' stroke='#e5e7eb'/><text x='70' y='{:.1}' text-anchor='end' font-family='sans-serif' font-size='11'>{value:.0}</text>\n",
            y + 4.0
        ));
    }
    for (index, row) in observations.iter().enumerate() {
        let center = left + 60.0 + index as f64 * step;
        let input_height = plot_height * f64::from(row.max_input_degree) / maximum;
        let solve_height = plot_height * f64::from(row.total_solving_degree) / maximum;
        output.push_str(&format!(
            "<rect x='{:.1}' y='{:.1}' width='{bar_width}' height='{input_height:.1}' fill='#2e90fa'/><rect x='{:.1}' y='{:.1}' width='{bar_width}' height='{solve_height:.1}' fill='#7f56d9'/><text x='{center:.1}' y='552' text-anchor='middle' font-family='sans-serif' font-size='11'>{}</text><text x='{center:.1}' y='574' text-anchor='middle' font-family='sans-serif' font-size='10' fill='#555'>pairs {}</text>\n",
            center - bar_width - 2.0,
            top + plot_height - input_height,
            center + 2.0,
            top + plot_height - solve_height,
            row.name.replace("montgomery-", "mont-").replace("quotient-", "quot-").replace("weierstrass-", "wei-").replace("unsatisfiable-control", "unsat") ,
            row.selected_pairs
        ));
    }
    output.push_str("<rect x='930' y='620' width='16' height='16' fill='#2e90fa'/><text x='954' y='633' font-family='sans-serif' font-size='13'>max input degree</text><rect x='1080' y='620' width='16' height='16' fill='#7f56d9'/><text x='1104' y='633' font-family='sans-serif' font-size='13'>total solving degree</text>\n<text x='22' y='315' text-anchor='middle' transform='rotate(-90 22 315)' font-family='sans-serif' font-size='13'>total degree</text>\n</svg>\n");
    output
}

fn run() -> Result<(), String> {
    let args = std::env::args().collect::<Vec<_>>();
    let input = option_value(&args, "--input").unwrap_or_else(|| usage());
    let analysis_source = option_value(&args, "--analysis-source").unwrap_or_else(|| usage());
    let out_dir = option_value(&args, "--out-dir").unwrap_or_else(|| usage());
    let report: DegreeStudyReport = read_json(&input)?;
    let rows = validate(&report)?;
    let findings = findings(&rows);
    let raw_report = file_hash(&input)?;
    let analysis_source = file_hash(&analysis_source)?;
    let summary = Summary {
        schema: "ic.coordinate_rosetta.degree_summary/v1",
        evidence_status: report.evidence_status,
        raw_report,
        analysis_source,
        protocol_sha256: report.config.protocol_sha256,
        instrument_commit: report.config.instrument_commit,
        monomial_order: report.config.monomial_order,
        max_degree: report.config.max_degree,
        fixture_id: report.fixture.fixture_id,
        source_curve_id: report.fixture.source_curve_id,
        source_curve_uid: report.fixture.source_curve_uid,
        destination_curve_id: report.fixture.destination_curve_id,
        destination_curve_uid: report.fixture.destination_curve_uid,
        controls_passed: report.controls.passed(),
        findings,
        observations: report.observations,
        solving_degree_convention: report.solving_degree_convention,
        regularity_convention: report.regularity_convention,
        limitations: report.limitations,
    };
    fs::create_dir_all(out_dir.join("figures"))
        .map_err(|error| format!("{}: {error}", out_dir.display()))?;
    let json = serde_json::to_string_pretty(&summary).map_err(|error| error.to_string())?;
    fs::write(out_dir.join("summary.json"), format!("{json}\n"))
        .map_err(|error| error.to_string())?;
    fs::write(
        out_dir.join("REPORT.md"),
        markdown(&summary, &report.fixture.hessian_exclusion),
    )
    .map_err(|error| error.to_string())?;
    fs::write(
        out_dir.join("figures/degree_comparison.svg"),
        svg(&summary.observations),
    )
    .map_err(|error| error.to_string())?;
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("degree report refused: {error}");
        std::process::exit(2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn observation(name: &str, input: u32, total: u32, pairs: usize) -> Observation {
        Observation {
            name: name.into(),
            coordinate_model: name.into(),
            control_role: "test".into(),
            expected_outcome: if name.ends_with("unsatisfiable-control") {
                "inconsistent".into()
            } else {
                "planted_solution".into()
            },
            variables: 2,
            equations: 3,
            input_monomials: 12,
            max_input_degree: input,
            semi_regular_degree: Some(if name == "edwards-affine-bilinear" {
                4
            } else {
                3
            }),
            known_solution_valid: !name.ends_with("unsatisfiable-control"),
            solver_complete: true,
            expected_outcome_passed: true,
            inconsistent: name.ends_with("unsatisfiable-control"),
            pairs_above_bound: 0,
            selected_pairs: pairs,
            f4_steps: 2,
            productive_solving_degree: total,
            total_solving_degree: total,
            raw_max_step_degree: total,
            affine_regularity_index: if name.ends_with("unsatisfiable-control") {
                None
            } else {
                Some(2)
            },
            basis_max_degree: 2,
            basis_polynomials: 2,
            basis_monomials: 4,
            max_rows: 8,
            max_columns: 12,
            field_multiplications: 20,
        }
    }

    #[test]
    fn frozen_comparisons_are_reduced_without_wall_time() {
        let mut observations = vec![
            observation("montgomery-u-semaev", 4, 4, 4),
            observation("montgomery-u-aa", 4, 4, 4),
            observation("weierstrass-x-semaev", 4, 4, 4),
            observation("montgomery-affine-slope", 3, 3, 12),
            observation("quotient-w-squarefree", 4, 4, 4),
            observation("quotient-w-raw-norm", 16, 16, 4),
            observation("edwards-y-semaev", 4, 4, 4),
            observation("edwards-affine-bilinear", 2, 4, 20),
            observation("montgomery-u-unsatisfiable-control", 4, 4, 4),
        ];
        observations[7].basis_max_degree = 3;
        let report = DegreeStudyReport {
            schema: "ic.coordinate_rosetta.degree_study/v1".into(),
            evidence_status: "pcc:unverified".into(),
            config: StudyConfig {
                protocol_sha256: "1".repeat(64),
                instrument_commit: "2".repeat(40),
                monomial_order: "grevlex".into(),
                max_degree: 32,
                wall_time_admissible: false,
            },
            fixture: Fixture {
                fixture_id: "test".into(),
                source_curve_id: "source".into(),
                source_curve_uid: "source-uid".into(),
                destination_curve_id: "destination".into(),
                destination_curve_uid: "destination-uid".into(),
                source_group_order: 1_004,
                subgroup_order: 251,
                subgroup_map_passed: true,
                raw_norm_identity_passed: true,
                hessian_included: false,
                hessian_exclusion: "order is not divisible by three".into(),
            },
            observations,
            controls: Controls {
                inventory_passed: true,
                aa_observation_identical: true,
                positive_controls_passed: true,
                negative_control_passed: true,
                all_solver_runs_complete: true,
            },
            solving_degree_convention: "test".into(),
            regularity_convention: "test".into(),
            limitations: vec!["test fixture".into()],
        };
        let rows = validate(&report).expect("synthetic report validates");
        let findings = findings(&rows);
        assert!(findings.squarefree_w_matches_u_degree_profile);
        assert_eq!(findings.raw_norm_total_degree_increase, 12);
        assert_eq!(findings.edwards_bilinear_input_degree_reduction, 2);
        assert_eq!(findings.edwards_bilinear_selected_pair_difference, 16);
        assert!(findings.montgomery_affine_is_unique_minimum_total_degree);
        assert!(svg(&report.observations).contains("[pcc:unverified]"));
    }
}
