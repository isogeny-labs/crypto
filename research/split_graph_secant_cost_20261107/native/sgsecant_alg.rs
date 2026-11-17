//! Multivariate F_p polynomials (packed monomials, DRL), counted Buchberger
//! (normal selection strategy), FGLM to lex, univariate root finding and the
//! triangular rational-root extractor. No coordinate-cube assignment search.
use crate::ops::*;
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

pub type Mono = u64; // byte i = exponent of variable i (exponent < 128)
pub type Poly = Vec<(Mono, u64)>; // sorted by DRL, leading term first, nonzero coeffs

const HI: u64 = 0x8080808080808080;
#[inline]
pub fn deg(m: Mono) -> u32 {
    (m.wrapping_mul(0x0101010101010101) >> 56) as u32
}
#[inline]
pub fn divides(a: Mono, b: Mono) -> bool {
    ((b | HI).wrapping_sub(a)) & HI == HI
}
#[inline]
pub fn exp(m: Mono, i: usize) -> u32 {
    ((m >> (8 * i)) & 0xff) as u32
}
pub fn var(i: usize) -> Mono {
    1u64 << (8 * i)
}
pub fn drl(a: Mono, b: Mono, nv: usize) -> Ordering {
    let (da, db) = (deg(a), deg(b));
    if da != db {
        return da.cmp(&db);
    }
    for i in (0..nv).rev() {
        let (ea, eb) = (exp(a, i), exp(b, i));
        if ea != eb {
            return eb.cmp(&ea);
        }
    }
    Ordering::Equal
}
/// Key whose numeric order is lex order with variable 0 the largest.
#[inline]
pub fn lexkey(m: Mono) -> u64 {
    m.swap_bytes()
}
pub fn lcm(a: Mono, b: Mono, nv: usize) -> Mono {
    let mut r = 0;
    for i in 0..nv {
        r |= (exp(a, i).max(exp(b, i)) as u64) << (8 * i);
    }
    r
}
pub fn coprime(a: Mono, b: Mono, nv: usize) -> bool {
    (0..nv).all(|i| exp(a, i) == 0 || exp(b, i) == 0)
}

pub fn normalize(mut t: Vec<(Mono, u64)>, p: u64, nv: usize) -> Poly {
    t.sort_by(|a, b| drl(b.0, a.0, nv));
    let mut out: Poly = Vec::new();
    for (m, c) in t {
        let c = c % p;
        if let Some(l) = out.last_mut() {
            if l.0 == m {
                l.1 = (l.1 + c) % p;
                continue;
            }
        }
        out.push((m, c));
    }
    out.retain(|x| x.1 != 0);
    out
}

/// f - c * mono * g (merge). Charged: one mul per g term when c != 1, one add per
/// coincident term pair, native per monomial comparison.
pub fn sub_mul(f: &[(Mono, u64)], g: &[(Mono, u64)], c: u64, mono: Mono, p: u64, nv: usize) -> Poly {
    let mut out = Vec::with_capacity(f.len() + g.len());
    let (mut i, mut j) = (0, 0);
    if c != 1 {
        mul_n(g.len() as u64);
    }
    while i < f.len() || j < g.len() {
        if j == g.len() {
            out.push(f[i]);
            i += 1;
            continue;
        }
        let gm = g[j].0 + mono;
        let gc = if c == 1 { g[j].1 } else { g[j].1 * c % p };
        if i == f.len() {
            out.push((gm, if gc == 0 { 0 } else { p - gc }));
            j += 1;
            continue;
        }
        native_n(1);
        match drl(f[i].0, gm, nv) {
            Ordering::Greater => {
                out.push(f[i]);
                i += 1;
            }
            Ordering::Less => {
                out.push((gm, if gc == 0 { 0 } else { p - gc }));
                j += 1;
            }
            Ordering::Equal => {
                add_n(1);
                let v = (f[i].1 + p - gc) % p;
                if v != 0 {
                    out.push((f[i].0, v));
                }
                i += 1;
                j += 1;
            }
        }
    }
    out.retain(|x| x.1 != 0);
    out
}
pub fn shift(g: &[(Mono, u64)], m: Mono) -> Poly {
    g.iter().map(|&(a, c)| (a + m, c)).collect()
}
pub fn make_monic(mut f: Poly, p: u64) -> Poly {
    if f.is_empty() || f[0].1 == 1 {
        return f;
    }
    let iv = finv(f[0].1, p);
    for t in f.iter_mut() {
        t.1 = fmul(t.1, iv, p);
    }
    f
}

