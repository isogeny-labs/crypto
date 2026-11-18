// N4 v2 relation stages: sorted bucketing (arm S, same relation set as B0) and double large primes
// with graph-cycle recombination (arm D). Deterministic: hash maps are used for lookups only, never
// iterated. Mod-ell additions are overflow-safe for ell < 2^64.
#![allow(dead_code)]
use crate::common::*;
use crate::ic::{cross, pnorm, Curve, Rel, RelOut, Row, P3};
use std::collections::{BTreeMap, HashMap, HashSet};

/// Injective packing of a normalised projective point (first nonzero coordinate 1), q < 2^31.
#[inline(always)]
pub fn pack2(p: &P3) -> u64 {
    if p[0] != 0 {
        debug_assert!(p[0] == 1);
        (1u64 << 62) | (p[1] << 31) | p[2]
    } else if p[1] != 0 {
        debug_assert!(p[1] == 1);
        (2u64 << 62) | p[2]
    } else {
        3u64 << 62
    }
}

#[inline(always)]
pub fn la2(a: u64, b: u64, ell: u64) -> u64 {
    bump_ladd(1);
    let (s, o) = a.overflowing_add(b);
    if o || s >= ell {
        s.wrapping_sub(ell)
    } else {
        s
    }
}
#[inline(always)]
pub fn ls2(a: u64, b: u64, ell: u64) -> u64 {
    bump_ladd(1);
    if a >= b {
        a - b
    } else {
        a + (ell - b)
    }
}
#[inline(always)]
pub fn lm2(a: u64, b: u64, ell: u64) -> u64 {
    bump_lmul(1);
    ((a as u128 * b as u128) % ell as u128) as u64
}
/// mod-ell inversion, charged as 60 lmul (B0 does not charge its Lanczos inversions; noted).
pub fn linv(a: u64, ell: u64) -> u64 {
    bump_lmul(60);
    inv_mod_u64(a, ell)
}
pub fn to_mod(v: i64, ell: u64) -> u64 {
    if v >= 0 {
        (v as u64) % ell
    } else {
        let r = ((-(v as i128)) as u128 % ell as u128) as u64;
        if r == 0 {
            0
        } else {
            ell - r
        }
    }
}

/// N4's sample_points with the injective packing (identical output for q < 2^21).
pub fn sample_points2(cv: &Curve, n: usize, rng: &mut Rng) -> Vec<P3> {
    let q = cv.f.q;
    let mut seen: HashSet<u64> = HashSet::new();
    let mut out = Vec::new();
    while out.len() < n {
        let a = [rng.below(q), rng.below(q), rng.below(q)];
        let b = [rng.below(q), rng.below(q), rng.below(q)];
        if a == [0, 0, 0] || b == [0, 0, 0] {
            continue;
        }
        let a = pnorm(&cv.f, &a);
        let b = pnorm(&cv.f, &b);
        if a == b {
            continue;
        }
        for p in cv.line_points(&a, &b, rng) {
            if out.len() < n && seen.insert(pack2(&p)) {
                out.push(p);
            }
        }
    }
    out
}

// ------------------------------------------------------------------ process CPU time
#[repr(C)]
struct TimeVal {
    sec: i64,
    usec: i32,
}
#[repr(C)]
struct RUsage {
    utime: TimeVal,
    stime: TimeVal,
    rest: [i64; 14],
}
extern "C" {
    fn getrusage(who: i32, usage: *mut RUsage) -> i32;
}
/// user + system CPU seconds of this process (RUSAGE_SELF)
pub fn cpu_now() -> f64 {
    let mut u = RUsage { utime: TimeVal { sec: 0, usec: 0 }, stime: TimeVal { sec: 0, usec: 0 }, rest: [0; 14] };
    unsafe {
        getrusage(0, &mut u);
    }
    u.utime.sec as f64 + u.utime.usec as f64 * 1e-6 + u.stime.sec as f64 + u.stime.usec as f64 * 1e-6
}

// ------------------------------------------------------------------ arm S: sorted bucketing
pub struct SortStats {
    pub sort_keys: u64,
    pub sort_cmp: u64,
    pub dup_skipped: u64,
}

/// Same relation set as N4's find_relations (lines with 3 or 4 factor-base points), found from the
/// smallest factor-base index on each line by sorting the lines through that point.
pub fn find_relations_sorted(cv: &Curve, fb: &[P3]) -> (RelOut, SortStats) {
    let f = &cv.f;
    let n = fb.len();
    let idx: HashMap<u64, u32> = fb.iter().enumerate().map(|(i, p)| (pack2(p), i as u32)).collect();
    bump_hash(n as u64);
    let mut out = RelOut { full: vec![], lp: vec![], lp_pts: vec![], pairs: 0, multi_lines: 0, unsure: 0 };
    let mut st = SortStats { sort_keys: 0, sort_cmp: 0, dup_skipped: 0 };
    let mut lp_ids: HashMap<u64, u32> = HashMap::new();
    let mut keys: Vec<(u64, u32)> = Vec::with_capacity(n);
    let mut ls: Vec<P3> = Vec::with_capacity(n);
    let mut piv: Vec<u64> = Vec::with_capacity(n);
    let mut pref: Vec<u64> = Vec::with_capacity(n);
    let mut invs: Vec<u64> = Vec::with_capacity(n);
    for i in 0..n {
        let m = n - i - 1;
        if m == 0 {
            break;
        }
        ls.clear();
        piv.clear();
        for j in i + 1..n {
            let l = cross(f, &fb[i], &fb[j]);
            let pv = *l.iter().find(|x| **x != 0).unwrap_or(&1);
            piv.push(pv);
            ls.push(l);
        }
        pref.clear();
        let mut acc = 1u64;
        for &p in &piv {
            pref.push(acc);
            acc = f.mul(acc, p);
        }
        let mut inv_acc = f.inv(acc);
        invs.clear();
        invs.resize(m, 0);
        for k in (0..m).rev() {
            invs[k] = f.mul(inv_acc, pref[k]);
            inv_acc = f.mul(inv_acc, piv[k]);
        }
        keys.clear();
        for (k, l) in ls.iter().enumerate() {
            let key = pack2(&[f.mul(l[0], invs[k]), f.mul(l[1], invs[k]), f.mul(l[2], invs[k])]);
            keys.push((key, (i + 1 + k) as u32));
        }
        out.pairs += m as u64;
        bump_hash(m as u64); // one unpriced operation per sorted key (B0: one hash op per pair)
        st.sort_keys += m as u64;
        let mut cmpc = 0u64;
        keys.sort_unstable_by(|a, b| {
            cmpc += 1;
            a.cmp(b)
        });
        st.sort_cmp += cmpc;
        let mut s = 0;
        while s < keys.len() {
            let mut e = s + 1;
            while e < keys.len() && keys[e].0 == keys[s].0 {
                e += 1;
            }
            if e - s >= 2 {
                out.multi_lines += 1;
                let mut pts: Vec<u32> = Vec::with_capacity(e - s + 1);
                pts.push(i as u32);
                for t in s..e {
                    pts.push(keys[t].1);
                }
                process_multi(cv, fb, &idx, &pts, &mut out, &mut lp_ids, &mut st);
            }
            s = e;
        }
    }
    (out, st)
}

