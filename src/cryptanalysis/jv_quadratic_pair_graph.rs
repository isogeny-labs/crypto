//! Grounded signed graph for already-checked cofactor relations.
//!
//! Each source has zero, one or two distinct large classes with coefficients
//! +/-1. A retained anchor is an integer source combination whose large part
//! is exactly +D_v. New edges extend such anchors; both known endpoints yield
//! an exact cancellation witness. Ungrounded edges are discarded. These rows
//! are not rank certificates. The caller must check all principal relations
//! and class identities before admission; this core does not prove geometry.
use crate::cryptanalysis::jv_bitops as work;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub type Class = [u64; 6];
pub type Witness = Vec<(usize, i8)>;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LargeTerm {
    pub class: Class,
    pub coefficient: i8,
}
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Options {
    pub max_sources: usize,
    pub max_vertices: usize,
    pub max_witness_terms: usize,
}
impl Default for Options {
    fn default() -> Self {
        Self {
            max_sources: 262_144,
            max_vertices: 65_536,
            max_witness_terms: 256,
        }
    }
}
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Stats {
    pub sources: usize,
    pub vertices: usize,
    pub full_rows: u64,
    pub anchored_cycles: u64,
    pub one_class_anchors: u64,
    pub two_class_extensions: u64,
    pub discarded_ungrounded: u64,
    pub vertex_refusals: u64,
    pub witness_refusals: u64,
    pub max_stored_witness: usize,
    pub max_emitted_witness: usize,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Outcome {
    Full { witness: Witness },
    Anchor { class: Class, witness: Witness },
    Ungrounded,
    VertexCap,
    WitnessCap,
}
pub struct Graph {
    options: Options,
    anchors: BTreeMap<Class, Witness>,
    stats: Stats,
}
/// Aggregate exact signed source coefficients, never coefficients modulo 2 or L.
fn combine(parts: &[(i8, &[(usize, i8)])]) -> Witness {
    let mut coefficients: BTreeMap<usize, i16> = BTreeMap::new();
    let mut words = 0;
    for &(scale, terms) in parts {
        for &(id, coefficient) in terms {
            *coefficients.entry(id).or_default() += scale as i16 * coefficient as i16;
            words += 5;
        }
    }
    work::words(words);
    coefficients
        .into_iter()
        .filter(|(_, coefficient)| *coefficient != 0)
        .map(|(id, coefficient)| {
            (
                id,
                i8::try_from(coefficient).expect("at most two signed anchor paths"),
            )
        })
        .collect()
}
impl Graph {
    pub fn new(options: Options) -> Result<Self, &'static str> {
        if !(1..=1_000_000).contains(&options.max_sources)
            || !(1..=262_144).contains(&options.max_vertices)
            || !(1..=4096).contains(&options.max_witness_terms)
        {
            return Err("bounded sources, vertices and witness support required");
        }
        Ok(Self {
            options,
            anchors: BTreeMap::new(),
            stats: Stats::default(),
        })
    }
    /// Diagnostic reads do not move the scientific accounting state.
    pub fn stats(&self) -> &Stats {
        &self.stats
    }
    pub fn anchors(&self) -> &BTreeMap<Class, Witness> {
        &self.anchors
    }
    fn get(&self, class: &Class) -> Option<Witness> {
        work::probe(6);
        self.anchors.get(class).map(|w| {
            work::words((2 * w.len()) as u64);
            w.clone()
        })
    }
    fn emit(&mut self, witness: Witness, zero_large: bool) -> Outcome {
        if witness.len() > self.options.max_witness_terms {
            self.stats.witness_refusals += 1;
            return Outcome::WitnessCap;
        }
        self.stats.max_emitted_witness = self.stats.max_emitted_witness.max(witness.len());
        if zero_large {
            self.stats.full_rows += 1;
        } else {
            self.stats.anchored_cycles += 1;
        }
        Outcome::Full { witness }
    }
    fn retain(&mut self, class: Class, witness: Witness, two_large: bool) -> Outcome {
        if witness.len() > self.options.max_witness_terms {
            self.stats.witness_refusals += 1;
            return Outcome::WitnessCap;
        }
        if self.anchors.len() >= self.options.max_vertices {
            self.stats.vertex_refusals += 1;
            return Outcome::VertexCap;
        }
        // Retained anchors follow simple paths: each source coefficient is +/-1.
        assert!(witness.iter().all(|&(_, c)| c == 1 || c == -1));
        self.stats.max_stored_witness = self.stats.max_stored_witness.max(witness.len());
        work::probe(6);
        work::words((2 * witness.len() + 6) as u64);
        assert!(self.anchors.insert(class, witness.clone()).is_none());
        self.stats.vertices = self.anchors.len();
        if two_large {
            self.stats.two_class_extensions += 1;
        } else {
            self.stats.one_class_anchors += 1;
        }
        Outcome::Anchor { class, witness }
    }
    /// Source ids are consecutive valid-source ids, including discarded edges
    /// and bounded refusals. Invalid shapes leave the entire state unchanged.
    pub fn accept(&mut self, source: usize, large: &[LargeTerm]) -> Result<Outcome, &'static str> {
        if source != self.stats.sources
            || source >= self.options.max_sources
            || large.len() > 2
            || large.iter().any(|x| ![-1, 1].contains(&x.coefficient))
            || (large.len() == 2 && large[0].class == large[1].class)
        {
            return Err("source order/cap, distinct classes and unit coefficients required");
        }
        self.stats.sources += 1;
        work::words(4 + 7 * large.len() as u64);
        let unit = vec![(source, 1)];
        let outcome = match large {
            [] => self.emit(unit, true),
            [x] => match self.get(&x.class) {
                Some(a) => self.emit(combine(&[(1, &unit), (-x.coefficient, &a)]), false),
                None => self.retain(x.class, vec![(source, x.coefficient)], false),
            },
            [x, y] => match (self.get(&x.class), self.get(&y.class)) {
                (Some(a), Some(b)) => self.emit(
                    combine(&[(1, &unit), (-x.coefficient, &a), (-y.coefficient, &b)]),
                    false,
                ),
                (Some(a), None) => self.retain(
                    y.class,
                    combine(&[(y.coefficient, &unit), (-x.coefficient * y.coefficient, &a)]),
                    true,
                ),
                (None, Some(b)) => self.retain(
                    x.class,
                    combine(&[(x.coefficient, &unit), (-x.coefficient * y.coefficient, &b)]),
                    true,
                ),
                (None, None) => {
                    self.stats.discarded_ungrounded += 1;
                    Outcome::Ungrounded
                }
            },
            _ => unreachable!("shape checked"),
        };
        Ok(outcome)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn term(class: usize, coefficient: i8) -> LargeTerm {
        LargeTerm {
            class: [class as u64, 0, 0, 0, 0, 0],
            coefficient,
        }
    }
    /// Independent integer incidence multiplication, unrelated to graph paths.
    fn incidence(sources: &[Vec<LargeTerm>], witness: &Witness) -> BTreeMap<Class, i64> {
        let mut out = BTreeMap::new();
        for &(id, weight) in witness {
            for edge in &sources[id] {
                *out.entry(edge.class).or_default() += weight as i64 * edge.coefficient as i64;
            }
        }
        out.retain(|_, c| *c != 0);
        out
    }
    fn checked(
        graph: &mut Graph,
        sources: &mut Vec<Vec<LargeTerm>>,
        row: Vec<LargeTerm>,
    ) -> Outcome {
        let id = sources.len();
        sources.push(row.clone());
        let out = graph.accept(id, &row).unwrap();
        match &out {
            Outcome::Full { witness } => assert!(incidence(sources, witness).is_empty()),
            Outcome::Anchor { class, witness } => {
                assert_eq!(incidence(sources, witness), BTreeMap::from([(*class, 1)]))
            }
            _ => {}
        }
        for (class, witness) in graph.anchors() {
            assert_eq!(incidence(sources, witness), BTreeMap::from([(*class, 1)]));
        }
        out
    }
    #[test]
    fn every_orientation_of_anchors_and_extensions_cancels_exactly() {
        for a in [-1, 1] {
            for b in [-1, 1] {
                for c in [-1, 1] {
                    for d in [-1, 1] {
                        for e in [-1, 1] {
                            let mut g = Graph::new(Options::default()).unwrap();
                            let mut rows = Vec::new();
                            checked(&mut g, &mut rows, vec![term(0, a)]);
                            checked(&mut g, &mut rows, vec![term(0, b), term(1, c)]);
                            assert!(matches!(
                                checked(&mut g, &mut rows, vec![term(0, d), term(1, e)]),
                                Outcome::Full { .. }
                            ));
                            assert!(matches!(
                                checked(&mut g, &mut rows, vec![term(1, -c)]),
                                Outcome::Full { .. }
                            ));
                        }
                    }
                }
            }
        }
    }
    #[test]
    fn ungrounded_odd_cycle_is_not_a_full_relation() {
        let mut g = Graph::new(Options::default()).unwrap();
        let mut rows = Vec::new();
        for (a, b) in [(0, 1), (1, 2), (2, 0)] {
            assert_eq!(
                checked(&mut g, &mut rows, vec![term(a, 1), term(b, 1)]),
                Outcome::Ungrounded
            );
        }
        assert_eq!(g.stats().anchored_cycles, 0);
        assert!(g.anchors().is_empty());
        // Summing this unsigned triangle would leave 2 of every large class.
        assert_eq!(
            incidence(&rows, &vec![(0, 1), (1, 1), (2, 1)])
                .values()
                .copied()
                .collect::<Vec<_>>(),
            vec![2, 2, 2]
        );
    }
    #[test]
    fn common_paths_retain_integer_coefficient_two() {
        let mut g = Graph::new(Options::default()).unwrap();
        let mut rows = Vec::new();
        checked(&mut g, &mut rows, vec![term(0, 1)]);
        checked(&mut g, &mut rows, vec![term(0, 1), term(1, 1)]);
        let out = checked(&mut g, &mut rows, vec![term(0, 1), term(1, -1)]);
        let Outcome::Full { witness } = out else {
            panic!("grounded edge")
        };
        assert!(witness.iter().any(|&(_, c)| c.abs() == 2));
        assert_eq!(witness, vec![(0, -2), (1, 1), (2, 1)]);
    }
    #[test]
    fn caps_preserve_existing_anchors_and_consume_valid_source_ids() {
        let mut g = Graph::new(Options {
            max_vertices: 1,
            ..Options::default()
        })
        .unwrap();
        let mut rows = Vec::new();
        checked(&mut g, &mut rows, vec![term(0, 1)]);
        let old = g.anchors().clone();
        assert_eq!(
            checked(&mut g, &mut rows, vec![term(1, 1)]),
            Outcome::VertexCap
        );
        assert_eq!(g.anchors(), &old);
        let mut g = Graph::new(Options {
            max_witness_terms: 2,
            ..Options::default()
        })
        .unwrap();
        let mut rows = Vec::new();
        checked(&mut g, &mut rows, vec![term(0, -1)]);
        checked(&mut g, &mut rows, vec![term(0, 1), term(1, -1)]);
        let old = g.anchors().clone();
        assert_eq!(
            checked(&mut g, &mut rows, vec![term(1, 1), term(2, -1)]),
            Outcome::WitnessCap
        );
        assert_eq!(g.anchors(), &old);
        assert_eq!(g.stats().sources, 3);
        assert!(matches!(
            checked(&mut g, &mut rows, vec![]),
            Outcome::Full { .. }
        ));
    }
    #[test]
    fn invalid_shapes_and_source_order_are_atomic() {
        let mut g = Graph::new(Options::default()).unwrap();
        let before = g.stats().clone();
        for (id, row) in [
            (1, vec![]),
            (0, vec![term(0, 0)]),
            (0, vec![term(0, 1), term(0, -1)]),
            (0, vec![term(0, 1), term(1, 1), term(2, 1)]),
        ] {
            assert!(g.accept(id, &row).is_err());
            assert_eq!(g.stats(), &before);
            assert!(g.anchors().is_empty());
        }
        assert!(Graph::new(Options {
            max_witness_terms: 0,
            ..Options::default()
        })
        .is_err());
    }
    #[test]
    fn synthetic_signed_incidence_controls_replay_every_witness() {
        for seed in 0..64usize {
            let mut g = Graph::new(Options::default()).unwrap();
            let mut rows = Vec::new();
            checked(&mut g, &mut rows, vec![term(seed % 16, 1)]);
            for i in 0..256usize {
                let a = (i * 7 + seed) % 16;
                let mut b = (i * 5 + seed + 1) % 16;
                if a == b {
                    b = (b + 1) % 16;
                }
                let s = if i & 1 == 0 { 1 } else { -1 };
                let t = if i & 2 == 0 { 1 } else { -1 };
                let row = if i % 11 == 0 {
                    vec![]
                } else if i % 7 == 0 {
                    vec![term(a, s)]
                } else {
                    vec![term(a, s), term(b, t)]
                };
                checked(&mut g, &mut rows, row);
            }
            assert_eq!(g.stats().sources, 257);
            assert!(g.stats().anchored_cycles > 0);
        }
    }
}
