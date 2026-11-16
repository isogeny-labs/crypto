//! Isogeny-kernel vs mu_k (Petit-style) factor bases for m = 2 point
//! decomposition over prime fields. Protocol: ../PROTOCOL.md (frozen before data).
//!
//! Q1  exact decomposition yield |D| for every arm, by enumeration of all pairs.
//! Q2  counted Buchberger work on the decomposition systems, four presentations.
//! Q3  exact terminal/fibre check of the Velu chains (radical or not).
//! S1  structure-constant density of the chain quotient algebra.
//!
//! Everything except the `wall_us` column is a deterministic counted quantity.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::fmt::Write as _;
use std::time::Instant;

// ------------------------------------------------------------------ rng
struct Rng(u64);
impl Rng {
    fn new(seed: u64) -> Self { Rng(seed) }
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }
    fn below(&mut self, n: u64) -> u64 { self.next() % n }
}
fn derive(master: u64, label: &str) -> u64 {
    // FNV-1a of the label, mixed with the master seed.
    let mut h: u64 = 0xcbf29ce484222325;
    for b in label.bytes() { h ^= b as u64; h = h.wrapping_mul(0x100000001b3); }
    let mut r = Rng::new(master ^ h);
    r.next()
}

// ------------------------------------------------------------------ field
#[derive(Clone)]
struct F { p: u64, sqrt: Vec<i64> }
impl F {
    fn new(p: u64) -> Self {
        let mut sqrt = vec![-1i64; p as usize];
        for y in 0..p { let s = (y * y % p) as usize; if sqrt[s] < 0 { sqrt[s] = y as i64; } }
        F { p, sqrt }
    }
    fn add(&self, a: u64, b: u64) -> u64 { (a + b) % self.p }
    fn sub(&self, a: u64, b: u64) -> u64 { (a + self.p - b % self.p) % self.p }
    fn mul(&self, a: u64, b: u64) -> u64 { a * b % self.p }
    fn neg(&self, a: u64) -> u64 { (self.p - a % self.p) % self.p }
    fn pow(&self, mut a: u64, mut e: u64) -> u64 {
        let mut r = 1; a %= self.p;
        while e > 0 { if e & 1 == 1 { r = self.mul(r, a); } a = self.mul(a, a); e >>= 1; }
        r
    }
    fn inv(&self, a: u64) -> u64 { assert!(a % self.p != 0, "inverse of 0"); self.pow(a, self.p - 2) }
    fn sq_root(&self, a: u64) -> Option<u64> { let s = self.sqrt[(a % self.p) as usize]; if s < 0 { None } else { Some(s as u64) } }
    fn primitive_root(&self) -> u64 {
        let n = self.p - 1; let fs = factor(n);
        (2..self.p).find(|&g| fs.iter().all(|&(q, _)| self.pow(g, n / q) != 1)).unwrap()
    }
}
fn factor(mut n: u64) -> Vec<(u64, u32)> {
    let mut v = vec![]; let mut d = 2;
    while d * d <= n { let mut e = 0; while n % d == 0 { n /= d; e += 1; } if e > 0 { v.push((d, e)); } d += 1; }
    if n > 1 { v.push((n, 1)); }
    v
}
fn v2(mut n: u64) -> u32 { let mut e = 0; while n % 2 == 0 { n /= 2; e += 1; } e }

// ------------------------------------------------------------------ curves
type Pt = Option<(u64, u64)>;
#[derive(Clone, Copy, Debug)]
struct Ec { a: u64, b: u64 }
impl Ec {
    fn rhs(&self, f: &F, x: u64) -> u64 { f.add(f.add(f.mul(f.mul(x, x), x), f.mul(self.a, x)), self.b) }
    fn on(&self, f: &F, p: Pt) -> bool { match p { None => true, Some((x, y)) => f.mul(y, y) == self.rhs(f, x) } }
    fn add(&self, f: &F, p: Pt, q: Pt) -> Pt {
        let (x1, y1) = match p { None => return q, Some(v) => v };
        let (x2, y2) = match q { None => return p, Some(v) => v };
        let l = if x1 == x2 {
            if f.add(y1, y2) == 0 { return None; }
            f.mul(f.add(f.mul(3, f.mul(x1, x1)), self.a), f.inv(f.mul(2, y1)))
        } else { f.mul(f.sub(y2, y1), f.inv(f.sub(x2, x1))) };
        let x3 = f.sub(f.sub(f.mul(l, l), x1), x2);
        let y3 = f.sub(f.mul(l, f.sub(x1, x3)), y1);
        Some((x3, y3))
    }
    fn mul(&self, f: &F, mut n: u64, p: Pt) -> Pt {
        let mut r = None; let mut b = p;
        while n > 0 { if n & 1 == 1 { r = self.add(f, r, b); } b = self.add(f, b, b); n >>= 1; }
        r
    }
    fn points(&self, f: &F) -> Vec<(u64, u64)> {
        let mut v = vec![];
        for x in 0..f.p { let r = self.rhs(f, x); if r == 0 { v.push((x, 0)); } else if let Some(s) = f.sq_root(r) { v.push((x, s)); v.push((x, f.p - s)); } }
        v
    }
    fn order(&self, f: &F) -> u64 { self.points(f).len() as u64 + 1 }
    fn nonsingular(&self, f: &F) -> bool { f.add(f.mul(4, f.pow(self.a, 3)), f.mul(27, f.mul(self.b, self.b))) != 0 }
}
fn pt_order(e: &Ec, f: &F, p: Pt, n: u64) -> u64 {
    // order of p given that n*p = O
    let mut o = n;
    for (q, _) in factor(n) { while o % q == 0 && e.mul(f, o / q, p).is_none() { o /= q; } }
    o
}

// ------------------------------------------------------------------ polynomials
const NV: usize = 16;
type Mono = [u8; NV];
type Key = [u16; NV + 1];
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Ord_ { Grevlex, Lex }
fn key(m: &Mono, ord: Ord_, n: usize) -> Key {
    let mut k = [0u16; NV + 1];
    match ord {
        Ord_::Lex => { for i in 0..n { k[i] = m[i] as u16; } }
        Ord_::Grevlex => {
            k[0] = m[..n].iter().map(|&e| e as u16).sum();
            for i in 0..n { k[1 + i] = 255 - m[n - 1 - i] as u16; }
        }
    }
    k
}
fn mdeg(m: &Mono) -> u32 { m.iter().map(|&e| e as u32).sum() }
fn mmul(a: &Mono, b: &Mono) -> Option<Mono> {
    let mut r = [0u8; NV];
    for i in 0..NV { let s = a[i] as u16 + b[i] as u16; if s > 250 { return None; } r[i] = s as u8; }
    Some(r)
}
fn mdiv(a: &Mono, b: &Mono) -> Option<Mono> {
    let mut r = [0u8; NV];
    for i in 0..NV { if a[i] < b[i] { return None; } r[i] = a[i] - b[i]; }
    Some(r)
}
fn mlcm(a: &Mono, b: &Mono) -> Mono { let mut r = [0u8; NV]; for i in 0..NV { r[i] = a[i].max(b[i]); } r }
fn coprime(a: &Mono, b: &Mono) -> bool { (0..NV).all(|i| a[i] == 0 || b[i] == 0) }

