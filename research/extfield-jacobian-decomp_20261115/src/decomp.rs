//! Nagao/Gaudry decomposition on genus-2 Jacobians over F_{q^2}.
//!
//! For R = (u, v), u monic of degree 2, write h = v + u*(x + b)/a and
//! G = (f - h^2)/u = -(1/a^2) * (x^4 + c3 x^3 + c2 x^2 + c1 x + c0).
//! R + P1 + .. + P4 ~ 6*inf with x(P_i) in F_q iff all c_k lie in F_q and
//! the quartic splits over F_q.  With a = z0 + z1 s, b = z2 + z3 s the
//! conditions Im(c_k) = 0 are four quadrics in four unknowns over F_q
//! (Bezout number 16).  They are solved by a degree-5 Macaulay matrix,
//! multiplication matrices on the quotient, a characteristic polynomial and
//! root finding over F_q.  A pair-hash collinearity oracle is the
//! correctness reference.

use super::fq2::{Fld, E, ONE, ZERO};
use super::jac::{Curve, Div};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering as AO};

/// Native F_q multiply-adds charged by the decomposition solver.
pub static NATIVE_MULS: AtomicU64 = AtomicU64::new(0);
pub fn charge(n: u64) {
    NATIVE_MULS.fetch_add(n, AO::Relaxed);
}
pub fn native_now() -> u64 {
    NATIVE_MULS.load(AO::Relaxed)
}

// ---------------------------------------------------------------- equations

/// Monomials of a quadric in z0..z3: 1, z0..z3, then z_i z_j (i <= j).
pub fn term_exps() -> Vec<[u8; 4]> {
    let mut t = vec![[0u8; 4]];
    for i in 0..4 {
        let mut e = [0u8; 4];
        e[i] = 1;
        t.push(e);
    }
    for i in 0..4 {
        for j in i..4 {
            let mut e = [0u8; 4];
            e[i] += 1;
            e[j] += 1;
            t.push(e);
        }
    }
    t
}

fn quad_index(i: usize, j: usize) -> usize {
    // i, j in 0..5 where 0 = constant and k = z_{k-1}
    match (i, j) {
        (0, 0) => 0,
        (0, k) | (k, 0) => k,
        (i, j) => {
            let (i, j) = (i.min(j) - 1, i.max(j) - 1);
            let mut idx = 5;
            for a in 0..4 {
                for b in a..4 {
                    if a == i && b == j {
                        return idx;
                    }
                    idx += 1;
                }
            }
            unreachable!()
        }
    }
}

type Lin = [E; 5];
type Quad = [E; 15];

fn mul_lin(fl: &Fld, l1: &Lin, l2: &Lin) -> Quad {
    let mut out = [ZERO; 15];
    for i in 0..5 {
        for j in 0..5 {
            if l1[i] == ZERO || l2[j] == ZERO {
                continue;
            }
            let idx = quad_index(i, j);
            out[idx] = fl.add(out[idx], fl.mul(l1[i], l2[j]));
        }
    }
    out
}

fn qacc(fl: &Fld, acc: &mut Quad, coef: E, q: &Quad) {
    for k in 0..15 {
        acc[k] = fl.add(acc[k], fl.mul(coef, q[k]));
    }
}

