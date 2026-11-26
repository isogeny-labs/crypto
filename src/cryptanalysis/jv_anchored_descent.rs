//! Graph-anchored multi-pattern quadratic descent and cold per-instance F4 trace replay
//! for extension-field genus-3 cover index calculus (`E(F_{p^6}) -> Jac(C / F_{p^2})`).
//!
//! Standard Nagao descent (`jv_cover::nagao_decompose`) accepts a solution of the
//! Weil-restricted system only when the degree-6 quotient polynomial `S(x) in F_p[x]`
//! splits into six distinct linear factors `(1^6)` (asymptotic share `1/720`).
//! When Phase-1 relation collection has already grounded a subset `A` of canonical
//! `F_p`-irreducible degree-2 divisor classes (`AnchorRecord` with `scale in {1, 2}`),
//! any squarefree sextic `S(x)` whose `F_p`-factorization pattern lies in
//! `{(1^6), (1^4, 2), (1^2, 2^2), (2^3)}` and whose quadratic Mumford factors all
//! hit `A` eliminates into an exact integer combination of factor-base elements:
//!
//! `[scale] * R = sum_k d_k * B_k` in `Jac(C / F_{p^2})`, `scale in {1, 2}`.
//!
//! Because the subgroup order `ell` is an odd prime (`gcd(2, ell) = 1`), both unit
//! anchors (`scale = 1`) and frustrated odd-cycle half-anchors (`scale = 2`) yield
//! exact modular rows without division during witness expansion.

use std::collections::{BTreeMap, HashMap, VecDeque};
use std::time::Instant;

use rand::rngs::StdRng;
use rand::Rng;
use serde::{Deserialize, Serialize};

use super::f4_fp::{self, F4Options, F4Trace};
use super::gaudry_cubic::{PolyRing, UPoly};
use super::jv_bitops::{self as work};
use super::jv_cover::{
    self as cover, peval, verify_dec, BaseEl, Ctx, Dec6, Div, Fld, Fq, Hyp, LinMode, NagaoCost,
    ZeroDimStats, E2,
};
use super::jv_sieve::{fdeg, FpPoly, FpRing};

/// Exact census of the admissible degree-2 `F_p`-irreducible divisor universe on `C / F_{p^2}`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct UniverseCensus {
    pub p: u64,
    pub irreducible_quadratics: u64,
    pub unramified_unsigned_classes: u64,
    pub ramified_unsigned_classes: u64,
    pub total_unsigned_classes: u64,
}

/// Exact `O(p^2)` census of unsigned canonical degree-2 `F_p`-irreducible Mumford classes
/// `[U(x), ±V(x)]` on `y^2 = H(x)` over `F_{p^2} = F_p[sqrt(w)]`.
///
/// Every monic irreducible `U(x) in F_p[x]` of degree 2 corresponds uniquely to a
/// Frobenius-conjugate root pair `{a + b*sqrt(w), a - b*sqrt(w)}` with `0 <= a < p` and
/// `1 <= b <= (p - 1) / 2`. An element `z in F_{p^2}^*` is a square in `F_{p^2}` iff its
/// norm `N_{F_{p^2}/F_p}(z) = z_0^2 - w * z_1^2` is a quadratic residue in `F_p^*`.
pub fn count_quadratic_universe(ctx: &Ctx) -> UniverseCensus {
    let f = &ctx.f.f;
    let p = f.p;
    let w = f.w;
    let hx = &ctx.cov.hx;

    let mut is_qr = vec![false; p as usize];
    for x in 1..p {
        is_qr[((x as u128 * x as u128) % p as u128) as usize] = true;
    }

    let fp2_sqrts = |z: E2| -> u64 {
        if z == E2::ZERO {
            return 1;
        }
        let z0_sq = (z.0[0] as u128 * z.0[0] as u128) % p as u128;
        let z1_sq = (z.0[1] as u128 * z.0[1] as u128) % p as u128;
        let wz1 = (w as u128 * z1_sq) % p as u128;
        let norm = ((z0_sq + p as u128 - wz1) % p as u128) as usize;
        if is_qr[norm] {
            2
        } else {
            0
        }
    };

    let half = (p - 1) / 2;
    let mut unramified = 0u64;
    let mut ramified = 0u64;
    for a in 0..p {
        for b in 1..=half {
            let alpha = E2([a, b]);
            let conj = E2([a, p - b]);
            let r1 = fp2_sqrts(peval(f, hx, &alpha));
            if r1 == 0 {
                continue;
            }
            let r2 = fp2_sqrts(peval(f, hx, &conj));
            if r2 == 0 {
                continue;
            }
            if r1 == 2 && r2 == 2 {
                unramified += 2;
            } else {
                ramified += 1;
            }
        }
    }

    UniverseCensus {
        p,
        irreducible_quadratics: p * (p - 1) / 2,
        unramified_unsigned_classes: unramified,
        ramified_unsigned_classes: ramified,
        total_unsigned_classes: unramified + ramified,
    }
}

/// Canonical unsigned `[u0, u1, v00, v01, v10, v11]` representation matching
/// `jv_principal_relation::Remainder` (`neg < key` flips sign to `-1`).
pub fn canonical_quadratic_class(f: &Fq, u_fp: &[u64; 3], v: &[E2]) -> ([u64; 6], i8) {
    debug_assert_eq!(u_fp[2], 1);
    let a = v.first().copied().unwrap_or(E2::ZERO);
    let b = v.get(1).copied().unwrap_or(E2::ZERO);
    let (na, nb) = (f.neg(&a), f.neg(&b));
    let key = [u_fp[0], u_fp[1], a.0[0], a.0[1], b.0[0], b.0[1]];
    let neg = [u_fp[0], u_fp[1], na.0[0], na.0[1], nb.0[0], nb.0[1]];
    work::words(18);
    if neg < key {
        (neg, -1)
    } else {
        (key, 1)
    }
}

pub fn class_to_divisor(key: &[u64; 6]) -> Div<E2> {
    let mut v = vec![E2([key[2], key[3]]), E2([key[4], key[5]])];
    while v.last() == Some(&E2::ZERO) {
        v.pop();
    }
    Div {
        u: vec![E2([key[0], 0]), E2([key[1], 0]), E2::ONE],
        v,
    }
}

/// Grounded degree-2 `F_p`-irreducible divisor witness.
///
/// Invariant: summing `weight * Source_{source_id}` over `(source_id, weight)` in `witness`
/// yields `small_part + (scale as i64) * D(class) == 0` in `Jac(C)`.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct AnchorRecord {
    pub class: [u64; 6],
    pub witness: Vec<(usize, i16)>,
    pub scale: u8,
}

#[derive(Clone, Debug, Default)]
pub struct AnchorTable {
    records: HashMap<[u64; 6], AnchorRecord>,
    pub unit_anchors: usize,
    pub half_anchors: usize,
    pub max_witness_terms: usize,
}

