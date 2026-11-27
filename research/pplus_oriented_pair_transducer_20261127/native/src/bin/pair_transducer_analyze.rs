use std::collections::BTreeMap;
use std::env;
use std::fmt::Write as _;
use std::fs;

const DEPTHS: usize = 6;
const EXPECTED_QUERIES: usize = 60;
const FE_LIMIT_TENTHS: u64 = 16_000_000;
const PROBE_LIMIT: u64 = 1_600_000;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct DepthMetrics {
    depth: usize,
    signed_leaves: u64,
    leaf_rank_gf2: u64,
    semantic_entries: u64,
    root_support: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Projection {
    residual_queries: u64,
    root_support: u64,
    miss_probes: u64,
    optimistic_probes: u64,
    panel_fe_tenths: [u64; 3],
}

#[derive(Clone, Copy, Debug)]
struct GrowthAssessment {
    ratio_4_to_5: f64,
    quadratic_4_to_5: f64,
    pass_4_to_5: bool,
    ratio_5_to_6: f64,
    quadratic_5_to_6: f64,
    pass_5_to_6: bool,
    slope_3_to_6: Option<f64>,
    pass_slope: bool,
}

impl GrowthAssessment {
    fn passes(self) -> bool {
        self.pass_4_to_5 && self.pass_5_to_6 && self.pass_slope
    }
}

#[derive(Debug)]
struct Candidate {
    depths: Vec<DepthMetrics>,
    projection: Projection,
    query_count: usize,
    all_replay_ok: bool,
    depth_six_finite_misses: usize,
}

fn fields(line: &str) -> BTreeMap<&str, &str> {
    line.split_whitespace()
        .skip(1)
        .map(|token| token.split_once('=').expect("malformed key=value token"))
        .collect()
}

fn number<T: std::str::FromStr>(map: &BTreeMap<&str, &str>, key: &str) -> T
where
    T::Err: std::fmt::Debug,
{
    map.get(key)
        .unwrap_or_else(|| panic!("missing field {key}"))
        .parse()
        .unwrap_or_else(|error| panic!("invalid field {key}: {error:?}"))
}

fn parse_candidate(text: &str) -> Candidate {
    let mut depths = Vec::new();
    let mut projection = None;
    let mut query_count = 0;
    let mut all_replay_ok = true;
    let mut depth_six_finite_misses = 0;

    for line in text.lines() {
        if line.starts_with("DEPTH ") {
            let map = fields(line);
            depths.push(DepthMetrics {
                depth: number(&map, "depth"),
                signed_leaves: number(&map, "signed_leaves"),
                leaf_rank_gf2: number(&map, "leaf_rank_gf2"),
                semantic_entries: number(&map, "semantic_entries"),
                root_support: number(&map, "root_support"),
            });
        } else if line.starts_with("QUERY ") {
            let map = fields(line);
            query_count += 1;
            all_replay_ok &= number::<u8>(&map, "replay_ok") == 1;
            if number::<usize>(&map, "depth") == 6
                && number::<u8>(&map, "target_inf") == 0
                && number::<u8>(&map, "found") == 0
            {
                depth_six_finite_misses += 1;
            }
        } else if line.starts_with("PROJECTION ") {
            let map = fields(line);
            projection = Some(Projection {
                residual_queries: number(&map, "residual_queries"),
                root_support: number(&map, "root_support"),
                miss_probes: number(&map, "miss_probes"),
                optimistic_probes: number(&map, "optimistic_probes"),
                panel_fe_tenths: [
                    number(&map, "panel1_fe_tenths"),
                    number(&map, "panel2_fe_tenths"),
                    number(&map, "panel3_fe_tenths"),
                ],
            });
        }
    }

    Candidate {
        depths,
        projection: projection.expect("missing PROJECTION row"),
        query_count,
        all_replay_ok,
        depth_six_finite_misses,
    }
}

fn verifier_ok(text: &str) -> bool {
    let depth_rows = text
        .lines()
        .filter(|line| line.starts_with("VERIFY ") && line.ends_with("status=ok"))
        .count();
    depth_rows == DEPTHS
        && text.lines().any(|line| {
            line == format!("SUMMARY depths={DEPTHS} queries={EXPECTED_QUERIES} status=ok")
        })
}

fn strict_subquadratic_transition(
    previous_metric: u64,
    next_metric: u64,
    previous_leaves: u64,
    next_leaves: u64,
) -> bool {
    previous_metric != 0
        && next_metric != 0
        && u128::from(next_metric) * u128::from(previous_leaves).pow(2)
            < u128::from(previous_metric) * u128::from(next_leaves).pow(2)
}

fn log_log_slope(leaves: &[u64], metric: &[u64]) -> Option<f64> {
    if leaves.len() != metric.len() || leaves.is_empty() {
        return None;
    }
    if leaves.iter().chain(metric).any(|&value| value == 0) {
        return None;
    }
    let xs: Vec<_> = leaves.iter().map(|&value| (value as f64).ln()).collect();
    let ys: Vec<_> = metric.iter().map(|&value| (value as f64).ln()).collect();
    let count = xs.len() as f64;
    let mean_x = xs.iter().sum::<f64>() / count;
    let mean_y = ys.iter().sum::<f64>() / count;
    let covariance = xs
        .iter()
        .zip(&ys)
        .map(|(x, y)| (x - mean_x) * (y - mean_y))
        .sum::<f64>();
    let variance = xs.iter().map(|x| (x - mean_x).powi(2)).sum::<f64>();
    (variance > 0.0).then_some(covariance / variance)
}

fn assess_growth(depths: &[DepthMetrics], metric: fn(&DepthMetrics) -> u64) -> GrowthAssessment {
    assert_eq!(depths.len(), DEPTHS, "expected six depth rows");
    for (index, row) in depths.iter().enumerate() {
        assert_eq!(row.depth, index + 1, "depth rows are not canonical");
    }
    let d4 = depths[3];
    let d5 = depths[4];
    let d6 = depths[5];
    let ratio_4_to_5 = metric(&d5) as f64 / metric(&d4) as f64;
    let ratio_5_to_6 = metric(&d6) as f64 / metric(&d5) as f64;
    let quadratic_4_to_5 = (d5.signed_leaves as f64 / d4.signed_leaves as f64).powi(2);
    let quadratic_5_to_6 = (d6.signed_leaves as f64 / d5.signed_leaves as f64).powi(2);
    let leaves: Vec<_> = depths[2..].iter().map(|row| row.signed_leaves).collect();
    let values: Vec<_> = depths[2..].iter().map(metric).collect();
    let slope_3_to_6 = log_log_slope(&leaves, &values);
    GrowthAssessment {
        ratio_4_to_5,
        quadratic_4_to_5,
        pass_4_to_5: strict_subquadratic_transition(
            metric(&d4),
            metric(&d5),
            d4.signed_leaves,
            d5.signed_leaves,
        ),
        ratio_5_to_6,
        quadratic_5_to_6,
        pass_5_to_6: strict_subquadratic_transition(
            metric(&d5),
            metric(&d6),
            d5.signed_leaves,
            d6.signed_leaves,
        ),
        pass_slope: slope_3_to_6.is_some_and(|slope| slope < 2.0),
        slope_3_to_6,
    }
}

fn write_growth(output: &mut String, name: &str, growth: GrowthAssessment, comma: bool) {
    let slope = growth
        .slope_3_to_6
        .map_or_else(|| "null".to_owned(), |value| format!("{value:.9}"));
    writeln!(
        output,
        "    \"{name}\": {{\"ratio_4_to_5\": {:.9}, \"quadratic_4_to_5\": {:.9}, \"pass_4_to_5\": {}, \"ratio_5_to_6\": {:.9}, \"quadratic_5_to_6\": {:.9}, \"pass_5_to_6\": {}, \"slope_3_to_6\": {}, \"pass_slope\": {}, \"pass\": {}}}{}",
        growth.ratio_4_to_5,
        growth.quadratic_4_to_5,
        growth.pass_4_to_5,
        growth.ratio_5_to_6,
        growth.quadratic_5_to_6,
        growth.pass_5_to_6,
        slope,
        growth.pass_slope,
        growth.passes(),
        if comma { "," } else { "" }
    )
    .unwrap();
}

fn render(candidate: &Candidate, verifier_pass: bool) -> String {
    let rank = assess_growth(&candidate.depths, |row| row.leaf_rank_gf2);
    let semantic = assess_growth(&candidate.depths, |row| row.semantic_entries);
    let support = assess_growth(&candidate.depths, |row| row.root_support);
    let correctness_pass = verifier_pass
        && candidate.query_count == EXPECTED_QUERIES
        && candidate.all_replay_ok
        && candidate.depth_six_finite_misses > 0;
    let growth_pass = rank.passes() && semantic.passes() && support.passes();
    let panels_pass = candidate
        .projection
        .panel_fe_tenths
        .iter()
        .all(|&value| value < FE_LIMIT_TENTHS);
    let probe_floor_pass = candidate.projection.optimistic_probes < PROBE_LIMIT;
    let projection_pass = panels_pass && probe_floor_pass;
    let eligible = correctness_pass && growth_pass && projection_pass;
    let verdict = if eligible {
        "promote-to-m7-integration"
    } else {
        "stop-before-m7-integration"
    };

    let mut output = String::new();
    writeln!(
        output,
        "{{\n  \"schema\": \"isogeny-labs/pplus-oriented-pair-transducer-analysis/v1\","
    )
    .unwrap();
    writeln!(
        output,
        "  \"correctness\": {{\"verifier_pass\": {verifier_pass}, \"candidate_queries\": {}, \"all_replay_ok\": {}, \"depth_six_finite_misses\": {}, \"pass\": {correctness_pass}}},",
        candidate.query_count, candidate.all_replay_ok, candidate.depth_six_finite_misses
    )
    .unwrap();
    writeln!(output, "  \"depths\": [").unwrap();
    for (index, row) in candidate.depths.iter().enumerate() {
        writeln!(
            output,
            "    {{\"depth\": {}, \"signed_leaves\": {}, \"leaf_rank_gf2\": {}, \"semantic_entries\": {}, \"root_support\": {}}}{}",
            row.depth,
            row.signed_leaves,
            row.leaf_rank_gf2,
            row.semantic_entries,
            row.root_support,
            if index + 1 == candidate.depths.len() { "" } else { "," }
        )
        .unwrap();
    }
    writeln!(output, "  ],\n  \"growth\": {{").unwrap();
    write_growth(&mut output, "leaf_rank_gf2", rank, true);
    write_growth(&mut output, "semantic_entries", semantic, true);
    write_growth(&mut output, "root_support", support, false);
    writeln!(output, "  }},").unwrap();
    writeln!(
        output,
        "  \"projection\": {{\"residual_queries\": {}, \"root_support\": {}, \"miss_probes\": {}, \"optimistic_probes\": {}, \"panel_fe_tenths\": [{}, {}, {}], \"field_equivalent_limit_tenths\": {FE_LIMIT_TENTHS}, \"probe_limit\": {PROBE_LIMIT}, \"panels_pass\": {panels_pass}, \"probe_floor_pass\": {probe_floor_pass}, \"pass\": {projection_pass}}},",
        candidate.projection.residual_queries,
        candidate.projection.root_support,
        candidate.projection.miss_probes,
        candidate.projection.optimistic_probes,
        candidate.projection.panel_fe_tenths[0],
        candidate.projection.panel_fe_tenths[1],
        candidate.projection.panel_fe_tenths[2]
    )
    .unwrap();
    writeln!(
        output,
        "  \"promotion\": {{\"correctness_pass\": {correctness_pass}, \"growth_pass\": {growth_pass}, \"projection_pass\": {projection_pass}, \"eligible\": {eligible}, \"verdict\": \"{verdict}\"}}\n}}"
    )
    .unwrap();
    output
}

fn main() {
    let mut args = env::args().skip(1);
    let candidate_path = args
        .next()
        .expect("usage: pair-transducer-analyze CANDIDATE VERIFIER");
    let verifier_path = args
        .next()
        .expect("usage: pair-transducer-analyze CANDIDATE VERIFIER");
    assert!(args.next().is_none(), "unexpected extra argument");
    let candidate_text = fs::read_to_string(candidate_path).expect("cannot read candidate output");
    let verifier_text = fs::read_to_string(verifier_path).expect("cannot read verifier output");
    let candidate = parse_candidate(&candidate_text);
    assert_eq!(candidate.depths.len(), DEPTHS, "expected six depth rows");
    print!("{}", render(&candidate, verifier_ok(&verifier_text)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transition_comparison_is_exact() {
        assert!(strict_subquadratic_transition(100, 399, 10, 20));
        assert!(!strict_subquadratic_transition(100, 400, 10, 20));
        assert!(!strict_subquadratic_transition(0, 1, 10, 20));
    }

    #[test]
    fn slope_distinguishes_linear_and_quadratic() {
        let leaves = [4, 8, 16, 32];
        assert!((log_log_slope(&leaves, &leaves).unwrap() - 1.0).abs() < 1e-12);
        let squares = [16, 64, 256, 1024];
        assert!((log_log_slope(&leaves, &squares).unwrap() - 2.0).abs() < 1e-12);
        assert_eq!(log_log_slope(&leaves, &[1, 0, 3, 4]), None);
    }

    #[test]
    fn verifier_summary_is_strict() {
        let text = concat!(
            "VERIFY depth=1 status=ok\n",
            "VERIFY depth=2 status=ok\n",
            "VERIFY depth=3 status=ok\n",
            "VERIFY depth=4 status=ok\n",
            "VERIFY depth=5 status=ok\n",
            "VERIFY depth=6 status=ok\n",
            "SUMMARY depths=6 queries=60 status=ok\n"
        );
        assert!(verifier_ok(text));
        assert!(!verifier_ok("SUMMARY depths=6 queries=60 status=ok\n"));
    }
}
