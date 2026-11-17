//! Pilot: the "2+1 linear leaf" inside the field-product-gate CDCL (`LinCdcl`).
//!
//! Standalone: `rustc -O lin21.rs` (standard library only). The solver is included UNCHANGED
//! from `examples/pdp_lin_cdcl.rs`.
//!
//! Family: E: y^2 + xy = x^3 + a x^2 + b over F_{2^n} (a = b = 1), polynomial basis,
//! V = <1, z, .., z^(l-1)> (elements < 2^l). Problem (ALG0): given x_R outside V, are there
//! x1, x2, x3 in V and t in F with S3(x1, t, x_R) = 0 and S3(t, x2, x3) = 0, where
//! S3(u, v, w) = (uv + uw + vw)^2 + uvw + b.
//!
//! Subcommands: `selftest`, `targets`, `run` (one arm on one target, one JSON line).

#[path = "../../examples/pdp_lin_cdcl.rs"]
mod pdp_lin_cdcl;
use pdp_lin_cdcl::{ChainInstance, LinCdcl, Result as LinResult};
use std::cell::Cell;
use std::collections::HashSet;
use std::time::Instant;

// ------------------------------------------------------------------ field

struct Gf {
    n: u32,
    irr: u64,
    /// echelon basis of the image of v -> v^2 + v, with preimages, indexed by leading bit
    as_basis: Vec<(u64, u64)>,
    muls: Cell<u64>,
    invs: Cell<u64>,
    as_solves: Cell<u64>,
}

fn deg(a: u64) -> i32 {
    63 - a.leading_zeros() as i32
}

fn raw_mul(n: u32, irr: u64, a: u64, b: u64) -> u64 {
    let mut r = 0u64;
    let mut a = a;
    let mut b = b;
    while b != 0 {
        if b & 1 == 1 {
            r ^= a;
        }
        b >>= 1;
        a <<= 1;
        if (a >> n) & 1 == 1 {
            a ^= irr;
        }
    }
    r
}

fn poly_gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        while a != 0 && deg(a) >= deg(b) {
            a ^= b << (deg(a) - deg(b));
        }
        std::mem::swap(&mut a, &mut b);
    }
    a
}

fn is_irreducible(n: u32, f: u64) -> bool {
    // Ben-Or: gcd(x^(2^i) + x, f) = 1 for 1 <= i <= n/2, and x^(2^n) = x.
    let mut p = 2u64; // x
    for i in 1..=n {
        p = raw_mul(n, f, p, p);
        if i <= n / 2 && poly_gcd(f, p ^ 2) != 1 {
            return false;
        }
    }
    p == 2
}

/// First irreducible in this order: trinomials x^n + x^k + 1 by increasing k; then pentanomials
/// x^n + x^k3 + x^k2 + x^k1 + 1 by increasing (k3, k2, k1).
fn find_irr(n: u32) -> u64 {
    for k in 1..n {
        let f = (1u64 << n) | (1u64 << k) | 1;
        if is_irreducible(n, f) {
            return f;
        }
    }
    for k3 in 3..n {
        for k2 in 2..k3 {
            for k1 in 1..k2 {
                let f = (1u64 << n) | (1u64 << k3) | (1u64 << k2) | (1u64 << k1) | 1;
                if is_irreducible(n, f) {
                    return f;
                }
            }
        }
    }
    panic!("no irreducible found");
}

impl Gf {
    fn new(n: u32, irr: u64) -> Gf {
        let mut g = Gf {
            n,
            irr,
            as_basis: vec![(0, 0); n as usize],
            muls: Cell::new(0),
            invs: Cell::new(0),
            as_solves: Cell::new(0),
        };
        for j in 0..n {
            let e = 1u64 << j;
            let mut img = raw_mul(n, irr, e, e) ^ e;
            let mut pre = e;
            while img != 0 {
                let d = deg(img) as usize;
                if g.as_basis[d].0 == 0 {
                    g.as_basis[d] = (img, pre);
                    break;
                }
                img ^= g.as_basis[d].0;
                pre ^= g.as_basis[d].1;
            }
        }
        g
    }
    fn mul(&self, a: u64, b: u64) -> u64 {
        self.muls.set(self.muls.get() + 1);
        raw_mul(self.n, self.irr, a, b)
    }
    fn sq(&self, a: u64) -> u64 {
        self.mul(a, a)
    }
    fn inv(&self, a: u64) -> u64 {
        assert!(a != 0);
        self.invs.set(self.invs.get() + 1);
        let (mut u, mut v) = (a, self.irr);
        let (mut g1, mut g2) = (1u64, 0u64);
        while u != 1 {
            let mut j = deg(u) - deg(v);
            if j < 0 {
                std::mem::swap(&mut u, &mut v);
                std::mem::swap(&mut g1, &mut g2);
                j = -j;
            }
            u ^= v << j;
            g1 ^= g2 << j;
        }
        g1
    }
    fn sqrt(&self, a: u64) -> u64 {
        let mut r = a;
        for _ in 0..self.n - 1 {
            r = self.sq(r);
        }
        r
    }
    /// One root v of v^2 + v = c (the other is v + 1), if any.
    fn as_solve(&self, c: u64) -> Option<u64> {
        self.as_solves.set(self.as_solves.get() + 1);
        let mut c = c;
        let mut v = 0u64;
        while c != 0 {
            let d = deg(c) as usize;
            let (img, pre) = self.as_basis[d];
            if img == 0 {
                return None;
            }
            c ^= img;
            v ^= pre;
        }
        Some(v)
    }
    /// Roots in F of a t^2 + b t + c (a != 0).
    fn quad_roots(&self, a: u64, b: u64, c: u64) -> ([u64; 2], usize) {
        if b == 0 {
            let r = self.sqrt(self.mul(c, self.inv(a)));
            return ([r, 0], 1);
        }
        let ib = self.inv(b);
        let k = self.mul(b, self.inv(a));
        let rhs = self.mul(self.mul(c, a), self.sq(ib));
        match self.as_solve(rhs) {
            None => ([0, 0], 0),
            Some(u) => ([self.mul(k, u), self.mul(k, u ^ 1)], 2),
        }
    }
    fn reset_counts(&self) {
        self.muls.set(0);
        self.invs.set(0);
        self.as_solves.set(0);
    }
}

