//! Retain checked two-class edges until an endpoint has an exact unit anchor.
//!
//! Only components reached by a one-class source emit rows. Every retained
//! anchor has integer large part +D_v; every emitted witness has large part 0.
//! Unanchored cycles do not emit. No modular inverse or unsigned-cycle rule is
//! used. Sources must already have passed principal/class admission upstream.
use super::quadratic_pair_graph::{Class, LargeTerm, Witness};
use crate::cryptanalysis::jv_bitops as work;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, VecDeque};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Options {
    pub max_sources: usize,
    pub max_vertices: usize,
    /// Lifetime bound, including edges already activated; never silently reused.
    pub max_deferred_edges: usize,
    pub max_witness_terms: usize,
}
impl Default for Options {
    fn default() -> Self {
        Self {
            max_sources: 262144,
            max_vertices: 262144,
            max_deferred_edges: 262144,
            max_witness_terms: 256,
        }
    }
}
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Stats {
    pub sources: usize,
    pub vertices: usize,
    pub anchors: usize,
    pub full_sources: u64,
    pub cycles: u64,
    pub deferred: usize,
    pub activated: usize,
    pub live_deferred: usize,
    pub one_class_anchors: u64,
    pub two_class_anchors: u64,
    pub vertex_refusals: u64,
    pub edge_refusals: u64,
    pub witness_refusals: u64,
    pub max_stored_witness: usize,
    pub max_emitted_witness: usize,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Event {
    Full {
        source: usize,
        witness: Witness,
    },
    Anchor {
        source: usize,
        class: Class,
        witness: Witness,
    },
    Deferred {
        source: usize,
    },
    Refused {
        source: usize,
        reason: String,
    },
}
#[derive(Default)]
struct Node {
    anchor: Option<Witness>,
    pending: Vec<usize>,
}
struct Edge {
    source: usize,
    terms: [LargeTerm; 2],
    live: bool,
}
pub struct Graph {
    options: Options,
    nodes: BTreeMap<Class, Node>,
    edges: Vec<Edge>,
    stats: Stats,
}
fn combine(parts: &[(i8, &[(usize, i8)])]) -> Result<Witness, &'static str> {
    let mut out = BTreeMap::<usize, i16>::new();
    for &(scale, row) in parts {
        for &(id, c) in row {
            let v = out.entry(id).or_default();
            *v = v
                .checked_add(scale as i16 * c as i16)
                .ok_or("witness overflow")?;
            work::words(5);
        }
    }
    out.into_iter()
        .filter(|(_, v)| *v != 0)
        .map(|(id, v)| {
            i8::try_from(v)
                .map(|c| (id, c))
                .map_err(|_| "witness coefficient overflow")
        })
        .collect()
}
impl Graph {
    pub fn new(options: Options) -> Result<Self, &'static str> {
        if !(1..=1_000_000).contains(&options.max_sources)
            || !(1..=262144).contains(&options.max_vertices)
            || !(1..=1_000_000).contains(&options.max_deferred_edges)
            || !(1..=4096).contains(&options.max_witness_terms)
        {
            return Err("bounded source, vertex, edge and witness options required");
        }
        Ok(Self {
            options,
            nodes: BTreeMap::new(),
            edges: vec![],
            stats: Stats::default(),
        })
    }
    pub fn stats(&self) -> &Stats {
        &self.stats
    }
    pub fn anchors(&self) -> impl Iterator<Item = (&Class, &Witness)> {
        self.nodes
            .iter()
            .filter_map(|(k, n)| n.anchor.as_ref().map(|w| (k, w)))
    }
    fn get(&self, k: &Class) -> Option<Witness> {
        work::probe(6);
        self.nodes.get(k).and_then(|n| n.anchor.as_ref()).map(|w| {
            work::words(2 * w.len() as u64);
            w.clone()
        })
    }
    fn refuse(&mut self, source: usize, reason: &str) -> Event {
        Event::Refused {
            source,
            reason: reason.into(),
        }
    }
    fn emit(&mut self, source: usize, witness: Witness, full: bool) -> Event {
        if witness.len() > self.options.max_witness_terms {
            self.stats.witness_refusals += 1;
            return self.refuse(source, "witness cap");
        }
        self.stats.max_emitted_witness = self.stats.max_emitted_witness.max(witness.len());
        if full {
            self.stats.full_sources += 1;
        } else {
            self.stats.cycles += 1;
        }
        Event::Full { source, witness }
    }
    fn anchor(
        &mut self,
        source: usize,
        class: Class,
        witness: Witness,
        two: bool,
        queue: &mut VecDeque<Class>,
    ) -> Event {
        if witness.len() > self.options.max_witness_terms {
            self.stats.witness_refusals += 1;
            return self.refuse(source, "witness cap");
        }
        if !self.nodes.contains_key(&class) && self.nodes.len() >= self.options.max_vertices {
            self.stats.vertex_refusals += 1;
            return self.refuse(source, "vertex cap");
        }
        // Newly anchored endpoint was absent from all previous anchor paths.
        assert!(witness.iter().all(|&(_, c)| [-1, 1].contains(&c)));
        let n = self.nodes.entry(class).or_default();
        assert!(n.anchor.is_none());
        n.anchor = Some(witness.clone());
        work::probe(6);
        work::words((2 * witness.len() + 7) as u64);
        self.stats.anchors += 1;
        self.stats.vertices = self.nodes.len();
        self.stats.max_stored_witness = self.stats.max_stored_witness.max(witness.len());
        if two {
            self.stats.two_class_anchors += 1;
        } else {
            self.stats.one_class_anchors += 1;
        }
        queue.push_back(class);
        Event::Anchor {
            source,
            class,
            witness,
        }
    }
    fn process(
        &mut self,
        source: usize,
        large: &[LargeTerm],
        queue: &mut VecDeque<Class>,
    ) -> Result<Event, &'static str> {
        let unit = [(source, 1)];
        Ok(match large {
            [] => self.emit(source, unit.to_vec(), true),
            [x] => match self.get(&x.class) {
                Some(a) => self.emit(source, combine(&[(1, &unit), (-x.coefficient, &a)])?, false),
                None => self.anchor(source, x.class, vec![(source, x.coefficient)], false, queue),
            },
            [x, y] => match (self.get(&x.class), self.get(&y.class)) {
                (Some(a), Some(b)) => self.emit(
                    source,
                    combine(&[(1, &unit), (-x.coefficient, &a), (-y.coefficient, &b)])?,
                    false,
                ),
                (Some(a), None) => self.anchor(
                    source,
                    y.class,
                    combine(&[(y.coefficient, &unit), (-x.coefficient * y.coefficient, &a)])?,
                    true,
                    queue,
                ),
                (None, Some(b)) => self.anchor(
                    source,
                    x.class,
                    combine(&[(x.coefficient, &unit), (-x.coefficient * y.coefficient, &b)])?,
                    true,
                    queue,
                ),
                (None, None) => {
                    let needed = large
                        .iter()
                        .filter(|x| !self.nodes.contains_key(&x.class))
                        .count();
                    if self.nodes.len() + needed > self.options.max_vertices {
                        self.stats.vertex_refusals += 1;
                        self.refuse(source, "vertex cap")
                    } else if self.edges.len() >= self.options.max_deferred_edges {
                        self.stats.edge_refusals += 1;
                        self.refuse(source, "deferred edge cap")
                    } else {
                        let id = self.edges.len();
                        self.edges.push(Edge {
                            source,
                            terms: [*x, *y],
                            live: true,
                        });
                        for term in large {
                            self.nodes.entry(term.class).or_default().pending.push(id);
                            work::probe(6);
                            work::words(8);
                        }
                        self.stats.vertices = self.nodes.len();
                        self.stats.deferred += 1;
                        self.stats.live_deferred += 1;
                        Event::Deferred { source }
                    }
                }
            },
            _ => unreachable!("validated arity"),
        })
    }
    /// Consecutive original source ids; one arrival may wake earlier edges.
    /// Invalid shapes leave all state unchanged. Bounded refusals consume the id.
    pub fn accept(
        &mut self,
        source: usize,
        large: &[LargeTerm],
    ) -> Result<Vec<Event>, &'static str> {
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
        let mut queue = VecDeque::new();
        let mut events = vec![self.process(source, large, &mut queue)?];
        while let Some(class) = queue.pop_front() {
            let pending =
                std::mem::take(&mut self.nodes.get_mut(&class).expect("queued anchor").pending);
            work::probe(6);
            work::words(2 + pending.len() as u64);
            for id in pending {
                work::words(2);
                if !self.edges[id].live {
                    continue;
                }
                let e = &mut self.edges[id];
                e.live = false;
                let (origin, terms) = (e.source, e.terms);
                self.stats.live_deferred -= 1;
                self.stats.activated += 1;
                let event = self.process(origin, &terms, &mut queue)?;
                assert!(!matches!(event, Event::Deferred { .. }));
                events.push(event);
            }
        }
        Ok(events)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn t(id: u64, c: i8) -> LargeTerm {
        LargeTerm {
            class: [id, 0, 0, 0, 0, 0],
            coefficient: c,
        }
    }
    fn incidence(sources: &[Vec<LargeTerm>], w: &Witness) -> BTreeMap<Class, i64> {
        let mut out = BTreeMap::new();
        for &(id, c) in w {
            for t in &sources[id] {
                *out.entry(t.class).or_default() += c as i64 * t.coefficient as i64;
            }
        }
        out.retain(|_, v| *v != 0);
        out
    }
    fn checked(
        g: &mut Graph,
        sources: &mut Vec<Vec<LargeTerm>>,
        row: Vec<LargeTerm>,
    ) -> Vec<Event> {
        let id = sources.len();
        sources.push(row.clone());
        let events = g.accept(id, &row).unwrap();
        for e in &events {
            match e {
                Event::Full { witness, .. } => assert!(incidence(sources, witness).is_empty()),
                Event::Anchor { class, witness, .. } => {
                    assert_eq!(incidence(sources, witness), BTreeMap::from([(*class, 1)]))
                }
                _ => {}
            }
        }
        for (k, w) in g.anchors() {
            assert_eq!(incidence(sources, w), BTreeMap::from([(*k, 1)]));
        }
        assert_eq!(g.stats.deferred, g.stats.activated + g.stats.live_deferred);
        events
    }
    #[test]
    fn deferred_chain_wakes_with_every_sign_orientation() {
        for bits in 0..32 {
            let sign = |i| if bits & (1 << i) == 0 { 1 } else { -1 };
            let mut g = Graph::new(Options::default()).unwrap();
            let mut s = vec![];
            checked(&mut g, &mut s, vec![t(0, sign(0)), t(1, sign(1))]);
            checked(&mut g, &mut s, vec![t(1, sign(2)), t(2, sign(3))]);
            assert_eq!(g.stats.anchors, 0);
            let events = checked(&mut g, &mut s, vec![t(2, sign(4))]);
            assert_eq!(
                events
                    .iter()
                    .filter(|e| matches!(e, Event::Anchor { .. }))
                    .count(),
                3
            );
            assert_eq!(g.stats.live_deferred, 0);
            assert_eq!(checked(&mut g, &mut s, vec![t(0, 1)]).len(), 1);
            assert_eq!(g.stats.cycles, 1);
        }
    }
    #[test]
    fn signed_cycle_waits_for_anchor_and_keeps_coefficient_two() {
        let mut g = Graph::new(Options::default()).unwrap();
        let mut s = vec![];
        for (x, y) in [(0, 1), (1, 2), (2, 0)] {
            checked(&mut g, &mut s, vec![t(x, 1), t(y, 1)]);
        }
        assert_eq!(g.stats.cycles, 0);
        assert_eq!(g.stats.anchors, 0);
        let e = checked(&mut g, &mut s, vec![t(0, 1)]);
        assert_eq!(g.stats.cycles, 1);
        assert!(e.iter().any(
            |e| matches!(e,Event::Full{witness,..} if witness.iter().any(|&(_,c)|c.abs()==2))
        ));
    }
    #[test]
    fn bounded_refusals_and_invalid_inputs_do_not_add_edges() {
        let mut g = Graph::new(Options {
            max_vertices: 2,
            max_deferred_edges: 1,
            ..Options::default()
        })
        .unwrap();
        let mut s = vec![];
        checked(&mut g, &mut s, vec![t(0, 1), t(1, 1)]);
        let saved = g.stats.clone();
        for bad in [
            vec![t(1, 0)],
            vec![t(1, 1), t(1, -1)],
            vec![t(0, 1), t(1, 1), t(2, 1)],
        ] {
            assert!(g.accept(1, &bad).is_err());
            assert_eq!(g.stats, saved);
        }
        checked(&mut g, &mut s, vec![t(0, -1), t(1, 1)]);
        assert_eq!(g.stats.edge_refusals, 1);
        checked(&mut g, &mut s, vec![t(1, 1), t(2, 1)]);
        assert_eq!(g.stats.vertex_refusals, 1);
        assert_eq!(g.stats.vertices, 2);
        checked(&mut g, &mut s, vec![t(0, 1)]);
        assert_eq!(g.stats.anchors, 2);
        assert_eq!(g.stats.activated, 1);
    }
    #[test]
    fn witness_cap_is_explicit_and_full_sources_are_independent() {
        let mut g = Graph::new(Options {
            max_witness_terms: 1,
            ..Options::default()
        })
        .unwrap();
        let mut s = vec![];
        checked(&mut g, &mut s, vec![t(0, 1), t(1, 1)]);
        checked(&mut g, &mut s, vec![t(0, 1)]);
        assert_eq!(g.stats.witness_refusals, 1);
        assert_eq!(g.stats.live_deferred, 0);
        assert_eq!(g.stats.anchors, 1);
        checked(&mut g, &mut s, vec![]);
        assert_eq!(g.stats.full_sources, 1);
    }
    #[test]
    fn synthetic_streams_check_every_integer_incidence() {
        for seed in 0..64u64 {
            let mut z = seed + 1;
            let mut g = Graph::new(Options::default()).unwrap();
            let mut sources = vec![];
            for i in 0..257 {
                z = z.wrapping_mul(6364136223846793005).wrapping_add(1);
                let x = (z >> 20) % 31;
                let y = (x + 1 + (z >> 32) % 30) % 31;
                let a = if z & 1 == 0 { 1 } else { -1 };
                let b = if z & 2 == 0 { 1 } else { -1 };
                let row = if i % 29 == 0 {
                    vec![]
                } else if i % 11 == 0 {
                    vec![t(x, a)]
                } else {
                    vec![t(x, a), t(y, b)]
                };
                checked(&mut g, &mut sources, row);
            }
        }
    }
    #[test]
    fn option_bounds_and_source_cap_are_enforced() {
        for o in [
            Options {
                max_sources: 0,
                ..Options::default()
            },
            Options {
                max_vertices: 262145,
                ..Options::default()
            },
            Options {
                max_deferred_edges: 0,
                ..Options::default()
            },
            Options {
                max_witness_terms: 4097,
                ..Options::default()
            },
        ] {
            assert!(Graph::new(o).is_err());
        }
        let mut g = Graph::new(Options {
            max_sources: 1,
            ..Options::default()
        })
        .unwrap();
        g.accept(0, &[]).unwrap();
        let saved = g.stats.clone();
        assert!(g.accept(1, &[]).is_err());
        assert_eq!(g.stats, saved);
    }
}
