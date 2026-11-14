//! Traces accelerate candidate basis construction. They never certify a basis.
//! Every accepted replay checks input reductions and every Buchberger pair.
//! Native traces are bounded in retained words, not transient matrix storage.

use super::*;
use std::collections::BTreeMap;

#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize)]
pub struct TraceStats {
    pub learned: u64,
    pub replay_attempts: u64,
    pub accepted: u64,
    pub rejected: u64,
    pub fallbacks: u64,
    pub certification_reductions: u64,
    pub certification_pairs: u64,
    pub certification_multiplications: u64,
    pub certification_term_steps: u64,
    pub retained_words: usize,
    pub storage_refusals: u64,
}

#[derive(Clone, PartialEq, Eq)]
struct Key {
    p: u64,
    vars: usize,
    order: Ordering,
    degree: u32,
    weights: Option<Vec<u8>>,
    profile: Vec<Option<usize>>,
    input_lms: Vec<Vec<u32>>,
}

struct Entry {
    key: Key,
    trace: F4Trace,
    words: usize,
}

/// Create a separate pool per target and encoding/variable-order family.
/// Fields and profiles are also checked on every node. A key hit alone does
/// not authorize a replay: numerical specializations still need certification.
pub struct TracePool {
    family: String,
    capacity: usize,
    max_words: usize,
    entries: Vec<Entry>,
    stats: TraceStats,
}

impl TracePool {
    pub fn new(family: &str, capacity: usize, max_words: usize) -> Result<Self, String> {
        if family.is_empty()
            || family.len() > 512
            || !(1..=16).contains(&capacity)
            || !(1..=1_000_000).contains(&max_words)
        {
            return Err(
                "trace family, capacity 1..16 and retained-word cap 1..1000000 required".into(),
            );
        }
        Ok(Self {
            family: family.into(),
            capacity,
            max_words,
            entries: vec![],
            stats: TraceStats::default(),
        })
    }

    pub fn family(&self) -> &str {
        &self.family
    }
    pub fn stats(&self) -> TraceStats {
        self.stats.clone()
    }

    pub(super) fn run(
        &mut self,
        input: &[Poly],
        vars: usize,
        p: u64,
        opts: &F4Options,
        profile: &[Option<usize>],
    ) -> F4Report {
        let key = Key {
            p,
            vars,
            order: opts.order,
            degree: opts.max_degree,
            weights: opts.weights.clone(),
            profile: profile.to_vec(),
            input_lms: input
                .iter()
                .map(|g| {
                    normalise(g, p, opts.order)
                        .first()
                        .map_or_else(Vec::new, |t| t.0.clone())
                })
                .collect(),
        };
        let mut rejected_work = 0;
        let mut rejected_shape = (0, 0);
        if let Some(i) = self.entries.iter().position(|e| e.key == key) {
            self.stats.replay_attempts += 1;
            let mut r = f4_replay(input, vars, p, opts, &self.entries[i].trace);
            let before = self.stats.certification_multiplications;
            let accepted = !r.timed_out
                && !r.trace_mismatch
                && certify(input, &r.basis, p, opts, &mut self.stats);
            r.field_ops += self.stats.certification_multiplications - before;
            if expired(opts) {
                r.timed_out = true;
            }
            if accepted && !r.timed_out {
                self.stats.accepted += 1;
                return r;
            }
            self.stats.rejected += 1;
            if r.timed_out {
                return r;
            }
            rejected_work = r.field_ops;
            rejected_shape = (r.max_rows, r.max_cols);
            self.stats.fallbacks += 1;
            let old = self.entries.remove(i);
            self.stats.retained_words -= old.words;
        }
        let (mut r, trace) = f4_record(input, vars, p, opts);
        r.field_ops += rejected_work;
        r.max_rows = r.max_rows.max(rejected_shape.0);
        r.max_cols = r.max_cols.max(rejected_shape.1);
        self.stats.learned += 1;
        if !r.timed_out && !trace.stopped && r.pairs_above_bound == 0 {
            let words = trace.input_lms.iter().map(Vec::len).sum::<usize>()
                + trace
                    .steps
                    .iter()
                    .map(|s| {
                        1 + s.rows.iter().map(|(_, m)| 1 + m.len()).sum::<usize>()
                            + s.new_lms.iter().map(Vec::len).sum::<usize>()
                    })
                    .sum::<usize>();
            if words <= self.max_words {
                while !self.entries.is_empty()
                    && (self.entries.len() >= self.capacity
                        || self.stats.retained_words + words > self.max_words)
                {
                    self.stats.retained_words -= self.entries.remove(0).words;
                }
                self.stats.retained_words += words;
                self.entries.push(Entry { key, trace, words });
            } else {
                self.stats.storage_refusals += 1;
            }
        }
        r
    }
}