/// Polynomial during construction: unordered map monomial -> coefficient.
#[derive(Clone, Default, Debug)]
struct P { t: BTreeMap<Mono, u64> }
impl P {
    fn c(f: &F, c: u64) -> P { let mut p = P::default(); if c % f.p != 0 { p.t.insert([0; NV], c % f.p); } p }
    fn var(i: usize) -> P { let mut m = [0u8; NV]; m[i] = 1; let mut p = P::default(); p.t.insert(m, 1); p }
    fn add(&self, f: &F, o: &P) -> P {
        let mut r = self.clone();
        for (m, c) in &o.t { let e = r.t.entry(*m).or_insert(0); *e = f.add(*e, *c); if *e == 0 { r.t.remove(m); } }
        r
    }
    fn scale(&self, f: &F, c: u64) -> P { let mut r = P::default(); for (m, v) in &self.t { let w = f.mul(*v, c % f.p); if w != 0 { r.t.insert(*m, w); } } r }
    fn sub(&self, f: &F, o: &P) -> P { self.add(f, &o.scale(f, f.p - 1)) }
    fn mul(&self, f: &F, o: &P) -> P {
        let mut r = P::default();
        for (m1, c1) in &self.t { for (m2, c2) in &o.t {
            let m = mmul(m1, m2).expect("degree overflow in construction");
            let e = r.t.entry(m).or_insert(0); *e = f.add(*e, f.mul(*c1, *c2)); if *e == 0 { r.t.remove(&m); }
        } }
        r
    }
    fn pow(&self, f: &F, e: u32) -> P { let mut r = P::c(f, 1); for _ in 0..e { r = r.mul(f, self); } r }
    fn eval(&self, f: &F, x: &[u64]) -> u64 {
        let mut s = 0;
        for (m, c) in &self.t { let mut t = *c; for i in 0..x.len() { if m[i] > 0 { t = f.mul(t, f.pow(x[i], m[i] as u64)); } } s = f.add(s, t); }
        s
    }
}
/// Ordered, monic polynomial used by the Groebner engine: terms in decreasing order.
#[derive(Clone, Debug)]
struct Q { t: Vec<(Mono, u64)> }
fn to_q(f: &F, p: &P, ord: Ord_, n: usize) -> Option<Q> {
    if p.t.is_empty() { return None; }
    let mut t: Vec<(Mono, u64)> = p.t.iter().map(|(m, c)| (*m, *c)).collect();
    t.sort_by(|a, b| key(&b.0, ord, n).cmp(&key(&a.0, ord, n)));
    let li = f.inv(t[0].1);
    for x in t.iter_mut() { x.1 = f.mul(x.1, li); }
    Some(Q { t })
}
fn q_to_p(q: &Q) -> P { let mut p = P::default(); for (m, c) in &q.t { p.t.insert(*m, *c); } p }

#[derive(Clone, Debug, Default)]
struct GbStats {
    status: String, // "consistent", "inconsistent", "censored_budget", "censored_degree"
    termops: u64, spairs: u64, crit1: u64, crit2: u64, zero_red: u64, basis: u64, max_deg: u32, wall_us: u128,
}
enum RedErr { Budget, Degree }

/// Full reduction of f modulo g (each element of g monic). `ops` counts one per
/// coefficient multiply-subtract on a reductor term.
fn reduce(f: &F, terms: Vec<(Mono, u64)>, g: &[Q], ord: Ord_, n: usize, ops: &mut u64, budget: u64) -> Result<Vec<(Mono, u64)>, RedErr> {
    let mut h: BTreeMap<Key, (Mono, u64)> = BTreeMap::new();
    for (m, c) in terms { if c != 0 { let k = key(&m, ord, n); let e = h.entry(k).or_insert((m, 0)); e.1 = f.add(e.1, c); if e.1 == 0 { h.remove(&k); } } }
    let mut rem: Vec<(Mono, u64)> = vec![];
    while let Some((_, (m, c))) = h.pop_last() {
        let mut done = false;
        for gi in g {
            if let Some(q) = mdiv(&m, &gi.t[0].0) {
                for (gm, gc) in &gi.t[1..] {
                    let mm = mmul(gm, &q).ok_or(RedErr::Degree)?;
                    let k = key(&mm, ord, n);
                    let e = h.entry(k).or_insert((mm, 0));
                    e.1 = f.sub(e.1, f.mul(c, *gc));
                    if e.1 == 0 { h.remove(&k); }
                }
                *ops += gi.t.len() as u64;
                if *ops > budget { return Err(RedErr::Budget); }
                done = true; break;
            }
        }
        if !done { rem.push((m, c)); }
    }
    Ok(rem)
}
fn is_const(t: &[(Mono, u64)]) -> bool { t.len() == 1 && mdeg(&t[0].0) == 0 }

/// Buchberger with the normal selection strategy and Buchberger's two criteria
/// (Cox-Little-O'Shea, Ch. 2 Sec. 10). Stops early when the ideal contains 1.
fn groebner(f: &F, gens: &[P], ord: Ord_, n: usize, budget: u64) -> (GbStats, Vec<Q>) {
    let t0 = Instant::now();
    let mut st = GbStats::default();
    let mut basis: Vec<Q> = vec![];
    let mut pending: HashSet<(usize, usize)> = HashSet::new();
    // normal strategy: smallest lcm of leading monomials first, ties by indices
    let mut queue: std::collections::BTreeSet<(Key, usize, usize)> = std::collections::BTreeSet::new();
    let fail = |mut st: GbStats, e: RedErr, t0: Instant| { st.status = match e { RedErr::Budget => "censored_budget".into(), RedErr::Degree => "censored_degree".into() }; st.wall_us = t0.elapsed().as_micros(); st };
    macro_rules! push_poly { ($r:expr) => {{
        let r: Vec<(Mono, u64)> = $r;
        if !r.is_empty() {
            if is_const(&r) { st.status = "inconsistent".into(); st.basis = basis.len() as u64 + 1; st.wall_us = t0.elapsed().as_micros(); return (st, vec![]); }
            let mut pp = P::default(); for (m, c) in r { pp.t.insert(m, c); }
            let q = to_q(f, &pp, ord, n).unwrap();
            st.max_deg = st.max_deg.max(mdeg(&q.t[0].0));
            let idx = basis.len(); basis.push(q);
            for i in 0..idx { pending.insert((i, idx)); queue.insert((key(&mlcm(&basis[i].t[0].0, &basis[idx].t[0].0), ord, n), i, idx)); }
        }
    }}; }
    for g in gens {
        let q = match to_q(f, g, ord, n) { Some(q) => q, None => continue };
        let r = match reduce(f, q.t.clone(), &basis, ord, n, &mut st.termops, budget) { Ok(r) => r, Err(e) => return (fail(st, e, t0), vec![]) };
        push_poly!(r);
    }
    loop {
        let (i, j) = match queue.pop_first() { None => break, Some((_, i, j)) => (i, j) };
        pending.remove(&(i, j));
        let (li, lj) = (basis[i].t[0].0, basis[j].t[0].0);
        if coprime(&li, &lj) { st.crit1 += 1; continue; }
        let l = mlcm(&li, &lj);
        let crit2 = (0..basis.len()).any(|k| k != i && k != j && mdiv(&l, &basis[k].t[0].0).is_some()
            && !pending.contains(&(i.min(k), i.max(k))) && !pending.contains(&(j.min(k), j.max(k))));
        if crit2 { st.crit2 += 1; continue; }
        st.spairs += 1;
        let (qi, qj) = (mdiv(&l, &li).unwrap(), mdiv(&l, &lj).unwrap());
        let mut s: Vec<(Mono, u64)> = vec![];
        for (m, c) in &basis[i].t[1..] { match mmul(m, &qi) { Some(mm) => s.push((mm, *c)), None => return (fail(st, RedErr::Degree, t0), vec![]) } }
        for (m, c) in &basis[j].t[1..] { match mmul(m, &qj) { Some(mm) => s.push((mm, f.neg(*c))), None => return (fail(st, RedErr::Degree, t0), vec![]) } }
        st.termops += (basis[i].t.len() + basis[j].t.len()) as u64;
        let r = match reduce(f, s, &basis, ord, n, &mut st.termops, budget) { Ok(r) => r, Err(e) => return (fail(st, e, t0), vec![]) };
        if r.is_empty() { st.zero_red += 1; continue; }
        push_poly!(r);
    }
    st.status = "consistent".into();
    st.basis = basis.len() as u64;
    st.wall_us = t0.elapsed().as_micros();
    (st, basis)
}

