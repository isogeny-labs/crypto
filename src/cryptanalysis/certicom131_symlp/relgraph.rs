//! Double-large-prime relation graph over `Z/r` for any width of `r`.
//!
//! [`large_prime_filter`](crate::cryptanalysis::large_prime_filter) and the
//! Frobenius collector in `ic_framework::large_prime` keep coefficients in
//! a `u64`; the 131-bit Certicom subgroups have `r ≈ 2^130`.  This graph
//! keeps every coefficient as a `BigUint` reduced modulo `r`.
//!
//! A relation is `[a]G + [b]Q = Σ s_i·F_i + Σ t_k·L_k` with factor-base
//! elements `F_i`, at most two large primes `L_k`, and signs `s, t = ±1`.
//! Following Gaudry–Thomé–Thériault–Diem, a relation with large primes is
//! an edge of a graph whose vertices are the large primes plus one special
//! vertex `ROOT`: one large prime `L` gives the edge `ROOT — L`, two give
//! `L_1 — L_2`.  A relation with none is full and passes straight through.
//!
//! A spanning forest is kept with union–find.  An edge that joins two
//! trees extends the forest; an edge inside one tree closes a cycle.  Along
//! a cycle every large-prime vertex sits between exactly two edges, so a
//! linear combination with coefficients `c_k` chosen vertex by vertex
//! (`c_{k+1} = −c_k · t_k(L) / t_{k+1}(L)`) cancels each of them.  A cycle
//! through `ROOT` is started and ended there, where nothing needs to
//! cancel, so it always yields a full relation.  A cycle of large primes
//! only must also cancel at its closing vertex; with `±1` signs that holds
//! for exactly one of the two sign parities, and the other leaves `±2c·L`
//! (counted as `odd_cycles`, not emitted).
//!
//! Every emitted relation carries the input relation ids and their
//! coefficients, and [`LpGraph::replay`] recomputes it from those inputs
//! alone, so a caller can check it independently, in the group or in the
//! scalars.

use std::collections::{HashMap, VecDeque};

use num_bigint::BigUint;
use num_traits::{One, Zero};
use serde::Serialize;

/// A large-prime identifier (an index into the large-prime domain).
pub type LpId = u64;

/// `[a]G + [b]Q = Σ s·F_i + Σ t·L_k`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Relation {
    pub a: BigUint,
    pub b: BigUint,
    pub fb: Vec<(u32, i8)>,
    pub lp: Vec<(LpId, i8)>,
}