fn expired(opts: &F4Options) -> bool {
    opts.deadline.is_some_and(|d| Instant::now() >= d)
}

// Zero-reduction with explicit coefficient-multiplication and term-step
// counters. Input and basis are monic, sorted and immutable. A resource cap
// refuses certification, causing ordinary F4 fallback, rather than accepting
// an unchecked replay. The cap bounds this checker, not the native F4 run.
fn reduces_to_zero(
    f: &Poly,
    basis: &[Poly],
    p: u64,
    opts: &F4Options,
    stats: &mut TraceStats,
) -> bool {
    stats.certification_reductions += 1;
    let mut work = BTreeMap::<Vec<u32>, u64>::new();
    for (m, c) in f {
        let v = work.entry(m.clone()).or_default();
        *v = (*v + c) % p;
    }
    let mut steps = 0usize;
    while !work.is_empty() {
        if expired(opts) || steps >= 1_000_000 || work.len() > 100_000 {
            return false;
        }
        let m = work
            .keys()
            .max_by(|a, b| cmp_monomial(a, b, opts.order))
            .unwrap()
            .clone();
        let c = work.remove(&m).unwrap();
        if c == 0 {
            continue;
        }
        steps += 1;
        stats.certification_term_steps += 1;
        let Some(g) = basis.iter().find(|g| divides(&g[0].0, &m)) else {
            return false;
        };
        for (e, gc) in &g[1..] {
            let shifted: Option<Vec<u32>> = e
                .iter()
                .zip(&m)
                .zip(&g[0].0)
                .map(|((x, a), b)| x.checked_add(a - b))
                .collect();
            let Some(shifted) = shifted else {
                return false;
            };
            let v = work.entry(shifted).or_default();
            *v = (*v + mulmod(p - c, *gc, p)) % p;
            stats.certification_multiplications += 1;
        }
    }
    true
}