/// The four real quadrics Im(c_k)(z) = 0 for R = (u, v), deg u = 2.
/// Returns (equations, w) with w the cubic (f - v^2)/u.
pub fn equations(cv: &Curve, r: &Div) -> Option<([[u32; 15]; 4], Vec<E>)> {
    let fl = &cv.fld;
    if r.u.len() != 3 {
        return None;
    }
    let num = fl.psub(&cv.f, &fl.pmul(&r.v, &r.v));
    let (w, rem) = fl.pdivrem(&num, &r.u);
    if !rem.is_empty() || w.len() != 4 {
        return None;
    }
    let g = |p: &[E], i: usize| p.get(i).copied().unwrap_or(ZERO);
    let (u0, u1) = (g(&r.u, 0), g(&r.u, 1));
    let (v0, v1) = (g(&r.v, 0), g(&r.v, 1));
    let (w0, w1, w2, w3) = (g(&w, 0), g(&w, 1), g(&w, 2), g(&w, 3));
    let s = E { a: 0, b: 1 };
    let a_l: Lin = [ZERO, ONE, s, ZERO, ZERO];
    let b_l: Lin = [ZERO, ZERO, ZERO, ONE, s];
    let aa = mul_lin(fl, &a_l, &a_l);
    let bb = mul_lin(fl, &b_l, &b_l);
    let ab = mul_lin(fl, &a_l, &b_l);
    let as_quad = |l: &Lin| -> Quad {
        let mut q = [ZERO; 15];
        q[..5].copy_from_slice(l);
        q
    };
    let aq = as_quad(&a_l);
    let bq = as_quad(&b_l);
    let mut one = [ZERO; 15];
    one[0] = ONE;
    let two = fl.embed(2);
    let neg = |x: E| fl.neg(x);
    let mut c3 = [ZERO; 15];
    qacc(fl, &mut c3, u1, &one);
    qacc(fl, &mut c3, two, &bq);
    qacc(fl, &mut c3, neg(w3), &aa);
    let mut c2 = [ZERO; 15];
    qacc(fl, &mut c2, ONE, &bb);
    qacc(fl, &mut c2, fl.mul(two, u1), &bq);
    qacc(fl, &mut c2, u0, &one);
    qacc(fl, &mut c2, fl.mul(two, v1), &aq);
    qacc(fl, &mut c2, neg(w2), &aa);
    let mut c1 = [ZERO; 15];
    qacc(fl, &mut c1, u1, &bb);
    qacc(fl, &mut c1, fl.mul(two, u0), &bq);
    qacc(fl, &mut c1, fl.mul(two, v1), &ab);
    qacc(fl, &mut c1, fl.mul(two, v0), &aq);
    qacc(fl, &mut c1, neg(w1), &aa);
    let mut c0 = [ZERO; 15];
    qacc(fl, &mut c0, u0, &bb);
    qacc(fl, &mut c0, fl.mul(two, v0), &ab);
    qacc(fl, &mut c0, neg(w0), &aa);
    let mut eqs = [[0u32; 15]; 4];
    for (k, c) in [c3, c2, c1, c0].iter().enumerate() {
        for t in 0..15 {
            eqs[k][t] = c[t].b;
        }
    }
    charge(400);
    Some((eqs, w))
}

fn eval_quad(fl: &Fld, eq: &[u32; 15], z: &[u32; 4], terms: &[[u8; 4]]) -> u32 {
    let mut acc = 0u32;
    for (t, e) in terms.iter().enumerate() {
        if eq[t] == 0 {
            continue;
        }
        let mut m = eq[t];
        for i in 0..4 {
            for _ in 0..e[i] {
                m = fl.qmul(m, z[i]);
            }
        }
        acc = fl.qadd(acc, m);
    }
    acc
}

// ---------------------------------------------------------------- Macaulay

pub struct Macaulay {
    pub d: usize,
    pub mons: Vec<[u8; 4]>,
    col_of: HashMap<u32, usize>,
    mults: Vec<[u8; 4]>,
    term_cols: Vec<[usize; 15]>,
    pub ncols: usize,
    pub nrows: usize,
}

fn pack(e: &[u8; 4]) -> u32 {
    e[0] as u32 | (e[1] as u32) << 8 | (e[2] as u32) << 16 | (e[3] as u32) << 24
}
fn deg(e: &[u8; 4]) -> usize {
    e.iter().map(|&x| x as usize).sum()
}
fn grevlex(a: &[u8; 4], b: &[u8; 4]) -> Ordering {
    deg(a).cmp(&deg(b)).then_with(|| {
        for i in (0..4).rev() {
            if a[i] != b[i] {
                return if a[i] < b[i] {
                    Ordering::Greater
                } else {
                    Ordering::Less
                };
            }
        }
        Ordering::Equal
    })
}

