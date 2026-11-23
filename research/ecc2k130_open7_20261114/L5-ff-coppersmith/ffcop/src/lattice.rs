//! Coppersmith / Howgrave-Graham lattices over F_2[t] for bivariate polynomials mod f^k.
use crate::gf2x::P;
use crate::popov::{self, Stats};
use std::collections::BTreeMap;

pub type Bi = BTreeMap<(usize, usize), P>;

pub fn bi_add_term(a: &mut Bi, m: (usize, usize), c: &P) {
    if c.is_zero() { return; }
    let e = a.entry(m).or_insert_with(P::zero);
    e.xor_shl(c, 0);
    if e.is_zero() { a.remove(&m); }
}
pub fn bi_mul(a: &Bi, b: &Bi) -> Bi {
    let mut r = Bi::new();
    for (ma, ca) in a { for (mb, cb) in b { bi_add_term(&mut r, (ma.0 + mb.0, ma.1 + mb.1), &ca.mul(cb)); } }
    r
}
pub fn bi_scale(a: &Bi, c: &P) -> Bi { a.iter().map(|(m, x)| (*m, x.mul(c))).filter(|(_, x)| !x.is_zero()).collect() }
pub fn bi_shift(a: &Bi, s: (usize, usize)) -> Bi { a.iter().map(|(m, x)| ((m.0 + s.0, m.1 + s.1), x.clone())).collect() }
pub fn bi_pow(a: &Bi, e: usize) -> Bi {
    let mut r = Bi::new();
    r.insert((0, 0), P::one());
    for _ in 0..e { r = bi_mul(&r, a); }
    r
}
/// exact evaluation over F_2[t]
pub fn bi_eval(a: &Bi, x1: &P, x2: &P) -> P {
    let mut acc = P::zero();
    for (m, c) in a { acc = acc.add(&c.mul(&x1.pow(m.0)).mul(&x2.pow(m.1))); }
    acc
}

/// Monomial set + weights + basis (generators) for a lattice.
pub struct Lat {
    pub mons: Vec<(usize, usize)>,
    pub idx: BTreeMap<(usize, usize), usize>,
    pub wts: (usize, usize), // degree weight per variable, in units of L
    pub gens: Vec<Bi>,        // generator polynomials (unscaled)
    pub diag_deg: Vec<i64>,   // deg of diagonal (unscaled) per square-basis row, if triangular
}

pub enum Form { XBox, EForm }

/// Basis for multiplicity k.
/// XBox: monomials X1^a X2^b, a,b <= 2k; row (a,b): s=min(a/2,b/2), X1^{a-2s}X2^{b-2s} g^s f^{k-s}; leading coeff of g at X1^2X2^2 is 1.
/// EForm: monomials e1^a e2^b with a + 2b <= 4k; row (a,b): s=b/2, e1^a e2^{b-2s} g^s f^{k-s}; leading coeff of g at e2^2 is 1.
pub fn build(form: &Form, g: &Bi, f: &P, k: usize, wts: (usize, usize)) -> Lat {
    let mut mons = vec![];
    match form {
        Form::XBox => { for a in 0..=2 * k { for b in 0..=2 * k { mons.push((a, b)); } } }
        Form::EForm => { for b in 0..=2 * k { for a in 0..=(4 * k - 2 * b) { mons.push((a, b)); } } }
    }
    let idx: BTreeMap<_, _> = mons.iter().enumerate().map(|(i, m)| (*m, i)).collect();
    let gp: Vec<Bi> = (0..=k).map(|s| bi_pow(g, s)).collect();
    let fp: Vec<P> = (0..=k).map(|s| f.pow(s)).collect();
    let mut gens = vec![];
    let mut diag = vec![];
    for &(a, b) in &mons {
        let (s, sh) = match form {
            Form::XBox => { let s = (a / 2).min(b / 2); (s, (a - 2 * s, b - 2 * s)) }
            Form::EForm => { let s = b / 2; (s, (a, b - 2 * s)) }
        };
        let poly = bi_scale(&bi_shift(&gp[s], sh), &fp[k - s]);
        assert_eq!(poly.get(&(a, b)), Some(&fp[k - s]), "triangular leading coefficient");
        gens.push(poly);
        diag.push(((k - s) * (f.deg() as usize)) as i64);
    }
    Lat { mons, idx, wts, gens, diag_deg: diag }
}

impl Lat {
    pub fn dim(&self) -> usize { self.mons.len() }
    pub fn w(&self, m: (usize, usize)) -> usize { m.0 * self.wts.0 + m.1 * self.wts.1 }
    /// scaled rows: coefficient of monomial m multiplied by t^{L w(m)}
    pub fn rows(&self, gens: &[Bi], l_bound: usize) -> Vec<Vec<P>> {
        gens.iter().map(|g| {
            let mut row = vec![P::zero(); self.dim()];
            for (m, c) in g {
                let j = *self.idx.get(m).expect("monomial outside set");
                row[j] = c.shl(l_bound * self.w(*m));
            }
            row
        }).collect()
    }
    pub fn det_deg(&self, l_bound: usize) -> i64 {
        self.diag_deg.iter().sum::<i64>() + self.mons.iter().map(|m| (l_bound * self.w(*m)) as i64).sum::<i64>()
    }
    pub fn unscale(&self, row: &[P], l_bound: usize) -> Bi {
        let mut b = Bi::new();
        for (j, c) in row.iter().enumerate() {
            if c.is_zero() { continue; }
            let s = l_bound * self.w(self.mons[j]);
            assert!(c.low_zero(s), "scaled entry not divisible by t^s");
            b.insert(self.mons[j], c.shr(s));
        }
        b
    }
}

pub struct Reduced { pub rows: Vec<Vec<P>>, pub degs: Vec<i64>, pub st: Stats }

pub fn reduce(rows: Vec<Vec<P>>) -> Reduced {
    let mut st = Stats::default();
    let red = popov::weak_popov(rows, &mut st);
    let degs = red.iter().map(|r| popov::row_deg(r)).collect();
    Reduced { rows: red, degs, st }
}

/// Exact division by g (leading coefficient 1 in the chosen lex order). Returns Some(q) if h = q g.
/// order_b_major = false: compare (a, b); true: compare (b, a).
pub fn bi_div_exact(h: &Bi, g: &Bi, order_b_major: bool) -> Option<Bi> {
    let key = |m: &(usize, usize)| if order_b_major { (m.1, m.0) } else { (m.0, m.1) };
    let lm = *g.keys().max_by_key(|m| key(m)).unwrap();
    if !g[&lm].is_one() { return None; }
    let mut r = h.clone();
    let mut q = Bi::new();
    loop {
        let lt = match r.keys().max_by_key(|m| key(m)) { None => break, Some(m) => *m };
        if lt.0 < lm.0 || lt.1 < lm.1 { return None; }
        let c = r[&lt].clone();
        let sh = (lt.0 - lm.0, lt.1 - lm.1);
        bi_add_term(&mut q, sh, &c);
        for (m, gc) in g { bi_add_term(&mut r, (m.0 + sh.0, m.1 + sh.1), &gc.mul(&c)); }
        debug_assert!(!r.contains_key(&lt));
    }
    Some(q)
}

/// strip all factors of g; returns (stripped poly, number of factors removed)
pub fn strip_g(h: &Bi, g: &Bi, order_b_major: bool) -> (Bi, usize) {
    let mut cur = h.clone();
    let mut c = 0;
    while let Some(q) = bi_div_exact(&cur, g, order_b_major) { if q.is_empty() { break; } cur = q; c += 1; }
    (cur, c)
}