fn certify(
    input: &[Poly],
    basis: &[Poly],
    p: u64,
    opts: &F4Options,
    stats: &mut TraceStats,
) -> bool {
    if basis
        .iter()
        .any(|g| g.is_empty() || g[0].1 != 1 || *g != normalise(g, p, opts.order))
    {
        return false;
    }
    for f in input {
        if !reduces_to_zero(f, basis, p, opts, stats) {
            return false;
        }
    }
    let mut pairs = 0usize;
    for i in 0..basis.len() {
        for j in 0..i {
            pairs += 1;
            stats.certification_pairs += 1;
            if pairs > 100_000 || expired(opts) {
                return false;
            }
            let l = lcm(&basis[i][0].0, &basis[j][0].0);
            let mut terms = Vec::new();
            for (g, negative) in [(&basis[i], false), (&basis[j], true)] {
                for (m, c) in g {
                    let shifted: Option<Vec<u32>> = m
                        .iter()
                        .zip(&l)
                        .zip(&g[0].0)
                        .map(|((x, a), b)| x.checked_add(a - b))
                        .collect();
                    let Some(shifted) = shifted else {
                        return false;
                    };
                    terms.push((shifted, if negative { p - c } else { *c }));
                }
            }
            if !reduces_to_zero(&terms, basis, p, opts, stats) {
                return false;
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    fn system() -> Vec<Poly> {
        vec![
            vec![(vec![1, 1], 1), (vec![0, 0], 4)],
            vec![(vec![2, 0], 1), (vec![0, 0], 4)],
        ]
    }
    fn roots(pool: &mut TracePool) -> Vec<Vec<u64>> {
        let mut roots = vec![];
        let r = solve_visit_reusing(
            &system(),
            2,
            5,
            &F4Options::new(Ordering::Grevlex, 8),
            &[Some((0..5).collect()), Some((0..5).collect())],
            pool,
            |x| {
                roots.push(x.to_vec());
                false
            },
        )
        .unwrap();
        assert!(r.complete);
        roots.sort();
        roots
    }
    #[test]
    fn accepted_replay_preserves_complete_roots_and_records_work() {
        let mut pool = TracePool::new("aps:partitioned3", 4, 10000).unwrap();
        assert_eq!(roots(&mut pool), vec![vec![1, 1], vec![4, 4]]);
        assert_eq!(roots(&mut pool), vec![vec![1, 1], vec![4, 4]]);
        assert!(pool.stats().accepted > 0);
        assert!(pool.stats().certification_reductions > 0);
    }
    #[test]
    fn damaged_trace_cannot_drop_solutions_and_falls_back() {
        let mut pool = TracePool::new("aps:partitioned3", 4, 10000).unwrap();
        let expected = roots(&mut pool);
        // Alter the recorded input fingerprint while preserving the pool key,
        // so this is a real cache hit followed by a native mismatch.
        for e in &mut pool.entries {
            if let Some(m) = e.trace.input_lms.first_mut() {
                if let Some(power) = m.first_mut() {
                    *power += 1;
                }
            }
        }
        assert_eq!(roots(&mut pool), expected);
        assert!(pool.stats().rejected > 0 && pool.stats().fallbacks > 0);
    }
    #[test]
    fn storage_cap_retains_only_traces_that_fit_without_restricting_search() {
        let mut pool = TracePool::new("aps:partitioned3", 1, 1).unwrap();
        assert_eq!(roots(&mut pool), vec![vec![1, 1], vec![4, 4]]);
        assert_eq!(pool.stats().retained_words, 1);
        assert_eq!(pool.entries.len(), 1);
        assert_eq!(pool.entries[0].words, 1);
        assert!(pool.stats().storage_refusals > 0);
        assert!(TracePool::new("", 1, 10).is_err());
        assert!(TracePool::new("aps", 17, 10).is_err());
    }

    #[test]
    fn buchberger_check_rejects_inputs_that_are_not_a_groebner_basis() {
        let input = system();
        let basis: Vec<_> = input
            .iter()
            .map(|g| normalise(g, 5, Ordering::Grevlex))
            .collect();
        let mut stats = TraceStats::default();
        assert!(!certify(
            &input,
            &basis,
            5,
            &F4Options::new(Ordering::Grevlex, 8),
            &mut stats
        ));
        assert!(stats.certification_pairs > 0);
    }

    #[test]
    fn coefficient_specialization_never_reuses_the_old_root_set() {
        let mut pool = TracePool::new("aps:partitioned3", 4, 10000).unwrap();
        assert_eq!(roots(&mut pool), vec![vec![1, 1], vec![4, 4]]);
        let mut changed = system();
        changed[0].pop();
        let mut actual = vec![];
        let r = solve_visit_reusing(
            &changed,
            2,
            5,
            &F4Options::new(Ordering::Grevlex, 8),
            &[Some((0..5).collect()), Some((0..5).collect())],
            &mut pool,
            |x| {
                actual.push(x.to_vec());
                false
            },
        )
        .unwrap();
        actual.sort();
        assert!(r.complete);
        assert_eq!(actual, vec![vec![1, 0], vec![4, 0]]);
        assert!(pool.stats().replay_attempts > 0);
    }
}