impl AnchorTable {
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            records: HashMap::with_capacity(cap),
            unit_anchors: 0,
            half_anchors: 0,
            max_witness_terms: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    pub fn contains(&self, class: &[u64; 6]) -> bool {
        self.records.contains_key(class)
    }

    pub fn insert(&mut self, record: AnchorRecord) -> bool {
        if self.records.contains_key(&record.class) {
            return false;
        }
        debug_assert!(matches!(record.scale, 1 | 2));
        if record.scale == 1 {
            self.unit_anchors += 1;
        } else {
            self.half_anchors += 1;
        }
        self.max_witness_terms = self.max_witness_terms.max(record.witness.len());
        self.records.insert(record.class, record);
        true
    }

    pub fn insert_or_tighten(&mut self, record: AnchorRecord) -> bool {
        debug_assert!(matches!(record.scale, 1 | 2));
        if let Some(existing) = self.records.get_mut(&record.class) {
            let better = record.scale < existing.scale
                || (record.scale == existing.scale
                    && record.witness.len() < existing.witness.len());
            if better {
                if existing.scale == 2 && record.scale == 1 {
                    self.half_anchors = self.half_anchors.saturating_sub(1);
                    self.unit_anchors += 1;
                }
                self.max_witness_terms = self.max_witness_terms.max(record.witness.len());
                *existing = record;
                return true;
            }
            return false;
        }
        self.insert(record)
    }

    #[inline]
    pub fn get(&self, class: &[u64; 6]) -> Option<&AnchorRecord> {
        self.records.get(class)
    }

    /// Counted hash lookup (`jv_bitops::probe(6)`).
    #[inline]
    pub fn get_counted(&self, class: &[u64; 6]) -> Option<&AnchorRecord> {
        work::probe(6);
        let found = self.records.get(class);
        if let Some(rec) = found {
            work::words((2 * rec.witness.len()) as u64);
        }
        found
    }

    pub fn iter(&self) -> impl Iterator<Item = (&[u64; 6], &AnchorRecord)> {
        self.records.iter()
    }
}

/// Summary of the linear-time unanchored odd-sign cycle harvester.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CycleHarvestStats {
    pub unanchored_edges: usize,
    pub unanchored_vertices: usize,
    pub components: usize,
    pub frustrated_components: usize,
    pub balanced_cycles: usize,
    pub odd_cycles: usize,
    pub harvested_half_anchors: usize,
    pub witness_cap_refusals: usize,
}

/// Signed unanchored two-class source `(source_id, class_u, sign_u, class_v, sign_v)`
/// representing `small(source_id) + sign_u * D(class_u) + sign_v * D(class_v) = 0`.
pub type UnanchoredEdge = (usize, [u64; 6], i8, [u64; 6], i8);

fn combine_i16(parts: &[(i16, &[(usize, i16)])]) -> Option<Vec<(usize, i16)>> {
    let mut acc = BTreeMap::<usize, i32>::new();
    for &(scale, terms) in parts {
        if scale == 0 {
            continue;
        }
        for &(src, w) in terms {
            let entry = acc.entry(src).or_default();
            *entry = entry.checked_add(scale as i32 * w as i32)?;
            work::words(4);
        }
    }
    let mut out = Vec::with_capacity(acc.len());
    for (src, v) in acc {
        if v != 0 {
            out.push((src, i16::try_from(v).ok()?));
        }
    }
    Some(out)
}