#[derive(Default, Clone, Debug)]
pub struct Gb {
    pub polys: Vec<Poly>, // monic, reduced, sorted by LT (DRL ascending)
    pub handled_degree: u32,
    pub input_degree: u32,
    pub pairs_total: u64,
    pub pairs_processed: u64,
    pub zero_reductions: u64,
    pub max_terms: usize,
    pub inconsistent: bool,
}
impl Gb {
    pub fn lts(&self) -> Vec<Mono> {
        self.polys.iter().map(|f| f[0].0).collect()
    }
}

pub fn reduce(f: Poly, g: &[Poly], lts: &[Mono], p: u64, nv: usize) -> Poly {
    let mut f = f;
    let mut done: Poly = Vec::new();
    while !f.is_empty() {
        let (lt, c) = f[0];
        let mut hit = None;
        for (k, l) in lts.iter().enumerate() {
            native_n(1);
            if divides(*l, lt) {
                hit = Some(k);
                break;
            }
        }
        match hit {
            Some(k) => {
                f = sub_mul(&f, &g[k], c, lt - lts[k], p, nv);
            }
            None => {
                done.push(f[0]);
                f.remove(0);
            }
        }
    }
    done
}

/// Buchberger, normal strategy, product + chain criteria. Err(()) = budget hit.
pub fn groebner(gens: &[Poly], p: u64, nv: usize) -> Result<Gb, ()> {
    let mut out = Gb::default();
    let mut g: Vec<Poly> = Vec::new();
    let mut lts: Vec<Mono> = Vec::new();
    let mut pending: BTreeMap<(u32, usize, usize), Mono> = BTreeMap::new(); // key (lcm deg, ...) ordering refined below
    let mut pend: Vec<(usize, usize, Mono)> = Vec::new();
    let mut pend_set: HashSet<(usize, usize)> = HashSet::new();
    let _ = &mut pending;
    for f in gens {
        if let Some(l) = f.first() {
            out.input_degree = out.input_degree.max(deg(l.0));
        }
    }
    let mut queue: Vec<Poly> = gens.to_vec();
    queue.sort_by_key(|f| f.first().map(|l| deg(l.0)).unwrap_or(0));
    let mut qi = 0usize;
    // add inputs one by one (reduced against current basis)
    let mut add_poly = |f: Poly,
                        g: &mut Vec<Poly>,
                        lts: &mut Vec<Mono>,
                        pend: &mut Vec<(usize, usize, Mono)>,
                        pend_set: &mut HashSet<(usize, usize)>,
                        total_pairs: &mut u64|
     -> bool {
        let r = reduce(f, g, lts, p, nv);
        if r.is_empty() {
            return false;
        }
        let r = make_monic(r, p);
        let k = g.len();
        for (i, l) in lts.iter().enumerate() {
            let lc = lcm(*l, r[0].0, nv);
            pend.push((i, k, lc));
            pend_set.insert((i, k));
            *total_pairs += 1;
        }
        lts.push(r[0].0);
        g.push(r);
        true
    };
    while qi < queue.len() {
        let f = queue[qi].clone();
        qi += 1;
        let before = g.len();
        add_poly(f, &mut g, &mut lts, &mut pend, &mut pend_set, &mut out.pairs_total);
        if g.len() > before && g[before].len() == 1 && g[before][0].0 == 0 {
            out.inconsistent = true;
            out.polys = vec![g[before].clone()];
            return Ok(out);
        }
    }
    while !pend.is_empty() {
        if over_budget() {
            return Err(());
        }
        // normal strategy: smallest lcm degree, then smallest lcm in DRL
        let mut best = 0;
        for (idx, e) in pend.iter().enumerate() {
            native_n(1);
            let c = drl(e.2, pend[best].2, nv);
            if c == Ordering::Less {
                best = idx;
            }
        }
        let (i, j, lc) = pend.swap_remove(best);
        pend_set.remove(&(i, j));
        // product criterion
        native_n(1);
        if coprime(lts[i], lts[j], nv) {
            continue;
        }
        // chain criterion: some k with LT_k | lcm and pairs (i,k),(j,k) no longer pending
        let mut chain = false;
        for k in 0..g.len() {
            if k == i || k == j {
                continue;
            }
            native_n(1);
            if divides(lts[k], lc) {
                let a = (i.min(k), i.max(k));
                let b = (j.min(k), j.max(k));
                native_n(2);
                if !pend_set.contains(&a) && !pend_set.contains(&b) {
                    chain = true;
                    break;
                }
            }
        }
        if chain {
            continue;
        }
        out.pairs_processed += 1;
        out.handled_degree = out.handled_degree.max(deg(lc));
        let s = sub_mul(&shift(&g[i], lc - lts[i]), &g[j], 1, lc - lts[j], p, nv);
        let r = reduce(s, &g, &lts, p, nv);
        if r.is_empty() {
            out.zero_reductions += 1;
            continue;
        }
        let r = make_monic(r, p);
        out.max_terms = out.max_terms.max(r.len());
        let k = g.len();
        if r.len() == 1 && r[0].0 == 0 {
            out.inconsistent = true;
            out.polys = vec![r];
            return Ok(out);
        }
        for (ii, l) in lts.iter().enumerate() {
            let lc2 = lcm(*l, r[0].0, nv);
            pend.push((ii, k, lc2));
            pend_set.insert((ii, k));
            out.pairs_total += 1;
        }
        lts.push(r[0].0);
        g.push(r);
    }
    // minimalize
    let n = g.len();
    let mut keep = vec![true; n];
    for a in 0..n {
        for b in 0..n {
            if a != b && keep[b] {
                native_n(1);
                if divides(lts[b], lts[a]) && (lts[a] != lts[b] || b < a) {
                    keep[a] = false;
                    break;
                }
            }
        }
    }
    let mut min: Vec<Poly> = (0..n).filter(|&i| keep[i]).map(|i| g[i].clone()).collect();
    // tail-reduce each against the others
    let mlts: Vec<Mono> = min.iter().map(|f| f[0].0).collect();
    for a in 0..min.len() {
        let others: Vec<usize> = (0..min.len()).filter(|&b| b != a).collect();
        let og: Vec<Poly> = others.iter().map(|&b| min[b].clone()).collect();
        let ol: Vec<Mono> = others.iter().map(|&b| mlts[b]).collect();
        let head = min[a][0];
        let tail = min[a][1..].to_vec();
        let rt = reduce(tail, &og, &ol, p, nv);
        let mut f = vec![head];
        f.extend(rt);
        min[a] = f;
    }
    min.sort_by(|a, b| drl(a[0].0, b[0].0, nv));
    out.polys = min;
    Ok(out)
}