/// B0's handling of one line with >= 3 factor-base points (pts sorted, pts[0] the smallest).
fn process_multi(cv: &Curve, fb: &[P3], idx: &HashMap<u64, u32>, pts: &[u32], out: &mut RelOut, lp_ids: &mut HashMap<u64, u32>, st: &mut SortStats) {
    let f = &cv.f;
    if pts.len() >= 4 {
        if pts.len() > 4 {
            out.unsure += 1;
            return;
        }
        out.full.push(Rel { c: pts.iter().map(|&i| (i, 1u8)).collect(), lp: None });
        return;
    }
    let (ia, ib, ic) = (pts[0], pts[1], pts[2]);
    let (a, b, c) = (fb[ia as usize], fb[ib as usize], fb[ic as usize]);
    let g = cv.restrict(&a, &b);
    if g[0] != 0 || g[4] != 0 {
        out.unsure += 1;
        return;
    }
    let ab = cross(f, &a, &b);
    let k = (0..3).find(|&k| ab[k] != 0).unwrap();
    let cb = cross(f, &c, &b);
    let ac = cross(f, &a, &c);
    let ik = f.inv(ab[k]);
    let sc = f.mul(cb[k], ik);
    let tc = f.mul(ac[k], ik);
    if tc == 0 {
        out.unsure += 1;
        return;
    }
    let rho_c = f.mul(sc, f.inv(tc));
    let mut mult = [1u8, 1, 1];
    let mut lp_pt: Option<P3> = None;
    if g[3] == 0 {
        mult[0] = 2;
    } else {
        let sum = f.neg(f.mul(g[2], f.inv(g[3])));
        let rho4 = f.sub(sum, rho_c);
        let p4 = [
            f.add(f.mul(rho4, a[0]), b[0]),
            f.add(f.mul(rho4, a[1]), b[1]),
            f.add(f.mul(rho4, a[2]), b[2]),
        ];
        let p4 = pnorm(f, &p4);
        if p4 == a {
            mult[0] = 2;
        } else if p4 == b {
            mult[1] = 2;
        } else if p4 == c {
            mult[2] = 2;
        } else {
            bump_hash(1);
            if idx.contains_key(&pack2(&p4)) {
                // a 4-point line seen from its second-smallest point: found from the smallest
                out.unsure += 1;
                st.dup_skipped += 1;
                return;
            }
            lp_pt = Some(p4);
        }
    }
    let cs = vec![(ia, mult[0]), (ib, mult[1]), (ic, mult[2])];
    match lp_pt {
        None => {
            let tot: u8 = cs.iter().map(|x| x.1).sum();
            if tot == 4 {
                out.full.push(Rel { c: cs, lp: None });
            } else {
                out.unsure += 1;
            }
        }
        Some(p4) => {
            bump_hash(1);
            let id = *lp_ids.entry(pack2(&p4)).or_insert_with(|| {
                out.lp_pts.push(p4);
                (out.lp_pts.len() - 1) as u32
            });
            out.lp.push(Rel { c: cs, lp: Some(id) });
        }
    }
}

/// B0's matrix rows from a RelOut (full rows with kappa, large-prime rows subtracted from the first
/// relation of each large prime), with ordered maps. Columns: fb[1..] -> 0..n-2, kappa -> n-1.
pub fn rows_from_relout(ro: &RelOut, n: usize, ell: u64) -> (Vec<Row>, Vec<Row>) {
    let eta_col = (n - 1) as u32;
    let col = |i: u32| -> Option<u32> { if i == 0 { None } else { Some(i - 1) } };
    let mut full_rows: Vec<Row> = Vec::new();
    for r in &ro.full {
        let mut row: Row = Vec::new();
        for &(i, m) in &r.c {
            if let Some(c) = col(i) {
                row.push((c, m as u64));
            }
        }
        row.push((eta_col, ell - 1));
        row.sort();
        full_rows.push(row);
    }
    let mut lp_groups: BTreeMap<u32, Vec<usize>> = BTreeMap::new();
    for (ri, r) in ro.lp.iter().enumerate() {
        lp_groups.entry(r.lp.unwrap()).or_default().push(ri);
    }
    let mut lp_rows: Vec<Row> = Vec::new();
    for (_l, rs) in &lp_groups {
        for w in 1..rs.len() {
            let mut m: BTreeMap<u32, i64> = BTreeMap::new();
            for &(i, mu) in &ro.lp[rs[w]].c {
                if let Some(c) = col(i) {
                    *m.entry(c).or_insert(0) += mu as i64;
                }
            }
            for &(i, mu) in &ro.lp[rs[0]].c {
                if let Some(c) = col(i) {
                    *m.entry(c).or_insert(0) -= mu as i64;
                }
            }
            let row: Row = m.into_iter().filter(|x| x.1 != 0).map(|(c, v)| (c, to_mod(v, ell))).collect();
            if !row.is_empty() {
                lp_rows.push(row);
            }
        }
    }
    (full_rows, lp_rows)
}