// ------------------------------------------------------------------ chains
/// One presentation of a factor base V as the level-0 fibre of a chain of
/// degree-2 maps. Step j: X_{j+1} = map_j(X_j). Level L is the constant `term`.
#[derive(Clone, Debug)]
enum Step { Square, Velu { x0: u64, v: u64 }, Mob { alpha: u64 } }
#[derive(Clone, Debug)]
struct Chain { steps: Vec<Step>, term: u64, mobius_alpha: Option<u64> }
impl Chain {
    fn len(&self) -> usize { self.steps.len() }
    /// Step equation in variables (cur, next); `next` is either a variable or the terminal constant.
    fn eq(&self, f: &F, j: usize, cur: &P, next: &P) -> P {
        match self.steps[j] {
            Step::Square => next.sub(f, &cur.mul(f, cur)),
            // X'(X - x0) - X^2 + x0 X - v
            Step::Velu { x0, v } => next.mul(f, &cur.sub(f, &P::c(f, x0))).sub(f, &cur.mul(f, cur)).add(f, &cur.scale(f, x0)).sub(f, &P::c(f, v)),
            // Y'(2Y - a - 1) - Y^2 + a
            Step::Mob { alpha } => next.mul(f, &cur.scale(f, 2).sub(f, &P::c(f, alpha + 1))).sub(f, &cur.mul(f, cur)).add(f, &P::c(f, alpha)),
        }
    }
    fn fwd(&self, f: &F, j: usize, x: u64) -> Option<u64> {
        match self.steps[j] {
            Step::Square => Some(f.mul(x, x)),
            Step::Velu { x0, v } => { let d = f.sub(x, x0); if d == 0 { None } else { Some(f.add(x, f.mul(v, f.inv(d)))) } }
            Step::Mob { alpha } => { let d = f.sub(f.mul(2, x), alpha + 1); if d == 0 { None } else { Some(f.mul(f.sub(f.mul(x, x), alpha), f.inv(d))) } } // (Y^2 - a)/(2Y - a - 1)
        }
    }
    /// Roots X of step j with given image t, with multiplicity (double root -> 2).
    fn back(&self, f: &F, j: usize, t: u64) -> Vec<(u64, u32)> {
        // all steps are X^2 + bX + c = 0 after clearing
        let (b, c) = match self.steps[j] {
            Step::Square => (0, f.neg(t)),
            Step::Velu { x0, v } => (f.neg(f.add(x0, t)), f.add(f.mul(x0, t), v)),
            Step::Mob { alpha } => (f.neg(f.mul(2, t)), f.add(f.mul(t, alpha + 1), f.neg(alpha))),
        };
        let disc = f.sub(f.mul(b, b), f.mul(4, c));
        let i2 = f.inv(2);
        if disc == 0 { return vec![(f.mul(f.neg(b), i2), 2)]; }
        match f.sq_root(disc) {
            None => vec![],
            Some(s) => vec![(f.mul(f.sub(s, b), i2), 1), (f.mul(f.neg(f.add(s, b)), i2), 1)],
        }
    }
    /// level-0 fibre over F_p: (distinct points, total multiplicity, all rational?)
    fn fibre(&self, f: &F) -> (Vec<u64>, u64, bool) {
        let mut cur: Vec<(u64, u64)> = vec![(self.term, 1)];
        let mut rational = true;
        for j in (0..self.len()).rev() {
            let mut nxt = vec![];
            for (t, m) in &cur {
                let r = self.back(f, j, *t);
                let tot: u32 = r.iter().map(|x| x.1).sum();
                if tot < 2 { rational = false; }
                for (x, mm) in r { nxt.push((x, m * mm as u64)); }
            }
            cur = nxt;
        }
        let mult: u64 = cur.iter().map(|x| x.1).sum();
        let mut d: Vec<u64> = cur.iter().map(|x| x.0).collect(); d.sort(); d.dedup();
        (d, mult, rational)
    }
    fn levels(&self, f: &F, x0: u64) -> Option<Vec<u64>> {
        let mut v = vec![x0];
        for j in 0..self.len() { let n = self.fwd(f, j, *v.last().unwrap())?; v.push(n); }
        if *v.last().unwrap() != self.term { return None; }
        v.pop();
        Some(v)
    }
}

// ------------------------------------------------------------------ factor bases
fn mu_coset(f: &F, k: u32, coset: bool) -> (Vec<u64>, Chain) {
    let g = f.primitive_root();
    let gen = f.pow(g, (f.p - 1) >> k);
    let zeta = if coset { f.pow(g, (f.p - 1) >> (k + 1)) } else { 1 };
    let mut v: Vec<u64> = (0..(1u64 << k)).map(|i| f.mul(zeta, f.pow(gen, i))).collect();
    v.sort();
    (v, Chain { steps: vec![Step::Square; k as usize], term: if coset { f.p - 1 } else { 1 }, mobius_alpha: None })
}
/// Moebius presentation of the coset chain: y = (x - a)/(x - 1).
fn mobius_chain(f: &F, k: u32, alpha: u64) -> Chain {
    let m = |x: u64| f.mul(f.sub(x, alpha), f.inv(f.sub(x, 1)));
    Chain { steps: vec![Step::Mob { alpha }; k as usize], term: m(f.p - 1), mobius_alpha: Some(alpha) }
}
fn mobius_inv(f: &F, alpha: u64, y: u64) -> u64 { f.mul(f.sub(y, alpha), f.inv(f.sub(y, 1))) }

