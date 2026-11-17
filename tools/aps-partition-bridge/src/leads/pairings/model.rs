use crypto_lib::cryptanalysis::f4_fp::TraceStats;
use serde::Serialize;

#[derive(Serialize)]
pub struct Selection {
    pub cell: usize,
    pub candidate_product_scores: [usize; 3],
    pub selected_pairing: usize,
    pub domain_coordinate_visits: usize,
    pub score_candidates: usize,
    pub score_multiplications: usize,
    pub score_additions: usize,
}

#[derive(Serialize)]
pub struct CellResult {
    pub cell: usize,
    pub pairing: usize,
    pub permutation: [usize; 4],
    pub original_parts: Vec<Vec<u64>>,
    pub complete: bool,
    pub timed_out: bool,
    pub error: Option<String>,
    pub expected: Vec<Vec<u64>>,
    pub observed: Vec<Vec<u64>>,
    pub raw_witnesses: Vec<Vec<u64>>,
}

#[derive(Serialize)]
pub struct Route {
    pub pairing: Option<usize>,
    pub permutation: Option<[usize; 4]>,
    pub selected_input_rule: bool,
    pub shared: bool,
    pub cells: Vec<CellResult>,
    pub observed_roots: Vec<Vec<u64>>,
    pub recovered_logarithm: Option<u64>,
    pub recovery_error: Option<String>,
    pub pool_stats: Option<TraceStats>,
    pub complete_and_equal: bool,
}

#[derive(Serialize)]
pub struct Sample {
    pub seed: u64,
    pub fixture_error: Option<String>,
    pub selection_error: Option<String>,
    pub selections: Vec<Selection>,
    pub routes: Vec<Route>,
    pub route_errors: Vec<String>,
    pub selected_roots_from_cold_controls: Vec<Vec<u64>>,
    pub selected_cold_replay_complete: bool,
    pub selected_route: Option<Route>,
    pub selected_route_error: Option<String>,
}

#[derive(Serialize)]
pub struct Report {
    pub schema: &'static str,
    pub heuristic: &'static str,
    pub selected_route_uses_cold_control_outputs: bool,
    pub actual_selected_route_executed: bool,
    pub performance_trial: bool,
    pub cost_complete: bool,
    pub samples: Vec<Sample>,
    pub planned_routes: usize,
    pub planned_control_routes: usize,
    pub planned_selected_routes: usize,
    pub attempted_routes: usize,
    pub completed_routes: usize,
    pub failed_or_incomplete_routes: usize,
    pub excluded_routes_by_rule: usize,
    pub complete_and_equal: bool,
}
