//! Genus-4 non-hyperelliptic (3,3) curves, round 2: the relation phase finds
//! the residual points of a plane section by exact root finding (a binary
//! cubic over F_p) and a hash lookup into the factor base instead of scanning
//! it; partial relations with one or two residual points outside the factor
//! base are merged by the repository's large-prime filter; the factor base is
//! sampled by root finding and sized by the rule m = ceil(c * 1.25^rung *
//! p^alpha); the individual logarithm decomposes over the points whose logs
//! are known.  Local development check only.
//!
//! Subcommands: `dlp P CURVE BASESEED RHO_TARGETS OUT GIT`, `yield P K SEED OUT GIT`.
#[path = "../../research/low-gonality-genus4_20261115/src/g4.rs"]
mod g4;
#[path = "../../research/low-gonality-genus4_20261115/src/ic.rs"]
mod ic;
#[path = "../../research/low-gonality-genus4_20261121/src/rf.rs"]
mod rf;

use crypto_lib::cryptanalysis::jv_bitops;
use crypto_lib::cryptanalysis::large_prime_filter::{
    filter_large_primes, LargePrimeFilterOptions, PartialRelation,
};
use g4::*;
use ic::*;
use rf::*;
use serde_json::{json, Value};
use std::cell::Cell;
use std::collections::{HashMap, HashSet};
use std::time::Instant;

const RUNGS: usize = 12;
const RUNG_MULT: f64 = 1.25;

pub fn random_curve(p: u32, rng: &mut u64) -> Curve {
    loop {
        let mut g = [0u32; 16];
        for x in g.iter_mut() {
            *x = rnd(rng, p as u64) as u32;
        }
        if let Ok(mut c) = Curve::new(p, g) {
            if c.set_base(rng).is_ok() {
                return c;
            }
        }
    }
}

fn pack(q: &[u32; 4]) -> u64 {
    ((q[0] as u64) << 48) | ((q[1] as u64) << 32) | ((q[2] as u64) << 16) | q[3] as u64
}

/// Plane (as a linear form on [tv, tu, sv, su]) through three points of P^3.
fn plane3(a: &[u64; 4], b: &[u64; 4], c: &[u64; 4], p: u64, ctr: &Cell<u64>) -> [u64; 4] {
    let m = [a, b, c];
    let mut out = [0u64; 4];
    for (k, o) in out.iter_mut().enumerate() {
        let cols: Vec<usize> = (0..4).filter(|&x| x != k).collect();
        let g = |r: usize, j: usize| m[r][cols[j]];
        let t = |x: u64, y: u64| (mm(x, y, p, ctr) + p) % p;
        let d1 = (t(g(1, 1), g(2, 2)) + p - t(g(1, 2), g(2, 1))) % p;
        let d2 = (t(g(1, 2), g(2, 0)) + p - t(g(1, 0), g(2, 2))) % p;
        let d3 = (t(g(1, 0), g(2, 1)) + p - t(g(1, 1), g(2, 0))) % p;
        let det = (mm(g(0, 0), d1, p, ctr) + mm(g(0, 1), d2, p, ctr) + mm(g(0, 2), d3, p, ctr)) % p;
        *o = if k % 2 == 0 { det } else { (p - det) % p };
    }
    out
}

/// Binary cubic of the fibre `key` of ruling 0 (fixed (s:t), polynomial in
/// w = u/v) or ruling 1 (fixed (u:v), polynomial in w = t/s).
fn fibre_cubic(g: &[u32; 16], p: u64, ruling: usize, key: u32, ctr: &Cell<u64>) -> Poly {
    let (a, b) = if key as u64 == p {
        if ruling == 0 {
            (0u64, 1u64)
        } else {
            (1, 0)
        }
    } else if ruling == 0 {
        (1, key as u64)
    } else {
        (key as u64, 1)
    };
    // ruling 0: (a,b) = (s,t); ruling 1: (a,b) = (u,v)
    let pows = |x: u64| -> [u64; 4] {
        let mut v = [1u64; 4];
        for k in 1..4 {
            v[k] = mm(v[k - 1], x, p, ctr);
        }
        v
    };
    let (pa, pb) = (pows(a), pows(b));
    let mut out = vec![0u64; 4];
    for i in 0..4 {
        for j in 0..4 {
            let cf = g[4 * i + j] as u64;
            if cf == 0 {
                continue;
            }
            if ruling == 0 {
                // coefficient of u^j v^(3-j): g * s^i t^(3-i)
                let t = mm(pa[i], pb[3 - i], p, ctr);
                out[j] = (out[j] + mm(cf, t, p, ctr)) % p;
            } else {
                // coefficient of s^i t^(3-i) -> w^(3-i): g * u^j v^(3-j)
                let t = mm(pa[j], pb[3 - j], p, ctr);
                out[3 - i] = (out[3 - i] + mm(cf, t, p, ctr)) % p;
            }
        }
    }
    out
}

/// G restricted to the conic L = 0, as a polynomial in t (s = 1).
fn plane_sextic(g: &[u32; 16], l: &[u64; 4], p: u64, ctr: &Cell<u64>) -> Poly {
    // L = u (l3 s + l1 t) + v (l2 s + l0 t)  =>  (u:v) = (-(l2 s + l0 t) : l3 s + l1 t)
    let u: Poly = vec![(p - l[2] % p) % p, (p - l[0] % p) % p];
    let v: Poly = vec![l[3] % p, l[1] % p];
    let mut up: Vec<Poly> = vec![vec![1]];
    let mut vp: Vec<Poly> = vec![vec![1]];
    for k in 1..4 {
        up.push(pmul(&up[k - 1], &u, p, ctr));
        vp.push(pmul(&vp[k - 1], &v, p, ctr));
    }
    let mut out = vec![0u64; 7];
    for j in 0..4 {
        // B_j(t) = sum_i g[4i+j] t^(3-i)
        let mut b = [0u64; 4];
        for i in 0..4 {
            b[3 - i] = g[4 * i + j] as u64;
        }
        let mut t = vec![b[0], b[1], b[2], b[3]];
        while t.last() == Some(&0) {
            t.pop();
        }
        let uv = pmul(&up[j], &vp[3 - j], p, ctr);
        let term = pmul(&t, &uv, p, ctr);
        for (k, &c) in term.iter().enumerate() {
            out[k] = (out[k] + c) % p;
        }
    }
    while out.last() == Some(&0) {
        out.pop();
    }
    out
}

#[derive(Clone, Copy)]
struct Cand {
    q: [u32; 4],
    f: Option<usize>,
}