// ---------------------------------------------------------------- FGLM
/// Standard monomials of a zero-dimensional DRL basis, in DRL order.
pub fn staircase(lts: &[Mono], nv: usize) -> Option<Vec<Mono>> {
    let mut bound = vec![u32::MAX; nv];
    for l in lts {
        for i in 0..nv {
            if *l == (1u64 << (8 * i)) * exp(*l, i) as u64 && exp(*l, i) > 0 {
                bound[i] = bound[i].min(exp(*l, i));
            }
        }
    }
    if bound.iter().any(|b| *b == u32::MAX) {
        return None;
    }
    let mut out = Vec::new();
    let mut stack = vec![(0usize, 0u64)];
    fn rec(i: usize, m: u64, nv: usize, bound: &[u32], lts: &[Mono], out: &mut Vec<Mono>) {
        if i == nv {
            native_n(lts.len() as u64);
            if !lts.iter().any(|l| divides(*l, m)) {
                out.push(m);
            }
            return;
        }
        for e in 0..bound[i] {
            rec(i + 1, m + ((e as u64) << (8 * i)), nv, bound, lts, out);
        }
    }
    let _ = &mut stack;
    rec(0, 0, nv, &bound, lts, &mut out);
    out.sort_by(|a, b| drl(*a, *b, nv));
    Some(out)
}