// ------------------------------------------------------------------ arm D: double large primes
/// One line relation: factor-base points with multiplicity, and up to two large-prime vertices.
#[derive(Clone)]
pub struct DRel {
    pub c: Vec<(u32, u8)>,
    pub u: u32, // vertex ids (0 = root/known); full relations have u = v = u32::MAX
    pub v: u32,
}

pub struct DOut {
    pub full: Vec<DRel>,
    pub edges: Vec<DRel>,
    pub lp_pts: Vec<P3>, // vertex id v >= 1 -> lp_pts[v - 1]
    pub pairs: u64,
    pub rational_pairs: u64,
    pub dup_skipped: u64,
    pub unsure: u64,
    pub n1lp: u64,
    pub n2lp: u64,
    pub nloop: u64,
}

/// gradient (3) and half-Hessian upper triangle (6: xx/2, yy/2, zz/2, xy, xz, yz) of F at p, and the
/// six degree-2 monomials of p (x^2, y^2, z^2, xy, xz, yz).
fn local_data(cv: &Curve, p: &P3, inv2: u64) -> ([u64; 3], [u64; 6], [u64; 6]) {
    let f = &cv.f;
    let mut pw = [[1u64; 5]; 3];
    for i in 0..3 {
        for e in 1..5 {
            pw[i][e] = f.mul(pw[i][e - 1], p[i]);
        }
    }
    let mut g = [0u64; 3];
    let mut h = [0u64; 6]; // full second derivatives: xx, yy, zz, xy, xz, yz
    for (mi, &(a, b, c)) in cv.mons.iter().enumerate() {
        let co = cv.coef[mi];
        if co == 0 {
            continue;
        }
        let e = [a as usize, b as usize, c as usize];
        let term = |d: [usize; 3]| -> u64 {
            // coefficient * prod_i falling(e_i, d_i) * p_i^(e_i - d_i)
            let mut k = co;
            for i in 0..3 {
                if e[i] < d[i] {
                    return 0;
                }
                let mut fall = 1u64;
                for t in 0..d[i] {
                    fall *= (e[i] - t) as u64;
                }
                if fall != 1 {
                    k = f.mul(k, fall % f.q);
                }
                if e[i] - d[i] > 0 {
                    k = f.mul(k, pw[i][e[i] - d[i]]);
                }
            }
            k
        };
        g[0] = f.add(g[0], term([1, 0, 0]));
        g[1] = f.add(g[1], term([0, 1, 0]));
        g[2] = f.add(g[2], term([0, 0, 1]));
        h[0] = f.add(h[0], term([2, 0, 0]));
        h[1] = f.add(h[1], term([0, 2, 0]));
        h[2] = f.add(h[2], term([0, 0, 2]));
        h[3] = f.add(h[3], term([1, 1, 0]));
        h[4] = f.add(h[4], term([1, 0, 1]));
        h[5] = f.add(h[5], term([0, 1, 1]));
    }
    let hh = [f.mul(h[0], inv2), f.mul(h[1], inv2), f.mul(h[2], inv2), h[3], h[4], h[5]];
    let mo = [f.mul(p[0], p[0]), f.mul(p[1], p[1]), f.mul(p[2], p[2]), f.mul(p[0], p[1]), f.mul(p[0], p[2]), f.mul(p[1], p[2])];
    (g, hh, mo)
}