#[derive(Clone)]
struct Aux { e: Ec, g: (u64, u64), #[allow(dead_code)] n: u64 }
/// Auxiliary curve with a rational point G of order 2^(k+2).
fn find_aux(f: &F, k: u32, rng: &mut Rng, max_tries: u64) -> Option<Aux> {
    let want = k + 2;
    for _ in 0..max_tries {
        let e = Ec { a: rng.below(f.p), b: rng.below(f.p) };
        if !e.nonsingular(f) { continue; }
        let n = e.order(f);
        if v2(n) < want { continue; }
        let pts = e.points(f);
        for _ in 0..64 {
            let p = Some(pts[rng.below(pts.len() as u64) as usize]);
            let q = e.mul(f, n >> v2(n), p);
            if q.is_none() { continue; }
            let o = pt_order(&e, f, q, 1 << v2(n));
            let w = v2(o);
            if w >= want { let g = e.mul(f, 1 << (w - want), q).unwrap(); return Some(Aux { e, g, n }); }
        }
    }
    None
}
/// Velu chain of `steps` 2-isogenies from <G>; returns (V = x of odd multiples, chain).
fn velu_chain(f: &F, aux: &Aux, k: u32, steps: usize) -> (Vec<u64>, Chain) {
    let ord = 1u64 << (k + 2);
    let mut e = aux.e; let mut g = Some(aux.g); let mut o = ord;
    let mut st = vec![];
    for _ in 0..steps {
        let t = e.mul(f, o / 2, g).unwrap();
        assert_eq!(t.1, 0);
        let x0 = t.0;
        let v = f.add(f.mul(3, f.mul(x0, x0)), e.a);
        let w = f.mul(x0, v);
        let e2 = Ec { a: f.sub(e.a, f.mul(5, v)), b: f.sub(e.b, f.mul(7, w)) };
        let (x, y) = g.unwrap();
        let d = f.inv(f.sub(x, x0));
        let gx = f.add(x, f.mul(v, d));
        let gy = f.sub(y, f.mul(f.mul(v, y), f.mul(d, d)));
        let g2 = Some((gx, gy));
        assert!(e2.on(f, g2), "Velu image not on codomain");
        st.push(Step::Velu { x0, v });
        e = e2; g = g2; o /= 2;
    }
    let term = g.unwrap().0;
    let mut vset: Vec<u64> = (1..ord).step_by(2).map(|n| aux.e.mul(f, n, Some(aux.g)).unwrap().0).collect();
    vset.sort(); vset.dedup();
    (vset, Chain { steps: st, term, mobius_alpha: None })
}

// ------------------------------------------------------------------ decomposition
fn s3(f: &F, e: &Ec, x1: &P, x2: &P, x3: u64) -> P {
    // (x1-x2)^2 x3^2 - 2((x1+x2)(x1x2+a)+2b) x3 + (x1x2-a)^2 - 4b(x1+x2)
    let d = x1.sub(f, x2);
    let s = x1.add(f, x2);
    let pr = x1.mul(f, x2);
    let t1 = d.mul(f, &d).scale(f, f.mul(x3, x3));
    let t2 = s.mul(f, &pr.add(f, &P::c(f, e.a))).add(f, &P::c(f, f.mul(2, e.b))).scale(f, f.mul(2, x3));
    let u = pr.sub(f, &P::c(f, e.a));
    let t3 = u.mul(f, &u).sub(f, &s.scale(f, f.mul(4, e.b)));
    t1.sub(f, &t2).add(f, &t3)
}
fn s3_val(f: &F, e: &Ec, x1: u64, x2: u64, x3: u64) -> u64 {
    let d = f.sub(x1, x2); let s = f.add(x1, x2); let pr = f.mul(x1, x2);
    let t1 = f.mul(f.mul(d, d), f.mul(x3, x3));
    let t2 = f.mul(f.mul(2, x3), f.add(f.mul(s, f.add(pr, e.a)), f.mul(2, e.b)));
    let u = f.sub(pr, e.a);
    f.add(f.sub(t1, t2), f.sub(f.mul(u, u), f.mul(f.mul(4, e.b), s)))
}

/// Decomposition system for target x-coordinate xr. Variables: index 2j+i for
/// party i in {0,1} at level j < L.
fn system(f: &F, e: &Ec, ch: &Chain, xr: u64) -> (Vec<P>, usize) {
    let l = ch.len(); let n = 2 * l;
    let var = |i: usize, j: usize| -> P { if j == l { P::c(f, ch.term) } else { P::var(2 * j + i) } };
    let mut eqs = vec![];
    // summation polynomial first
    let s = match ch.mobius_alpha {
        None => s3(f, e, &var(0, 0), &var(1, 0), xr),
        Some(a) => {
            // substitute x_i = (y_i - a)/(y_i - 1) and clear (y1-1)^2 (y2-1)^2
            let (y1, y2) = (var(0, 0), var(1, 0));
            let (n1, d1) = (y1.sub(f, &P::c(f, a)), y1.sub(f, &P::c(f, 1)));
            let (n2, d2) = (y2.sub(f, &P::c(f, a)), y2.sub(f, &P::c(f, 1)));
            let base = s3(f, e, &P::var(14), &P::var(15), xr); // placeholder vars, degree <= 2 each
            let mut acc = P::default();
            for (m, c) in &base.t {
                let (ea, eb) = (m[14] as u32, m[15] as u32);
                let term = n1.pow(f, ea).mul(f, &d1.pow(f, 2 - ea)).mul(f, &n2.pow(f, eb)).mul(f, &d2.pow(f, 2 - eb)).scale(f, *c);
                acc = acc.add(f, &term);
            }
            acc
        }
    };
    eqs.push(s);
    for i in 0..2 { for j in 0..l { eqs.push(ch.eq(f, j, &var(i, j), &var(i, j + 1))); } }
    (eqs, n)
}

/// Exact yield: F = points with x in V, D = {P1 + P2 : P1, P2 in F} \ {O}.
fn yield_exact(f: &F, e: &Ec, v: &[u64]) -> (u64, u64, HashSet<(u64, u64)>) {
    let vs: HashSet<u64> = v.iter().copied().collect();
    let fpts: Vec<(u64, u64)> = e.points(f).into_iter().filter(|p| vs.contains(&p.0)).collect();
    let mut d = HashSet::new();
    for a in 0..fpts.len() { for b in a..fpts.len() { if let Some(s) = e.add(f, Some(fpts[a]), Some(fpts[b])) { d.insert(s); } } }
    (fpts.len() as u64, d.len() as u64, d)
}
fn expected_d(nf: u64, npts: u64) -> f64 {
    let pairs = (nf * (nf + 1)) as f64 / 2.0;
    npts as f64 * (1.0 - (-pairs / npts as f64).exp())
}

// ------------------------------------------------------------------ stats
fn median(v: &mut Vec<f64>) -> f64 { v.sort_by(|a, b| a.partial_cmp(b).unwrap()); let n = v.len(); if n == 0 { f64::NAN } else if n % 2 == 1 { v[n / 2] } else { (v[n / 2 - 1] + v[n / 2]) / 2.0 } }
fn boot_ci<Fn_: Fn(&[f64], &[f64]) -> f64>(a: &[f64], b: &[f64], stat: Fn_, reps: usize, seed: u64) -> (f64, f64) {
    let mut rng = Rng::new(seed); let mut out = vec![];
    for _ in 0..reps {
        let ra: Vec<f64> = (0..a.len()).map(|_| a[rng.below(a.len() as u64) as usize]).collect();
        let rb: Vec<f64> = (0..b.len()).map(|_| b[rng.below(b.len() as u64) as usize]).collect();
        out.push(stat(&ra, &rb));
    }
    out.sort_by(|x, y| x.partial_cmp(y).unwrap());
    (out[(reps as f64 * 0.025) as usize], out[(reps as f64 * 0.975) as usize - 1])
}
fn med_of(a: &[f64]) -> f64 { let mut v = a.to_vec(); median(&mut v) }

// ------------------------------------------------------------------ config
const MASTER: u64 = 20261102;
const Q1_PRIMES: [u64; 5] = [257, 1153, 7681, 12289, 40961];
const Q1_K: [u32; 3] = [3, 4, 5];
const Q1_NE: usize = 16;
const Q1_NAUX: usize = 8;
const Q1_NRAND: usize = 8;
const Q2_PRIMES: [u64; 4] = [257, 1153, 7681, 12289];
const Q2_K: [u32; 2] = [3, 4];
const Q2_NE: usize = 4;
const Q2_NAUX: usize = 2;
const Q2_NALPHA: usize = 2;
const Q2_NR: usize = 6; // per stratum
const BUDGET_GREVLEX: u64 = 50_000_000;
const BUDGET_LEX: u64 = 20_000_000;
const AUX_TRIES: u64 = 200_000;
const BOOT: usize = 4000;

fn targets(f: &F, k: u32, n: usize) -> Vec<Ec> {
    let mut rng = Rng::new(derive(MASTER, &format!("targets/{}/{}", f.p, k)));
    let mut v = vec![];
    while v.len() < n { let e = Ec { a: rng.below(f.p), b: rng.below(f.p) }; if e.nonsingular(f) { v.push(e); } }
    v
}
fn auxes(f: &F, k: u32, n: usize) -> Vec<Aux> {
    let mut rng = Rng::new(derive(MASTER, &format!("aux/{}/{}", f.p, k)));
    let mut v = vec![];
    while v.len() < n { match find_aux(f, k, &mut rng, AUX_TRIES) { Some(a) => v.push(a), None => break } }
    v
}

// ------------------------------------------------------------------ main
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let out = args.get(1).cloned().unwrap_or_else(|| "results".into());
    let which = args.get(2).cloned().unwrap_or_else(|| "all".into());
    std::fs::create_dir_all(&out).unwrap();
    let mut summary = String::new();
    writeln!(summary, "{{\"schema\":\"isomu-fb-summary/v1\",\"master_seed\":{},", MASTER).unwrap();
    if which == "all" || which == "q3" { q3_s1(&out, &mut summary); }
    if which == "all" || which == "q1" { q1(&out, &mut summary); }
    if which == "analyze" { analyze(&out); return; }
    if which == "all" || which == "q2" { q2(&out, &mut summary, args.get(3).map(|x| x.parse().unwrap()), args.get(4).map(|x| x.parse().unwrap())); }
    summary.push_str("\"end\":true}\n");
    std::fs::write(format!("{}/summary.json", out), summary).unwrap();
}

