use super::*;

#[derive(Serialize)]
pub struct Outcome {
    pub solver_called: bool,
    pub complete: bool,
    pub timed_out: bool,
    pub error: Option<String>,
    pub roots: BTreeSet<Vec<u64>>,
    pub raw_witnesses: Vec<Vec<u64>>,
}

#[derive(Serialize)]
pub struct CellReport {
    pub cell_index: usize,
    pub parts: Vec<Vec<u64>>,
    pub valuation_safe: bool,
    pub left_support: Vec<u64>,
    pub right_support: Vec<u64>,
    pub intersection: Vec<u64>,
    pub auxiliary_vanishing: Poly,
    pub construction_pair_additions: usize,
    pub direct_join_pair_matches: usize,
    pub direct_join_roots: BTreeSet<Vec<u64>>,
    pub expected_roots: BTreeSet<Vec<u64>>,
    pub full_field: Outcome,
    pub constrained: Outcome,
    pub complete_and_equal: bool,
}

#[derive(Serialize)]
pub struct Sample {
    pub seed: u64,
    pub cells: Vec<CellReport>,
    pub cell_errors: Vec<CellError>,
    pub planned_cells: usize,
    pub attempted_cells: usize,
    pub completed_cells: usize,
    pub failed_or_incomplete_cells: usize,
    pub expected_union: Option<BTreeSet<Vec<u64>>>,
    pub union_error: Option<String>,
    pub constrained_union: BTreeSet<Vec<u64>>,
    pub complete_and_equal: bool,
}

#[derive(Serialize)]
pub struct Report {
    pub construction_charged_as_diagnostics_only: bool,
    pub performance_trial: bool,
    pub planned_seeds: usize,
    pub attempted_seeds: usize,
    pub seed_errors: Vec<(u64, String)>,
    pub samples: Vec<Sample>,
    pub complete_and_equal: bool,
}

#[derive(Serialize)]
pub struct CellError {
    pub cell_index: usize,
    pub parts: Vec<Vec<u64>>,
    pub error: String,
}
