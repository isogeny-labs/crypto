//! Spanning-forest elimination for hyperelliptic relations with at most two
//! off-base rational places. Only unit place coefficients are admitted: this
//! keeps all factor-base coefficients integral and checked in `i64`.

use std::collections::{HashMap, HashSet, VecDeque};

use num_bigint::BigUint;
use num_traits::Zero;

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
    /// Insert a unit-coefficient partial. A tree edge is retained; a closing
    /// edge yields a factor-base relation only when its cycle is balanced.
    pub(super) fn push(
        &mut self,
        relation: HecRelation,
        off_base: &[(BigUint, i64)],
        n: &BigUint,
    ) -> Option<HecRelation> {
        let edge = match off_base {
            [(x, cx)] if cx.unsigned_abs() == 1 => Edge {
                u: Vertex::Place(x.to_u32_digits()),
                v: Vertex::Ground,
                cu: *cx,
                cv: 0,
                relation,
            },
            [(x, cx), (y, cy)] if x != y && cx.unsigned_abs() == 1 && cy.unsigned_abs() == 1 => {
                Edge {
                    u: Vertex::Place(x.to_u32_digits()),
                    v: Vertex::Place(y.to_u32_digits()),
                    cu: *cx,
                    cv: *cy,
                    relation,
                }
            }
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
    let mut relation = closing.relation.clone();
    let mut places = HashMap::new();
    places.insert(closing.u.clone(), closing.cu);
    if closing.v != Vertex::Ground {
        places.insert(closing.v.clone(), closing.cv);
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
    if places.values().any(|&v| v != 0) {
        return None;
    }
    if relation.coef_a.is_zero() && relation.coef_b.is_zero() && relation.entries.is_empty() {
        return None;
    }
    Some(relation)
}

fn eliminate_at(
    relation: &mut HecRelation,
    places: &mut HashMap<Vertex, i64>,
    at: &Vertex,
    edge: &Edge,
    n: &BigUint,
) -> Option<()> {
    let current = places.remove(at)?;
    let coefficient = edge.coefficient(at)?;
    let scale = -current * coefficient; // both coefficients are ±1
    *relation = add_scaled(relation, &edge.relation, scale, n)?;
    let other = edge.other(at)?;
    if other != Vertex::Ground {
        let next = edge.coefficient(&other)? * scale;
        let value = places.get(&other).copied().unwrap_or(0) + next;
        if value == 0 {
            places.remove(&other);
        } else {
            places.insert(other, value);
        }
    }
    Some(())
}

fn add_scaled(a: &HecRelation, b: &HecRelation, scale: i64, n: &BigUint) -> Option<HecRelation> {
    let modular = |x: &BigUint, y: &BigUint| -> BigUint {
        if scale == 1 {
            (x + y) % n
        } else {
            (x + n - (y % n)) % n
        }
    };
    let mut entries: HashMap<usize, i64> = a.entries.iter().copied().collect();
    for &(idx, value) in &b.entries {
        let signed = value.checked_mul(scale)?;
        let next = entries
            .get(&idx)
            .copied()
            .unwrap_or(0)
            .checked_add(signed)?;
        if next == 0 {
            entries.remove(&idx);
        } else {
            entries.insert(idx, next);
        }
    }
    let mut entries: Vec<_> = entries.into_iter().collect();
    entries.sort_unstable_by_key(|&(idx, _)| idx);
    Some(HecRelation {
        coef_a: modular(&a.coef_a, &b.coef_a),
        coef_b: modular(&a.coef_b, &b.coef_b),
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
}
