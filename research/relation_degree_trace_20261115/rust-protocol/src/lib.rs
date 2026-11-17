//! Calculation-only adapter for COMPACT_ANALYSIS_CONTRACT.md.
//! No process execution, fixture generation or execution-admission authority.
pub mod callgrind;
pub mod cas_formats;
use serde::{Deserialize, Serialize};

#[cfg(test)]
extern crate self as compact_speed_analysis;
#[cfg(test)]
#[path = "../tests/support/mod.rs"]
mod support;

const DRAWS: usize = 100_000;
const SEED: u64 = 20_261_115;
const CAMPAIGN_NS: u64 = 7_200_000_000_000;
const TARGET: f64 = 15.;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Arm {
    F4,
    Certified,
    Compact,
}
const ARMS: [Arm; 3] = [Arm::F4, Arm::Certified, Arm::Compact];
const CYCLE: [[Arm; 3]; 6] = [
    [Arm::F4, Arm::Certified, Arm::Compact],
    [Arm::Certified, Arm::Compact, Arm::F4],
    [Arm::Compact, Arm::F4, Arm::Certified],
    [Arm::Compact, Arm::Certified, Arm::F4],
    [Arm::Certified, Arm::F4, Arm::Compact],
    [Arm::F4, Arm::Compact, Arm::Certified],
];

