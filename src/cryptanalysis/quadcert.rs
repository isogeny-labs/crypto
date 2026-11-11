//! # Certified quadratic rewriting of polynomial systems over `F_p`.
//!
//! Several binaries here (`p256_dickson_quadratic_s3`, the S3 chain
//! circuits, the symmetric `S4` rewrites) rewrite a decomposition system
//! into a lower-degree one by hand, with their own polynomial type and no
//! record of why the rewrite has the same solutions. This module does the
//! rewrite once, emits a certificate, and checks the certificate with a
//! separate replay that does not reuse the rewriting search.
//!
//! The method follows the certificate-producing quadratic compiler of the
//! public *Z is Diophantine in Q* project (github.com/elliotglazer/
//! diophantine-z-q, `quartics/quartic-962/compiler`). That repository has no
//! licence file, so nothing is copied: this is an independent
//! implementation over `F_p` of the same three ideas.
//!
//! 1. **Gate programs.** A system is written as assignments `v = e`,
//!    quotients (`den·inv = 1`, `out = num·inv`, so a quotient can never
//!    take a zero-denominator branch) and constraints `e = 0`. Some input
//!    variables are *projected*: the claim is always about the projection of
//!    the zero set onto them, never about the auxiliary coordinates.
//! 2. **Quadratisation.** Every monomial of degree above two is shortened by
//!    a fresh auxiliary `a` with the defining residual `a − x·y`. The checker
//!    expands every auxiliary back and requires each rewritten residual to
//!    equal its source residual exactly.
//! 3. **Certified eliminations.** A residual `c·v + L` with `c` a non-zero
//!    constant, `v` not projected and `v ∉ L` determines `v = −L/c`
//!    uniquely, so substituting it everywhere and dropping the residual is a
//!    bijection on zero sets that fixes the projected coordinates. Only
//!    substitutions that keep every residual quadratic are taken.
//!
//! Two circuit macros from the same source are included because they apply
//! directly to summation-polynomial systems: a balanced product tree for the
//! elementary symmetric functions of the summands' coordinates, and the
//! square split `P(w) = a_d·Q(w)² + R(w)` that evaluates an even-degree
//! univariate (for example a factor-base defining polynomial) with powers
//! only up to half its degree.
//!
//! A certificate establishes equality of projected zero sets over `F_p`
//! (and over every extension of `F_p`, since every step is an exact
//! polynomial identity or a unique solve). It says nothing about solving
//! cost: whether the quadratic system is cheaper for F4 is an empirical
//! question for a frozen experiment, not something a certificate shows.

use std::collections::{BTreeMap, BTreeSet};

/// A monomial is the sorted multiset of its variable indices
/// (`x0²·x3` is `[0, 0, 3]`); the empty monomial is the constant.
pub type Monomial = Vec<u32>;

/// A polynomial over `F_p`: monomial → coefficient in `1..p`.
pub type Poly = BTreeMap<Monomial, u64>;

fn mulm(a: u64, b: u64, p: u64) -> u64 {
    ((a as u128 * b as u128) % p as u128) as u64
}

fn powm(mut a: u64, mut e: u64, p: u64) -> u64 {
    let mut r = 1 % p;
    a %= p;
    while e > 0 {
        if e & 1 == 1 {
            r = mulm(r, a, p);
        }
        a = mulm(a, a, p);
        e >>= 1;
    }
    r
}

/// Inverse modulo the prime `p` (Fermat). `a` must be non-zero mod `p`.
pub fn invm(a: u64, p: u64) -> u64 {
    assert!(a % p != 0, "inverse of zero");
    powm(a, p - 2, p)
}

pub fn constant(c: u64, p: u64) -> Poly {
    let mut out = Poly::new();
    if c % p != 0 {
        out.insert(vec![], c % p);
    }
    out
}

pub fn var(v: u32) -> Poly {
    let mut out = Poly::new();
    out.insert(vec![v], 1);
    out
}

pub fn add(a: &Poly, b: &Poly, p: u64) -> Poly {
    axpy(a, b, 1, p)
}

/// `a + s·b`.
pub fn axpy(a: &Poly, b: &Poly, s: u64, p: u64) -> Poly {
    let mut out = a.clone();
    for (m, c) in b {
        let e = out.entry(m.clone()).or_insert(0);
        *e = (*e + mulm(*c, s, p)) % p;
        if *e == 0 {
            out.remove(m);
        }
    }
    out
}