fn q3_s1(out: &str, summary: &mut String) {
    let mut q3 = String::from("p,k,arm,instance,steps,quotient_dim,distinct_fibre,total_multiplicity,fibre_rational,radical,fibre_equals_V,V_size\n");
    let mut s1 = String::from("p,k,arm,instance,steps,basis_size,products,nonzero_structure_constants,per_product\n");
    let mut fails = 0u64;
    for &p in &Q2_PRIMES { for &k in &Q2_K {
        let f = F::new(p);
        if (p - 1) % (1 << (k + 1)) != 0 { continue; }
        let mut arms: Vec<(String, String, Vec<u64>, Chain)> = vec![];
        let (v0, c0) = mu_coset(&f, k, false); arms.push(("A0".into(), "0".into(), v0, c0));
        let (v1, c1) = mu_coset(&f, k, true); arms.push(("A1".into(), "0".into(), v1.clone(), c1));
        let mut ar = Rng::new(derive(MASTER, &format!("alpha/{}/{}", p, k)));
        for a in 0..Q2_NALPHA {
            let alpha = loop { let x = ar.below(p); if x > 1 && x != p - 1 { break x; } };
            let ch = mobius_chain(&f, k, alpha);
            let mut vy: Vec<u64> = v1.iter().map(|&x| f.mul(f.sub(x, alpha), f.inv(f.sub(x, 1)))).collect(); vy.sort();
            arms.push(("A2".into(), format!("{}", a), vy, ch));
        }
        for (ai, aux) in auxes(&f, k, Q2_NAUX).iter().enumerate() {
            let (vb, cb) = velu_chain(&f, aux, k, k as usize); arms.push(("B".into(), format!("{}", ai), vb.clone(), cb));
            let (_, cb2) = velu_chain(&f, aux, k, k as usize + 1); arms.push(("B2".into(), format!("{}", ai), vb, cb2));
        }
        for (arm, inst, v, ch) in &arms {
            let (d, mult, rat) = ch.fibre(&f);
            let dim = 1u64 << ch.len();
            let eqv = d == *v;
            if !eqv { fails += 1; }
            writeln!(q3, "{},{},{},{},{},{},{},{},{},{},{},{}", p, k, arm, inst, ch.len(), dim, d.len(), mult, rat, d.len() as u64 == dim, eqv, v.len()).unwrap();
            // S1: quotient algebra of one party's chain, squarefree basis
            let l = ch.len();
            let mut gens = vec![];
            for j in 0..l { let nx = if j + 1 == l { P::c(&f, ch.term) } else { P::var(j + 1) }; gens.push(ch.eq(&f, j, &P::var(j), &nx)); }
            let gq: Vec<Q> = gens.iter().map(|g| to_q(&f, g, Ord_::Grevlex, l).unwrap()).collect();
            for g in &gq { let lm = g.t[0].0; assert!(mdeg(&lm) == 2 && lm.iter().filter(|&&e| e > 0).count() == 1, "chain LM not a pure square"); }
            let mut nz = 0u64; let mut ops = 0u64;
            let nb = 1usize << l;
            for u in 0..nb { for w in 0..nb {
                let mut m = [0u8; NV]; for i in 0..l { m[i] = ((u >> i) & 1) as u8 + ((w >> i) & 1) as u8; }
                let r = match reduce(&f, vec![(m, 1)], &gq, Ord_::Grevlex, l, &mut ops, u64::MAX) { Ok(r) => r, Err(_) => panic!() };
                nz += r.len() as u64;
            } }
            writeln!(s1, "{},{},{},{},{},{},{},{},{:.3}", p, k, arm, inst, l, nb, nb * nb, nz, nz as f64 / (nb * nb) as f64).unwrap();
        }
    } }
    std::fs::write(format!("{}/q3_terminal.csv", out), q3).unwrap();
    std::fs::write(format!("{}/s1_structure.csv", out), s1).unwrap();
    writeln!(summary, "\"q3_fibre_mismatches\":{},", fails).unwrap();
}

