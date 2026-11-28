//! Shortest-cycle (BFS) large-prime forest for arm D2S (study `b2lsp-20261127/v1`).
//!
//! Same split and signed-graph semantics as #5291's `graph.rs` (`LpGraph`):
//! columns `1 ..= fs` of the full subfield factor base are the small base,
//! the rest are large primes; a resolved relation with 0 large primes is a
//! full row, with 1 an edge to the special vertex `SV`, with 2 an edge, with
//! 3 it is discarded.  What differs is the spanning forest:
//!
//! * #5291 keeps one incrementally re-rooted forest, whose paths grew to
//!   360–650 edges at n = 54 (yield weight 81).
//! * Here relations are only *stored* while they arrive (plus a union-find
//!   that counts cycle-closing edges, an upper bound on the yields).  When
//!   the stopping rule asks for the LA, the forest is rebuilt from scratch by
//!   breadth-first search: from `SV` in `SV`'s component, from the
//!   union-find root elsewhere.  Every vertex then has a shortest path to its
//!   root, so a non-tree edge `(u, w)` closes a cycle of length at most
//!   `depth(u) + depth(w) + 1`.
//! * A non-tree edge's relation is reduced deepest column first along the
//!   BFS parent edges, so each vertex is eliminated at most once: the cost is
//!   the path to the lowest common ancestor (further up only when the cycle is
//!   frustrated).  As in `LpGraph`, a frustrated cycle in a component without
//!   `SV` grounds that component, and later leftovers at its root are removed
//!   with the grounding relation; so the yields are `E − rank` of the signed
//!   incidence matrix, the same count as `LpGraph`'s on the same relations.
//!
//! Counting (protocol "Cost unit"): every classification, union-find, BFS
//! adjacency and reduction step is one `h`; mod-ℓ arithmetic follows #5290's
//! `sparse_la.rs` convention: a product is one `z` unless an operand is ±1
//! (then it is a sign change, uncharged, and counted in `pm1_skipped`), an
//! addition one `za`, an inversion `2·bits(ℓ)` `z` (cached per build for the
//! few coefficients other than ±1).
use crate::field::CF;
use crate::graph::{inv_mod, Comb};
use crate::ic::Row;
use std::collections::{HashMap, VecDeque};

pub const NONE: u32 = u32::MAX;

#[inline(always)]
fn mm(a: u64, b: u64, ell: u64) -> u64 {
    ((a as u128 * b as u128) % ell as u128) as u64
}

#[derive(Clone, Copy, Debug, Default)]
pub struct BStats {
    pub rows: u64,
    pub full: u64,
    pub one_lp: u64,
    pub two_lp: u64,
    pub discarded_3lp: u64,
    pub cycle_edges_uf: u64,
    pub builds: u64,
    pub pm1_skipped: u64,
}

#[derive(Clone, Debug, Default)]
pub struct Build {
    pub yields_full: u64,
    pub yields_cycle: u64,
    pub groundings: u64,
    pub tree_edges: u64,
    pub comps_bfs: u64,
    pub max_depth: u64,
    pub mean_depth_used: f64,
    pub reduce_steps: u64,
    pub max_reduce_steps: u64,
    pub weight_sum: u64,
    pub weight_max: u64,
    pub cycle_len_sum: u64,
    pub cycle_len_max: u64,
    /// cycle combinations that cancel completely (0 = 0, e.g. the same
    /// decomposition stored twice): carry no information and are dropped
    /// (amendment A1: kept, they are a permanent useless kernel vector)
    pub empty_dropped: u64,
}

pub struct BfsLp {
    pub ell: u64,
    pub fs: usize,
    pub f: usize,
    sv: u32,
    pub full: Vec<Comb>,
    ends: Vec<(u32, u32)>,
    pub rels: Vec<Comb>,
    uf: Vec<u32>,
    size: Vec<u32>,
    pub st: BStats,
}