/// Producer-supplied process/validation records. Digests are equality keys,
/// not proof of basis arithmetic or authentic measurement provenance.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Sample {
    pub arm: Arm,
    pub complete: bool,
    pub input_sha256: String,
    pub worker_sha256: String,
    pub validation_input_sha256: String,
    /// Parent interval from before launch through output consumption/admission.
    pub worker_wall_ns: u64,
    pub validation_wall_ns: u64,
    pub basis_sha256: String,
    pub external_basis_sha256: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Case {
    pub schema: String,
    pub prime: u64,
    pub variables: usize,
    pub seed: u64,
    pub input_sha256: String,
    pub worker_sha256: String,
    pub reference_basis_sha256: String,
    /// Mature original-input reference generation/conversion, charged per call.
    pub setup_wall_ns: u64,
    pub warmups: Vec<Sample>,
    /// Groups in F4/certified/compact order; each pair retains its call order.
    pub aa_pairs: [Vec<[Sample; 2]>; 3],
    /// Calls are in the frozen order for their block index.
    pub blocks: Vec<Vec<Sample>>,
}
#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct Statistics {
    pub samples: usize,
    pub median_paired_ratio: f64,
    pub percentile_95: [f64; 2],
    pub adjusted_lower_1_over_720: f64,
}
#[derive(Clone, Debug, Serialize)]
pub struct AaResult {
    pub arm: Arm,
    pub native: Statistics,
    pub full_cost: Statistics,
    pub admitted: bool,
}
#[derive(Clone, Debug, Serialize)]
pub struct Comparison {
    pub control: Arm,
    pub native: Statistics,
    pub full_cost: Statistics,
}
#[derive(Debug, Serialize)]
pub struct Report {
    pub schema: &'static str,
    pub prime: u64,
    pub variables: usize,
    pub seed: u64,
    pub input_sha256: String,
    pub worker_sha256: String,
    pub status: &'static str,
    pub reasons: Vec<String>,
    pub aa: Option<Vec<AaResult>>,
    pub comparisons: Option<Vec<Comparison>>,
    pub numerical_rule_passes: bool,
    pub execution_admission_verified: bool,
    pub family_confirmed: bool,
    pub standing: &'static str,
}
fn digest(s: &str) -> bool {
    s.len() == 64
        && s.bytes()
            .all(|c| c.is_ascii_digit() || (b'a'..=b'f').contains(&c))
}
fn paid(sample: &Sample, setup: u64) -> Result<u64, String> {
    if !sample.complete {
        return Err("incomplete/failed call supplies no numerical comparison".into());
    }
    if sample.worker_wall_ns == 0 || sample.validation_wall_ns == 0 {
        return Err("worker and validation costs must both be positive".into());
    }
    let sum = sample
        .worker_wall_ns
        .checked_add(sample.validation_wall_ns)
        .and_then(|s| s.checked_add(setup))
        .ok_or("charged duration overflow")?;
    if sum > CAMPAIGN_NS {
        return Err("charged duration exceeds original campaign bound".into());
    }
    Ok(sum)
}
fn validate(case: &Case) -> Result<(), String> {
    if case.schema != "compact-speed-transcript/v1"
        || case.prime != 2_147_483_647
        || ![3, 4, 6].contains(&case.variables)
        || ![101, 307, 911].contains(&case.seed)
    {
        return Err("unsupported schema/roster; fresh cases require family admission".into());
    }
    if !digest(&case.input_sha256)
        || !digest(&case.worker_sha256)
        || !digest(&case.reference_basis_sha256)
    {
        return Err("malformed input/worker/reference identity".into());
    }
    if case.setup_wall_ns == 0 || case.setup_wall_ns > CAMPAIGN_NS {
        return Err("positive bounded shared validation setup cost required".into());
    }
    if case.warmups.len() != 3 || case.warmups.iter().map(|s| s.arm).collect::<Vec<_>>() != ARMS {
        return Err("exactly one warmup per arm in F/E/C order required".into());
    }
    for (arm, pairs) in ARMS.iter().zip(&case.aa_pairs) {
        if pairs.len() != 7 || pairs.iter().flatten().any(|s| s.arm != *arm) {
            return Err("exactly seven A/A pairs per arm required".into());
        }
    }
    if case.blocks.len() != 21 {
        return Err(format!(
            "incomplete/invalid block count: {}; expected 21, no replacements",
            case.blocks.len()
        ));
    }
    for (index, block) in case.blocks.iter().enumerate() {
        if block.len() != 3 || block.iter().map(|s| s.arm).collect::<Vec<_>>() != CYCLE[index % 6] {
            return Err(format!("wrong arm count/order in block {index}"));
        }
    }
    let expected = &case.reference_basis_sha256;
    for sample in case
        .warmups
        .iter()
        .chain(case.aa_pairs.iter().flatten().flatten())
        .chain(case.blocks.iter().flatten())
    {
        paid(sample, case.setup_wall_ns)?;
        if sample.input_sha256 != case.input_sha256
            || sample.worker_sha256 != case.worker_sha256
            || sample.validation_input_sha256 != case.input_sha256
        {
            return Err("worker/validator input or worker identity mismatch".into());
        }
        if !digest(&sample.basis_sha256)
            || !digest(&sample.external_basis_sha256)
            || sample.basis_sha256 != *expected
            || sample.external_basis_sha256 != *expected
        {
            return Err("malformed/mismatched native or mature basis digest".into());
        }
    }
    Ok(())
}
fn next(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9e3779b97f4a7c15);
    let mut x = *state;
    x = (x ^ (x >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    x = (x ^ (x >> 27)).wrapping_mul(0x94d049bb133111eb);
    x ^ (x >> 31)
}
fn index(state: &mut u64, n: usize) -> usize {
    let n = n as u64;
    let threshold = n.wrapping_neg() % n;
    loop {
        let value = next(state);
        if value >= threshold {
            return (value % n) as usize;
        }
    }
}
fn median(values: &mut [f64]) -> f64 {
    let mid = values.len() / 2;
    *values.select_nth_unstable_by(mid, f64::total_cmp).1
}
fn nearest_rank(n: usize, numerator: usize, denominator: usize) -> usize {
    (n * numerator).div_ceil(denominator) - 1
}
/// Shared indices in each draw preserve pairing across controls AND scopes.
fn intervals<const K: usize>(values: &[[f64; K]]) -> [Statistics; K] {
    let n = values.len();
    assert!(n > 0 && n <= 25 && n % 2 == 1);
    let mut state = SEED;
    let mut draws: [Vec<f64>; K] = std::array::from_fn(|_| Vec::with_capacity(DRAWS));
    let mut sample = [[0.; K]; 25];
    for _ in 0..DRAWS {
        for slot in &mut sample[..n] {
            *slot = values[index(&mut state, n)];
        }
        for (k, distribution) in draws.iter_mut().enumerate() {
            let mut column = [0.; 25];
            for (slot, row) in column[..n].iter_mut().zip(&sample[..n]) {
                *slot = row[k];
            }
            distribution.push(median(&mut column[..n]));
        }
    }
    std::array::from_fn(|k| {
        draws[k].sort_unstable_by(f64::total_cmp);
        let mut original: Vec<_> = values.iter().map(|row| row[k]).collect();
        Statistics {
            samples: n,
            median_paired_ratio: median(&mut original),
            percentile_95: [
                draws[k][nearest_rank(DRAWS, 1, 40)],
                draws[k][nearest_rank(DRAWS, 39, 40)],
            ],
            adjusted_lower_1_over_720: draws[k][nearest_rank(DRAWS, 1, 720)],
        }
    })
}
fn includes_one(stats: &Statistics) -> bool {
    stats.percentile_95[0] <= 1. && stats.percentile_95[1] >= 1.
}
/// Calculator-only entry point for separately validated paired transcripts.
/// Existing roster/admission rules above remain unchanged. No execution or
/// correctness standing follows from supplying numerical arrays.
pub fn paired_statistics<const K: usize>(values: &[[f64; K]]) -> Result<[Statistics; K], String> {
    if K == 0
        || ![7, 21].contains(&values.len())
        || values.iter().flatten().any(|x| !x.is_finite() || *x <= 0.)
    {
        return Err("paired calculator requires7/21 complete positive finite rows".into());
    }
    Ok(intervals(values))
}
/// Explicit four-route 24-balanced-plus-one-block adapter. Legacy7/21 admission is unchanged.
pub fn paired_statistics_25<const K: usize>(values: &[[f64; K]]) -> Result<[Statistics; K], String> {
    if K == 0 || values.len()!=25 || values.iter().flatten().any(|x|!x.is_finite() || *x<=0.) {
        return Err("balanced calculator requires25 positive finite rows".into());
    }
    Ok(intervals(values))
}
#[cfg(test)]
#[test]
fn explicit_balanced_adapter_keeps_legacy_roster(){
    let rows=[[2.0];25];assert!(paired_statistics(&rows).is_err());
    let stats=paired_statistics_25(&rows).unwrap();assert_eq!(stats[0].median_paired_ratio,2.0);assert_eq!(stats[0].adjusted_lower_1_over_720,2.0);
    assert!(paired_statistics_25(&[[2.0];21]).is_err());
}
/// Numerical A/A gate for captured calls. Full transcript/provenance validation
/// is separate; this helper grants no execution or mathematical standing.
pub fn aa_control(arm: Arm, pairs: &[[Sample; 2]], setup: u64) -> Result<AaResult, String> {
    if pairs.len() != 7 || setup == 0 || setup > CAMPAIGN_NS {
        return Err("seven pairs and positive bounded setup required".into());
    }
    let ratios: Vec<_> = pairs
        .iter()
        .map(|[a, b]| {
            if a.arm != arm || b.arm != arm {
                return Err("A/A arm mismatch".into());
            }
            Ok([
                a.worker_wall_ns as f64 / b.worker_wall_ns as f64,
                paid(a, setup)? as f64 / paid(b, setup)? as f64,
            ])
        })
        .collect::<Result<_, String>>()?;
    let [native, full_cost] = intervals(&ratios);
    let admitted = includes_one(&native) && includes_one(&full_cost);
    Ok(AaResult {
        arm,
        native,
        full_cost,
        admitted,
    })
}
pub fn analyze(case: &Case) -> Report {
    let mut report = Report {
        schema: "compact-speed-analysis/v1",
        prime: case.prime,
        variables: case.variables,
        seed: case.seed,
        input_sha256: case.input_sha256.clone(),
        worker_sha256: case.worker_sha256.clone(),
        status: "invalid_or_incomplete",
        reasons: Vec::new(),
        aa: None,
        comparisons: None,
        numerical_rule_passes: false,
        execution_admission_verified: false,
        family_confirmed: false,
        standing: "producer calculation only; execution/mathematical provenance and omitted costs are not verified; no family or signed performance verdict",
    };
    if let Err(reason) = validate(case) {
        report.reasons.push(reason);
        return report;
    }
    let aa: Vec<_> = ARMS
        .iter()
        .zip(&case.aa_pairs)
        .map(|(arm, pairs)| aa_control(*arm, pairs, case.setup_wall_ns).unwrap())
        .collect();
    let admitted = aa.iter().all(|a| a.admitted);
    report.aa = Some(aa);
    if !admitted {
        report.status = "aa_rejected";
        report
            .reasons
            .push("A/A interval excludes one; no main comparison".into());
        return report;
    }
    let ratios: Vec<_> = case
        .blocks
        .iter()
        .map(|block| {
            let sample = |arm| block.iter().find(|s| s.arm == arm).unwrap();
            let c = sample(Arm::Compact);
            let f = sample(Arm::F4);
            let e = sample(Arm::Certified);
            [
                f.worker_wall_ns as f64 / c.worker_wall_ns as f64,
                e.worker_wall_ns as f64 / c.worker_wall_ns as f64,
                paid(f, case.setup_wall_ns).unwrap() as f64
                    / paid(c, case.setup_wall_ns).unwrap() as f64,
                paid(e, case.setup_wall_ns).unwrap() as f64
                    / paid(c, case.setup_wall_ns).unwrap() as f64,
            ]
        })
        .collect();
    let [f_native, e_native, f_full, e_full] = intervals(&ratios);
    report.numerical_rule_passes =
        f_full.adjusted_lower_1_over_720 > TARGET && e_full.adjusted_lower_1_over_720 > TARGET;
    report.status = "calculated_exploratory_case";
    report.comparisons = Some(vec![
        Comparison {
            control: Arm::F4,
            native: f_native,
            full_cost: f_full,
        },
        Comparison {
            control: Arm::Certified,
            native: e_native,
            full_cost: e_full,
        },
    ]);
    report
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn synthetic_constant_full_cost_ratios_and_no_execution_standing() {
        let report = analyze(&support::synthetic());
        assert_eq!(report.status, "calculated_exploratory_case");
        assert!(report.numerical_rule_passes);
        assert!(!report.execution_admission_verified && !report.family_confirmed);
        for (comparison, expected) in report.comparisons.unwrap().iter().zip([20., 24.]) {
            assert_eq!(comparison.full_cost.median_paired_ratio, expected);
            assert_eq!(comparison.full_cost.percentile_95, [expected; 2]);
            assert_eq!(comparison.full_cost.adjusted_lower_1_over_720, expected);
        }
    }
    #[test]
    fn setup_cost_cannot_be_hidden_by_reporting_native_ratios() {
        let mut case = support::synthetic();
        case.setup_wall_ns = 1_000_000;
        let report = analyze(&case);
        assert!(!report.numerical_rule_passes);
        for c in report.comparisons.unwrap() {
            assert!(c.native.adjusted_lower_1_over_720 > 15.);
            assert!(c.full_cost.adjusted_lower_1_over_720 < 2.);
        }
    }
    #[test]
    fn strict_threshold_requires_both_controls() {
        let mut case = support::synthetic();
        for block in &mut case.blocks {
            block
                .iter_mut()
                .find(|s| s.arm == Arm::F4)
                .unwrap()
                .worker_wall_ns = 1498;
        }
        let report = analyze(&case);
        assert!(!report.numerical_rule_passes);
        let comparisons = report.comparisons.unwrap();
        assert_eq!(comparisons[0].full_cost.adjusted_lower_1_over_720, 15.);
        assert!(comparisons[1].full_cost.adjusted_lower_1_over_720 > 15.);
    }
    #[test]
    fn failed_aa_prevents_main_numeric_comparison() {
        let mut case = support::synthetic();
        for pair in &mut case.aa_pairs[0] {
            pair[1].worker_wall_ns *= 2;
        }
        let report = analyze(&case);
        assert_eq!(report.status, "aa_rejected");
        assert!(!report.aa.unwrap()[0].admitted);
        assert!(report.comparisons.is_none());
        assert!(!report.numerical_rule_passes);
    }
    #[test]
    fn incomplete_reordered_and_mismatched_cases_supply_no_numeric_win() {
        let original = support::synthetic();
        let mutations: [fn(&mut Case); 12] = [
            |c| {
                c.blocks.pop();
            },
            |c| c.blocks[0][0].complete = false,
            |c| c.blocks[0].swap(0, 1),
            |c| c.blocks[0][0].basis_sha256 = "cd".repeat(32),
            |c| c.warmups[0].external_basis_sha256 = "cd".repeat(32),
            |c| {
                c.aa_pairs[1].pop();
            },
            |c| c.aa_pairs[2][0][1].arm = Arm::F4,
            |c| {
                c.warmups.pop();
            },
            |c| c.blocks[0][0].basis_sha256 = "not a digest".into(),
            |c| c.blocks[0][0].input_sha256 = "ef".repeat(32),
            |c| c.blocks[0][0].worker_sha256 = "ef".repeat(32),
            |c| c.blocks[0][0].validation_input_sha256 = "ef".repeat(32),
        ];
        for mutate in mutations {
            let mut case = original.clone();
            mutate(&mut case);
            let report = analyze(&case);
            assert_eq!(report.status, "invalid_or_incomplete");
            assert!(report.comparisons.is_none() && !report.numerical_rule_passes);
            assert!(!report.reasons.is_empty());
        }
    }
    #[test]
    fn unsupported_roster_and_invalid_accounting_rejected() {
        let mutations: [fn(&mut Case); 9] = [
            |c| c.seed = 19001, // identifier only; no EC fixture is generated
            |c| c.prime = 9007199254740881,
            |c| c.variables = 5,
            |c| c.setup_wall_ns = 0,
            |c| c.blocks[0][0].validation_wall_ns = 0,
            |c| c.blocks[0][0].worker_wall_ns = 0,
            |c| c.blocks[0][0].worker_wall_ns = u64::MAX,
            |c| c.blocks[0][0].worker_wall_ns = CAMPAIGN_NS,
            |c| c.schema = "arbitrary".into(),
        ];
        for mutate in mutations {
            let mut case = support::synthetic();
            mutate(&mut case);
            let report = analyze(&case);
            assert!(report.comparisons.is_none() && !report.numerical_rule_passes);
            assert!(!report.reasons.is_empty());
        }
    }
    #[test]
    fn paired_estimator_is_not_ratio_of_marginal_medians() {
        let mut case = support::synthetic();
        for (i, block) in case.blocks.iter_mut().enumerate() {
            for sample in block {
                sample.worker_wall_ns = match sample.arm {
                    Arm::F4 => [1, 10, 100][i % 3],
                    Arm::Compact => [100, 1, 10][i % 3],
                    Arm::Certified => 100,
                };
            }
        }
        let report = analyze(&case);
        let comparison = &report.comparisons.unwrap()[0];
        // Both marginal medians are 10, while 14/21 paired ratios are 10.
        assert_eq!(comparison.native.median_paired_ratio, 10.);
    }
    #[test]
    fn resampling_is_deterministic_and_joint_across_columns() {
        let values: Vec<_> = (1..=21).map(|i| [i as f64, (10 * i) as f64]).collect();
        let stats = intervals(&values);
        assert_eq!(stats, intervals(&values));
        assert_eq!(
            stats[0].median_paired_ratio * 10.,
            stats[1].median_paired_ratio
        );
        assert_eq!(
            stats[0].percentile_95.map(|x| x * 10.),
            stats[1].percentile_95
        );
        assert_eq!(
            stats[0].adjusted_lower_1_over_720 * 10.,
            stats[1].adjusted_lower_1_over_720
        );
    }
    #[test]
    fn nearest_rank_convention_retains_adjusted_tail() {
        assert_eq!(nearest_rank(10000, 1, 40), 249);
        assert_eq!(nearest_rank(10000, 39, 40), 9749);
        assert_eq!(nearest_rank(100000, 1, 720), 138);
        assert_eq!(nearest_rank(100000, 1, 40), 2499);
        assert_eq!(nearest_rank(100000, 39, 40), 97499);
    }
    #[test]
    fn unknown_fields_are_not_silently_accepted() {
        let mut value = serde_json::to_value(support::synthetic()).unwrap();
        value["verified"] = serde_json::json!(true);
        assert!(serde_json::from_value::<Case>(value).is_err());
    }
    #[test]
    fn public_aa_gate_rejects_invalid_costs_and_arm_pairing() {
        let mut case = support::synthetic();
        assert!(aa_control(Arm::F4, &case.aa_pairs[0], 0).is_err());
        assert!(aa_control(Arm::Certified, &case.aa_pairs[0], 1).is_err());
        case.aa_pairs[0][0][1].worker_wall_ns = 0;
        assert!(aa_control(Arm::F4, &case.aa_pairs[0], 1).is_err());
    }
}