#[cfg(combo)]
pub use fastchain::*;
#[cfg(combo)]
mod fastchain {
use super::*;
// ---- lane "chain": compiled form of the same chain (time-only change) ----
// The interpreted op list is regrouped into segments "dst = src^(2^nsq) [* r[mul]]" and evaluated
// with a Barrett reduction (exact for q < 2^32) in place of the hardware u64 division. Same ops in
// the same order on the same values, so the result is identical; the counted multiplications are
// charged once per call as ch.cost(), exactly what Chain::eval charged op by op.
#[derive(Clone, Copy)]
pub struct Seg {
    pub dst: u8,
    pub src: u8,
    pub nsq: u16,
    pub mul: u8, // NOMUL = no trailing multiplication
}
pub const NOMUL: u8 = 255;
pub struct FastChain {
    pub segs: Vec<Seg>,
    pub out: u8,
    pub q: u64,
    pub m: u64, // floor(2^64 / q)
    pub cost: u64,
    pub use_chain: bool,
}
impl FastChain {
    pub fn new(ch: &Chain, q: u64) -> FastChain {
        assert!(q > 2 && q < (1u64 << 32));
        let m = ((1u128 << 64) / q as u128) as u64;
        let mut segs: Vec<Seg> = Vec::new();
        let mut i = 0;
        let ops = &ch.ops;
        while i < ops.len() {
            // a segment starts with Sq(d, s), continues with Sq(d, d)..., and may end with Mul(d, d, y)
            let (d, s) = match ops[i] {
                COp::Sq(d, s) => (d, s),
                COp::Mul(d, x, y) => {
                    // stray multiplication: d = x * y (not produced by Chain::new, kept for generality)
                    segs.push(Seg { dst: d, src: x, nsq: 0, mul: y });
                    i += 1;
                    continue;
                }
            };
            let mut nsq: u16 = 1;
            i += 1;
            while i < ops.len() {
                match ops[i] {
                    COp::Sq(d2, s2) if d2 == d && s2 == d => {
                        nsq += 1;
                        i += 1;
                    }
                    _ => break,
                }
            }
            let mut mul = NOMUL;
            if i < ops.len() {
                if let COp::Mul(d2, x, y) = ops[i] {
                    if d2 == d && x == d && y != d {
                        mul = y;
                        i += 1;
                    }
                }
            }
            segs.push(Seg { dst: d, src: s, nsq, mul });
        }
        // the segment form must charge exactly the op count
        let cost: u64 = segs.iter().map(|g| g.nsq as u64 + (g.mul != NOMUL) as u64).sum();
        assert_eq!(cost, ch.cost() as u64);
        FastChain { segs, out: ch.out, q, m, cost, use_chain: ch.cost() < ch.plain_cost }
    }
    #[inline(always)]
    fn red(&self, x: u64) -> u64 {
        // x < q^2 < 2^64: quo in {floor(x/q) - 1, floor(x/q)}
        let quo = ((x as u128 * self.m as u128) >> 64) as u64;
        let r = x - quo * self.q;
        if r >= self.q { r - self.q } else { r }
    }
    #[inline]
    pub fn eval_raw(&self, a: u64) -> u64 {
        let mut r = [0u64; 64];
        r[0] = a;
        for g in &self.segs {
            let mut x = r[g.src as usize];
            for _ in 0..g.nsq {
                x = self.red(x * x);
            }
            if g.mul != NOMUL {
                x = self.red(x * r[g.mul as usize]);
            }
            r[g.dst as usize] = x;
        }
        r[self.out as usize]
    }
}
/// Same value and same counted cost as pow_fixed(f, ch, a, e).
#[inline]
pub fn pow_fast(f: &Fp, fc: &FastChain, a: u64, e: u64) -> u64 {
    if fc.use_chain {
        bump_mul(fc.cost);
        fc.eval_raw(a)
    } else {
        f.pow(a, e)
    }
}
}
/// All pairs i < j of factor-base points: the residual quadratic of their line from the local data,
/// one square root, the two residual points (batch-inverted), classified as factor-base points or
/// large primes. Emitted once per line (from its two smallest factor-base indices).
pub fn find_relations_2lp(cv: &Curve, fb: &[P3]) -> DOut {
    let f = &cv.f;
    let q = f.q;
    let n = fb.len();
    let inv2 = f.inv(2);
    let mut idx = KeyIdx::with_capacity(n);
    for (i, p) in fb.iter().enumerate() {
        idx.insert_new(pack2(p), i as u32);
    }
    bump_hash(n as u64);
    let loc: Vec<([u64; 3], [u64; 6], [u64; 6])> = fb.iter().map(|p| local_data(cv, p, inv2)).collect();
    let mut out = DOut { full: vec![], edges: vec![], lp_pts: vec![], pairs: 0, rational_pairs: 0, dup_skipped: 0, unsure: 0, n1lp: 0, n2lp: 0, nloop: 0 };
    let mut vid = VidMap::with_capacity(n * n / 4);
    struct Pend {
        cs: Vec<(u32, u8)>,
        nl: u8,
        lp: [P3; 2],
        key: [u64; 2],
    }
    let mut pend: Vec<Pend> = Vec::with_capacity(n);
    let e_sqrt = (q + 1) / 4;
    let chain = Chain::new(e_sqrt);
    #[cfg(combo)]
    let fchain = FastChain::new(&chain, q);
    // per-i scratch
    struct Cand {
        j: u32,
        kind: u8, // 0: two roots u = (-g2 +- r) / (2 g3); 1: A tangent + root -g1 A + g2 B (point given); 2: double root; 3: A triple
        g2: u64,
        r: u64,
        g3: u64,
        pt: P3,
    }
    let nrm = std::env::var("NRM").is_ok();
    let mut cands: Vec<Cand> = Vec::with_capacity(n);
    for i in 0..n {
        let a = fb[i];
        let (ga, ha, _) = loc[i];
        cands.clear();
        for j in i + 1..n {
            let b = fb[j];
            let (gb, _, mb) = loc[j];
            out.pairs += 1;
            // g3 = grad F(A) . B ; g1 = grad F(B) . A ; g2 = 1/2 B^T H(A) B
            // normalised points (first coordinate 1): the products by that coordinate and by b0^2 are free
            let nb = nrm && b[0] == 1;
            let na = nrm && a[0] == 1;
            let g3 = f.add(f.add(if nb { ga[0] } else { f.mul(ga[0], b[0]) }, f.mul(ga[1], b[1])), f.mul(ga[2], b[2]));
            let g1 = f.add(f.add(if na { gb[0] } else { f.mul(gb[0], a[0]) }, f.mul(gb[1], a[1])), f.mul(gb[2], a[2]));
            let mut g2 = if nb { ha[0] } else { f.add(0, f.mul(ha[0], mb[0])) };
            for t in 1..6 {
                g2 = f.add(g2, f.mul(ha[t], mb[t]));
            }
            if g3 == 0 {
                if g2 == 0 {
                    if g1 == 0 {
                        out.unsure += 1;
                        continue;
                    }
                    cands.push(Cand { j: j as u32, kind: 3, g2, r: 0, g3, pt: [0; 3] });
                } else {
                    // roots (1:0) = A and (-g1 : g2)
                    let p = [
                        f.add(f.mul(f.neg(g1), a[0]), f.mul(g2, b[0])),
                        f.add(f.mul(f.neg(g1), a[1]), f.mul(g2, b[1])),
                        f.add(f.mul(f.neg(g1), a[2]), f.mul(g2, b[2])),
                    ];
                    cands.push(Cand { j: j as u32, kind: 1, g2, r: 0, g3, pt: p });
                }
                continue;
            }
            let disc = if nrm {
                let t = f.add(g1, g1);
                f.sub(f.mul(g2, g2), f.mul(f.add(t, t), g3))
            } else {
                f.sub(f.mul(g2, g2), f.mul(f.mul(4, g1), g3))
            };
            if disc == 0 {
                cands.push(Cand { j: j as u32, kind: 2, g2, r: 0, g3, pt: [0; 3] });
                continue;
            }
            #[cfg(combo)]
            let r = pow_fast(f, &fchain, disc, e_sqrt);
            #[cfg(not(combo))]
            let r = pow_fixed(f, &chain, disc, e_sqrt);
            if f.mul(r, r) != disc {
                continue; // residual irreducible over F_q
            }
            cands.push(Cand { j: j as u32, kind: 0, g2, r, g3, pt: [0; 3] });
        }
        out.rational_pairs += cands.len() as u64;
        // batch inversion of 2 g3 (kinds 0, 2)
        let mut den: Vec<u64> = cands.iter().map(|c| if c.kind == 0 || c.kind == 2 { f.add(c.g3, c.g3) } else { 1 }).collect();
        batch_inv(f, &mut den);
        // residual points (unnormalised), then batch-normalise
        let mut pts: Vec<P3> = Vec::with_capacity(2 * cands.len());
        let mut owner: Vec<(usize, u8)> = Vec::with_capacity(2 * cands.len()); // (cand, slot)
        for (ci, c) in cands.iter().enumerate() {
            let b = fb[c.j as usize];
            let lin = |u: u64| -> P3 { [if nrm && a[0] == 1 { f.add(u, b[0]) } else { f.add(f.mul(u, a[0]), b[0]) }, f.add(f.mul(u, a[1]), b[1]), f.add(f.mul(u, a[2]), b[2])] };
            match c.kind {
                0 => {
                    let ng2 = f.neg(c.g2);
                    let u1 = f.mul(f.add(ng2, c.r), den[ci]);
                    let u2 = f.mul(f.sub(ng2, c.r), den[ci]);
                    pts.push(lin(u1));
                    owner.push((ci, 0));
                    pts.push(lin(u2));
                    owner.push((ci, 1));
                }
                1 => {
                    pts.push(c.pt);
                    owner.push((ci, 0));
                }
                2 => {
                    let u0 = f.mul(f.neg(c.g2), den[ci]);
                    pts.push(lin(u0));
                    owner.push((ci, 0));
                }
                _ => {}
            }
        }
        let mut pv: Vec<u64> = pts.iter().map(|p| *p.iter().find(|x| **x != 0).unwrap_or(&1)).collect();
        batch_inv(f, &mut pv);
        for (k, p) in pts.iter_mut().enumerate() {
            if pv[k] != 1 {
                if nrm && p[0] != 0 {
                    // the normalising coordinate becomes 1 without a multiplication
                    *p = [1, f.mul(p[1], pv[k]), f.mul(p[2], pv[k])];
                } else {
                    *p = [f.mul(p[0], pv[k]), f.mul(p[1], pv[k]), f.mul(p[2], pv[k])];
                }
            }
        }
        // hash lane: per-candidate classification without per-candidate allocations (pass 1), then
        // large-prime vertex ids in candidate order with software prefetch (pass 2). Same pushes, same
        // order, same counters as v3.
        let mut off: usize = 0;
        pend.clear();
        for c in cands.iter() {
            let np = match c.kind {
                0 => 2,
                1 | 2 => 1,
                _ => 0,
            };
            let base = off;
            off += np;
            let j = c.j;
            let b = fb[j as usize];
            let mut ma: u8 = 1;
            let mut mb: u8 = 1;
            let mut fbres: [u32; 2] = [0; 2];
            let mut nf = 0usize;
            let mut lp: [P3; 2] = [[0; 3]; 2];
            let mut nl = 0usize;
            let mut resid: [P3; 2] = [[0; 3]; 2];
            let nr = match c.kind {
                0 => {
                    resid[0] = pts[base];
                    resid[1] = pts[base + 1];
                    2
                }
                1 => {
                    ma += 1;
                    resid[0] = pts[base];
                    1
                }
                2 => {
                    resid[0] = pts[base];
                    resid[1] = pts[base];
                    2
                }
                _ => {
                    ma += 2;
                    0
                }
            };
            for t in 0..nr {
                let p = resid[t];
                if p == a {
                    ma += 1;
                } else if p == b {
                    mb += 1;
                } else {
                    bump_hash(1);
                    match idx.get(pack2(&p)) {
                        Some(k) => {
                            fbres[nf] = k;
                            nf += 1;
                        }
                        None => {
                            lp[nl] = p;
                            nl += 1;
                        }
                    }
                }
            }
            if fbres[..nf].iter().any(|&k| k < j) {
                out.dup_skipped += 1;
                continue;
            }
            let mut cs: Vec<(u32, u8)> = Vec::with_capacity(2 + nf);
            cs.push((i as u32, ma));
            cs.push((j, mb));
            if nf == 2 && fbres[1] < fbres[0] {
                fbres.swap(0, 1);
            }
            for &k in &fbres[..nf] {
                if let Some(last) = cs.last_mut() {
                    if last.0 == k {
                        last.1 += 1;
                        continue;
                    }
                }
                cs.push((k, 1));
            }
            let key = [pack2(&lp[0]), pack2(&lp[1])];
            pend.push(Pend { cs, nl: nl as u8, lp, key });
        }
        // pass 2
        const PF: usize = 8;
        for k in 0..pend.len().min(PF) {
            for t in 0..pend[k].nl as usize {
                vid.prefetch(pend[k].key[t]);
            }
        }
        for k in 0..pend.len() {
            if k + PF < pend.len() {
                let e = &pend[k + PF];
                for t in 0..e.nl as usize {
                    vid.prefetch(e.key[t]);
                }
            }
            let e = &mut pend[k];
            let cs = std::mem::take(&mut e.cs);
            let mut ids: [u32; 2] = [0; 2];
            for t in 0..e.nl as usize {
                bump_hash(1);
                let p = e.lp[t];
                let lp_pts = &mut out.lp_pts;
                ids[t] = vid.get_or_insert_with(e.key[t], || {
                    lp_pts.push(p);
                    lp_pts.len() as u32
                });
            }
            match e.nl {
                0 => out.full.push(DRel { c: cs, u: u32::MAX, v: u32::MAX }),
                1 => {
                    out.n1lp += 1;
                    out.edges.push(DRel { c: cs, u: ids[0], v: 0 })
                }
                _ => {
                    if ids[0] == ids[1] {
                        out.nloop += 1;
                    } else {
                        out.n2lp += 1;
                    }
                    out.edges.push(DRel { c: cs, u: ids[0], v: ids[1] })
                }
            }
        }
    }
    out
}