/// Harvest exact `scale = 2` (`+2 * D(v)`) half-anchors from frustrated (odd-sign) cycles
/// inside the unanchored 2-LP subgraph in `O(|V_un| + |E_un|)` time.
pub fn harvest_frustrated_cycle_anchors(
    edges: &[UnanchoredEdge],
    table: &mut AnchorTable,
    max_witness_terms: usize,
) -> CycleHarvestStats {
    let mut stats = CycleHarvestStats::default();
    let mut class_of: Vec<[u64; 6]> = Vec::new();
    let mut id_of: HashMap<[u64; 6], usize> = HashMap::new();
    let mut adj: Vec<Vec<(usize, usize, i8, i8, usize)>> = Vec::new();
    let mut kept_edges: Vec<(usize, usize, usize, i8, i8)> = Vec::new();

    for &(src, cu, su, cv, sv) in edges {
        work::probe(12);
        if table.contains(&cu) || table.contains(&cv) || cu == cv {
            continue;
        }
        let u = *id_of.entry(cu).or_insert_with(|| {
            let id = class_of.len();
            class_of.push(cu);
            adj.push(Vec::new());
            id
        });
        let v = *id_of.entry(cv).or_insert_with(|| {
            let id = class_of.len();
            class_of.push(cv);
            adj.push(Vec::new());
            id
        });
        let eid = kept_edges.len();
        kept_edges.push((src, u, v, su, sv));
        adj[u].push((v, src, su, sv, eid));
        adj[v].push((u, src, sv, su, eid));
    }

    stats.unanchored_edges = kept_edges.len();
    stats.unanchored_vertices = class_of.len();
    if class_of.is_empty() {
        return stats;
    }

    let n = class_of.len();
    let mut comp_id = vec![usize::MAX; n];
    let mut tree_edge_used = vec![false; kept_edges.len()];
    let mut eps = vec![0i8; n];
    let mut parent: Vec<Option<(usize, usize, i8, i8)>> = vec![None; n];
    let mut queue = VecDeque::new();

    for start in 0..n {
        if comp_id[start] != usize::MAX {
            continue;
        }
        let cid = stats.components;
        stats.components += 1;
        let mut comp_nodes = Vec::new();
        comp_id[start] = cid;
        eps[start] = -1;
        queue.push_back(start);
        while let Some(p_node) = queue.pop_front() {
            comp_nodes.push(p_node);
            for &(nb, src, sp, snb, eid) in &adj[p_node] {
                work::words(3);
                if comp_id[nb] == usize::MAX {
                    comp_id[nb] = cid;
                    tree_edge_used[eid] = true;
                    // Source: small + sp * D(p_node) + snb * D(nb) = 0
                    // W_T(nb) = snb * Source - (snb * sp) * W_T(p_node)
                    // Large part: D(nb) - (snb * sp) * eps[p_node] * D(root)
                    eps[nb] = -(snb * sp) * eps[p_node];
                    parent[nb] = Some((p_node, src, sp, snb));
                    queue.push_back(nb);
                }
            }
        }

        // Check non-tree edges inside this component for an odd-sign cycle.
        let mut best_odd_edge: Option<(usize, usize, usize, i8, i8)> = None;
        for &u in &comp_nodes {
            for &(v, src, su, sv, eid) in &adj[u] {
                if u > v || tree_edge_used[eid] {
                    continue;
                }
                let lhs = su * eps[u];
                let rhs = sv * eps[v];
                if lhs == -rhs {
                    stats.balanced_cycles += 1;
                } else if lhs == rhs {
                    stats.odd_cycles += 1;
                    if best_odd_edge.is_none() {
                        best_odd_edge = Some((src, u, v, su, sv));
                    }
                }
            }
        }

        let Some((_, root_u, _, _, _)) = best_odd_edge else {
            continue;
        };
        stats.frustrated_components += 1;

        // Re-root BFS at `root_u` so the shortest odd cycle sits right at the root.
        let mut local_vis =
            HashMap::<usize, (i8, Vec<(usize, i16)>)>::with_capacity(comp_nodes.len());
        let mut local_used = HashMap::<usize, bool>::new();
        let mut rq = VecDeque::new();
        local_vis.insert(root_u, (-1i8, Vec::new()));
        rq.push_back(root_u);
        let mut best_root_cycle: Option<Vec<(usize, i16)>> = None;

        while let Some(p_node) = rq.pop_front() {
            let (ep, wp) = local_vis[&p_node].clone();
            for &(nb, src, sp, snb, eid) in &adj[p_node] {
                if local_used.contains_key(&eid) {
                    continue;
                }
                if let std::collections::hash_map::Entry::Vacant(e) = local_vis.entry(nb) {
                    local_used.insert(eid, true);
                    let edge_unit = [(src, snb as i16)];
                    let scale_parent = -(snb as i16 * sp as i16);
                    if let Some(wnb) = combine_i16(&[(1, &edge_unit), (scale_parent, &wp)]) {
                        if wnb.len() <= max_witness_terms {
                            let enb = -(snb * sp) * ep;
                            e.insert((enb, wnb));
                            rq.push_back(nb);
                        } else {
                            stats.witness_cap_refusals += 1;
                        }
                    }
                } else if p_node < nb {
                    let (enb, wnb) = &local_vis[&nb];
                    if sp * ep == snb * (*enb) {
                        // W_cyc = Source - sp * W_T(p_node) - snb * W_T(nb)
                        // Large part is -(sp * ep + snb * enb) * D(root_u) = -2*(sp*ep)*D(root_u).
                        let tau = -(sp * ep) as i16;
                        let edge_raw = [(src, 1i16)];
                        if let Some(w_two_root) = combine_i16(&[
                            (tau, &edge_raw),
                            (-tau * sp as i16, &wp),
                            (-tau * snb as i16, wnb),
                        ]) {
                            let replace = best_root_cycle
                                .as_ref()
                                .is_none_or(|cur| w_two_root.len() < cur.len());
                            if replace {
                                best_root_cycle = Some(w_two_root);
                            }
                        }
                    }
                }
            }
        }

        let Some(w_two_root) = best_root_cycle else {
            continue;
        };
        if w_two_root.len() <= max_witness_terms
            && table.insert(AnchorRecord {
                class: class_of[root_u],
                witness: w_two_root.clone(),
                scale: 2,
            })
        {
            stats.harvested_half_anchors += 1;
        }
        for (&node, (enode, wnode)) in &local_vis {
            if node == root_u {
                continue;
            }
            // W_2(node) = 2 * W_T(node) - enode * W_2(root_u) -> large part +2 * D(node).
            if let Some(w_two) = combine_i16(&[(2, wnode), (-(*enode as i16), &w_two_root)]) {
                if w_two.len() <= max_witness_terms {
                    if table.insert(AnchorRecord {
                        class: class_of[node],
                        witness: w_two,
                        scale: 2,
                    }) {
                        stats.harvested_half_anchors += 1;
                    }
                } else {
                    stats.witness_cap_refusals += 1;
                }
            }
        }
    }

    stats
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DescentMode {
    Original,
    Traced,
    Anchored,
    TracedAnchored,
}

impl DescentMode {
    pub fn uses_trace(self) -> bool {
        matches!(self, Self::Traced | Self::TracedAnchored)
    }

    pub fn uses_anchors(self) -> bool {
        matches!(self, Self::Anchored | Self::TracedAnchored)
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Original => "original",
            Self::Traced => "traced",
            Self::Anchored => "anchored",
            Self::TracedAnchored => "traced-anchored",
        }
    }
}

/// Process-isolated cold F4 trace owned by a single discrete-log invocation.
#[derive(Default)]
pub struct LocalTraceState {
    trace: Option<F4Trace>,
    pub recorded: u64,
    pub replayed: u64,
    pub fallbacks: u64,
}

impl LocalTraceState {
    pub fn new() -> Self {
        Self::default()
    }

