//! Double-large-prime graph for arm D2 (study `b2lp-20261127/v1`).
//!
//! Columns of the full subfield factor base `fb` (index 0 is the 2-torsion
//! point `T`, which never appears in a row) are split into the *small base*
//! `1 ..= fs` and the *large primes* `fs+1 .. F`.  A resolved relation
//! (`ic::Row`, exact in `E`) whose large support has size 0 goes straight to
//! the linear algebra; size 1 is an edge `u — SV` to the special vertex;
//! size 2 an edge `u — w`; size 3 is discarded.
//!
//! Every relation is an affine form over `Z/ℓ`: `Σ_c e_c log(P_c) = a + b·d`.
//! The graph keeps a spanning forest (parent pointers, each with the edge's
//! relation) and a union-find over the same components.  Because the classes
//! are `±P` and the coefficients `±1, ±2`, a cycle need not cancel its large
//! primes over `Z/ℓ`: the components are those of a *signed* graph.  A
//! component is *grounded* when it contains `SV` or once one of its cycles was
//! frustrated (that cycle's combination, which expresses the root's large
//! prime in small primes, is kept as the component's grounding relation).
//! A new edge then yields one relation free of large primes exactly when
//!
//! * it closes a cycle in a grounded component, or a balanced cycle in any
//!   component, or
//! * it joins two grounded components;
//!
//! so the number of yields is `E − rank(incidence)`, i.e.
//! `E − V + #(balanced components without SV)`, and none is lost.
//!
//! Every product or addition mod ℓ is counted as `z` / `za` (as the echelon
//! does), every inversion mod ℓ `2·bits(ℓ)` `z`, every union-find step,
//! parent-pointer step, re-rooting step and column classification one `h`.
use crate::field::CF;
use crate::gen::powmod;
use crate::ic::Row;

pub const NONE: u32 = u32::MAX;

/// `Σ cols[k].1 · log(P_{cols[k].0}) = a + b·d  (mod ℓ)`, cols sorted, coefficients non-zero.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Comb {
    pub cols: Vec<(usize, u64)>,
    pub a: u64,
    pub b: u64,
}

impl Comb {
    pub fn from_row(row: &Row, ell: u64) -> Comb {
        let mut cols: Vec<(usize, u64)> = row
            .cols
            .iter()
            .map(|&(c, e)| (c, (((e % ell as i64) + ell as i64) as u64) % ell))
            .filter(|e| e.1 != 0)
            .collect();
        cols.sort_unstable();
        Comb { cols, a: row.a % ell, b: row.b % ell }
    }
    pub fn to_row(&self) -> Row {
        Row { cols: self.cols.iter().map(|&(c, e)| (c, e as i64)).collect(), a: self.a, b: self.b }
    }
    pub fn coef(&self, c: usize) -> u64 {
        match self.cols.binary_search_by_key(&c, |e| e.0) {
            Ok(k) => self.cols[k].1,
            Err(_) => 0,
        }
    }
}

#[inline]
fn mm(a: u64, b: u64, ell: u64) -> u64 {
    ((a as u128 * b as u128) % ell as u128) as u64
}

pub fn inv_mod(cf: &CF, a: u64, ell: u64) -> u64 {
    cf.zmul(2 * (64 - ell.leading_zeros()) as u64);
    powmod(a, ell - 2, ell)
}

/// `acc ← acc + λ·r` (sparse, counted: one `z` and one `za` per entry of `r`, two each for `a, b`).
pub fn axpy(cf: &CF, acc: &mut Comb, lam: u64, r: &Comb, ell: u64) {
    let mut out = Vec::with_capacity(acc.cols.len() + r.cols.len());
    let (mut i, mut j) = (0, 0);
    while i < acc.cols.len() || j < r.cols.len() {
        if j == r.cols.len() || (i < acc.cols.len() && acc.cols[i].0 < r.cols[j].0) {
            out.push(acc.cols[i]);
            i += 1;
        } else if i == acc.cols.len() || r.cols[j].0 < acc.cols[i].0 {
            out.push((r.cols[j].0, mm(lam, r.cols[j].1, ell)));
            j += 1;
        } else {
            let v = (acc.cols[i].1 + mm(lam, r.cols[j].1, ell)) % ell;
            if v != 0 {
                out.push((acc.cols[i].0, v));
            }
            i += 1;
            j += 1;
        }
    }
    cf.zmul(r.cols.len() as u64 + 2);
    cf.zadd(r.cols.len() as u64 + 2);
    acc.cols = out;
    acc.a = (acc.a + mm(lam, r.a, ell)) % ell;
    acc.b = (acc.b + mm(lam, r.b, ell)) % ell;
}

