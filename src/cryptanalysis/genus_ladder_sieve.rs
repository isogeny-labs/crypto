//! # Sieved relation search on hyperelliptic Jacobians (smoke instrument)
//!
//! Every figure this module produces is `[pcc:unverified]` and a SMOKE
//! figure (small instances, no frozen protocol).  Notes and numbers:
//! `research/hecdlp_sieve_20261116/NOTES.md`.
//!
//! The genus ladder (`genus_ladder::run_ic_ext`) finds relations by a walk in
//! the Jacobian: one Cantor step, then a splitting test of the degree-`g`
//! divisor, per candidate.  This module finds them by **sieving** a
//! one-parameter family of functions `y - v(x)` and performs no group
//! operation in the relation phase.
//!
//! ## The family
//!
//! Let `C : y² = f(x)`, `deg f = 2g + 1`, `f` monic, and let `(u, v)` be a
//! Mumford pair with `u` monic of degree `g`, `deg v < g`, `v² ≡ f (mod u)`.
//! For `t ∈ F_p` put `v_t = v + t·u` (degree `g`).  The function `y - v_t`
//! has a pole of order `2g + 1` at infinity (`y` does, `v_t` has pole order
//! `2g`), so `2g + 1` zeros: the points of the divisor `(u, v)` and the
//! points `(r, v_t(r))` for the roots `r` of
//!
//! ```text
//! R_t(x) = (f - v_t²)/u = R_0(x) - 2t·v(x) - t²·u(x),   R_0 = (f - v²)/u,
//! ```
//!
//! monic of degree `g + 1`.  Hence in the Jacobian
//! `(u, v) + Σ_j ([Q_j] − [∞]) = 0`.
//!
//! * **Relations among factor-base places** use `(u, v)` = `g` factor-base
//!   points `(x_i, ±y_i)` (a *family*; `u = Π(x − x_i)`, `v` the Lagrange
//!   interpolant).  Then `Σ_i ε_i ℓ(x_i) + Σ_j ε_j ℓ(r_j) = 0`: `2g + 1`
//!   places, no target, right-hand side 0.
//! * **The sieve.**  For a factor-base abscissa `x` (not a root of `u`) and
//!   canonical `y_x`, `R_t(x) = 0` iff `v_t(x) = ±y_x` iff
//!   `t = (±y_x − v(x))/u(x)`: two values of `t`, one batched inversion for
//!   the whole base.  A counter array over `t ∈ F_p` collects the hits; a
//!   `t` with `g + 1 − lp` hits is a relation with `lp` large primes.  The
//!   cofactor of degree `lp ≤ 2` is found from the two top coefficients of
//!   `R_t` (`e_1 = t² − q_g`, `e_2 = q_{g−1} − 2t v_{g−1} − t² u_{g−1}`),
//!   a square-root table and no polynomial exponentiation.  There is no
//!   splitting test.
//! * **Individual logarithm** of the target: `(u, v) = T + aG` (reduced,
//!   degree `g`): sieve for a `t` whose `R_t` has all `g + 1` roots in the
//!   base; `a` steps by one Cantor addition of `G` until one is found.  Two
//!   such rows (different `a`) fix the scale of the homogeneous relation
//!   system and the unknown `t`.  Their group operations are counted.
//!
//! The rows go through `ic_control::{build_rows, peel, wiedemann_coord}`
//! unchanged.  `Rel` holds five base entries and two vertices, so a family's
//! `g` fixed points enter through one family vertex `w_F` and a defining row
//! `Σ ε_i ℓ(x_i) − ℓ(w_F) = 0`; a relation with two true large primes at
//! `g = 3` (five entries) is written directly.  At `g = 4` two large primes
//! do not fit (`2g − 1 = 7 > 5`): the effective `lp_max` is 1.
//!
//! ## What is counted
//!
//! Every `F_p` multiplication and addition of the base table, the families,
//! the sieve (each counter-array update is charged three adds: increment,
//! link, reset), the cofactors and the individual-log rows (`rel`, `ind`),
//! the linear algebra (`la`, as `run_ic_ext`), and every group operation of
//! the individual-log walk and of the certificate.  Not counted: look-ups
//! in the character and square-root tables (the baseline does not count its
//! `chi` look-ups either), loop control.

use std::collections::HashMap;

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

use super::chained_transfer::ic_control::{
    addm, build_rows, mulmod, pa, peel, pm, ppow, ps, subm, wiedemann, wiedemann_coord, Dsu, Nat,
    Rel, REL_FB,
};
use super::genus_ladder::{Instance, JDiv};
use super::ic_boundary::{CountedGroup, GroupOps};

const NONE: u32 = u32::MAX;

/// Parameters of [`run_sieve_ic`].
#[derive(Clone, Copy, Debug)]
pub struct SieveParams {
    /// Factor-base columns (abscissae with `f(x)` a non-zero square).
    pub nb: usize,
    pub seed: u64,
    /// Large primes per relation, 0 to 2 (2 only where it fits `Rel`).
    pub lp_max: usize,
    /// Families sieved before giving up.
    pub max_families: u64,
    /// Individual-log rows made before the first solve (at least 2).
    pub ind_rows: usize,
    /// Families (one Cantor addition each) one individual-log row may cost.
    pub ind_max_families: u64,
    /// Re-check every relation by group arithmetic (tests only).
    pub verify_relations: bool,
}