    fn solve_zero_dim(
        &mut self,
        input: &[f4_fp::Poly],
        n: usize,
        p: u64,
        opts: &F4Options,
        rng: &mut StdRng,
        use_trace: bool,
    ) -> (Vec<Vec<u64>>, ZeroDimStats) {
        let mut st = ZeroDimStats::default();
        let r = if !use_trace {
            f4_fp::f4(input, n, p, opts)
        } else if let Some(trace) = &self.trace {
            let rep = f4_fp::f4_replay(input, n, p, opts, trace);
            if rep.trace_mismatch {
                self.fallbacks += 1;
                st.trace_mismatch = true;
            } else {
                self.replayed += 1;
                st.replayed = true;
            }
            rep
        } else {
            let (rep, recorded) = f4_fp::f4_record(input, n, p, opts);
            if !rep.timed_out && !rep.inconsistent && rep.pairs_above_bound == 0 {
                self.trace = Some(recorded);
                self.recorded += 1;
            }
            rep
        };

        let t_lin = Instant::now();
        st.f4_muls = r.field_ops;
        st.degree_reached = r.degree_reached;
        st.solving_degree = r.solving_degree;
        st.first_hilbert_deviation_degree = r.first_hilbert_deviation_degree;
        st.last_hilbert_deviation_degree = r.last_hilbert_deviation_degree;
        st.staircase_extinction_degree = r.staircase_extinction_degree;
        st.max_rows = r.max_rows;
        st.max_cols = r.max_cols;
        st.f4_ms = r.ms;
        st.stopped_at = r.staircase_at_stop;
        st.hilbert_applied = r.hilbert == f4_fp::HilbertStop::Applied;
        st.hilbert_fallback = matches!(
            r.hilbert,
            f4_fp::HilbertStop::CheckFailed | f4_fp::HilbertStop::Violated
        );
        st.hilbert_check_muls = r.hilbert_check_ops;
        if r.timed_out {
            st.timed_out = true;
            return (Vec::new(), st);
        }
        if r.inconsistent {
            st.inconsistent = true;
            return (Vec::new(), st);
        }
        if r.pairs_above_bound > 0 {
            st.incomplete = true;
            return (Vec::new(), st);
        }
        let sols = cover::linear_stage_from_basis(
            &r.basis,
            input,
            n,
            p,
            rng,
            LinMode::Fast,
            &mut st,
            t_lin,
        );
        (sols, st)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SexticPattern {
    #[serde(rename = "1^6")]
    P1_6,
    #[serde(rename = "1^4+2")]
    P1_4_2,
    #[serde(rename = "1^2+2^2")]
    P1_2_2_2,
    #[serde(rename = "2^3")]
    P2_3,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct PatternHistogram {
    pub fp_sextics_tested: u64,
    pub non_squarefree_refused: u64,
    pub odd_remainder_refused: u64,
    pub higher_degree_refused: u64,
    pub seen_p1_6: u64,
    pub seen_p1_4_2: u64,
    pub seen_p1_2_2_2: u64,
    pub seen_p2_3: u64,
    pub anchor_misses: u64,
    pub accepted_p1_6: u64,
    pub accepted_p1_4_2: u64,
    pub accepted_p1_2_2_2: u64,
    pub accepted_p2_3: u64,
    pub accepted_half_scale: u64,
}

impl PatternHistogram {
    pub fn merge(&mut self, o: &Self) {
        self.fp_sextics_tested += o.fp_sextics_tested;
        self.non_squarefree_refused += o.non_squarefree_refused;
        self.odd_remainder_refused += o.odd_remainder_refused;
        self.higher_degree_refused += o.higher_degree_refused;
        self.seen_p1_6 += o.seen_p1_6;
        self.seen_p1_4_2 += o.seen_p1_4_2;
        self.seen_p1_2_2_2 += o.seen_p1_2_2_2;
        self.seen_p2_3 += o.seen_p2_3;
        self.anchor_misses += o.anchor_misses;
        self.accepted_p1_6 += o.accepted_p1_6;
        self.accepted_p1_4_2 += o.accepted_p1_4_2;
        self.accepted_p1_2_2_2 += o.accepted_p1_2_2_2;
        self.accepted_p2_3 += o.accepted_p2_3;
        self.accepted_half_scale += o.accepted_half_scale;
    }
}

/// Self-certifying witness for one accepted Nagao decomposition row.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AnchoredDescentCert {
    pub residual_index: u64,
    pub a_scalar: u64,
    pub b_scalar: u64,
    pub scale: u8,
    pub pattern: SexticPattern,
    pub residual_u: Vec<[u64; 2]>,
    pub residual_v: Vec<[u64; 2]>,
    pub a_poly: Vec<[u64; 2]>,
    pub c_poly: Vec<[u64; 2]>,
    pub sextic_s: [u64; 7],
    pub linear_terms: Vec<(usize, i8)>,
    pub quadratic_terms: Vec<([u64; 6], i8, u8, Vec<(usize, i16)>)>,
    pub expanded_row: Vec<(usize, i64)>,
}

fn poly_deriv_fp(ring: &FpRing, a: &[u64]) -> FpPoly {
    if a.len() <= 1 {
        return Vec::new();
    }
    let p = ring.p;
    let mut out = Vec::with_capacity(a.len() - 1);
    for (i, &c) in a.iter().enumerate().skip(1) {
        work::hook_mul(p);
        out.push(((i as u128 * c as u128) % p as u128) as u64);
    }
    while out.last() == Some(&0) {
        out.pop();
    }
    out
}

/// Binary powering `base^exp mod modulus` in `F_p[x]` (`modulus` monic, `deg >= 1`).
fn powmod_poly_fp(ring: &FpRing, base: &[u64], mut exp: u128, modulus: &[u64]) -> FpPoly {
    let mut acc = vec![1u64];
    let mut b = ring.rem(base, modulus);
    while exp > 0 {
        if exp & 1 == 1 {
            acc = ring.rem(&ring.mul(&acc, &b), modulus);
        }
        exp >>= 1;
        if exp > 0 {
            b = ring.rem(&ring.sqr(&b), modulus);
        }
    }
    acc
}

/// Split a monic squarefree `F_p[x]` polynomial of degree 2 or 4 or 6 known to be a
/// product of distinct monic irreducible quadratics (`gcd(q, x^p - x) == 1` and `x^{p^2} == x mod q`).
pub fn split_even_quadratics(
    ring: &FpRing,
    q: &[u64],
    rng: &mut StdRng,
    max_trials: usize,
) -> Option<Vec<[u64; 3]>> {
    match fdeg(q) {
        0 => Some(Vec::new()),
        2 => Some(vec![[q[0], q[1], q[2]]]),
        4 | 6 => {
            let p = ring.p;
            let exp = ((p as u128) * (p as u128) - 1) / 2;
            for _ in 0..max_trials {
                work::rng_words(2);
                let t0 = rng.gen_range(0..p);
                let t1 = rng.gen_range(1..p);
                let h = powmod_poly_fp(ring, &[t0, t1], exp, q);
                let h_minus_1 = ring.sub(&h, &[1]);
                if h_minus_1.is_empty() {
                    continue;
                }
                let g = ring.gcd(q, &h_minus_1);
                let dg = fdeg(&g);
                if dg == 2 || dg == 4 {
                    let other = ring.exact_div(q, &g);
                    let mut left = split_even_quadratics(ring, &g, rng, max_trials)?;
                    let mut right = split_even_quadratics(ring, &other, rng, max_trials)?;
                    left.append(&mut right);
                    left.sort_unstable();
                    return Some(left);
                }
            }
            None
        }
        _ => None,
    }
}

fn inverse_linear_mod_quad(f: &Fq, c0: E2, u_fp: &[u64; 3]) -> Option<[E2; 2]> {
    let d0 = f.sub(&c0, &f.from_fp(u_fp[1]));
    let d1 = f.neg(&E2::ONE);
    let det = f.add(&d0, &f.from_fp(u_fp[0]));
    // c(x) = c0 + x, u(x) = u0 + u1 x + x^2:
    // c(x) * ((c0 - u1) - x) = c0(c0 - u1) + (u1 - c0 + c0) x - x^2
    //   = c0(c0 - u1) + u1 x - (x^2 + u1 x + u0) + u1 x + u0? Use exact Cramer:
    let _ = (d0, d1, det);
    let u1 = u_fp[1];
    let u0 = u_fp[0];
    let num0 = f.sub(&c0, &f.from_fp(u1));
    let num1 = f.neg(&E2::ONE);
    // (c0 + x)(num0 + num1 x) = c0*num0 + (c0*num1 + num0) x + num1 x^2
    // Since num1 = -1 and x^2 == -u1 x - u0 (mod U):
    // == c0*(c0 - u1) + (-c0 + c0 - u1 + u1) x + u0 = c0*(c0 - u1) + u0!
    let determinant = f.add(&f.mul(&c0, &num0), &f.from_fp(u0));
    if determinant == E2::ZERO {
        return None;
    }
    let inv = f.inv(&determinant);
    Some([f.mul(&num0, &inv), f.mul(&num1, &inv)])
}

fn rem_poly_mod_quad(f: &Fq, a: &[E2], u_fp: &[u64; 3]) -> [E2; 2] {
    let mut r = a.to_vec();
    while r.len() >= 3 {
        let top = r.pop().unwrap();
        let idx = r.len();
        if top != E2::ZERO {
            r[idx - 1] = f.sub(&r[idx - 1], &f.scale(&top, u_fp[1]));
            r[idx - 2] = f.sub(&r[idx - 2], &f.scale(&top, u_fp[0]));
            work::words(2);
        }
    }
    [
        r.first().copied().unwrap_or(E2::ZERO),
        r.get(1).copied().unwrap_or(E2::ZERO),
    ]
}

fn mul_linear_mod_quad(f: &Fq, a: &[E2; 2], b: &[E2; 2], u_fp: &[u64; 3]) -> [E2; 2] {
    let c0 = f.mul(&a[0], &b[0]);
    let c1 = f.add(&f.mul(&a[0], &b[1]), &f.mul(&a[1], &b[0]));
    let c2 = f.mul(&a[1], &b[1]);
    rem_poly_mod_quad(f, &[c0, c1, c2], u_fp)
}

/// Decompose a single Jacobian residual `r = [a_scalar] G_J + [b_scalar] Q_J` under `mode`.
///
/// `source_small` maps `source_id -> small_terms` (`&[(base_idx, sign)]`) so matched
/// anchors expand directly into exact integer factor-base rows and are verified in `Jac(C)`.
#[allow(clippy::too_many_arguments)]
pub fn decompose_residual_anchored<F>(
    ctx: &Ctx,
    base: &[BaseEl],
    base_by_x: &[u32],
    by_x_map: &HashMap<u64, usize>,
    anchors: &AnchorTable,
    source_small: F,
    residual_index: u64,
    a_scalar: u64,
    b_scalar: u64,
    r: &Div<E2>,
    opts: &F4Options,
    mode: DescentMode,
    trace_state: &mut LocalTraceState,
    rng: &mut StdRng,
) -> (Option<AnchoredDescentCert>, NagaoCost, PatternHistogram)
where
    F: Fn(usize) -> &'static [(usize, i8)] + Copy,
{
    decompose_residual_anchored_ref(
        ctx,
        base,
        base_by_x,
        by_x_map,
        anchors,
        &|id| source_small(id),
        residual_index,
        a_scalar,
        b_scalar,
        r,
        opts,
        mode,
        trace_state,
        rng,
    )
}

/// Lifetime-polymorphic variant of [`decompose_residual_anchored`].
#[allow(clippy::too_many_arguments)]
pub fn decompose_residual_anchored_ref<'a>(
    ctx: &Ctx,
    base: &[BaseEl],
    base_by_x: &[u32],
    by_x_map: &HashMap<u64, usize>,
    anchors: &AnchorTable,
    source_small: &dyn Fn(usize) -> &'a [(usize, i8)],
    residual_index: u64,
    a_scalar: u64,
    b_scalar: u64,
    r: &Div<E2>,
    opts: &F4Options,
    mode: DescentMode,
    trace_state: &mut LocalTraceState,
    rng: &mut StdRng,
) -> (Option<AnchoredDescentCert>, NagaoCost, PatternHistogram) {
    let f = &ctx.f.f;
    let p = f.p;
    let mut cost = NagaoCost::default();
    let mut hist = PatternHistogram::default();
    if r.u.len() != 4 {
        cost.degenerate = true;
        return (None, cost, hist);
    }

    let t_all = Instant::now();
    let m0 = f.muls();
    let mut v = r.v.clone();
    v.resize(3, E2::ZERO);
    let forms = cover::weil_forms(f, &ctx.cov.hx, &r.u, &v);
    let polys = forms.polys(&ctx.reim, p);
    f.count_public(6 * 10 * 8);
    cost.weil_muls = f.muls() - m0;

    let (sols, st) = trace_state.solve_zero_dim(&polys, 6, p, opts, rng, mode.uses_trace());
    cost.f4_muls = st.f4_muls;
    cost.lin_muls = st.lin_muls;
    cost.delta = st.delta;
    cost.degree_reached = st.degree_reached;
    cost.first_hilbert_deviation_degree = st.first_hilbert_deviation_degree;
    cost.last_hilbert_deviation_degree = st.last_hilbert_deviation_degree;
    cost.staircase_extinction_degree = st.staircase_extinction_degree;
    cost.max_rows = st.max_rows;
    cost.max_cols = st.max_cols;
    cost.f4_ms = st.f4_ms;
    cost.lin_ms = st.lin_ms;
    cost.inconsistent = st.inconsistent;
    cost.incomplete = st.incomplete;
    cost.timed_out = st.timed_out;
    cost.nonseparating = st.nonseparating;
    cost.lin_fallback = st.lin_fallback;
    cost.stopped_at = st.stopped_at;
    cost.hilbert_applied = st.hilbert_applied;
    cost.hilbert_fallback = st.hilbert_fallback;
    cost.hilbert_check_muls = st.hilbert_check_muls;
    cost.replayed = st.replayed;
    cost.trace_mismatch = st.trace_mismatch;
    cost.fp_solutions = sols.len();

    let m1 = f.muls();
    let uc = |k: isize| -> E2 {
        if k >= 0 && (k as usize) < r.u.len() {
            r.u[k as usize]
        } else {
            E2::ZERO
        }
    };
    let vc = |k: isize| -> E2 {
        if k >= 0 && (k as usize) < v.len() {
            v[k as usize]
        } else {
            E2::ZERO
        }
    };

    // Baseline `(1^6)`-only path preserves exact parity with `jv_cover::nagao_decompose`.
    if !mode.uses_anchors() {
        let ring = PolyRing::new(p);
        let mut out_set = std::collections::BTreeSet::<Dec6>::new();
        let mut best_cert_data: Option<(Dec6, [u64; 7], Vec<[u64; 2]>, Vec<[u64; 2]>)> = None;
        'sol6: for s in &sols {
            let lam = [E2([s[0], s[1]]), E2([s[2], s[3]]), E2([s[4], s[5]])];
            let mut coeffs = Vec::with_capacity(7);
            for k in 0..6 {
                let c = forms.eval(f, k, &lam);
                if c.0[1] != 0 {
                    continue 'sol6;
                }
                coeffs.push(c.0[0]);
            }
            coeffs.push(1);
            hist.fp_sextics_tested += 1;
            let roots = ring.roots(&UPoly(coeffs.clone()), rng);
            if roots.len() != 6 {
                continue 'sol6;
            }
            hist.seen_p1_6 += 1;
            let ak: Vec<E2> = (0..5isize)
                .map(|k| {
                    let mut a = f.neg(&vc(k - 1));
                    a = f.add(&a, &f.mul(&lam[0], &uc(k)));
                    a = f.add(&a, &f.mul(&lam[1], &uc(k - 1)));
                    f.sub(&a, &f.mul(&lam[2], &vc(k)))
                })
                .collect();
            let jc = ctx.jac();
            let mut options: Vec<(usize, Vec<i8>)> = Vec::with_capacity(6);
            for &x in &roots {
                let xe = f.from_fp(x);
                work::probe(1);
                let Some(&i) = by_x_map.get(&x) else {
                    continue 'sol6;
                };
                let mu = f.add(&lam[2], &xe);
                if mu == E2::ZERO {
                    options.push((i, vec![1, -1]));
                    continue;
                }
                let y = f.neg(&f.mul(&peval(f, &ak, &xe), &f.inv(&mu)));
                if y == E2::ZERO || f.sq(&y) != peval(f, &ctx.cov.hx, &xe) {
                    continue 'sol6;
                }
                let eps: i8 = if y == base[i].y {
                    -1
                } else if y == f.neg(&base[i].y) {
                    1
                } else {
                    continue 'sol6;
                };
                options.push((i, vec![eps]));
            }
            if options.len() != 6 {
                continue 'sol6;
            }
            let free: Vec<usize> = (0..6).filter(|&k| options[k].1.len() > 1).collect();
            if free.len() > 4 {
                continue 'sol6;
            }
            for mask in 0..(1usize << free.len()) {
                let mut terms: Vec<(usize, i8)> = options.iter().map(|(i, v)| (*i, v[0])).collect();
                for (bit, &pos) in free.iter().enumerate() {
                    terms[pos].1 = options[pos].1[(mask >> bit) & 1];
                }
                terms.sort_unstable();
                let Ok(terms_arr) = terms.try_into() else {
                    continue;
                };
                let cand = Dec6 { terms: terms_arr };
                if verify_dec(&jc, base, r, &cand) {
                    let sextic_s: [u64; 7] = coeffs.clone().try_into().unwrap();
                    let a_enc = ak.iter().map(|z| z.0).collect::<Vec<_>>();
                    let c_enc = vec![lam[2].0, E2::ONE.0];
                    let inserted = out_set.insert(cand);
                    let replace = best_cert_data
                        .as_ref()
                        .map_or(inserted, |(cur, _, _, _)| cand < *cur);
                    if replace {
                        best_cert_data = Some((cand, sextic_s, a_enc, c_enc));
                    }
                    break;
                }
            }
        }
        cost.post_muls = f.muls() - m1 + ring.muls.get();
        cost.total_ms = t_all.elapsed().as_secs_f64() * 1e3;
        if let Some((dec, sextic_s, a_poly, c_poly)) = best_cert_data {
            hist.accepted_p1_6 += 1;
            let linear_terms: Vec<(usize, i8)> =
                dec.terms.iter().map(|&(idx, eps)| (idx, -eps)).collect();
            let expanded_row: Vec<(usize, i64)> = dec
                .terms
                .iter()
                .map(|&(idx, eps)| (idx, eps as i64))
                .collect();
            let cert = AnchoredDescentCert {
                residual_index,
                a_scalar,
                b_scalar,
                scale: 1,
                pattern: SexticPattern::P1_6,
                residual_u: r.u.iter().map(|z| z.0).collect(),
                residual_v: r.v.iter().map(|z| z.0).collect(),
                a_poly,
                c_poly,
                sextic_s,
                linear_terms,
                quadratic_terms: Vec::new(),
                expanded_row,
            };
            return (Some(cert), cost, hist);
        }
        return (None, cost, hist);
    }

