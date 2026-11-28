//! Spanning-forest elimination for hyperelliptic relations with at most two
//! off-base rational places. Integral cross-products eliminate repeated
//! places; wide intermediate coefficients are kept exact, and a cycle is
//! withheld only if an emitted factor-base coefficient exceeds `i64`.

use std::collections::{HashMap, HashSet, VecDeque};

use num_bigint::{BigInt, BigUint};
use num_integer::Integer;
use num_traits::{Signed, ToPrimitive, Zero};

use super::hyperelliptic_index_calculus::HecRelation;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Vertex {
    Ground,
    Place(Vec<u32>),
}

#[derive(Clone)]
struct Edge {
    u: Vertex,
    v: Vertex,
    cu: i64,
    cv: i64,
    relation: HecRelation,
}

/// The cycle accumulator can exceed `i64` before later edges cancel it.
/// Only a complete factor-base row is converted back to `HecRelation`.
#[derive(Clone)]
struct WideRelation {
    coef_a: BigUint,
    coef_b: BigUint,
    entries: HashMap<usize, BigInt>,
}

impl From<&HecRelation> for WideRelation {
    fn from(relation: &HecRelation) -> Self {
        let mut entries = HashMap::<usize, BigInt>::new();
        for &(index, value) in &relation.entries {
            *entries.entry(index).or_default() += value;
        }
        entries.retain(|_, value| !value.is_zero());
        Self {
            coef_a: relation.coef_a.clone(),
            coef_b: relation.coef_b.clone(),
            entries,
        }
    }
}

impl WideRelation {
    fn finish(self) -> Option<HecRelation> {
        let mut entries: Vec<_> = self
            .entries
            .into_iter()
            .map(|(index, value)| Some((index, value.to_i64()?)))
            .collect::<Option<_>>()?;
        entries.sort_unstable_by_key(|&(index, _)| index);
        Some(HecRelation {
            coef_a: self.coef_a,
            coef_b: self.coef_b,
            entries,
        })
    }
}

impl Edge {
    fn coefficient(&self, at: &Vertex) -> Option<i64> {
        if &self.u == at {
            Some(self.cu)
        } else if &self.v == at {
            Some(self.cv)
        } else {
            None
        }
    }

    fn other(&self, at: &Vertex) -> Option<Vertex> {
        if &self.u == at {
            Some(self.v.clone())
        } else if &self.v == at {
            Some(self.u.clone())
        } else {
            None
        }
    }
}

#[derive(Default)]
pub(super) struct TwoLargePrimeGraph {
    edges: Vec<Edge>,
    adjacent: HashMap<Vertex, Vec<usize>>,
}

impl TwoLargePrimeGraph {
    /// Insert a partial. A tree edge is retained; a closing
    /// edge yields a factor-base relation only when its cycle is balanced.
    pub(super) fn push(
        &mut self,
        relation: HecRelation,
        off_base: &[(BigUint, i64)],
        n: &BigUint,
    ) -> Option<HecRelation> {
        let edge = match off_base {
            [(x, cx)] if *cx != 0 => Edge {
                u: Vertex::Place(x.to_u32_digits()),
                v: Vertex::Ground,
                cu: *cx,
                cv: 0,
                relation,
            },
            [(x, cx), (y, cy)] if x != y && *cx != 0 && *cy != 0 => Edge {
                u: Vertex::Place(x.to_u32_digits()),
                v: Vertex::Place(y.to_u32_digits()),
                cu: *cx,
                cv: *cy,
                relation,
            },
            _ => return None,
        };

        if let Some(path) = self.path(&edge.u, &edge.v) {
            return combine_cycle(&edge, &path, &self.edges, n);
        }
        let idx = self.edges.len();
        self.adjacent.entry(edge.u.clone()).or_default().push(idx);
        self.adjacent.entry(edge.v.clone()).or_default().push(idx);
        self.edges.push(edge);
        None
    }