// ------------------------------------------------------------------ curve

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Pt {
    Inf,
    Aff(u64, u64),
}

struct Curve<'a> {
    gf: &'a Gf,
    a: u64,
    b: u64,
}

impl<'a> Curve<'a> {
    fn on_curve(&self, p: Pt) -> bool {
        match p {
            Pt::Inf => true,
            Pt::Aff(x, y) => {
                let g = self.gf;
                let x2 = g.sq(x);
                g.sq(y) ^ g.mul(x, y) == g.mul(x2, x) ^ g.mul(self.a, x2) ^ self.b
            }
        }
    }
    /// A rational point with abscissa x, if any (the other is its negative).
    fn lift_x(&self, x: u64) -> Option<Pt> {
        let g = self.gf;
        if x == 0 {
            return Some(Pt::Aff(0, g.sqrt(self.b)));
        }
        let ix = g.inv(x);
        let rhs = x ^ self.a ^ g.mul(self.b, g.sq(ix));
        g.as_solve(rhs).map(|z| Pt::Aff(x, g.mul(x, z)))
    }
    fn neg(&self, p: Pt) -> Pt {
        match p {
            Pt::Inf => Pt::Inf,
            Pt::Aff(x, y) => Pt::Aff(x, x ^ y),
        }
    }
    fn add(&self, p: Pt, q: Pt) -> Pt {
        let g = self.gf;
        match (p, q) {
            (Pt::Inf, _) => q,
            (_, Pt::Inf) => p,
            (Pt::Aff(x1, y1), Pt::Aff(x2, y2)) => {
                if x1 == x2 {
                    if y1 != y2 || x1 == 0 {
                        return Pt::Inf; // q = -p (or p = q = 2-torsion)
                    }
                    let lam = x1 ^ g.mul(y1, g.inv(x1));
                    let x3 = g.sq(lam) ^ lam ^ self.a;
                    let y3 = g.sq(x1) ^ g.mul(lam ^ 1, x3);
                    Pt::Aff(x3, y3)
                } else {
                    let lam = g.mul(y1 ^ y2, g.inv(x1 ^ x2));
                    let x3 = g.sq(lam) ^ lam ^ x1 ^ x2 ^ self.a;
                    let y3 = g.mul(lam, x1 ^ x3) ^ x3 ^ y1;
                    Pt::Aff(x3, y3)
                }
            }
        }
    }
    fn s3(&self, u: u64, v: u64, w: u64) -> u64 {
        let g = self.gf;
        let uv = g.mul(u, v);
        g.sq(uv ^ g.mul(u, w) ^ g.mul(v, w)) ^ g.mul(uv, w) ^ self.b
    }
    /// (all three abscissae lift to rational points, some signed sum has abscissa x_r)
    fn lift_check(&self, xs: [u64; 3], x_r: u64) -> (bool, bool) {
        let p: Vec<Option<Pt>> = xs.iter().map(|&x| self.lift_x(x)).collect();
        if p.iter().any(|q| q.is_none()) {
            return (false, false);
        }
        let (p1, p2, p3) = (p[0].unwrap(), p[1].unwrap(), p[2].unwrap());
        for q in [p1, p2, p3] {
            assert!(self.on_curve(q));
        }
        for s2 in 0..2 {
            for s3 in 0..2 {
                let a = if s2 == 0 { p2 } else { self.neg(p2) };
                let b = if s3 == 0 { p3 } else { self.neg(p3) };
                if let Pt::Aff(x, _) = self.add(self.add(p1, a), b) {
                    if x == x_r {
                        return (true, true);
                    }
                }
            }
        }
        (true, false)
    }
}

// ------------------------------------------------------------------ exact oracle: pair enumeration

struct OracleOut {
    witness: Option<[u64; 3]>,
    pairs: u64,
    solutions: u64,
}