impl BfsLp {
    pub fn new(ell: u64, f: usize, fs: usize) -> Self {
        let nv = f + 1;
        BfsLp { ell, fs, f, sv: f as u32, full: vec![], ends: vec![], rels: vec![], uf: (0..nv as u32).collect(), size: vec![1; nv], st: BStats::default() }
    }
    #[inline]
    pub fn is_small(&self, c: usize) -> bool {
        c >= 1 && c <= self.fs
    }
    fn find(&mut self, cf: &CF, mut x: u32) -> u32 {
        let mut r = x;
        while self.uf[r as usize] != r {
            cf.hash();
            r = self.uf[r as usize];
        }
        while self.uf[x as usize] != r {
            cf.hash();
            let nx = self.uf[x as usize];
            self.uf[x as usize] = r;
            x = nx;
        }
        r
    }
    /// Upper bound on the yields available now: full rows plus cycle-closing edges
    /// (exact in SV's component; at most one too many per component without SV).
    pub fn y_ub(&self) -> u64 {
        self.st.full + self.st.cycle_edges_uf
    }
    /// Store one resolved relation (classification and union-find counted as `h`).
    pub fn add(&mut self, cf: &CF, row: &Row) {
        self.st.rows += 1;
        let comb = Comb::from_row(row, self.ell);
        for _ in &comb.cols {
            cf.hash();
        }
        let large: Vec<usize> = comb.cols.iter().map(|e| e.0).filter(|&c| !self.is_small(c)).collect();
        let (u, w) = match large.len() {
            0 => {
                self.st.full += 1;
                self.full.push(comb);
                return;
            }
            1 => {
                self.st.one_lp += 1;
                (large[0] as u32, self.sv)
            }
            2 => {
                self.st.two_lp += 1;
                (large[0] as u32, large[1] as u32)
            }
            _ => {
                self.st.discarded_3lp += 1;
                return;
            }
        };
        self.ends.push((u, w));
        self.rels.push(comb);
        let (ru, rw) = (self.find(cf, u), self.find(cf, w));
        cf.hash();
        if ru == rw {
            self.st.cycle_edges_uf += 1;
        } else {
            let (big, small) = if self.size[ru as usize] >= self.size[rw as usize] { (ru, rw) } else { (rw, ru) };
            self.uf[small as usize] = big;
            self.size[big as usize] += self.size[small as usize];
        }
    }