impl SieveParams {
    pub fn new(nb: usize, seed: u64, lp_max: usize) -> SieveParams {
        SieveParams {
            nb,
            seed,
            lp_max,
            max_families: 200_000,
            ind_rows: 2,
            ind_max_families: 200_000,
            verify_relations: false,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct SieveResult {
    pub p: u64,
    pub genus: u32,
    pub lp_max_eff: usize,
    pub recovered: Option<u64>,
    pub certified: bool,
    pub exhausted: bool,
    /// Group operations: individual-log walk plus certificates.
    pub group: GroupOps,
    pub ind_group_ops: u64,
    pub verify_ops: u64,
    /// `F_p` work inside those group operations (certificate cross-check and
    /// test verification taken out).
    pub cantor_mul: u64,
    pub cantor_inv: u64,
    /// Base table, families, sieve and relation extraction.
    pub rel: Nat,
    /// Individual-log families, sieve and rows.
    pub ind: Nat,
    pub la: Nat,
    pub la_graph: Nat,
    pub la_peel: Nat,
    pub la_wied: Nat,
    pub la_fallbacks: u32,
    pub fb_cols: usize,
    pub families: u64,
    /// Relations (excluding defining and individual rows).
    pub relations: u64,
    pub rels_by_lp: [u64; 3],
    pub sieve_hits: u64,
    pub sieve_cands: u64,
    pub cand_rejects: u64,
    pub wide_rejects: u64,
    pub ind_rows: usize,
    pub ind_families: u64,
    pub bad_relations: u64,
    pub cycles: i64,
    pub core_rows: usize,
    pub core_cols: usize,
    pub attempts: u32,
    pub wall_ns: u64,
}

impl SieveResult {
    pub fn group_ops(&self) -> u64 {
        self.group.adds + self.group.doubles
    }
    pub fn native_mul(&self) -> u64 {
        self.rel.mul + self.ind.mul + self.la.mul
    }
    pub fn native_add(&self) -> u64 {
        self.rel.add + self.ind.add + self.la.add
    }
}

// ── counted helpers ─────────────────────────────────────────────────────

/// `Σ c_i x^i` by Horner: `len − 1` multiplications and additions.
fn horner(p: u64, c: &[u64], x: u64, n: &mut Nat) -> u64 {
    let mut acc = c[c.len() - 1];
    for i in (0..c.len() - 1).rev() {
        acc = pa(p, pm(p, acc, x, n), c[i], n);
    }
    acc
}

/// A one-parameter family `v_t = v + t·u` and the two top coefficients of
/// `R_0 = (f − v²)/u` it needs: `q_g`, `q_{g−1}`.
#[derive(Clone, Debug)]
struct Fam {
    /// `u`, lowest degree first, with its leading 1 (`g + 1` entries).
    u: Vec<u64>,
    /// `v`, lowest degree first (`g` entries).
    v: Vec<u64>,
    qg: u64,
    qg1: u64,
}

impl Fam {
    fn new(p: u64, g: usize, f: &[u64], u: Vec<u64>, v: Vec<u64>, n: &mut Nat) -> Fam {
        debug_assert!(u.len() == g + 1 && v.len() == g && u[g] == 1);
        // N = f − v² has top coefficients f_{2g+1} = 1, f_{2g}, f_{2g−1}
        // (deg v² ≤ 2g − 2); N = u·q with q monic of degree g + 1.
        let qg = ps(p, f[2 * g], u[g - 1], n);
        let t1 = ps(p, f[2 * g - 1], pm(p, u[g - 1], qg, n), n);
        let qg1 = ps(p, t1, u[g - 2], n);
        Fam { u, v, qg, qg1 }
    }
}

#[derive(Clone, Copy)]
struct Rec {
    t: u32,
    col: u32,
    neg: bool,
    next: u32,
}

/// One relation candidate read off a sieve hit list.
struct Cand {
    /// `(column, ε, x)` of the base roots.
    hits: Vec<(u32, i8, u64)>,
    /// `(x, ε)` of the large primes (2-torsion roots dropped).
    lps: Vec<(u32, i8)>,
    /// Roots whose point has `y = 0` (order 2): their log is 0 modulo the
    /// odd `r`, so they are left out of the row (kept for the exact check).
    tors: Vec<u32>,
}

struct Tables {
    p: u64,
    f: Vec<u64>,
    /// Base abscissae and canonical `y ≤ (p−1)/2`, by column.
    xs: Vec<u32>,
    ys: Vec<u32>,
    col: Vec<u32>,
    chi: Vec<i8>,
    sq: Vec<u32>,
    inv2: u64,
    head: Vec<u32>,
    cnt: Vec<u8>,
    recs: Vec<Rec>,
}

impl Tables {
    fn hit(&mut self, t: u64, col: u32, neg: bool, thr: u8, cands: &mut Vec<u32>, n: &mut Nat) {
        // increment, link, reset
        n.add += 3;
        let idx = self.recs.len() as u32;
        self.recs.push(Rec {
            t: t as u32,
            col,
            neg,
            next: self.head[t as usize],
        });
        self.head[t as usize] = idx;
        let c = &mut self.cnt[t as usize];
        *c += 1;
        if *c == thr {
            cands.push(t as u32);
        }
    }

    fn clear(&mut self) -> u64 {
        let k = self.recs.len() as u64;
        for r in &self.recs {
            self.head[r.t as usize] = NONE;
            self.cnt[r.t as usize] = 0;
        }
        self.recs.clear();
        k
    }

    /// Sieve the family over the whole base; returns the `t` with at least
    /// `thr` hits.  The hit lists stay in `recs` until [`Tables::clear`].
    fn sieve(&mut self, fam: &Fam, thr: u8, n: &mut Nat) -> Vec<u32> {
        let p = self.p;
        let m = self.xs.len();
        let mut idx: Vec<u32> = Vec::with_capacity(m);
        let mut wv: Vec<u64> = Vec::with_capacity(m);
        let mut v0s: Vec<u64> = Vec::with_capacity(m);
        for c in 0..m {
            let x = u64::from(self.xs[c]);
            let w = horner(p, &fam.u, x, n);
            if w == 0 {
                continue; // a root of u: the family does not reach it
            }
            let v0 = horner(p, &fam.v, x, n);
            idx.push(c as u32);
            wv.push(w);
            v0s.push(v0);
        }
        let k = idx.len();
        let mut cands = Vec::new();
        if k == 0 {
            return cands;
        }
        // Montgomery batch inversion: 3(k − 1) products and one inverse.
        let mut pre = Vec::with_capacity(k);
        let mut acc = wv[0];
        pre.push(acc);
        for &w in &wv[1..] {
            acc = pm(p, acc, w, n);
            pre.push(acc);
        }
        let mut inv = ppow(p, acc, p - 2, n);
        let mut winv = vec![0u64; k];
        for i in (1..k).rev() {
            winv[i] = pm(p, inv, pre[i - 1], n);
            inv = pm(p, inv, wv[i], n);
        }
        winv[0] = inv;
        for i in 0..k {
            let c = idx[i];
            let y = u64::from(self.ys[c as usize]);
            let tp = pm(p, ps(p, y, v0s[i], n), winv[i], n);
            let tm = ps(p, 0, pm(p, pa(p, y, v0s[i], n), winv[i], n), n);
            self.hit(tp, c, false, thr, &mut cands, n);
            self.hit(tm, c, true, thr, &mut cands, n);
        }
        cands
    }

    /// Read the hit list of `t` and the cofactor of `R_t`; `None` if the
    /// cofactor does not split, meets the base, or is a repeated root.
    fn extract(&self, fam: &Fam, t: u64, g: usize, n: &mut Nat) -> Option<Cand> {
        let p = self.p;
        let nh = self.cnt[t as usize] as usize;
        let nl = g + 1 - nh;
        let mut hits = Vec::with_capacity(nh);
        let mut i = self.head[t as usize];
        while i != NONE {
            let r = self.recs[i as usize];
            hits.push((
                r.col,
                if r.neg { -1i8 } else { 1 },
                u64::from(self.xs[r.col as usize]),
            ));
            i = r.next;
        }
        let mut lps = Vec::new();
        let mut tors = Vec::new();
        if nl == 0 {
            return Some(Cand {
                hits,
                lps,
                tors: Vec::new(),
            });
        }
        let tt = pm(p, t, t, n);
        let e1 = ps(p, tt, fam.qg, n);
        let mut sh = 0u64;
        for h in &hits {
            sh = pa(p, sh, h.2, n);
        }
        let s = ps(p, e1, sh, n);
        let roots: Vec<u64> = if nl == 1 {
            vec![s]
        } else {
            let tv = pm(p, t, fam.v[g - 1], n);
            let two_tv = pa(p, tv, tv, n);
            let t2u = pm(p, tt, fam.u[g - 1], n);
            let e2 = ps(p, ps(p, fam.qg1, two_tv, n), t2u, n);
            let (mut e2h, mut run) = (0u64, 0u64);
            for h in &hits {
                e2h = pa(p, e2h, pm(p, h.2, run, n), n);
                run = pa(p, run, h.2, n);
            }
            let prod = ps(p, ps(p, e2, e2h, n), pm(p, sh, s, n), n);
            let disc = ps(p, pm(p, s, s, n), pm(p, 4, prod, n), n);
            if self.chi[disc as usize] != 1 {
                return None; // not split, or a double root
            }
            let sd = u64::from(self.sq[disc as usize]);
            let r1 = pm(p, pa(p, s, sd, n), self.inv2, n);
            let r2 = pm(p, ps(p, s, sd, n), self.inv2, n);
            vec![r1, r2]
        };
        for x in roots {
            if self.col[x as usize] != NONE {
                return None; // a base abscissa the sieve did not report
            }
            let vx = horner(p, &fam.v, x, n);
            let ux = horner(p, &fam.u, x, n);
            let y = pa(p, vx, pm(p, t, ux, n), n);
            if y == 0 {
                tors.push(x as u32);
                continue; // 2-torsion: log 0 in the odd-order part
            }
            let eps = if y > (p - 1) / 2 { -1i8 } else { 1 };
            lps.push((x as u32, eps));
        }
        Some(Cand { hits, lps, tors })
    }

    /// The family through `g` base points `(x_i, s_i y_i)`.
    fn fam_from_points(&self, g: usize, cols: &[u32], signs: &[i8], n: &mut Nat) -> Fam {
        let p = self.p;
        let xs: Vec<u64> = cols
            .iter()
            .map(|&c| u64::from(self.xs[c as usize]))
            .collect();
        let ys: Vec<u64> = cols
            .iter()
            .zip(signs)
            .map(|(&c, &s)| {
                let y = u64::from(self.ys[c as usize]);
                if s > 0 {
                    y
                } else {
                    p - y
                }
            })
            .collect();
        let mut u = vec![1u64];
        for &xi in &xs {
            let mut nu = vec![0u64; u.len() + 1];
            for (j, &c) in u.iter().enumerate() {
                nu[j + 1] = pa(p, nu[j + 1], c, n);
                nu[j] = ps(p, nu[j], pm(p, xi, c, n), n);
            }
            u = nu;
        }
        let mut v = vec![0u64; g];
        for i in 0..g {
            let mut d = 1u64;
            for j in 0..g {
                if j != i {
                    d = pm(p, d, ps(p, xs[i], xs[j], n), n);
                }
            }
            let dinv = ppow(p, d, p - 2, n);
            let ci = pm(p, ys[i], dinv, n);
            // N = u / (x − x_i)
            let mut nq = vec![0u64; g];
            nq[g - 1] = 1;
            for j in (1..g).rev() {
                nq[j - 1] = pa(p, u[j], pm(p, xs[i], nq[j], n), n);
            }
            for j in 0..g {
                v[j] = pa(p, v[j], pm(p, ci, nq[j], n), n);
            }
        }
        Fam::new(p, g, &self.f, u, v, n)
    }
}

fn sum_is_identity(inst: &Instance, d0: JDiv, pts: &[(u64, u64)]) -> bool {
    let c = &inst.curve;
    let mut d = d0;
    for &(x, y) in pts {
        match c.point(x, y) {
            Some(pt) => d = c.cantor(d, pt),
            None => return false,
        }
    }
    d.is_identity()
}

struct Solver<'a> {
    inst: &'a Instance,
    prm: SieveParams,
    g: usize,
    lp_eff: usize,
    tab: Tables,
    rng: StdRng,
    res: SieveResult,
    ops: GroupOps,
    rels: Vec<Rel>,
    dsu: Dsu,
    vid: HashMap<u32, u32>,
    /// Current individual-log divisor `T + aG` and `a`.
    ind_state: Option<(JDiv, u64)>,
    /// Field work of cross-checks and test verification inside the curve's
    /// counters, to take back out.
    uncounted_mul: u64,
    uncounted_inv: u64,
}

impl<'a> Solver<'a> {
    fn new(inst: &'a Instance, prm: SieveParams) -> Solver<'a> {
        let p = inst.p;
        let g = inst.genus as usize;
        assert!(
            (3..=4).contains(&g),
            "the sieve is written for genus 3 and 4"
        );
        assert!(g < REL_FB);
        let mut res = SieveResult {
            p,
            genus: inst.genus,
            ..Default::default()
        };
        let f = inst.curve.f_coeffs();
        // Base table: as `run_ic_ext`, plus the inverse table `y ↦ y²`.
        let mut chi = vec![-1i8; p as usize];
        chi[0] = 0;
        let mut sq = vec![0u32; p as usize];
        for y in 1..=(p - 1) / 2 {
            let a = (y * y % p) as usize;
            chi[a] = 1;
            sq[a] = y as u32;
        }
        res.rel.mul += (p - 1) / 2;
        let mut col = vec![NONE; p as usize];
        let (mut xs, mut ys) = (Vec::new(), Vec::new());
        for x in 0..p {
            if xs.len() >= prm.nb {
                break;
            }
            let mut fx = 0u64;
            for &cf in f.iter().rev() {
                fx = (fx * x + cf) % p;
            }
            res.rel.mul += f.len() as u64;
            res.rel.add += f.len() as u64;
            if chi[fx as usize] == 1 {
                col[x as usize] = xs.len() as u32;
                xs.push(x as u32);
                ys.push(sq[fx as usize]);
            }
        }
        res.fb_cols = xs.len();
        let lp_eff = if prm.lp_max >= 2 && 2 * g - 1 > REL_FB {
            1
        } else {
            prm.lp_max.min(2)
        };
        res.lp_max_eff = lp_eff;
        let tab = Tables {
            p,
            f,
            xs,
            ys,
            col,
            chi,
            sq,
            inv2: p.div_ceil(2),
            head: vec![NONE; p as usize],
            cnt: vec![0; p as usize],
            recs: Vec::new(),
        };
        Solver {
            inst,
            prm,
            g,
            lp_eff,
            tab,
            rng: StdRng::seed_from_u64(prm.seed),
            res,
            ops: GroupOps::default(),
            rels: Vec::new(),
            dsu: Dsu::new(),
            vid: HashMap::new(),
            ind_state: None,
            uncounted_mul: 0,
            uncounted_inv: 0,
        }
    }

    fn push_rel(&mut self, rel: Rel) {
        let (a, _, b, _) = rel.endpoints();
        self.dsu.edge(a, b);
        self.rels.push(rel);
    }

    fn empty_rel() -> Rel {
        Rel {
            fb: [(0, 0); REL_FB],
            nfb: 0,
            lp: [(0, 0); 2],
            nlp: 0,
            tcoef: 0,
            rhs: 0,
        }
    }

    fn lp_vertex(&mut self, x: u32) -> u32 {
        if let Some(&v) = self.vid.get(&x) {
            return v;
        }
        let v = self.dsu.add_vertex();
        self.vid.insert(x, v);
        v
    }

    /// Run `f`, then take the curve's field work it spent out of the tally.
    fn uncounted<T>(&mut self, f: impl FnOnce(&Instance) -> T) -> T {
        let inst = self.inst;
        let c = &inst.curve;
        let (m0, i0) = (c.fmul(), c.finv());
        let out = f(inst);
        self.uncounted_mul += c.fmul() - m0;
        self.uncounted_inv += c.finv() - i0;
        out
    }

    // ── one family of relations among base places ──
    fn family_round(&mut self) {
        let g = self.g;
        let m = self.tab.xs.len();
        let mut cols: Vec<u32> = Vec::with_capacity(g);
        while cols.len() < g {
            let c = self.rng.gen_range(0..m) as u32;
            if !cols.contains(&c) {
                cols.push(c);
            }
        }
        let signs: Vec<i8> = (0..g)
            .map(|_| if self.rng.gen::<bool>() { 1 } else { -1 })
            .collect();
        let fam = self
            .tab
            .fam_from_points(g, &cols, &signs, &mut self.res.rel);
        let thr = (g + 1 - self.lp_eff) as u8;
        let cands = self.tab.sieve(&fam, thr, &mut self.res.rel);
        self.res.families += 1;
        let p = self.inst.p;
        let mut wv: Option<u32> = None;
        for &t in &cands {
            self.res.sieve_cands += 1;
            let Some(c) = self.tab.extract(&fam, u64::from(t), g, &mut self.res.rel) else {
                self.res.cand_rejects += 1;
                continue;
            };
            let nl = c.lps.len();
            let mut rel = Self::empty_rel();
            let hits_fb: Vec<(u32, i8)> = c.hits.iter().map(|h| (h.0, h.1)).collect();
            if nl == 2 {
                // direct: the g fixed points and the base roots in one row
                let total = g + hits_fb.len();
                if total > REL_FB {
                    self.res.wide_rejects += 1;
                    continue;
                }
                for (i, &col) in cols.iter().enumerate() {
                    rel.fb[rel.nfb as usize] = (col, signs[i]);
                    rel.nfb += 1;
                }
                for &e in &hits_fb {
                    rel.fb[rel.nfb as usize] = e;
                    rel.nfb += 1;
                }
                for &(x, e) in &c.lps {
                    let id = self.lp_vertex(x);
                    rel.lp[rel.nlp as usize] = (id, e);
                    rel.nlp += 1;
                }
            } else {
                let w = match wv {
                    Some(w) => w,
                    None => {
                        let w = self.dsu.add_vertex();
                        let mut def = Self::empty_rel();
                        for (i, &col) in cols.iter().enumerate() {
                            def.fb[def.nfb as usize] = (col, signs[i]);
                            def.nfb += 1;
                        }
                        def.lp[0] = (w, -1);
                        def.nlp = 1;
                        self.push_rel(def);
                        wv = Some(w);
                        w
                    }
                };
                for &e in &hits_fb {
                    rel.fb[rel.nfb as usize] = e;
                    rel.nfb += 1;
                }
                rel.lp[0] = (w, 1);
                rel.nlp = 1;
                if let Some(&(x, e)) = c.lps.first() {
                    let id = self.lp_vertex(x);
                    rel.lp[1] = (id, e);
                    rel.nlp = 2;
                }
            }
            if self.prm.verify_relations {
                let mut pts: Vec<(u64, u64)> = Vec::new();
                let tab = &self.tab;
                for (i, &col) in cols.iter().enumerate() {
                    let y = u64::from(tab.ys[col as usize]);
                    pts.push((
                        u64::from(tab.xs[col as usize]),
                        if signs[i] > 0 { y } else { p - y },
                    ));
                }
                for h in &c.hits {
                    let y = u64::from(tab.ys[h.0 as usize]);
                    pts.push((h.2, if h.1 > 0 { y } else { p - y }));
                }
                let mut ok = true;
                for &(x, e) in &c.lps {
                    let y = u64::from(tab.sq[(tab.f_at(x as u64)) as usize]);
                    if y == 0 {
                        ok = false;
                        continue;
                    }
                    pts.push((u64::from(x), if e > 0 { y } else { p - y }));
                }
                for &x in &c.tors {
                    pts.push((u64::from(x), 0));
                }
                let good = ok && self.uncounted(|i| sum_is_identity(i, JDiv::IDENTITY, &pts));
                if !good {
                    self.res.bad_relations += 1;
                }
            }
            self.res.rels_by_lp[c.lps.len().min(2)] += 1;
            self.res.relations += 1;
            self.push_rel(rel);
        }
        self.res.sieve_hits += self.tab.clear();
    }

    // ── an individual-log row: a full-base decomposition of T + aG ──
    fn ind_row(&mut self) -> bool {
        let inst = self.inst;
        let (g, r, h) = (self.g, inst.r, inst.h);
        let curve = &inst.curve;
        let p = inst.p;
        let (mut d, mut a) = match self.ind_state.take() {
            Some(s) => s,
            None => {
                let a0 = self.rng.gen_range(1..r);
                let ag = curve.mul(&mut self.ops, inst.g, a0);
                (curve.add(&mut self.ops, ag, inst.target), a0)
            }
        };
        let ops0 = self.ops.adds + self.ops.doubles;
        let mut fams = 0u64;
        let mut found = false;
        while fams < self.prm.ind_max_families {
            if d.deg as usize == g {
                fams += 1;
                self.res.ind_families += 1;
                let u = curve.u_coeffs(&d);
                let v = curve.v_coeffs(&d);
                let fam = Fam::new(p, g, &self.tab.f.clone(), u, v, &mut self.res.ind);
                let cands = self.tab.sieve(&fam, (g + 1) as u8, &mut self.res.ind);
                let mut row = None;
                for &t in &cands {
                    let Some(c) = self.tab.extract(&fam, u64::from(t), g, &mut self.res.ind) else {
                        continue;
                    };
                    debug_assert!(c.lps.is_empty() && c.hits.len() == g + 1);
                    let mut rel = Self::empty_rel();
                    for h2 in &c.hits {
                        rel.fb[rel.nfb as usize] = (h2.0, -h2.1);
                        rel.nfb += 1;
                    }
                    // Σ (−ε) ℓ − h·t = h·a
                    rel.tcoef = subm(0, h % r, r);
                    rel.rhs = mulmod(h % r, a, r);
                    self.res.ind.mul += 1;
                    if self.prm.verify_relations {
                        let tab = &self.tab;
                        let pts: Vec<(u64, u64)> = c
                            .hits
                            .iter()
                            .map(|h2| {
                                let y = u64::from(tab.ys[h2.0 as usize]);
                                (h2.2, if h2.1 > 0 { y } else { p - y })
                            })
                            .collect();
                        let good = self.uncounted(|i| sum_is_identity(i, d, &pts));
                        if !good {
                            self.res.bad_relations += 1;
                        }
                    }
                    row = Some(rel);
                    break;
                }
                self.res.sieve_hits += self.tab.clear();
                if let Some(rel) = row {
                    self.push_rel(rel);
                    found = true;
                    d = curve.add(&mut self.ops, d, inst.g);
                    a = addm(a, 1, r);
                    break;
                }
            }
            d = curve.add(&mut self.ops, d, inst.g);
            a = addm(a, 1, r);
        }
        self.res.ind_group_ops += self.ops.adds + self.ops.doubles - ops0;
        self.ind_state = Some((d, a));
        if found {
            self.res.ind_rows += 1;
        }
        found
    }
}

impl Tables {
    fn f_at(&self, x: u64) -> u64 {
        let mut fx = 0u64;
        for &cf in self.f.iter().rev() {
            fx = (fx * x + cf) % self.p;
        }
        fx
    }
}

/// Solve the DLP of `inst` by sieved relations; certified by `[t]G = T`.
pub fn run_sieve_ic(inst: &Instance, prm: SieveParams) -> SieveResult {
    let start = std::time::Instant::now();
    let r = inst.r;
    let mut s = Solver::new(inst, prm);
    let (fm0, fi0) = (inst.curve.fmul(), inst.curve.finv());
    let ncols = s.tab.xs.len();
    let tcol = ncols as u32;
    let nunk = ncols + 1;
    let margin = |n: usize| 8 + n / 100;
    let mut target = (nunk + margin(nunk)) as i64;
    let mut solved = false;

    for _ in 0..prm.ind_rows.max(2) {
        if !s.ind_row() {
            s.res.exhausted = true;
        }
    }
    'outer: while !s.res.exhausted && !solved {
        loop {
            s.res.cycles = s.dsu.cycles0();
            if s.res.cycles >= target {
                break;
            }
            if s.res.families >= prm.max_families {
                s.res.exhausted = true;
                break 'outer;
            }
            s.family_round();
        }
        s.res.attempts += 1;
        let nver = s.dsu.parent.len();
        let mut g_nat = Nat::default();
        let mut rows = build_rows(&s.rels, nver, tcol, r, &mut g_nat);
        // the homogeneous rows carry an explicit zero in the target column
        for row in rows.iter_mut() {
            g_nat.add += row.0.len() as u64;
            row.0.retain(|&(_, v)| v != 0);
        }
        let mut p_nat = Nat::default();
        let (core, cc, ct) = peel(&rows, nunk, tcol, &mut p_nat);
        s.res.la_graph.merge(g_nat);
        s.res.la_peel.merge(p_nat);
        s.res.la.merge(g_nat);
        s.res.la.merge(p_nat);
        s.res.core_rows = core.len();
        s.res.core_cols = cc;
        if core.len() < cc + margin(cc) {
            target = s.res.cycles + (nunk as i64 / 20).max(8);
            continue;
        }
        let mut w_nat = Nat::default();
        let mut answer = None;
        let certify = |t: u64, sv: &mut Solver| -> bool {
            let inst = sv.inst;
            let c = &inst.curve;
            let v0 = sv.ops.adds + sv.ops.doubles;
            let chk = c.mul(&mut sv.ops, inst.g, t);
            sv.res.verify_ops = sv.ops.adds + sv.ops.doubles - v0;
            let (cm, ci) = (c.fmul(), c.finv());
            let ok = chk == inst.target && c.mul_rtl(inst.g, t) == inst.target;
            sv.uncounted_mul += c.fmul() - cm;
            sv.uncounted_inv += c.finv() - ci;
            ok
        };
        let t = wiedemann_coord(&core, cc, ct as usize, r, &mut s.rng, true, &mut w_nat);
        match t {
            Some(t) if certify(t, &mut s) => answer = Some(t),
            _ => s.res.la_fallbacks += 1,
        }
        if answer.is_none() {
            let sol = wiedemann(&core, cc, r, &mut s.rng, true, &mut w_nat).map(|x| x[ct as usize]);
            if let Some(t) = sol {
                if certify(t, &mut s) {
                    answer = Some(t);
                }
            }
        }
        s.res.la_wied.merge(w_nat);
        s.res.la.merge(w_nat);
        if let Some(t) = answer {
            s.res.recovered = Some(t);
            s.res.certified = true;
            solved = true;
            continue;
        }
        // the system was rank deficient: one more target row, more relations
        target = s.res.cycles + (nunk as i64 / 20).max(8);
        if s.res.ind_rows < 12 && !s.ind_row() {
            s.res.exhausted = true;
        }
    }
    s.res.la.add += s.dsu.ops;
    s.res.group = s.ops;
    let c = &inst.curve;
    s.res.cantor_mul = (c.fmul() - fm0).saturating_sub(s.uncounted_mul);
    s.res.cantor_inv = (c.finv() - fi0).saturating_sub(s.uncounted_inv);
    s.res.wall_ns = start.elapsed().as_nanos() as u64;
    s.res
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cryptanalysis::genus_ladder::{
        fb_size_2lp, make_instance, restart_sqrt, run_ic_ext, Arm, IcExt, IcParams,
    };

    fn first_instance(p: u64, g: u32, skip: usize) -> Instance {
        let mut seen = 0;
        for cs in 1..600u64 {
            if let Ok(i) = make_instance(p, g, cs, 7, 64) {
                if seen == skip {
                    return i;
                }
                seen += 1;
            }
        }
        panic!("no instance at p = {p}, g = {g}");
    }

    /// Every relation is a principal divisor (group-arithmetic check), the
    /// individual-log rows too, and the log certifies, at genus 3 and 4 with
    /// 0, 1 and 2 large primes.
    #[test]
    fn sieved_relations_are_valid_and_certify() {
        for (p, g) in [(131u64, 3u32), (257, 3), (131, 4), (257, 4)] {
            let inst = first_instance(p, g, 0);
            for lp in 0..=2usize {
                let mut prm = SieveParams::new(fb_size_2lp(p, g, 2.0), 11 + lp as u64, lp);
                prm.verify_relations = true;
                let x = run_sieve_ic(&inst, prm);
                assert_eq!(x.bad_relations, 0, "p={p} g={g} lp={lp}");
                assert!(x.relations > 0);
                if x.certified {
                    assert_eq!(x.recovered.unwrap() % inst.r, inst.t_secret % inst.r);
                }
                assert!(x.certified, "p={p} g={g} lp={lp} {x:?}");
            }
        }
    }

    /// The family identity: for a random divisor and every `t`, the roots of
    /// `R_t` are exactly the x with `v_t(x)² = f(x)` outside `u`, and the
    /// closed-form `e_1`, `e_2` agree with the roots found by brute force.
    #[test]
    fn family_top_coefficients_match_brute_force() {
        let p = 131u64;
        let g = 3usize;
        let inst = first_instance(p, 3, 0);
        let f = inst.curve.f_coeffs();
        let mut st = 5u64;
        let mut d = inst.curve.random_divisor(&mut st);
        while d.deg as usize != g {
            d = inst.curve.random_divisor(&mut st);
        }
        let u = inst.curve.u_coeffs(&d);
        let v = inst.curve.v_coeffs(&d);
        let mut n = Nat::default();
        let fam = Fam::new(p, g, &f, u.clone(), v.clone(), &mut n);
        for t in 0..p {
            // v_t, R_t = (f − v_t²)/u by schoolbook
            let mut vt = vec![0u64; g + 1];
            for j in 0..g {
                vt[j] = (vt[j] + v[j]) % p;
            }
            for j in 0..=g {
                vt[j] = (vt[j] + t * u[j]) % p;
            }
            let mut num = f.clone();
            for i in 0..=g {
                for j in 0..=g {
                    let k = i + j;
                    num[k] = (num[k] + p * p - vt[i] * vt[j] % p) % p;
                }
            }
            // divide by u (monic)
            let mut q = vec![0u64; g + 2];
            for k in (g..num.len()).rev() {
                let c = num[k];
                q[k - g] = c;
                for j in 0..=g {
                    num[k - g + j] = (num[k - g + j] + p * p - c * u[j] % p) % p;
                }
            }
            assert!(num[..g].iter().all(|&c| c == 0), "exact division");
            assert_eq!(q[g + 1], 1);
            // coefficient of x^g is −e1, of x^{g−1} is e2
            let e1 = (p - q[g]) % p;
            let e2 = q[g - 1];
            let tt = t * t % p;
            assert_eq!(e1, (tt + p - fam.qg) % p);
            let e2f = (fam.qg1 + 2 * p * p - 2 * t * v[g - 1] % p - tt * u[g - 1] % p) % p;
            assert_eq!(e2, e2f);
        }
    }

    /// The sieve reports exactly the base abscissae that are roots of `R_t`.
    #[test]
    fn sieve_hits_are_roots() {
        let p = 257u64;
        let inst = first_instance(p, 3, 0);
        let prm = SieveParams::new(fb_size_2lp(p, 3, 2.0), 1, 2);
        let mut s = Solver::new(&inst, prm);
        let m = s.tab.xs.len();
        let cols: Vec<u32> = vec![1, 5, 9];
        let signs = vec![1i8, -1, 1];
        let mut n = Nat::default();
        let fam = s.tab.fam_from_points(3, &cols, &signs, &mut n);
        let _ = s.tab.sieve(&fam, 1, &mut n);
        let mut count = 0usize;
        for rec in s.tab.recs.clone() {
            let x = u64::from(s.tab.xs[rec.col as usize]);
            let t = u64::from(rec.t);
            let vx = horner(p, &fam.v, x, &mut n);
            let ux = horner(p, &fam.u, x, &mut n);
            let y = (vx + t * ux) % p;
            let want = u64::from(s.tab.ys[rec.col as usize]);
            assert_eq!(y, if rec.neg { (p - want) % p } else { want });
            assert_eq!(y * y % p, s.tab.f_at(x));
            count += 1;
        }
        // 2 hits per base abscissa outside the three fixed ones
        assert_eq!(count, 2 * (m - 3));
        // the fixed points are interpolated
        for (i, &c) in cols.iter().enumerate() {
            let x = u64::from(s.tab.xs[c as usize]);
            let y = horner(p, &fam.v, x, &mut n);
            let want = u64::from(s.tab.ys[c as usize]);
            assert_eq!(y, if signs[i] > 0 { want } else { p - want });
        }
    }

    /// Smoke comparison against `run_ic_ext` (coord_square + unit_t), printed
    /// and written as JSON lines to `research/hecdlp_sieve_20261116/`.
    /// `cargo test --release genus_ladder_sieve -- --ignored --nocapture`.
    #[test]
    #[ignore]
    fn smoke_vs_run_ic_ext() {
        use std::fmt::Write as _;
        let cfgs: [(u64, u32, usize); 4] = [(1031, 3, 2), (2053, 3, 2), (1031, 4, 1), (2053, 4, 1)];
        let mut out = String::new();
        for (p, g, ninst) in cfgs {
            let mut got = 0;
            for cs in 1..2000u64 {
                if got >= ninst {
                    break;
                }
                let Ok(inst) = make_instance(p, g, cs, 7, 8) else {
                    continue;
                };
                got += 1;
                for seed in 1..=3u64 {
                    let nb = fb_size_2lp(p, g, 2.0);
                    let prm = IcParams {
                        arm: Arm::DoubleLp,
                        nb,
                        seed,
                        max_trials: 200_000_000,
                        restart: restart_sqrt(inst.r),
                        prefilter: true,
                        verify_relations: false,
                        sge: None,
                    };
                    let b = run_ic_ext(
                        &inst,
                        prm,
                        IcExt {
                            coord_square: true,
                            unit_t: true,
                            ferrari: false,
                            sge_coord: false,
                        },
                    );
                    let bn = b.rel.mul + b.rel.add + b.la.mul + b.la.add;
                    let bg = b.group_ops();
                    let _ = writeln!(
                        out,
                        "{{\"smoke\":true,\"arm\":\"run_ic_ext(coord,unitT)\",\"p\":{p},\"g\":{g},\"curve_seed\":{cs},\"seed\":{seed},\"r\":{},\"h\":{},\"nb\":{nb},\"cert\":{},\"group_ops\":{bg},\"cantor_mul\":{},\"rel_mul\":{},\"rel_add\":{},\"la_mul\":{},\"la_add\":{},\"native_all\":{bn},\"relations\":{},\"trials\":{},\"wall_ms\":{}}}",
                        inst.r, inst.h, b.certified, b.cantor_mul, b.rel.mul, b.rel.add,
                        b.la.mul, b.la.add, b.relations, b.trials, b.wall_ns / 1_000_000
                    );
                    for lp in [0usize, 1, 2] {
                        let sp = SieveParams::new(nb, seed, lp);
                        let x = run_sieve_ic(&inst, sp);
                        let xn = x.native_mul() + x.native_add();
                        let _ = writeln!(
                            out,
                            "{{\"smoke\":true,\"arm\":\"sieve(lp<={lp})\",\"lp_eff\":{},\"p\":{p},\"g\":{g},\"curve_seed\":{cs},\"seed\":{seed},\"r\":{},\"h\":{},\"nb\":{nb},\"cert\":{},\"exhausted\":{},\"group_ops\":{},\"ind_group_ops\":{},\"cantor_mul\":{},\"rel_mul\":{},\"rel_add\":{},\"ind_mul\":{},\"ind_add\":{},\"la_mul\":{},\"la_add\":{},\"native_all\":{xn},\"relations\":{},\"by_lp\":[{},{},{}],\"families\":{},\"sieve_hits\":{},\"cands\":{},\"cand_rejects\":{},\"wide_rejects\":{},\"ind_rows\":{},\"ind_families\":{},\"fallbacks\":{},\"attempts\":{},\"core\":[{},{}],\"wall_ms\":{}}}",
                            x.lp_max_eff, inst.r, inst.h, x.certified, x.exhausted, x.group_ops(),
                            x.ind_group_ops, x.cantor_mul, x.rel.mul, x.rel.add, x.ind.mul,
                            x.ind.add, x.la.mul, x.la.add, x.relations, x.rels_by_lp[0],
                            x.rels_by_lp[1], x.rels_by_lp[2], x.families, x.sieve_hits,
                            x.sieve_cands, x.cand_rejects, x.wide_rejects, x.ind_rows,
                            x.ind_families, x.la_fallbacks, x.attempts, x.core_rows,
                            x.core_cols, x.wall_ns / 1_000_000
                        );
                    }
                }
            }
        }
        print!("{out}");
        let dir = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/research/hecdlp_sieve_20261116"
        );
        let _ = std::fs::create_dir_all(dir);
        let _ = std::fs::write(format!("{dir}/smoke.jsonl"), &out);
    }
}