fn q1(out: &str, summary: &mut String) {
    let mut rows = String::from("p,k,npts,target,arm,instance,V_size,F_size,D_size,p_dec,expected_D,ratio\n");
    let mut cells = String::from("p,k,n_aux,median_ratio_A0,median_ratio_A1,median_ratio_B,median_ratio_R,median_ratio_PC,delta_B_minus_R,ci_lo,ci_hi,rel_effect,aa_delta,aa_ci_lo,aa_ci_hi,pc_valid,median_pdec_A0,median_pdec_B,median_pdec_R\n");
    for &p in &Q1_PRIMES { for &k in &Q1_K {
        let f = F::new(p);
        if (p - 1) % (1 << (k + 1)) != 0 { continue; }
        let tg = targets(&f, k, Q1_NE);
        let ax = auxes(&f, k, Q1_NAUX);
        let (v0, _) = mu_coset(&f, k, false);
        let (v1, _) = mu_coset(&f, k, true);
        let vb: Vec<Vec<u64>> = ax.iter().map(|a| velu_chain(&f, a, k, k as usize).0).collect();
        let mut rr = Rng::new(derive(MASTER, &format!("randV/{}/{}", p, k)));
        let vr: Vec<Vec<u64>> = (0..Q1_NRAND).map(|_| { let mut s = HashSet::new(); while s.len() < 1 << k { s.insert(1 + rr.below(p - 1)); } let mut v: Vec<u64> = s.into_iter().collect(); v.sort(); v }).collect();
        let mut col: HashMap<String, Vec<f64>> = HashMap::new();
        let mut pdec: HashMap<String, Vec<f64>> = HashMap::new();
        let mut rpc = Rng::new(derive(MASTER, &format!("pc/{}/{}", p, k)));
        for (ti, e) in tg.iter().enumerate() {
            let npts = e.order(&f) - 1;
            let mut arms: Vec<(String, String, Vec<u64>)> = vec![("A0".into(), "0".into(), v0.clone()), ("A1".into(), "0".into(), v1.clone())];
            for (i, v) in vb.iter().enumerate() { arms.push(("B".into(), i.to_string(), v.clone())); }
            for (i, v) in vr.iter().enumerate() { arms.push((if i < Q1_NRAND / 2 { "Ra" } else { "Rb" }.into(), i.to_string(), v.clone())); }
            // positive control: x(jP0), j = 1..2^k, with distinct abscissas
            let pts = e.points(&f);
            let pcv = (0..1000).find_map(|_| {
                let p0 = Some(pts[rpc.below(pts.len() as u64) as usize]);
                let v: Vec<u64> = (1..=(1u64 << k)).filter_map(|j| e.mul(&f, j, p0).map(|q| q.0)).collect();
                let mut s = v.clone(); s.sort(); s.dedup();
                if s.len() == 1 << k { Some(s) } else { None }
            });
            if let Some(v) = pcv { arms.push(("PC".into(), "0".into(), v)); }
            for (arm, inst, v) in &arms {
                let (nf, nd, _) = yield_exact(&f, e, v);
                let ex = expected_d(nf, npts);
                let ratio = if ex > 0.0 { nd as f64 / ex } else { f64::NAN };
                writeln!(rows, "{},{},{},{},{},{},{},{},{},{:.6},{:.3},{:.6}", p, k, npts, ti, arm, inst, v.len(), nf, nd, nd as f64 / npts as f64, ex, ratio).unwrap();
                let key = if arm == "Ra" || arm == "Rb" { vec![arm.clone(), "R".into()] } else { vec![arm.clone()] };
                for kk in key { if ratio.is_finite() { col.entry(kk.clone()).or_default().push(ratio); } pdec.entry(kk).or_default().push(nd as f64 / npts as f64); }
            }
        }
        let g = |s: &str| col.get(s).cloned().unwrap_or_default();
        let (b, r, ra, rb, pc) = (g("B"), g("R"), g("Ra"), g("Rb"), g("PC"));
        let diff = |x: &[f64], y: &[f64]| med_of(x) - med_of(y);
        let (lo, hi) = if b.is_empty() { (f64::NAN, f64::NAN) } else { boot_ci(&b, &r, diff, BOOT, derive(MASTER, &format!("boot/{}/{}", p, k))) };
        let (alo, ahi) = boot_ci(&ra, &rb, diff, BOOT, derive(MASTER, &format!("bootaa/{}/{}", p, k)));
        let pc_valid = !pc.is_empty() && med_of(&pc) < 0.5 * med_of(&r);
        let pd = |s: &str| med_of(&pdec.get(s).cloned().unwrap_or_default());
        writeln!(cells, "{},{},{},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{},{:.5},{:.5},{:.5}", p, k, ax.len(), med_of(&g("A0")), med_of(&g("A1")), med_of(&b), med_of(&r), med_of(&pc),
            diff(&b, &r), lo, hi, diff(&b, &r) / med_of(&r), diff(&ra, &rb), alo, ahi, pc_valid, pd("A0"), pd("B"), pd("R")).unwrap();
    } }
    std::fs::write(format!("{}/q1_yield.csv", out), rows).unwrap();
    std::fs::write(format!("{}/q1_cells.csv", out), cells).unwrap();
    summary.push_str("\"q1\":\"q1_cells.csv\",\n");
}

