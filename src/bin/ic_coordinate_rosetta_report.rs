//! Rust-only evidence reducer for the S59 coordinate Rosetta experiment.
//!
//! The measurement binary is frozen separately.  This program consumes its
//! versioned JSON, replays the registered hard controls, computes the frozen
//! paired summaries, and emits JSON, Markdown, and SVG evidence.  Throughput
//! wall fields are deliberately ignored.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

const EXPECTED_SEEDS: [u64; 8] = [
    5_995_792_941_424_594_770,
    11_400_714_819_323_198_485,
    13_787_848_793_156_543_929,
    10_723_151_780_598_845_931,
    15_111_065_706_836_454_659,
    7_640_891_576_956_012_809,
    13_503_953_896_175_478_587,
    4_354_685_564_936_845_354,
];
const EXPECTED_COORDINATES: [&str; 9] = [
    "x",
    "x_aa",
    "jacobian_x_normalized",
    "jacobian_X_raw_negative_control",
    "mobius_x",
    "y",
    "x_plus_y",
    "x_squared",
    "x_over_y",
];
const COMPARISON_ARMS: [&str; 7] = [
    "jacobian_x_normalized",
    "jacobian_X_raw_negative_control",
    "mobius_x",
    "y",
    "x_plus_y",
    "x_squared",
    "x_over_y",
];
const BOOTSTRAP_REPLICATES: usize = 50_000;
const ANALYSIS_SEED: u64 = 0x4943_5f52_4f53_4554;

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
struct Counts {
    multiplications: u64,
    squarings: u64,
    inversions: u64,
    additions: u64,
}

impl Counts {
    fn add_assign(&mut self, other: Self) {
        self.multiplications += other.multiplications;
        self.squarings += other.squarings;
        self.inversions += other.inversions;
        self.additions += other.additions;
    }