/// ALG0 by enumeration of all pairs x1 <= x2 in V (zero included), the roots t of
/// S3(x1, x2, t) = 0 and the roots x3 of S3(t, x3, x_R) = 0. Never stops early.
fn oracle_pairs(cv: &Curve, l: u32, x_r: u64) -> OracleOut {
    let g = cv.gf;
    let b = cv.b;
    let pow = 1u64 << l;
    let mut out = OracleOut { witness: None, pairs: 0, solutions: 0 };
    let xr2 = g.sq(x_r);
    for x1 in 0..pow {
        for x2 in x1..pow {
            out.pairs += 1;
            let q = x1 ^ x2;
            let p = g.mul(x1, x2);
            // q^2 t^2 + p t + (p^2 + b) = 0
            let c = g.sq(p) ^ b;
            let (ts, nt) = if q == 0 {
                if p == 0 {
                    ([0, 0], 0)
                } else {
                    ([g.mul(c, g.inv(p)), 0], 1)
                }
            } else {
                g.quad_roots(g.sq(q), p, c)
            };
            for &t in &ts[..nt] {
                // (t + xR)^2 x3^2 + t xR x3 + (t^2 xR^2 + b) = 0
                let a2 = g.sq(t ^ x_r);
                let bb = g.mul(t, x_r);
                let cc = g.mul(g.sq(t), xr2) ^ b;
                let (xs, nx) = if a2 == 0 {
                    if bb == 0 {
                        ([0, 0], 0)
                    } else {
                        ([g.mul(cc, g.inv(bb)), 0], 1)
                    }
                } else {
                    g.quad_roots(a2, bb, cc)
                };
                for &x3 in &xs[..nx] {
                    if x3 < pow {
                        debug_assert!(cv.s3(x1, x2, t) == 0 && cv.s3(t, x3, x_r) == 0);
                        out.solutions += 1;
                        if out.witness.is_none() {
                            out.witness = Some([x1, x2, x3]);
                        }
                    }
                }
            }
        }
    }
    out
}

// ------------------------------------------------------------------ plain native 2+1 scan

#[derive(Default, Clone)]
struct ScanOut {
    witness: Option<[u64; 3]>,
    x1_visited: u64,
    x1_with_roots: u64,
    leaves: u64,
    leaves_consistent: u64,
    row_xors: u64,
    xtimes: u64,
    solutions: u64,
}

/// For every x1 in V \ {0}: the roots t of S3(x1, t, x_R) = 0; for each t the F2-linear system
/// p^2 + s^2 t^2 + p t = b in (s in V, p in <V.V>), then a root z in V of z^2 + s z + p.
fn scan21(cv: &Curve, l: u32, x_r: u64, stop_at_first: bool) -> ScanOut {
    let g = cv.gf;
    let n = g.n;
    let b = cv.b;
    let pow = 1u64 << l;
    let np = 2 * l - 1; // dim <V.V>
    let ncols = (l + np) as usize;
    assert!(ncols <= 63);
    // z^(2j) reduced, j < 2l-1
    let zsq: Vec<u64> = (0..np).map(|j| raw_mul(n, g.irr, 1u64 << j, 1u64 << j)).collect();
    let xtime = |a: u64| -> u64 {
        let a = a << 1;
        if (a >> n) & 1 == 1 {
            a ^ g.irr
        } else {
            a
        }
    };
    let mut out = ScanOut::default();
    let mut basis = vec![(0u64, 0u64); n as usize];
    for x1 in 1..pow {
        out.x1_visited += 1;
        let a2 = g.sq(x1 ^ x_r);
        let bb = g.mul(x_r, x1);
        let cc = g.sq(bb) ^ b;
        let (ts, nt) = g.quad_roots(a2, bb, cc);
        if nt > 0 {
            out.x1_with_roots += 1;
        }
        for &t in &ts[..nt] {
            out.leaves += 1;
            for e in basis.iter_mut() {
                *e = (0, 0);
            }
            let mut kernel: Vec<u64> = Vec::new();
            let t2 = g.sq(t);
            // columns: s_j -> t^2 z^(2j) (j < l); p_j -> z^(2j) + t z^j (j < 2l-1)
            let mut c_s = t2;
            let mut c_p = t;
            for j in 0..ncols {
                let mut col = if j < l as usize {
                    let c = c_s;
                    c_s = xtime(xtime(c_s));
                    out.xtimes += 2;
                    c
                } else {
                    let jj = j - l as usize;
                    let c = zsq[jj] ^ c_p;
                    c_p = xtime(c_p);
                    out.xtimes += 1;
                    c
                };
                let mut comb = 1u64 << j;
                loop {
                    if col == 0 {
                        kernel.push(comb);
                        break;
                    }
                    let d = deg(col) as usize;
                    if basis[d].0 == 0 {
                        basis[d] = (col, comb);
                        break;
                    }
                    col ^= basis[d].0;
                    comb ^= basis[d].1;
                    out.row_xors += 1;
                }
            }
            let mut rhs = b;
            let mut sol = 0u64;
            let mut ok = true;
            while rhs != 0 {
                let d = deg(rhs) as usize;
                if basis[d].0 == 0 {
                    ok = false;
                    break;
                }
                rhs ^= basis[d].0;
                sol ^= basis[d].1;
                out.row_xors += 1;
            }
            if !ok {
                continue;
            }
            out.leaves_consistent += 1;
            let kd = kernel.len();
            assert!(kd <= 20, "leaf kernel too large");
            for m in 0..(1u64 << kd) {
                let mut v = sol;
                for (i, kv) in kernel.iter().enumerate() {
                    if (m >> i) & 1 == 1 {
                        v ^= kv;
                    }
                }
                let s = v & (pow - 1);
                let p = v >> l;
                // root z in V of z^2 + s z + p
                let z = if s == 0 {
                    Some(g.sqrt(p))
                } else {
                    let is = g.inv(s);
                    g.as_solve(g.mul(p, g.sq(is))).map(|w| g.mul(s, w))
                };
                if let Some(z) = z {
                    if z < pow {
                        out.solutions += 1;
                        if out.witness.is_none() {
                            out.witness = Some([x1, z, z ^ s]);
                        }
                        if stop_at_first {
                            return out;
                        }
                    }
                }
            }
        }
    }
    out
}