/// `only_p`/`only_k`: development filter; the frozen run passes neither.
fn q2(out: &str, summary: &mut String, only_p: Option<u64>, only_k: Option<u32>) {
    let path = format!("{}/q2_systems.csv", out);
    let mut rows = String::from("p,k,order,target,arm,instance,stratum,r_index,xr,nvars,status,gate_ok,termops,spairs,crit1,crit2,zero_red,basis,max_deg,wall_us\n");
    let mut gate_fail = 0u64; let mut total = 0u64;
    std::fs::write(&path, &rows).unwrap(); rows.clear();
    for &p in &Q2_PRIMES { for &k in &Q2_K {
        if only_p.map_or(false, |x| x != p) || only_k.map_or(false, |x| x != k) { continue; }
        let f = F::new(p);
        if (p - 1) % (1 << (k + 1)) != 0 { continue; }
        let tg = targets(&f, k, Q1_NE); // same target list as Q1, first Q2_NE used
        let ax = auxes(&f, k, Q2_NAUX);
        let (v0, c0) = mu_coset(&f, k, false);
        let (v1, c1) = mu_coset(&f, k, true);
        let mut ar = Rng::new(derive(MASTER, &format!("alpha/{}/{}", p, k)));
        let alphas: Vec<u64> = (0..Q2_NALPHA).map(|_| loop { let x = ar.below(p); if x > 1 && x != p - 1 { break x; } }).collect();
        // arm list: (name, instance, V in x-coordinates, chain)
        let mut arms: Vec<(String, String, Vec<u64>, Chain)> = vec![("A0".into(), "0".into(), v0, c0), ("A1".into(), "0".into(), v1.clone(), c1)];
        for (i, &a) in alphas.iter().enumerate() { arms.push(("A2".into(), i.to_string(), v1.clone(), mobius_chain(&f, k, a))); }
        for (i, a) in ax.iter().enumerate() {
            let (vb, cb) = velu_chain(&f, a, k, k as usize); arms.push(("B".into(), i.to_string(), vb.clone(), cb));
            let (_, cb2) = velu_chain(&f, a, k, k as usize + 1); arms.push(("B2".into(), i.to_string(), vb, cb2));
        }
        for (ti, e) in tg.iter().take(Q2_NE).enumerate() {
            let all: Vec<(u64, u64)> = e.points(&f);
            for (arm, inst, v, ch) in &arms {
                let (_, _, d) = yield_exact(&f, e, v);
                let xcons = |xr: u64| v.iter().any(|&a| v.iter().any(|&b| s3_val(&f, e, a, b, xr) == 0));
                let mut rs = Rng::new(derive(MASTER, &format!("R/{}/{}/{}/{}/{}", p, k, ti, arm, inst)));
                let mut dv: Vec<(u64, u64)> = d.iter().copied().collect(); dv.sort();
                let mut picks: Vec<(&str, u64)> = vec![];
                if !dv.is_empty() { for _ in 0..Q2_NR { picks.push(("dec", dv[rs.below(dv.len() as u64) as usize].0)); } }
                let mut got = 0; let mut tries = 0;
                while got < Q2_NR && tries < 100_000 { tries += 1; let q = all[rs.below(all.len() as u64) as usize]; if !d.contains(&q) && !xcons(q.0) { picks.push(("nodec", q.0)); got += 1; } }
                for (ri, (stratum, xr)) in picks.iter().enumerate() {
                    // brute-force truth on x-coordinates; one witness for the evaluation gate
                    let witness = v.iter().find_map(|&a| v.iter().find(|&&b| s3_val(&f, e, a, b, *xr) == 0).map(|&b| (a, b)));
                    let (eqs, n) = system(&f, e, ch, *xr);
                    for ord in [Ord_::Grevlex, Ord_::Lex] {
                        let budget = if ord == Ord_::Grevlex { BUDGET_GREVLEX } else { BUDGET_LEX };
                        let (st, gb) = groebner(&f, &eqs, ord, n, budget);
                        total += 1;
                        let gate_ok = match st.status.as_str() {
                            "consistent" => witness.map_or(false, |(a, b)| {
                                let (ya, yb) = match ch.mobius_alpha { None => (a, b), Some(al) => (f.mul(f.sub(a, al), f.inv(f.sub(a, 1))), f.mul(f.sub(b, al), f.inv(f.sub(b, 1)))) };
                                match (ch.levels(&f, ya), ch.levels(&f, yb)) {
                                    (Some(la), Some(lb)) => {
                                        let mut pt = vec![0u64; n]; for j in 0..ch.len() { pt[2 * j] = la[j]; pt[2 * j + 1] = lb[j]; }
                                        gb.iter().all(|g| q_to_p(g).eval(&f, &pt) == 0) && eqs.iter().all(|g| g.eval(&f, &pt) == 0)
                                    }
                                    _ => false,
                                }
                            }),
                            "inconsistent" => witness.is_none(),
                            _ => true, // censored: no claim
                        };
                        if !gate_ok { gate_fail += 1; }
                        writeln!(rows, "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}", p, k, if ord == Ord_::Grevlex { "grevlex" } else { "lex" }, ti, arm, inst, stratum, ri, xr, n,
                            st.status, gate_ok, st.termops, st.spairs, st.crit1, st.crit2, st.zero_red, st.basis, st.max_deg, st.wall_us).unwrap();
                    }
                }
                let _ = mobius_inv; // kept for the report's inverse-map formula
            }
            { use std::io::Write; let mut fh = std::fs::OpenOptions::new().append(true).open(&path).unwrap(); fh.write_all(rows.as_bytes()).unwrap(); rows.clear(); }
            eprintln!("q2 p={} k={} target={} done", p, k, ti);
        }
    } }
    writeln!(summary, "\"q2_systems\":{},\"q2_gate_failures\":{},", total, gate_fail).unwrap();
}

// ------------------------------------------------------------------ analysis (frozen with the protocol)
fn read_csv(path: &str) -> (Vec<String>, Vec<Vec<String>>) {
    let s = std::fs::read_to_string(path).unwrap_or_else(|e| panic!("{}: {}", path, e));
    let mut it = s.lines();
    let h: Vec<String> = it.next().unwrap().split(',').map(|x| x.to_string()).collect();
    (h, it.filter(|l| !l.is_empty()).map(|l| l.split(',').map(|x| x.to_string()).collect()).collect())
}
fn col(h: &[String], name: &str) -> usize { h.iter().position(|x| x == name).unwrap_or_else(|| panic!("no column {}", name)) }