    // Anchored multi-pattern path: `(1^6), (1^4, 2), (1^2, 2^2), (2^3)`.
    let fp_ring = FpRing::new(p);
    let poly_ring = PolyRing::new(p);
    let jc = ctx.jac();

    'sol: for s in &sols {
        let lam = [E2([s[0], s[1]]), E2([s[2], s[3]]), E2([s[4], s[5]])];
        let mut coeffs = [0u64; 7];
        for k in 0..6 {
            let c = forms.eval(f, k, &lam);
            if c.0[1] != 0 {
                continue 'sol;
            }
            coeffs[k] = c.0[0];
        }
        coeffs[6] = 1;
        hist.fp_sextics_tested += 1;

        // 1. Distinct-factor screen: `gcd(S, S') == 1`.
        let deriv = poly_deriv_fp(&fp_ring, &coeffs);
        if fdeg(&fp_ring.gcd(&coeffs, &deriv)) != 0 {
            hist.non_squarefree_refused += 1;
            continue 'sol;
        }

        // 2. Linear factor part `g1 = gcd(S, x^p - x)`.
        let xp_mod_s = fp_ring.powmod_x(p as u128, &coeffs);
        let xp_minus_x = fp_ring.sub(&xp_mod_s, &[0, 1]);
        let g1 = if xp_minus_x.is_empty() {
            coeffs.to_vec()
        } else {
            fp_ring.gcd(&coeffs, &xp_minus_x)
        };
        let d1 = fdeg(&g1) as usize;
        let dq = 6usize.saturating_sub(d1);
        if !matches!(dq, 0 | 2 | 4 | 6) {
            hist.odd_remainder_refused += 1;
            continue 'sol;
        }

