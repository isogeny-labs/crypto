//! Online admission of partial relations under a memory cap, scored by how
//! soon the *useful* factor-base rank reaches its target.
//!
//! Model. Relations arrive in collection order, each tagged with the draw
//! (trial) that produced it. A complete relation (no large prime after
//! canonicalisation mod `ℓ`) is always kept. A partial relation (one or two
//! large primes) is stored only if an [`AdmissionRule`] accepts it *on
//! arrival* and fewer than `cap` partials are stored; nothing is ever evicted.
//! Stored partials enter a [`ForestWitnessExtractor`]; each time one closes a
//! cycle, the combined relation (the witness's sources, summed mod `ℓ`) is
//! harvested at once. The score is the draw at which the rank of the
//! factor-base block of all kept and harvested rows first reaches `target`:
//! the collection cost the rule pays, under that memory cap, to obtain a full
//! relation matrix.
//!
//! Why this metric. LP left nullity (`super::gain_dsu_rank`) is not useful
//! rank: combinations can be dependent in the factor-base block, and once it
//! exceeds the factor-base size further combinations are surplus
//! (`super::projected_rank` makes the same point for materialised rows). The
//! first version of this module scored offline selections by left nullity;
//! that design and why it was replaced are in
//! `research/lp_retention_20261125/README.md`.
//!
//! Guarantee used by the tests: [`AdmissionRule::All`] stores a superset of
//! every other rule's partials at every draw, and the cycle space of a
//! subgraph lies in that of the graph, so its useful rank is never lower and
//! it reaches any target no later. [`AdmissionRule::CompleteOnly`] is the
//! matching lower reference.
//!
//! Arithmetic is over the prime `ℓ` (caller obligation, as in the parent
//! module). Nothing here claims a speedup of a complete solve.

use super::super::large_prime_filter::PartialRelation;
use super::witness::ForestWitnessExtractor;
use super::{add_mod, inv_mod, mul_mod, neg_mod, valid_modulus, LpGraphError};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap, HashSet};

/// Which arriving partials a collector stores. Every rule except `All` and
/// `CompleteOnly` also stops storing once `cap` partials are stored.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AdmissionRule {
    /// Store every partial, no cap: the upper reference.
    All,
    /// Store no partial: complete relations only, the lower reference.
    CompleteOnly,
    /// Store partials in arrival order until the cap is reached.
    FirstCome,
    /// Store only one-large-prime partials.
    SinglesOnly,
    /// Store a partial only if every large-prime identity is at most the
    /// bound: a tighter large-prime bound. Meaningful where identities are
    /// sizes (F_p*); on curves an identity is a point label.
    MaxLp(u64),
    /// Store a one-LP partial always; store a two-LP partial only if one of
    /// its large primes appeared in an earlier arrival (stored or not). Needs
    /// a set of seen identities, which is O(distinct LPs), not O(rows).
    SeenLp,
}

impl AdmissionRule {
    pub fn name(&self) -> String {
        match self {
            AdmissionRule::All => "all".into(),
            AdmissionRule::CompleteOnly => "complete_only".into(),
            AdmissionRule::FirstCome => "first_come".into(),
            AdmissionRule::SinglesOnly => "singles_only".into(),
            AdmissionRule::MaxLp(b) => format!("max_lp_{b}"),
            AdmissionRule::SeenLp => "seen_lp".into(),
        }
    }
}

/// One relation in collection order. `draw` is the 1-based trial that
/// produced it; trials that produced nothing do not appear.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StreamRow {
    pub draw: u64,
    pub relation: PartialRelation,
}

/// Exact incremental rank of factor-base rows over `F_ℓ` (dense pivots; for
/// factor bases of up to a few thousand columns).
#[derive(Clone, Debug)]
pub struct IncrementalRank {
    modulus: u64,
    columns: usize,
    pivots: Vec<Option<Vec<u64>>>,
    rank: usize,
}

impl IncrementalRank {
    pub fn new(columns: usize, modulus: u64) -> Result<Self, LpGraphError> {
        if !valid_modulus(modulus) {
            return Err(LpGraphError::InvalidModulus);
        }
        Ok(Self {
            modulus,
            columns,
            pivots: vec![None; columns],
            rank: 0,
        })
    }

    pub fn rank(&self) -> usize {
        self.rank
    }

    /// Add a sparse row `(column, coefficient)`; returns whether rank grew.
    pub fn add(&mut self, row: &[(u32, u64)]) -> Result<bool, LpGraphError> {
        let p = self.modulus;
        let mut dense = vec![0u64; self.columns];
        for &(c, a) in row {
            let c = c as usize;
            if c >= self.columns {
                return Err(LpGraphError::ColumnOutOfRange { column: c });
            }
            dense[c] = add_mod(dense[c], a % p, p);
        }
        for col in 0..self.columns {
            let a = dense[col];
            if a == 0 {
                continue;
            }
            match &self.pivots[col] {
                Some(piv) => {
                    for (x, &y) in dense[col..].iter_mut().zip(&piv[col..]) {
                        if y != 0 {
                            *x = add_mod(*x, neg_mod(mul_mod(a, y, p), p), p);
                        }
                    }
                }
                None => {
                    let inv = inv_mod(a, p).ok_or(LpGraphError::NonInvertible { value: a })?;
                    for x in dense[col..].iter_mut() {
                        *x = mul_mod(*x, inv, p);
                    }
                    self.pivots[col] = Some(dense);
                    self.rank += 1;
                    return Ok(true);
                }
            }
        }
        Ok(false)
    }
}