pub fn sub(a: &Poly, b: &Poly, p: u64) -> Poly {
    axpy(a, b, p - 1, p)
}

pub fn scale(a: &Poly, s: u64, p: u64) -> Poly {
    axpy(&Poly::new(), a, s, p)
}

pub fn mul(a: &Poly, b: &Poly, p: u64) -> Poly {
    let mut out = Poly::new();
    for (m, c) in a {
        for (n, d) in b {
            let mut k = m.clone();
            k.extend_from_slice(n);
            k.sort_unstable();
            let e = out.entry(k).or_insert(0);
            *e = (*e + mulm(*c, *d, p)) % p;
        }
    }
    out.retain(|_, c| *c != 0);
    out
}

pub fn degree(a: &Poly) -> usize {
    a.keys().map(Vec::len).max().unwrap_or(0)
}

pub fn variables(a: &Poly) -> BTreeSet<u32> {
    a.keys().flat_map(|m| m.iter().copied()).collect()
}

/// Substitute `v := r` everywhere in `a`.
pub fn substitute(a: &Poly, v: u32, r: &Poly, p: u64) -> Poly {
    let mut out = Poly::new();
    let mut powers: Vec<Poly> = vec![constant(1, p)];
    for (m, c) in a {
        let k = m.iter().filter(|&&x| x == v).count();
        let rest: Monomial = m.iter().copied().filter(|&x| x != v).collect();
        while powers.len() <= k {
            let next = mul(powers.last().unwrap(), r, p);
            powers.push(next);
        }
        let mut base = Poly::new();
        base.insert(rest, *c);
        out = add(&out, &mul(&base, &powers[k], p), p);
    }
    out
}

/// Evaluate at a full assignment `x` (indexed by variable).
pub fn eval(a: &Poly, x: &[u64], p: u64) -> u64 {
    a.iter().fold(0, |acc, (m, c)| {
        let t = m.iter().fold(*c, |t, &v| mulm(t, x[v as usize], p));
        (acc + t) % p
    })
}

/// Dense exponent vectors over `n` variables, for `f4_fp::normalise`.
pub fn to_dense_terms(a: &Poly, n: usize) -> Vec<(Vec<u32>, u64)> {
    a.iter()
        .map(|(m, c)| {
            let mut e = vec![0u32; n];
            for &v in m {
                e[v as usize] += 1;
            }
            (e, *c)
        })
        .collect()
}

/// A gate program: the source system and its projected inputs.
#[derive(Clone, Debug)]
pub struct Builder {
    pub p: u64,
    next: u32,
    pub projected: Vec<u32>,
    /// Source residuals, each meaning `r = 0`.
    pub residuals: Vec<Poly>,
}

impl Builder {
    pub fn new(p: u64) -> Self {
        assert!(p > 2, "the square split divides by 2");
        Self {
            p,
            next: 0,
            projected: vec![],
            residuals: vec![],
        }
    }

    pub fn n_vars(&self) -> u32 {
        self.next
    }

    fn fresh(&mut self) -> u32 {
        let v = self.next;
        self.next += 1;
        v
    }

    /// A free input; projected inputs are the coordinates the claim is about.
    pub fn input(&mut self, projected: bool) -> u32 {
        let v = self.fresh();
        if projected {
            self.projected.push(v);
        }
        v
    }

    /// `v = e` for a fresh `v`.
    pub fn assign(&mut self, e: &Poly) -> u32 {
        let v = self.fresh();
        self.residuals.push(sub(&var(v), e, self.p));
        v
    }

    /// `out = num / den` with `den ≠ 0` enforced by `den·inv = 1`.
    pub fn quotient(&mut self, num: &Poly, den: &Poly) -> u32 {
        let p = self.p;
        let inv = self.fresh();
        self.residuals
            .push(sub(&mul(den, &var(inv), p), &constant(1, p), p));
        self.assign(&mul(num, &var(inv), p))
    }

    pub fn constrain(&mut self, e: &Poly) {
        self.residuals.push(e.clone());
    }