        let q_rem = if dq == 0 {
            vec![1u64]
        } else {
            fp_ring.exact_div(&coeffs, &g1)
        };

        // 3. For `dq in {4, 6}`, require `x^{p^2} == x (mod q_rem)` so `q_rem` splits into quadratics.
        if dq >= 4 {
            let xp_q = fp_ring.rem(&xp_mod_s, &q_rem);
            let xp2_q = powmod_poly_fp(&fp_ring, &xp_q, p as u128, &q_rem);
            if xp2_q != vec![0, 1] {
                hist.higher_degree_refused += 1;
                continue 'sol;
            }
        }

        let pattern = match dq {
            0 => {
                hist.seen_p1_6 += 1;
                SexticPattern::P1_6
            }
            2 => {
                hist.seen_p1_4_2 += 1;
                SexticPattern::P1_4_2
            }
            4 => {
                hist.seen_p1_2_2_2 += 1;
                SexticPattern::P1_2_2_2
            }
            6 => {
                hist.seen_p2_3 += 1;
                SexticPattern::P2_3
            }
            _ => unreachable!(),
        };

        // 4. Split `q_rem` into `dq / 2` monic irreducible quadratics (`dq == 2` needs zero CZ work).
        let Some(quads) = split_even_quadratics(&fp_ring, &q_rem, rng, 32) else {
            continue 'sol;
        };

        // Build `A(x)` (`deg <= 4`) once; `C(x) = lam[2] + x`.
        let ak: Vec<E2> = (0..5isize)
            .map(|k| {
                let mut a = f.neg(&vc(k - 1));
                a = f.add(&a, &f.mul(&lam[0], &uc(k)));
                a = f.add(&a, &f.mul(&lam[1], &uc(k - 1)));
                f.sub(&a, &f.mul(&lam[2], &vc(k)))
            })
            .collect();