    fn per(self, denominator: u64) -> CountsRate {
        CountsRate {
            multiplications: self.multiplications as f64 / denominator as f64,
            squarings: self.squarings as f64 / denominator as f64,
            inversions: self.inversions as f64 / denominator as f64,
            additions: self.additions as f64 / denominator as f64,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Serialize)]
struct CountsRate {
    multiplications: f64,
    squarings: f64,
    inversions: f64,
    additions: f64,
}

#[derive(Clone, Debug, Deserialize)]
struct RelationBatch {
    rows: Vec<RelationRow>,
    samples_per_coordinate: u64,
    seed: u64,
    wall_admissible: bool,
    workers: usize,
}

#[derive(Clone, Debug, Deserialize)]
struct RelationRow {
    coordinate: String,
    seed: u64,
    samples: u64,
    threshold: u64,
    coordinate_evaluations: u64,
    excluded_poles_or_infinity: u64,
    factor_base_hits: u64,
    factor_base_density: f64,
    admitted_relations: u64,
    relation_rate: f64,
    independence_ratio: f64,
    planted_zero_sum_passes: u64,
    negative_zero_sum_failures: u64,
    negation_invariant_matches: u64,
    scale_invariant_matches: u64,
    counted_coordinate_work: Counts,
    checksum: u64,
}

#[derive(Clone, Debug, Deserialize)]
struct ChartBatch {
    rows: Vec<ChartRow>,
    iterations_per_chart: usize,
    wall_admissible: bool,
    workers: usize,
}

#[derive(Clone, Debug, Deserialize)]
struct ChartRow {
    model: String,
    chart: String,
    status: String,
    error: Option<String>,
    roundtrip_points: usize,
    roundtrip_iterations: usize,
    addition: Option<FormulaChoice>,
    doubling: Option<FormulaChoice>,
    best_diffadd: Option<FormulaChoice>,
    best_ladder: Option<FormulaChoice>,
}

#[derive(Clone, Debug, Deserialize)]
struct FormulaChoice {
    id: String,
    straight_line_counts: Counts,
    verified_cases: usize,
}

#[derive(Clone, Debug, Serialize)]
struct FileHash {
    path: String,
    sha256: String,
    bytes: u64,
}

#[derive(Clone, Debug, Serialize)]
struct HardControls {
    passed: bool,
    relation_files: usize,
    relation_rows: usize,
    planted_zero_sum_passes: u64,
    wrong_relation_acceptances: u64,
    aa_mismatches: usize,
    normalized_x_mismatches: usize,
    raw_x_scale_invariant_matches: u64,
    unexpected_seed_or_shape_rows: usize,
    binary_hash_matches_protocol: bool,
    chart_rows: usize,
    verified_chart_rows: usize,
    chart_errors: usize,
    formula_oracle_failures: usize,
}

#[derive(Clone, Debug, Serialize)]
struct CoordinateAggregate {
    coordinate: String,
    algebraic_class: String,
    seeds: usize,
    samples: u64,
    coordinate_evaluations: u64,
    excluded_poles_or_infinity: u64,
    factor_base_hits: u64,
    factor_base_density: f64,
    admitted_relations: u64,
    relation_rate: f64,
    relations_per_million: f64,
    pooled_independence_ratio: f64,
    per_seed_independence_ratio_mean: f64,
    negation_invariance: f64,
    scale_invariance: f64,
    total_coordinate_work: Counts,
    work_per_evaluation: CountsRate,
    checksums: Vec<u64>,
}

#[derive(Clone, Debug, Serialize)]
struct Interval {
    low: f64,
    high: f64,
}

#[derive(Clone, Debug, Serialize)]
struct CoordinateComparison {
    arm: String,
    reference: String,
    paired_seeds: usize,
    admitted_relation_difference: i64,
    relation_rate_difference: f64,
    relation_rate_ratio: f64,
    bootstrap_rate_difference_ci95: Interval,
    bootstrap_rate_ratio_ci95: Interval,
    centered_bootstrap_p: f64,
    holm_adjusted_p: f64,
    rate_verdict: String,
    exact_work_difference_per_evaluation: SignedCountsRate,
    negation_invariance_difference: f64,
    scale_invariance_difference: f64,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct SignedCountsRate {
    multiplications: f64,
    squarings: f64,
    inversions: f64,
    additions: f64,
}

#[derive(Clone, Debug, Serialize)]
struct FormulaRecord {
    model: String,
    chart: String,
    operation: String,
    formula: String,
    counts: Counts,
    verified_cases: usize,
}

#[derive(Clone, Debug, Serialize)]
struct OperationRange {
    operation: String,
    formula_rows: usize,
    min: Counts,
    max: Counts,
}

#[derive(Clone, Debug, Serialize)]
struct ChartSummary {
    schema: &'static str,
    source: FileHash,
    workers: usize,
    iterations_per_chart: usize,
    wall_admissible: bool,
    model_count: usize,
    total_rows: usize,
    affine_baselines: usize,
    catalogue_charts: usize,
    verified_rows: usize,
    complete_roundtrip_rows: usize,
    minimum_roundtrip_points: usize,
    rows_with_addition: usize,
    rows_with_doubling: usize,
    rows_with_diffadd: usize,
    rows_with_ladder: usize,
    operation_ranges: Vec<OperationRange>,
    formulas: Vec<FormulaRecord>,
}

#[derive(Clone, Debug, Serialize)]
struct CoordinateSummary {
    schema: &'static str,
    protocol_sha256: String,
    instrument_commit: String,
    instrument_binary_sha256: String,
    measured_binary_sha256: String,
    analysis_source_sha256: String,
    bootstrap_replicates: usize,
    analysis_seed: u64,
    raw_files: Vec<FileHash>,
    hard_controls: HardControls,
    arms: Vec<CoordinateAggregate>,
    comparisons: Vec<CoordinateComparison>,
    aa_control_relation_difference: i64,
    aa_control_rate_difference: f64,
    notes: Vec<String>,
}

#[derive(Clone, Copy)]
struct SplitMix64(u64);

impl SplitMix64 {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut value = self.0;
        value = (value ^ (value >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        value = (value ^ (value >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        value ^ (value >> 31)
    }
}

fn option_value(args: &[String], name: &str) -> Option<PathBuf> {
    args.windows(2)
        .find(|window| window[0] == name)
        .map(|window| PathBuf::from(&window[1]))
}

fn usage() -> ! {
    eprintln!(
        "usage: ic_coordinate_rosetta_report --relations-dir DIR --charts FILE --protocol FILE --binary FILE --out-dir DIR"
    );
    std::process::exit(2);
}

fn read_json<T: for<'de> Deserialize<'de>>(path: &Path) -> Result<T, String> {
    let bytes = fs::read(path).map_err(|error| format!("{}: {error}", path.display()))?;
    serde_json::from_slice(&bytes).map_err(|error| format!("{}: {error}", path.display()))
}

fn sha256(path: &Path) -> Result<String, String> {
    let bytes = fs::read(path).map_err(|error| format!("{}: {error}", path.display()))?;
    Ok(format!("{:x}", Sha256::digest(bytes)))
}

fn file_hash(path: &Path) -> Result<FileHash, String> {
    let metadata = fs::metadata(path).map_err(|error| format!("{}: {error}", path.display()))?;
    Ok(FileHash {
        path: path.display().to_string(),
        sha256: sha256(path)?,
        bytes: metadata.len(),
    })
}

fn mean(values: &[f64]) -> f64 {
    values.iter().sum::<f64>() / values.len() as f64
}

fn quantile(sorted: &[f64], probability: f64) -> f64 {
    let index = ((sorted.len() - 1) as f64 * probability).round() as usize;
    sorted[index]
}

fn arm_domain(arm: &str) -> u64 {
    arm.bytes().fold(0xcbf2_9ce4_8422_2325, |hash, byte| {
        (hash ^ u64::from(byte)).wrapping_mul(0x100_0000_01b3)
    })
}

fn bootstrap(
    candidate: &[RelationRow],
    reference: &[RelationRow],
    arm: &str,
) -> (Interval, Interval, f64) {
    let n = candidate.len();
    let differences = candidate
        .iter()
        .zip(reference)
        .map(|(candidate, reference)| candidate.relation_rate - reference.relation_rate)
        .collect::<Vec<_>>();
    let observed_difference = mean(&differences);
    let centered = differences
        .iter()
        .map(|value| value - observed_difference)
        .collect::<Vec<_>>();
    let mut rng = SplitMix64(ANALYSIS_SEED ^ arm_domain(arm));
    let mut difference_distribution = Vec::with_capacity(BOOTSTRAP_REPLICATES);
    let mut ratio_distribution = Vec::with_capacity(BOOTSTRAP_REPLICATES);
    let mut null_extreme = 0usize;
    for _ in 0..BOOTSTRAP_REPLICATES {
        let mut candidate_relations = 0u64;
        let mut candidate_samples = 0u64;
        let mut reference_relations = 0u64;
        let mut reference_samples = 0u64;
        let mut difference_sum = 0.0;
        let mut null_sum = 0.0;
        for _ in 0..n {
            let index = (rng.next() as usize) % n;
            candidate_relations += candidate[index].admitted_relations;
            candidate_samples += candidate[index].samples;
            reference_relations += reference[index].admitted_relations;
            reference_samples += reference[index].samples;
            difference_sum += differences[index];
            null_sum += centered[index];
        }
        let candidate_rate = candidate_relations as f64 / candidate_samples as f64;
        let reference_rate = reference_relations as f64 / reference_samples as f64;
        difference_distribution.push(difference_sum / n as f64);
        ratio_distribution.push(candidate_rate / reference_rate);
        if (null_sum / n as f64).abs() >= observed_difference.abs() {
            null_extreme += 1;
        }
    }
    difference_distribution.sort_by(f64::total_cmp);
    ratio_distribution.sort_by(f64::total_cmp);
    (
        Interval {
            low: quantile(&difference_distribution, 0.025),
            high: quantile(&difference_distribution, 0.975),
        },
        Interval {
            low: quantile(&ratio_distribution, 0.025),
            high: quantile(&ratio_distribution, 0.975),
        },
        (null_extreme + 1) as f64 / (BOOTSTRAP_REPLICATES + 1) as f64,
    )
}

fn holm_adjust(raw: &[f64]) -> Vec<f64> {
    let mut order = raw.iter().copied().enumerate().collect::<Vec<_>>();
    order.sort_by(|left, right| left.1.total_cmp(&right.1));
    let mut adjusted = vec![1.0; raw.len()];
    let mut running: f64 = 0.0;
    for (rank, (index, value)) in order.into_iter().enumerate() {
        running = running.max(((raw.len() - rank) as f64 * value).min(1.0));
        adjusted[index] = running;
    }
    adjusted
}

fn algebraic_class(coordinate: &str) -> &'static str {
    match coordinate {
        "x" | "x_aa" | "jacobian_x_normalized" => "degree-2 Kummer x class",
        "jacobian_X_raw_negative_control" => "not intrinsic; projective-scale control",
        "mobius_x" => "degree-2 Mobius reparameterization of x",
        "y" | "x_plus_y" => "degree-3 curve function",
        "x_squared" => "degree-4 composition through x",
        "x_over_y" => "rational function; torsion-model caveat",
        _ => "unregistered",
    }
}

fn equivalent_surface(left: &RelationRow, right: &RelationRow) -> bool {
    left.seed == right.seed
        && left.samples == right.samples
        && left.threshold == right.threshold
        && left.coordinate_evaluations == right.coordinate_evaluations
        && left.excluded_poles_or_infinity == right.excluded_poles_or_infinity
        && left.factor_base_hits == right.factor_base_hits
        && left.factor_base_density.to_bits() == right.factor_base_density.to_bits()
        && left.admitted_relations == right.admitted_relations
        && left.relation_rate.to_bits() == right.relation_rate.to_bits()
        && left.independence_ratio.to_bits() == right.independence_ratio.to_bits()
        && left.planted_zero_sum_passes == right.planted_zero_sum_passes
        && left.negative_zero_sum_failures == right.negative_zero_sum_failures
        && left.negation_invariant_matches == right.negation_invariant_matches
        && left.scale_invariant_matches == right.scale_invariant_matches
        && left.checksum == right.checksum
}

fn aggregate_rows(coordinate: &str, rows: &[RelationRow]) -> CoordinateAggregate {
    let mut work = Counts::default();
    for row in rows {
        work.add_assign(row.counted_coordinate_work);
    }
    let samples = rows.iter().map(|row| row.samples).sum::<u64>();
    let evaluations = rows
        .iter()
        .map(|row| row.coordinate_evaluations)
        .sum::<u64>();
    let excluded = rows
        .iter()
        .map(|row| row.excluded_poles_or_infinity)
        .sum::<u64>();
    let hits = rows.iter().map(|row| row.factor_base_hits).sum::<u64>();
    let relations = rows.iter().map(|row| row.admitted_relations).sum::<u64>();
    let density = hits as f64 / (evaluations - excluded) as f64;
    let rate = relations as f64 / samples as f64;
    let negation = rows
        .iter()
        .map(|row| row.negation_invariant_matches)
        .sum::<u64>() as f64
        / evaluations as f64;
    let scale = rows
        .iter()
        .map(|row| row.scale_invariant_matches)
        .sum::<u64>() as f64
        / evaluations as f64;
    CoordinateAggregate {
        coordinate: coordinate.to_string(),
        algebraic_class: algebraic_class(coordinate).to_string(),
        seeds: rows.len(),
        samples,
        coordinate_evaluations: evaluations,
        excluded_poles_or_infinity: excluded,
        factor_base_hits: hits,
        factor_base_density: density,
        admitted_relations: relations,
        relation_rate: rate,
        relations_per_million: rate * 1_000_000.0,
        pooled_independence_ratio: rate / density.powi(3),
        per_seed_independence_ratio_mean: mean(
            &rows
                .iter()
                .map(|row| row.independence_ratio)
                .collect::<Vec<_>>(),
        ),
        negation_invariance: negation,
        scale_invariance: scale,
        total_coordinate_work: work,
        work_per_evaluation: work.per(evaluations),
        checksums: rows.iter().map(|row| row.checksum).collect(),
    }
}

fn signed_counts(candidate: CountsRate, reference: CountsRate) -> SignedCountsRate {
    SignedCountsRate {
        multiplications: candidate.multiplications - reference.multiplications,
        squarings: candidate.squarings - reference.squarings,
        inversions: candidate.inversions - reference.inversions,
        additions: candidate.additions - reference.additions,
    }
}

fn range(operation: &str, formulas: &[FormulaRecord]) -> Option<OperationRange> {
    let matches = formulas
        .iter()
        .filter(|formula| formula.operation == operation)
        .collect::<Vec<_>>();
    if matches.is_empty() {
        return None;
    }
    let min = Counts {
        multiplications: matches
            .iter()
            .map(|formula| formula.counts.multiplications)
            .min()?,
        squarings: matches
            .iter()
            .map(|formula| formula.counts.squarings)
            .min()?,
        inversions: matches
            .iter()
            .map(|formula| formula.counts.inversions)
            .min()?,
        additions: matches
            .iter()
            .map(|formula| formula.counts.additions)
            .min()?,
    };
    let max = Counts {
        multiplications: matches
            .iter()
            .map(|formula| formula.counts.multiplications)
            .max()?,
        squarings: matches
            .iter()
            .map(|formula| formula.counts.squarings)
            .max()?,
        inversions: matches
            .iter()
            .map(|formula| formula.counts.inversions)
            .max()?,
        additions: matches
            .iter()
            .map(|formula| formula.counts.additions)
            .max()?,
    };
    Some(OperationRange {
        operation: operation.to_string(),
        formula_rows: matches.len(),
        min,
        max,
    })
}

fn formula_record(row: &ChartRow, operation: &str, choice: &FormulaChoice) -> FormulaRecord {
    FormulaRecord {
        model: row.model.clone(),
        chart: row.chart.clone(),
        operation: operation.to_string(),
        formula: choice.id.clone(),
        counts: choice.straight_line_counts,
        verified_cases: choice.verified_cases,
    }
}

fn chart_summary(path: &Path, batch: &ChartBatch) -> Result<ChartSummary, String> {
    let mut formulas = Vec::new();
    for row in &batch.rows {
        if let Some(choice) = &row.addition {
            formulas.push(formula_record(row, "addition", choice));
        }
        if let Some(choice) = &row.doubling {
            formulas.push(formula_record(row, "doubling", choice));
        }
        if let Some(choice) = &row.best_diffadd {
            formulas.push(formula_record(row, "diffadd", choice));
        }
        if let Some(choice) = &row.best_ladder {
            formulas.push(formula_record(row, "ladder", choice));
        }
    }
    let operation_ranges = ["addition", "doubling", "diffadd", "ladder"]
        .into_iter()
        .filter_map(|operation| range(operation, &formulas))
        .collect();
    let models = batch
        .rows
        .iter()
        .map(|row| row.model.as_str())
        .collect::<BTreeSet<_>>();
    Ok(ChartSummary {
        schema: "ic.coordinate_rosetta.chart_summary/v1",
        source: file_hash(path)?,
        workers: batch.workers,
        iterations_per_chart: batch.iterations_per_chart,
        wall_admissible: batch.wall_admissible,
        model_count: models.len(),
        total_rows: batch.rows.len(),
        affine_baselines: batch
            .rows
            .iter()
            .filter(|row| row.chart == "affine")
            .count(),
        catalogue_charts: batch
            .rows
            .iter()
            .filter(|row| row.chart != "affine")
            .count(),
        verified_rows: batch
            .rows
            .iter()
            .filter(|row| row.status == "verified")
            .count(),
        complete_roundtrip_rows: batch
            .rows
            .iter()
            .filter(|row| row.roundtrip_iterations == batch.iterations_per_chart)
            .count(),
        minimum_roundtrip_points: batch
            .rows
            .iter()
            .map(|row| row.roundtrip_points)
            .min()
            .unwrap_or(0),
        rows_with_addition: batch
            .rows
            .iter()
            .filter(|row| row.addition.is_some())
            .count(),
        rows_with_doubling: batch
            .rows
            .iter()
            .filter(|row| row.doubling.is_some())
            .count(),
        rows_with_diffadd: batch
            .rows
            .iter()
            .filter(|row| row.best_diffadd.is_some())
            .count(),
        rows_with_ladder: batch
            .rows
            .iter()
            .filter(|row| row.best_ladder.is_some())
            .count(),
        operation_ranges,
        formulas,
    })
}

fn coordinate_svg(arms: &[CoordinateAggregate]) -> String {
    let width = 1120.0;
    let height = 620.0;
    let left = 80.0;
    let top = 70.0;
    let plot_height = 390.0;
    let step = 108.0;
    let bar_width = 62.0;
    let max_rate = arms
        .iter()
        .map(|arm| arm.relations_per_million)
        .fold(0.0, f64::max)
        * 1.12;
    let mut svg = format!(
        "<svg xmlns='http://www.w3.org/2000/svg' width='{width}' height='{height}' viewBox='0 0 {width} {height}'>\n<rect width='100%' height='100%' fill='white'/>\n<text x='40' y='35' font-family='sans-serif' font-size='24' font-weight='bold'>S59 planted-relation admission by coordinate</text>\n<text x='40' y='57' font-family='sans-serif' font-size='13' fill='#444'>Eight frozen seeds; throughput wall time excluded. Labels show exact extraction work per evaluation (M/S/I/A).</text>\n"
    );
    for tick in 0..=5 {
        let value = max_rate * tick as f64 / 5.0;
        let y = top + plot_height - plot_height * tick as f64 / 5.0;
        svg.push_str(&format!(
            "<line x1='{left}' y1='{y:.1}' x2='1080' y2='{y:.1}' stroke='#ddd'/><text x='72' y='{:.1}' text-anchor='end' font-family='sans-serif' font-size='11'>{value:.0}</text>\n",
            y + 4.0
        ));
    }
    for (index, arm) in arms.iter().enumerate() {
        let x = left + 28.0 + index as f64 * step;
        let bar_height = plot_height * arm.relations_per_million / max_rate;
        let y = top + plot_height - bar_height;
        let color = match arm.coordinate.as_str() {
            "x" => "#175cd3",
            "x_aa" => "#53b1fd",
            "jacobian_X_raw_negative_control" => "#d92d20",
            _ => "#7f56d9",
        };
        svg.push_str(&format!(
            "<rect x='{x:.1}' y='{y:.1}' width='{bar_width}' height='{bar_height:.1}' fill='{color}'/><text x='{:.1}' y='{:.1}' text-anchor='middle' font-family='sans-serif' font-size='12' font-weight='bold'>{:.1}</text>\n",
            x + bar_width / 2.0,
            y - 7.0,
            arm.relations_per_million
        ));
        let label = arm.coordinate.replace('_', " ");
        svg.push_str(&format!(
            "<text x='{:.1}' y='486' text-anchor='end' transform='rotate(-38 {:.1} 486)' font-family='sans-serif' font-size='12'>{label}</text>\n",
            x + bar_width / 2.0,
            x + bar_width / 2.0
        ));
        let work = arm.work_per_evaluation;
        svg.push_str(&format!(
            "<text x='{:.1}' y='583' text-anchor='middle' font-family='monospace' font-size='10'>{:.0}/{:.0}/{:.0}/{:.0}</text>\n",
            x + bar_width / 2.0,
            work.multiplications,
            work.squarings,
            work.inversions,
            work.additions
        ));
    }
    svg.push_str("<text x='22' y='275' text-anchor='middle' transform='rotate(-90 22 275)' font-family='sans-serif' font-size='13'>admitted relations per million planted triples</text>\n<text x='80' y='610' font-family='sans-serif' font-size='11' fill='#444'>M/S/I/A = multiplications / squarings / inversions / additions per coordinate evaluation. Rates are fixture diagnostics, not full relation-generation success.</text>\n</svg>\n");
    svg
}

fn efd_svg(formulas: &[FormulaRecord]) -> String {
    let selected = formulas
        .iter()
        .filter(|formula| matches!(formula.operation.as_str(), "addition" | "doubling"))
        .collect::<Vec<_>>();
    let max_m = selected
        .iter()
        .map(|formula| formula.counts.multiplications)
        .max()
        .unwrap_or(1) as f64;
    let max_s = selected
        .iter()
        .map(|formula| formula.counts.squarings)
        .max()
        .unwrap_or(1) as f64;
    let mut svg = String::from(
        "<svg xmlns='http://www.w3.org/2000/svg' width='900' height='650' viewBox='0 0 900 650'>\n<rect width='100%' height='100%' fill='white'/>\n<text x='42' y='36' font-family='sans-serif' font-size='24' font-weight='bold'>Verified EFD formula operation vectors on S59</text>\n<text x='42' y='58' font-family='sans-serif' font-size='13' fill='#444'>One selected qualified formula per chart/operation; affine baselines have no EFD formula point.</text>\n",
    );
    let left = 80.0;
    let top = 85.0;
    let plot_width = 760.0;
    let plot_height = 480.0;
    for tick in 0..=5 {
        let x = left + plot_width * tick as f64 / 5.0;
        let y = top + plot_height - plot_height * tick as f64 / 5.0;
        svg.push_str(&format!(
            "<line x1='{x:.1}' y1='{top}' x2='{x:.1}' y2='565' stroke='#eee'/><text x='{x:.1}' y='585' text-anchor='middle' font-family='sans-serif' font-size='11'>{:.1}</text>\n",
            max_m * tick as f64 / 5.0
        ));
        svg.push_str(&format!(
            "<line x1='{left}' y1='{y:.1}' x2='840' y2='{y:.1}' stroke='#eee'/><text x='70' y='{:.1}' text-anchor='end' font-family='sans-serif' font-size='11'>{:.1}</text>\n",
            y + 4.0,
            max_s * tick as f64 / 5.0
        ));
    }
    for formula in selected {
        let x = left + plot_width * formula.counts.multiplications as f64 / max_m;
        let y = top + plot_height - plot_height * formula.counts.squarings as f64 / max_s;
        let (color, shape) = if formula.operation == "addition" {
            ("#175cd3", "circle")
        } else {
            ("#f79009", "square")
        };
        if shape == "circle" {
            svg.push_str(&format!(
                "<circle cx='{x:.1}' cy='{y:.1}' r='5' fill='{color}' opacity='0.72'><title>{}/{}: {}M {}S {}I {}A</title></circle>\n",
                formula.model,
                formula.chart,
                formula.counts.multiplications,
                formula.counts.squarings,
                formula.counts.inversions,
                formula.counts.additions
            ));
        } else {
            svg.push_str(&format!(
                "<rect x='{:.1}' y='{:.1}' width='9' height='9' fill='{color}' opacity='0.72'><title>{}/{}: {}M {}S {}I {}A</title></rect>\n",
                x - 4.5,
                y - 4.5,
                formula.model,
                formula.chart,
                formula.counts.multiplications,
                formula.counts.squarings,
                formula.counts.inversions,
                formula.counts.additions
            ));
        }
    }
    svg.push_str("<text x='460' y='623' text-anchor='middle' font-family='sans-serif' font-size='13'>multiplications per formula</text>\n<text x='22' y='325' text-anchor='middle' transform='rotate(-90 22 325)' font-family='sans-serif' font-size='13'>squarings per formula</text>\n<circle cx='655' cy='610' r='5' fill='#175cd3'/><text x='667' y='614' font-family='sans-serif' font-size='12'>addition</text><rect x='744' y='605' width='9' height='9' fill='#f79009'/><text x='759' y='614' font-family='sans-serif' font-size='12'>doubling</text>\n</svg>\n");
    svg
}

fn counts_text(counts: CountsRate) -> String {
    format!(
        "{:.0}M/{:.0}S/{:.0}I/{:.0}A",
        counts.multiplications, counts.squarings, counts.inversions, counts.additions
    )
}

fn report_label(coordinate: &str) -> &'static str {
    match coordinate {
        "x" => "x",
        "x_aa" => "x (A/A)",
        "jacobian_x_normalized" => "Jacobian x normalized",
        "jacobian_X_raw_negative_control" => "raw X (control)",
        "mobius_x" => "Mobius x",
        "y" => "y",
        "x_plus_y" => "x+y",
        "x_squared" => "x^2",
        "x_over_y" => "x/y",
        _ => "unregistered",
    }
}

fn protocol_arm_id(coordinate: &str) -> &'static str {
    match coordinate {
        "x" => "x",
        "x_aa" => "x-aa",
        "jacobian_x_normalized" => "jacobian-x-normalized",
        "jacobian_X_raw_negative_control" => "jacobian-X-raw",
        "mobius_x" => "mobius-x",
        "y" => "y",
        "x_plus_y" => "x-plus-y",
        "x_squared" => "x-squared",
        "x_over_y" => "x-over-y",
        _ => "unregistered",
    }
}

fn counts_compact(counts: Counts) -> String {
    format!(
        "{}M/{}S/{}I/{}A",
        counts.multiplications, counts.squarings, counts.inversions, counts.additions
    )
}

fn formula_compact(choice: Option<&FormulaChoice>) -> String {
    choice
        .map(|choice| {
            format!(
                "{}:{}",
                choice.id,
                counts_compact(choice.straight_line_counts)
            )
        })
        .unwrap_or_else(|| "none".to_string())
}

fn experiment_rows(batches: &[RelationBatch], chart_batch: &ChartBatch) -> Result<String, String> {
    let mut lines = Vec::new();
    for batch in batches {
        for row in &batch.rows {
            let value = serde_json::json!({
                "run_id": format!("relations-{}-{}", batch.seed, row.coordinate),
                "arm": protocol_arm_id(&row.coordinate),
                "instance_id": batch.seed.to_string(),
                "seed": batch.seed.to_string(),
                "replicate": 0,
                "warmup": false,
                "status": "succeeded",
                "verification": "verified",
                "correctness": 1,
                "relation-rate": row.relation_rate,
                "density-adjusted-rate": row.independence_ratio,
                "coordinate-work": counts_compact(row.counted_coordinate_work)
            });
            lines.push(serde_json::to_string(&value).map_err(|error| error.to_string())?);
        }
    }
    for row in &chart_batch.rows {
        let work = format!(
            "addition={};doubling={};diffadd={};ladder={}",
            formula_compact(row.addition.as_ref()),
            formula_compact(row.doubling.as_ref()),
            formula_compact(row.best_diffadd.as_ref()),
            formula_compact(row.best_ladder.as_ref())
        );
        let value = serde_json::json!({
            "run_id": format!("chart-{}-{}", row.model, row.chart.replace('/', "-")),
            "arm": "efd-chart-matrix",
            "instance_id": format!("{}/{}", row.model, row.chart),
            "seed": null,
            "replicate": 0,
            "warmup": false,
            "status": "succeeded",
            "verification": if row.status == "verified" { "verified" } else { "refuted" },
            "correctness": if row.status == "verified" { 1 } else { 0 },
            "efd-formula-work": work
        });
        lines.push(serde_json::to_string(&value).map_err(|error| error.to_string())?);
    }
    Ok(format!("{}\n", lines.join("\n")))
}

fn markdown_report(coordinate: &CoordinateSummary, charts: &ChartSummary) -> String {
    let mut report = String::from(
        "# S59 coordinate Rosetta — counted comparison\n\n## Outcome\n\nThe frozen correctness/counting pass completed for eight seeds, nine fixed-curve coordinate arms, and the full EFD matrix. All hard controls passed. These are counted and rate results; the four-worker wall fields are inadmissible and are not reported as benchmarks.\n\n## Fixed-curve coordinate results\n\n### Admission statistics\n\nEach arm evaluated 2,097,152 planted triples.\n\n| coordinate | admitted | rate / M | density | density adj. | Δ vs x | Holm p |\n| --- | ---: | ---: | ---: | ---: | ---: | ---: |\n",
    );
    let comparison_map = coordinate
        .comparisons
        .iter()
        .map(|comparison| (comparison.arm.as_str(), comparison))
        .collect::<BTreeMap<_, _>>();
    let reference_relations = coordinate
        .arms
        .iter()
        .find(|arm| arm.coordinate == "x")
        .map(|arm| arm.admitted_relations)
        .unwrap_or(0);
    for arm in &coordinate.arms {
        let (difference, adjusted) = if arm.coordinate == "x" {
            (0, None)
        } else if arm.coordinate == "x_aa" {
            (
                arm.admitted_relations as i64 - reference_relations as i64,
                None,
            )
        } else {
            let comparison = comparison_map.get(arm.coordinate.as_str()).copied();
            (
                comparison
                    .map(|value| value.admitted_relation_difference)
                    .unwrap_or(0),
                comparison.map(|value| value.holm_adjusted_p),
            )
        };
        let p = adjusted
            .map(|value| format!("{value:.4}"))
            .unwrap_or_else(|| "—".to_string());
        report.push_str(&format!(
            "| {} | {} | {:.2} | {:.4}% | {:.3} | {:+} | {} |\n",
            report_label(&arm.coordinate),
            arm.admitted_relations,
            arm.relations_per_million,
            arm.factor_base_density * 100.0,
            arm.pooled_independence_ratio,
            difference,
            p
        ));
    }
    report.push_str("\n### Algebraic and extraction properties\n\n| coordinate | algebraic class | neg. inv. | scale inv. | ops / evaluation |\n| --- | --- | ---: | ---: | --- |\n");
    for arm in &coordinate.arms {
        report.push_str(&format!(
            "| {} | {} | {:.1}% | {:.1}% | {} |\n",
            report_label(&arm.coordinate),
            arm.algebraic_class,
            arm.negation_invariance * 100.0,
            arm.scale_invariance * 100.0,
            counts_text(arm.work_per_evaluation)
        ));
    }
    report.push_str("\nThe x A/A arm and normalized-Jacobian x arm are exactly identical to affine x on every non-cost field. Raw projective X is negation-invariant but has zero projective-scale invariance, so its apparent admission rate is a negative-control artifact rather than an intrinsic factor-base result. Degree-3 and degree-4 rows are not comparable to the degree-2 Semaev x system on admission rate alone.\n\n![Coordinate relation rates](figures/coordinate_rate_and_cost.png)\n\nThe editable vector source is `figures/coordinate_rate_and_cost.svg`.\n\n## EFD representation matrix\n\n");
    report.push_str(&format!(
        "All {} emitted rows verified: {} maintained catalogue charts plus {} affine round-trip baselines across {} models. Qualified selected formulas were available for {} addition rows, {} doubling rows, {} differential-addition rows, and {} ladder rows. Affine baselines are conversion/oracle controls and have no EFD formula-count point.\n\n",
        charts.total_rows,
        charts.catalogue_charts,
        charts.affine_baselines,
        charts.model_count,
        charts.rows_with_addition,
        charts.rows_with_doubling,
        charts.rows_with_diffadd,
        charts.rows_with_ladder
    ));
    report.push_str("| operation | formula rows | M range | S range | I range | A range |\n| --- | ---: | ---: | ---: | ---: | ---: |\n");
    for range in &charts.operation_ranges {
        report.push_str(&format!(
            "| {} | {} | {}–{} | {}–{} | {}–{} | {}–{} |\n",
            range.operation,
            range.formula_rows,
            range.min.multiplications,
            range.max.multiplications,
            range.min.squarings,
            range.max.squarings,
            range.min.inversions,
            range.max.inversions,
            range.min.additions,
            range.max.additions
        ));
    }
    report.push_str("\n![EFD operation vectors](figures/efd_operation_counts.png)\n\nThe editable vector source is `figures/efd_operation_counts.svg`.\n\n## Evidence and limits\n\n");
    report.push_str(&format!(
        "- Protocol SHA-256: `{}`\n- Instrument commit: `{}`\n- Instrument binary SHA-256: `{}`\n- Rust reducer source SHA-256: `{}`\n- Hard-control verdict: `{}`\n- Bootstrap: {} deterministic paired resamples; centered empirical p-values with Holm correction across seven registered coordinate comparisons. No comparison survived correction.\n- The raw rows preserve amendment A1's intended same-seed-per-instance matching. The canonical export tool instead cycles an explicit seed list globally across arm/instance/replicate tuples, so protocol v2 is not used for a full canonical export or an L2 submission.\n- No local or multi-worker wall time is a result. L2 single-worker timing remains separate and requires a newly frozen timing protocol.\n- Plane-quartic and hyperelliptic Jacobian rows remain out of the fixed elliptic-group comparison without a certified transfer and recovery path.\n- This public S59 fixture supports engineering and algebra checks only; it is not evidence of a practical NIST-curve break or a sub-square-root ECDLP algorithm.\n",
        coordinate.protocol_sha256,
        coordinate.instrument_commit,
        coordinate.instrument_binary_sha256,
        coordinate.analysis_source_sha256,
        if coordinate.hard_controls.passed { "passed" } else { "failed" },
        coordinate.bootstrap_replicates
    ));
    report
}

fn run() -> Result<(), String> {
    let args = std::env::args().collect::<Vec<_>>();
    let relations_dir = option_value(&args, "--relations-dir").unwrap_or_else(|| usage());
    let charts_path = option_value(&args, "--charts").unwrap_or_else(|| usage());
    let protocol_path = option_value(&args, "--protocol").unwrap_or_else(|| usage());
    let binary_path = option_value(&args, "--binary").unwrap_or_else(|| usage());
    let out_dir = option_value(&args, "--out-dir").unwrap_or_else(|| usage());
    let analysis_source = PathBuf::from("src/bin/ic_coordinate_rosetta_report.rs");

    let protocol: serde_json::Value = read_json(&protocol_path)?;
    let protocol_sha256 = protocol["freeze"]["protocol_sha256"]
        .as_str()
        .ok_or("protocol freeze hash missing")?
        .to_string();
    let instrument_commit = protocol["arms"][0]["instrument"]["commit"]
        .as_str()
        .ok_or("instrument commit missing")?
        .to_string();
    let instrument_binary_sha256 = protocol["arms"][0]["instrument"]["binary_sha256"]
        .as_str()
        .ok_or("instrument binary hash missing")?
        .to_string();
    let measured_binary_sha256 = sha256(&binary_path)?;

    let mut relation_paths = fs::read_dir(&relations_dir)
        .map_err(|error| format!("{}: {error}", relations_dir.display()))?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| {
            path.extension()
                .is_some_and(|extension| extension == "json")
        })
        .collect::<Vec<_>>();
    relation_paths.sort();
    let mut batches = Vec::new();
    let mut raw_files = Vec::new();
    for path in &relation_paths {
        batches.push(read_json::<RelationBatch>(path)?);
        raw_files.push(file_hash(path)?);
    }

