//! Online Exact-`Z` Signed Frame-Matroid Graph & Watched-Root `3-LP` Hypergraph
//! for Extension-Field Index Calculus (`jv_signed_matroid_graph`).
//!
//! Unifies and strictly generalizes the partial-relation graph engines in this
//! crate for divisor groups where relation witnesses must hold over `Z`
//! (without modular inversions `2^-1 mod ell` that would corrupt Jacobian
//! cofactor checks):
//!
//! 1. **Dense `u32` Vertex Interning & Contiguous Linear `ZWitness` Merges**:
//!    `HashMap<[u64; 6], u32>` interns 48-byte canonical Mumford classes once
//!    into `u32` IDs; witness combinations use cache-contiguous two-pointer
//!    merges on `Vec<(usize, i16)>` rather than `BTreeMap` node allocations.
//! 2. **Complete Zaslavsky Signed-Matroid Cycle Harvesting over `Z`**:
//!    - Maintains union-by-size components (`comp_root[v]` in `O(1)`),
//!      smaller-tree re-rooting (`O(V log V)` total edge flips), and LCA
//!      shared-suffix cancellation (`O(dist(u, LCA) + dist(v, LCA))`).
//!    - **Balanced (even-sign) cycles** emit complete `Z`-relations (`large == 0`)
//!      at the LCA immediately, even in completely unanchored components.
//!    - **Frustrated (odd-sign) cycles** install a `Half` (`+2 * D(root)`)
//!      root witness on first arrival; every subsequent cycle in that component,
//!      every `(Half, Half)` bridge, and every `(Unit, Half)` bridge emits a
//!      complete `Z`-relation immediately.
//! 3. **Dynamic Shortest-Witness Relaxation & Multi-Source Dijkstra Export**:
//!    - Whenever a shorter `Unit` or `Half` witness reaches an anchored root,
//!      the cycle difference is emitted and the stored root representative is
//!      tightened in place (preserving the exact row space while shrinking
//!      future LA and descent row weights).
//!    - `export_anchor_table` runs multi-source shortest-witness propagation
//!      from all direct `1-LP` hits and `Unit` roots simultaneously across all
//!      retained `2-LP` edges, assigning every anchored vertex its minimum-hop
//!      `scale = 1` (or `scale = 2`) integer witness for Phase-3 multi-pattern
//!      descent.
//! 4. **Watched-Root `k = 3` (`3-LP`, `(1^3, 2^3)`) Hypergraph Cascade**:
//!    - Accepts arity `0..=3` unit-signed partial relations, reducing anchored
//!      endpoints and opposite-parity same-component pairs immediately, and
//!      parking unreduced `3-LP` hyperedges on component watch lists so newly
//!      anchored or merged components trigger cascading `3-LP -> 2-LP -> 1-LP -> Full`
//!      peeling.

use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

use serde::{Deserialize, Serialize};

use crate::cryptanalysis::jv_anchored_descent::{AnchorRecord, AnchorTable};
use crate::cryptanalysis::jv_bitops as work;
use crate::cryptanalysis::jv_sieve::two_large_prime::quadratic_pair_graph::{Class, LargeTerm};

/// Sorted `(source_id, nonzero i16 coefficient)` integer witness over `Z`.
pub type ZWitness = Vec<(usize, i16)>;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct MatroidOptions {
    pub max_sources: usize,
    pub max_vertices: usize,
    pub max_tree_edges: usize,
    pub max_hyperedges: usize,
    pub max_witness_terms: usize,
    pub max_abs_coeff: i16,
}