    /// A tree path, returned as directed steps from `start` to `end`.
    fn path(&self, start: &Vertex, end: &Vertex) -> Option<Vec<(Vertex, usize)>> {
        let mut queue = VecDeque::from([start.clone()]);
        let mut seen = HashSet::from([start.clone()]);
        let mut parent: HashMap<Vertex, (Vertex, usize)> = HashMap::new();
        while let Some(at) = queue.pop_front() {
            if &at == end {
                break;
            }
            for &idx in self.adjacent.get(&at).into_iter().flatten() {
                let next = self.edges[idx].other(&at)?;
                if seen.insert(next.clone()) {
                    parent.insert(next.clone(), (at.clone(), idx));
                    queue.push_back(next);
                }
            }
        }
        if !seen.contains(end) {
            return None;
        }
        let mut reversed = Vec::new();
        let mut at = end.clone();
        while &at != start {
            let (prev, idx) = parent.get(&at)?.clone();
            reversed.push((prev.clone(), idx));
            at = prev;
        }
        reversed.reverse();
        Some(reversed)
    }
}

fn combine_cycle(
    closing: &Edge,
    path: &[(Vertex, usize)],
    tree: &[Edge],
    n: &BigUint,
) -> Option<HecRelation> {
    let mut relation = WideRelation::from(&closing.relation);
    let mut places = HashMap::new();
    places.insert(closing.u.clone(), BigInt::from(closing.cu));
    if closing.v != Vertex::Ground {
        places.insert(closing.v.clone(), BigInt::from(closing.cv));
    }
    if let Some(ground_step) = path.iter().position(|(at, _)| *at == Vertex::Ground) {
        // The tree path passes through ground. Eliminate each endpoint
        // toward ground independently; a ground vertex has no logarithm.
        for (at, idx) in &path[..ground_step] {
            eliminate_at(&mut relation, &mut places, at, &tree[*idx], n)?;
        }
        for (at, idx) in path[ground_step..].iter().rev() {
            let reverse_at = tree[*idx].other(at)?;
            eliminate_at(&mut relation, &mut places, &reverse_at, &tree[*idx], n)?;
        }
    } else {
        for (at, idx) in path {
            eliminate_at(&mut relation, &mut places, at, &tree[*idx], n)?;
        }
    }
    if places.values().any(|value| !value.is_zero()) {
        return None;
    }
    if relation.coef_a.is_zero() && relation.coef_b.is_zero() && relation.entries.is_empty() {
        return None;
    }
    relation.finish()
}

fn eliminate_at(
    relation: &mut WideRelation,
    places: &mut HashMap<Vertex, BigInt>,
    at: &Vertex,
    edge: &Edge,
    n: &BigUint,
) -> Option<()> {
    let current = places.remove(at)?;
    let coefficient = BigInt::from(edge.coefficient(at)?);
    let divisor = current.abs().gcd(&coefficient.abs());
    let scale_relation = coefficient.abs() / &divisor;
    let scale_edge = -current * coefficient.signum() / &divisor;
    *relation = linear_combination(relation, &scale_relation, &edge.relation, &scale_edge, n)?;
    let mut updated: HashMap<Vertex, BigInt> = HashMap::new();
    for (vertex, value) in places.iter() {
        updated.insert(vertex.clone(), value * &scale_relation);
    }
    let other = edge.other(at)?;
    if other != Vertex::Ground {
        let next = BigInt::from(edge.coefficient(&other)?) * &scale_edge;
        *updated.entry(other).or_default() += next;
    }
    *places = updated
        .into_iter()
        .filter(|(_, value)| !value.is_zero())
        .collect();
    Some(())
}