fn monomials_upto(d: usize) -> Vec<[u8; 4]> {
    let mut v = Vec::new();
    for a in 0..=d {
        for b in 0..=d - a {
            for c in 0..=d - a - b {
                for e in 0..=d - a - b - c {
                    v.push([a as u8, b as u8, c as u8, e as u8]);
                }
            }
        }
    }
    v
}

impl Macaulay {
    pub fn new(d: usize) -> Macaulay {
        let mut mons = monomials_upto(d);
        mons.sort_by(|a, b| grevlex(b, a)); // descending: leading terms first
        let col_of: HashMap<u32, usize> =
            mons.iter().enumerate().map(|(i, m)| (pack(m), i)).collect();
        let mut mults = monomials_upto(d - 2);
        mults.sort_by(|a, b| grevlex(b, a));
        let terms = term_exps();
        let mut term_cols = Vec::new();
        for m in &mults {
            let mut row = [0usize; 15];
            for (t, e) in terms.iter().enumerate() {
                let mut x = [0u8; 4];
                for i in 0..4 {
                    x[i] = m[i] + e[i];
                }
                row[t] = col_of[&pack(&x)];
            }
            term_cols.push(row);
        }
        let ncols = mons.len();
        let nrows = 4 * mults.len();
        Macaulay {
            d,
            mons,
            col_of,
            mults,
            term_cols,
            ncols,
            nrows,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Status {
    Ok,
    Inconsistent,
    Insufficient,
    Ambiguous,
}

pub struct SolveOut {
    pub sols: Vec<[u32; 4]>,
    pub status: Status,
    pub dim: usize,
}

fn matmul(fl: &Fld, a: &[Vec<u32>], b: &[Vec<u32>]) -> Vec<Vec<u32>> {
    let n = a.len();
    let q = fl.q as u64;
    let mut out = vec![vec![0u32; n]; n];
    for i in 0..n {
        for j in 0..n {
            let mut acc = 0u64;
            for k in 0..n {
                acc += a[i][k] as u64 * b[k][j] as u64;
            }
            out[i][j] = (acc % q) as u32;
        }
    }
    charge((n * n * n) as u64);
    out
}

/// Characteristic polynomial (ascending coefficients, monic) by Faddeev-LeVerrier.
fn charpoly(fl: &Fld, a: &[Vec<u32>]) -> Vec<u32> {
    let n = a.len();
    let mut c = vec![0u32; n + 1];
    c[n] = 1;
    let mut m = vec![vec![0u32; n]; n];
    for k in 1..=n {
        let mut p = matmul(fl, a, &m);
        for (i, row) in p.iter_mut().enumerate() {
            row[i] = fl.qadd(row[i], c[n - k + 1]);
        }
        m = p;
        let am = matmul(fl, a, &m);
        let mut tr = 0u32;
        for (i, row) in am.iter().enumerate() {
            tr = fl.qadd(tr, row[i]);
        }
        c[n - k] = fl.qneg(fl.qmul(tr, fl.qinv(k as u32 % fl.q)));
    }
    c
}

fn ptrim(p: &mut Vec<u32>) {
    while p.last() == Some(&0) {
        p.pop();
    }
}

fn pmod(fl: &Fld, a: &[u32], m: &[u32]) -> Vec<u32> {
    let mut r = a.to_vec();
    ptrim(&mut r);
    let dm = m.len() - 1;
    let linv = fl.qinv(*m.last().unwrap());
    while r.len() > dm {
        let shift = r.len() - 1 - dm;
        let c = fl.qmul(*r.last().unwrap(), linv);
        for (i, &mi) in m.iter().enumerate() {
            r[shift + i] = fl.qsub(r[shift + i], fl.qmul(c, mi));
        }
        charge(m.len() as u64);
        r.pop();
        ptrim(&mut r);
    }
    r
}

fn pmulm(fl: &Fld, a: &[u32], b: &[u32], m: &[u32]) -> Vec<u32> {
    if a.is_empty() || b.is_empty() {
        return Vec::new();
    }
    let mut out = vec![0u32; a.len() + b.len() - 1];
    for (i, &x) in a.iter().enumerate() {
        for (j, &y) in b.iter().enumerate() {
            out[i + j] = fl.qadd(out[i + j], fl.qmul(x, y));
        }
    }
    charge((a.len() * b.len()) as u64);
    ptrim(&mut out);
    pmod(fl, &out, m)
}

fn pgcd(fl: &Fld, a: &[u32], b: &[u32]) -> Vec<u32> {
    let (mut x, mut y) = (a.to_vec(), b.to_vec());
    ptrim(&mut x);
    ptrim(&mut y);
    while !y.is_empty() {
        let r = pmod(fl, &x, &y);
        x = y;
        y = r;
    }
    x
}

fn peval_q(fl: &Fld, p: &[u32], x: u32) -> u32 {
    let mut acc = 0u32;
    for &c in p.iter().rev() {
        acc = fl.qadd(fl.qmul(acc, x), c);
    }
    acc
}

/// Rational roots of a polynomial over F_q (distinct), by gcd with x^q - x
/// then evaluation on F_q.
fn rational_roots(fl: &Fld, chi: &[u32]) -> Vec<u32> {
    let mut res: Vec<u32> = vec![1];
    let mut base: Vec<u32> = pmod(fl, &[0, 1], chi);
    let mut e = fl.q;
    while e > 0 {
        if e & 1 == 1 {
            res = pmulm(fl, &res, &base, chi);
        }
        base = pmulm(fl, &base, &base, chi);
        e >>= 1;
    }
    let mut diff = res.clone();
    if diff.len() < 2 {
        diff.resize(2, 0);
    }
    diff[1] = fl.qsub(diff[1], 1);
    ptrim(&mut diff);
    let g = if diff.is_empty() {
        chi.to_vec()
    } else {
        pgcd(fl, chi, &diff)
    };
    if g.len() <= 1 {
        return Vec::new();
    }
    let mut roots = Vec::new();
    for x in 0..fl.q {
        if peval_q(fl, &g, x) == 0 {
            roots.push(x);
        }
    }
    charge(g.len() as u64 * fl.q as u64);
    roots
}

/// Null space of B (n x n) mod q: returns basis vectors.
fn nullspace(fl: &Fld, b: &[Vec<u32>]) -> Vec<Vec<u32>> {
    let n = b.len();
    let q = fl.q as u64;
    let mut m: Vec<Vec<u32>> = b.to_vec();
    let mut pivcol = Vec::new();
    let mut r = 0;
    for c in 0..n {
        let Some(p) = (r..n).find(|&i| m[i][c] != 0) else {
            continue;
        };
        m.swap(p, r);
        let inv = fl.qinv(m[r][c]);
        for j in 0..n {
            m[r][j] = fl.qmul(m[r][j], inv);
        }
        for i in 0..n {
            if i != r && m[i][c] != 0 {
                let f = m[i][c] as u64;
                for j in 0..n {
                    let t = (m[i][j] as u64 + (q - f) * m[r][j] as u64) % q;
                    m[i][j] = t as u32;
                }
            }
        }
        charge((n * n) as u64);
        pivcol.push(c);
        r += 1;
    }
    let mut out = Vec::new();
    for free in 0..n {
        if pivcol.contains(&free) {
            continue;
        }
        let mut v = vec![0u32; n];
        v[free] = 1;
        for (row, &pc) in pivcol.iter().enumerate() {
            v[pc] = fl.qneg(m[row][free]);
        }
        out.push(v);
    }
    out
}

pub fn solve_quadrics(mac: &Macaulay, fl: &Fld, eqs: &[[u32; 15]; 4], lrng: &mut u64) -> SolveOut {
    let q = fl.q;
    assert!(q <= 2039, "lazy u32 reduction needs q <= 2039");
    let (nr, nc) = (mac.nrows, mac.ncols);
    let mut mat = vec![0u32; nr * nc];
    let nm = mac.mults.len();
    for k in 0..4 {
        for mi in 0..nm {
            let row = k * nm + mi;
            for t in 0..15 {
                let c = eqs[k][t];
                if c != 0 {
                    mat[row * nc + mac.term_cols[mi][t]] = c;
                }
            }
        }
    }
    let mut piv_row = vec![usize::MAX; nc];
    let mut r = 0usize;
    let mut ops = 0u64;
    for c in 0..nc {
        let Some(p) = (r..nr).find(|&i| mat[i * nc + c] % q != 0) else {
            continue;
        };
        if p != r {
            for j in 0..nc {
                mat.swap(p * nc + j, r * nc + j);
            }
        }
        let inv = fl.qinv(mat[r * nc + c] % q) as u64;
        for j in c..nc {
            mat[r * nc + j] = ((mat[r * nc + j] % q) as u64 * inv % q as u64) as u32;
        }
        let (head, tail) = mat.split_at_mut(r * nc);
        let (prow, tail) = tail.split_at_mut(nc);
        for row in head.chunks_mut(nc).chain(tail.chunks_mut(nc)) {
            let cf = row[c] % q;
            if cf == 0 {
                continue;
            }
            let m = q - cf;
            for j in c..nc {
                row[j] += m * prow[j];
            }
            ops += (nc - c) as u64;
        }
        piv_row[c] = r;
        r += 1;
        if r == nr {
            break;
        }
    }
    for x in mat.iter_mut() {
        *x %= q;
    }
    charge(ops);
    let const_col = nc - 1; // the constant monomial is smallest
    if piv_row[const_col] != usize::MAX {
        return SolveOut {
            sols: Vec::new(),
            status: Status::Inconsistent,
            dim: 0,
        };
    }
    let nonpiv: Vec<usize> = (0..nc).filter(|&c| piv_row[c] == usize::MAX).collect();
    let n = nonpiv.len();
    if std::env::var("DBG_NP").is_ok() {
        let mut h = vec![0usize; mac.d + 1];
        for &c in &nonpiv {
            h[deg(&mac.mons[c])] += 1;
        }
        eprintln!("D={} nonpivot degree histogram {:?}", mac.d, h);
    }
    if nonpiv.iter().any(|&c| deg(&mac.mons[c]) >= mac.d) {
        return SolveOut {
            sols: Vec::new(),
            status: Status::Insufficient,
            dim: n,
        };
    }
    let idx_in_n: HashMap<usize, usize> = nonpiv.iter().enumerate().map(|(i, &c)| (c, i)).collect();
    let mut mm: Vec<Vec<Vec<u32>>> = Vec::new();
    for var in 0..4 {
        let mut mx = vec![vec![0u32; n]; n];
        for (j, &cj) in nonpiv.iter().enumerate() {
            let mut e = mac.mons[cj];
            e[var] += 1;
            let c = mac.col_of[&pack(&e)];
            if piv_row[c] == usize::MAX {
                mx[idx_in_n[&c]][j] = 1;
            } else {
                let row = piv_row[c];
                for (t, &ct) in nonpiv.iter().enumerate() {
                    mx[t][j] = fl.qneg(mat[row * nc + ct]);
                }
            }
        }
        mm.push(mx);
    }
    let const_idx = idx_in_n[&const_col];
    let terms = term_exps();
    let mut sols: Vec<[u32; 4]> = Vec::new();
    let mut status = Status::Ok;
    for attempt in 0..4 {
        let mut coef = [0u32; 4];
        for cf in coef.iter_mut() {
            *lrng = lrng
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            *cf = 1 + ((*lrng >> 33) % (q as u64 - 1)) as u32;
        }
        let mut ml = vec![vec![0u32; n]; n];
        for var in 0..4 {
            for i in 0..n {
                for j in 0..n {
                    ml[i][j] = fl.qadd(ml[i][j], fl.qmul(coef[var], mm[var][i][j]));
                }
            }
        }
        let chi = charpoly(fl, &ml);
        let roots = rational_roots(fl, &chi);
        let mut found: Vec<[u32; 4]> = Vec::new();
        let mut ambiguous = false;
        for &lam in &roots {
            let mut bm = vec![vec![0u32; n]; n];
            for i in 0..n {
                for j in 0..n {
                    bm[i][j] = ml[j][i];
                }
                bm[i][i] = fl.qsub(bm[i][i], lam);
            }
            let ns = nullspace(fl, &bm);
            if ns.len() != 1 {
                ambiguous = true;
                continue;
            }
            let v = &ns[0];
            if v[const_idx] == 0 {
                continue; // solution at infinity
            }
            let vinv = fl.qinv(v[const_idx]);
            let mut z = [0u32; 4];
            for var in 0..4 {
                let mut acc = 0u32;
                for t in 0..n {
                    acc = fl.qadd(acc, fl.qmul(mm[var][t][const_idx], v[t]));
                }
                z[var] = fl.qmul(acc, vinv);
            }
            if (0..4).all(|k| eval_quad(fl, &eqs[k], &z, &terms) == 0) {
                found.push(z);
            }
        }
        if ambiguous && attempt < 3 {
            status = Status::Ambiguous;
            continue;
        }
        sols = found;
        if !ambiguous {
            status = Status::Ok;
        }
        break;
    }
    sols.sort();
    sols.dedup();
    SolveOut {
        sols,
        status,
        dim: n,
    }
}

/// Macaulay solves at increasing degree until the quotient closes.
pub fn solve_escalating(
    macs: &[Macaulay],
    fl: &Fld,
    eqs: &[[u32; 15]; 4],
    lrng: &mut u64,
) -> (SolveOut, usize) {
    let mut last = None;
    for (i, m) in macs.iter().enumerate() {
        let out = solve_quadrics(m, fl, eqs, lrng);
        if out.status != Status::Insufficient {
            return (out, i);
        }
        last = Some((out, i));
    }
    last.expect("at least one Macaulay degree")
}

// ------------------------------------------------------- relations / oracle

/// A decomposition R + sum mult_i * P_i = 0.
#[derive(Clone, Debug)]
pub struct Decomp {
    pub ab: (E, E),
    pub pts: Vec<(u32, E, u32)>, // (x, y, multiplicity)
}

/// Turn a solution z of the quadrics into a decomposition when the quartic
/// splits over F_q.  `squarefree_only` drops repeated roots.
pub fn decomposition_from_solution(
    cv: &Curve,
    r: &Div,
    w: &[E],
    z: &[u32; 4],
    squarefree_only: bool,
) -> Option<Decomp> {
    let fl = &cv.fld;
    let a = E { a: z[0], b: z[1] };
    let b = E { a: z[2], b: z[3] };
    if a == ZERO {
        return None;
    }
    let g = |p: &[E], i: usize| p.get(i).copied().unwrap_or(ZERO);
    let (u0, u1) = (g(&r.u, 0), g(&r.u, 1));
    let (v0, v1) = (g(&r.v, 0), g(&r.v, 1));
    let (w0, w1, w2, w3) = (g(w, 0), g(w, 1), g(w, 2), g(w, 3));
    let two = fl.embed(2);
    let aa = fl.mul(a, a);
    let bb = fl.mul(b, b);
    let ab = fl.mul(a, b);
    let c3 = fl.sub(fl.add(u1, fl.mul(two, b)), fl.mul(w3, aa));
    let c2 = fl.sub(
        fl.add(
            fl.add(bb, fl.mul(fl.mul(two, u1), b)),
            fl.add(u0, fl.mul(fl.mul(two, v1), a)),
        ),
        fl.mul(w2, aa),
    );
    let c1 = fl.sub(
        fl.add(
            fl.add(fl.mul(u1, bb), fl.mul(fl.mul(two, u0), b)),
            fl.add(fl.mul(fl.mul(two, v1), ab), fl.mul(fl.mul(two, v0), a)),
        ),
        fl.mul(w1, aa),
    );
    let c0 = fl.sub(
        fl.add(fl.mul(u0, bb), fl.mul(fl.mul(two, v0), ab)),
        fl.mul(w0, aa),
    );
    if c3.b != 0 || c2.b != 0 || c1.b != 0 || c0.b != 0 {
        return None;
    }
    let mut quartic = vec![c0.a, c1.a, c2.a, c3.a, 1u32];
    let mut roots: Vec<(u32, u32)> = Vec::new();
    for x in 0..fl.q {
        if quartic.len() <= 1 {
            break;
        }
        let mut m = 0;
        while quartic.len() > 1 && peval_q(fl, &quartic, x) == 0 {
            let n = quartic.len() - 1;
            let mut qt = vec![0u32; n];
            let mut carry = 0u32;
            for i in (0..n).rev() {
                carry = fl.qadd(quartic[i + 1], fl.qmul(carry, x));
                qt[i] = carry;
            }
            quartic = qt;
            m += 1;
        }
        if m > 0 {
            roots.push((x, m));
        }
    }
    charge(4 * fl.q as u64);
    if quartic.len() != 1 {
        return None;
    }
    if squarefree_only && roots.iter().any(|&(_, m)| m > 1) {
        return None;
    }
    let ainv = fl.inv(a);
    let mut pts = Vec::new();
    for (x, m) in roots {
        let xe = fl.embed(x);
        let s = fl.mul(fl.add(xe, b), ainv);
        let h = fl.add(fl.peval(&r.v, xe), fl.mul(fl.peval(&r.u, xe), s));
        if fl.mul(h, h) != cv.eval_f(xe) {
            return None;
        }
        pts.push((x, h, m));
    }
    Some(Decomp { ab: (a, b), pts })
}

/// Group-law check of a decomposition: R + sum mult_i P_i = O.
pub fn verify_decomp(cv: &Curve, r: &Div, d: &Decomp) -> bool {
    let mut acc = r.clone();
    for &(x, y, m) in &d.pts {
        let p = cv.point(cv.fld.embed(x), y);
        for _ in 0..m {
            acc = cv.add(&acc, &p);
        }
    }
    cv.is_identity(&acc)
}

pub struct Fb {
    pub pts: Vec<(u32, E)>,
    pub class_of: HashMap<(u32, u32, u32), (usize, i8)>,
    pub n_classes: usize,
}

pub fn build_fb(cv: &Curve) -> Fb {
    let fl = &cv.fld;
    let mut pts = Vec::new();
    let mut class_of = HashMap::new();
    let mut n_classes = 0usize;
    for x in 0..fl.q {
        let fx = cv.eval_f(fl.embed(x));
        let Some(y) = fl.sqrt(fx) else { continue };
        if y == ZERO {
            pts.push((x, ZERO));
            class_of.insert((x, 0, 0), (n_classes, 0));
            n_classes += 1;
            continue;
        }
        let ny = fl.neg(y);
        let (cy, oy) = if (y.a, y.b) < (ny.a, ny.b) {
            (y, ny)
        } else {
            (ny, y)
        };
        pts.push((x, cy));
        pts.push((x, oy));
        class_of.insert((x, cy.a, cy.b), (n_classes, 1));
        class_of.insert((x, oy.a, oy.b), (n_classes, -1));
        n_classes += 1;
    }
    charge(30 * fl.q as u64);
    Fb {
        pts,
        class_of,
        n_classes,
    }
}

/// Pair-hash oracle: all (a, b) such that four factor-base points with
/// distinct x lie on one line t = alpha x + beta in F_{q^2} (alpha != 0),
/// t = (y - v(x)) / u(x).
pub fn oracle(cv: &Curve, r: &Div, fb: &Fb) -> Vec<(E, E)> {
    let fl = &cv.fld;
    let mut pts: Vec<(u32, E)> = Vec::new();
    for &(x, y) in &fb.pts {
        let xe = fl.embed(x);
        let ux = fl.peval(&r.u, xe);
        if ux == ZERO {
            continue;
        }
        let t = fl.mul(fl.sub(y, fl.peval(&r.v, xe)), fl.inv(ux));
        pts.push((x, t));
    }
    let mut buckets: HashMap<(u32, u32, u32, u32), Vec<u32>> = HashMap::new();
    for i in 0..pts.len() {
        for j in i + 1..pts.len() {
            let (xi, ti) = pts[i];
            let (xj, tj) = pts[j];
            if xi == xj {
                continue;
            }
            let dx = fl.qinv(fl.qsub(xj, xi));
            let alpha = fl.scale(fl.sub(tj, ti), dx);
            let beta = fl.sub(ti, fl.scale(alpha, xi));
            let e = buckets
                .entry((alpha.a, alpha.b, beta.a, beta.b))
                .or_default();
            if !e.contains(&(i as u32)) {
                e.push(i as u32);
            }
            if !e.contains(&(j as u32)) {
                e.push(j as u32);
            }
        }
    }
    let mut out = Vec::new();
    for ((aa, ab, ba, bb), v) in buckets {
        let alpha = E { a: aa, b: ab };
        if alpha == ZERO || v.len() != 4 {
            continue;
        }
        let beta = E { a: ba, b: bb };
        let ainv = fl.inv(alpha);
        out.push((ainv, fl.mul(beta, ainv)));
    }
    out.sort_by_key(|&(a, b)| (a.a, a.b, b.a, b.b));
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::jac::Rng;

    #[test]
    fn grevlex_order_and_monomials() {
        let m = monomials_upto(5);
        assert_eq!(m.len(), 126);
        let mac = Macaulay::new(5);
        assert_eq!(mac.ncols, 126);
        assert_eq!(mac.nrows, 140);
        assert_eq!(deg(&mac.mons[mac.ncols - 1]), 0);
    }

    #[test]
    fn solver_matches_oracle_and_group_law() {
        let fld = Fld::new(31);
        let mut rng = Rng(77);
        let cv = loop {
            let f = Curve::random_f(&fld, &mut rng);
            let c = Curve::new(fld.clone(), f);
            if c.is_squarefree() {
                break c;
            }
        };
        let macs = [Macaulay::new(5), Macaulay::new(6), Macaulay::new(7)];
        let fb = build_fb(&cv);
        let mut lr = 12345u64;
        let (mut agree, mut found, mut tested, mut degenerate) = (0, 0, 0, 0);
        for _ in 0..400 {
            let r = cv.random_elt(&mut rng);
            let Some((eqs, w)) = equations(&cv, &r) else {
                continue;
            };
            let ux_root = (0..fld.q).any(|x| cv.fld.peval(&r.u, fld.embed(x)) == ZERO);
            if ux_root {
                continue;
            }
            tested += 1;
            let (out, _lvl) = solve_escalating(&macs, &fld, &eqs, &mut lr);
            if out.status == Status::Insufficient {
                degenerate += 1;
                tested -= 1;
                continue;
            }
            assert_eq!(out.status, Status::Ok);
            let mut sv: Vec<(E, E)> = Vec::new();
            for z in &out.sols {
                if let Some(d) = decomposition_from_solution(&cv, &r, &w, z, true) {
                    assert!(verify_decomp(&cv, &r, &d), "group-law check failed");
                    sv.push(d.ab);
                }
            }
            sv.sort_by_key(|&(a, b)| (a.a, a.b, b.a, b.b));
            let orc = oracle(&cv, &r, &fb);
            if sv == orc {
                agree += 1;
            } else {
                eprintln!(
                    "DISAGREE status={:?} dim={} nsols={} solver={:?} oracle={:?} u={:?} v={:?}",
                    out.status,
                    out.dim,
                    out.sols.len(),
                    sv,
                    orc,
                    r.u,
                    r.v
                );
            }
            found += orc.len();
        }
        assert_eq!(agree, tested, "solver and oracle disagree");
        assert!(found > 0, "no decomposition at all in the sample");
        assert!(
            degenerate * 10 < tested.max(1) + degenerate,
            "too many degenerate R"
        );
    }
}
