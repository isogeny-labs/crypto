use aps_partition_lab::signature::Signature;
use crypto_lib::cryptanalysis::f4_fp::TraceStats;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Mode {
    Cold,
    Shared,
    SignatureBuckets,
}

#[derive(Serialize)]
pub struct CellResult {
    pub coloring: usize,
    pub fixed_coordinate: u64,
    pub parts: Vec<Vec<u64>>,
    pub feature: Signature,
    pub complete: bool,
    pub timed_out: bool,
    pub expected: Vec<Vec<u64>>,
    pub observed: Vec<Vec<u64>>,
    pub error: Option<String>,
}

#[derive(Serialize)]
pub struct PoolResult {
    pub feature: Option<Signature>,
    pub stats: TraceStats,
}

#[derive(Serialize)]
pub struct ModeReport {
    pub mode: Mode,
    pub empty_colorings: Vec<usize>,
    pub cells: Vec<CellResult>,
    pub root_incidents: usize,
    pub represented_roots: Vec<Vec<u64>>,
    pub unrepresented_roots: Vec<Vec<u64>>,
    pub recovered_logarithm: Option<u64>,
    pub recovery_error: Option<String>,
    pub expected_informative_incidents: usize,
    pub expected_zero_b_incidents: usize,
    pub pools: Vec<PoolResult>,
    pub complete_and_equal: bool,
}