/// A set of known points (the factor base), D0 first, Pa = pts[0].
struct Fset {
    p: u64,
    g: [u32; 16],
    pts: Vec<[u32; 4]>,
    ev1: Vec<[u64; 4]>,
    map: HashMap<u64, usize>,
    fib: [HashMap<u32, Vec<usize>>; 2],
}

fn key_of(p: u64, q: &[u32; 4], ruling: usize) -> u32 {
    if ruling == 0 {
        if q[0] == 0 {
            p as u32
        } else {
            q[1]
        }
    } else if q[3] == 0 {
        p as u32
    } else {
        q[2]
    }
}

impl Fset {
    fn new(c: &Curve, pts: Vec<[u32; 4]>) -> Fset {
        let mut f = Fset {
            p: c.pp(),
            g: c.g,
            pts: vec![],
            ev1: vec![],
            map: HashMap::new(),
            fib: [HashMap::new(), HashMap::new()],
        };
        for q in pts {
            f.push(c, q);
        }
        f
    }
    fn push(&mut self, c: &Curve, q: [u32; 4]) -> bool {
        if self.map.contains_key(&pack(&q)) {
            return false;
        }
        let i = self.pts.len();
        let v = c.eval_vec(1, &q);
        self.ev1
            .push([v[0] as u64, v[1] as u64, v[2] as u64, v[3] as u64]);
        self.map.insert(pack(&q), i);
        for r in 0..2 {
            self.fib[r]
                .entry(key_of(self.p, &q, r))
                .or_default()
                .push(i);
        }
        self.pts.push(q);
        true
    }
    fn len(&self) -> usize {
        self.pts.len()
    }
    /// Find the F index of the point (s:t)=(s,t), (u:v)=(uu,vv) (any scaling).
    fn lookup(&self, s: u64, t: u64, uu: u64, vv: u64, ctr: &Cell<u64>) -> Option<usize> {
        let p = self.p;
        let key = if s == 0 { p as u32 } else { t as u32 };
        ctr.set(ctr.get() + 1); // hash probe
        let list = self.fib[0].get(&key)?;
        for &i in list {
            let q = &self.pts[i];
            // (q2:q3) == (uu:vv)  <=>  q2*vv - q3*uu = 0
            let d = (mm(q[2] as u64, vv, p, ctr) + p - mm(q[3] as u64, uu, p, ctr)) % p;
            if d == 0 {
                return Some(i);
            }
        }
        None
    }
}

#[derive(Default, Clone, Debug)]
struct CStats {
    pairs: u64,
    zero_plane: u64,
    degenerate: u64,
    split: u64,
    /// number of A planes whose three residual points have k points in F (k = 0..3)
    n_f: [u64; 4],
    anomalies: u64,
    fibres_probed: u64,
    fibres_split: u64,
    native: u64,
}

struct Raw {
    f: Vec<u32>,
    lp: Vec<u64>,
    special: u32, // 0 = kappa, 1 = tau0, 2 = tau1
    kind: u8,     // 0 = A, 1 = B
}

fn canon_point(p: u64, s: u64, t: u64, uu: u64, vv: u64, ctr: &Cell<u64>) -> [u32; 4] {
    let (cs, ct) = if s == 0 {
        (0u32, 1u32)
    } else {
        (1u32, t as u32)
    };
    let (cu, cv) = if vv == 0 {
        (1u32, 0u32)
    } else {
        let iv = invm(vv, p, ctr);
        (mm(uu, iv, p, ctr) as u32, 1u32)
    };
    [cs, ct, cu, cv]
}

impl Fset {
    /// Residual points of the plane through Pa, F[a], F[b]: Some(three
    /// distinct rational points, none of them one of the three) or None.
    fn residual(&self, a: usize, b: usize, ctr: &Cell<u64>, st: &mut CStats) -> Option<[Cand; 3]> {
        let p = self.p;
        let l = plane3(&self.ev1[0], &self.ev1[a], &self.ev1[b], p, ctr);
        if l == [0; 4] {
            st.zero_plane += 1;
            return None;
        }
        let det = (mm(l[3], l[0], p, ctr) + p - mm(l[2], l[1], p, ctr)) % p;
        if det == 0 {
            // degenerate plane section: scan the factor base (rare, ~1/p)
            st.degenerate += 1;
            let mut others = vec![];
            for q in 0..self.len() {
                if q == 0 || q == a || q == b {
                    continue;
                }
                let e = (0..4).map(|k| l[k] * self.ev1[q][k]).sum::<u64>() % p;
                ctr.set(ctr.get() + 4);
                if e == 0 {
                    others.push(Cand {
                        q: self.pts[q],
                        f: Some(q),
                    });
                }
            }
            return if others.len() == 3 {
                Some([others[0], others[1], others[2]])
            } else {
                None
            };
        }
        let mut f = plane_sextic(&self.g, &l, p, ctr);
        let known = [0usize, a, b];
        let mut known_st: Vec<Root> = vec![];
        for &k in &known {
            let q = &self.pts[k];
            if q[0] == 0 {
                known_st.push(Root::Inf);
            } else {
                known_st.push(Root::Fin(q[1] as u64));
                match div_linear(&f, q[1] as u64, p, ctr) {
                    Some(nf) => f = nf,
                    None => {
                        st.anomalies += 1;
                        return None;
                    }
                }
            }
        }
        let rs = distinct_roots(&f, 3, p, ctr)?;
        if !rs.complete {
            return None;
        }
        st.split += 1;
        let mut out: Vec<Cand> = vec![];
        for r in &rs.roots {
            if known_st.contains(r) {
                return None; // tangency: residual root meets a known point
            }
            let (s, t) = match *r {
                Root::Fin(x) => (1u64, x),
                Root::Inf => (0u64, 1u64),
            };
            // (u:v) = (-(l2 s + l0 t) : l3 s + l1 t)
            let uu = (p - (mm(l[2], s, p, ctr) + mm(l[0], t, p, ctr)) % p) % p;
            let vv = (mm(l[3], s, p, ctr) + mm(l[1], t, p, ctr)) % p;
            match self.lookup(s, t, uu, vv, ctr) {
                Some(i) => out.push(Cand {
                    q: self.pts[i],
                    f: Some(i),
                }),
                None => out.push(Cand {
                    q: canon_point(p, s, t, uu, vv, ctr),
                    f: None,
                }),
            }
        }
        if out.len() != 3 {
            st.anomalies += 1;
            return None;
        }
        Some([out[0], out[1], out[2]])
    }
}

