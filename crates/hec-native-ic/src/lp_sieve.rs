//! Reduced-base large-prime line sieve against the walk (feasibility probe;
//! LEADS-20261114 lead 1, follow-up A; not under a frozen protocol).
//!
//! Base `B`: the first `nb` degree-1 places of `factor_base` (by `x`).  Every
//! other degree-1 place is a large prime.  A relation is classified by the
//! number of distinct large-prime `x` it involves: 0 = full, 1 = one-partial
//! (1P), 2 = two-partial (2P), more = discarded.
//!
//! **Sieve.**  For a line `D = (u, v)`, `deg u = g`, and `a_c = v + c·u`,
//! `w_c = (f − a_c²)/u = W₀ − 2c·v − c²·u` with `W₀ = (f − v²)/u` monic of
//! degree `g + 1`.  Its top coefficients are `[x^g] w_c = W₀_g − c²` and
//! `[x^{g−1}] w_c = W₀_{g−1} − 2c·v_{g−1} − c²·u_{g−1}`.  Only base points
//! are bucketed.  A `c` with `h` base hits:
//! - `h = g + 1`: full.
//! - `h = g`: the last root is `x* = −[x^g]w_c − Σ r_i`, `y* = a_c(x*)`.
//! - `h = g − 1`: the last two roots solve `x² − S·x + P` with
//!   `S = −[x^g]w_c − e₁`, `P = [x^{g−1}]w_c − S·e₁ − e₂` (`e₁, e₂` the
//!   elementary symmetric functions of the known roots); one Legendre symbol,
//!   and one square root when it is a residue.
//! Every relation is `D + Σ (x_i, a_c(x_i)) = 0`, with multiplicity.
//!
//! **Walk.**  The engine's walk (`d_step_in = 4`, 64 step places drawn from
//! `B`) and oracle; a trial whose `u` splits is classified the same way.
//!
//! Work is counted with feature `count-ops` (field products); without it the
//! `ops_*` fields are zero.

use crate::fp::{op_counts, Fp, OpCounts};
use crate::jac::{Curve, Div};
use crate::oracle::{split, OracleStats};
use crate::poly::Poly;
use crate::rng::Rng;

/// The reduced base and an `x`-index of every degree-1 place.
pub struct Base {
    /// `(x, y)` of the base places, `y ≤ (p−1)/2`.
    pub b: Vec<(u64, u64)>,
    /// `in_b[x]`: index in `b`, or `u32::MAX`.
    pub in_b: Vec<u32>,
    /// Degree-1 places in all (`x` values with `f(x)` a square).
    pub places: usize,
}