pub struct LexElem {
    pub lead: Mono,
    pub terms: Vec<(Mono, u64)>, // lead first (coef 1), then tail (any order)
}

/// FGLM: lex Groebner basis (x0 > x1 > ...) from a zero-dimensional DRL basis.
pub fn fglm(gb: &Gb, p: u64, nv: usize) -> Option<(Vec<LexElem>, usize)> {
    let lts = gb.lts();
    let std = staircase(&lts, nv)?;
    let d = std.len();
    if d == 0 {
        return Some((vec![], 0));
    }
    let idx: HashMap<Mono, usize> = std.iter().enumerate().map(|(i, m)| (*m, i)).collect();
    // multiplication matrices: column s of M_v = NF(std[s] * x_v) as dense vec of length d
    let mut mats: Vec<Vec<Vec<(usize, u64)>>> = Vec::new();
    for v in 0..nv {
        let mut cols = Vec::with_capacity(d);
        for s in 0..d {
            let t = std[s] + var(v);
            native_n(1);
            if let Some(&k) = idx.get(&t) {
                cols.push(vec![(k, 1u64)]);
            } else {
                let r = reduce(vec![(t, 1)], &gb.polys, &lts, p, nv);
                cols.push(r.iter().map(|(m, c)| (idx[m], *c)).collect());
            }
        }
        mats.push(cols);
    }
    // enumerate monomials in increasing lex order
    let mut cand: BTreeSet<u64> = BTreeSet::new();
    let mut parent: HashMap<Mono, (Mono, usize)> = HashMap::new();
    cand.insert(lexkey(0));
    let mut stairs: Vec<Mono> = Vec::new();
    let mut svec: HashMap<Mono, Vec<u64>> = HashMap::new(); // normal form vector of staircase monomials
    // echelon rows: (pivot, vec, combo over stairs)
    let mut rows: Vec<(usize, Vec<u64>, Vec<u64>)> = Vec::new();
    let mut lexlts: Vec<Mono> = Vec::new();
    let mut elems: Vec<LexElem> = Vec::new();
    while let Some(k) = cand.pop_first() {
        let t = k.swap_bytes();
        native_n(lexlts.len() as u64);
        if lexlts.iter().any(|l| divides(*l, t)) {
            continue;
        }
        // vector of NF(t)
        let v: Vec<u64> = if t == 0 {
            let mut e = vec![0u64; d];
            e[idx[&0]] = 1;
            e
        } else {
            let (par, vi) = parent[&t];
            let pv = &svec[&par];
            let mut out = vec![0u64; d];
            for (s, &c) in pv.iter().enumerate() {
                if c == 0 {
                    continue;
                }
                for &(r, mc) in &mats[vi][s] {
                    out[r] = fadd(out[r], fmul(c, mc, p), p);
                }
            }
            out
        };
        // reduce against echelon rows
        let n_st = stairs.len();
        let mut w = v.clone();
        let mut combo = vec![0u64; n_st + 1];
        combo[n_st] = 1;
        for (piv, rv, rc) in rows.iter() {
            let c = w[*piv];
            if c == 0 {
                continue;
            }
            for i in 0..d {
                let x = fmul(c, rv[i], p);
                w[i] = fsub(w[i], x, p);
            }
            for i in 0..rc.len() {
                let x = fmul(c, rc[i], p);
                combo[i] = fsub(combo[i], x, p);
            }
        }
        if let Some(piv) = w.iter().position(|x| *x != 0) {
            // independent: accept
            let iv = finv(w[piv], p);
            for x in w.iter_mut() {
                *x = fmul(*x, iv, p);
            }
            for x in combo.iter_mut() {
                *x = fmul(*x, iv, p);
            }
            rows.push((piv, w, combo));
            svec.insert(t, v);
            stairs.push(t);
            for vi in 0..nv {
                let ch = t + var(vi);
                if exp(ch, vi) < 127 && !parent.contains_key(&ch) {
                    parent.insert(ch, (t, vi));
                    cand.insert(lexkey(ch));
                }
            }
        } else {
            // dependency: t + sum combo[k] stairs[k] in I
            let mut terms = vec![(t, 1u64)];
            for (kk, c) in combo.iter().take(n_st).enumerate() {
                if *c != 0 {
                    terms.push((stairs[kk], *c));
                }
            }
            lexlts.push(t);
            elems.push(LexElem { lead: t, terms });
        }
        if stairs.len() > d + 1 {
            return None;
        }
    }
    Some((elems, d))
}