// ------------------------------------------------------------------ the lin21 encoding

struct Lin21Instance {
    solver: LinCdcl,
    x1: Vec<u32>,
    s: Vec<u32>,
    zz: Vec<u32>,
    t: Vec<u32>,
}

impl Lin21Instance {
    /// Blocks (n bits each): x1, u, A2, B, k, E, w, C, t, s, p, ST, H, zz, Q.
    ///   static rows: A2 = (x1 + xR)^2;  B = xR x1;  C = B^2 + b;  w = u^2 + u;
    ///                p^2 + ST^2 + H = b;  zz^2 + Q + p = 0
    ///   gates:       B = k A2;  E = B k;  C = E w;  t = k u;  ST = t s;  H = t p;  Q = s zz
    /// So k = B/A2, t = k u and A2 t^2 + B t + C = E (u^2 + u) + C: the quadratic
    /// S3(x1, t, xR) = 0 becomes the fixed-kernel equation E w = C, whose two roots differ in
    /// bit 0 of u only. Priority: the l bits of x1 (and u_0 if `prio_u0`).
    fn build(n: u32, l: u32, irr: u64, x_r: u64, b: u64, prio_u0: bool) -> Lin21Instance {
        let nn = n as usize;
        let mut next = 1u32;
        let mut block = || {
            let v: Vec<u32> = (next..next + n).collect();
            next += n;
            v
        };
        let x1 = block();
        let u = block();
        let a2 = block();
        let bb = block();
        let k = block();
        let e = block();
        let w = block();
        let c = block();
        let t = block();
        let s = block();
        let p = block();
        let st = block();
        let h = block();
        let zz = block();
        let q = block();
        let mut sv = LinCdcl::new((next - 1) as usize, n, irr);
        let sq = sv.sq_matrix();
        let mr = sv.mul_matrix(x_r);
        let xr2 = sv.gf_mul(x_r, x_r);
        // domains
        for j in l as usize..nn {
            sv.add_clause(vec![-(x1[j] as i32)]);
            sv.add_clause(vec![-(s[j] as i32)]);
            sv.add_clause(vec![-(zz[j] as i32)]);
        }
        for j in (2 * l - 1) as usize..nn {
            sv.add_clause(vec![-(p[j] as i32)]);
        }
        // static linear rows
        sv.add_linear_map(&a2, &x1, &sq, xr2);
        sv.add_linear_map(&bb, &x1, &mr, 0);
        sv.add_linear_map(&c, &bb, &sq, b);
        let sq_plus_id: Vec<u64> = (0..nn).map(|j| sq[j] ^ (1u64 << j)).collect();
        sv.add_linear_map(&w, &u, &sq_plus_id, 0);
        for kk in 0..nn {
            let mut vars = vec![h[kk]];
            for j in 0..nn {
                if (sq[j] >> kk) & 1 == 1 {
                    vars.push(p[j]);
                    vars.push(st[j]);
                }
            }
            sv.add_xor(vars, (b >> kk) & 1 == 1);
        }
        for kk in 0..nn {
            let mut vars = vec![q[kk], p[kk]];
            for j in 0..nn {
                if (sq[j] >> kk) & 1 == 1 {
                    vars.push(zz[j]);
                }
            }
            sv.add_xor(vars, false);
        }
        // gates z = x * y
        sv.add_gate(k.clone(), a2.clone(), bb.clone());
        sv.add_gate(bb.clone(), k.clone(), e.clone());
        sv.add_gate(e.clone(), w.clone(), c.clone());
        sv.add_gate(k.clone(), u.clone(), t.clone());
        sv.add_gate(t.clone(), s.clone(), st.clone());
        sv.add_gate(t.clone(), p.clone(), h.clone());
        sv.add_gate(s.clone(), zz.clone(), q.clone());
        let mut prio: Vec<u32> = x1[..l as usize].to_vec();
        if prio_u0 {
            prio.push(u[0]);
        }
        sv.set_priority(&prio);
        Lin21Instance { solver: sv, x1, s, zz, t }
    }
}