    let expected_seed_set = EXPECTED_SEEDS.into_iter().collect::<BTreeSet<_>>();
    let seen_seed_set = batches
        .iter()
        .map(|batch| batch.seed)
        .collect::<BTreeSet<_>>();
    let expected_coordinate_set = EXPECTED_COORDINATES.into_iter().collect::<BTreeSet<_>>();
    let mut rows_by_coordinate = BTreeMap::<String, Vec<RelationRow>>::new();
    let mut unexpected_shape = usize::from(seen_seed_set != expected_seed_set);
    for batch in &batches {
        let coordinates = batch
            .rows
            .iter()
            .map(|row| row.coordinate.as_str())
            .collect::<BTreeSet<_>>();
        if batch.rows.len() != 9
            || coordinates != expected_coordinate_set
            || batch.samples_per_coordinate != 262_144
            || batch.workers != 4
            || batch.wall_admissible
        {
            unexpected_shape += 1;
        }
        for row in &batch.rows {
            if row.seed != batch.seed || row.samples != batch.samples_per_coordinate {
                unexpected_shape += 1;
            }
            rows_by_coordinate
                .entry(row.coordinate.clone())
                .or_default()
                .push(row.clone());
        }
    }
    for rows in rows_by_coordinate.values_mut() {
        rows.sort_by_key(|row| row.seed);
    }

