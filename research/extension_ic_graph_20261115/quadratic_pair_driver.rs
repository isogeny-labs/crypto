//! Native cold relation-collection component; public curve input only.
use crate::quadratic_pair_relation::Certificate;
use crypto_lib::cryptanalysis::{
    jv_cover::{BaseEl, EllE, Fq3, PtE6, E2, E6},
    residual_walk::is_prime_u64,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub const SOURCE_CAP: usize = 262_144;
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PublicPoint {
    pub x: [[u64; 2]; 3],
    pub y: [[u64; 2]; 3],
    pub inf: bool,
}
impl PublicPoint {
    pub fn native(&self) -> PtE6 {
        PtE6 {
            x: E6(self.x.map(E2)),
            y: E6(self.y.map(E2)),
            inf: self.inf,
        }
    }
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PublicCurve {
    pub p: u64,
    pub l: u64,
    pub alpha: [[u64; 2]; 3],
    pub g: PublicPoint,
    pub q: PublicPoint,
}
impl PublicCurve {
    pub fn validate(&self) -> Result<(), String> {
        if self.p != 1009
            || self.l != 263807419864521289
            || !is_prime_u64(self.p)
            || !is_prime_u64(self.l)
            || self.g.inf
            || self.q.inf
            || self
                .alpha
                .iter()
                .flatten()
                .chain(self.g.x.iter().flatten())
                .chain(self.g.y.iter().flatten())
                .chain(self.q.x.iter().flatten())
                .chain(self.q.y.iter().flatten())
                .any(|&x| x >= self.p)
        {
            return Err("invalid registered public domain".into());
        }
        let f = Fq3::new(self.p);
        let ec = EllE::new(&f, &E6(self.alpha.map(E2)));
        let (g, q) = (self.g.native(), self.q.native());
        if !ec.on_curve(&g)
            || !ec.on_curve(&q)
            || !ec.mul(&g, self.l).inf
            || !ec.mul(&q, self.l).inf
        {
            return Err("invalid public subgroup".into());
        }
        Ok(())
    }
}
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Point {
    pub x: u64,
    pub y: [u64; 2],
}
pub fn points(base: &[BaseEl]) -> Vec<Point> {
    base.iter().map(|b| Point { x: b.x, y: b.y.0 }).collect()
}
#[derive(Clone, Serialize, Deserialize)]
pub struct Source {
    pub candidate_index: u64,
    pub b_index: u64,
    pub a0: Vec<u64>,
    pub a1: Vec<u64>,
    pub b: Vec<[u64; 2]>,
    pub s: u64,
    pub certificate: Certificate,
    pub graph: Value,
}
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Projection {
    pub terms: Vec<(usize, u64)>,
    pub witness: Vec<(usize, i8)>,
    pub rank: usize,
    pub products: u64,
}
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Checkpoint {
    pub b_count: u64,
    pub lines: u64,
    pub cohort_sha256: String,
    pub sources: usize,
    pub projections: usize,
    pub rank: usize,
}
#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Counts {
    pub b_count: u64,
    pub lines: u64,
    pub candidates: u64,
    pub refused: u64,
    pub duplicates: u64,
    pub full_rows: u64,
    pub partial_rows: u64,
    pub one_class_rows: u64,
    pub two_class_rows: u64,
    pub vertices: usize,
    pub collisions: u64,
    pub rank: usize,
    pub products: u64,
    pub normalization_products: u64,
    pub subtractions: u64,
}
#[derive(Serialize, Deserialize)]
pub struct Record {
    pub schema: String,
    pub source_revision: String,
    pub input_sha256: String,
    pub mode: String,
    pub seed: u64,
    pub public: PublicCurve,
    pub base: Vec<Point>,
    pub h: Vec<[u64; 2]>,
    pub sources: Vec<Source>,
    pub projections: Vec<Projection>,
    pub checkpoints: Vec<Checkpoint>,
    pub counts: Counts,
    pub rank_target: usize,
    pub stop: String,
    pub inner_wall_ns: u128,
    pub work_native: Value,
    pub graph_stats: Value,
    pub graph_options: Value,
    pub split_options: Value,
    pub splits: Vec<SplitRecord>,
    pub max_split_records: usize,
    pub split_seed_xor: u64,
    pub root_finder: String,
    pub sieve_selection: String,
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SplitRecord {
    pub candidate_index: u64,
    pub b_index: u64,
    pub a0: Vec<u64>,
    pub a1: Vec<u64>,
    pub b: Vec<[u64; 2]>,
    pub s: u64,
    pub tail: Vec<u64>,
    pub report: Value,
}
pub fn signed(sign: i8, p: u64) -> u64 {
    (sign as i128).rem_euclid(p as i128) as u64
}