struct Collected {
    raws: Vec<Raw>,
    st: CStats,
}

/// Relations from the factor base `fs`: A (plane sections through Pa, two F
/// points and three residual points, at most two outside F) and B (fibres of
/// both rulings through an F point, at most two points outside F).
fn collect_rf(c: &Curve, fs: &Fset, do_b: bool) -> Collected {
    let ctr = Cell::new(0u64);
    let mut st = CStats::default();
    let mut raws: Vec<Raw> = vec![];
    let m = fs.len();
    let mut seen: HashSet<[u64; 5]> = HashSet::new();
    for a in 1..m {
        for b in a + 1..m {
            st.pairs += 1;
            let Some(res) = fs.residual(a, b, &ctr, &mut st) else {
                continue;
            };
            let nf = res.iter().filter(|x| x.f.is_some()).count();
            st.n_f[nf] += 1;
            if nf == 0 {
                continue;
            }
            let mut ids: Vec<u64> = vec![pack(&fs.pts[a]), pack(&fs.pts[b])];
            ids.extend(res.iter().map(|x| pack(&x.q)));
            ids.sort_unstable();
            let key: [u64; 5] = [ids[0], ids[1], ids[2], ids[3], ids[4]];
            if !seen.insert(key) {
                continue;
            }
            let mut fcols: Vec<u32> = vec![(a - 1) as u32, (b - 1) as u32];
            let mut lp: Vec<u64> = vec![];
            for x in &res {
                match x.f {
                    Some(i) => {
                        if i != 0 {
                            fcols.push((i - 1) as u32)
                        }
                    }
                    None => lp.push(pack(&x.q)),
                }
            }
            raws.push(Raw {
                f: fcols,
                lp,
                special: 0,
                kind: 0,
            });
        }
    }
    if do_b {
        let mut fseen: HashSet<(usize, u32)> = HashSet::new();
        for i in 0..m {
            for r in 0..2usize {
                let key = key_of(fs.p, &fs.pts[i], r);
                if !fseen.insert((r, key)) {
                    continue;
                }
                st.fibres_probed += 1;
                let cub = fibre_cubic(&fs.g, fs.p, r, key, &ctr);
                let Some(rs) = distinct_roots(&cub, 3, fs.p, &ctr) else {
                    continue;
                };
                if !rs.complete {
                    continue;
                }
                st.fibres_split += 1;
                let mut fcols = vec![];
                let mut lp = vec![];
                for ro in &rs.roots {
                    let q = match (r, *ro) {
                        (0, Root::Fin(w)) => [fs.pts[i][0], fs.pts[i][1], w as u32, 1],
                        (0, Root::Inf) => [fs.pts[i][0], fs.pts[i][1], 1, 0],
                        (_, Root::Fin(w)) => [1, w as u32, fs.pts[i][2], fs.pts[i][3]],
                        (_, Root::Inf) => [0, 1, fs.pts[i][2], fs.pts[i][3]],
                    };
                    match fs.map.get(&pack(&q)) {
                        Some(&j) => {
                            if j != 0 {
                                fcols.push((j - 1) as u32)
                            }
                        }
                        None => lp.push(pack(&q)),
                    }
                }
                if lp.len() <= 2 {
                    raws.push(Raw {
                        f: fcols,
                        lp,
                        special: 1 + r as u32,
                        kind: 1,
                    });
                }
            }
        }
    }
    let _ = c;
    st.native = ctr.get();
    Collected { raws, st }
}

/// Sample the factor base by root finding of random fibres: uniform over
/// rational points (accept a fibre with k points with probability k/3).
fn sample_fb(c: &Curve, m: usize, rng: &mut u64, ctr: &Cell<u64>) -> Fset {
    let p = c.pp();
    let mut d0: Vec<[u32; 4]> = c.d0.iter().map(|&i| c.pts[i]).collect();
    // Pa first (already d0[0])
    let mut fs = Fset::new(c, std::mem::take(&mut d0));
    let target = m.min(c.pts.len() * 9 / 10).max(4);
    while fs.len() < target {
        let key = rnd(rng, p + 1) as u32;
        let cub = fibre_cubic(&c.g, p, 0, key, ctr);
        let Some(rs) = distinct_roots(&cub, 3, p, ctr) else {
            continue;
        };
        let k = rs.roots.len();
        if k == 0 || rnd(rng, 3) as usize >= k {
            continue;
        }
        let ro = rs.roots[rnd(rng, k as u64) as usize];
        let (s, t) = if key as u64 == p {
            (0u32, 1u32)
        } else {
            (1, key)
        };
        let q = match ro {
            Root::Fin(w) => [s, t, w as u32, 1],
            Root::Inf => [s, t, 1, 0],
        };
        fs.push(c, q);
        ctr.set(ctr.get() + 12); // evaluation of the four S_1 monomials
    }
    fs
}

#[derive(Default, Clone, Debug)]
struct Rung {
    alpha: f64,
    rung: usize,
    m: usize,
    ok_la: bool,
    reason: String,
    pairs: u64,
    zero_plane: u64,
    degenerate: u64,
    split: u64,
    n_f: [u64; 4],
    anomalies: u64,
    raws: usize,
    raws_a: usize,
    raws_b: usize,
    partial: usize,
    lp_vertices: usize,
    filter_core: usize,
    filter_peeled: usize,
    filter_complete: usize,
    filter_cycles: usize,
    filter_merges: usize,
    rows: usize,
    nused: usize,
    nnz: usize,
    kernel_dim: i64,
    la_ops: u64,
    la_matvecs: u64,
    u_size: usize,
    // costs
    native_p_rel: u64,
    native_p_fb: u64,
    native_ell_filter: u64,
    native_ell_la: u64,
    ops_shared: u64,
    nat_shared_indiv: u64,
    steps_shared: u64,
    tgt_ok: Vec<bool>,
    tgt_ops: Vec<u64>,
    tgt_nat: Vec<u64>,
    tgt_steps: Vec<u64>,
    secs: f64,
}

struct Walk2 {
    adders: Vec<(Elt, u64)>,
}