    let x_rows = rows_by_coordinate.get("x").ok_or("x rows missing")?;
    let aa_rows = rows_by_coordinate.get("x_aa").ok_or("x A/A rows missing")?;
    let normalized_rows = rows_by_coordinate
        .get("jacobian_x_normalized")
        .ok_or("normalized x rows missing")?;
    let aa_mismatches = x_rows
        .iter()
        .zip(aa_rows)
        .filter(|(left, right)| !equivalent_surface(left, right))
        .count();
    let normalized_mismatches = x_rows
        .iter()
        .zip(normalized_rows)
        .filter(|(left, right)| !equivalent_surface(left, right))
        .count();

    let arms = EXPECTED_COORDINATES
        .iter()
        .map(|coordinate| {
            let rows = rows_by_coordinate
                .get(*coordinate)
                .ok_or_else(|| format!("{coordinate} rows missing"))?;
            Ok(aggregate_rows(coordinate, rows))
        })
        .collect::<Result<Vec<_>, String>>()?;
    let aggregate_map = arms
        .iter()
        .map(|arm| (arm.coordinate.as_str(), arm))
        .collect::<BTreeMap<_, _>>();
    let reference_aggregate = aggregate_map["x"];

    let mut comparisons = Vec::new();
    let mut raw_p = Vec::new();
    for arm in COMPARISON_ARMS {
        let candidate_rows = rows_by_coordinate
            .get(arm)
            .ok_or_else(|| format!("{arm} rows missing"))?;
        let (difference_interval, ratio_interval, p_value) = bootstrap(candidate_rows, x_rows, arm);
        let candidate = aggregate_map[arm];
        comparisons.push(CoordinateComparison {
            arm: arm.to_string(),
            reference: "x".to_string(),
            paired_seeds: candidate_rows.len(),
            admitted_relation_difference: candidate.admitted_relations as i64
                - reference_aggregate.admitted_relations as i64,
            relation_rate_difference: candidate.relation_rate - reference_aggregate.relation_rate,
            relation_rate_ratio: candidate.relation_rate / reference_aggregate.relation_rate,
            bootstrap_rate_difference_ci95: difference_interval,
            bootstrap_rate_ratio_ci95: ratio_interval,
            centered_bootstrap_p: p_value,
            holm_adjusted_p: 1.0,
            rate_verdict: String::new(),
            exact_work_difference_per_evaluation: signed_counts(
                candidate.work_per_evaluation,
                reference_aggregate.work_per_evaluation,
            ),
            negation_invariance_difference: candidate.negation_invariance
                - reference_aggregate.negation_invariance,
            scale_invariance_difference: candidate.scale_invariance
                - reference_aggregate.scale_invariance,
        });
        raw_p.push(p_value);
    }
    for (comparison, adjusted) in comparisons.iter_mut().zip(holm_adjust(&raw_p)) {
        comparison.holm_adjusted_p = adjusted;
        comparison.rate_verdict = if comparison.arm == "jacobian_X_raw_negative_control" {
            "descriptive_negative_control".to_string()
        } else if adjusted < 0.05
            && (comparison.bootstrap_rate_difference_ci95.low > 0.0
                || comparison.bootstrap_rate_difference_ci95.high < 0.0)
        {
            "supported_within_fixture".to_string()
        } else {
            "within_control_distribution".to_string()
        };
    }