impl Base {
    pub fn new(fb: &[(u64, u64)], nb: usize, p: u64) -> Base {
        let nb = nb.min(fb.len());
        let mut in_b = vec![u32::MAX; p as usize];
        for (j, &(x, _)) in fb[..nb].iter().enumerate() {
            in_b[x as usize] = j as u32;
        }
        Base {
            b: fb[..nb].to_vec(),
            in_b,
            places: fb.len(),
        }
    }
    #[inline]
    fn is_base(&self, x: u64) -> bool {
        self.in_b[x as usize] != u32::MAX
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Classes {
    pub full: u64,
    pub p1: u64,
    pub p2: u64,
    /// More than two large primes, or a degenerate quadratic (repeated root).
    pub discarded: u64,
}

impl Classes {
    fn add(&mut self, large: usize) {
        match large {
            0 => self.full += 1,
            1 => self.p1 += 1,
            2 => self.p2 += 1,
            _ => self.discarded += 1,
        }
    }
}

/// The partial's edge for the large-prime graph: `(x₁, x₂)` for a 2P,
/// `(x₁, ONE)` for a 1P (`ONE` is the special vertex).
pub const ONE: u32 = u32::MAX;
fn edge(base: &Base, xs: &[u64]) -> Option<(u32, u32)> {
    let mut l = [ONE; 2];
    let mut n = 0;
    for &x in xs {
        if !base.is_base(x) && !l[..n].contains(&(x as u32)) {
            if n == 2 {
                return None;
            }
            l[n] = x as u32;
            n += 1;
        }
    }
    if n == 0 {
        None
    } else {
        Some((l[0], l[1]))
    }
}

/// Distinct large-prime `x` among the roots.
fn large_count(base: &Base, xs: &[u64]) -> usize {
    let mut seen: [u64; 8] = [u64::MAX; 8];
    let mut n = 0;
    for &x in xs {
        if !base.is_base(x) && !seen[..n].contains(&x) {
            seen[n] = x;
            n += 1;
        }
    }
    n
}

#[derive(Clone, Debug, Default)]
pub struct LpSieveReport {
    pub lines: u64,
    pub degenerate_lines: u64,
    pub classes: Classes,
    /// Partials in the order found, as large-prime graph edges.
    pub edges: Vec<(u32, u32)>,
    /// Candidates examined: `c` with `g` hits, `c` with `g − 1` hits, and of
    /// those the quadratics that were non-residues.
    pub cand_g: u64,
    pub cand_gm1: u64,
    pub nonresidue: u64,
    pub verified: u64,
    pub ops_rel: OpCounts,
    pub ops_verify: OpCounts,
    pub ns: u128,
    /// Fulls + independent cycles of the large-prime graph.
    pub useful: u64,
    pub reached: bool,
}

#[inline]
fn coef(p: &Poly, i: usize) -> u64 {
    if i < p.n {
        p.c[i]
    } else {
        0
    }
}

/// Run `lines` sieve lines from `D₁ + D₂`; verify the first `verify` relations
/// of each class in the Jacobian (excluded from `ops_rel`).
/// With `target > 0`, stop as soon as fulls + graph cycles reach `target`
/// (`reached` is then set); `lines` is the budget.
pub fn lp_sieve(
    curve: &Curve,
    d1: &Div,
    d2: &Div,
    base: &Base,
    lines: u64,
    seed: u64,
    verify: u64,
    target: u64,
) -> LpSieveReport {
    let t_all = std::time::Instant::now();
    let fp: &Fp = &curve.fp;
    let g = curve.g;
    let p = fp.p;
    let mut rep = LpSieveReport::default();
    let mut rng = Rng::new(seed);
    let o_start = op_counts();
    let mut ops_verify = OpCounts::default();

    let nb = base.b.len();
    let mut cnt = vec![0u8; p as usize];
    let mut slot = vec![u32::MAX; p as usize];
    let mut touched: Vec<u32> = Vec::with_capacity(2 * nb);
    let mut hits: Vec<(u32, u32)> = Vec::with_capacity(2 * nb); // (c, j·2 + sign bit)
    let mut idx: Vec<u32> = Vec::with_capacity(nb);
    let mut uval: Vec<u64> = Vec::with_capacity(nb);
    let mut vval: Vec<u64> = Vec::with_capacity(nb);
    let mut pref: Vec<u64> = Vec::with_capacity(nb);
    let mut groups: Vec<(u32, Vec<u32>)> = Vec::new();
    let mut verified = [0u64; 3];
    let mut graph = Graph::default();

    let mut r = curve.add(d1, d2);
    'lines: while rep.lines < lines {
        rep.lines += 1;
        if rep.lines > 1 {
            r = if rng.below(2) == 0 { curve.add(&r, d1) } else { curve.add(&r, d2) };
        }
        if r.u.deg() != g as isize {
            rep.degenerate_lines += 1;
            continue;
        }
        let (u, v) = (r.u, r.v);
        let w0 = curve.f.sub(&v.mul(&v, fp), fp).div_exact(&u, fp);
        let (w0g, w0g1) = (coef(&w0, g), coef(&w0, g - 1));
        let (vg1, ug1) = (coef(&v, g - 1), coef(&u, g - 1));

        idx.clear();
        uval.clear();
        vval.clear();
        for (j, &(x, _)) in base.b.iter().enumerate() {
            let ux = u.eval(x, fp);
            if ux == 0 {
                continue;
            }
            idx.push(j as u32);
            uval.push(ux);
            vval.push(v.eval(x, fp));
        }
        let n = idx.len();
        if n == 0 {
            continue;
        }
        pref.clear();
        let mut acc = 1u64;
        for &ux in &uval {
            pref.push(acc);
            acc = fp.mul(acc, ux);
        }
        let mut inv = fp.inv(acc);
        for i in (0..n).rev() {
            let ui = fp.mul(inv, pref[i]);
            inv = fp.mul(inv, uval[i]);
            uval[i] = ui;
        }
        hits.clear();
        touched.clear();
        for i in 0..n {
            let j = idx[i];
            let y = base.b[j as usize].1;
            for s in 0..(if y == 0 { 1 } else { 2 }) {
                let sy = if s == 0 { y } else { fp.neg(y) };
                let c = fp.mul(fp.sub(sy, vval[i]), uval[i]) as u32;
                if cnt[c as usize] == 0 {
                    touched.push(c);
                }
                cnt[c as usize] = cnt[c as usize].saturating_add(1);
                hits.push((c, j * 2 + s));
            }
        }
        groups.clear();
        for &(c, js) in &hits {
            let h = cnt[c as usize] as usize;
            if h + 1 < g {
                continue;
            }
            let sl = &mut slot[c as usize];
            if *sl == u32::MAX {
                *sl = groups.len() as u32;
                groups.push((c, Vec::with_capacity(g + 1)));
            }
            groups[*sl as usize].1.push(js);
        }
        for &c in &touched {
            cnt[c as usize] = 0;
            slot[c as usize] = u32::MAX;
        }

        // Finish every candidate.
        for (c, grp) in &groups {
            let c = *c as u64;
            let h = grp.len();
            let mut xs: [u64; 8] = [0; 8];
            let mut nx = 0;
            for &js in grp {
                xs[nx] = base.b[(js / 2) as usize].0;
                nx += 1;
            }
            let c2 = fp.mul(c, c);
            let top = fp.sub(w0g, c2); // [x^g] w_c
            let e1 = xs[..nx].iter().fold(0u64, |a, &x| fp.add(a, x));
            if h == g + 1 {
                // full
            } else if h == g {
                rep.cand_g += 1;
                xs[nx] = fp.neg(fp.add(top, e1));
                nx += 1;
            } else {
                rep.cand_gm1 += 1;
                let mut e2 = 0u64;
                for i in 0..nx {
                    for k in i + 1..nx {
                        e2 = fp.add(e2, fp.mul(xs[i], xs[k]));
                    }
                }
                let next = fp.sub(fp.sub(w0g1, fp.mul(fp.add(c, c), vg1)), fp.mul(c2, ug1));
                let s = fp.neg(fp.add(top, e1));
                let pr = fp.sub(fp.sub(next, fp.mul(s, e1)), e2);
                let disc = fp.sub(fp.mul(s, s), fp.mul(4 % p, pr));
                if disc == 0 {
                    rep.classes.discarded += 1;
                    continue;
                }
                let Some(sq) = fp.sqrt(disc) else {
                    rep.nonresidue += 1;
                    continue;
                };
                let half = (p + 1) / 2;
                xs[nx] = fp.mul(fp.add(s, sq), half);
                xs[nx + 1] = fp.mul(fp.sub(s, sq), half);
                nx += 2;
            }
            let large = large_count(base, &xs[..nx]);
            rep.classes.add(large);
            if large == 1 || large == 2 {
                let (a, b) = edge(base, &xs[..nx]).unwrap();
                rep.edges.push((a, b));
                if graph.add(a, b) {
                    rep.useful += 1;
                }
            } else if large == 0 {
                rep.useful += 1;
            }
            // The large primes' y = a_c(x) is part of the relation (row).
            let mut ys: [u64; 8] = [0; 8];
            for i in h..nx {
                ys[i] = fp.add(v.eval(xs[i], fp), fp.mul(c, u.eval(xs[i], fp)));
            }
            let cls = large.min(3);
            if cls < 3 && verified[cls] < verify {
                let o0 = op_counts();
                for (i, &js) in grp.iter().enumerate() {
                    let y = base.b[(js / 2) as usize].1;
                    ys[i] = if js % 2 == 0 { y } else { fp.neg(y) };
                }
                let mut e = curve.zero();
                for i in 0..nx {
                    e = curve.add_place(&e, xs[i], ys[i]);
                }
                assert!(
                    curve.is_zero(&curve.add(&e, &r)),
                    "large-prime sieve relation fails in the Jacobian (class {cls}, h {h})"
                );
                verified[cls] += 1;
                rep.verified += 1;
                ops_verify.add(&op_counts().since(&o0));
            }
            if target > 0 && rep.useful >= target {
                rep.reached = true;
                break 'lines;
            }
        }
    }
    rep.ops_rel = op_counts().since(&o_start).since(&ops_verify);
    rep.ops_verify = ops_verify;
    rep.ns = t_all.elapsed().as_nanos();
    rep
}

#[derive(Clone, Debug, Default)]
pub struct LpWalkReport {
    pub trials: u64,
    pub split: u64,
    pub classes: Classes,
    pub edges: Vec<(u32, u32)>,
    pub oracle: OracleStats,
    pub useful: u64,
    pub reached: bool,
    pub ops_rel: OpCounts,
    pub ns: u128,
}

/// The engine's walk (as `ic::solve_dlp`: `d_step_in = 4`, 64 step places,
/// here drawn from `B`) for `trials` trials; classify every split `u`.
/// With `target > 0`, stop as soon as fulls + graph cycles reach `target`.
pub fn lp_walk(curve: &Curve, d1: &Div, d2: &Div, base: &Base, trials: u64, seed: u64, target: u64) -> LpWalkReport {
    let t_all = std::time::Instant::now();
    let fp = &curve.fp;
    let mut rep = LpWalkReport::default();
    let mut rng = Rng::new(seed);
    let o_start = op_counts();
    let nb = base.b.len();
    let branches = 64.min(nb);
    let mut order: Vec<u32> = (0..nb as u32).collect();
    for i in 0..branches {
        let j = i + rng.below((nb - i) as u64) as usize;
        order.swap(i, j);
    }
    let steps: Vec<u32> = order[..branches].to_vec();
    let mut r = curve.add(d1, d2);
    let mut xs = [0u64; 8];
    let mut graph = Graph::default();
    while rep.trials < trials {
        rep.trials += 1;
        let roll = rng.below(4);
        r = if roll == 0 {
            curve.add(&r, d1)
        } else if roll == 1 {
            curve.add(&r, d2)
        } else {
            let s = rng.below(branches as u64) as usize;
            let (x, y) = base.b[steps[s] as usize];
            curve.add_place(&r, x, y)
        };
        let Some(roots) = split(&r.u, fp, &mut rep.oracle) else {
            continue;
        };
        rep.split += 1;
        let mut n = 0;
        for &(x, _) in roots.as_slice() {
            xs[n] = x;
            n += 1;
        }
        let large = large_count(base, &xs[..n]);
        rep.classes.add(large);
        if large == 1 || large == 2 {
            let (a, b) = edge(base, &xs[..n]).unwrap();
            rep.edges.push((a, b));
            if graph.add(a, b) {
                rep.useful += 1;
            }
        } else if large == 0 {
            rep.useful += 1;
        }
        if target > 0 && rep.useful >= target {
            rep.reached = true;
            break;
        }
    }
    rep.ops_rel = op_counts().since(&o_start);
    rep.ns = t_all.elapsed().as_nanos();
    rep
}

/// Incremental large-prime graph: `add` returns true when the edge closes
/// a cycle (one more full relation, GTTD's 2LP count).
#[derive(Default)]
pub struct Graph {
    id: std::collections::HashMap<u32, u32>,
    parent: Vec<u32>,
    pub cycles: u64,
}

impl Graph {
    fn find(&mut self, mut a: u32) -> u32 {
        while self.parent[a as usize] != a {
            let gp = self.parent[self.parent[a as usize] as usize];
            self.parent[a as usize] = gp;
            a = gp;
        }
        a
    }
    fn vertex(&mut self, x: u32) -> u32 {
        let n = self.parent.len() as u32;
        let v = *self.id.entry(x).or_insert(n);
        if v == n {
            self.parent.push(n);
        }
        v
    }
    pub fn add(&mut self, a: u32, b: u32) -> bool {
        let (va, vb) = (self.vertex(a), self.vertex(b));
        let (ra, rb) = (self.find(va), self.find(vb));
        if ra == rb {
            self.cycles += 1;
            true
        } else {
            self.parent[ra as usize] = rb;
            false
        }
    }
}

/// Independent cycles in the large-prime graph of the first `n` edges
/// (union–find; each edge that closes a cycle adds one).  Every cycle gives
/// one full relation (Gaudry–Thomé–Thériault–Diem's 2LP count).
pub fn cycles(edges: &[(u32, u32)], n: usize) -> u64 {
    use std::collections::HashMap;
    let mut id: HashMap<u32, usize> = HashMap::new();
    let mut parent: Vec<usize> = Vec::new();
    fn find(parent: &mut [usize], mut a: usize) -> usize {
        while parent[a] != a {
            parent[a] = parent[parent[a]];
            a = parent[a];
        }
        a
    }
    let mut c = 0;
    for &(a, b) in &edges[..n.min(edges.len())] {
        let mut v = |x: u32, parent: &mut Vec<usize>| {
            *id.entry(x).or_insert_with(|| {
                parent.push(parent.len());
                parent.len() - 1
            })
        };
        let (ia, ib) = (v(a, &mut parent), v(b, &mut parent));
        let (ra, rb) = (find(&mut parent, ia), find(&mut parent, ib));
        if ra == rb {
            c += 1;
        } else {
            parent[ra] = rb;
        }
    }
    c
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instance::{factor_base, thread_curve};

    /// Every class of large-prime sieve relation holds in the Jacobian
    /// (asserted inside), at genus 2, 3 and 4.
    #[test]
    fn lp_sieve_relations_hold() {
        for &(g, p, nb) in &[(2usize, 1009u64, 60usize), (3, 2039, 120), (4, 509, 80)] {
            let curve = (1..50).find_map(|c| thread_curve(p, c, g)).unwrap();
            let fb = factor_base(&curve);
            let base = Base::new(&fb, nb, p);
            let mut rng = Rng::new(7);
            let place = |rng: &mut Rng| {
                let (x, y) = fb[rng.below(fb.len() as u64) as usize];
                curve.place(x, y)
            };
            let mut d1 = curve.zero();
            let mut d2 = curve.zero();
            for _ in 0..g {
                d1 = curve.add(&d1, &place(&mut rng));
                d2 = curve.add(&d2, &place(&mut rng));
            }
            let rep = lp_sieve(&curve, &d1, &d2, &base, 300, 1, u64::MAX, 0);
            assert!(rep.classes.p2 > 0 && rep.classes.p1 > 0, "g={g}: {:?}", rep.classes);
            assert_eq!(rep.verified, rep.classes.full + rep.classes.p1 + rep.classes.p2);
        }
    }
}