// ------------------------------------------------------------------ helpers

struct SplitMix(u64);
impl SplitMix {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }
}

/// Target admission rule: x_R outside V, R rational, and sqrt(b)/x_R outside V (so R is not
/// "base point + 2-torsion point", the case with non-liftable twist models).
fn target_ok(cv: &Curve, l: u32, x: u64) -> bool {
    let pow = 1u64 << l;
    if x < pow || cv.lift_x(x).is_none() {
        return false;
    }
    cv.gf.mul(cv.gf.sqrt(cv.b), cv.gf.inv(x)) >= pow
}

struct SatOut {
    status: &'static str,
    xs: Option<[u64; 3]>,
    json: String,
}

fn stats_json(s: &pdp_lin_cdcl::Stats, n_vars: usize, build_ms: f64, wall_ms: f64) -> String {
    format!(
        "\"conflicts\":{},\"decisions\":{},\"propagations\":{},\"xor_propagations\":{},\"gje_calls\":{},\"gate_linearisations\":{},\"learnt_clauses\":{},\"learnt_lits\":{},\"learnt_priority_width\":{:?},\"n_vars\":{},\"build_ms\":{:.4},\"wall_ms\":{:.4}",
        s.conflicts, s.decisions, s.propagations, s.xor_propagations, s.gje_calls,
        s.gate_linearisations, s.learnt_clauses, s.learnt_lits, s.learnt_priority_width,
        n_vars, build_ms, wall_ms
    )
}

fn run_lin(n: u32, l: u32, irr: u64, x_r: u64, b: u64, lex: bool, budget: u64) -> SatOut {
    let t0 = Instant::now();
    let mut inst = ChainInstance::build(n, l, irr, x_r, b, lex);
    let build_ms = t0.elapsed().as_secs_f64() * 1e3;
    inst.solver.conflict_budget = budget;
    let t1 = Instant::now();
    let r = inst.solver.solve();
    let wall_ms = t1.elapsed().as_secs_f64() * 1e3;
    let (status, xs) = match r {
        LinResult::Sat => (
            "sat",
            Some([
                inst.solver.model_block(&inst.x1),
                inst.solver.model_block(&inst.x2),
                inst.solver.model_block(&inst.x3),
            ]),
        ),
        LinResult::Unsat => ("unsat", None),
        LinResult::Unknown => ("unknown", None),
    };
    SatOut { status, xs, json: stats_json(&inst.solver.stats, inst.solver.n_vars, build_ms, wall_ms) }
}

fn run_lin21(n: u32, l: u32, irr: u64, x_r: u64, b: u64, prio_u0: bool, budget: u64) -> SatOut {
    let t0 = Instant::now();
    let mut inst = Lin21Instance::build(n, l, irr, x_r, b, prio_u0);
    let build_ms = t0.elapsed().as_secs_f64() * 1e3;
    inst.solver.conflict_budget = budget;
    let t1 = Instant::now();
    let r = inst.solver.solve();
    let wall_ms = t1.elapsed().as_secs_f64() * 1e3;
    let (status, xs) = match r {
        LinResult::Sat => {
            let z = inst.solver.model_block(&inst.zz);
            let s = inst.solver.model_block(&inst.s);
            ("sat", Some([inst.solver.model_block(&inst.x1), z, z ^ s]))
        }
        LinResult::Unsat => ("unsat", None),
        LinResult::Unknown => ("unknown", None),
    };
    let _ = &inst.t;
    SatOut { status, xs, json: stats_json(&inst.solver.stats, inst.solver.n_vars, build_ms, wall_ms) }
}

fn sat_arm(arm: &str, n: u32, l: u32, irr: u64, x_r: u64, b: u64, budget: u64) -> SatOut {
    match arm {
        "lin" => run_lin(n, l, irr, x_r, b, true, budget),
        "lin-nolex" => run_lin(n, l, irr, x_r, b, false, budget),
        "lin21" => run_lin21(n, l, irr, x_r, b, false, budget),
        "lin21-pu" => run_lin21(n, l, irr, x_r, b, true, budget),
        _ => panic!("unknown SAT arm {arm}"),
    }
}

fn xs_json(xs: Option<[u64; 3]>) -> String {
    match xs {
        Some(x) => format!("[{},{},{}]", x[0], x[1], x[2]),
        None => "null".into(),
    }
}

/// (in V^3, liftable, sums to target, both chain equations hold for some t)
fn check_model(cv: &Curve, l: u32, xs: [u64; 3], x_r: u64) -> (bool, bool, bool) {
    let in_base = xs.iter().all(|&x| x < (1u64 << l));
    let (lift, sums) = cv.lift_check(xs, x_r);
    (in_base, lift, sums)
}

// ------------------------------------------------------------------ selftest