/// A full relation: `[a]G + [b]Q = Σ c_i·F_i`, with the input relation
/// ids and coefficients it was built from.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FullRelation {
    pub a: BigUint,
    pub b: BigUint,
    /// Sorted by factor-base index, zero coefficients removed.
    pub fb: Vec<(u32, BigUint)>,
    pub inputs: Vec<(usize, BigUint)>,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct GraphStats {
    pub relations_in: u64,
    pub full_direct: u64,
    pub one_large: u64,
    pub two_large: u64,
    /// Rejected: more than two large primes, or a sign other than ±1.
    pub rejected: u64,
    pub vertices: u64,
    pub forest_edges: u64,
    pub cycles: u64,
    pub full_from_cycles: u64,
    pub odd_cycles: u64,
    pub longest_cycle: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Vertex {
    Root,
    Lp(LpId),
}

pub struct LpGraph {
    r: BigUint,
    rels: Vec<Relation>,
    /// Vertex → dense id.
    ids: HashMap<Vertex, usize>,
    verts: Vec<Vertex>,
    parent: Vec<usize>,
    rank: Vec<u8>,
    /// Forest adjacency: (neighbour, relation id).
    tree: Vec<Vec<(usize, usize)>>,
    pub stats: GraphStats,
}

fn signed(s: i8, r: &BigUint) -> BigUint {
    if s >= 0 {
        BigUint::one()
    } else {
        r - BigUint::one()
    }
}

/// `x^{-1} mod r` for prime `r` (Fermat).
fn inv_mod(x: &BigUint, r: &BigUint) -> BigUint {
    x.modpow(&(r - BigUint::from(2u8)), r)
}

impl LpGraph {
    /// `r` must be prime (coefficients are divided by large-prime signs,
    /// which are units for any `r > 2`; primality is needed by the caller's
    /// linear algebra, not here).
    pub fn new(r: BigUint) -> Self {
        assert!(r > BigUint::from(2u8), "modulus must exceed 2");
        Self {
            r,
            rels: Vec::new(),
            ids: HashMap::new(),
            verts: Vec::new(),
            parent: Vec::new(),
            rank: Vec::new(),
            tree: Vec::new(),
            stats: GraphStats::default(),
        }
    }

    pub fn modulus(&self) -> &BigUint {
        &self.r
    }

    pub fn relation(&self, id: usize) -> &Relation {
        &self.rels[id]
    }

    fn vertex(&mut self, v: Vertex) -> usize {
        if let Some(&i) = self.ids.get(&v) {
            return i;
        }
        let i = self.verts.len();
        self.ids.insert(v, i);
        self.verts.push(v);
        self.parent.push(i);
        self.rank.push(0);
        self.tree.push(Vec::new());
        self.stats.vertices += 1;
        i
    }

    fn find(&mut self, mut x: usize) -> usize {
        while self.parent[x] != x {
            self.parent[x] = self.parent[self.parent[x]];
            x = self.parent[x];
        }
        x
    }

    fn union(&mut self, a: usize, b: usize) {
        let (ra, rb) = (self.find(a), self.find(b));
        if ra == rb {
            return;
        }
        match self.rank[ra].cmp(&self.rank[rb]) {
            std::cmp::Ordering::Less => self.parent[ra] = rb,
            std::cmp::Ordering::Greater => self.parent[rb] = ra,
            std::cmp::Ordering::Equal => {
                self.parent[rb] = ra;
                self.rank[ra] += 1;
            }
        }
    }

    /// Forest path from `from` to `to` as (vertex sequence, relation ids):
    /// `verts[0] = from`, `verts[k] = to`, edge `k` joins `verts[k−1]` and
    /// `verts[k]`.
    fn tree_path(&self, from: usize, to: usize) -> (Vec<usize>, Vec<usize>) {
        let mut prev: HashMap<usize, (usize, usize)> = HashMap::new();
        let mut queue = VecDeque::from([from]);
        prev.insert(from, (usize::MAX, usize::MAX));
        while let Some(v) = queue.pop_front() {
            if v == to {
                break;
            }
            for &(w, e) in &self.tree[v] {
                if let std::collections::hash_map::Entry::Vacant(slot) = prev.entry(w) {
                    slot.insert((v, e));
                    queue.push_back(w);
                }
            }
        }
        let mut verts = vec![to];
        let mut edges = Vec::new();
        let mut cur = to;
        while cur != from {
            let (p, e) = prev[&cur];
            edges.push(e);
            verts.push(p);
            cur = p;
        }
        verts.reverse();
        edges.reverse();
        (verts, edges)
    }

    /// The signed coefficient of large prime `l` in relation `id`.
    fn lp_coef(&self, id: usize, l: LpId) -> BigUint {
        let mut c = BigUint::zero();
        for &(k, t) in &self.rels[id].lp {
            if k == l {
                c = (c + signed(t, &self.r)) % &self.r;
            }
        }
        c
    }

    /// Add one relation; returns the full relations it completes (zero or
    /// one).
    pub fn add(&mut self, rel: Relation) -> Option<FullRelation> {
        self.stats.relations_in += 1;
        if rel.lp.len() > 2
            || rel.fb.iter().any(|&(_, s)| s != 1 && s != -1)
            || rel.lp.iter().any(|&(_, t)| t != 1 && t != -1)
        {
            self.stats.rejected += 1;
            return None;
        }
        let id = self.rels.len();
        self.rels.push(rel);
        let rel = &self.rels[id];
        match rel.lp.len() {
            0 => {
                self.stats.full_direct += 1;
                return Some(self.combine(&[(id, BigUint::one())]));
            }
            1 => self.stats.one_large += 1,
            _ => self.stats.two_large += 1,
        }
        let (u, v) = match rel.lp.as_slice() {
            [(l, _)] => (Vertex::Root, Vertex::Lp(*l)),
            [(l1, _), (l2, _)] => (Vertex::Lp(*l1), Vertex::Lp(*l2)),
            _ => unreachable!(),
        };
        let (u, v) = (self.vertex(u), self.vertex(v));
        if u == v {
            // L − L: the two terms cancel only with opposite signs.
            self.stats.cycles += 1;
            self.stats.longest_cycle = self.stats.longest_cycle.max(1);
            let Vertex::Lp(l) = self.verts[u] else {
                unreachable!()
            };
            if self.lp_coef(id, l).is_zero() {
                self.stats.full_from_cycles += 1;
                return Some(self.combine(&[(id, BigUint::one())]));
            }
            self.stats.odd_cycles += 1;
            return None;
        }
        if self.find(u) != self.find(v) {
            self.union(u, v);
            self.tree[u].push((v, id));
            self.tree[v].push((u, id));
            self.stats.forest_edges += 1;
            return None;
        }
        // A cycle: forest path v → u, closed by the new edge u — v.
        self.stats.cycles += 1;
        let (mut verts, mut edges) = self.tree_path(v, u);
        // Cycle as a closed walk: verts[0] = v … verts[k] = u, edges[0..k],
        // then the new edge back to v.  Shared vertex between edge j and
        // edge j+1 (cyclically) is verts[j+1]; between the new edge and
        // edges[0] it is verts[0].
        edges.push(id);
        // Shared vertices in cycle order: verts[1], …, verts[k], verts[0].
        verts.rotate_left(1);
        let len = edges.len();
        self.stats.longest_cycle = self.stats.longest_cycle.max(len as u64);
        // Rotate so that, if ROOT is on the cycle, it is the closing vertex
        // (the shared vertex between the last and the first edge).
        if let Some(pos) = verts.iter().position(|&w| self.verts[w] == Vertex::Root) {
            // Closing vertex is verts[len−1]; move ROOT there.
            let shift = (pos + 1) % len;
            verts.rotate_left(shift);
            edges.rotate_left(shift);
        }
        // Propagate coefficients across the shared vertices verts[0..len−1].
        let mut coefs = vec![BigUint::one()];
        for j in 0..len - 1 {
            let Vertex::Lp(l) = self.verts[verts[j]] else {
                unreachable!("ROOT is only the closing vertex")
            };
            let tj = self.lp_coef(edges[j], l);
            let tn = self.lp_coef(edges[j + 1], l);
            let next =
                (&self.r - (&coefs[j] * &tj % &self.r) * inv_mod(&tn, &self.r) % &self.r) % &self.r;
            coefs.push(next);
        }
        let inputs: Vec<(usize, BigUint)> = edges.iter().copied().zip(coefs).collect();
        let full = self.combine(&inputs);
        if self.lp_residual(&inputs).is_empty() {
            self.stats.full_from_cycles += 1;
            Some(full)
        } else {
            self.stats.odd_cycles += 1;
            None
        }
    }

    /// Σ c·(relation), reduced mod `r`, ignoring large primes.
    fn combine(&self, inputs: &[(usize, BigUint)]) -> FullRelation {
        let r = &self.r;
        let mut a = BigUint::zero();
        let mut b = BigUint::zero();
        let mut fb: HashMap<u32, BigUint> = HashMap::new();
        for (id, c) in inputs {
            let rel = &self.rels[*id];
            a = (a + c * &rel.a) % r;
            b = (b + c * &rel.b) % r;
            for &(i, s) in &rel.fb {
                let e = fb.entry(i).or_insert_with(BigUint::zero);
                *e = (&*e + c * signed(s, r)) % r;
            }
        }
        let mut fb: Vec<(u32, BigUint)> = fb.into_iter().filter(|(_, c)| !c.is_zero()).collect();
        fb.sort_by_key(|(i, _)| *i);
        FullRelation {
            a,
            b,
            fb,
            inputs: inputs.to_vec(),
        }
    }

    /// The large primes left with a nonzero coefficient by `inputs`.
    fn lp_residual(&self, inputs: &[(usize, BigUint)]) -> Vec<(LpId, BigUint)> {
        let r = &self.r;
        let mut acc: HashMap<LpId, BigUint> = HashMap::new();
        for (id, c) in inputs {
            for &(l, t) in &self.rels[*id].lp {
                let e = acc.entry(l).or_insert_with(BigUint::zero);
                *e = (&*e + c * signed(t, r)) % r;
            }
        }
        acc.into_iter().filter(|(_, c)| !c.is_zero()).collect()
    }

    /// Recompute `full` from its inputs alone: true when the inputs cancel
    /// every large prime and reproduce `a`, `b` and the factor-base
    /// coefficients exactly.
    pub fn replay(&self, full: &FullRelation) -> bool {
        if full.inputs.iter().any(|(id, _)| *id >= self.rels.len()) {
            return false;
        }
        self.lp_residual(&full.inputs).is_empty() && self.combine(&full.inputs) == *full
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_bigint::RandBigInt;
    use rand::{rngs::StdRng, Rng, SeedableRng};

    /// The ECC2-131 and ECCp-131 subgroup orders.
    fn orders() -> Vec<BigUint> {
        vec![
            BigUint::parse_bytes(b"0400000000000000026ABB991FE311FE83", 16).unwrap(),
            BigUint::parse_bytes(b"048E1D43F293469E317F7ED728F6B8E6F1", 16).unwrap(),
        ]
    }

    /// Synthetic world with known logarithms: factor-base logs `f`, large-
    /// prime logs `l`, target log `d`.  A relation is generated by picking
    /// its terms and `b`, then `a = Σ s·log − b·d`, so it holds in any
    /// group of order `r` where `Q = [d]G`.
    struct World {
        r: BigUint,
        f: Vec<BigUint>,
        l: Vec<BigUint>,
        d: BigUint,
    }

    impl World {
        fn new(r: BigUint, nf: usize, nl: usize, rng: &mut StdRng) -> Self {
            let f = (0..nf).map(|_| rng.gen_biguint_below(&r)).collect();
            let l = (0..nl).map(|_| rng.gen_biguint_below(&r)).collect();
            let d = rng.gen_biguint_below(&r);
            Self { r, f, l, d }
        }

        fn relation(&self, fb: Vec<(u32, i8)>, lp: Vec<(LpId, i8)>, rng: &mut StdRng) -> Relation {
            let r = &self.r;
            let mut sum = BigUint::zero();
            for &(i, s) in &fb {
                sum = (sum + signed(s, r) * &self.f[i as usize]) % r;
            }
            for &(k, t) in &lp {
                sum = (sum + signed(t, r) * &self.l[k as usize]) % r;
            }
            let b = rng.gen_biguint_below(r);
            let a = (sum + r - (&b * &self.d) % r) % r;
            Relation { a, b, fb, lp }
        }

        /// `a + b·d ≡ Σ c_i·f_i (mod r)`.
        fn holds(&self, full: &FullRelation) -> bool {
            let r = &self.r;
            let lhs = (&full.a + &full.b * &self.d) % r;
            let rhs = full.fb.iter().fold(BigUint::zero(), |acc, (i, c)| {
                (acc + c * &self.f[*i as usize]) % r
            });
            lhs == rhs
        }
    }

    fn sign(rng: &mut StdRng) -> i8 {
        if rng.gen() {
            1
        } else {
            -1
        }
    }

    #[test]
    fn a_one_large_prime_pair_closes_through_root() {
        for r in orders() {
            let mut rng = StdRng::seed_from_u64(1);
            let w = World::new(r.clone(), 4, 4, &mut rng);
            let mut g = LpGraph::new(r);
            let e1 = w.relation(vec![(0, 1)], vec![(2, 1)], &mut rng);
            let e2 = w.relation(vec![(1, -1)], vec![(2, -1)], &mut rng);
            assert!(g.add(e1).is_none());
            let full = g.add(e2).expect("two partials sharing L close a cycle");
            assert!(g.replay(&full));
            assert!(w.holds(&full));
            assert_eq!(g.stats.full_from_cycles, 1);
        }
    }

    #[test]
    fn two_large_prime_chains_and_cycles_yield_valid_full_relations() {
        for r in orders() {
            let mut rng = StdRng::seed_from_u64(2);
            let (nf, nl) = (12, 60);
            let w = World::new(r.clone(), nf, nl, &mut rng);
            let mut g = LpGraph::new(r);
            let mut fulls = Vec::new();
            for _ in 0..400 {
                let fb: Vec<(u32, i8)> = (0..2)
                    .map(|_| (rng.gen_range(0..nf as u32), sign(&mut rng)))
                    .collect();
                let nlp = rng.gen_range(0..=2);
                let lp: Vec<(LpId, i8)> = (0..nlp)
                    .map(|_| (rng.gen_range(0..nl as u64), sign(&mut rng)))
                    .collect();
                let rel = w.relation(fb, lp, &mut rng);
                if let Some(full) = g.add(rel) {
                    fulls.push(full);
                }
            }
            assert!(g.stats.full_from_cycles > 20, "{:?}", g.stats);
            assert!(g.stats.two_large > 0 && g.stats.longest_cycle >= 3);
            for full in &fulls {
                assert!(g.replay(full), "replay");
                assert!(w.holds(full), "full relation is false in the scalars");
            }
            // Bookkeeping: every input is counted once.
            let s = &g.stats;
            assert_eq!(
                s.relations_in,
                s.full_direct + s.one_large + s.two_large + s.rejected
            );
            assert_eq!(s.cycles, s.full_from_cycles + s.odd_cycles);
        }
    }

    #[test]
    fn an_odd_large_prime_cycle_is_not_emitted() {
        let r = orders().remove(0);
        let mut rng = StdRng::seed_from_u64(3);
        let w = World::new(r.clone(), 2, 3, &mut rng);
        let mut g = LpGraph::new(r);
        // L0 + L1, L1 + L2, L2 + L0 with all signs +: any ±1 combination
        // cancelling L0 and L1 leaves 2·L2.
        assert!(g
            .add(w.relation(vec![(0, 1)], vec![(0, 1), (1, 1)], &mut rng))
            .is_none());
        assert!(g
            .add(w.relation(vec![(1, 1)], vec![(1, 1), (2, 1)], &mut rng))
            .is_none());
        assert!(g
            .add(w.relation(vec![(0, -1)], vec![(2, 1), (0, 1)], &mut rng))
            .is_none());
        assert_eq!(g.stats.odd_cycles, 1);
        // The same triangle with one sign flipped closes.
        let mut g = LpGraph::new(w.r.clone());
        g.add(w.relation(vec![(0, 1)], vec![(0, 1), (1, 1)], &mut rng));
        g.add(w.relation(vec![(1, 1)], vec![(1, 1), (2, 1)], &mut rng));
        let full = g
            .add(w.relation(vec![(0, -1)], vec![(2, 1), (0, -1)], &mut rng))
            .expect("even cycle");
        assert!(g.replay(&full) && w.holds(&full));
    }

    #[test]
    fn a_tampered_full_relation_fails_replay() {
        let r = orders().remove(1);
        let mut rng = StdRng::seed_from_u64(4);
        let w = World::new(r.clone(), 3, 3, &mut rng);
        let mut g = LpGraph::new(r);
        g.add(w.relation(vec![(0, 1)], vec![(1, 1)], &mut rng));
        let mut full = g
            .add(w.relation(vec![(2, 1)], vec![(1, 1)], &mut rng))
            .unwrap();
        assert!(g.replay(&full));
        full.a += 1u8;
        assert!(!g.replay(&full));
    }
}