        // 5. Check quadratic anchor membership FIRST before splitting `g1`!
        let mut quad_matches: Vec<([u64; 6], i8, u8, Vec<(usize, i16)>)> =
            Vec::with_capacity(quads.len());
        for u_fp in &quads {
            let Some(c_inv) = inverse_linear_mod_quad(f, lam[2], u_fp) else {
                continue 'sol;
            };
            let a_mod = rem_poly_mod_quad(f, &ak, u_fp);
            let prod = mul_linear_mod_quad(f, &a_mod, &c_inv, u_fp);
            let v_zero = [f.neg(&prod[0]), f.neg(&prod[1])];
            // Mumford validity `v_zero^2 == H (mod U)`.
            let v_sq = mul_linear_mod_quad(f, &v_zero, &v_zero, u_fp);
            let h_mod = rem_poly_mod_quad(f, &ctx.cov.hx, u_fp);
            if v_sq != h_mod {
                continue 'sol;
            }
            let (class_key, c_sign) = canonical_quadratic_class(f, u_fp, &v_zero);
            let Some(anchor) = anchors.get_counted(&class_key) else {
                hist.anchor_misses += 1;
                continue 'sol;
            };
            quad_matches.push((class_key, c_sign, anchor.scale, anchor.witness.clone()));
        }

        // 6. All quadratic factors are grounded! Now extract the `d1` linear roots of `g1`.
        let roots = if d1 == 0 {
            Vec::new()
        } else {
            let r_vec = poly_ring.roots(&UPoly(g1), rng);
            if r_vec.len() != d1 {
                continue 'sol;
            }
            r_vec
        };

        let mut linear_terms: Vec<(usize, i8)> = Vec::with_capacity(d1);
        for &x in &roots {
            work::words(1);
            let idx_u32 = *base_by_x.get(x as usize).unwrap_or(&u32::MAX);
            if idx_u32 == u32::MAX {
                continue 'sol;
            }
            let b_idx = idx_u32 as usize;
            let xe = f.from_fp(x);
            let mu = f.add(&lam[2], &xe);
            if mu == E2::ZERO {
                continue 'sol;
            }
            // Inversion-free ordinate binding: `A(x) + sigma * mu(x) * y_B == 0`.
            let ax = peval(f, &ak, &xe);
            if ax == E2::ZERO {
                continue 'sol;
            }
            let uy = f.mul(&mu, &base[b_idx].y);
            let sigma: i8 = if f.add(&ax, &uy) == E2::ZERO {
                1
            } else if f.sub(&ax, &uy) == E2::ZERO {
                -1
            } else {
                continue 'sol;
            };
            linear_terms.push((b_idx, sigma));
        }
        linear_terms.sort_unstable_by_key(|&(idx, _)| idx);
        if linear_terms.windows(2).any(|w| w[0].0 == w[1].0) {
            continue 'sol;
        }

        // 7. Combine linear terms and anchor witnesses at exact integer scale `S = lcm(scale_j) in {1, 2}`:
        //    `S * R + S * sum sigma_i B_{b_i} - sum_j (S / scale_j) * c_j * sum_{(src, w)} w * small(src) = 0`
        //    `=> [S] R = sum_k d_k B_k`.
        let scale: u8 = if quad_matches.iter().any(|&(_, _, sc, _)| sc == 2) {
            2
        } else {
            1
        };
        let s_i64 = scale as i64;
        let mut row_acc = BTreeMap::<usize, i64>::new();
        for &(b_idx, sigma) in &linear_terms {
            *row_acc.entry(b_idx).or_default() -= s_i64 * (sigma as i64);
            work::words(3);
        }
        for &(_, c_sign, a_scale, ref witness) in &quad_matches {
            let mult = (s_i64 / (a_scale as i64)) * (c_sign as i64);
            for &(src_id, w) in witness {
                let coef_outer = mult * (w as i64);
                for &(b_idx, s_term) in source_small(src_id) {
                    *row_acc.entry(b_idx).or_default() += coef_outer * (s_term as i64);
                    work::words(3);
                }
            }
        }
        let expanded_row: Vec<(usize, i64)> =
            row_acc.into_iter().filter(|&(_, c)| c != 0).collect();

        // 8. Independent Cantor Jacobian check `[scale] R == sum_k d_k B_k`.
        if !verify_expanded_jacobian(&jc, base, r, scale, &expanded_row) {
            continue 'sol;
        }

        match pattern {
            SexticPattern::P1_6 => hist.accepted_p1_6 += 1,
            SexticPattern::P1_4_2 => hist.accepted_p1_4_2 += 1,
            SexticPattern::P1_2_2_2 => hist.accepted_p1_2_2_2 += 1,
            SexticPattern::P2_3 => hist.accepted_p2_3 += 1,
        }
        if scale == 2 {
            hist.accepted_half_scale += 1;
        }

        cost.post_muls = f.muls() - m1 + fp_ring.muls() + poly_ring.muls.get();
        cost.total_ms = t_all.elapsed().as_secs_f64() * 1e3;
        let cert = AnchoredDescentCert {
            residual_index,
            a_scalar,
            b_scalar,
            scale,
            pattern,
            residual_u: r.u.iter().map(|z| z.0).collect(),
            residual_v: r.v.iter().map(|z| z.0).collect(),
            a_poly: ak.iter().map(|z| z.0).collect(),
            c_poly: vec![lam[2].0, E2::ONE.0],
            sextic_s: coeffs,
            linear_terms,
            quadratic_terms: quad_matches,
            expanded_row,
        };
        return (Some(cert), cost, hist);
    }

    cost.post_muls = f.muls() - m1 + fp_ring.muls() + poly_ring.muls.get();
    cost.total_ms = t_all.elapsed().as_secs_f64() * 1e3;
    (None, cost, hist)
}