// ------------------------------------------------- univariate F_p[t]
pub type Up = Vec<u64>; // low to high, trimmed
fn trim(mut a: Up) -> Up {
    while a.last() == Some(&0) {
        a.pop();
    }
    a
}
fn u_mulmod(a: &Up, b: &Up, f: &Up, p: u64) -> Up {
    if a.is_empty() || b.is_empty() {
        return vec![];
    }
    let mut r = vec![0u64; a.len() + b.len() - 1];
    for (i, x) in a.iter().enumerate() {
        for (j, y) in b.iter().enumerate() {
            let t = fmul(*x, *y, p);
            r[i + j] = fadd(r[i + j], t, p);
        }
    }
    u_rem(trim(r), f, p)
}
/// remainder of a by monic-or-not f (f nonzero)
fn u_rem(a: Up, f: &Up, p: u64) -> Up {
    u_divrem(a, f, p).1
}
fn u_divrem(a: Up, f: &Up, p: u64) -> (Up, Up) {
    let df = f.len() - 1;
    let mut r = a;
    if r.len() <= df {
        return (vec![], trim(r));
    }
    let lcinv = if f[df] == 1 { 1 } else { finv(f[df], p) };
    let mut q = vec![0u64; r.len() - df];
    while r.len() > df && !r.is_empty() {
        let k = r.len() - 1 - df;
        let c = if lcinv == 1 { r[r.len() - 1] } else { fmul(r[r.len() - 1], lcinv, p) };
        q[k] = c;
        if c != 0 {
            for i in 0..=df {
                let t = fmul(c, f[i], p);
                let ri = k + i;
                r[ri] = fsub(r[ri], t, p);
            }
        }
        r.pop();
        r = trim(r);
        if r.len() <= df {
            break;
        }
    }
    (trim(q), trim(r))
}
fn u_gcd(a: Up, b: Up, p: u64) -> Up {
    let (mut a, mut b) = (trim(a), trim(b));
    while !b.is_empty() {
        let r = u_rem(a, &b, p);
        a = b;
        b = r;
    }
    if a.is_empty() {
        return a;
    }
    let iv = finv(*a.last().unwrap(), p);
    a.iter().map(|c| fmul(*c, iv, p)).collect()
}
fn u_powx(e: u64, f: &Up, p: u64) -> Up {
    // x^e mod f
    let mut base = u_rem(vec![0, 1], f, p);
    let mut r: Up = vec![1];
    let mut e = e;
    while e > 0 {
        if e & 1 == 1 {
            r = u_mulmod(&r, &base, f, p);
        }
        e >>= 1;
        if e > 0 {
            base = u_mulmod(&base, &base, f, p);
        }
    }
    r
}
fn u_powmod(b: &Up, e: u64, f: &Up, p: u64) -> Up {
    let mut base = u_rem(b.clone(), f, p);
    let mut r: Up = vec![1];
    let mut e = e;
    while e > 0 {
        if e & 1 == 1 {
            r = u_mulmod(&r, &base, f, p);
        }
        e >>= 1;
        if e > 0 {
            base = u_mulmod(&base, &base, f, p);
        }
    }
    r
}
/// All distinct roots in F_p of f (p odd prime).
pub fn roots_fp(f: &Up, p: u64) -> Vec<u64> {
    let f = trim(f.clone());
    if f.len() <= 1 {
        return vec![];
    }
    if f.len() == 2 {
        let iv = finv(f[1], p);
        return vec![fmul(fneg(f[0], p), iv, p)];
    }
    // g = gcd(f, x^p - x)
    let mut xp = u_powx(p, &f, p);
    // x^p - x
    if xp.len() < 2 {
        xp.resize(2, 0);
    }
    xp[1] = fsub(xp[1], 1, p);
    let xp = trim(xp);
    let g = u_gcd(f.clone(), xp, p);
    let mut out = Vec::new();
    split_linear(g, p, &mut out, 1);
    out.sort_unstable();
    out
}
fn split_linear(g: Up, p: u64, out: &mut Vec<u64>, a0: u64) {
    if g.len() <= 1 {
        return;
    }
    if g.len() == 2 {
        let iv = finv(g[1], p);
        out.push(fmul(fneg(g[0], p), iv, p));
        return;
    }
    let mut a = a0;
    loop {
        // h = (x+a)^((p-1)/2) - 1 mod g
        let base: Up = if a % p == 0 { vec![0, 1] } else { vec![a % p, 1] };
        let mut h = u_powmod(&base, (p - 1) / 2, &g, p);
        if h.is_empty() {
            h = vec![0];
        }
        h[0] = fsub(h[0], 1, p);
        let h = trim(h);
        let d = u_gcd(g.clone(), h, p);
        if d.len() > 1 && d.len() < g.len() {
            let (q, _) = u_divrem(g.clone(), &d, p);
            split_linear(d, p, out, a + 1);
            split_linear(q, p, out, a + 1);
            return;
        }
        a += 1;
        if a > a0 + 4 * p {
            // cannot happen for squarefree split g; safety against infinite loop
            return;
        }
    }
}