/// Montgomery batch inversion in place (all entries nonzero).
pub fn batch_inv(f: &Fp, v: &mut [u64]) {
    if v.is_empty() {
        return;
    }
    let mut pref = Vec::with_capacity(v.len());
    let mut acc = 1u64;
    for &x in v.iter() {
        pref.push(acc);
        acc = f.mul(acc, x);
    }
    let mut ia = f.inv(acc);
    for k in (0..v.len()).rev() {
        let x = v[k];
        v[k] = f.mul(ia, pref[k]);
        ia = f.mul(ia, x);
    }
}

// ------------------------------------------------------------------ large-prime graph
/// A linear form over the matrix columns (fb[1..] -> 0..n-2, kappa -> n-1) with small integer
/// coefficients. The log equation of a line relation is  sum m_i x_i + sum x_L = eta.
pub type Form = Vec<(u32, i64)>;

pub fn form_add(acc: &mut BTreeMap<u32, i64>, f: &Form, s: i64) {
    for &(c, v) in f {
        *acc.entry(c).or_insert(0) += s * v;
    }
}

pub struct Graph {
    pub nv: usize,
    pub par: Vec<u32>,
    pub pe: Vec<u32>, // parent edge (u32::MAX for tree roots / unvisited)
    pub depth: Vec<u32>,
    pub troot: Vec<u32>,
    pub order: Vec<u32>, // BFS order of visited vertices
    pub tree_edge: Vec<bool>,
    pub labels: Vec<Form>, // r_e = eta - sum m_i x_i
    pub anchor: Vec<Option<Form>>, // per tree root: 2 x_t = T
    pub comps: u64,
}