#[allow(clippy::too_many_arguments)]
fn find_record(
    c: &Curve,
    ub_pts: &[[u32; 9]],
    lam: &[u64],
    d0sum: u64,
    ell: u64,
    start: Elt,
    a0: u64,
    wk: &Walk2,
    max_steps: u64,
    native: &mut u64,
    rng: &mut u64,
) -> (Option<(u64, u64)>, u64) {
    let p = c.pp();
    let mut u = start;
    let mut a = a0 % ell;
    let mut steps = 0u64;
    while steps < max_steps {
        // support of u among the points with known logs
        let w = &u.w;
        let mut supp: [usize; 5] = [0; 5];
        let mut ns = 0usize;
        for (i, ev) in ub_pts.iter().enumerate() {
            let mut s = 0u64;
            for (x, y) in w[0].iter().zip(ev.iter()) {
                s += *x as u64 * *y as u64;
            }
            *native += 9;
            if !s.is_multiple_of(p) {
                continue;
            }
            let mut ok = true;
            for r in w.iter().skip(1) {
                let mut s = 0u64;
                for (x, y) in r.iter().zip(ev.iter()) {
                    s += *x as u64 * *y as u64;
                }
                *native += 9;
                if !s.is_multiple_of(p) {
                    ok = false;
                    break;
                }
            }
            if ok {
                if ns < 5 {
                    supp[ns] = i;
                }
                ns += 1;
            }
        }
        if ns == 4 {
            let s = supp[..4].iter().fold(0u64, |t, &i| (t + lam[i]) % ell);
            let s = (s + ell - d0sum) % ell;
            return (Some((s, a)), steps);
        }
        let j = rnd(rng, wk.adders.len() as u64) as usize;
        u = c.add(&u, &wk.adders[j].0);
        a = (a + wk.adders[j].1) % ell;
        steps += 1;
    }
    (None, steps)
}