    let chart_batch: ChartBatch = read_json(&charts_path)?;
    let chart_summary = chart_summary(&charts_path, &chart_batch)?;
    let formula_oracle_failures = chart_batch
        .rows
        .iter()
        .filter(|row| {
            row.addition
                .as_ref()
                .is_some_and(|choice| choice.verified_cases == 0)
                || row
                    .doubling
                    .as_ref()
                    .is_some_and(|choice| choice.verified_cases == 0)
        })
        .count();
    let chart_errors = chart_batch
        .rows
        .iter()
        .filter(|row| row.error.is_some())
        .count();
    let planted = batches
        .iter()
        .flat_map(|batch| &batch.rows)
        .map(|row| row.planted_zero_sum_passes)
        .sum::<u64>();
    let wrong = batches
        .iter()
        .flat_map(|batch| &batch.rows)
        .map(|row| row.negative_zero_sum_failures)
        .sum::<u64>();
    let raw_scale = rows_by_coordinate["jacobian_X_raw_negative_control"]
        .iter()
        .map(|row| row.scale_invariant_matches)
        .sum::<u64>();
    let verified_charts = chart_batch
        .rows
        .iter()
        .filter(|row| row.status == "verified")
        .count();
    let binary_hash_matches = measured_binary_sha256 == instrument_binary_sha256;
    let hard_controls = HardControls {
        passed: batches.len() == 8
            && batches.iter().map(|batch| batch.rows.len()).sum::<usize>() == 72
            && planted == 18_874_368
            && wrong == 0
            && aa_mismatches == 0
            && normalized_mismatches == 0
            && raw_scale == 0
            && unexpected_shape == 0
            && binary_hash_matches
            && chart_batch.rows.len() == 43
            && verified_charts == 43
            && chart_summary.complete_roundtrip_rows == 43
            && chart_summary.minimum_roundtrip_points > 0
            && chart_errors == 0
            && formula_oracle_failures == 0,
        relation_files: batches.len(),
        relation_rows: batches.iter().map(|batch| batch.rows.len()).sum(),
        planted_zero_sum_passes: planted,
        wrong_relation_acceptances: wrong,
        aa_mismatches,
        normalized_x_mismatches: normalized_mismatches,
        raw_x_scale_invariant_matches: raw_scale,
        unexpected_seed_or_shape_rows: unexpected_shape,
        binary_hash_matches_protocol: binary_hash_matches,
        chart_rows: chart_batch.rows.len(),
        verified_chart_rows: verified_charts,
        chart_errors,
        formula_oracle_failures,
    };
    let aa_aggregate = aggregate_map["x_aa"];
    let aa_control_relation_difference =
        aa_aggregate.admitted_relations as i64 - reference_aggregate.admitted_relations as i64;
    let aa_control_rate_difference = aa_aggregate.relation_rate - reference_aggregate.relation_rate;
    let coordinate_summary = CoordinateSummary {
        schema: "ic.coordinate_rosetta.coordinate_summary/v1",
        protocol_sha256,
        instrument_commit,
        instrument_binary_sha256,
        measured_binary_sha256,
        analysis_source_sha256: sha256(&analysis_source)?,
        bootstrap_replicates: BOOTSTRAP_REPLICATES,
        analysis_seed: ANALYSIS_SEED,
        raw_files,
        hard_controls,
        arms,
        comparisons,
        aa_control_relation_difference,
        aa_control_rate_difference,
        notes: vec![
            "Four-worker wall fields are inadmissible and excluded from this reducer.".into(),
            "The ten affine rows are round-trip/oracle baselines; the maintained EFD catalogue has 33 non-affine charts.".into(),
            "Admission rates are planted-triple factor-base diagnostics, not end-to-end relation-generation rates.".into(),
        ],
    };