/// r_e for a relation: eta - sum m_i x_i  (P0 = fb[0] has log 0 and is dropped).
pub fn label_of(c: &[(u32, u8)], n: usize) -> Form {
    let mut v: Form = Vec::new();
    for &(i, m) in c {
        if i != 0 {
            v.push((i - 1, -(m as i64)));
        }
    }
    v.push(((n - 1) as u32, 1));
    v
}

pub fn build_graph(d: &DOut, n: usize) -> Graph {
    let nv = d.lp_pts.len() + 1;
    let ne = d.edges.len();
    // CSR adjacency in edge order
    let mut deg = vec![0u32; nv + 1];
    for e in &d.edges {
        deg[e.u as usize] += 1;
        if e.v != e.u {
            deg[e.v as usize] += 1;
        }
    }
    let mut start = vec![0usize; nv + 1];
    for v in 0..nv {
        start[v + 1] = start[v] + deg[v] as usize;
    }
    let mut fill = start.clone();
    let mut adj = vec![0u32; start[nv]];
    bump_hash(2 * ne as u64); // adjacency build and BFS traversal (unpriced native work)
    for (ei, e) in d.edges.iter().enumerate() {
        adj[fill[e.u as usize]] = ei as u32;
        fill[e.u as usize] += 1;
        if e.v != e.u {
            adj[fill[e.v as usize]] = ei as u32;
            fill[e.v as usize] += 1;
        }
    }
    let mut g = Graph {
        nv,
        par: vec![u32::MAX; nv],
        pe: vec![u32::MAX; nv],
        depth: vec![0; nv],
        troot: vec![u32::MAX; nv],
        order: Vec::with_capacity(nv),
        tree_edge: vec![false; ne],
        labels: d.edges.iter().map(|e| label_of(&e.c, n)).collect(),
        anchor: vec![None; nv],
        comps: 0,
    };
    let mut queue: std::collections::VecDeque<u32> = std::collections::VecDeque::new();
    for s in 0..nv {
        if g.troot[s] != u32::MAX || (s != 0 && deg[s] == 0) {
            continue;
        }
        g.comps += 1;
        g.troot[s] = s as u32;
        queue.push_back(s as u32);
        while let Some(x) = queue.pop_front() {
            g.order.push(x);
            for k in start[x as usize]..start[x as usize + 1] {
                let ei = adj[k] as usize;
                let e = &d.edges[ei];
                let y = if e.u == x { e.v } else { e.u };
                if g.troot[y as usize] == u32::MAX {
                    g.troot[y as usize] = s as u32;
                    g.par[y as usize] = x;
                    g.pe[y as usize] = ei as u32;
                    g.depth[y as usize] = g.depth[x as usize] + 1;
                    g.tree_edge[ei] = true;
                    queue.push_back(y);
                }
            }
        }
    }
    g
}

impl Graph {
    /// x_c = F + s * x_w for the ancestor w of c.
    fn walk(&self, mut c: u32, w: u32, acc: &mut BTreeMap<u32, i64>, scale: i64) -> i64 {
        let mut s = 1i64;
        while c != w {
            bump_hash(1);
            let e = self.pe[c as usize] as usize;
            form_add(acc, &self.labels[e], scale * s);
            s = -s;
            c = self.par[c as usize];
        }
        s
    }
    fn lca(&self, mut u: u32, mut v: u32) -> u32 {
        while self.depth[u as usize] > self.depth[v as usize] {
            u = self.par[u as usize];
        }
        while self.depth[v as usize] > self.depth[u as usize] {
            v = self.par[v as usize];
        }
        while u != v {
            u = self.par[u as usize];
            v = self.par[v as usize];
        }
        u
    }
}