// ---------------------------------------------------- triangular extraction
fn eval_mono_spec(m: Mono, vals: &[Option<(u64, Vec<u64>)>], keep: usize) -> Option<(u64, u32)> {
    // returns (product of assigned powers, exponent of `keep`) ; uses power tables
    let mut acc = 1u64;
    let _ = (m, vals, keep);
    let _ = &mut acc;
    None
}

/// Rational points of a lex Groebner basis by triangular back-substitution.
/// Returns (points, fallback_levels, unresolved) .
pub fn triangular_roots(elems: &[LexElem], p: u64, nv: usize) -> (Vec<Vec<u64>>, u32, bool) {
    let _ = eval_mono_spec;
    let mut pts = Vec::new();
    let mut fallbacks = 0u32;
    let mut unresolved = false;
    let mut vals = vec![0u64; nv];
    fn rec(
        level: isize,
        nv: usize,
        elems: &[LexElem],
        p: u64,
        vals: &mut Vec<u64>,
        pts: &mut Vec<Vec<u64>>,
        fb: &mut u32,
        unres: &mut bool,
    ) {
        if level < 0 {
            pts.push(vals.clone());
            return;
        }
        let l = level as usize;
        // power tables for assigned variables (> l)
        let mut pw: Vec<Vec<u64>> = vec![vec![]; nv];
        for v in (l + 1)..nv {
            let maxe = elems
                .iter()
                .flat_map(|e| e.terms.iter().map(move |t| exp(t.0, v)))
                .max()
                .unwrap_or(0) as usize;
            let mut t = vec![1u64; maxe + 1];
            for k in 1..=maxe {
                t[k] = fmul(t[k - 1], vals[v], p);
            }
            pw[v] = t;
        }
        let mut polys: Vec<Up> = Vec::new();
        for e in elems {
            // only elements free of variables < l
            native_n(e.terms.len() as u64);
            if e.terms.iter().any(|t| (0..l).any(|v| exp(t.0, v) > 0)) {
                continue;
            }
            let mut u: Up = vec![0u64; 1];
            for &(m, c) in &e.terms {
                let mut coef = c;
                for v in (l + 1)..nv {
                    let ex = exp(m, v) as usize;
                    if ex > 0 {
                        coef = fmul(coef, pw[v][ex], p);
                    }
                }
                let ex = exp(m, l) as usize;
                if u.len() <= ex {
                    u.resize(ex + 1, 0);
                }
                u[ex] = fadd(u[ex], coef, p);
            }
            let u = trim(u);
            if u.is_empty() {
                continue;
            }
            if u.len() == 1 {
                return; // nonzero constant: no point above this partial assignment
            }
            polys.push(u);
        }
        let cands: Vec<u64> = if polys.is_empty() {
            *fb += 1;
            if *fb > 64 {
                *unres = true;
                return;
            }
            (0..p).collect()
        } else {
            let mut g = polys[0].clone();
            for q in polys.iter().skip(1) {
                g = u_gcd(g, q.clone(), p);
                if g.len() <= 1 {
                    break;
                }
            }
            if g.len() <= 1 {
                return;
            }
            roots_fp(&g, p)
        };
        for r in cands {
            vals[l] = r;
            rec(level - 1, nv, elems, p, vals, pts, fb, unres);
        }
    }
    rec(nv as isize - 1, nv, elems, p, &mut vals, &mut pts, &mut fallbacks, &mut unresolved);
    (pts, fallbacks, unresolved)
}