    /// Elementary symmetric functions `e_1..e_k` of `xs`, as the
    /// coefficients of `∏(1 + x_i·T)` built by a balanced product tree.
    /// Every coefficient gate is bilinear in its two children's
    /// coefficients, hence quadratic. Returns `[e_1, …, e_k]`.
    pub fn elementary_symmetric(&mut self, xs: &[u32]) -> Vec<u32> {
        fn build(b: &mut Builder, xs: &[u32]) -> Vec<Poly> {
            // Coefficients c_0 = 1, c_1, …, c_len of ∏(1 + x_i T).
            if xs.len() == 1 {
                return vec![constant(1, b.p), var(xs[0])];
            }
            let (l, r) = xs.split_at(xs.len() / 2);
            let (cl, cr) = (build(b, l), build(b, r));
            let p = b.p;
            let mut out = vec![constant(1, p)];
            for k in 1..=xs.len() {
                let mut e = Poly::new();
                for i in 0..=k {
                    let j = k - i;
                    if i < cl.len() && j < cr.len() {
                        e = add(&e, &mul(&cl[i], &cr[j], p), p);
                    }
                }
                // The top-level coefficients are assigned so later
                // gates see variables; inner ones too, to stay quadratic.
                out.push(var(b.assign(&e)));
            }
            out
        }
        let coeffs = build(self, xs);
        coeffs[1..]
            .iter()
            .map(|c| *c.keys().next().unwrap().first().unwrap())
            .collect()
    }

    /// `w, w², …, w^h` as variables (each `w^{i} = w^{i-1}·w`).
    pub fn powers(&mut self, w: u32, h: usize) -> Vec<u32> {
        let mut out = vec![w];
        for _ in 1..h {
            let prev = *out.last().unwrap();
            let next = self.assign(&mul(&var(prev), &var(w), self.p));
            out.push(next);
        }
        out
    }

    /// A quadratic expression equal to `P(w) = Σ coeffs[i]·w^i`.
    ///
    /// Odd degree `d` is written `P = P_{d-1}(w) + c_d·w^{(d-1)/2}·w^{(d+1)/2}`,
    /// with the split applied to the even part. For even `d`,
    /// `P = a_d·Q² + R`, `Q` monic of degree `d/2`, `deg R < d/2`, so only
    /// powers up to `w^{d/2}` (and `w^d` never) are materialised.
    pub fn univariate_square_split(&mut self, coeffs: &[u64], w: u32) -> Poly {
        let p = self.p;
        let mut c: Vec<u64> = coeffs.iter().map(|x| x % p).collect();
        while c.len() > 1 && *c.last().unwrap() == 0 {
            c.pop();
        }
        let d = c.len() - 1;
        if d <= 2 {
            let mut out = Poly::new();
            for (i, &ci) in c.iter().enumerate() {
                let mut m = Poly::new();
                m.insert(vec![w; i], 1);
                out = axpy(&out, &m, ci, p);
            }
            return out;
        }
        if d % 2 == 1 {
            // P = P_even + c_d·w^d with w^d = w^{(d-1)/2}·w^{(d+1)/2}.
            let top = c[d];
            let mut even = c.clone();
            even.pop();
            let base = self.univariate_square_split(&even, w);
            let h = (d + 1) / 2;
            let pw = self.powers(w, h);
            let t = mul(&var(pw[h - 2]), &var(pw[h - 1]), p);
            return axpy(&base, &t, top, p);
        }
        let h = d / 2;
        let ad = c[d];
        let ad_inv = invm(ad, p);
        let two_inv = invm(2, p);
        // Q = w^h + q_{h-1} w^{h-1} + … + q_0. Match coefficients of w^{2h-k}
        // for k = 1..h: P_{2h-k}/a_d = 2·q_{h-k} + Σ_{i+j=2h-k, h-k<i,j<h} q_i q_j
        // (q_h = 1).
        let mut q = vec![0u64; h + 1];
        q[h] = 1;
        for k in 1..=h {
            let target = 2 * h - k;
            let mut s = 0u64;
            for i in (h - k + 1)..h {
                let j = target - i;
                if j > h - k && j < h {
                    s = (s + mulm(q[i], q[j], p)) % p;
                }
            }
            let lhs = mulm(c[target], ad_inv, p);
            q[h - k] = mulm((lhs + p - s) % p, two_inv, p);
        }
        let pw = self.powers(w, h);
        let pw_poly = |i: usize| -> Poly {
            if i == 0 {
                constant(1, p)
            } else {
                var(pw[i - 1])
            }
        };
        let mut qpoly = Poly::new();
        for (i, &qi) in q.iter().enumerate() {
            qpoly = axpy(&qpoly, &pw_poly(i), qi, p);
        }
        // R = P − a_d·Q² as coefficient vectors, degree < h.
        let mut q2 = vec![0u64; 2 * h + 1];
        for i in 0..=h {
            for j in 0..=h {
                q2[i + j] = (q2[i + j] + mulm(q[i], q[j], p)) % p;
            }
        }
        let mut rpoly = Poly::new();
        for i in 0..=d {
            let ri = (c[i] + p - mulm(ad, q2[i], p)) % p;
            if ri != 0 {
                assert!(i < h, "square split remainder degree");
                rpoly = axpy(&rpoly, &pw_poly(i), ri, p);
            }
        }
        let qv = self.assign(&qpoly);
        axpy(&rpoly, &mul(&var(qv), &var(qv), p), ad, p)
    }
}