/// Factor-base relations from the non-tree edges, lightest (by d(u) + d(v)) first, until `need` rows.
/// Returns rows (mod ell) and the counts of anchors / skipped edges.
pub fn cycle_rows(g: &mut Graph, d: &DOut, need: usize, ell: u64) -> (Vec<Row>, u64, u64) {
    let mut nt: Vec<(u32, u32)> = Vec::new();
    for (ei, e) in d.edges.iter().enumerate() {
        if !g.tree_edge[ei] {
            nt.push((g.depth[e.u as usize] + g.depth[e.v as usize], ei as u32));
        }
    }
    nt.sort_unstable();
    let mut rows: Vec<Row> = Vec::new();
    let mut anchors = 0u64;
    let mut skipped = 0u64;
    for &(_, ei) in &nt {
        if rows.len() >= need {
            break;
        }
        let e = &d.edges[ei as usize];
        let (u, v) = (e.u, e.v);
        let w = g.lca(u, v);
        let mut a: BTreeMap<u32, i64> = BTreeMap::new();
        // a := F_u + F_v ; parities
        let pu = g.walk(u, w, &mut a, 1);
        let pv = g.walk(v, w, &mut a, 1);
        // equation: a + (pu + pv) x_w = r_e
        let re = g.labels[ei as usize].clone();
        let mut row: BTreeMap<u32, i64> = BTreeMap::new();
        if pu + pv == 0 {
            form_add(&mut row, &a.iter().map(|(&c, &x)| (c, x)).collect(), 1);
            form_add(&mut row, &re, -1);
        } else {
            let p = pu; // pu == pv
            // R = r_e - a ; 2p x_w = R ; x_w = F_w + pw x_t
            let mut rr: BTreeMap<u32, i64> = BTreeMap::new();
            form_add(&mut rr, &re, 1);
            form_add(&mut rr, &a.iter().map(|(&c, &x)| (c, x)).collect(), -1);
            let t = g.troot[w as usize];
            let mut fw: BTreeMap<u32, i64> = BTreeMap::new();
            let pw = g.walk(w, t, &mut fw, 1);
            let fwv: Form = fw.iter().map(|(&c, &x)| (c, x)).collect();
            let rrv: Form = rr.iter().map(|(&c, &x)| (c, x)).collect();
            if t == 0 {
                form_add(&mut row, &rrv, 1);
                form_add(&mut row, &fwv, -2 * p);
            } else if let Some(tf) = g.anchor[t as usize].clone() {
                form_add(&mut row, &rrv, 1);
                form_add(&mut row, &fwv, -2 * p);
                form_add(&mut row, &tf, -p * pw);
            } else {
                // 2 x_t = pw (p R - 2 F_w)
                let mut tf: BTreeMap<u32, i64> = BTreeMap::new();
                form_add(&mut tf, &rrv, pw * p);
                form_add(&mut tf, &fwv, -2 * pw);
                g.anchor[t as usize] = Some(tf.into_iter().filter(|x| x.1 != 0).collect());
                anchors += 1;
                continue;
            }
        }
        let r: Row = row.into_iter().filter(|x| x.1 != 0).map(|(c, x)| (c, to_mod(x, ell))).filter(|x| x.1 != 0).collect();
        if r.is_empty() {
            skipped += 1;
            continue;
        }
        rows.push(r);
    }
    (rows, anchors, skipped)
}

/// Logs of the large primes reachable from the root or from an anchored tree root, by propagation
/// along the BFS order: x_c = r_e - x_par. `xcol(c)` = log of matrix column c (None if unknown).
pub fn propagate_logs(g: &Graph, d: &DOut, xcol: &dyn Fn(u32) -> Option<u64>, ell: u64) -> Vec<Option<u64>> {
    let mut x: Vec<Option<u64>> = vec![None; g.nv];
    let evalf = |f: &Form| -> Option<u64> {
        let mut s = 0u64;
        for &(c, v) in f {
            let xv = xcol(c)?;
            let m = to_mod(v, ell);
            s = if m == 1 { la2(s, xv, ell) } else if m == ell - 1 { ls2(s, xv, ell) } else { la2(s, lm2(m, xv, ell), ell) };
        }
        Some(s)
    };
    let inv2 = (ell + 1) / 2;
    for &c in &g.order {
        let cu = c as usize;
        if g.troot[cu] == c {
            if c == 0 {
                x[0] = Some(0);
            } else if let Some(tf) = &g.anchor[cu] {
                x[cu] = evalf(tf).map(|t| lm2(t, inv2, ell));
            }
            continue;
        }
        let p = g.par[cu] as usize;
        if let (Some(xp), Some(re)) = (x[p], evalf(&g.labels[g.pe[cu] as usize])) {
            x[cu] = Some(ls2(re, xp, ell));
        }
    }
    let _ = d;
    x
}