impl Default for MatroidOptions {
    fn default() -> Self {
        Self {
            max_sources: 524_288,
            max_vertices: 262_144,
            max_tree_edges: 262_144,
            max_hyperedges: 131_072,
            max_witness_terms: 256,
            max_abs_coeff: 4096,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct MatroidStats {
    pub sources_seen: usize,
    pub vertices: usize,
    pub tree_edges: usize,
    pub full_zero_lp: u64,
    pub unit_anchors_installed: u64,
    pub half_anchors_installed: u64,
    pub half_upgraded_to_unit: u64,
    pub balanced_unanchored_cycles: u64,
    pub anchored_cycles: u64,
    pub half_bridge_cycles: u64,
    pub hyper3_received: u64,
    pub hyper3_parked: u64,
    pub hyper3_woken: u64,
    pub hyper3_reduced_to_full: u64,
    pub hyper3_reduced_to_1lp: u64,
    pub hyper3_reduced_to_2lp: u64,
    pub anchor_relaxations: u64,
    pub reroot_flips: u64,
    pub lca_shared_steps_saved: u64,
    pub emitted_relations: u64,
    pub max_emitted_witness: usize,
    pub witness_cap_refusals: u64,
    pub capacity_refusals: u64,
}

#[derive(Clone, Debug)]
enum EdgeWitness {
    Single(usize),
    Multi(ZWitness),
}

impl EdgeWitness {
    fn as_slice_buf<'a>(&'a self, tmp: &'a mut [(usize, i16); 1]) -> &'a [(usize, i16)] {
        match self {
            Self::Single(s) => {
                tmp[0] = (*s, 1);
                &tmp[..]
            }
            Self::Multi(v) => v.as_slice(),
        }
    }

    fn len(&self) -> usize {
        match self {
            Self::Single(_) => 1,
            Self::Multi(v) => v.len(),
        }
    }
}

#[derive(Clone, Debug)]
struct TreeEdge {
    witness: EdgeWitness,
    coeff_child: i8,
    coeff_parent: i8,
}

#[derive(Clone, Debug, Default)]
enum RootAnchor {
    #[default]
    None,
    /// Integer witness with exact large part `+2 * D(root)`.
    Half(ZWitness),
    /// Integer witness with exact large part `+1 * D(root)`.
    Unit(ZWitness),
}

#[derive(Clone, Debug)]
struct StoredEdge2 {
    u: u32,
    v: u32,
    cu: i8,
    cv: i8,
    witness: EdgeWitness,
}

#[derive(Clone, Debug)]
struct HyperEdge3 {
    live: bool,
    base_witness: ZWitness,
    terms: [(u32, i8); 3],
}

enum LcaOutcome {
    BalancedCycle(ZWitness),
    FrustratedRoot(ZWitness),
    CapRefused,
}

/// Linear two-pointer combination `sa * a + sb * b` on sorted `(usize, i16)` slices.
pub fn merge2_z(
    a: &[(usize, i16)],
    sa: i16,
    b: &[(usize, i16)],
    sb: i16,
    max_terms: usize,
    max_abs: i16,
) -> Option<ZWitness> {
    if sa == 0 && sb == 0 {
        return Some(Vec::new());
    }
    let mut out = Vec::with_capacity(a.len() + b.len());
    let (mut i, mut j) = (0usize, 0usize);
    while i < a.len() && j < b.len() {
        work::words(1);
        match a[i].0.cmp(&b[j].0) {
            Ordering::Less => {
                if sa != 0 {
                    let v = (a[i].1 as i32) * (sa as i32);
                    work::words(2);
                    if v.abs() > max_abs as i32 {
                        return None;
                    }
                    if v != 0 {
                        out.push((a[i].0, v as i16));
                    }
                }
                i += 1;
            }
            Ordering::Greater => {
                if sb != 0 {
                    let v = (b[j].1 as i32) * (sb as i32);
                    work::words(2);
                    if v.abs() > max_abs as i32 {
                        return None;
                    }
                    if v != 0 {
                        out.push((b[j].0, v as i16));
                    }
                }
                j += 1;
            }
            Ordering::Equal => {
                let v = (a[i].1 as i32) * (sa as i32) + (b[j].1 as i32) * (sb as i32);
                work::words(3);
                if v.abs() > max_abs as i32 {
                    return None;
                }
                if v != 0 {
                    out.push((a[i].0, v as i16));
                }
                i += 1;
                j += 1;
            }
        }
    }
    while i < a.len() {
        if sa != 0 {
            let v = (a[i].1 as i32) * (sa as i32);
            work::words(2);
            if v.abs() > max_abs as i32 {
                return None;
            }
            if v != 0 {
                out.push((a[i].0, v as i16));
            }
        }
        i += 1;
    }
    while j < b.len() {
        if sb != 0 {
            let v = (b[j].1 as i32) * (sb as i32);
            work::words(2);
            if v.abs() > max_abs as i32 {
                return None;
            }
            if v != 0 {
                out.push((b[j].0, v as i16));
            }
        }
        j += 1;
    }
    if out.len() > max_terms {
        None
    } else {
        Some(out)
    }
}

fn scale_z(a: &[(usize, i16)], s: i16, max_terms: usize, max_abs: i16) -> Option<ZWitness> {
    merge2_z(a, s, &[], 0, max_terms, max_abs)
}

pub struct SignedMatroidGraph {
    options: MatroidOptions,
    class_to_id: HashMap<Class, u32>,
    id_to_class: Vec<Class>,
    comp_root: Vec<u32>,
    members: Vec<Vec<u32>>,
    tree_parent: Vec<u32>,
    tree_edge: Vec<Option<TreeEdge>>,
    root_anchor: Vec<RootAnchor>,
    best_direct_unit: Vec<Option<ZWitness>>,
    comp_hyper_watch: Vec<Vec<usize>>,
    edges2_for_export: Vec<StoredEdge2>,
    adj2_for_export: Vec<Vec<usize>>,
    hyperedges: Vec<HyperEdge3>,
    wakeup_queue: VecDeque<usize>,
    stats: MatroidStats,
}

impl SignedMatroidGraph {
    pub fn new(options: MatroidOptions) -> Result<Self, &'static str> {
        if options.max_sources == 0
            || options.max_vertices == 0
            || options.max_tree_edges == 0
            || options.max_witness_terms == 0
            || options.max_abs_coeff <= 0
        {
            return Err("positive matroid graph bounds required");
        }
        Ok(Self {
            options,
            class_to_id: HashMap::new(),
            id_to_class: Vec::new(),
            comp_root: Vec::new(),
            members: Vec::new(),
            tree_parent: Vec::new(),
            tree_edge: Vec::new(),
            root_anchor: Vec::new(),
            best_direct_unit: Vec::new(),
            comp_hyper_watch: Vec::new(),
            edges2_for_export: Vec::new(),
            adj2_for_export: Vec::new(),
            hyperedges: Vec::new(),
            wakeup_queue: VecDeque::new(),
            stats: MatroidStats::default(),
        })
    }

    pub fn stats(&self) -> &MatroidStats {
        &self.stats
    }

    pub fn live_hyperedges(&self) -> usize {
        self.hyperedges.iter().filter(|h| h.live).count()
    }

    fn intern_class(&mut self, c: Class) -> Option<u32> {
        work::probe(6);
        if let Some(&id) = self.class_to_id.get(&c) {
            work::words(1);
            return Some(id);
        }
        if self.id_to_class.len() >= self.options.max_vertices {
            self.stats.capacity_refusals += 1;
            return None;
        }
        let id = self.id_to_class.len() as u32;
        self.class_to_id.insert(c, id);
        self.id_to_class.push(c);
        self.comp_root.push(id);
        self.members.push(vec![id]);
        self.tree_parent.push(id);
        self.tree_edge.push(None);
        self.root_anchor.push(RootAnchor::None);
        self.best_direct_unit.push(None);
        self.comp_hyper_watch.push(Vec::new());
        self.adj2_for_export.push(Vec::new());
        self.stats.vertices = self.id_to_class.len();
        work::words(8);
        Some(id)
    }

    fn record_direct_unit(&mut self, u: u32, w: &ZWitness) {
        let slot = &mut self.best_direct_unit[u as usize];
        let replace = slot.as_ref().is_none_or(|cur| w.len() < cur.len());
        if replace {
            *slot = Some(w.clone());
            work::words(w.len() as u64 + 1);
        }
    }

    fn record_export_edge(&mut self, u: u32, v: u32, cu: i8, cv: i8, witness: EdgeWitness) {
        let eid = self.edges2_for_export.len();
        self.edges2_for_export.push(StoredEdge2 {
            u,
            v,
            cu,
            cv,
            witness,
        });
        self.adj2_for_export[u as usize].push(eid);
        self.adj2_for_export[v as usize].push(eid);
        work::words(4);
    }

    /// Walk tree path `start -> stop` (`stop` must be an ancestor of `start`, e.g. `comp_root[start]`),
    /// returning `(w_path, pi)` such that `L(w_path) = +1 * D(start) - pi * D(stop)`.
    fn path_until(&self, start: u32, stop: u32) -> Option<(ZWitness, i8)> {
        let mut cur = start;
        let mut pi = 1i8;
        let mut acc = Vec::new();
        let mut single_buf = [(0usize, 0i16); 1];
        while cur != stop {
            work::words(2);
            let p = self.tree_parent[cur as usize];
            debug_assert_ne!(cur, p);
            let edge = self.tree_edge[cur as usize]
                .as_ref()
                .expect("non-root vertex has tree edge");
            let step_scale = (pi * edge.coeff_child) as i16;
            let rhs = edge.witness.as_slice_buf(&mut single_buf);
            acc = merge2_z(
                &acc,
                1,
                rhs,
                step_scale,
                self.options.max_witness_terms,
                self.options.max_abs_coeff,
            )?;
            pi *= -(edge.coeff_child * edge.coeff_parent);
            cur = p;
        }
        Some((acc, pi))
    }

    fn unit_witness_at(&self, u: u32) -> Option<ZWitness> {
        let r = self.comp_root[u as usize];
        let RootAnchor::Unit(ref w1_r) = self.root_anchor[r as usize] else {
            return None;
        };
        if let Some(ref direct) = self.best_direct_unit[u as usize] {
            if direct.len() == 1 {
                work::words(2);
                return Some(direct.clone());
            }
        }
        let (w_path, pi) = self.path_until(u, r)?;
        let cand = merge2_z(
            &w_path,
            1,
            w1_r,
            pi as i16,
            self.options.max_witness_terms,
            self.options.max_abs_coeff,
        )?;
        if let Some(ref direct) = self.best_direct_unit[u as usize] {
            if direct.len() < cand.len() {
                return Some(direct.clone());
            }
        }
        Some(cand)
    }

    fn emit_checked(&mut self, w: ZWitness, out: &mut Vec<ZWitness>) -> bool {
        if w.is_empty() || w.len() > self.options.max_witness_terms {
            self.stats.witness_cap_refusals += 1;
            return false;
        }
        self.stats.emitted_relations += 1;
        self.stats.max_emitted_witness = self.stats.max_emitted_witness.max(w.len());
        out.push(w);
        true
    }

    fn enqueue_comp_hyperedges(&mut self, r: u32) {
        let watched = std::mem::take(&mut self.comp_hyper_watch[r as usize]);
        for hid in watched {
            if self.hyperedges[hid].live {
                self.wakeup_queue.push_back(hid);
                work::words(1);
            }
        }
    }

    fn install_unit_at_root(&mut self, r: u32, w1_r: ZWitness, emitted: &mut Vec<ZWitness>) {
        let prev = std::mem::replace(&mut self.root_anchor[r as usize], RootAnchor::None);
        match prev {
            RootAnchor::None => {
                self.root_anchor[r as usize] = RootAnchor::Unit(w1_r);
                self.stats.unit_anchors_installed += 1;
                self.enqueue_comp_hyperedges(r);
            }
            RootAnchor::Half(w2_r) => {
                if let Some(w_full) = merge2_z(
                    &w2_r,
                    1,
                    &w1_r,
                    -2,
                    self.options.max_witness_terms,
                    self.options.max_abs_coeff,
                ) {
                    if self.emit_checked(w_full, emitted) {
                        self.stats.anchored_cycles += 1;
                    }
                } else {
                    self.stats.witness_cap_refusals += 1;
                }
                self.root_anchor[r as usize] = RootAnchor::Unit(w1_r);
                self.stats.half_upgraded_to_unit += 1;
                self.stats.unit_anchors_installed += 1;
                self.enqueue_comp_hyperedges(r);
            }
            RootAnchor::Unit(cur_w1) => {
                if let Some(w_full) = merge2_z(
                    &w1_r,
                    1,
                    &cur_w1,
                    -1,
                    self.options.max_witness_terms,
                    self.options.max_abs_coeff,
                ) {
                    if self.emit_checked(w_full, emitted) {
                        self.stats.anchored_cycles += 1;
                    }
                } else {
                    self.stats.witness_cap_refusals += 1;
                }
                if w1_r.len() < cur_w1.len() {
                    self.root_anchor[r as usize] = RootAnchor::Unit(w1_r);
                    self.stats.anchor_relaxations += 1;
                } else {
                    self.root_anchor[r as usize] = RootAnchor::Unit(cur_w1);
                }
            }
        }
    }

    fn install_half_at_root(&mut self, r: u32, w2_r: ZWitness, emitted: &mut Vec<ZWitness>) {
        let prev = std::mem::replace(&mut self.root_anchor[r as usize], RootAnchor::None);
        match prev {
            RootAnchor::None => {
                self.root_anchor[r as usize] = RootAnchor::Half(w2_r);
                self.stats.half_anchors_installed += 1;
            }
            RootAnchor::Half(cur_w2) => {
                if let Some(w_full) = merge2_z(
                    &w2_r,
                    1,
                    &cur_w2,
                    -1,
                    self.options.max_witness_terms,
                    self.options.max_abs_coeff,
                ) {
                    if self.emit_checked(w_full, emitted) {
                        self.stats.anchored_cycles += 1;
                    }
                } else {
                    self.stats.witness_cap_refusals += 1;
                }
                if w2_r.len() < cur_w2.len() {
                    self.root_anchor[r as usize] = RootAnchor::Half(w2_r);
                    self.stats.anchor_relaxations += 1;
                } else {
                    self.root_anchor[r as usize] = RootAnchor::Half(cur_w2);
                }
            }
            RootAnchor::Unit(cur_w1) => {
                if let Some(w_full) = merge2_z(
                    &w2_r,
                    1,
                    &cur_w1,
                    -2,
                    self.options.max_witness_terms,
                    self.options.max_abs_coeff,
                ) {
                    if self.emit_checked(w_full, emitted) {
                        self.stats.anchored_cycles += 1;
                    }
                } else {
                    self.stats.witness_cap_refusals += 1;
                }
                self.root_anchor[r as usize] = RootAnchor::Unit(cur_w1);
            }
        }
    }

    fn chain_to_root(&self, mut v: u32) -> Vec<u32> {
        let mut out = vec![v];
        while self.tree_parent[v as usize] != v {
            v = self.tree_parent[v as usize];
            out.push(v);
            work::words(1);
        }
        out
    }

    fn lca_reduce_same_comp(
        &mut self,
        w_edge: &[(usize, i16)],
        u: u32,
        a: i8,
        v: u32,
        b: i8,
    ) -> LcaOutcome {
        let cu = self.chain_to_root(u);
        let cv = self.chain_to_root(v);
        debug_assert_eq!(cu.last(), cv.last());
        let r = *cu.last().expect("nonempty chain");
        let mut shared = 0usize;
        while shared < cu.len()
            && shared < cv.len()
            && cu[cu.len() - 1 - shared] == cv[cv.len() - 1 - shared]
        {
            shared += 1;
            work::words(1);
        }
        let lca = cu[cu.len() - shared];
        if shared > 1 {
            self.stats.lca_shared_steps_saved += (shared - 1) as u64;
        }
        let Some((w_u, pi_u)) = self.path_until(u, lca) else {
            return LcaOutcome::CapRefused;
        };
        let Some((w_v, pi_v)) = self.path_until(v, lca) else {
            return LcaOutcome::CapRefused;
        };
        let Some(t1) = merge2_z(
            w_edge,
            1,
            &w_u,
            -(a as i16),
            self.options.max_witness_terms,
            self.options.max_abs_coeff,
        ) else {
            return LcaOutcome::CapRefused;
        };
        let Some(w_at_lca) = merge2_z(
            &t1,
            1,
            &w_v,
            -(b as i16),
            self.options.max_witness_terms,
            self.options.max_abs_coeff,
        ) else {
            return LcaOutcome::CapRefused;
        };
        let sum_lca = (a * pi_u) as i16 + (b * pi_v) as i16;
        if sum_lca == 0 {
            LcaOutcome::BalancedCycle(w_at_lca)
        } else {
            let s_lca = a * pi_u; // in {-1, +1}; sum_lca = 2 * s_lca
            let Some((w_lca_r, pi_lca)) = self.path_until(lca, r) else {
                return LcaOutcome::CapRefused;
            };
            let Some(w_raw) = merge2_z(
                &w_at_lca,
                1,
                &w_lca_r,
                -(2 * s_lca as i16),
                self.options.max_witness_terms,
                self.options.max_abs_coeff,
            ) else {
                return LcaOutcome::CapRefused;
            };
            let tau = (s_lca * pi_lca) as i16;
            match scale_z(
                &w_raw,
                tau,
                self.options.max_witness_terms,
                self.options.max_abs_coeff,
            ) {
                Some(w_2r) => LcaOutcome::FrustratedRoot(w_2r),
                None => LcaOutcome::CapRefused,
            }
        }
    }

    /// Reroot smaller tree `r_small` at vertex `u` (`comp_root[u] == r_small`).
    fn reroot_small_tree(&mut self, u: u32) {
        let mut path = Vec::new();
        let mut cur = u;
        while self.tree_parent[cur as usize] != cur {
            let p = self.tree_parent[cur as usize];
            let edge = self.tree_edge[cur as usize]
                .take()
                .expect("edge along path");
            path.push((cur, p, edge));
            cur = p;
            work::words(2);
        }
        self.tree_parent[u as usize] = u;
        self.tree_edge[u as usize] = None;
        for (child, par, mut edge) in path {
            std::mem::swap(&mut edge.coeff_child, &mut edge.coeff_parent);
            self.tree_parent[par as usize] = child;
            self.tree_edge[par as usize] = Some(edge);
            self.stats.reroot_flips += 1;
            work::words(3);
        }
    }

    fn process_1lp(&mut self, u: u32, coeff: i8, w_base: ZWitness, emitted: &mut Vec<ZWitness>) {
        let Some(w_unit_u) = scale_z(
            &w_base,
            coeff as i16,
            self.options.max_witness_terms,
            self.options.max_abs_coeff,
        ) else {
            self.stats.witness_cap_refusals += 1;
            return;
        };
        self.record_direct_unit(u, &w_unit_u);
        let r = self.comp_root[u as usize];
        let Some((w_path, pi)) = self.path_until(u, r) else {
            self.stats.witness_cap_refusals += 1;
            return;
        };
        // L(w_unit_u - w_path) = D(u) - (D(u) - pi D(r)) = pi D(r).
        let Some(w_pi_r) = merge2_z(
            &w_unit_u,
            1,
            &w_path,
            -1,
            self.options.max_witness_terms,
            self.options.max_abs_coeff,
        ) else {
            self.stats.witness_cap_refusals += 1;
            return;
        };
        let Some(w1_r) = scale_z(
            &w_pi_r,
            pi as i16,
            self.options.max_witness_terms,
            self.options.max_abs_coeff,
        ) else {
            self.stats.witness_cap_refusals += 1;
            return;
        };
        self.install_unit_at_root(r, w1_r, emitted);
    }

    fn process_2lp(
        &mut self,
        mut u: u32,
        mut a: i8,
        mut v: u32,
        mut b: i8,
        edge_witness: EdgeWitness,
        emitted: &mut Vec<ZWitness>,
    ) {
        let mut single_buf = [(0usize, 0i16); 1];
        if u == v {
            let w_slice = edge_witness.as_slice_buf(&mut single_buf);
            if a + b == 0 {
                if self.emit_checked(w_slice.to_vec(), emitted) {
                    self.stats.balanced_unanchored_cycles += 1;
                }
            } else {
                // a == b -> +/- 2 * D(u).
                let r = self.comp_root[u as usize];
                if let Some((w_path, pi)) = self.path_until(u, r) {
                    if let Some(w_raw) = merge2_z(
                        w_slice,
                        a as i16,
                        &w_path,
                        -2,
                        self.options.max_witness_terms,
                        self.options.max_abs_coeff,
                    ) {
                        if let Some(w2_r) = scale_z(
                            &w_raw,
                            pi as i16,
                            self.options.max_witness_terms,
                            self.options.max_abs_coeff,
                        ) {
                            self.install_half_at_root(r, w2_r, emitted);
                            return;
                        }
                    }
                }
                self.stats.witness_cap_refusals += 1;
            }
            return;
        }

        // Record in export multigraph so multi-source Dijkstra can traverse every 2-LP edge.
        if edge_witness.len() <= self.options.max_witness_terms {
            self.record_export_edge(u, v, a, b, edge_witness.clone());
        }

        let mut r1 = self.comp_root[u as usize];
        let mut r2 = self.comp_root[v as usize];
        if r1 == r2 {
            let w_slice = edge_witness.as_slice_buf(&mut single_buf);
            match self.lca_reduce_same_comp(w_slice, u, a, v, b) {
                LcaOutcome::BalancedCycle(w_full) => {
                    let is_unanchored = matches!(self.root_anchor[r1 as usize], RootAnchor::None);
                    if self.emit_checked(w_full, emitted) {
                        if is_unanchored {
                            self.stats.balanced_unanchored_cycles += 1;
                        } else {
                            self.stats.anchored_cycles += 1;
                        }
                    }
                }
                LcaOutcome::FrustratedRoot(w2_r) => {
                    self.install_half_at_root(r1, w2_r, emitted);
                }
                LcaOutcome::CapRefused => {
                    self.stats.witness_cap_refusals += 1;
                }
            }
            return;
        }

        // Ensure `r1` is the smaller component (`|T_1| <= |T_2|`).
        if self.members[r1 as usize].len() > self.members[r2 as usize].len() {
            std::mem::swap(&mut u, &mut v);
            std::mem::swap(&mut a, &mut b);
            std::mem::swap(&mut r1, &mut r2);
        }

        // Special case `(Unit, Unit)`: keep both shallow `Unit` trees separate, emit cycle,
        // and relax either root witness if crossing the bridge gives a shorter witness.
        if matches!(self.root_anchor[r1 as usize], RootAnchor::Unit(_))
            && matches!(self.root_anchor[r2 as usize], RootAnchor::Unit(_))
        {
            let w_slice = edge_witness.as_slice_buf(&mut single_buf);
            let (Some(w1_u), Some(w1_v)) = (self.unit_witness_at(u), self.unit_witness_at(v))
            else {
                self.stats.witness_cap_refusals += 1;
                return;
            };
            if let Some(t1) = merge2_z(
                w_slice,
                1,
                &w1_u,
                -(a as i16),
                self.options.max_witness_terms,
                self.options.max_abs_coeff,
            ) {
                if let Some(w_full) = merge2_z(
                    &t1,
                    1,
                    &w1_v,
                    -(b as i16),
                    self.options.max_witness_terms,
                    self.options.max_abs_coeff,
                ) {
                    if self.emit_checked(w_full, emitted) {
                        self.stats.anchored_cycles += 1;
                    }
                } else {
                    self.stats.witness_cap_refusals += 1;
                }
            } else {
                self.stats.witness_cap_refusals += 1;
            }
            // Bidirectional root witness relaxation.
            if let (Some((wp_v, pi_v)), RootAnchor::Unit(ref cur_r2)) =
                (self.path_until(v, r2), &self.root_anchor[r2 as usize])
            {
                if let Some(t_v) = merge2_z(
                    w_slice,
                    b as i16,
                    &w1_u,
                    -((a * b) as i16),
                    self.options.max_witness_terms,
                    self.options.max_abs_coeff,
                ) {
                    if let Some(diff) = merge2_z(
                        &t_v,
                        1,
                        &wp_v,
                        -1,
                        self.options.max_witness_terms,
                        self.options.max_abs_coeff,
                    ) {
                        if let Some(cand_r2) = scale_z(
                            &diff,
                            pi_v as i16,
                            self.options.max_witness_terms,
                            self.options.max_abs_coeff,
                        ) {
                            if cand_r2.len() < cur_r2.len() {
                                self.root_anchor[r2 as usize] = RootAnchor::Unit(cand_r2);
                                self.stats.anchor_relaxations += 1;
                            }
                        }
                    }
                }
            }
            if let (Some((wp_u, pi_u)), RootAnchor::Unit(ref cur_r1)) =
                (self.path_until(u, r1), &self.root_anchor[r1 as usize])
            {
                if let Some(t_u) = merge2_z(
                    w_slice,
                    a as i16,
                    &w1_v,
                    -((a * b) as i16),
                    self.options.max_witness_terms,
                    self.options.max_abs_coeff,
                ) {
                    if let Some(diff) = merge2_z(
                        &t_u,
                        1,
                        &wp_u,
                        -1,
                        self.options.max_witness_terms,
                        self.options.max_abs_coeff,
                    ) {
                        if let Some(cand_r1) = scale_z(
                            &diff,
                            pi_u as i16,
                            self.options.max_witness_terms,
                            self.options.max_abs_coeff,
                        ) {
                            if cand_r1.len() < cur_r1.len() {
                                self.root_anchor[r1 as usize] = RootAnchor::Unit(cand_r1);
                                self.stats.anchor_relaxations += 1;
                            }
                        }
                    }
                }
            }
            return;
        }

        if self.stats.tree_edges >= self.options.max_tree_edges {
            self.stats.capacity_refusals += 1;
            return;
        }

        // Translate `r1`'s anchor (if any) across `(u, v)` into an anchor at `r2` BEFORE flipping `r1`.
        let w_slice = edge_witness.as_slice_buf(&mut single_buf);
        let anc_r1 = std::mem::replace(&mut self.root_anchor[r1 as usize], RootAnchor::None);
        let translated_for_r2: Option<(bool, ZWitness)> = match anc_r1 {
            RootAnchor::None => None,
            RootAnchor::Unit(w1_r1) => {
                let mut ok = None;
                if let (Some((wp_u, pi_u)), Some((wp_v, pi_v))) =
                    (self.path_until(u, r1), self.path_until(v, r2))
                {
                    if let Some(w1_u) = merge2_z(
                        &wp_u,
                        1,
                        &w1_r1,
                        pi_u as i16,
                        self.options.max_witness_terms,
                        self.options.max_abs_coeff,
                    ) {
                        if let Some(w1_v) = merge2_z(
                            w_slice,
                            b as i16,
                            &w1_u,
                            -((a * b) as i16),
                            self.options.max_witness_terms,
                            self.options.max_abs_coeff,
                        ) {
                            if let Some(diff) = merge2_z(
                                &w1_v,
                                1,
                                &wp_v,
                                -1,
                                self.options.max_witness_terms,
                                self.options.max_abs_coeff,
                            ) {
                                if let Some(cand_r2) = scale_z(
                                    &diff,
                                    pi_v as i16,
                                    self.options.max_witness_terms,
                                    self.options.max_abs_coeff,
                                ) {
                                    ok = Some((true, cand_r2));
                                }
                            }
                        }
                    }
                }
                if ok.is_none() {
                    // Restore and refuse without merging if witness cap exceeded.
                    self.root_anchor[r1 as usize] = RootAnchor::Unit(w1_r1);
                    self.stats.witness_cap_refusals += 1;
                    return;
                }
                ok
            }
            RootAnchor::Half(w2_r1) => {
                let mut ok = None;
                if let (Some((wp_u, pi_u)), Some((wp_v, pi_v))) =
                    (self.path_until(u, r1), self.path_until(v, r2))
                {
                    if let Some(w2_u) = merge2_z(
                        &wp_u,
                        2,
                        &w2_r1,
                        pi_u as i16,
                        self.options.max_witness_terms,
                        self.options.max_abs_coeff,
                    ) {
                        if let Some(w2_v) = merge2_z(
                            w_slice,
                            (2 * b) as i16,
                            &w2_u,
                            -((a * b) as i16),
                            self.options.max_witness_terms,
                            self.options.max_abs_coeff,
                        ) {
                            if let Some(diff) = merge2_z(
                                &w2_v,
                                1,
                                &wp_v,
                                -2,
                                self.options.max_witness_terms,
                                self.options.max_abs_coeff,
                            ) {
                                if let Some(cand_r2) = scale_z(
                                    &diff,
                                    pi_v as i16,
                                    self.options.max_witness_terms,
                                    self.options.max_abs_coeff,
                                ) {
                                    ok = Some((false, cand_r2));
                                }
                            }
                        }
                    }
                }
                if ok.is_none() {
                    self.root_anchor[r1 as usize] = RootAnchor::Half(w2_r1);
                    self.stats.witness_cap_refusals += 1;
                    return;
                }
                ok
            }
        };

        // Reroot `r1` at `u` and attach `u -> v` into `r2`.
        self.reroot_small_tree(u);
        self.tree_parent[u as usize] = v;
        self.tree_edge[u as usize] = Some(TreeEdge {
            witness: edge_witness,
            coeff_child: a,
            coeff_parent: b,
        });
        self.stats.tree_edges += 1;

        let moved_members = std::mem::take(&mut self.members[r1 as usize]);
        for &m in &moved_members {
            self.comp_root[m as usize] = r2;
            work::words(1);
        }
        self.members[r2 as usize].extend(moved_members);

        let moved_watches = std::mem::take(&mut self.comp_hyper_watch[r1 as usize]);
        let r2_is_unit = matches!(self.root_anchor[r2 as usize], RootAnchor::Unit(_));
        for hid in moved_watches {
            if !self.hyperedges[hid].live {
                continue;
            }
            if r2_is_unit {
                self.wakeup_queue.push_back(hid);
            } else {
                // Also wake if two endpoints of `hid` now share component `r2`!
                let ts = self.hyperedges[hid].terms;
                let c0 = self.comp_root[ts[0].0 as usize];
                let c1 = self.comp_root[ts[1].0 as usize];
                let c2 = self.comp_root[ts[2].0 as usize];
                if c0 == c1 || c0 == c2 || c1 == c2 {
                    self.wakeup_queue.push_back(hid);
                } else {
                    self.comp_hyper_watch[r2 as usize].push(hid);
                }
            }
            work::words(2);
        }

        if let Some((is_unit, cand_w)) = translated_for_r2 {
            let was_half_r2 = matches!(self.root_anchor[r2 as usize], RootAnchor::Half(_));
            if is_unit {
                self.install_unit_at_root(r2, cand_w, emitted);
            } else {
                if was_half_r2 {
                    self.stats.half_bridge_cycles += 1;
                }
                self.install_half_at_root(r2, cand_w, emitted);
            }
        }
    }

    fn process_3lp_terms(
        &mut self,
        mut w_base: ZWitness,
        raw_terms: [(u32, i8); 3],
        allow_park: bool,
        emitted: &mut Vec<ZWitness>,
    ) {
        // Step 1: Eliminate any endpoint whose component is already `Unit`-anchored.
        let mut rem = Vec::<(u32, i8)>::with_capacity(3);
        for (u, c) in raw_terms {
            if let Some(w1_u) = self.unit_witness_at(u) {
                let Some(next_w) = merge2_z(
                    &w_base,
                    1,
                    &w1_u,
                    -(c as i16),
                    self.options.max_witness_terms,
                    self.options.max_abs_coeff,
                ) else {
                    self.stats.witness_cap_refusals += 1;
                    return;
                };
                w_base = next_w;
            } else {
                rem.push((u, c));
            }
        }

        // Step 2: If all 3 remain unanchored, check if any pair lies in the same component
        // with opposite LCA parity (`BalancedCycle`), canceling that pair into a `1-LP` unit anchor!
        if rem.len() == 3 {
            let pairs = [(0usize, 1usize, 2usize), (0, 2, 1), (1, 2, 0)];
            for (i, j, k) in pairs {
                let (ui, ci) = rem[i];
                let (uj, cj) = rem[j];
                if self.comp_root[ui as usize] == self.comp_root[uj as usize] {
                    if let LcaOutcome::BalancedCycle(w_after_pair) =
                        self.lca_reduce_same_comp(&w_base, ui, ci, uj, cj)
                    {
                        let (uk, ck) = rem[k];
                        self.stats.hyper3_reduced_to_1lp += 1;
                        self.process_1lp(uk, ck, w_after_pair, emitted);
                        return;
                    }
                }
            }
        }

        match rem.as_slice() {
            [] => {
                if self.emit_checked(w_base, emitted) {
                    self.stats.hyper3_reduced_to_full += 1;
                }
            }
            &[(u, c)] => {
                self.stats.hyper3_reduced_to_1lp += 1;
                self.process_1lp(u, c, w_base, emitted);
            }
            &[(u, a), (v, b)] => {
                self.stats.hyper3_reduced_to_2lp += 1;
                self.process_2lp(u, a, v, b, EdgeWitness::Multi(w_base), emitted);
            }
            &[t0, t1, t2] => {
                if !allow_park {
                    return;
                }
                if self.hyperedges.len() >= self.options.max_hyperedges {
                    self.stats.capacity_refusals += 1;
                    return;
                }
                let hid = self.hyperedges.len();
                self.hyperedges.push(HyperEdge3 {
                    live: true,
                    base_witness: w_base,
                    terms: [t0, t1, t2],
                });
                let mut seen_roots = [u32::MAX; 3];
                let mut nr = 0usize;
                for (u, _) in [t0, t1, t2] {
                    let r = self.comp_root[u as usize];
                    if !seen_roots[..nr].contains(&r) {
                        seen_roots[nr] = r;
                        nr += 1;
                        self.comp_hyper_watch[r as usize].push(hid);
                    }
                }
                self.stats.hyper3_parked += 1;
                work::words(6);
            }
            _ => unreachable!(),
        }
    }

    fn drain_wakeups(&mut self, emitted: &mut Vec<ZWitness>) {
        while let Some(hid) = self.wakeup_queue.pop_front() {
            if !self.hyperedges[hid].live {
                continue;
            }
            let ts = self.hyperedges[hid].terms;
            let any_unit = ts.iter().any(|&(u, _)| {
                matches!(
                    self.root_anchor[self.comp_root[u as usize] as usize],
                    RootAnchor::Unit(_)
                )
            });
            let c0 = self.comp_root[ts[0].0 as usize];
            let c1 = self.comp_root[ts[1].0 as usize];
            let c2 = self.comp_root[ts[2].0 as usize];
            let has_pair = c0 == c1 || c0 == c2 || c1 == c2;
            if !any_unit && !has_pair {
                continue;
            }
            self.hyperedges[hid].live = false;
            self.stats.hyper3_woken += 1;
            let w_base = std::mem::take(&mut self.hyperedges[hid].base_witness);
            // If `has_pair` was frustrated rather than balanced and `any_unit` is false,
            // allow re-parking on the remaining roots only when `any_unit` was false -> to avoid infinite re-park,
            // only re-park if `any_unit` is false AND don't re-wake until a `Unit` arrives!
            if any_unit {
                self.process_3lp_terms(w_base, ts, false, emitted);
            } else {
                // Try balanced pair cancellation; if frustrated, keep live on current distinct roots for future Unit!
                let before_emitted = self.stats.hyper3_reduced_to_1lp;
                self.process_3lp_terms(w_base.clone(), ts, false, emitted);
                if self.stats.hyper3_reduced_to_1lp == before_emitted {
                    self.hyperedges[hid].live = true;
                    self.hyperedges[hid].base_witness = w_base;
                    let mut seen = [u32::MAX; 3];
                    let mut nr = 0usize;
                    for (u, _) in ts {
                        let r = self.comp_root[u as usize];
                        if !seen[..nr].contains(&r) {
                            seen[nr] = r;
                            nr += 1;
                            self.comp_hyper_watch[r as usize].push(hid);
                        }
                    }
                }
            }
        }
    }

    /// Accept one consecutive source `source` with `0..=3` unit-coefficient large quadratic terms.
    /// Combine duplicate classes in `large` automatically before matroid admission.
    /// Returns all newly closed complete `Z`-relation witnesses (`large part == 0`).
    pub fn accept(
        &mut self,
        source: usize,
        large: &[LargeTerm],
    ) -> Result<Vec<ZWitness>, &'static str> {
        if source != self.stats.sources_seen || source >= self.options.max_sources {
            return Err("consecutive source id within max_sources required");
        }
        if large.len() > 3 || large.iter().any(|t| ![-1, 1].contains(&t.coefficient)) {
            return Err("arity <= 3 and unit coefficients {-1, +1} required");
        }
        self.stats.sources_seen += 1;
        work::words(3 + 4 * large.len() as u64);

        // Merge any duplicate classes inside `large`.
        let mut combined = Vec::<(Class, i8)>::with_capacity(large.len());
        for term in large {
            if let Some(pos) = combined.iter().position(|(c, _)| *c == term.class) {
                combined[pos].1 += term.coefficient;
            } else {
                combined.push((term.class, term.coefficient));
            }
        }
        combined.retain(|&(_, c)| c != 0);

        let mut emitted = Vec::new();
        match combined.as_slice() {
            [] => {
                self.stats.full_zero_lp += 1;
                self.emit_checked(vec![(source, 1)], &mut emitted);
            }
            &[(c0, a0)] => {
                let Some(u0) = self.intern_class(c0) else {
                    return Ok(emitted);
                };
                if a0.abs() == 1 {
                    self.process_1lp(u0, a0, vec![(source, 1)], &mut emitted);
                } else if a0.abs() == 2 {
                    let sign = a0.signum();
                    self.process_2lp(
                        u0,
                        sign,
                        u0,
                        sign,
                        EdgeWitness::Single(source),
                        &mut emitted,
                    );
                }
            }
            &[(c0, a0), (c1, a1)] => {
                let (Some(u0), Some(u1)) = (self.intern_class(c0), self.intern_class(c1)) else {
                    return Ok(emitted);
                };
                if a0.abs() == 1 && a1.abs() == 1 {
                    self.process_2lp(u0, a0, u1, a1, EdgeWitness::Single(source), &mut emitted);
                } else {
                    // e.g. `(2 * D(u0) + 1 * D(u1))` from `[u0, u0, u1]` in a 3-LP source!
                    self.stats.hyper3_received += 1;
                    let (u_dup, s_dup, u_one, s_one) = if a0.abs() == 2 {
                        (u0, a0.signum(), u1, a1)
                    } else {
                        (u1, a1.signum(), u0, a0)
                    };
                    self.process_3lp_terms(
                        vec![(source, 1)],
                        [(u_dup, s_dup), (u_dup, s_dup), (u_one, s_one)],
                        true,
                        &mut emitted,
                    );
                }
            }
            &[(c0, a0), (c1, a1), (c2, a2)] => {
                self.stats.hyper3_received += 1;
                let (Some(u0), Some(u1), Some(u2)) = (
                    self.intern_class(c0),
                    self.intern_class(c1),
                    self.intern_class(c2),
                ) else {
                    return Ok(emitted);
                };
                self.process_3lp_terms(
                    vec![(source, 1)],
                    [(u0, a0), (u1, a1), (u2, a2)],
                    true,
                    &mut emitted,
                );
            }
            _ => unreachable!(),
        }

        self.drain_wakeups(&mut emitted);
        Ok(emitted)
    }

    /// Multi-source shortest-witness Dijkstra export for Phase-3 multi-pattern descent:
    /// 1. Seeds a min-length priority queue from ALL direct `1-LP` vertices AND all `Unit`
    ///    component roots simultaneously, relaxing across all retained `2-LP` multigraph
    ///    edges so every reachable vertex gets a minimum-weight `scale = 1` witness.
    /// 2. Propagates `scale = 2` witnesses across all `Half`-anchored components.
    pub fn export_anchor_table(&self) -> (AnchorTable, usize, usize) {
        let v_count = self.id_to_class.len();
        let mut best_unit: Vec<Option<ZWitness>> = vec![None; v_count];
        let mut pq = BinaryHeap::<Reverse<(usize, u32)>>::new();

        for u in 0..v_count {
            if let Some(ref w) = self.best_direct_unit[u] {
                if w.len() <= self.options.max_witness_terms {
                    best_unit[u] = Some(w.clone());
                    pq.push(Reverse((w.len(), u as u32)));
                    work::words(2);
                }
            }
        }
        for r in 0..v_count {
            if self.comp_root[r] == r as u32 {
                if let RootAnchor::Unit(ref wr) = self.root_anchor[r] {
                    let replace = best_unit[r].as_ref().is_none_or(|cur| wr.len() < cur.len());
                    if replace && wr.len() <= self.options.max_witness_terms {
                        best_unit[r] = Some(wr.clone());
                        pq.push(Reverse((wr.len(), r as u32)));
                        work::words(2);
                    }
                }
            }
        }

        let mut single_buf = [(0usize, 0i16); 1];
        while let Some(Reverse((wlen, u))) = pq.pop() {
            let Some(ref wu) = best_unit[u as usize] else {
                continue;
            };
            if wu.len() < wlen {
                continue;
            }
            let wu_clone = wu.clone();
            for &eid in &self.adj2_for_export[u as usize] {
                let e = &self.edges2_for_export[eid];
                let (nb, cu, cnb) = if e.u == u {
                    (e.v, e.cu, e.cv)
                } else {
                    (e.u, e.cv, e.cu)
                };
                let target_bound = best_unit[nb as usize]
                    .as_ref()
                    .map_or(self.options.max_witness_terms + 1, |cur| cur.len());
                if wu_clone.len() + 1 >= target_bound && matches!(e.witness, EdgeWitness::Single(_))
                {
                    continue;
                }
                let ew_slice = e.witness.as_slice_buf(&mut single_buf);
                if let Some(cand_nb) = merge2_z(
                    ew_slice,
                    cnb as i16,
                    &wu_clone,
                    -((cu * cnb) as i16),
                    self.options.max_witness_terms,
                    self.options.max_abs_coeff,
                ) {
                    if cand_nb.len() < target_bound
                        && cand_nb.len() <= self.options.max_witness_terms
                    {
                        let len_nb = cand_nb.len();
                        best_unit[nb as usize] = Some(cand_nb);
                        pq.push(Reverse((len_nb, nb)));
                    }
                }
            }
        }

        // Fallback tree walk for any `Unit` vertex missed if an intermediate edge hit cap,
        // plus `Half` (`scale = 2`) components.
        let mut table = AnchorTable::with_capacity(v_count);
        let mut unit_count = 0usize;
        let mut half_count = 0usize;
        let mut visited_half = HashSet::<u32>::new();

        for u in 0..v_count {
            if let Some(wu) = best_unit[u].take() {
                if table.insert_or_tighten(AnchorRecord {
                    class: self.id_to_class[u],
                    witness: wu,
                    scale: 1,
                }) {
                    unit_count += 1;
                }
                continue;
            }
            let r = self.comp_root[u];
            match &self.root_anchor[r as usize] {
                RootAnchor::Unit(wr) => {
                    if let Some((wp, pi)) = self.path_until(u as u32, r) {
                        if let Some(wu) = merge2_z(
                            &wp,
                            1,
                            wr,
                            pi as i16,
                            self.options.max_witness_terms,
                            self.options.max_abs_coeff,
                        ) {
                            if table.insert_or_tighten(AnchorRecord {
                                class: self.id_to_class[u],
                                witness: wu,
                                scale: 1,
                            }) {
                                unit_count += 1;
                            }
                        }
                    }
                }
                RootAnchor::Half(w2r) => {
                    visited_half.insert(r);
                    if let Some((wp, pi)) = self.path_until(u as u32, r) {
                        if let Some(w2u) = merge2_z(
                            &wp,
                            2,
                            w2r,
                            pi as i16,
                            self.options.max_witness_terms,
                            self.options.max_abs_coeff,
                        ) {
                            if table.insert_or_tighten(AnchorRecord {
                                class: self.id_to_class[u],
                                witness: w2u,
                                scale: 2,
                            }) {
                                half_count += 1;
                            }
                        }
                    }
                }
                RootAnchor::None => {}
            }
        }
        let _ = visited_half;
        (table, unit_count, half_count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    fn term(id: u64, c: i8) -> LargeTerm {
        LargeTerm {
            class: [id, 0, 0, 0, 0, 0],
            coefficient: c,
        }
    }

    fn large_incidence(sources: &[Vec<LargeTerm>], w: &ZWitness) -> BTreeMap<Class, i64> {
        let mut out = BTreeMap::<Class, i64>::new();
        for &(sid, coeff) in w {
            for t in &sources[sid] {
                *out.entry(t.class).or_default() += (coeff as i64) * (t.coefficient as i64);
            }
        }
        out.retain(|_, v| *v != 0);
        out
    }

    #[test]
    fn unanchored_balanced_and_frustrated_cycles_harvest_exact_z_relations() {
        let mut g = SignedMatroidGraph::new(MatroidOptions::default()).unwrap();
        let mut sources = Vec::new();
        let mut push = |g: &mut SignedMatroidGraph, row: Vec<LargeTerm>| -> Vec<ZWitness> {
            let sid = sources.len();
            sources.push(row.clone());
            let out = g.accept(sid, &row).unwrap();
            for w in &out {
                assert!(
                    large_incidence(&sources, w).is_empty(),
                    "nonzero large part: {w:?}"
                );
            }
            out
        };

        // 1. Unanchored balanced 4-cycle: (0+1), (1+2), (2+3), (3+0) -> e0 - e1 + e2 - e3 == 0!
        // Note `jv_deferred_pair_graph` emits ZERO relations here without a 1-LP anchor!
        assert!(push(&mut g, vec![term(0, 1), term(1, 1)]).is_empty());
        assert!(push(&mut g, vec![term(1, 1), term(2, 1)]).is_empty());
        assert!(push(&mut g, vec![term(2, 1), term(3, 1)]).is_empty());
        let bal = push(&mut g, vec![term(3, 1), term(0, 1)]);
        assert_eq!(
            bal.len(),
            1,
            "balanced cycle must emit immediately without 1-LP anchor"
        );
        assert_eq!(g.stats().balanced_unanchored_cycles, 1);

        // 2. First frustrated odd cycle (0+2) on the same unanchored component installs Half (+2*D(r)).
        let odd1 = push(&mut g, vec![term(0, 1), term(2, 1)]);
        assert!(odd1.is_empty());
        assert_eq!(g.stats().half_anchors_installed, 1);

        // 3. Second frustrated odd cycle (1+3) in a Half component IMMEDIATELY emits a complete Z-relation!
        let odd2 = push(&mut g, vec![term(1, 1), term(3, 1)]);
        assert_eq!(odd2.len(), 1);

        // 4. Separate frustrated triangle (10+11), (11+12), (12+10) -> Half on second component,
        //    then bridge (0 + 10) between two Half components emits a complete Z-relation!
        assert!(push(&mut g, vec![term(10, 1), term(11, 1)]).is_empty());
        assert!(push(&mut g, vec![term(11, 1), term(12, 1)]).is_empty());
        assert!(push(&mut g, vec![term(12, 1), term(10, 1)]).is_empty());
        let bridge = push(&mut g, vec![term(0, 1), term(10, 1)]);
        assert_eq!(
            bridge.len(),
            1,
            "(Half, Half) bridge must emit exact Z-relation"
        );
        assert_eq!(g.stats().half_bridge_cycles, 1);

        // Export anchor table: all 7 vertices have valid scale=2 witnesses!
        let (table, unit_n, half_n) = g.export_anchor_table();
        assert_eq!(unit_n, 0);
        assert_eq!(half_n, 7);
        for v in [0u64, 1, 2, 3, 10, 11, 12] {
            let k = [v, 0, 0, 0, 0, 0];
            let rec = table.get(&k).expect("half anchor exported");
            assert_eq!(rec.scale, 2);
            assert_eq!(
                large_incidence(&sources, &rec.witness),
                BTreeMap::from([(k, 2)])
            );
        }
    }

    #[test]
    fn watched_3lp_hypergraph_cascades_and_verificaion_holds_on_random_streams() {
        for seed in 0..48u64 {
            let mut z = seed.wrapping_mul(0x9E37_79B9_7F4A_7C15).wrapping_add(1);
            let mut g = SignedMatroidGraph::new(MatroidOptions::default()).unwrap();
            let mut def =
                crate::cryptanalysis::jv_sieve::two_large_prime::deferred_pair_graph::Graph::new(
                    Default::default(),
                )
                .unwrap();
            let mut sources = Vec::new();
            let mut emitted_matroid = 0usize;
            let mut emitted_def = 0usize;

            for step in 0..320usize {
                z = z
                    .wrapping_mul(6364136223846793005)
                    .wrapping_add(1442695040888963407);
                let u = (z >> 16) % 29;
                let v = (u + 1 + ((z >> 28) % 28)) % 29;
                let w = (v + 1 + ((z >> 40) % 27)) % 29;
                let s0 = if (z & 1) == 0 { 1 } else { -1 };
                let s1 = if (z & 2) == 0 { 1 } else { -1 };
                let s2 = if (z & 4) == 0 { 1 } else { -1 };

                // On even seeds test pure <=2-LP stream vs `jv_deferred_pair_graph`;
                // on odd seeds include `3-LP` hyperedges!
                let row = if step % 37 == 0 {
                    vec![]
                } else if step % 17 == 0 {
                    vec![term(u, s0)]
                } else if (seed & 1) == 1 && step % 3 == 0 && u != w {
                    vec![term(u, s0), term(v, s1), term(w, s2)]
                } else {
                    vec![term(u, s0), term(v, s1)]
                };

                let sid = sources.len();
                sources.push(row.clone());
                let out = g.accept(sid, &row).unwrap();
                for wit in &out {
                    assert!(
                        large_incidence(&sources, wit).is_empty(),
                        "seed={seed} step={step}: nonzero large part"
                    );
                }
                emitted_matroid += out.len();

                if (seed & 1) == 0 {
                    let evs = def.accept(sid, &row).unwrap();
                    emitted_def += evs
                        .iter()
                        .filter(|e| {
                            matches!(
                                e,
                                crate::cryptanalysis::jv_sieve::two_large_prime::deferred_pair_graph::Event::Full { .. }
                            )
                        })
                        .count();
                }
            }

            if (seed & 1) == 0 {
                assert!(
                    emitted_matroid >= emitted_def,
                    "seed={seed}: SignedMatroidGraph ({emitted_matroid}) must emit >= DeferredPairGraph ({emitted_def})"
                );
            }

            let (table, _, _) = g.export_anchor_table();
            for v in 0..29u64 {
                let k = [v, 0, 0, 0, 0, 0];
                if let Some(rec) = table.get(&k) {
                    assert_eq!(
                        large_incidence(&sources, &rec.witness),
                        BTreeMap::from([(k, rec.scale as i64)]),
                        "seed={seed} v={v}: invalid exported anchor"
                    );
                }
            }
        }
    }
}