#[allow(clippy::too_many_arguments)]
fn attempt(
    c: &Curve,
    ell: u64,
    g: &Elt,
    hs: &[Elt],
    alpha: f64,
    rung: usize,
    c0: f64,
    rng: &mut u64,
    max_indiv_steps: u64,
) -> Rung {
    let t0 = Instant::now();
    let p = c.pp();
    let n = c.pts.len();
    let mf = c0 * RUNG_MULT.powi(rung as i32) * (p as f64).powf(alpha);
    let m = (mf.ceil() as usize).max(8);
    let mut out = Rung {
        alpha,
        rung,
        m,
        kernel_dim: -1,
        tgt_ok: vec![false; hs.len()],
        tgt_ops: vec![0; hs.len()],
        tgt_nat: vec![0; hs.len()],
        tgt_steps: vec![0; hs.len()],
        ..Default::default()
    };
    // factor base by root finding
    let fbctr = Cell::new(0u64);
    let fs = sample_fb(c, m, rng, &fbctr);
    out.m = fs.len();
    out.native_p_fb = fbctr.get();
    let _ = n;
    // relations
    let col = collect_rf(c, &fs, true);
    out.native_p_rel = col.st.native;
    out.pairs = col.st.pairs;
    out.zero_plane = col.st.zero_plane;
    out.degenerate = col.st.degenerate;
    out.split = col.st.split;
    out.n_f = col.st.n_f;
    out.anomalies = col.st.anomalies;
    out.raws = col.raws.len();
    out.raws_a = col.raws.iter().filter(|r| r.kind == 0).count();
    out.raws_b = col.raws.iter().filter(|r| r.kind == 1).count();
    out.partial = col.raws.iter().filter(|r| !r.lp.is_empty()).count();
    let nf = fs.len() - 1;
    let kappa = nf as u32;
    let sp_col = |s: u32| -> u32 { kappa + s };
    let mut lpset: HashSet<u64> = HashSet::new();
    for r in &col.raws {
        for &x in &r.lp {
            lpset.insert(x);
        }
    }
    out.lp_vertices = lpset.len();
    // filter
    let mut parts: Vec<PartialRelation> = vec![];
    for (id, r) in col.raws.iter().enumerate() {
        let mut small: Vec<(u32, u64)> = r.f.iter().map(|&x| (x, 1u64)).collect();
        small.push((sp_col(r.special), ell - 1));
        let large: Vec<(u64, u64)> = r.lp.iter().map(|&x| (x, 1u64)).collect();
        parts.push(PartialRelation {
            id: id as u64,
            small,
            large,
            rhs: 0,
        });
    }
    jv_bitops::enable(true);
    jv_bitops::claim();
    let before = jv_bitops::snapshot();
    let fr = filter_large_primes(
        parts,
        &LargePrimeFilterOptions {
            modulus: ell,
            ..Default::default()
        },
    );
    let after = jv_bitops::snapshot();
    jv_bitops::enable(false);
    let (complete, rep) = match fr {
        Ok(x) => x,
        Err(e) => {
            out.reason = format!("filter error {e:?}");
            out.secs = t0.elapsed().as_secs_f64();
            return out;
        }
    };
    out.native_ell_filter = after.since(&before).total().counts.iter().sum::<u64>();
    out.filter_core = rep.core_relations;
    out.filter_peeled = rep.peeled_relations;
    out.filter_complete = rep.complete_relations;
    out.filter_cycles = rep.cycles_closed;
    out.filter_merges = rep.merges;
    let mut rows: Vec<Row> = vec![];
    for cr in &complete {
        assert_eq!(cr.rhs, 0, "homogeneous relations");
        rows.push(cr.small.iter().map(|&(cc, v)| (cc, v as i64)).collect());
    }
    // K = T1 + T2
    rows.push(vec![(sp_col(0), 1), (sp_col(1), -1), (sp_col(2), -1)]);
    let ncols = nf + 3;
    let (rows, back) = compact(&rows, ncols);
    let nused = back.len();
    out.rows = rows.len();
    out.nused = nused;
    out.nnz = rows.iter().map(|r| r.len()).sum();
    if rows.len() + 1 < nused {
        out.reason = format!(
            "too few relations: {} rows, {} used columns",
            rows.len(),
            nused
        );
        out.secs = t0.elapsed().as_secs_f64();
        return out;
    }
    let dense = if nused <= 600 {
        Some(dense_kernel(&rows, nused, ell))
    } else {
        None
    };
    if let Some((kd, _)) = &dense {
        out.kernel_dim = *kd as i64;
        if *kd != 1 {
            out.reason = format!("kernel dimension {kd}");
            out.secs = t0.elapsed().as_secs_f64();
            return out;
        }
    }
    let Some(la) = wiedemann_kernel(&rows, nused, ell, rng) else {
        out.reason = "wiedemann failed".into();
        out.secs = t0.elapsed().as_secs_f64();
        return out;
    };
    out.la_ops = la.ops;
    out.la_matvecs = la.matvecs;
    out.native_ell_la = la.ops;
    if let Some((_, dv)) = &dense {
        let i0 = (0..nused).find(|&i| dv[i] != 0).unwrap();
        let sc = mulmod(la.vec[i0], invmod(dv[i0], ell).unwrap(), ell);
        for i in 0..nused {
            if mulmod(sc, dv[i], ell) != la.vec[i] {
                out.reason = "wiedemann/dense kernel mismatch".into();
                out.secs = t0.elapsed().as_secs_f64();
                return out;
            }
        }
    }
    out.ok_la = true;
    // logs of F points and kappa
    let mut lam_col = vec![None::<u64>; ncols];
    for (k, &oc) in back.iter().enumerate() {
        lam_col[oc] = Some(la.vec[k]);
    }
    let mut known: HashMap<u64, u64> = HashMap::new(); // pack -> log (scaled)
    known.insert(pack(&fs.pts[0]), 0);
    for i in 1..fs.len() {
        if let Some(v) = lam_col[i - 1] {
            known.insert(pack(&fs.pts[i]), v);
        }
    }
    if c.d0.iter().any(|&d| !known.contains_key(&pack(&c.pts[d]))) {
        out.reason = "D0 point not covered by relations".into();
        out.secs = t0.elapsed().as_secs_f64();
        return out;
    }
    // one-step logs of large primes: lam(LP) = special - sum(F) from partial rows with a single unknown
    let lamof = |r: &Raw, known: &HashMap<u64, u64>, fs: &Fset| -> Option<u64> {
        let mut s = 0u64;
        for &x in &r.f {
            s = (s + known.get(&pack(&fs.pts[x as usize + 1]))?) % ell;
        }
        let sv = lam_col[sp_col(r.special) as usize]?;
        Some((sv + ell - s) % ell)
    };
    loop {
        let mut grew = false;
        for r in &col.raws {
            let unk: Vec<u64> =
                r.lp.iter()
                    .copied()
                    .filter(|x| !known.contains_key(x))
                    .collect();
            if unk.len() != 1 {
                continue;
            }
            // other LPs known
            let mut s_other = 0u64;
            for x in r.lp.iter().filter(|x| **x != unk[0]) {
                s_other = (s_other + known[x]) % ell;
            }
            if let Some(v) = lamof(r, &known, &fs) {
                known.insert(unk[0], (v + ell - s_other) % ell);
                grew = true;
            }
        }
        if !grew {
            break;
        }
    }
    out.u_size = known.len();
    // decomposition base
    let mut ub_pts: Vec<[u32; 9]> = vec![];
    let mut lam: Vec<u64> = vec![];
    let mut ev2_native = 0u64;
    for (&k, &v) in known.iter() {
        let q = [
            (k >> 48) as u32,
            ((k >> 32) & 0xffff) as u32,
            ((k >> 16) & 0xffff) as u32,
            (k & 0xffff) as u32,
        ];
        let e = c.eval_vec(2, &q);
        let mut a9 = [0u32; 9];
        a9.copy_from_slice(&e[..9]);
        ub_pts.push(a9);
        lam.push(v);
        ev2_native += 27;
    }
    let d0sum: u64 =
        c.d0.iter()
            .fold(0u64, |s, &d| (s + known[&pack(&c.pts[d])]) % ell);
    // group-operation accounting
    let ops0 = c.n_ops.get();
    let nat0 = c.cnt.get() - c.inner.get();
    let mut nat_shared = ev2_native;
    // adders: multiples of g
    let mut adders = vec![];
    for _ in 0..16 {
        let a = 1 + rnd(rng, 1 << 14);
        adders.push((c.mul_scalar(g, a as u128), a % ell));
    }
    let wk = Walk2 { adders };
    let a0 = 1 + rnd(rng, 1 << 12);
    let start = c.mul_scalar(g, a0 as u128);
    let (rec_g, steps_g) = find_record(
        c,
        &ub_pts,
        &lam,
        d0sum,
        ell,
        start,
        a0,
        &wk,
        max_indiv_steps,
        &mut nat_shared,
        rng,
    );
    out.steps_shared = steps_g;
    out.ops_shared = c.n_ops.get() - ops0;
    out.nat_shared_indiv = nat_shared + (c.cnt.get() - c.inner.get() - nat0);
    let Some((sg, ag)) = rec_g else {
        out.reason = "no g record".into();
        out.secs = t0.elapsed().as_secs_f64();
        return out;
    };
    let Some(inv_ag) = invmod(ag, ell) else {
        out.reason = "a_g not invertible".into();
        out.secs = t0.elapsed().as_secs_f64();
        return out;
    };
    let sigma = mulmod(sg, inv_ag, ell);
    let Some(inv_sigma) = invmod(sigma, ell) else {
        out.reason = "sigma zero".into();
        out.secs = t0.elapsed().as_secs_f64();
        return out;
    };
    for (ti, h) in hs.iter().enumerate() {
        let ops_t0 = c.n_ops.get();
        let nat_t0 = c.cnt.get() - c.inner.get();
        let mut nat_t = 0u64;
        let a1 = 1 + rnd(rng, 1 << 12);
        let b1 = 1 + rnd(rng, 1 << 12);
        let start = c.add(&c.mul_scalar(g, a1 as u128), &c.mul_scalar(h, b1 as u128));
        let (rec, steps) = find_record(
            c,
            &ub_pts,
            &lam,
            d0sum,
            ell,
            start,
            a1,
            &wk,
            max_indiv_steps,
            &mut nat_t,
            rng,
        );
        out.tgt_steps[ti] = steps;
        out.tgt_ops[ti] = c.n_ops.get() - ops_t0;
        out.tgt_nat[ti] = nat_t + (c.cnt.get() - c.inner.get() - nat_t0);
        if let Some((s, a)) = rec {
            // s = sigma (a + b1 x)
            let v = (mulmod(s, inv_sigma, ell) + ell - a) % ell;
            if let Some(ib) = invmod(b1 % ell, ell) {
                let x = mulmod(v, ib, ell);
                if c.mul_scalar(g, x as u128).key == h.key {
                    out.tgt_ok[ti] = true;
                }
            }
        }
    }
    out.reason = if out.tgt_ok.iter().all(|&b| b) {
        "ok".into()
    } else {
        "target not solved".into()
    };
    out.secs = t0.elapsed().as_secs_f64();
    out
}