fn analyze(out: &str) {
    let mut v = String::from("{\"schema\":\"isomu-fb-verdicts/v1\",\n");
    // ---- Q1
    let (h, rows) = read_csv(&format!("{}/q1_cells.csv", out));
    let (ilo, ihi, irel, ipc) = (col(&h, "ci_lo"), col(&h, "ci_hi"), col(&h, "rel_effect"), col(&h, "pc_valid"));
    let (mut excess, mut negative, mut nullc, mut pcbad) = (0, 0, 0, 0);
    for r in &rows {
        let (lo, hi, rel): (f64, f64, f64) = (r[ilo].parse().unwrap(), r[ihi].parse().unwrap(), r[irel].parse().unwrap());
        if lo > 0.0 && rel >= 0.05 { excess += 1; }
        if hi < 0.0 { negative += 1; }
        if (lo <= 0.0 && hi >= 0.0) || rel.abs() < 0.05 { nullc += 1; }
        if r[ipc] != "true" { pcbad += 1; }
    }
    let q1v = if excess >= 2 && negative == 0 { "yield_excess" } else if nullc == rows.len() { "null_H1_0_retained" } else { "inconclusive" };
    writeln!(v, "\"q1\":{{\"cells\":{},\"cells_excess\":{},\"cells_ci_below_0\":{},\"cells_null\":{},\"cells_pc_invalid\":{},\"verdict\":\"{}\"}},", rows.len(), excess, negative, nullc, pcbad, q1v).unwrap();
    // ---- Q2
    let (h, rows) = read_csv(&format!("{}/q2_systems.csv", out));
    let (ip, ik, io, ia, is, it, ig, ist) = (col(&h, "p"), col(&h, "k"), col(&h, "order"), col(&h, "arm"), col(&h, "stratum"), col(&h, "termops"), col(&h, "gate_ok"), col(&h, "status"));
    let gate_fail = rows.iter().filter(|r| r[ig] != "true").count();
    let mut groups: BTreeMap<(u64, u32, String, String, String), Vec<f64>> = BTreeMap::new();
    let mut cens: BTreeMap<(u64, u32, String, String, String), u64> = BTreeMap::new();
    for r in &rows {
        let kk = (r[ip].parse().unwrap(), r[ik].parse().unwrap(), r[io].clone(), r[is].clone(), r[ia].clone());
        groups.entry(kk.clone()).or_default().push(r[it].parse().unwrap());
        if r[ist].starts_with("censored") { *cens.entry(kk).or_default() += 1; }
    }
    let mut cells: BTreeMap<(u64, u32, String, String), BTreeMap<String, Vec<f64>>> = BTreeMap::new();
    for ((p, k, o, s, a), x) in &groups { cells.entry((*p, *k, o.clone(), s.clone())).or_default().insert(a.clone(), x.clone()); }
    let mut tab = String::from("p,k,order,stratum,med_A0,med_A1,med_A2,med_B,med_B2,cens_B2,A0_over_A1,A2_over_A1,A2_over_A1_lo,A2_over_A1_hi,B_over_A1,B_over_A1_lo,B_over_A1_hi,A2_over_B,A2_over_B_lo,A2_over_B_hi,B2_over_B,B_over_A0\n");
    let ratio = |x: &[f64], y: &[f64]| med_of(x) / med_of(y);
    let (mut sup, mut fal, mut rep, mut n_gr, mut n_lex) = (0, 0, 0, 0, 0);
    for ((p, k, o, s), m) in &cells {
        let g = |a: &str| m.get(a).cloned().unwrap_or_default();
        let (a0, a1, a2, b, b2) = (g("A0"), g("A1"), g("A2"), g("B"), g("B2"));
        let sd = |t: &str| derive(MASTER, &format!("q2boot/{}/{}/{}/{}/{}", p, k, o, s, t));
        let (l1, h1) = boot_ci(&a2, &a1, ratio, BOOT, sd("A2A1"));
        let (l2, h2) = boot_ci(&b, &a1, ratio, BOOT, sd("BA1"));
        let (l3, h3) = boot_ci(&a2, &b, ratio, BOOT, sd("A2B"));
        let (r21, rb1, r2b, rba0) = (ratio(&a2, &a1), ratio(&b, &a1), ratio(&a2, &b), ratio(&b, &a0));
        let cb2 = cens.get(&(*p, *k, o.clone(), s.clone(), "B2".into())).copied().unwrap_or(0);
        writeln!(tab, "{},{},{},{},{:.0},{:.0},{:.0},{:.0},{:.0},{},{:.3},{:.3},{:.3},{:.3},{:.3},{:.3},{:.3},{:.3},{:.3},{:.3},{:.3},{:.3}", p, k, o, s, med_of(&a0), med_of(&a1), med_of(&a2), med_of(&b), med_of(&b2), cb2,
            ratio(&a0, &a1), r21, l1, h1, rb1, l2, h2, r2b, l3, h3, ratio(&b2, &b), rba0).unwrap();
        if o == "grevlex" {
            n_gr += 1;
            if r21 >= 2.0 && rb1 >= 2.0 && (0.5..=2.0).contains(&r2b) { sup += 1; }
            if rb1 >= 2.0 && r21 < 1.25 { fal += 1; }
        } else { n_lex += 1; if rba0 >= 2.0 { rep += 1; } }
    }
    std::fs::write(format!("{}/q2_cells.csv", out), tab).unwrap();
    let h2v = if sup >= 12 { "H2_supported" } else if fal >= 12 { "H2_falsified_V_specific" } else { "inconclusive" };
    writeln!(v, "\"q2\":{{\"systems\":{},\"gate_failures\":{},\"grevlex_comparisons\":{},\"support_count\":{},\"falsify_count\":{},\"verdict\":\"{}\",\"lex_comparisons\":{},\"replication_B_over_A0_ge2\":{}}},", rows.len(), gate_fail, n_gr, sup, fal, h2v, n_lex, rep).unwrap();
    // ---- S1 / Q3 predictions
    let (h, rows) = read_csv(&format!("{}/s1_structure.csv", out));
    let (ia, ipp) = (col(&h, "arm"), col(&h, "per_product"));
    let s1_ok = rows.iter().all(|r| { let x: f64 = r[ipp].parse().unwrap(); match r[ia].as_str() { "A0" | "A1" => x == 1.0, "A2" | "B" => x > 2.0, _ => true } });
    let (h, rows) = read_csv(&format!("{}/q3_terminal.csv", out));
    let (ia, irad, ieq, idist, imul, ivs) = (col(&h, "arm"), col(&h, "radical"), col(&h, "fibre_equals_V"), col(&h, "distinct_fibre"), col(&h, "total_multiplicity"), col(&h, "V_size"));
    let q3_ok = rows.iter().all(|r| r[ieq] == "true" && match r[ia].as_str() {
        "B2" => r[irad] == "false" && r[idist] == r[ivs] && r[imul].parse::<u64>().unwrap() == 2 * r[ivs].parse::<u64>().unwrap(),
        _ => r[irad] == "true",
    });
    writeln!(v, "\"s1_prediction_holds\":{},\"q3_prediction_holds\":{}}}", s1_ok, q3_ok).unwrap();
    std::fs::write(format!("{}/verdicts.json", out), v).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn buchberger_small() {
        let f = F::new(101);
        let (x, y) = (P::var(0), P::var(1));
        let g1 = vec![x.mul(&f, &x).sub(&f, &P::c(&f, 1)), y.sub(&f, &x)];
        assert_eq!(groebner(&f, &g1, Ord_::Grevlex, 2, u64::MAX).0.status, "consistent");
        let g2 = vec![x.sub(&f, &P::c(&f, 1)), x.sub(&f, &P::c(&f, 2))];
        assert_eq!(groebner(&f, &g2, Ord_::Lex, 2, u64::MAX).0.status, "inconsistent");
        // x^2 - y, x y - 1 -> consistent; x^3 = 1 has solutions in F_101? 3 | 100? no, but over closure it is consistent
        let g3 = vec![x.mul(&f, &x).sub(&f, &y), x.mul(&f, &y).sub(&f, &P::c(&f, 1))];
        assert_eq!(groebner(&f, &g3, Ord_::Lex, 2, u64::MAX).0.status, "consistent");
    }
    #[test]
    fn s3_vanishes_on_sums() {
        let f = F::new(1153); let e = Ec { a: 7, b: 11 };
        let pts = e.points(&f);
        for i in 0..20 { let (a, b) = (pts[i * 7], pts[i * 13 + 3]); if let Some(c) = e.add(&f, Some(a), Some(b)) { assert_eq!(s3_val(&f, &e, a.0, b.0, c.0), 0); } }
    }
    #[test]
    fn velu_fibre_is_odd_multiples() {
        let f = F::new(1153); let mut r = Rng::new(5);
        let a = find_aux(&f, 3, &mut r, 100_000).unwrap();
        let (v, ch) = velu_chain(&f, &a, 3, 3);
        assert_eq!(v.len(), 8); assert_eq!(ch.fibre(&f).0, v);
    }
}
