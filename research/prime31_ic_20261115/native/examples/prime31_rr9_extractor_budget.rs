//! Deterministic, zero-solver-work feasibility gate for the public-only B8 extractor.
 //! Replays *prior* source-linked measurements. Does NOT measure a new solver.
 //! Uses the pinned original Prime31 curve certificate and Rust arithmetic.
use prime31_algebraic_ic::{check_curve, Arithmetic, PRIME31};
use serde::{Deserialize, Serialize};

const INPUT: &str = include_str!("../../rr9/RR9_EXTRACTOR_COST_GATE_INPUT_20261119.json");

#[derive(Debug, Deserialize)]
struct Input {
    curve: CurveInput,
    comparison: Comparison,
    queries: Queries,
    chart: Chart,
    historical_frontier: HistoricalFrontier,
}
#[derive(Debug, Deserialize)]
struct CurveInput {
    p: u64, a: u64, b: u64, n: u64, cofactor: u64, g: [u64; 2],
}
#[derive(Debug, Deserialize)]
struct Comparison {
    mean_signed_shared_bsgs_1024_ms: f64,
    mean_ternary_w8_1024_ms: f64,
    completed_logs: u64,
}
#[derive(Debug, Deserialize)]
struct Queries {
    training: u64,
    online: u64,
}
#[derive(Debug, Deserialize)]
struct Chart {
    b4_natural_negative_full_scan_mean_ms: f64,
    b4_negative_pairs: u64,
    b8_negative_pairs: u64,
    fp_products_per_pair: u64,
}
// The 2^20-entry shared BSGS cohort was measured in a DIFFERENT CI execution.
// We use this for a sensitivity calculation, never an observed paired crossover.
#[derive(Debug, Deserialize)]
struct HistoricalFrontier {
    curve_same_as_prime31: bool,
    same_public_target_cohort_as_4774: bool,
    same_host_run_as_4774: bool,
    n_public_logs: u64,
    baby_table_size: u64,
    setup_ms: f64,
    online_ms: f64,
    fully_charged_batch_ms: f64,
    memory_rss_kib: u64,
}
#[derive(Debug, Serialize)]
struct Gate {
    schema: &'static str,
    classification: &'static str,
    curve_checked_by_original_rust_arithmetic: bool,
    public_log_count: u64,
    training_queries: u64,
    online_queries: u64,
    query_total: u64,
    shared_bsgs_mean_ms: f64,
    ternary_generic_table_ic_mean_ms: f64,
    remaining_gap_to_bsgs_ms: f64,
    optimistic_extractor_upper_bound_ms_per_call: f64,
    nominal_20_percent_safety_budget_ms_per_call: f64,
    b4_negative_full_scan_ms_different_ci_run: f64,
    b4_to_per_call_budget_ratio_cross_run_diagnostic: f64,
    b8_exhaustive_pair_count: u64,
    b8_exhaustive_field_product_count: u128,
    b8_vs_b4_pair_count_ratio: u64,
    field_products_per_second_to_fit_one_b8_scan_in_entire_batch: f64,
    field_products_per_second_if_every_query_scans_b8_under_per_call_budget: f64,
    // HISTORICAL context only. Not a same-runner, same-Q comparison to #4774.
    historical_frontier_is_cross_run_and_cross_cohort: bool,
    historical_million_baby_bsgs_ms: f64,
    historical_million_baby_rss_kib: u64,
    historical_million_baby_setup_ms: f64,
    historical_million_baby_online_ms: f64,
    historical_optimistic_budget_ms_per_call_cross_run_sensitivity_only: f64,
    historical_optimistic_budget_ms_per_completed_log_cross_run_sensitivity_only: f64,
    historical_affine_to_million_baby_wall_ratio_cross_run_only: f64,
    historical_2pow20_baseline_remeasured_same_runner: bool,
    empirical_b8_scan_measured: bool,
    nonenumerative_solver_measured: bool,
    independent_signed_pcc_verified: bool,
}
fn calculate(inp: &Input) -> Gate {
    assert_eq!(inp.curve.p, PRIME31.p, "unchanged original field");
    assert_eq!(inp.curve.a, PRIME31.a);
    assert_eq!(inp.curve.b, PRIME31.b);
    assert_eq!(inp.curve.n, PRIME31.n);
    assert_eq!(inp.curve.g, [PRIME31.g.0, PRIME31.g.1]);
    assert_eq!(inp.curve.cofactor, 1);
    assert_eq!(inp.comparison.completed_logs, 1024);
    assert_eq!(inp.queries.training, 42);
    assert_eq!(inp.queries.online, 1177);
    assert_eq!(inp.chart.b4_negative_pairs, 1u64 << 24);
    assert_eq!(inp.chart.b8_negative_pairs, 1u64 << 32);
    assert_eq!(inp.chart.fp_products_per_pair, 30);
    assert!(inp.historical_frontier.curve_same_as_prime31);
    assert!(!inp.historical_frontier.same_public_target_cohort_as_4774);
    assert!(!inp.historical_frontier.same_host_run_as_4774);
    assert_eq!(inp.historical_frontier.n_public_logs, 1024);
    assert_eq!(inp.historical_frontier.baby_table_size, 1 << 20);
    assert_eq!(inp.historical_frontier.memory_rss_kib, 67924);
    assert!((inp.historical_frontier.setup_ms + inp.historical_frontier.online_ms
        - inp.historical_frontier.fully_charged_batch_ms).abs() < 0.002);
    let mut ar = Arithmetic::default();
    check_curve(PRIME31, &mut ar).expect("prime-order original Rust curve certificate");
    let calls = inp.queries.training + inp.queries.online;
    let limit_ms = inp.comparison.mean_signed_shared_bsgs_1024_ms / calls as f64;
    let b8_products = inp.chart.b8_negative_pairs as u128
        * inp.chart.fp_products_per_pair as u128;
    let full_batch_sec = inp.comparison.mean_signed_shared_bsgs_1024_ms / 1000.0;
    Gate {
        schema: "prime31.rr9-extractor-cost-gate.v1",
        classification: "conditional necessary budget from prior data; not new benchmark",
        curve_checked_by_original_rust_arithmetic: true,
        public_log_count: inp.comparison.completed_logs,
        training_queries: inp.queries.training,
        online_queries: inp.queries.online,
        query_total: calls,
        shared_bsgs_mean_ms: inp.comparison.mean_signed_shared_bsgs_1024_ms,
        ternary_generic_table_ic_mean_ms: inp.comparison.mean_ternary_w8_1024_ms,
        remaining_gap_to_bsgs_ms: inp.comparison.mean_ternary_w8_1024_ms
            - inp.comparison.mean_signed_shared_bsgs_1024_ms,
        optimistic_extractor_upper_bound_ms_per_call: limit_ms,
        nominal_20_percent_safety_budget_ms_per_call: limit_ms * 0.8,
        b4_negative_full_scan_ms_different_ci_run: inp.chart.b4_natural_negative_full_scan_mean_ms,
        b4_to_per_call_budget_ratio_cross_run_diagnostic: inp.chart.b4_natural_negative_full_scan_mean_ms / limit_ms,
        b8_exhaustive_pair_count: inp.chart.b8_negative_pairs,
        b8_exhaustive_field_product_count: b8_products,
        b8_vs_b4_pair_count_ratio: inp.chart.b8_negative_pairs / inp.chart.b4_negative_pairs,
        field_products_per_second_to_fit_one_b8_scan_in_entire_batch: b8_products as f64 / full_batch_sec,
        field_products_per_second_if_every_query_scans_b8_under_per_call_budget:
            b8_products as f64 / (limit_ms / 1000.0),
        historical_frontier_is_cross_run_and_cross_cohort: true,
        historical_million_baby_bsgs_ms: inp.historical_frontier.fully_charged_batch_ms,
        historical_million_baby_rss_kib: inp.historical_frontier.memory_rss_kib,
        historical_million_baby_setup_ms: inp.historical_frontier.setup_ms,
        historical_million_baby_online_ms: inp.historical_frontier.online_ms,
        historical_optimistic_budget_ms_per_call_cross_run_sensitivity_only:
            inp.historical_frontier.fully_charged_batch_ms / calls as f64,
        historical_optimistic_budget_ms_per_completed_log_cross_run_sensitivity_only:
            inp.historical_frontier.fully_charged_batch_ms
                / inp.historical_frontier.n_public_logs as f64,
        historical_affine_to_million_baby_wall_ratio_cross_run_only:
            inp.comparison.mean_signed_shared_bsgs_1024_ms
                / inp.historical_frontier.fully_charged_batch_ms,
        historical_2pow20_baseline_remeasured_same_runner: false,
        empirical_b8_scan_measured: false,
        nonenumerative_solver_measured: false,
        independent_signed_pcc_verified: false,
    }
}
fn main() {
    let inp: Input = serde_json::from_str(INPUT).expect("frozen input JSON");
    let gate = calculate(&inp);
    println!("{}", serde_json::to_string_pretty(&gate).unwrap());
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn exact_pair_products_and_curve() {
        let input: Input = serde_json::from_str(INPUT).unwrap();
        let x = calculate(&input);
        assert_eq!(x.query_total, 1219);
        assert_eq!(x.b8_exhaustive_field_product_count, 128_849_018_880);
        assert_eq!(x.b8_vs_b4_pair_count_ratio, 256);
        assert!(x.curve_checked_by_original_rust_arithmetic);
    }
    #[test]
    fn budget_is_optimistic_and_far_below_b4_scan() {
        let input: Input = serde_json::from_str(INPUT).unwrap();
        let x = calculate(&input);
        assert!((x.optimistic_extractor_upper_bound_ms_per_call - 2.74224236464315).abs() < 1e-10);
        assert!((x.remaining_gap_to_bsgs_ms - 4492.14005).abs() < 1e-4);
        assert!(x.b4_to_per_call_budget_ratio_cross_run_diagnostic > 500.0);
        assert!(!x.nonenumerative_solver_measured);
    }
    #[test]
    fn historical_million_baby_frontier_is_only_a_sensitivity_bound() {
        let input: Input = serde_json::from_str(INPUT).unwrap();
        let x = calculate(&input);
        assert!(x.historical_frontier_is_cross_run_and_cross_cohort);
        assert!(!x.historical_2pow20_baseline_remeasured_same_runner);
        assert!((x.historical_million_baby_bsgs_ms - 170.113).abs() < 1e-6);
        assert!((x.historical_optimistic_budget_ms_per_call_cross_run_sensitivity_only
            - 0.13955127153404429).abs() < 1e-10);
        assert!((x.historical_optimistic_budget_ms_per_completed_log_cross_run_sensitivity_only
            - 0.1661259765625).abs() < 1e-10);
        assert!((x.historical_affine_to_million_baby_wall_ratio_cross_run_only
            - 19.65042908243344).abs() < 1e-7);
    }
}