fn rung_json(r: &Rung) -> Value {
    json!({
        "alpha": r.alpha, "rung": r.rung, "m": r.m, "ok_la": r.ok_la, "reason": r.reason,
        "pairs": r.pairs, "zero_plane": r.zero_plane, "degenerate": r.degenerate, "split": r.split,
        "n_f": r.n_f, "anomalies": r.anomalies, "raws": r.raws, "raws_a": r.raws_a, "raws_b": r.raws_b,
        "partial": r.partial, "lp_vertices": r.lp_vertices,
        "filter_core": r.filter_core, "filter_peeled": r.filter_peeled, "filter_complete": r.filter_complete,
        "filter_cycles": r.filter_cycles, "filter_merges": r.filter_merges,
        "rows": r.rows, "nused": r.nused, "nnz": r.nnz, "kernel_dim": r.kernel_dim,
        "la_ops": r.la_ops, "la_matvecs": r.la_matvecs, "u_size": r.u_size,
        "native_p_rel": r.native_p_rel, "native_p_fb": r.native_p_fb,
        "native_ell_filter": r.native_ell_filter, "native_ell_la": r.native_ell_la,
        "ops_shared": r.ops_shared, "nat_shared_indiv": r.nat_shared_indiv, "steps_shared": r.steps_shared,
        "tgt_ok": r.tgt_ok, "tgt_ops": r.tgt_ops, "tgt_nat": r.tgt_nat, "tgt_steps": r.tgt_steps,
        "secs": r.secs
    })
}

fn mulmod_ns(p: u64) -> f64 {
    let mut best = f64::MAX;
    for _ in 0..5 {
        let t = Instant::now();
        let mut acc = 1u64;
        let mut x = 12345u64;
        let n = 20_000_000u64;
        for i in 0..n {
            x = (x * 7 + i) % p;
            acc = (acc + x * (acc | 3)) % p;
        }
        std::hint::black_box(acc);
        best = best.min(t.elapsed().as_nanos() as f64 / (2 * n) as f64);
    }
    best
}

fn factor_json(f: &[(u64, u32)]) -> Value {
    Value::Array(f.iter().map(|(q, e)| json!([q, e])).collect())
}

fn cmd_dlp(args: &[String]) {
    let p: u32 = args[0].parse().unwrap();
    let curve_idx: u64 = args[1].parse().unwrap();
    let base_seed: u64 = args[2].parse().unwrap();
    let rho_targets: usize = args[3].parse().unwrap();
    let outf = &args[4];
    let git = &args[5];
    let t_all = Instant::now();
    let mut chosen = None;
    for trial in 0..80u64 {
        let mut rng = base_seed
            .wrapping_mul(2_654_435_761)
            .wrapping_add(p as u64 * 1_000_003)
            .wrapping_add(curve_idx * 7_368_787)
            .wrapping_add(trial * 104_729 + 3)
            | 1;
        let cseed = rng;
        let c = random_curve(p, &mut rng);
        if c.pts.len() < (p as usize) / 2 {
            continue;
        }
        let Ok(nn) = order_bsgs(&c, &mut rng) else {
            continue;
        };
        let nn = nn as u64;
        let fac = factor(nn);
        let (ell, e) = *fac.iter().max_by_key(|x| x.0).unwrap();
        if e != 1 || ell.saturating_mul(16) < nn {
            eprintln!("trial {trial}: N={nn} factors={fac:?} rejected");
            continue;
        }
        chosen = Some((c, rng, cseed, trial, nn, ell));
        break;
    }
    let Some((c, mut rng, cseed, trial, nn, ell)) = chosen else {
        std::fs::write(outf, "{\"error\":\"no suitable curve\"}\n").unwrap();
        return;
    };
    let fac = factor(nn);
    let g = loop {
        let x0 = random_elt(&c, &mut rng);
        let g = c.mul_scalar(&x0, (nn / ell) as u128);
        if !c.is_zero(&g) {
            break g;
        }
    };
    assert!(c.is_zero(&c.mul_scalar(&g, ell as u128)));
    let ntarg = 3usize;
    let xs: Vec<u64> = (0..ntarg).map(|_| 1 + rnd(&mut rng, ell - 1)).collect();
    let hs: Vec<Elt> = xs.iter().map(|&x| c.mul_scalar(&g, x as u128)).collect();
    // calibration
    let t0 = Instant::now();
    let nadd = 60u64;
    let mut acc = g.clone();
    let ops0 = c.n_ops.get();
    let inner0 = c.inner.get();
    for _ in 0..nadd {
        acc = c.add(&acc, &hs[0]);
    }
    let t_add = t0.elapsed().as_nanos() as f64 / nadd as f64;
    let mulmods_per_add = (c.inner.get() - inner0) as f64 / (c.n_ops.get() - ops0) as f64;
    let t_mul_p = mulmod_ns(p as u64);
    let w_cal_p = t_mul_p / t_add;
    // fibre law (diagnostic, over all fibres of both pencils)
    let (h1, h2, fibres) = fibre_stats(&c);
    // index calculus arms
    let arms: [(f64, f64); 3] = [(0.5, 1.5), (2.0 / 3.0, 1.0), (0.75, 1.0)];
    let mut arms_json = vec![];
    for &(alpha, c0) in &arms {
        let mut passes = vec![];
        for _pass in 0..2 {
            let mut rungs = vec![];
            let mut solved_at: Option<usize> = None;
            for r in 0..RUNGS {
                let o = attempt(&c, ell, &g, &hs, alpha, r, c0, &mut rng, 60_000);
                eprintln!(
                    "p={p} curve={curve_idx} alpha={alpha:.3} rung={r} m={} rows={} nused={} la={} u={} ok={:?} {} ({:.1}s)",
                    o.m, o.rows, o.nused, o.ok_la, o.u_size, o.tgt_ok, o.reason, o.secs
                );
                let done = o.tgt_ok.iter().all(|&b| b);
                rungs.push(rung_json(&o));
                if done {
                    solved_at = Some(r);
                    break;
                }
            }
            passes.push(json!({"solved_at": solved_at, "rungs": rungs}));
        }
        arms_json.push(json!({"alpha": alpha, "c0": c0, "passes": passes}));
    }
    // rho
    let dbits = (((ell as f64).sqrt().log2() - 4.0).floor().max(2.0)) as u32;
    let mut rho_json = vec![];
    for t in 0..rho_targets.min(ntarg) {
        let tr = Instant::now();
        let ops_before = c.n_ops.get();
        let mut rr = base_seed
            .wrapping_mul(97)
            .wrapping_add(curve_idx * 31 + t as u64 * 7 + p as u64)
            | 1;
        let walk = make_adders(&c, &g, &hs[t], 16, ell, &mut rr);
        let setup = c.n_ops.get() - ops_before;
        let o = rho(
            &c,
            &g,
            &hs[t],
            ell,
            dbits,
            &walk,
            &mut rr,
            60 * ((ell as f64).sqrt() as u64 + 100),
        );
        let verified =
            o.x.is_some_and(|x| c.mul_scalar(&g, x as u128).key == hs[t].key);
        eprintln!(
            "rho p={p} curve={curve_idx} target={t}: adds={} steps={} verified={verified} ({:.0}s)",
            o.adds,
            o.steps,
            tr.elapsed().as_secs_f64()
        );
        rho_json.push(json!({
            "target": t, "verified": verified, "setup_adds": setup, "walk_adds": o.adds,
            "steps": o.steps, "dps": o.dps, "native": o.native, "restarts": o.restarts,
            "secs": tr.elapsed().as_secs_f64()
        }));
    }
    let res = json!({
        "p": p, "curve": curve_idx, "base_seed": base_seed, "curve_seed": cseed, "trial": trial, "git": git,
        "g": c.g.to_vec(), "npts": c.pts.len(), "N": nn, "factors": factor_json(&fac), "ell": ell,
        "x_true": xs, "fibres": fibres, "fib_hist_pencil1": h1.to_vec(), "fib_hist_pencil2": h2.to_vec(),
        "calib": {"t_add_ns": t_add, "mulmods_per_add": mulmods_per_add, "t_mulmod_p_ns": t_mul_p, "w_cal_p": w_cal_p},
        "dbits": dbits, "arms": arms_json, "rho": rho_json,
        "fallbacks": c.n_fallback.get(), "exceptional_keys": c.n_exc.get(), "keyfail": c.n_keyfail.get(),
        "total_secs": t_all.elapsed().as_secs_f64()
    });
    std::fs::write(outf, serde_json::to_string(&res).unwrap() + "\n").unwrap();
}