/// One certified elimination: residual `index` of the current system is
/// `c·var + L`, and `var := replacement = −L/c` everywhere.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Elimination {
    pub index: usize,
    pub var: u32,
    pub replacement: Poly,
}

#[derive(Clone, Debug)]
pub struct Certificate {
    pub p: u64,
    pub n_source: u32,
    pub n_total: u32,
    pub projected: Vec<u32>,
    pub source: Vec<Poly>,
    /// `(a, x, y)`: auxiliary `a` with residual `a − x·y`.
    pub aux: Vec<(u32, u32, u32)>,
    /// Rewritten source residuals followed by the auxiliary residuals.
    pub pre: Vec<Poly>,
    pub eliminations: Vec<Elimination>,
    /// The final quadratic system.
    pub residuals: Vec<Poly>,
}

impl Certificate {
    /// Variables still occurring in the final system or projected.
    pub fn active_variables(&self) -> BTreeSet<u32> {
        let mut s: BTreeSet<u32> = self.projected.iter().copied().collect();
        for r in &self.residuals {
            s.extend(variables(r));
        }
        s
    }

    /// Final residuals over a compact index set (projected inputs first),
    /// with the map `compact → original`.
    pub fn compacted(&self) -> (Vec<Poly>, Vec<u32>) {
        let mut order: Vec<u32> = self.projected.clone();
        for v in self.active_variables() {
            if !order.contains(&v) {
                order.push(v);
            }
        }
        let index: BTreeMap<u32, u32> = order
            .iter()
            .enumerate()
            .map(|(i, &v)| (v, i as u32))
            .collect();
        let polys = self
            .residuals
            .iter()
            .map(|r| {
                r.iter()
                    .map(|(m, c)| {
                        let mut k: Monomial = m.iter().map(|v| index[v]).collect();
                        k.sort_unstable();
                        (k, *c)
                    })
                    .collect()
            })
            .collect();
        (polys, order)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct CompileOptions {
    pub eliminate: bool,
    /// Reject an elimination whose substitution grows the total term
    /// count by more than this.
    pub max_growth: usize,
}

impl Default for CompileOptions {
    fn default() -> Self {
        Self {
            eliminate: true,
            max_growth: 400,
        }
    }
}

/// Split `r` as `c·v + L` with `c` constant and `v ∉ L`, if possible.
fn linear_in(r: &Poly, v: u32, p: u64) -> Option<(u64, Poly)> {
    let mut c = 0;
    let mut rest = Poly::new();
    for (m, k) in r {
        let n = m.iter().filter(|&&x| x == v).count();
        match (n, m.len()) {
            (0, _) => {
                rest.insert(m.clone(), *k);
            }
            (1, 1) => c = *k,
            _ => return None,
        }
    }
    (c % p != 0).then_some((c, rest))
}

fn replacement_of(c: u64, rest: &Poly, p: u64) -> Poly {
    scale(rest, (p - invm(c, p)) % p, p)
}

/// Quadratise, then (optionally) eliminate, recording a certificate.
pub fn compile(b: &Builder, opts: CompileOptions) -> Certificate {
    let p = b.p;
    let mut next = b.n_vars();
    let mut memo: BTreeMap<(u32, u32), u32> = BTreeMap::new();
    let mut aux: Vec<(u32, u32, u32)> = vec![];
    let mut rewritten = vec![];
    for r in &b.residuals {
        let mut cur = r.clone();
        while degree(&cur) > 2 {
            let mut out = Poly::new();
            for (m, c) in &cur {
                let mut m = m.clone();
                if m.len() > 2 {
                    let (x, y) = (m[0], m[1]);
                    let a = *memo.entry((x, y)).or_insert_with(|| {
                        let a = next;
                        next += 1;
                        aux.push((a, x, y));
                        a
                    });
                    m.drain(0..2);
                    m.push(a);
                    m.sort_unstable();
                }
                let e = out.entry(m).or_insert(0);
                *e = (*e + c) % p;
            }
            out.retain(|_, c| *c != 0);
            cur = out;
        }
        rewritten.push(cur);
    }
    let mut pre = rewritten;
    for &(a, x, y) in &aux {
        pre.push(sub(&var(a), &mul(&var(x), &var(y), p), p));
    }

    let projected: BTreeSet<u32> = b.projected.iter().copied().collect();
    let mut sys = pre.clone();
    let mut eliminations = vec![];
    while opts.eliminate {
        let size: usize = sys.iter().map(BTreeMap::len).sum();
        let mut best: Option<(usize, Elimination, Vec<Poly>)> = None;
        for (i, r) in sys.iter().enumerate() {
            for v in variables(r) {
                if projected.contains(&v) {
                    continue;
                }
                let Some((c, rest)) = linear_in(r, v, p) else {
                    continue;
                };
                let repl = replacement_of(c, &rest, p);
                let mut next_sys = Vec::with_capacity(sys.len() - 1);
                let mut ok = true;
                for (j, s) in sys.iter().enumerate() {
                    if j == i {
                        continue;
                    }
                    let t = substitute(s, v, &repl, p);
                    if degree(&t) > 2 {
                        ok = false;
                        break;
                    }
                    if !t.is_empty() {
                        next_sys.push(t);
                    }
                }
                if !ok {
                    continue;
                }
                let new_size: usize = next_sys.iter().map(BTreeMap::len).sum();
                if new_size > size + opts.max_growth {
                    continue;
                }
                if best.as_ref().map_or(true, |(s, _, _)| new_size < *s) {
                    best = Some((
                        new_size,
                        Elimination {
                            index: i,
                            var: v,
                            replacement: repl,
                        },
                        next_sys,
                    ));
                }
            }
        }
        match best {
            Some((_, e, next_sys)) => {
                eliminations.push(e);
                sys = next_sys;
            }
            None => break,
        }
    }

    Certificate {
        p,
        n_source: b.n_vars(),
        n_total: next,
        projected: b.projected.clone(),
        source: b.residuals.clone(),
        aux,
        pre,
        eliminations,
        residuals: sys,
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CheckReport {
    pub source_residuals: usize,
    pub aux: usize,
    pub eliminations: usize,
    pub final_residuals: usize,
    pub final_variables: usize,
    pub final_monomials: usize,
}

/// Independent replay of a certificate. On success the projected zero set
/// of `residuals` equals that of `source`, over `F_p` and its extensions.
pub fn check(cert: &Certificate) -> Result<CheckReport, String> {
    let p = cert.p;
    let projected: BTreeSet<u32> = cert.projected.iter().copied().collect();
    if projected.len() != cert.projected.len() {
        return Err("duplicate projected variable".into());
    }
    if let Some(v) = projected.iter().find(|&&v| v >= cert.n_source) {
        return Err(format!("projected variable {v} is not a source variable"));
    }
    for r in &cert.source {
        if let Some(v) = variables(r).into_iter().find(|&v| v >= cert.n_source) {
            return Err(format!("source residual uses non-source variable {v}"));
        }
    }
    // 1. Auxiliaries are fresh, increasing and defined from earlier variables.
    let mut expected = cert.n_source;
    for &(a, x, y) in &cert.aux {
        if a != expected {
            return Err(format!(
                "auxiliary {a} is not the next fresh index {expected}"
            ));
        }
        if x >= a || y >= a {
            return Err(format!("auxiliary {a} is defined from a later variable"));
        }
        expected += 1;
    }
    if expected != cert.n_total {
        return Err("variable count does not match the auxiliaries".into());
    }
    // 2. pre = rewritten source ++ auxiliary residuals, and each rewritten
    //    residual expands to its source residual exactly.
    if cert.pre.len() != cert.source.len() + cert.aux.len() {
        return Err("pre-system has the wrong length".into());
    }
    let expand = |r: &Poly| -> Poly {
        let mut t = r.clone();
        for &(a, x, y) in cert.aux.iter().rev() {
            t = substitute(&t, a, &mul(&var(x), &var(y), p), p);
        }
        t
    };
    for (i, (r, s)) in cert.pre.iter().zip(&cert.source).enumerate() {
        if degree(r) > 2 {
            return Err(format!("rewritten residual {i} is not quadratic"));
        }
        if &expand(r) != s {
            return Err(format!(
                "rewritten residual {i} does not expand to its source"
            ));
        }
    }
    for (k, &(a, x, y)) in cert.aux.iter().enumerate() {
        let want = sub(&var(a), &mul(&var(x), &var(y), p), p);
        if cert.pre[cert.source.len() + k] != want {
            return Err(format!("auxiliary residual {k} is not a − x·y"));
        }
    }
    // 3. Replay the eliminations.
    let mut sys = cert.pre.clone();
    for (k, e) in cert.eliminations.iter().enumerate() {
        if projected.contains(&e.var) {
            return Err(format!(
                "elimination {k} removes projected variable {}",
                e.var
            ));
        }
        let r = sys
            .get(e.index)
            .ok_or_else(|| format!("elimination {k} indexes a missing residual"))?;
        let (c, rest) = linear_in(r, e.var, p)
            .ok_or_else(|| format!("elimination {k}: residual is not c·v + L"))?;
        if replacement_of(c, &rest, p) != e.replacement {
            return Err(format!("elimination {k}: replacement is not −L/c"));
        }
        sys = sys
            .iter()
            .enumerate()
            .filter(|(j, _)| *j != e.index)
            .map(|(_, s)| substitute(s, e.var, &e.replacement, p))
            .filter(|s| !s.is_empty())
            .collect();
    }
    if sys != cert.residuals {
        return Err("replayed system differs from the recorded final system".into());
    }
    if let Some(i) = sys.iter().position(|r| degree(r) > 2) {
        return Err(format!("final residual {i} is not quadratic"));
    }
    Ok(CheckReport {
        source_residuals: cert.source.len(),
        aux: cert.aux.len(),
        eliminations: cert.eliminations.len(),
        final_residuals: sys.len(),
        final_variables: cert.active_variables().len(),
        final_monomials: sys.iter().map(BTreeMap::len).sum(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn projected_zeros(polys: &[Poly], n: usize, proj: &[u32], p: u64) -> BTreeSet<Vec<u64>> {
        let mut out = BTreeSet::new();
        let mut x = vec![0u64; n];
        loop {
            if polys.iter().all(|r| eval(r, &x, p) == 0) {
                out.insert(proj.iter().map(|&v| x[v as usize]).collect());
            }
            let mut i = 0;
            loop {
                if i == n {
                    return out;
                }
                x[i] += 1;
                if x[i] < p {
                    break;
                }
                x[i] = 0;
                i += 1;
            }
        }
    }

    fn small_program(p: u64) -> Builder {
        // x projected; y, z free. x³·y = 1, z = x·y + 2, z² = x + 1.
        let mut b = Builder::new(p);
        let x = b.input(true);
        let y = b.input(false);
        let x3y = mul(&mul(&mul(&var(x), &var(x), p), &var(x), p), &var(y), p);
        b.constrain(&sub(&x3y, &constant(1, p), p));
        let z = b.assign(&add(&mul(&var(x), &var(y), p), &constant(2, p), p));
        b.constrain(&sub(
            &mul(&var(z), &var(z), p),
            &add(&var(x), &constant(1, p), p),
            p,
        ));
        // A linear gate, eliminable: w = x + 3y, w·y = 2.
        let w = b.assign(&add(&var(x), &scale(&var(y), 3, p), p));
        b.constrain(&sub(&mul(&var(w), &var(y), p), &constant(2, p), p));
        b
    }

    #[test]
    fn projection_is_preserved_by_brute_force() {
        for p in [5u64, 7, 11] {
            let b = small_program(p);
            let cert = compile(&b, CompileOptions::default());
            check(&cert).unwrap();
            let src = projected_zeros(&cert.source, cert.n_source as usize, &cert.projected, p);
            let (fin, order) = cert.compacted();
            let k = cert.projected.len() as u32;
            let proj: Vec<u32> = (0..k).collect();
            assert_eq!(&order[..k as usize], &cert.projected[..]);
            let dst = projected_zeros(&fin, order.len(), &proj, p);
            assert_eq!(src, dst, "p = {p}");
        }
    }

    #[test]
    fn rejects_mutations() {
        let p = 7;
        let cert = compile(&small_program(p), CompileOptions::default());
        assert!(check(&cert).is_ok());
        assert!(!cert.eliminations.is_empty() && !cert.aux.is_empty());

        let mut m = cert.clone();
        let (k, c) = m.residuals[0]
            .iter()
            .next()
            .map(|(k, c)| (k.clone(), *c))
            .unwrap();
        m.residuals[0].insert(k, (c + 1) % p);
        assert!(check(&m).is_err(), "final coefficient");

        let mut m = cert.clone();
        m.aux[0].1 = m.aux[0].2 ^ 1;
        assert!(check(&m).is_err(), "auxiliary definition");

        let mut m = cert.clone();
        m.eliminations[0].var = m.projected[0];
        assert!(check(&m).is_err(), "projected elimination");

        let mut m = cert.clone();
        m.residuals.pop();
        assert!(check(&m).is_err(), "dropped residual");

        let mut m = cert.clone();
        m.source[0] = add(&m.source[0], &constant(1, p), p);
        assert!(check(&m).is_err(), "source changed");
    }

    #[test]
    fn elementary_symmetric_matches() {
        let p = 101;
        let mut b = Builder::new(p);
        let xs: Vec<u32> = (0..5).map(|_| b.input(true)).collect();
        let es = b.elementary_symmetric(&xs);
        assert_eq!(es.len(), 5);
        assert!(b.residuals.iter().all(|r| degree(r) <= 2));
        // Solve the assignment gates in order at a sample point.
        let vals = [3u64, 17, 42, 99, 5];
        let mut x = vec![0u64; b.n_vars() as usize];
        x[..5].copy_from_slice(&vals);
        for r in &b.residuals {
            // r = v − e with v the largest variable in r.
            let v = *variables(r).iter().max().unwrap();
            let e = sub(&var(v), r, p);
            x[v as usize] = eval(&e, &x, p);
        }
        for k in 1..=5usize {
            let mut want = 0u64;
            for mask in 0u32..32 {
                if mask.count_ones() as usize == k {
                    let t = (0..5)
                        .filter(|i| mask >> i & 1 == 1)
                        .fold(1, |t, i| mulm(t, vals[i], p));
                    want = (want + t) % p;
                }
            }
            assert_eq!(x[es[k - 1] as usize], want, "e_{k}");
        }
    }

    #[test]
    fn square_split_is_exact() {
        let p = 1009;
        for coeffs in [
            vec![5u64, 0, 7, 1, 0, 3, 11],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![4, 0, 0, 0, 0, 0, 0, 0, 0, 1000],
            vec![2, 3, 1],
        ] {
            let mut b = Builder::new(p);
            let w = b.input(true);
            let out = b.univariate_square_split(&coeffs, w);
            assert!(degree(&out) <= 2);
            assert!(b.residuals.iter().all(|r| degree(r) <= 2));
            for t in [0u64, 1, 2, 77, 1008] {
                let mut x = vec![0u64; b.n_vars() as usize];
                x[0] = t;
                for r in &b.residuals {
                    let v = *variables(r).iter().max().unwrap();
                    x[v as usize] = eval(&sub(&var(v), r, p), &x, p);
                }
                let want = coeffs
                    .iter()
                    .rev()
                    .fold(0, |acc, &c| (mulm(acc, t, p) + c) % p);
                assert_eq!(eval(&out, &x, p), want, "{coeffs:?} at {t}");
            }
        }
    }
}