    /// Rebuild the BFS forest and return every relation over the small base:
    /// the full rows (in arrival order), then one per non-tree edge in a
    /// component that has a cycle (in edge order), groundings excepted.
    pub fn build(&mut self, cf: &CF) -> (Vec<Comb>, Build) {
        let ell = self.ell;
        self.st.builds += 1;
        let nv = self.f + 1;
        let mut b = Build::default();
        let mut out: Vec<Comb> = self.full.clone();
        b.yields_full = out.len() as u64;
        for y in &out {
            b.weight_sum += y.cols.len() as u64;
            b.weight_max = b.weight_max.max(y.cols.len() as u64);
        }
        // components with a cycle (cyclomatic number >= 1), and SV's component
        let ne = self.ends.len();
        let mut roots_e: Vec<u32> = Vec::with_capacity(ne);
        for k in 0..ne {
            let r = self.find(cf, self.ends[k].0);
            roots_e.push(r);
        }
        let mut cyc: HashMap<u32, i64> = HashMap::new();
        let mut verts: HashMap<u32, i64> = HashMap::new();
        let mut seen_v = vec![false; nv];
        for k in 0..ne {
            cf.hash();
            *cyc.entry(roots_e[k]).or_insert(0) += 1;
            for v in [self.ends[k].0, self.ends[k].1] {
                if !seen_v[v as usize] {
                    seen_v[v as usize] = true;
                    *verts.entry(roots_e[k]).or_insert(0) += 1;
                }
            }
        }
        let sv_root = if seen_v[self.sv as usize] { Some(self.find(cf, self.sv)) } else { None };
        let active = |r: u32| -> bool { Some(r) == sv_root || cyc.get(&r).copied().unwrap_or(0) >= verts.get(&r).copied().unwrap_or(0) };
        // CSR adjacency over the edges of active components
        let mut deg = vec![0u32; nv + 1];
        for k in 0..ne {
            if active(roots_e[k]) {
                cf.hash();
                deg[self.ends[k].0 as usize] += 1;
                deg[self.ends[k].1 as usize] += 1;
            }
        }
        let mut start = vec![0u32; nv + 1];
        for v in 0..nv {
            start[v + 1] = start[v] + deg[v];
        }
        let mut fill = start.clone();
        let mut adj = vec![0u32; start[nv] as usize];
        for k in 0..ne {
            if active(roots_e[k]) {
                let (u, w) = self.ends[k];
                adj[fill[u as usize] as usize] = k as u32;
                fill[u as usize] += 1;
                adj[fill[w as usize] as usize] = k as u32;
                fill[w as usize] += 1;
            }
        }
        // BFS from SV, then from each other active component's union-find root
        let mut depth = vec![NONE; nv];
        let mut pedge = vec![NONE; nv];
        let mut tree = vec![false; ne];
        let mut comp_root = vec![NONE; nv];
        let mut bfs_roots: Vec<u32> = vec![];
        if seen_v[self.sv as usize] {
            bfs_roots.push(self.sv);
        }
        let mut other: Vec<u32> = cyc.keys().copied().filter(|&r| Some(r) != sv_root && active(r)).collect();
        other.sort_unstable();
        bfs_roots.extend(other);
        let mut q = VecDeque::new();
        for &r0 in &bfs_roots {
            b.comps_bfs += 1;
            depth[r0 as usize] = 0;
            comp_root[r0 as usize] = r0;
            q.push_back(r0);
            while let Some(v) = q.pop_front() {
                for k in start[v as usize]..start[v as usize + 1] {
                    cf.hash();
                    let e = adj[k as usize] as usize;
                    let (a, c) = self.ends[e];
                    let x = if a == v { c } else { a };
                    if depth[x as usize] == NONE {
                        depth[x as usize] = depth[v as usize] + 1;
                        pedge[x as usize] = e as u32;
                        comp_root[x as usize] = r0;
                        tree[e] = true;
                        b.tree_edges += 1;
                        b.max_depth = b.max_depth.max(depth[x as usize] as u64);
                        q.push_back(x);
                    }
                }
            }
        }
        // inverses of edge coefficients other than ±1 (cached per build, counted once)
        let mut inv_cache: HashMap<u64, u64> = HashMap::new();
        let mut grounds: HashMap<u32, Comb> = HashMap::new();
        let mut depth_used = 0u64;
        for e in 0..ne {
            if tree[e] || !active(roots_e[e]) {
                continue;
            }
            let (u, w) = self.ends[e];
            let r0 = comp_root[u as usize];
            debug_assert!(r0 != NONE && comp_root[w as usize] == r0);
            depth_used += depth[u as usize] as u64 + depth[w as usize] as u64;
            let clen = depth[u as usize] as u64 + depth[w as usize] as u64 + 1;
            let mut acc = self.rels[e].clone();
            let steps = self.reduce(cf, &mut acc, &depth, &pedge, &mut inv_cache);
            b.reduce_steps += steps;
            b.max_reduce_steps = b.max_reduce_steps.max(steps);
            let left: Vec<(usize, u64)> = acc.cols.iter().copied().filter(|&(c, _)| !self.is_small(c)).collect();
            if !left.is_empty() {
                assert!(left.len() == 1 && left[0].0 as u32 == r0 && r0 != self.sv, "a cycle reduces to at most the root column");
                match grounds.get(&r0) {
                    None => {
                        b.groundings += 1;
                        grounds.insert(r0, acc);
                        continue;
                    }
                    Some(g) => {
                        let gr = g.coef(r0 as usize);
                        let ig = self.inv_small(cf, gr, &mut inv_cache);
                        let lam = (ell - self.cmul(cf, left[0].1, ig)) % ell;
                        let g = g.clone();
                        self.axpy(cf, &mut acc, lam, &g);
                        assert!(acc.cols.iter().all(|&(c, _)| self.is_small(c)));
                    }
                }
            }
            cf.hash();
            if acc.cols.is_empty() {
                debug_assert!(acc.b == 0 && acc.a == 0, "an empty combination is 0 = 0");
                b.empty_dropped += 1;
                continue;
            }
            b.yields_cycle += 1;
            b.cycle_len_sum += clen;
            b.cycle_len_max = b.cycle_len_max.max(clen);
            b.weight_sum += acc.cols.len() as u64;
            b.weight_max = b.weight_max.max(acc.cols.len() as u64);
            out.push(acc);
        }
        b.mean_depth_used = depth_used as f64 / b.yields_cycle.max(1) as f64 / 2.0;
        (out, b)
    }