/// A-line yield study at fixed phi, with the old scan as a cross-check at p <= 509.
fn cmd_yield(args: &[String]) {
    let p: u32 = args[0].parse().unwrap();
    let k: usize = args[1].parse().unwrap();
    let seed: u64 = args[2].parse().unwrap();
    let outf = &args[3];
    let git = &args[4];
    let phis = [0.3f64, 0.5, 0.7, 1.0];
    let mut lines = String::new();
    for ci in 0..k {
        let mut rng = seed
            .wrapping_mul(1_000_003)
            .wrapping_add(p as u64 * 7919 + ci as u64 * 104_729 + 1)
            | 1;
        let cseed = rng;
        let c = random_curve(p, &mut rng);
        let n = c.pts.len();
        let (h1, h2, fibres) = fibre_stats(&c);
        let ev1 = ev1_table(&c);
        let mut cells = vec![];
        for &phi in &phis {
            let m = ((phi * n as f64).ceil() as usize).clamp(8, n);
            let mut in_f = vec![false; n];
            for &d in &c.d0 {
                in_f[d] = true;
            }
            let mut cnt = c.d0.len();
            while cnt < m {
                let i = rnd(&mut rng, n as u64) as usize;
                if !in_f[i] {
                    in_f[i] = true;
                    cnt += 1;
                }
            }
            let mut pts: Vec<[u32; 4]> = c.d0.iter().map(|&i| c.pts[i]).collect();
            for i in 0..n {
                if in_f[i] && !c.d0.contains(&i) {
                    pts.push(c.pts[i]);
                }
            }
            let fs = Fset::new(&c, pts);
            let t0 = Instant::now();
            let col = collect_rf(&c, &fs, false);
            let rf_secs = t0.elapsed().as_secs_f64();
            // scan cross-check
            let mut scan = Value::Null;
            if p <= 509 {
                let fidx: Vec<usize> = (0..n).filter(|&i| in_f[i]).collect();
                let lay = make_layout(&c, &fidx);
                let mut r2 = rng ^ 0x9e3779b97f4a7c15;
                let rs = collect(&c, &lay, &ev1, true, 0, &mut r2);
                scan = json!({"pairs": rs.st.pairs, "hits_a": rs.st.hits_a, "unique_a": rs.st.unique_a, "native": rs.st.native});
            }
            let uniq_complete = col
                .raws
                .iter()
                .filter(|r| r.kind == 0 && r.lp.is_empty())
                .count();
            cells.push(json!({
                "phi_target": phi, "m": fs.len(), "phi": fs.len() as f64 / n as f64,
                "pairs": col.st.pairs, "split": col.st.split, "n_f": col.st.n_f, "zero_plane": col.st.zero_plane,
                "degenerate": col.st.degenerate, "anomalies": col.st.anomalies,
                "unique_complete": uniq_complete, "native": col.st.native, "secs": rf_secs, "scan": scan
            }));
        }
        let line = json!({
            "p": p, "curve": ci, "seed": cseed, "git": git, "npts": n, "fibres": fibres,
            "fib_hist_pencil1": h1.to_vec(), "fib_hist_pencil2": h2.to_vec(), "cells": cells
        });
        lines.push_str(&(serde_json::to_string(&line).unwrap() + "\n"));
    }
    std::fs::write(outf, lines).unwrap();
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(|s| s.as_str()) {
        Some("dlp") => cmd_dlp(&args[2..]),
        Some("yield") => cmd_yield(&args[2..]),
        _ => eprintln!("usage: genus4_pencil_ic_lp dlp P CURVE BASESEED RHO_TARGETS OUT GIT | yield P K SEED OUT GIT"),
    }
}

#[cfg(test)]
mod tests {
    use super::g4::*;
    use super::*;

    fn fset_all(c: &Curve) -> Fset {
        let mut pts: Vec<[u32; 4]> = c.d0.iter().map(|&i| c.pts[i]).collect();
        for i in 0..c.pts.len() {
            if !c.d0.contains(&i) {
                pts.push(c.pts[i]);
            }
        }
        Fset::new(c, pts)
    }

    #[test]
    fn ev1_order_is_tv_tu_sv_su() {
        let mut rng = 3u64;
        let c = random_curve(31, &mut rng);
        for q in c.pts.iter().take(10) {
            let e = c.eval_vec(1, q);
            let (s, t, u, v) = (q[0] as u64, q[1] as u64, q[2] as u64, q[3] as u64);
            assert_eq!(
                [e[0] as u64, e[1] as u64, e[2] as u64, e[3] as u64],
                [t * v % 31, t * u % 31, s * v % 31, s * u % 31]
            );
        }
    }