fn selftest_cell(n: u32, l: u32, sample: Option<usize>, seed: u64) -> (String, bool) {
    let irr = find_irr(n);
    let gf = Gf::new(n, irr);
    let cv = Curve { gf: &gf, a: 1, b: 1 };
    let pow = 1u64 << l;
    // factor base with zero allowed (ALG0 semantics): all rational points with x in V
    let mut fb: Vec<Pt> = Vec::new();
    for x in 0..pow {
        if let Some(p) = cv.lift_x(x) {
            assert!(cv.on_curve(p));
            fb.push(p);
            let m = cv.neg(p);
            if m != p {
                fb.push(m);
            }
        }
    }
    // group law sanity: associativity and inverse on base points
    for i in 0..fb.len().min(12) {
        for j in 0..fb.len().min(12) {
            let (p, q) = (fb[i], fb[j]);
            let r = fb[(i * 7 + j * 3 + 1) % fb.len()];
            assert!(cv.add(cv.add(p, q), r) == cv.add(p, cv.add(q, r)), "associativity");
            assert!(cv.on_curve(cv.add(p, q)));
            assert!(cv.add(p, cv.neg(p)) == Pt::Inf);
        }
    }
    let mut pt0: HashSet<u64> = HashSet::new();
    let mut inf_sums = 0u64;
    for &p in &fb {
        for &q in &fb {
            let pq = cv.add(p, q);
            for &r in &fb {
                match cv.add(pq, r) {
                    Pt::Aff(x, _) => {
                        pt0.insert(x);
                    }
                    Pt::Inf => inf_sums += 1,
                }
            }
        }
    }
    let mut cand: Vec<u64> = (pow..(1u64 << n)).filter(|&x| cv.lift_x(x).is_some()).collect();
    let total_rational = cand.len();
    if let Some(k) = sample {
        // keep every rule-excluded target and every decomposable one, sample the rest
        let mut rng = SplitMix(seed);
        let mut keep: Vec<u64> = Vec::new();
        let mut rest: Vec<u64> = Vec::new();
        for &x in &cand {
            if !target_ok(&cv, l, x) || pt0.contains(&x) {
                keep.push(x);
            } else {
                rest.push(x);
            }
        }
        // cap the kept decomposables too
        while keep.len() > k {
            let i = (rng.next() % keep.len() as u64) as usize;
            keep.swap_remove(i);
        }
        for _ in 0..k.min(rest.len()) {
            let i = (rng.next() % rest.len() as u64) as usize;
            keep.push(rest.swap_remove(i));
        }
        cand = keep;
    }
    let arms = ["lin", "lin-nolex", "lin21", "lin21-pu"];
    let (mut n_targets, mut n_alg, mut n_rule_ok, mut n_excl) = (0u64, 0u64, 0u64, 0u64);
    let (mut verdict_mismatch, mut scan_mismatch, mut alg_ne_pt_ok, mut alg_ne_pt_excl) =
        (0u64, 0u64, 0u64, 0u64);
    let (mut models, mut nonlift_ok, mut nonlift_excl, mut bad_models) = (0u64, 0u64, 0u64, 0u64);
    let mut zero_only = 0u64; // decomposable targets all of whose oracle witnesses..: see below
    for &x in &cand {
        n_targets += 1;
        let ok = target_ok(&cv, l, x);
        if ok {
            n_rule_ok += 1;
        } else {
            n_excl += 1;
        }
        let orc = oracle_pairs(&cv, l, x);
        let alg = orc.witness.is_some();
        if alg {
            n_alg += 1;
        }
        let pt = pt0.contains(&x);
        if alg != pt {
            if ok {
                alg_ne_pt_ok += 1;
            } else {
                alg_ne_pt_excl += 1;
            }
        }
        if let Some(wt) = orc.witness {
            if wt.iter().any(|&v| v == 0) {
                zero_only += 1;
            }
        }
        let sc = scan21(&cv, l, x, false);
        if sc.witness.is_some() != alg {
            scan_mismatch += 1;
        }
        let sc1 = scan21(&cv, l, x, true);
        if sc1.witness.is_some() != alg {
            scan_mismatch += 1;
        }
        if let Some(w) = sc.witness {
            // the scan's witness must satisfy the chain for some t (algebraic check)
            let any = oracle_has(&cv, w, x);
            if !any {
                bad_models += 1;
            }
        }
        for arm in arms {
            let r = sat_arm(arm, n, l, irr, x, 1, u64::MAX);
            let sat = r.status == "sat";
            if sat != alg || r.status == "unknown" {
                verdict_mismatch += 1;
            }
            if let Some(xs) = r.xs {
                models += 1;
                let (in_base, lift, sums) = check_model(&cv, l, xs, x);
                if !in_base || !oracle_has(&cv, xs, x) {
                    bad_models += 1;
                }
                if !(lift && sums) {
                    if ok {
                        nonlift_ok += 1;
                    } else {
                        nonlift_excl += 1;
                    }
                }
            }
        }
    }
    let pass = verdict_mismatch == 0
        && scan_mismatch == 0
        && alg_ne_pt_ok == 0
        && nonlift_ok == 0
        && bad_models == 0;
    (
        format!(
            "{{\"n\":{n},\"l\":{l},\"irr\":{irr},\"base_points\":{},\"rational_targets_outside_V\":{total_rational},\"targets_tested\":{n_targets},\"rule_ok\":{n_rule_ok},\"rule_excluded\":{n_excl},\"alg_decomposable\":{n_alg},\"witness_with_zero\":{zero_only},\"triple_sums_at_infinity\":{inf_sums},\"sat_arm_verdict_mismatches\":{verdict_mismatch},\"scan_mismatches\":{scan_mismatch},\"alg_ne_point_truth_rule_ok\":{alg_ne_pt_ok},\"alg_ne_point_truth_rule_excluded\":{alg_ne_pt_excl},\"sat_models\":{models},\"nonliftable_models_rule_ok\":{nonlift_ok},\"nonliftable_models_rule_excluded\":{nonlift_excl},\"bad_models\":{bad_models},\"pass\":{pass}}}",
            fb.len()
        ),
        pass,
    )
}