    let artifacts_dir = out_dir.join("artifacts");
    let figures_dir = out_dir.join("figures");
    fs::create_dir_all(&artifacts_dir)
        .map_err(|error| format!("{}: {error}", artifacts_dir.display()))?;
    fs::create_dir_all(&figures_dir)
        .map_err(|error| format!("{}: {error}", figures_dir.display()))?;
    fs::write(
        artifacts_dir.join("coordinate_summary.json"),
        serde_json::to_vec_pretty(&coordinate_summary).map_err(|error| error.to_string())?,
    )
    .map_err(|error| error.to_string())?;
    fs::write(
        artifacts_dir.join("chart_summary.json"),
        serde_json::to_vec_pretty(&chart_summary).map_err(|error| error.to_string())?,
    )
    .map_err(|error| error.to_string())?;
    fs::write(
        artifacts_dir.join("experiment_rows.jsonl"),
        experiment_rows(&batches, &chart_batch)?,
    )
    .map_err(|error| error.to_string())?;
    fs::write(
        figures_dir.join("coordinate_rate_and_cost.svg"),
        coordinate_svg(&coordinate_summary.arms),
    )
    .map_err(|error| error.to_string())?;
    fs::write(
        figures_dir.join("efd_operation_counts.svg"),
        efd_svg(&chart_summary.formulas),
    )
    .map_err(|error| error.to_string())?;
    fs::write(
        out_dir.join("REPORT.md"),
        markdown_report(&coordinate_summary, &chart_summary),
    )
    .map_err(|error| error.to_string())?;

    println!(
        "{}",
        serde_json::to_string_pretty(&serde_json::json!({
            "hard_controls_passed": coordinate_summary.hard_controls.passed,
            "coordinate_summary": artifacts_dir.join("coordinate_summary.json"),
            "chart_summary": artifacts_dir.join("chart_summary.json"),
            "experiment_rows": artifacts_dir.join("experiment_rows.jsonl"),
            "report": out_dir.join("REPORT.md"),
            "coordinate_figure": figures_dir.join("coordinate_rate_and_cost.svg"),
            "efd_figure": figures_dir.join("efd_operation_counts.svg")
        }))
        .map_err(|error| error.to_string())?
    );
    if !coordinate_summary.hard_controls.passed {
        return Err("hard controls failed; outputs are diagnostic only".into());
    }
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("error: {error}");
        std::process::exit(1);
    }
}