    /// Root finder against the scan: for random F subsets and random pairs, the
    /// residual points found by root finding that lie in F equal the scan result,
    /// and the complete (3-in-F) hits coincide exactly.
    #[test]
    fn residual_matches_scan() {
        for (p, seed) in [(13u32, 1u64), (31, 2), (61, 3), (127, 4)] {
            let mut rng = seed * 1001 + 7;
            let c = random_curve(p, &mut rng);
            let n = c.pts.len();
            for frac in [0.4f64, 0.7, 1.0] {
                let m = ((frac * n as f64) as usize).max(8).min(n);
                let mut in_f = vec![false; n];
                for &d in &c.d0 {
                    in_f[d] = true;
                }
                let mut cnt = 4;
                while cnt < m {
                    let i = rnd(&mut rng, n as u64) as usize;
                    if !in_f[i] {
                        in_f[i] = true;
                        cnt += 1;
                    }
                }
                let mut pts: Vec<[u32; 4]> = c.d0.iter().map(|&i| c.pts[i]).collect();
                for i in 0..n {
                    if in_f[i] && !c.d0.contains(&i) {
                        pts.push(c.pts[i]);
                    }
                }
                let fs = Fset::new(&c, pts);
                let ctr = Cell::new(0);
                let mut st = CStats::default();
                let mut hits_rf = 0u64;
                let mut hits_scan = 0u64;
                let mut lp_checked = 0u64;
                for a in 1..fs.len() {
                    for b in a + 1..fs.len() {
                        // scan reference over the whole point set
                        let l = plane3(&fs.ev1[0], &fs.ev1[a], &fs.ev1[b], p as u64, &ctr);
                        let res = fs.residual(a, b, &ctr, &mut st);
                        if l == [0; 4] {
                            assert!(res.is_none());
                            continue;
                        }
                        let on_plane: Vec<usize> = (0..n)
                            .filter(|&q| {
                                let e = c.eval_vec(1, &c.pts[q]);
                                (0..4).map(|k| l[k] * e[k] as u64).sum::<u64>() % p as u64 == 0
                            })
                            .filter(|&q| {
                                let pk = pack(&c.pts[q]);
                                pk != pack(&fs.pts[0])
                                    && pk != pack(&fs.pts[a])
                                    && pk != pack(&fs.pts[b])
                            })
                            .collect();
                        // the scan over all rational points: exactly three distinct others
                        match res {
                            Some(r) => {
                                assert_eq!(on_plane.len(), 3, "p={p} a={a} b={b}");
                                let mut got: Vec<u64> = r.iter().map(|x| pack(&x.q)).collect();
                                let mut want: Vec<u64> =
                                    on_plane.iter().map(|&q| pack(&c.pts[q])).collect();
                                got.sort_unstable();
                                want.sort_unstable();
                                assert_eq!(got, want);
                                for x in &r {
                                    assert_eq!(x.f.is_some(), fs.map.contains_key(&pack(&x.q)));
                                }
                                lp_checked += r.iter().filter(|x| x.f.is_none()).count() as u64;
                                if r.iter().all(|x| x.f.is_some()) {
                                    hits_rf += 1;
                                }
                            }
                            None => {
                                // no complete split: the plane must not have three distinct other points
                                // (tangent planes may have fewer than three)
                                let det =
                                    (l[3] * l[0] + p as u64 * p as u64 - l[2] * l[1]) % p as u64;
                                // a degenerate section is scanned over F only (documented limitation)
                                assert!(
                                    det == 0 || on_plane.len() != 3,
                                    "missed hit p={p} a={a} b={b} {on_plane:?}"
                                );
                            }
                        }
                        let in_f_others = on_plane
                            .iter()
                            .filter(|&&q| fs.map.contains_key(&pack(&c.pts[q])))
                            .count();
                        if on_plane.len() == 3 && in_f_others == 3 {
                            hits_scan += 1;
                        }
                    }
                }
                assert_eq!(hits_rf, hits_scan, "p={p} frac={frac}");
                eprintln!(
                    "p={p} frac={frac} hits={hits_rf} lp_points_checked={lp_checked} anomalies={}",
                    st.anomalies
                );
                assert_eq!(st.anomalies, 0);
            }
        }
    }

    /// Collected relations (complete and with large primes) are true in Pic.
    #[test]
    fn lp_relations_are_true_p31() {
        let mut rng = 2025u64;
        let c = random_curve(31, &mut rng);
        let n = c.pts.len();
        let cls: Vec<Elt> = (0..n).map(|i| c.point_class(i)).collect();
        let idx_of: HashMap<u64, usize> = c
            .pts
            .iter()
            .enumerate()
            .map(|(i, q)| (pack(q), i))
            .collect();
        let ctr = Cell::new(0u64);
        let fs = sample_fb(&c, 14, &mut rng, &ctr);
        let col = collect_rf(&c, &fs, true);
        assert!(
            col.raws.iter().any(|r| !r.lp.is_empty()),
            "need a partial relation"
        );
        let class_of_row = |r: &Raw| -> Elt {
            let mut acc = c.zero().clone();
            for &x in &r.f {
                acc = c.add(&acc, &cls[idx_of[&pack(&fs.pts[x as usize + 1])]]);
            }
            for &x in &r.lp {
                acc = c.add(&acc, &cls[idx_of[&x]]);
            }
            acc
        };
        let mut by_special: HashMap<u32, Vec<Elt>> = HashMap::new();
        for r in &col.raws {
            by_special
                .entry(r.special)
                .or_default()
                .push(class_of_row(r));
        }
        let mut checked = 0;
        for v in by_special.values() {
            for w in v.windows(2) {
                assert_eq!(w[0].key, w[1].key);
                checked += 1;
            }
        }
        assert!(checked >= 2, "checked {checked}");
        if let (Some(k), Some(a), Some(b)) =
            (by_special.get(&0), by_special.get(&1), by_special.get(&2))
        {
            assert_eq!(k[0].key, c.add(&a[0], &b[0]).key, "K = T1 + T2");
        }
    }

    #[test]
    fn sampled_fb_points_are_on_curve_and_distinct() {
        let mut rng = 77u64;
        let c = random_curve(61, &mut rng);
        let ctr = Cell::new(0u64);
        let fs = sample_fb(&c, 30, &mut rng, &ctr);
        assert_eq!(fs.len(), 30);
        let all: HashSet<u64> = c.pts.iter().map(pack).collect();
        for q in &fs.pts {
            assert!(all.contains(&pack(q)));
            assert_eq!(c.eval_g(q), 0);
        }
        let _ = fset_all(&c);
    }
}