/// Algebraic check of a model: some t in F with S3(x1, x2, t) = 0 and S3(t, x3, xR) = 0, in
/// any of the three ways of pairing (the problem is symmetric in the summands).
fn oracle_has(cv: &Curve, xs: [u64; 3], x_r: u64) -> bool {
    let g = cv.gf;
    for (i, j, k) in [(0, 1, 2), (0, 2, 1), (1, 2, 0)] {
        let (x1, x2, x3) = (xs[i], xs[j], xs[k]);
        let q = x1 ^ x2;
        let p = g.mul(x1, x2);
        let c = g.sq(p) ^ cv.b;
        let (ts, nt) = if q == 0 {
            if p == 0 {
                ([0, 0], 0)
            } else {
                ([g.mul(c, g.inv(p)), 0], 1)
            }
        } else {
            g.quad_roots(g.sq(q), p, c)
        };
        for &t in &ts[..nt] {
            if cv.s3(x1, x2, t) == 0 && cv.s3(t, x3, x_r) == 0 {
                return true;
            }
        }
    }
    false
}

// ------------------------------------------------------------------ main

fn arg(v: &[String], name: &str) -> Option<String> {
    v.iter().position(|a| a == name).map(|i| v[i + 1].clone())
}

fn main() {
    let v: Vec<String> = std::env::args().collect();
    let cmd = v.get(1).map(|s| s.as_str()).unwrap_or("");
    match cmd {
        "selftest" => {
            let mut all = true;
            // (n, l, sample): n = 3l+2 exhaustive at l = 3; sampled at l = 4; n = 3l-1 and
            // n = 3l-2 (leaf not overdetermined) exhaustive at l = 3.
            for (n, l, sample) in [(11u32, 3u32, None), (8, 3, None), (7, 3, None), (14, 4, Some(60usize))] {
                let (js, pass) = selftest_cell(n, l, sample, 77);
                println!("{js}");
                all &= pass;
            }
            if !all {
                std::process::exit(1);
            }
        }
        "targets" => {
            let l: u32 = arg(&v, "--l").expect("--l").parse().unwrap();
            let n: u32 = arg(&v, "--n").map(|s| s.parse().unwrap()).unwrap_or(3 * l + 2);
            let seed: u64 = arg(&v, "--seed").expect("--seed").parse().unwrap();
            let n_uni: usize = arg(&v, "--uniform").map(|s| s.parse().unwrap()).unwrap_or(16);
            let n_pl: usize = arg(&v, "--planted").map(|s| s.parse().unwrap()).unwrap_or(4);
            let irr = find_irr(n);
            let gf = Gf::new(n, irr);
            let cv = Curve { gf: &gf, a: 1, b: 1 };
            let pow = 1u64 << l;
            let mask = (1u64 << n) - 1;
            let mut rng = SplitMix(seed ^ ((n as u64) << 32) ^ ((l as u64) << 16));
            let mut idx = 0usize;
            let mut emit = |kind: &str, x: u64, planted: Option<[u64; 3]>, orc: &OracleOut, ms: f64| {
                println!(
                    "{{\"idx\":{idx},\"kind\":\"{kind}\",\"n\":{n},\"l\":{l},\"irr\":{irr},\"seed\":{seed},\"x_r\":{x},\"planted_xs\":{},\"oracle_sat\":{},\"oracle_witness\":{},\"oracle_pairs\":{},\"oracle_solutions\":{},\"oracle_ms\":{ms:.3}}}",
                    xs_json(planted), orc.witness.is_some(), xs_json(orc.witness), orc.pairs, orc.solutions
                );
                idx += 1;
            };
            let mut got = 0;
            while got < n_uni {
                let x = rng.next() & mask;
                if !target_ok(&cv, l, x) {
                    continue;
                }
                let t0 = Instant::now();
                let orc = oracle_pairs(&cv, l, x);
                let ms = t0.elapsed().as_secs_f64() * 1e3;
                if orc.witness.is_none() {
                    got += 1;
                    emit("uniform", x, None, &orc, ms);
                } else {
                    emit("uniform-dec", x, None, &orc, ms);
                }
            }
            let mut fb: Vec<Pt> = Vec::new();
            for x in 1..pow {
                if let Some(p) = cv.lift_x(x) {
                    fb.push(p);
                    fb.push(cv.neg(p));
                }
            }
            let mut got = 0;
            while got < n_pl {
                let pick = |r: &mut SplitMix| fb[(r.next() % fb.len() as u64) as usize];
                let (p1, p2, p3) = (pick(&mut rng), pick(&mut rng), pick(&mut rng));
                let xo = |p: Pt| if let Pt::Aff(x, _) = p { x } else { unreachable!() };
                let xs = [xo(p1), xo(p2), xo(p3)];
                if xs[0] == xs[1] || xs[0] == xs[2] || xs[1] == xs[2] {
                    continue;
                }
                let r = cv.add(cv.add(p1, p2), p3);
                let Pt::Aff(x, _) = r else { continue };
                if !target_ok(&cv, l, x) {
                    continue;
                }
                let t0 = Instant::now();
                let orc = oracle_pairs(&cv, l, x);
                let ms = t0.elapsed().as_secs_f64() * 1e3;
                assert!(orc.witness.is_some(), "planted target not decomposable by the oracle");
                got += 1;
                emit("planted", x, Some(xs), &orc, ms);
            }
        }
        "run" => {
            let l: u32 = arg(&v, "--l").expect("--l").parse().unwrap();
            let n: u32 = arg(&v, "--n").map(|s| s.parse().unwrap()).unwrap_or(3 * l + 2);
            let x_r: u64 = arg(&v, "--xr").expect("--xr").parse().unwrap();
            let arm = arg(&v, "--arm").expect("--arm");
            let budget: u64 = arg(&v, "--budget").map(|s| s.parse().unwrap()).unwrap_or(u64::MAX);
            let reps: usize = arg(&v, "--reps").map(|s| s.parse().unwrap()).unwrap_or(21);
            let irr = find_irr(n);
            let gf = Gf::new(n, irr);
            let cv = Curve { gf: &gf, a: 1, b: 1 };
            let head = format!("\"arm\":\"{arm}\",\"n\":{n},\"l\":{l},\"irr\":{irr},\"x_r\":{x_r}");
            if arm == "scan" || arm == "scan-first" {
                let first = arm == "scan-first";
                gf.reset_counts();
                let out = scan21(&cv, l, x_r, first);
                let (muls, invs, ass) = (gf.muls.get(), gf.invs.get(), gf.as_solves.get());
                let mut times: Vec<f64> = Vec::new();
                for _ in 0..reps {
                    let t0 = Instant::now();
                    let o = scan21(&cv, l, x_r, first);
                    times.push(t0.elapsed().as_secs_f64() * 1e3);
                    assert!(o.witness == out.witness);
                }
                times.sort_by(|a, b| a.partial_cmp(b).unwrap());
                let (status, chk) = match out.witness {
                    Some(xs) => {
                        let (ib, lf, sm) = check_model(&cv, l, xs, x_r);
                        ("sat", format!("\"in_base\":{ib},\"liftable\":{lf},\"sums_to_target\":{sm},\"chain_ok\":{}", oracle_has(&cv, xs, x_r)))
                    }
                    None => ("unsat", "\"in_base\":null,\"liftable\":null,\"sums_to_target\":null,\"chain_ok\":null".into()),
                };
                println!(
                    "{{{head},\"status\":\"{status}\",\"model_xs\":{},{chk},\"x1_visited\":{},\"x1_with_roots\":{},\"leaves\":{},\"leaves_consistent\":{},\"solutions\":{},\"field_muls\":{muls},\"field_invs\":{invs},\"as_solves\":{ass},\"xtimes\":{},\"row_xors\":{},\"reps\":{reps},\"wall_ms\":{:.6},\"wall_ms_min\":{:.6}}}",
                    xs_json(out.witness), out.x1_visited, out.x1_with_roots, out.leaves, out.leaves_consistent,
                    out.solutions, out.xtimes, out.row_xors, times[times.len() / 2], times[0]
                );
            } else {
                let r = sat_arm(&arm, n, l, irr, x_r, 1, budget);
                let chk = match r.xs {
                    Some(xs) => {
                        let (ib, lf, sm) = check_model(&cv, l, xs, x_r);
                        format!("\"in_base\":{ib},\"liftable\":{lf},\"sums_to_target\":{sm},\"chain_ok\":{}", oracle_has(&cv, xs, x_r))
                    }
                    None => "\"in_base\":null,\"liftable\":null,\"sums_to_target\":null,\"chain_ok\":null".into(),
                };
                println!("{{{head},\"status\":\"{}\",\"model_xs\":{},{chk},{}}}", r.status, xs_json(r.xs), r.json);
            }
        }
        _ => {
            eprintln!("usage: lin21 selftest | targets --l L --seed S [--n N --uniform U --planted P] | run --l L --xr X --arm lin|lin-nolex|lin21|lin21-pu|scan|scan-first [--n N --budget B --reps K]");
            std::process::exit(2);
        }
    }
}