fn linear_combination(
    a: &WideRelation,
    ka: &BigInt,
    b: &HecRelation,
    kb: &BigInt,
    n: &BigUint,
) -> Option<WideRelation> {
    let scale_mod = |v: &BigUint, k: &BigInt| -> BigUint {
        let t = (v * k.abs().to_biguint().expect("absolute scalar")) % n;
        if k.is_negative() {
            (n - t) % n
        } else {
            t
        }
    };
    let mut entries = HashMap::<usize, BigInt>::new();
    for (&index, value) in &a.entries {
        *entries.entry(index).or_default() += value * ka;
    }
    for &(index, value) in &b.entries {
        *entries.entry(index).or_default() += BigInt::from(value) * kb;
    }
    entries.retain(|_, value| !value.is_zero());
    Some(WideRelation {
        coef_a: (scale_mod(&a.coef_a, ka) + scale_mod(&b.coef_a, kb)) % n,
        coef_b: (scale_mod(&a.coef_b, ka) + scale_mod(&b.coef_b, kb)) % n,
        entries,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rel(a: u32, entry: usize) -> HecRelation {
        HecRelation {
            coef_a: BigUint::from(a),
            coef_b: BigUint::zero(),
            entries: vec![(entry, 1)],
        }
    }

    #[test]
    fn balanced_cycle_cancels_both_places() {
        let n = BigUint::from(101u32);
        let (p, q, r) = (
            BigUint::from(7u32),
            BigUint::from(11u32),
            BigUint::from(13u32),
        );
        let mut graph = TwoLargePrimeGraph::default();
        assert!(graph
            .push(rel(1, 0), &[(p.clone(), 1), (q.clone(), 1)], &n)
            .is_none());
        assert!(graph
            .push(rel(2, 1), &[(q.clone(), 1), (r.clone(), 1)], &n)
            .is_none());
        let full = graph
            .push(rel(3, 2), &[(p.clone(), 1), (r.clone(), -1)], &n)
            .unwrap();
        assert_eq!(full.coef_a, BigUint::from(4u32));
        assert_eq!(full.entries, vec![(0, -1), (1, 1), (2, 1)]);
        assert!(graph.push(rel(4, 3), &[(p, 1), (r, 1)], &n).is_none());
    }

    #[test]
    fn path_through_ground_and_single_place_pair() {
        let n = BigUint::from(101u32);
        let (p, q) = (BigUint::from(7u32), BigUint::from(11u32));
        let mut graph = TwoLargePrimeGraph::default();
        assert!(graph.push(rel(1, 0), &[(p.clone(), 1)], &n).is_none());
        assert!(graph.push(rel(2, 1), &[(q.clone(), -1)], &n).is_none());
        let full = graph
            .push(rel(3, 2), &[(p.clone(), 1), (q, 1)], &n)
            .unwrap();
        assert_eq!(full.coef_a, BigUint::from(4u32)); // 3 - 1 + 2
        assert_eq!(full.entries, vec![(0, -1), (1, 1), (2, 1)]);
        let full = graph.push(rel(4, 3), &[(p, -1)], &n).unwrap();
        assert_eq!(full.coef_a, BigUint::from(5u32));
        assert_eq!(full.entries, vec![(0, 1), (3, 1)]);
    }

    #[test]
    fn repeated_places_cancel_with_integer_cross_products() {
        let n = BigUint::from(101u32);
        let (p, q, r) = (
            BigUint::from(7u32),
            BigUint::from(11u32),
            BigUint::from(13u32),
        );
        // Hidden logs P=17, Q=23, R=31 and each factor-base column=1.
        // 2*(P+2R) - (2P+Q) + (Q-4R) eliminates both places.
        let mut graph = TwoLargePrimeGraph::default();
        assert!(graph
            .push(rel(58, 0), &[(p.clone(), 2), (q.clone(), 1)], &n)
            .is_none());
        assert!(graph
            .push(rel(1, 1), &[(q, 1), (r.clone(), -4)], &n)
            .is_none());
        let full = graph.push(rel(80, 2), &[(p, 1), (r, 2)], &n).unwrap();
        assert_eq!(full.coef_a, BigUint::from(2u32));
        assert_eq!(full.entries, vec![(0, -1), (1, 1), (2, 2)]);
    }

    #[test]
    fn wide_intermediate_can_cancel_before_checked_i64_output() {
        let n = BigUint::from(101u32);
        let p = BigUint::from(7u32);
        let large = HecRelation {
            coef_a: BigUint::from(1u32),
            coef_b: BigUint::zero(),
            entries: vec![(0, i64::MAX)],
        };
        let mut graph = TwoLargePrimeGraph::default();
        assert!(graph.push(large.clone(), &[(p.clone(), 2)], &n).is_none());
        let full = graph.push(large, &[(p, 1)], &n).unwrap();
        assert_eq!(full.coef_a, BigUint::from(1u32));
        assert_eq!(full.entries, vec![(0, i64::MAX)]);
    }

    #[test]
    fn intermediate_above_i64_can_cancel_to_exact_output() {
        let n = BigUint::from(101u32);
        let source = HecRelation {
            coef_a: BigUint::from(1u32),
            coef_b: BigUint::zero(),
            entries: vec![(0, i64::MAX)],
        };
        let doubled = linear_combination(
            &WideRelation::from(&source),
            &BigInt::from(2),
            &source,
            &BigInt::zero(),
            &n,
        )
        .unwrap();
        assert_eq!(doubled.entries[&0], BigInt::from(i64::MAX) * 2);
        assert!(doubled.clone().finish().is_none());
        let cancelled =
            linear_combination(&doubled, &BigInt::from(1), &source, &BigInt::from(-1), &n)
                .unwrap()
                .finish()
                .unwrap();
        assert_eq!(cancelled.coef_a, BigUint::from(1u32));
        assert_eq!(cancelled.entries, vec![(0, i64::MAX)]);
    }

    #[test]
    fn graph_cycle_retains_wide_intermediate_until_final_cancellation() {
        let n = BigUint::from(101u32);
        let p = BigUint::from(7u32);
        let q = BigUint::from(11u32);
        let r = BigUint::from(13u32);
        let relation = |coefficient: i64| HecRelation {
            coef_a: BigUint::from((coefficient as i128).rem_euclid(101) as u64),
            coef_b: BigUint::zero(),
            entries: vec![(0, coefficient)],
        };
        let mut graph = TwoLargePrimeGraph::default();
        assert!(graph
            .push(relation(-i64::MAX), &[(p.clone(), 1), (q.clone(), 1)], &n)
            .is_none());
        assert!(graph
            .push(relation(-i64::MAX), &[(q, 1), (r.clone(), 1)], &n)
            .is_none());
        let full = graph
            .push(relation(i64::MAX), &[(p, 2), (r, -2)], &n)
            .unwrap();
        assert_eq!(full.entries, vec![(0, i64::MAX)]);
        assert_eq!(full.coef_a, BigUint::from((i64::MAX as i128 % 101) as u64));
    }

    #[test]
    fn every_emitted_small_cycle_preserves_a_symbolic_log_identity() {
        let n = BigUint::from(101u32);
        let places = [
            BigUint::from(7u32),
            BigUint::from(11u32),
            BigUint::from(13u32),
        ];
        let hidden_logs = [17i64, 23, 31];
        let signs = [-1i64, 1];
        let mut emitted = 0;
        for &a in &signs {
            for &b in &signs {
                for &c in &signs {
                    for &d in &signs {
                        for &e in &signs {
                            for &f in &signs {
                                let mut graph = TwoLargePrimeGraph::default();
                                let edges = [
                                    ([(0, a), (1, b)], 0usize),
                                    ([(1, c), (2, d)], 1),
                                    ([(0, e), (2, f)], 2),
                                ];
                                for (endpoints, idx) in edges {
                                    let value = 1 + endpoints
                                        .iter()
                                        .map(|&(j, s)| s * hidden_logs[j])
                                        .sum::<i64>();
                                    let relation = rel(value.rem_euclid(101) as u32, idx);
                                    let off_base: Vec<_> = endpoints
                                        .iter()
                                        .map(|&(j, s)| (places[j].clone(), s))
                                        .collect();
                                    if let Some(full) = graph.push(relation, &off_base, &n) {
                                        let lhs = full
                                            .coef_a
                                            .to_u32_digits()
                                            .first()
                                            .copied()
                                            .unwrap_or(0);
                                        let rhs = full
                                            .entries
                                            .iter()
                                            .map(|&(_, v)| v)
                                            .sum::<i64>()
                                            .rem_euclid(101)
                                            as u32;
                                        assert_eq!(lhs, rhs, "signs: {a} {b} {c} {d} {e} {f}");
                                        emitted += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        assert_eq!(emitted, 32);
    }
}
