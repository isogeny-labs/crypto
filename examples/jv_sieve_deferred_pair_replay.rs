//! Finite controls and graph-only mathematical replay of validated Runpod49 data.
//! No relation finder, scientific worker, wall clock or DLP trial is launched.
pub use crypto_lib::cryptanalysis;
#[allow(dead_code)]
#[path = "../src/cryptanalysis/jv_deferred_pair_graph.rs"]
mod deferred_pair_graph;
#[allow(dead_code)]
#[path = "../src/cryptanalysis/jv_quadratic_pair_graph.rs"]
mod quadratic_pair_graph;
use deferred_pair_graph::{Event, Graph, Options};
use quadratic_pair_graph::{Class, LargeTerm, Witness};
use serde::Deserialize;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, io::Write, path::Path};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
const L: u64 = 263807419614139553;
#[derive(Deserialize)]
struct Large {
    key: Class,
    coefficient: i8,
}
#[derive(Deserialize)]
struct Certificate {
    terms: Vec<(usize, i8)>,
    remainders: Vec<Large>,
}
#[derive(Deserialize)]
struct Source {
    b_index: u64,
    certificate: Certificate,
    graph: Value,
}
#[derive(Deserialize)]
struct Checkpoint {
    b_count: u64,
    projections: usize,
    rank: usize,
}
#[derive(Deserialize)]
struct KernelResult {
    kernel: Option<Vec<u64>>,
}
#[derive(Deserialize)]
struct RankCheck {
    result: KernelResult,
}
#[derive(Deserialize)]
struct Collection {
    base: Vec<Value>,
    sources: Vec<Source>,
    checkpoints: Vec<Checkpoint>,
    rank_checks: Vec<RankCheck>,
}
#[derive(Deserialize)]
struct Work {
    collection: Collection,
}
#[derive(Deserialize)]
struct Raw {
    source_revision: String,
    method: String,
    seed: u64,
    actual_scalar: u64,
    verified: bool,
    capped: bool,
    work: Work,
}
fn mul(a: u64, b: u64) -> u64 {
    ((a as u128 * b as u128) % L as u128) as u64
}
fn sub(a: u64, b: u64) -> u64 {
    ((a as u128 + L as u128 - b as u128) % L as u128) as u64
}
fn pow(mut a: u64, mut e: u64) -> u64 {
    let mut r = 1;
    while e != 0 {
        if e & 1 == 1 {
            r = mul(r, a);
        }
        a = mul(a, a);
        e >>= 1;
    }
    r
}
#[derive(Default)]
struct Rank(BTreeMap<usize, Vec<u64>>);
impl Rank {
    fn insert(&mut self, mut row: Vec<u64>) -> bool {
        for (&pivot, basis) in &self.0 {
            let scale = row[pivot];
            if scale == 0 {
                continue;
            }
            for (a, &b) in row.iter_mut().zip(basis).skip(pivot) {
                *a = sub(*a, mul(scale, b));
            }
        }
        if let Some(pivot) = row.iter().position(|&v| v != 0) {
            let inv = pow(row[pivot], L - 2);
            for a in row.iter_mut().skip(pivot) {
                *a = mul(*a, inv);
            }
            self.0.insert(pivot, row);
            true
        } else {
            false
        }
    }
}
// Direct integer incidence multiplication, independent of the queue and paths.
fn expand(
    sources: &[Source],
    w: &Witness,
    causal: usize,
    columns: usize,
) -> Result<(Vec<u64>, BTreeMap<Class, i64>)> {
    if w.is_empty()
        || w.windows(2).any(|p| p[0].0 >= p[1].0)
        || w.iter()
            .any(|&(id, c)| id > causal || ![-2, -1, 1, 2].contains(&c))
    {
        return Err("noncanonical/future/coefficient witness".into());
    }
    let mut small = vec![0i64; columns];
    let mut large = BTreeMap::new();
    for &(id, c) in w {
        let s = &sources.get(id).ok_or("source index")?.certificate;
        for &(i, v) in &s.terms {
            *small.get_mut(i).ok_or("small column")? += c as i64 * v as i64;
        }
        for t in &s.remainders {
            *large.entry(t.key).or_default() += c as i64 * t.coefficient as i64;
        }
    }
    large.retain(|_, v| *v != 0);
    Ok((
        small
            .into_iter()
            .map(|v| (v as i128).rem_euclid(L as i128) as u64)
            .collect(),
        large,
    ))
}
fn main() -> Result<()> {
    let a: Vec<_> = std::env::args().skip(1).collect();
    if a.len() != 2 {
        return Err("usage: GRAPH_REPLAY ORIGINAL_SCREENED49_RAW NEW_OUTPUT".into());
    }
    let bytes = fs::read(&a[0])?;
    let digest = format!("{:x}", Sha256::digest(&bytes));
    if digest != "e767e8a175a475645b3a899a32c2f0643ce20a750693aaa8797697934eb4bc56" {
        return Err("validated49original hash".into());
    }
    let w: Raw = serde_json::from_slice(&bytes)?;
    drop(bytes);
    if w.source_revision != "bd69a76db876fa1da120de0edf1033489327d937"
        || w.method != "discriminant-pairs"
        || w.seed != 389
        || !w.verified
        || w.capped
        || w.actual_scalar != 242624768583643540
    {
        return Err("original identity/endpoint".into());
    }
    let c = w.work.collection;
    let columns = c.base.len();
    let kernel = c
        .rank_checks
        .first()
        .and_then(|r| r.result.kernel.as_ref())
        .ok_or("original kernel")?;
    if columns != 509
        || kernel.len() != columns
        || kernel.last() != Some(&1)
        || c.sources.len() != 108058
    {
        return Err("original column/source/kernel shape".into());
    }
    let mut graph = Graph::new(Options::default())?;
    let mut baseline = quadratic_pair_graph::Graph::new(quadratic_pair_graph::Options::default())?;
    let mut rank = Rank::default();
    let mut source = 0usize;
    let mut full = 0usize;
    let mut oldfull = 0usize;
    let mut anchors = 0usize;
    let mut fullhash = Sha256::new();
    let mut trajectory = vec![];
    let mut first_rows: Option<Value> = None;
    let mut first_rank: Option<Value> = None;
    for (ci, cp) in c.checkpoints.iter().enumerate() {
        while source < c.sources.len() && c.sources[source].b_index < cp.b_count {
            let s = &c.sources[source];
            let large: Vec<_> = s
                .certificate
                .remainders
                .iter()
                .map(|r| LargeTerm {
                    class: r.key,
                    coefficient: r.coefficient,
                })
                .collect();
            let old = baseline.accept(source, &large)?;
            if serde_json::to_value(&old)? != s.graph {
                return Err("old graph replay against original".into());
            }
            if matches!(old, quadratic_pair_graph::Outcome::Full { .. }) {
                oldfull += 1;
            }
            for event in graph.accept(source, &large)? {
                match event {
                    Event::Anchor { class, witness, .. } => {
                        let (_, incidence) = expand(&c.sources, &witness, source, columns)?;
                        if incidence != BTreeMap::from([(class, 1)]) {
                            return Err("anchor integer incidence".into());
                        }
                        anchors += 1;
                    }
                    Event::Full { witness, .. } => {
                        let (row, incidence) = expand(&c.sources, &witness, source, columns)?;
                        if !incidence.is_empty() {
                            return Err("full integer incidence".into());
                        }
                        if row.iter().zip(kernel).fold(0u64, |s, (&a, &b)| {
                            ((s as u128 + mul(a, b) as u128) % L as u128) as u64
                        }) != 0
                        {
                            return Err("independent original kernel residual".into());
                        }
                        fullhash.update(serde_json::to_vec(&(source, &witness, &row))?);
                        full += 1;
                        if rank.0.len() < columns - 1 {
                            rank.insert(row);
                        }
                        if full >= 516 && first_rows.is_none() {
                            first_rows = Some(
                                json!({"causal_source":source,"b_index":s.b_index,"complete_batch_b_count":cp.b_count,"rows":full,"actual_rank":rank.0.len()}),
                            );
                        }
                        if rank.0.len() == columns - 1 && first_rank.is_none() {
                            first_rank = Some(
                                json!({"causal_source":source,"b_index":s.b_index,"complete_batch_b_count":cp.b_count,"rows":full,"actual_rank":rank.0.len()}),
                            );
                        }
                    }
                    _ => {}
                }
            }
            source += 1;
        }
        if oldfull != cp.projections {
            return Err("old complete checkpoint row count".into());
        }
        let point = json!({"b_count":cp.b_count,"sources":source,"baseline_rows":cp.projections,"baseline_certified_rank":cp.rank,
            "retained_rows":full,"retained_actual_rank":rank.0.len(),"vertices":graph.stats().vertices,
            "anchors":graph.stats().anchors,"live_deferred":graph.stats().live_deferred});
        if ci % 16 == 0 || ci + 1 == c.checkpoints.len() {
            trajectory.push(point);
        }
    }
    if source != c.sources.len() || anchors != graph.stats().anchors || oldfull != 522 {
        return Err("all original sources consumed".into());
    }
    let output = json!({"schema":"cover.deferred-pair.graph-replay/v1","recorded_input_sha256":digest,
        "recorded_source_revision":w.source_revision,"original_complete_mathematical_replay":"PCC-20261117-3319ad (UNVERIFIED)",
        "graph_options":Options::default(),"graph_stats":graph.stats(),"baseline_graph_stats":baseline.stats(),
        "original_sources_consumed":source,"independent_anchor_incidences_checked":anchors,
        "independent_full_incidences_checked":full,"projected_rows_sha256":format!("{:x}",fullhash.finalize()),
        "baseline_full_rows":oldfull,"final_actual_rank":rank.0.len(),"target_rank":columns-1,
        "first_516_rows":first_rows,"first_actual_target_rank":first_rank,"trajectory":trajectory,
        "no_scientific_worker_invoked":true,"recorded_source_projection_only":true,
        "new_natural_yield":null,"elapsed_performance":null,"complete_cost":false,"ratios":null,"rho_parity":false});
    let mut f = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(Path::new(&a[1]))?;
    serde_json::to_writer_pretty(&mut f, &output)?;
    f.write_all(b"\n")?;
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn independent_rank_rejects_dependence_and_checks_cancellation() {
        let mut r = Rank::default();
        assert!(r.insert(vec![1, 2, 3]));
        assert!(!r.insert(vec![2, 4, 6]));
        assert!(r.insert(vec![0, 1, 1]));
        let row = Source {
            b_index: 0,
            certificate: Certificate {
                terms: vec![(0, 1)],
                remainders: vec![Large {
                    key: [1; 6],
                    coefficient: 1,
                }],
            },
            graph: Value::Null,
        };
        assert_eq!(
            expand(&[row], &vec![(0, 1)], 0, 1).unwrap().1,
            BTreeMap::from([([1; 6], 1)])
        );
    }
}