    fn inv_small(&mut self, cf: &CF, c: u64, cache: &mut HashMap<u64, u64>) -> u64 {
        let ell = self.ell;
        if c == 1 || c == ell - 1 {
            return c; // ±1 is its own inverse (no arithmetic)
        }
        *cache.entry(c).or_insert_with(|| inv_mod(cf, c, ell))
    }
    #[inline]
    fn cmul(&mut self, cf: &CF, a: u64, b: u64) -> u64 {
        let ell = self.ell;
        if a == 1 || b == 1 || a == ell - 1 || b == ell - 1 {
            self.st.pm1_skipped += 1;
        } else {
            cf.zmul(1);
        }
        mm(a, b, ell)
    }
    /// `acc ← acc + λ·r` with the sparse-LA charging rule.
    fn axpy(&mut self, cf: &CF, acc: &mut Comb, lam: u64, r: &Comb) {
        let ell = self.ell;
        let mut out = Vec::with_capacity(acc.cols.len() + r.cols.len());
        let (mut i, mut j) = (0, 0);
        while i < acc.cols.len() || j < r.cols.len() {
            if j == r.cols.len() || (i < acc.cols.len() && acc.cols[i].0 < r.cols[j].0) {
                out.push(acc.cols[i]);
                i += 1;
            } else if i == acc.cols.len() || r.cols[j].0 < acc.cols[i].0 {
                let t = self.cmul(cf, lam, r.cols[j].1);
                out.push((r.cols[j].0, t));
                j += 1;
            } else {
                let t = self.cmul(cf, lam, r.cols[j].1);
                cf.zadd(1);
                let v = (acc.cols[i].1 + t) % ell;
                if v != 0 {
                    out.push((acc.cols[i].0, v));
                }
                i += 1;
                j += 1;
            }
        }
        let ta = self.cmul(cf, lam, r.a);
        let tb = self.cmul(cf, lam, r.b);
        cf.zadd(2);
        acc.cols = out;
        acc.a = (acc.a + ta) % ell;
        acc.b = (acc.b + tb) % ell;
    }
    /// Eliminate large columns deepest first along BFS parent edges; returns the step count.
    fn reduce(&mut self, cf: &CF, acc: &mut Comb, depth: &[u32], pedge: &[u32], cache: &mut HashMap<u64, u64>) -> u64 {
        let ell = self.ell;
        let mut steps = 0u64;
        loop {
            let mut pick: Option<(usize, u64, u32)> = None;
            for &(c, v) in &acc.cols {
                if self.is_small(c) {
                    continue;
                }
                let dc = depth[c];
                if dc == 0 || dc == NONE {
                    continue;
                }
                if pick.map(|p| dc > p.2 || (dc == p.2 && c > p.0)).unwrap_or(true) {
                    pick = Some((c, v, dc));
                }
            }
            let Some((c, alpha, _)) = pick else { break };
            cf.hash();
            steps += 1;
            let e = pedge[c] as usize;
            let r = std::mem::take(&mut self.rels[e]);
            let ce = r.coef(c);
            debug_assert!(ce != 0);
            let ic = self.inv_small(cf, ce, cache);
            let lam = (ell - self.cmul(cf, alpha, ic)) % ell;
            self.axpy(cf, acc, lam, &r);
            self.rels[e] = r;
            debug_assert_eq!(acc.coef(c), 0);
        }
        steps
    }
}