#[derive(Clone, Copy, Debug, Default)]
pub struct GStats {
    pub rows: u64,
    pub full: u64,
    pub one_lp: u64,
    pub two_lp: u64,
    pub discarded_3lp: u64,
    pub yields_cycle: u64,
    pub yields_join: u64,
    pub groundings: u64,
    pub tree_edges: u64,
    pub vertices_touched: u64,
    pub reduce_steps: u64,
    pub max_reduce_steps: u64,
    pub reroot_steps: u64,
    pub yield_weight_sum: u64,
    pub yield_weight_max: u64,
}

pub struct LpGraph {
    pub ell: u64,
    /// Small base: columns `1 ..= fs`.
    pub fs: usize,
    pub nv: usize,
    sv: u32,
    parent: Vec<u32>,
    pedge: Vec<u32>,
    uf: Vec<u32>,
    size: Vec<u32>,
    has_sv: Vec<bool>,
    ground: Vec<u32>,
    touched: Vec<bool>,
    pub rels: Vec<Comb>,
    grounds: Vec<Comb>,
    pub st: GStats,
}

impl LpGraph {
    /// `f` = number of factor-base classes (columns `0 .. f`), `fs` = small-base size.
    pub fn new(ell: u64, f: usize, fs: usize) -> Self {
        let nv = f + 1;
        LpGraph {
            ell,
            fs,
            nv,
            sv: f as u32,
            parent: vec![NONE; nv],
            pedge: vec![NONE; nv],
            uf: (0..nv as u32).collect(),
            size: vec![1; nv],
            has_sv: (0..nv).map(|v| v == f).collect(),
            ground: vec![NONE; nv],
            touched: vec![false; nv],
            rels: vec![],
            grounds: vec![],
            st: GStats::default(),
        }
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
    fn grounded(&self, root: u32) -> bool {
        self.has_sv[root as usize] || self.ground[root as usize] != NONE
    }
    /// Eliminate every large column that has a forest parent, walking up the
    /// forest; what is left are large columns that are tree roots.
    fn reduce(&mut self, cf: &CF, acc: &mut Comb) {
        let ell = self.ell;
        let mut steps = 0u64;
        loop {
            let mut pick = None;
            for &(c, v) in &acc.cols {
                if !self.is_small(c) && self.parent[c] != NONE {
                    pick = Some((c, v));
                    break;
                }
            }
            let Some((c, alpha)) = pick else { break };
            cf.hash();
            steps += 1;
            let e = self.pedge[c] as usize;
            let r = std::mem::take(&mut self.rels[e]);
            let ce = r.coef(c);
            debug_assert!(ce != 0);
            let lam = (ell - mm(alpha, inv_mod(cf, ce, ell), ell)) % ell;
            cf.zmul(1);
            axpy(cf, acc, lam, &r, ell);
            self.rels[e] = r;
            debug_assert_eq!(acc.coef(c), 0);
        }
        self.st.reduce_steps += steps;
        self.st.max_reduce_steps = self.st.max_reduce_steps.max(steps);
    }
    fn large_cols(&self, acc: &Comb) -> Vec<(usize, u64)> {
        acc.cols.iter().copied().filter(|&(c, _)| !self.is_small(c)).collect()
    }
    /// Remove the root column `r` of a grounded, SV-free component with its grounding relation.
    fn unground(&mut self, cf: &CF, acc: &mut Comb, r: usize) {
        let ell = self.ell;
        let root = self.find(cf, r as u32);
        let gi = self.ground[root as usize];
        assert!(gi != NONE, "grounded component");
        let mut g = self.grounds[gi as usize].clone();
        self.reduce(cf, &mut g);
        let gl = self.large_cols(&g);
        assert!(gl.len() == 1 && gl[0].0 == r, "grounding relation reduces to the root column");
        let alpha = acc.coef(r);
        let lam = (ell - mm(alpha, inv_mod(cf, gl[0].1, ell), ell)) % ell;
        cf.zmul(1);
        axpy(cf, acc, lam, &g, ell);
        // keep the reduced form (same relation, cheaper next time)
        self.grounds[gi as usize] = g;
    }
    fn reroot(&mut self, cf: &CF, x: u32) {
        let (mut prev, mut prev_e, mut cur) = (NONE, NONE, x);
        while cur != NONE {
            cf.hash();
            self.st.reroot_steps += 1;
            let (nxt, ne) = (self.parent[cur as usize], self.pedge[cur as usize]);
            self.parent[cur as usize] = prev;
            self.pedge[cur as usize] = prev_e;
            prev = cur;
            prev_e = ne;
            cur = nxt;
        }
    }
    fn touch(&mut self, v: u32) {
        if v != self.sv && !self.touched[v as usize] {
            self.touched[v as usize] = true;
            self.st.vertices_touched += 1;
        }
    }
    fn note_yield(&mut self, y: &Comb) {
        let w = y.cols.len() as u64;
        self.st.yield_weight_sum += w;
        self.st.yield_weight_max = self.st.yield_weight_max.max(w);
    }
    /// Feed one resolved relation; returns a relation over the small base if one is produced.
    pub fn add(&mut self, cf: &CF, row: &Row) -> Option<Comb> {
        self.st.rows += 1;
        let comb = Comb::from_row(row, self.ell);
        for _ in &comb.cols {
            cf.hash();
        }
        let large = self.large_cols(&comb);
        let (u, w) = match large.len() {
            0 => {
                self.st.full += 1;
                self.note_yield(&comb);
                return Some(comb);
            }
            1 => {
                self.st.one_lp += 1;
                (large[0].0 as u32, self.sv)
            }
            2 => {
                self.st.two_lp += 1;
                (large[0].0 as u32, large[1].0 as u32)
            }
            _ => {
                self.st.discarded_3lp += 1;
                return None;
            }
        };
        self.touch(u);
        self.touch(w);
        let e = self.rels.len() as u32;
        self.rels.push(comb.clone());
        let (ru, rw) = (self.find(cf, u), self.find(cf, w));
        if ru == rw {
            let mut acc = comb;
            self.reduce(cf, &mut acc);
            let left = self.large_cols(&acc);
            assert!(left.len() <= 1, "a cycle reduces to at most the root column");
            if left.is_empty() {
                self.st.yields_cycle += 1;
                self.note_yield(&acc);
                return Some(acc);
            }
            if self.grounded(ru) {
                self.unground(cf, &mut acc, left[0].0);
                assert!(self.large_cols(&acc).is_empty());
                self.st.yields_cycle += 1;
                self.note_yield(&acc);
                return Some(acc);
            }
            // frustrated cycle in an ungrounded component: it grounds the component.
            self.st.groundings += 1;
            self.ground[ru as usize] = self.grounds.len() as u32;
            self.grounds.push(acc);
            return None;
        }
        // two components joined by a tree edge
        let both = self.grounded(ru) && self.grounded(rw);
        let mut out = None;
        if both {
            let mut acc = comb;
            self.reduce(cf, &mut acc);
            for (c, _) in self.large_cols(&acc) {
                self.unground(cf, &mut acc, c);
            }
            assert!(self.large_cols(&acc).is_empty());
            self.st.yields_join += 1;
            self.note_yield(&acc);
            out = Some(acc);
        }
        // re-root the component without SV (else the smaller) at its endpoint
        let (x, y) = if self.has_sv[ru as usize] {
            (w, u)
        } else if self.has_sv[rw as usize] {
            (u, w)
        } else if self.size[ru as usize] <= self.size[rw as usize] {
            (u, w)
        } else {
            (w, u)
        };
        self.reroot(cf, x);
        self.parent[x as usize] = y;
        self.pedge[x as usize] = e;
        self.st.tree_edges += 1;
        // union
        cf.hash();
        let (big, small) = if self.size[ru as usize] >= self.size[rw as usize] { (ru, rw) } else { (rw, ru) };
        self.uf[small as usize] = big;
        self.size[big as usize] += self.size[small as usize];
        let sv = self.has_sv[ru as usize] || self.has_sv[rw as usize];
        self.has_sv[big as usize] = sv;
        let g = if sv {
            NONE
        } else if self.ground[big as usize] != NONE {
            self.ground[big as usize]
        } else {
            self.ground[small as usize]
        };
        self.ground[big as usize] = g;
        out
    }
    /// Components with at least one touched vertex (diagnostic, uncounted).
    pub fn census(&mut self) -> (u64, u64, u64) {
        let (mut comps, mut grounded, mut with_sv) = (0u64, 0u64, 0u64);
        let mut seen = vec![false; self.nv];
        for v in 0..self.nv {
            if !(self.touched[v] || v as u32 == self.sv) {
                continue;
            }
            let mut r = v as u32;
            while self.uf[r as usize] != r {
                r = self.uf[r as usize];
            }
            if !seen[r as usize] {
                seen[r as usize] = true;
                comps += 1;
                if self.grounded(r) {
                    grounded += 1;
                }
                if self.has_sv[r as usize] {
                    with_sv += 1;
                }
            }
        }
        (comps, grounded, with_sv)
    }
}