// ---- v3: fixed-exponent addition chain for the per-pair square root (same result as Fp::pow) ----
#[derive(Clone, Copy)]
pub enum COp {
    Sq(u8, u8),
    Mul(u8, u8, u8),
}
pub struct Chain {
    pub ops: Vec<COp>,
    pub out: u8,
    pub plain_cost: usize,
}
impl Chain {
    pub fn new(e: u64) -> Chain {
        assert!(e > 0);
        let bits = 64 - e.leading_zeros() as usize;
        let plain_cost = bits - 1 + e.count_ones() as usize;
        let mut ops: Vec<COp> = Vec::new();
        let mut nreg: u8 = 1; // reg 0 = a = T[1]
        let mut t: HashMap<u32, u8> = HashMap::new();
        t.insert(1, 0);
        fn tr(l: u32, ops: &mut Vec<COp>, nreg: &mut u8, t: &mut HashMap<u32, u8>) -> u8 {
            if let Some(&r) = t.get(&l) {
                return r;
            }
            let p = if l.is_power_of_two() { l / 2 } else { 1u32 << (31 - l.leading_zeros()) };
            let rest = l - p; // T[l] = T[p]^(2^rest) * T[rest]
            let tp = tr(p, ops, nreg, t);
            let tl = tr(rest, ops, nreg, t);
            let d = *nreg;
            *nreg += 1;
            ops.push(COp::Sq(d, tp));
            for _ in 1..rest {
                ops.push(COp::Sq(d, d));
            }
            ops.push(COp::Mul(d, d, tl));
            t.insert(l, d);
            d
        }
        // runs from the most significant bit
        let mut runs: Vec<(u8, u32)> = Vec::new();
        for i in (0..bits).rev() {
            let b = ((e >> i) & 1) as u8;
            match runs.last_mut() {
                Some((bb, n)) if *bb == b => *n += 1,
                _ => runs.push((b, 1)),
            }
        }
        let first = tr(runs[0].1, &mut ops, &mut nreg, &mut t);
        let acc = nreg;
        nreg += 1;
        let mut acc_init = false;
        let mut pending_sq = 0u32;
        let mut i = 1;
        // acc = T[first run]
        let mut cur = first;
        while i < runs.len() {
            let (b, n) = runs[i];
            if b == 0 {
                pending_sq += n;
                i += 1;
                continue;
            }
            let tl = tr(n, &mut ops, &mut nreg, &mut t);
            let sq = pending_sq + n;
            for k in 0..sq {
                ops.push(COp::Sq(acc, if k == 0 && !acc_init { cur } else { acc }));
            }
            acc_init = true;
            ops.push(COp::Mul(acc, acc, tl));
            cur = acc;
            pending_sq = 0;
            i += 1;
        }
        for k in 0..pending_sq {
            ops.push(COp::Sq(acc, if k == 0 && !acc_init { cur } else { acc }));
            acc_init = true;
            cur = acc;
        }
        assert!(nreg < 64);
        let out = if acc_init { acc } else { first };
        Chain { ops, out, plain_cost }
    }
    pub fn cost(&self) -> usize {
        self.ops.len()
    }
    #[inline]
    pub fn eval(&self, f: &Fp, a: u64) -> u64 {
        let mut r = [0u64; 64];
        r[0] = a;
        for op in &self.ops {
            match *op {
                COp::Sq(d, s) => r[d as usize] = f.mul(r[s as usize], r[s as usize]),
                COp::Mul(d, x, y) => r[d as usize] = f.mul(r[x as usize], r[y as usize]),
            }
        }
        r[self.out as usize]
    }
}
/// a^e through the chain when it is shorter than square-and-multiply, else Fp::pow.
#[inline]
pub fn pow_fixed(f: &Fp, ch: &Chain, a: u64, e: u64) -> u64 {
    if ch.cost() < ch.plain_cost {
        ch.eval(f, a)
    } else {
        f.pow(a, e)
    }
}

// ---- hash lane: open-addressing tables (lookup/insert only, never iterated) ----
#[inline(always)]
fn mix(k: u64) -> u64 {
    (k ^ (k >> 29)).wrapping_mul(0x9E37_79B9_7F4A_7C15)
}
/// Factor-base index: keys (pack2, never 0) in their own array so absent-key probes touch only keys.
pub struct KeyIdx {
    keys: Vec<u64>,
    vals: Vec<u32>,
    shift: u32,
    mask: usize,
}
impl KeyIdx {
    pub fn with_capacity(n: usize) -> KeyIdx {
        let cap = (8 * n.max(1)).next_power_of_two().max(16);
        KeyIdx { keys: vec![0; cap], vals: vec![0; cap], shift: 64 - cap.trailing_zeros(), mask: cap - 1 }
    }
    /// insert a key not yet present (pack2 is injective and fb has distinct points)
    pub fn insert_new(&mut self, k: u64, v: u32) {
        debug_assert!(k != 0);
        let mut h = (mix(k) >> self.shift) as usize;
        loop {
            if self.keys[h] == 0 {
                self.keys[h] = k;
                self.vals[h] = v;
                return;
            }
            assert!(self.keys[h] != k, "duplicate factor-base key");
            h = (h + 1) & self.mask;
        }
    }
    #[inline(always)]
    pub fn get(&self, k: u64) -> Option<u32> {
        let mut h = (mix(k) >> self.shift) as usize;
        loop {
            let x = unsafe { *self.keys.get_unchecked(h) };
            if x == k {
                return Some(self.vals[h]);
            }
            if x == 0 {
                return None;
            }
            h = (h + 1) & self.mask;
        }
    }
}
/// Large-prime vertex ids: interleaved (key, value) slots, linear probing, load <= 1/2.
#[derive(Clone, Copy)]
#[repr(C, align(16))]
struct Slot {
    k: u64,
    v: u32,
}
pub struct VidMap {
    t: Vec<Slot>,
    shift: u32,
    mask: usize,
    len: usize,
}
impl VidMap {
    pub fn with_capacity(n: usize) -> VidMap {
        let cap = (2 * n.max(1)).next_power_of_two().max(1024);
        VidMap { t: vec![Slot { k: 0, v: 0 }; cap], shift: 64 - cap.trailing_zeros(), mask: cap - 1, len: 0 }
    }
    #[inline(always)]
    pub fn prefetch(&self, k: u64) {
        let h = (mix(k) >> self.shift) as usize;
        #[cfg(target_arch = "x86_64")]
        unsafe {
            use std::arch::x86_64::{_mm_prefetch, _MM_HINT_T0};
            _mm_prefetch(self.t.as_ptr().add(h) as *const i8, _MM_HINT_T0);
        }
    }
    fn grow(&mut self) {
        let cap = 2 * self.t.len();
        let old = std::mem::replace(&mut self.t, vec![Slot { k: 0, v: 0 }; cap]);
        self.shift = 64 - cap.trailing_zeros();
        self.mask = cap - 1;
        for s in old {
            if s.k != 0 {
                let mut h = (mix(s.k) >> self.shift) as usize;
                while self.t[h].k != 0 {
                    h = (h + 1) & self.mask;
                }
                self.t[h] = s;
            }
        }
    }
    #[inline(always)]
    pub fn get_or_insert_with<F: FnOnce() -> u32>(&mut self, k: u64, f: F) -> u32 {
        debug_assert!(k != 0);
        let mut h = (mix(k) >> self.shift) as usize;
        loop {
            let s = unsafe { *self.t.get_unchecked(h) };
            if s.k == k {
                return s.v;
            }
            if s.k == 0 {
                break;
            }
            h = (h + 1) & self.mask;
        }
        let v = f();
        self.t[h] = Slot { k, v };
        self.len += 1;
        if 2 * self.len > self.t.len() {
            self.grow();
        }
        v
    }
}