pub fn eval_poly(f: &Poly, pt: &[u64], p: u64, nv: usize) -> u64 {
    let mut acc = 0;
    for &(m, c) in f {
        let mut t = c;
        for v in 0..nv {
            let e = exp(m, v);
            if e > 0 {
                t = fmul(t, fpow(pt[v], e as u64, p), p);
            }
        }
        acc = fadd(acc, t, p);
    }
    acc
}

#[derive(Debug, Clone, Default)]
pub struct Extract {
    pub points: Vec<Vec<u64>>,
    pub censored: bool,
    pub not_zero_dim: bool,
    pub inconsistent: bool,
    pub handled_degree: u32,
    pub input_degree: u32,
    pub dim: usize,
    pub pairs: u64,
    pub fallbacks: u32,
    pub unresolved: bool,
}

/// Full extractor: DRL basis -> FGLM lex -> triangular rational roots -> check
/// every candidate against the input generators.
pub fn extract(gens: &[Poly], p: u64, nv: usize) -> Extract {
    let mut ex = Extract::default();
    let save = phase(PH_GB);
    let gb = match groebner(gens, p, nv) {
        Ok(g) => g,
        Err(()) => {
            ex.censored = true;
            phase(save);
            return ex;
        }
    };
    ex.handled_degree = gb.handled_degree;
    ex.input_degree = gb.input_degree;
    ex.pairs = gb.pairs_processed;
    if gb.inconsistent {
        ex.inconsistent = true;
        phase(save);
        return ex;
    }
    phase(PH_FGLM);
    let fg = fglm(&gb, p, nv);
    let (elems, dim) = match fg {
        Some(x) => x,
        None => {
            ex.not_zero_dim = true;
            phase(save);
            return ex;
        }
    };
    ex.dim = dim;
    phase(PH_ROOTS);
    let (cand, fb, unres) = triangular_roots(&elems, p, nv);
    ex.fallbacks = fb;
    ex.unresolved = unres;
    phase(PH_VERIFY);
    let mut pts = Vec::new();
    for c in cand {
        if gens.iter().all(|g| eval_poly(g, &c, p, nv) == 0) {
            pts.push(c);
        }
    }
    pts.sort();
    pts.dedup();
    ex.points = pts;
    phase(save);
    ex
}