/// A harvested combination, kept only when `keep_harvested` is set: the
/// stored-partial ids and coefficients (replayable with
/// `large_prime_filter::replay_complete_relation`) and the combined row.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Harvested {
    pub draw: u64,
    pub sources: Vec<(u64, u64)>,
    pub small: Vec<(u32, u64)>,
    pub rhs: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdmissionOutcome {
    pub rule: AdmissionRule,
    pub cap: Option<usize>,
    pub target: usize,
    /// First draw at which useful rank reached `target`; `None` if never.
    pub draws_to_target: Option<u64>,
    /// Useful rank when the simulation stopped (at target or stream end).
    pub final_rank: usize,
    /// Stream rows read before stopping.
    pub rows_read: usize,
    pub complete_relations: usize,
    pub stored_partials: usize,
    pub harvested_relations: usize,
    /// Partials with more than two large primes: never stored.
    pub dropped_wide: usize,
    /// `(draw, rank)` at every rank increase.
    pub trace: Vec<(u64, usize)>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub harvested: Vec<Harvested>,
}

fn canonical<K: Ord + Copy>(terms: &[(K, u64)], p: u64) -> Vec<(K, u64)> {
    let mut m = BTreeMap::<K, u64>::new();
    for &(k, c) in terms {
        let e = m.entry(k).or_default();
        *e = add_mod(*e, c % p, p);
    }
    m.into_iter().filter(|&(_, c)| c != 0).collect()
}

/// Run one admission rule over `stream`, stopping when useful rank reaches
/// `target` (pass `usize::MAX` to read the whole stream).
pub fn simulate_admission(
    stream: &[StreamRow],
    fb_columns: usize,
    modulus: u64,
    rule: AdmissionRule,
    cap: Option<usize>,
    target: usize,
    keep_harvested: bool,
) -> Result<AdmissionOutcome, LpGraphError> {
    let p = modulus;
    let distinct: HashSet<u64> = stream
        .iter()
        .flat_map(|r| r.relation.large.iter().map(|&(q, _)| q))
        .collect();
    let mut column_of: HashMap<u64, usize> = HashMap::with_capacity(distinct.len());
    let mut forest = ForestWitnessExtractor::new(distinct.len(), p)?;
    let mut rank = IncrementalRank::new(fb_columns, p)?;
    let mut stored: Vec<&PartialRelation> = Vec::new();
    let mut seen: HashSet<u64> = HashSet::new();
    let cap = match rule {
        AdmissionRule::All | AdmissionRule::CompleteOnly => None,
        _ => cap,
    };
    let mut out = AdmissionOutcome {
        rule,
        cap,
        target,
        draws_to_target: None,
        final_rank: 0,
        rows_read: 0,
        complete_relations: 0,
        stored_partials: 0,
        harvested_relations: 0,
        dropped_wide: 0,
        trace: Vec::new(),
        harvested: Vec::new(),
    };
    let record = |grew: bool, draw: u64, rank: usize, out: &mut AdmissionOutcome| {
        if grew {
            out.trace.push((draw, rank));
            if rank >= target && out.draws_to_target.is_none() {
                out.draws_to_target = Some(draw);
            }
        }
    };
    for row in stream {
        if out.draws_to_target.is_some() {
            break;
        }
        out.rows_read += 1;
        let rel = &row.relation;
        let large = canonical(&rel.large, p);
        if large.is_empty() {
            out.complete_relations += 1;
            let grew = rank.add(&canonical(&rel.small, p))?;
            record(grew, row.draw, rank.rank(), &mut out);
            continue;
        }
        if large.len() > 2 {
            out.dropped_wide += 1;
            continue;
        }
        let full = cap.is_some_and(|c| stored.len() >= c);
        let admit = !full
            && match rule {
                AdmissionRule::All | AdmissionRule::FirstCome => true,
                AdmissionRule::CompleteOnly => false,
                AdmissionRule::SinglesOnly => large.len() == 1,
                AdmissionRule::MaxLp(b) => large.iter().all(|&(q, _)| q <= b),
                AdmissionRule::SeenLp => {
                    large.len() == 1 || large.iter().any(|(q, _)| seen.contains(q))
                }
            };
        if matches!(rule, AdmissionRule::SeenLp) {
            seen.extend(large.iter().map(|&(q, _)| q));
        }
        if !admit {
            continue;
        }
        let mut terms = Vec::with_capacity(large.len());
        for &(q, c) in &large {
            let next = column_of.len();
            terms.push((*column_of.entry(q).or_insert(next), c));
        }
        stored.push(rel);
        out.stored_partials += 1;
        if let Some(w) = forest.add_row(&terms)? {
            let mut small = Vec::new();
            let mut rhs = 0u64;
            let mut sources = Vec::with_capacity(w.sources.len());
            for &(i, a) in &w.sources {
                let src = stored[i];
                small.extend(src.small.iter().map(|&(c, e)| (c, mul_mod(a, e % p, p))));
                rhs = add_mod(rhs, mul_mod(a, src.rhs % p, p), p);
                sources.push((src.id, a));
            }
            let small = canonical(&small, p);
            out.harvested_relations += 1;
            let grew = rank.add(&small)?;
            if keep_harvested {
                out.harvested.push(Harvested {
                    draw: row.draw,
                    sources,
                    small,
                    rhs,
                });
            }
            record(grew, row.draw, rank.rank(), &mut out);
        }
    }
    out.final_rank = rank.rank();
    Ok(out)
}