/// Verify `[scale] * r == sum_{(idx, c)} c * base[idx].d` in `Jac(C)`.
pub fn verify_expanded_jacobian(
    jc: &Hyp<Fq>,
    base: &[BaseEl],
    r: &Div<E2>,
    scale: u8,
    expanded_row: &[(usize, i64)],
) -> bool {
    if !matches!(scale, 1 | 2) || !jc.is_valid(r) {
        return false;
    }
    let lhs = if scale == 1 { r.clone() } else { jc.add(r, r) };
    let mut rhs = jc.identity();
    for &(idx, coef) in expanded_row {
        let Some(b) = base.get(idx) else {
            return false;
        };
        if coef == 0 {
            continue;
        }
        let mag = coef.unsigned_abs() as u128;
        let term = jc.mul(&b.d, mag);
        let signed_term = if coef > 0 { term } else { jc.neg(&term) };
        rhs = jc.add(&rhs, &signed_term);
    }
    lhs == rhs
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    #[test]
    fn universe_census_matches_exhaustive_mumford_search_on_small_primes() {
        for &p in &[7u64, 11] {
            let f = Fq::new(p);
            // Pick a squarefree degree-7 monic polynomial over F_{p^2}.
            let hx = vec![
                E2([2, 1]),
                E2([1, 0]),
                E2([0, 2]),
                E2([3 % p, 1]),
                E2([1, 1]),
                E2([0, 0]),
                E2([2 % p, 0]),
                E2::ONE,
            ];
            // Brute-force all monic irreducible U(x) = u0 + u1 x + x^2 in F_p[x]
            // and all V(x) = v0 + v1 x in F_{p^2}[x] with V^2 == H (mod U).
            let mut classes = std::collections::BTreeSet::<[u64; 6]>::new();
            let mut irred = 0u64;
            for u0 in 0..p {
                for u1 in 0..p {
                    let disc = ((u1 * u1 + 4 * p * p - 4 * u0) % p) as u64;
                    if disc == 0 || (0..p).any(|t| (t * t) % p == disc) {
                        continue;
                    }
                    irred += 1;
                    let u_fp = [u0, u1, 1];
                    let h_mod = rem_poly_mod_quad(&f, &hx, &u_fp);
                    for v00 in 0..p {
                        for v01 in 0..p {
                            for v10 in 0..p {
                                for v11 in 0..p {
                                    let v = [E2([v00, v01]), E2([v10, v11])];
                                    if mul_linear_mod_quad(&f, &v, &v, &u_fp) == h_mod {
                                        let (key, _) = canonical_quadratic_class(&f, &u_fp, &v);
                                        classes.insert(key);
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Compare against the O(p^2) norm-QR census formula on the same `hx`.
            let w = f.w;
            let mut is_qr = vec![false; p as usize];
            for x in 1..p {
                is_qr[((x * x) % p) as usize] = true;
            }
            let fp2_sqrts = |z: E2| -> u64 {
                if z == E2::ZERO {
                    return 1;
                }
                let n = (z.0[0] * z.0[0] + p * p - (w * z.0[1] % p) * z.0[1]) % p;
                if is_qr[n as usize] {
                    2
                } else {
                    0
                }
            };
            let mut total = 0u64;
            for a in 0..p {
                for b in 1..=(p - 1) / 2 {
                    let r1 = fp2_sqrts(peval(&f, &hx, &E2([a, b])));
                    let r2 = fp2_sqrts(peval(&f, &hx, &E2([a, p - b])));
                    if r1 > 0 && r2 > 0 {
                        total += if r1 == 2 && r2 == 2 { 2 } else { 1 };
                    }
                }
            }
            assert_eq!(irred, p * (p - 1) / 2);
            assert_eq!(total, classes.len() as u64, "census mismatch at p={p}");
        }
    }

    #[test]
    fn split_even_quadratics_recovers_two_and_three_irreducible_factors() {
        for &p in &[11u64, 101, 1009] {
            let ring = FpRing::new(p);
            let mut rng = StdRng::seed_from_u64(99 + p);
            let mut irreds = Vec::new();
            for u0 in 1..p {
                for u1 in 0..p {
                    let disc = (u1 * u1 + 4 * p * p - 4 * u0) % p;
                    let legendre = powmod_poly_fp(&ring, &[disc], ((p - 1) / 2) as u128, &[0, 1]);
                    let _ = legendre;
                    // Euler criterion in F_p
                    let mut acc = 1u64;
                    let mut base = disc;
                    let mut e = (p - 1) / 2;
                    while e > 0 {
                        if e & 1 == 1 {
                            acc = (acc * base) % p;
                        }
                        base = (base * base) % p;
                        e >>= 1;
                    }
                    if acc == p - 1 {
                        irreds.push([u0, u1, 1u64]);
                        if irreds.len() == 6 {
                            break;
                        }
                    }
                }
                if irreds.len() == 6 {
                    break;
                }
            }
            let p4 = ring.mul(&irreds[0], &irreds[2]);
            let mut want2 = vec![irreds[0], irreds[2]];
            want2.sort_unstable();
            assert_eq!(split_even_quadratics(&ring, &p4, &mut rng, 32), Some(want2));

            let p6 = ring.mul(&p4, &irreds[5]);
            let mut want3 = vec![irreds[0], irreds[2], irreds[5]];
            want3.sort_unstable();
            assert_eq!(split_even_quadratics(&ring, &p6, &mut rng, 32), Some(want3));
        }
    }

    #[test]
    fn frustrated_cycle_harvester_grounds_odd_cycles_and_ignores_balanced_cycles() {
        let c0 = [1, 0, 0, 0, 0, 0];
        let c1 = [2, 0, 0, 0, 0, 0];
        let c2 = [3, 0, 0, 0, 0, 0];
        let c3 = [4, 0, 0, 0, 0, 0];

        // Balanced 3-cycle: D0 - D1 = 0, D1 - D2 = 0, D2 - D0 = 0 -> 0 half-anchors.
        let balanced = vec![(0, c0, 1, c1, -1), (1, c1, 1, c2, -1), (2, c2, 1, c0, -1)];
        let mut tab_bal = AnchorTable::with_capacity(8);
        let st_bal = harvest_frustrated_cycle_anchors(&balanced, &mut tab_bal, 64);
        assert_eq!(st_bal.balanced_cycles, 1);
        assert_eq!(st_bal.harvested_half_anchors, 0);

        // Frustrated 3-cycle + attached tail c3 across all 16 sign orientations on the cycle:
        for mask in 0..16u8 {
            let s0a = if mask & 1 != 0 { 1 } else { -1 };
            let s0b = if mask & 2 != 0 { 1 } else { -1 };
            let s1a = if mask & 4 != 0 { 1 } else { -1 };
            let s1b = if mask & 8 != 0 { 1 } else { -1 };
            // Force odd product (-s0a*s0b)*(-s1a*s1b)*(-s2a*s2b) == -1.
            let prod2 = (-s0a * s0b) * (-s1a * s1b);
            let (s2a, s2b) = if prod2 == 1 { (1, 1) } else { (1, -1) };
            let edges = vec![
                (10, c0, s0a, c1, s0b),
                (11, c1, s1a, c2, s1b),
                (12, c2, s2a, c0, s2b),
                (13, c2, 1, c3, -1),
            ];
            let mut tab = AnchorTable::with_capacity(8);
            let st = harvest_frustrated_cycle_anchors(&edges, &mut tab, 64);
            assert_eq!(st.harvested_half_anchors, 4, "mask={mask}");
            // Verify exact large-part conservation `+2 * D(target)` for every harvested vertex!
            for target in [c0, c1, c2, c3] {
                let rec = tab.records.get(&target).expect("anchored");
                assert_eq!(rec.scale, 2);
                let mut coeff_by_class = BTreeMap::<[u64; 6], i64>::new();
                for &(src, w) in &rec.witness {
                    let (_, u, su, v, sv) = edges.iter().copied().find(|e| e.0 == src).unwrap();
                    *coeff_by_class.entry(u).or_default() += w as i64 * su as i64;
                    *coeff_by_class.entry(v).or_default() += w as i64 * sv as i64;
                }
                coeff_by_class.retain(|_, v| *v != 0);
                assert_eq!(coeff_by_class.len(), 1, "mask={mask} target={target:?}");
                assert_eq!(coeff_by_class.get(&target), Some(&2));
            }
        }
    }
}
