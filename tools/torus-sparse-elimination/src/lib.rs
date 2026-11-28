//! Bounded ordinary sparse Buchberger and deliberately partial root extraction.
//! No full-base, F4, performance or absence-certificate claim.
use num_bigint::BigUint;
use num_traits::{One, Zero};
use serde::Serialize;
use std::cmp::Ordering;
use torus_s4::{Field, System};

pub mod controls;

#[derive(Clone, Debug, Serialize)]
pub struct Limits {
    pub variables: usize,
    pub equations: usize,
    pub degree: usize,
    pub terms: usize,
    pub pairs: usize,
    pub basis: usize,
    pub work: u64,
    pub bytes: u64,
    pub depth: usize,
}
impl Default for Limits {
    fn default() -> Self {
        Self {
            variables: 2048,
            equations: 4096,
            degree: 32,
            terms: 65536,
            pairs: 65536,
            basis: 4096,
            work: 1 << 24,
            bytes: 64 << 20,
            depth: 64,
        }
    }
}
#[derive(Clone, Debug, Default, Serialize)]
pub struct Counts {
    pub terms_reserved_total: usize,
    pub work: u64,
    pub represented_bytes_reserved: u64,
    pub field_add_sub: u64,
    pub field_mul: u64,
    pub field_inverse: u64,
    pub field_sqrt: u64,
    pub visits_reserved: u64,
    pub reductions: u64,
    pub pairs_processed: u64,
    pub buchberger_calls: u64,
    pub branch_nodes: u64,
    pub max_degree: usize,
    pub original_replay_calls: u64,
}
#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub enum Stop {
    Invalid(String),
    Cap(String),
    Unsupported(String),
}
type R<T> = std::result::Result<T, Stop>;
#[derive(Clone, Debug, Serialize)]
pub enum Answer {
    Root(Vec<String>),
    NoRootObservation,
    Stopped(Stop),
}
#[derive(Serialize)]
pub struct Report {
    pub answer: Answer,
    pub counts: Counts,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Mono(Vec<(usize, usize)>);
impl Mono {
    fn degree(&self) -> usize {
        self.0.iter().map(|x| x.1).sum()
    }
    fn raw_cmp(&self, r: &Self) -> Ordering {
        let d = self.degree().cmp(&r.degree());
        if d != Ordering::Equal {
            return d;
        }
        let (mut i, mut j) = (self.0.len(), r.0.len());
        while i > 0 || j > 0 {
            let a = (i > 0).then(|| self.0[i - 1]);
            let b = (j > 0).then(|| r.0[j - 1]);
            match (a, b) {
                (Some((ai, ae)), Some((bi, be))) if ai == bi => {
                    let c = be.cmp(&ae);
                    if c != Ordering::Equal {
                        return c;
                    }
                    i -= 1;
                    j -= 1;
                }
                (Some((ai, _)), Some((bi, _))) if ai > bi => return Ordering::Less,
                (Some(_), Some(_)) => return Ordering::Greater,
                (Some(_), None) => return Ordering::Less,
                (None, Some(_)) => return Ordering::Greater,
                (None, None) => break,
            }
        }
        Ordering::Equal
    }
}
#[derive(Clone, Debug)]
struct Polynomial(Vec<(Mono, BigUint)>); // Descending grevlex, nonzero coefficients.
struct Context {
    f: Field,
    limits: Limits,
    c: Counts,
    scalar_bytes: u64,
}
impl Context {
    fn work(&mut self, n: u64) -> R<()> {
        if n > self.limits.work.saturating_sub(self.c.work) {
            return Err(Stop::Cap("work".into()));
        }
        self.c.work += n;
        Ok(())
    }
    fn reserve(&mut self, n: u64) -> R<()> {
        if n > self
            .limits
            .bytes
            .saturating_sub(self.c.represented_bytes_reserved)
        {
            return Err(Stop::Cap("represented storage".into()));
        }
        self.c.represented_bytes_reserved += n;
        Ok(())
    }
    fn visit(&mut self, n: usize) -> R<()> {
        self.work(n as u64)?;
        self.c.visits_reserved += n as u64;
        Ok(())
    }
    fn scalar(&mut self) -> R<()> {
        self.reserve(self.scalar_bytes + 64)
    }
    fn add(&mut self, a: &BigUint, b: &BigUint) -> R<BigUint> {
        self.work(1)?;
        self.scalar()?;
        self.c.field_add_sub += 1;
        Ok(self.f.add(a, b))
    }
    fn sub(&mut self, a: &BigUint, b: &BigUint) -> R<BigUint> {
        self.work(1)?;
        self.scalar()?;
        self.c.field_add_sub += 1;
        Ok(self.f.sub(a, b))
    }
    fn mul(&mut self, a: &BigUint, b: &BigUint) -> R<BigUint> {
        self.work(1)?;
        self.scalar()?;
        self.c.field_mul += 1;
        Ok(self.f.mul(a, b))
    }
    fn copy_scalar(&mut self, a: &BigUint) -> R<BigUint> {
        self.visit(1)?;
        self.scalar()?;
        Ok(a.clone())
    }
    fn pow(&mut self, a: &BigUint, n: &BigUint) -> R<BigUint> {
        let mut r = BigUint::one();
        let mut b = self.copy_scalar(a)?;
        for i in 0..n.bits() {
            self.visit(1)?;
            if n.bit(i) {
                r = self.mul(&r, &b)?
            }
            if i + 1 < n.bits() {
                b = self.mul(&b, &b)?
            }
        }
        Ok(r)
    }
    fn inv(&mut self, a: &BigUint) -> R<BigUint> {
        self.work(1)?;
        self.c.field_inverse += 1;
        if a.is_zero() {
            return Err(Stop::Invalid("inverse0".into()));
        }
        if a.is_one() {
            self.scalar()?;
            return Ok(BigUint::one());
        }
        let n = &self.f.p - 2u32;
        self.pow(a, &n)
    }
    fn sqrt(&mut self, a: &BigUint) -> R<Option<BigUint>> {
        self.work(1)?;
        self.c.field_sqrt += 1;
        if a.is_zero() {
            self.scalar()?;
            return Ok(Some(BigUint::zero()));
        }
        if &self.f.p % 8u32 != 5u32.into() {
            return Err(Stop::Unsupported("sqrt chart p!=5 mod8".into()));
        }
        let n = (&self.f.p + 3u32) >> 3;
        let mut r = self.pow(a, &n)?;
        if self.mul(&r, &r)? != *a {
            let n = (&self.f.p - 1u32) >> 2;
            let i = self.pow(&2u32.into(), &n)?;
            r = self.mul(&r, &i)?;
        }
        Ok((self.mul(&r, &r)? == *a).then_some(r))
    }
    fn cmp(&mut self, a: &Mono, b: &Mono) -> R<Ordering> {
        self.visit(1 + 2 * (a.0.len() + b.0.len()))?;
        Ok(a.raw_cmp(b))
    }
    fn copy_mono(&mut self, a: &Mono) -> R<Mono> {
        self.visit(a.0.len() + 1)?;
        self.reserve(32 + 16 * a.0.len() as u64)?;
        Ok(a.clone())
    }
    fn mono_product(&mut self, a: &Mono, b: &Mono) -> R<Mono> {
        self.visit(a.0.len() + b.0.len() + 1)?;
        self.reserve(32 + 16 * (a.0.len() + b.0.len()) as u64)?;
        let (mut i, mut j) = (0, 0);
        let mut v = Vec::with_capacity(a.0.len() + b.0.len());
        while i < a.0.len() || j < b.0.len() {
            if j == b.0.len() || (i < a.0.len() && a.0[i].0 < b.0[j].0) {
                v.push(a.0[i]);
                i += 1
            } else if i == a.0.len() || b.0[j].0 < a.0[i].0 {
                v.push(b.0[j]);
                j += 1
            } else {
                v.push((
                    a.0[i].0,
                    a.0[i]
                        .1
                        .checked_add(b.0[j].1)
                        .ok_or_else(|| Stop::Cap("exponent".into()))?,
                ));
                i += 1;
                j += 1
            }
        }
        let m = Mono(v);
        self.degree(&m)?;
        Ok(m)
    }
    fn degree(&mut self, m: &Mono) -> R<()> {
        self.visit(m.0.len() + 1)?;
        let d = m.degree();
        if d > self.limits.degree {
            return Err(Stop::Cap("degree".into()));
        }
        self.c.max_degree = self.c.max_degree.max(d);
        Ok(())
    }
    fn quotient(&mut self, a: &Mono, b: &Mono) -> R<Option<Mono>> {
        self.visit(a.0.len() + b.0.len() + 1)?;
        self.reserve(32 + 16 * a.0.len() as u64)?;
        let mut out = Vec::with_capacity(a.0.len());
        let mut j = 0;
        for &(i, e) in &a.0 {
            if j < b.0.len() && b.0[j].0 < i {
                return Ok(None);
            }
            let sub = if j < b.0.len() && b.0[j].0 == i {
                let x = b.0[j].1;
                j += 1;
                x
            } else {
                0
            };
            if sub > e {
                return Ok(None);
            }
            if e > sub {
                out.push((i, e - sub))
            }
        }
        if j != b.0.len() {
            return Ok(None);
        }
        Ok(Some(Mono(out)))
    }
    fn lcm(&mut self, a: &Mono, b: &Mono) -> R<Mono> {
        self.visit(a.0.len() + b.0.len() + 1)?;
        self.reserve(32 + 16 * (a.0.len() + b.0.len()) as u64)?;
        let (mut i, mut j) = (0, 0);
        let mut v = Vec::with_capacity(a.0.len() + b.0.len());
        while i < a.0.len() || j < b.0.len() {
            if j == b.0.len() || (i < a.0.len() && a.0[i].0 < b.0[j].0) {
                v.push(a.0[i]);
                i += 1
            } else if i == a.0.len() || b.0[j].0 < a.0[i].0 {
                v.push(b.0[j]);
                j += 1
            } else {
                v.push((a.0[i].0, a.0[i].1.max(b.0[j].1)));
                i += 1;
                j += 1
            }
        }
        let m = Mono(v);
        self.degree(&m)?;
        Ok(m)
    }
    fn insert(&mut self, p: &mut Polynomial, m: Mono, c: BigUint) -> R<()> {
        if c.is_zero() {
            return Ok(());
        }
        self.degree(&m)?;
        let mut i = 0;
        while i < p.0.len() {
            match self.cmp(&m, &p.0[i].0)? {
                Ordering::Equal => {
                    let v = self.add(&p.0[i].1, &c)?;
                    if v.is_zero() {
                        p.0.remove(i);
                    } else {
                        p.0[i].1 = v
                    }
                    return Ok(());
                }
                Ordering::Greater => break,
                Ordering::Less => i += 1,
            }
        }
        if self.c.terms_reserved_total >= self.limits.terms {
            return Err(Stop::Cap("terms".into()));
        }
        self.visit(p.0.len() - i + 1)?;
        self.reserve(192)?;
        self.c.terms_reserved_total += 1;
        p.0.insert(i, (m, c));
        Ok(())
    }
    fn copy_poly(&mut self, p: &Polynomial) -> R<Polynomial> {
        if p.0.len()
            > self
                .limits
                .terms
                .saturating_sub(self.c.terms_reserved_total)
        {
            return Err(Stop::Cap("terms".into()));
        }
        self.c.terms_reserved_total += p.0.len();
        self.reserve(24 + 64 * p.0.len() as u64)?;
        let mut out = Vec::with_capacity(p.0.len());
        for (m, c) in &p.0 {
            out.push((self.copy_mono(m)?, self.copy_scalar(c)?))
        }
        Ok(Polynomial(out))
    }
    fn monic(&mut self, p: &Polynomial) -> R<Polynomial> {
        if p.0.is_empty() {
            return Ok(Polynomial(vec![]));
        }
        let inv = self.inv(&p.0[0].1)?;
        let mut r = Polynomial(vec![]);
        for (m, c) in &p.0 {
            let m = self.copy_mono(m)?;
            let c = self.mul(c, &inv)?;
            self.insert(&mut r, m, c)?
        }
        Ok(r)
    }
    fn shifted(&mut self, p: &Polynomial, m: &Mono, c: &BigUint) -> R<Polynomial> {
        let mut r = Polynomial(vec![]);
        for (a, b) in &p.0 {
            let a = self.mono_product(a, m)?;
            let b = self.mul(b, c)?;
            self.insert(&mut r, a, b)?
        }
        Ok(r)
    }
    fn subtract(&mut self, a: &Polynomial, b: &Polynomial) -> R<Polynomial> {
        let mut r = self.copy_poly(a)?;
        for (m, c) in &b.0 {
            let m = self.copy_mono(m)?;
            let c = self.sub(&0u32.into(), c)?;
            self.insert(&mut r, m, c)?
        }
        Ok(r)
    }
    fn remainder(&mut self, p: &Polynomial, basis: &[Polynomial]) -> R<Polynomial> {
        let mut f = self.copy_poly(p)?;
        let mut r = Polynomial(vec![]);
        while !f.0.is_empty() {
            self.work(1)?;
            self.c.reductions += 1;
            let lm = self.copy_mono(&f.0[0].0)?;
            let lc = self.copy_scalar(&f.0[0].1)?;
            let mut changed = false;
            for g in basis {
                self.visit(1)?;
                if let Some(q) = self.quotient(&lm, &g.0[0].0)? {
                    let multiple = self.shifted(g, &q, &lc)?;
                    f = self.subtract(&f, &multiple)?;
                    changed = true;
                    break;
                }
            }
            if !changed {
                let (m, c) = f.0.remove(0);
                self.insert(&mut r, m, c)?
            }
        }
        Ok(r)
    }
    fn buchberger(&mut self, input: &[Polynomial]) -> R<Vec<Polynomial>> {
        self.work(1)?;
        self.c.buchberger_calls += 1;
        let mut admitted = 0usize;
        for p in input {
            self.visit(1)?;
            if !p.0.is_empty() {
                admitted += 1;
            }
        }
        if admitted > self.limits.basis {
            return Err(Stop::Cap("basis".into()));
        }
        self.reserve(24 + 64 * admitted as u64)?;
        let mut basis = Vec::with_capacity(admitted);
        for f in input {
            if !f.0.is_empty() {
                basis.push(self.monic(f)?);
            }
        }
        let count = basis.len().saturating_mul(basis.len().saturating_sub(1)) / 2;
        if count > self.limits.pairs {
            return Err(Stop::Cap("pairs".into()));
        }
        self.reserve(24 + 24 * count as u64)?;
        let mut pairs = Vec::with_capacity(count);
        for i in 0..basis.len() {
            for j in i + 1..basis.len() {
                let d = self.lcm(&basis[i].0[0].0, &basis[j].0[0].0)?.degree();
                pairs.push((d, i, j));
            }
        }
        while !pairs.is_empty() {
            let mut at = 0;
            for i in 0..pairs.len() {
                self.visit(1)?;
                if pairs[i] < pairs[at] {
                    at = i
                }
            }
            let (_, i, j) = pairs.swap_remove(at);
            self.work(1)?;
            self.c.pairs_processed += 1;
            let a = &basis[i].0[0].0;
            let b = &basis[j].0[0].0;
            let l = self.lcm(a, b)?;
            if l.degree() == a.degree() + b.degree() {
                continue;
            }
            let qa = self
                .quotient(&l, a)?
                .ok_or_else(|| Stop::Invalid("LCM".into()))?;
            let qb = self
                .quotient(&l, b)?
                .ok_or_else(|| Stop::Invalid("LCM".into()))?;
            let fa = self.shifted(&basis[i], &qa, &BigUint::one())?;
            let fb = self.shifted(&basis[j], &qb, &BigUint::one())?;
            let s = self.subtract(&fa, &fb)?;
            let r = self.remainder(&s, &basis)?;
            if r.0.is_empty() {
                continue;
            }
            let r = self.monic(&r)?;
            let n = basis.len();
            if n >= self.limits.basis {
                return Err(Stop::Cap("basis".into()));
            }
            if r.0[0].0 .0.is_empty() {
                return Ok(vec![r]);
            }
            if pairs.len() + n > self.limits.pairs {
                return Err(Stop::Cap("pairs".into()));
            }
            self.reserve(64 + 24 * n as u64)?;
            for (i, b) in basis.iter().enumerate() {
                let d = self.lcm(&b.0[0].0, &r.0[0].0)?.degree();
                pairs.push((d, i, n));
            }
            basis.push(r);
        }
        Ok(basis)
    }
}

impl Context {
    fn input(&mut self, s: &System) -> R<Vec<Polynomial>> {
        let n = s.variables.len();
        if n == 0 {
            return Err(Stop::Invalid("no variables".into()));
        }
        if n > self.limits.variables {
            return Err(Stop::Cap("variables".into()));
        }
        if s.equations.len() > self.limits.equations {
            return Err(Stop::Cap("equations".into()));
        }
        self.reserve(24 + 64 * s.equations.len() as u64)?;
        let mut out = Vec::with_capacity(s.equations.len());
        for p in &s.equations {
            let mut r = Polynomial(vec![]);
            for (indices, coefficient) in &p.terms {
                self.visit(indices.len() + 1)?;
                if indices.len() > self.limits.degree {
                    return Err(Stop::Cap("degree".into()));
                }
                self.reserve(32 + 16 * indices.len() as u64)?;
                let mut m: Vec<(usize, usize)> = Vec::with_capacity(indices.len());
                for &i in indices {
                    if i >= n {
                        return Err(Stop::Invalid("variable index".into()));
                    }
                    let mut j = 0;
                    while j < m.len() && m[j].0 < i {
                        self.visit(1)?;
                        j += 1
                    }
                    if j < m.len() && m[j].0 == i {
                        m[j].1 += 1
                    } else {
                        self.visit(m.len() - j)?;
                        m.insert(j, (i, 1));
                    }
                }
                self.visit((coefficient.bits() / 64 + 1) as usize)?;
                self.scalar()?;
                let c = coefficient % &self.f.p;
                self.insert(&mut r, Mono(m), c)?;
            }
            out.push(r)
        }
        Ok(out)
    }
    fn restrict(
        &mut self,
        polys: &[Polynomial],
        assignment: &[Option<BigUint>],
    ) -> R<Vec<Polynomial>> {
        self.reserve(24 + 64 * polys.len() as u64)?;
        let mut out = Vec::with_capacity(polys.len());
        for p in polys {
            let mut r = Polynomial(vec![]);
            for (m, c) in &p.0 {
                let mut c = self.copy_scalar(c)?;
                self.reserve(32 + 16 * m.0.len() as u64)?;
                let mut remaining = Vec::with_capacity(m.0.len());
                for &(i, e) in &m.0 {
                    self.visit(1)?;
                    if let Some(a) = &assignment[i] {
                        let v = self.pow(a, &e.into())?;
                        c = self.mul(&c, &v)?
                    } else {
                        remaining.push((i, e))
                    }
                }
                self.insert(&mut r, Mono(remaining), c)?;
            }
            if !r.0.is_empty() {
                out.push(r)
            }
        }
        Ok(out)
    }
    fn univariate(&mut self, p: &Polynomial) -> R<Option<(usize, usize)>> {
        let mut variable = None;
        let mut degree = 0;
        for (m, _) in &p.0 {
            for &(v, e) in &m.0 {
                self.visit(1)?;
                if let Some(old) = variable {
                    if old != v {
                        return Ok(None);
                    }
                } else {
                    variable = Some(v)
                }
                degree = degree.max(e);
            }
        }
        Ok(variable.map(|v| (v, degree)))
    }
    fn roots(&mut self, p: &Polynomial, v: usize) -> R<Vec<BigUint>> {
        let mut c = [BigUint::zero(), BigUint::zero(), BigUint::zero()];
        for (m, a) in &p.0 {
            self.visit(m.0.len() + 1)?;
            let e = if m.0.is_empty() {
                0
            } else if m.0.len() == 1 && m.0[0].0 == v {
                m.0[0].1
            } else {
                return Err(Stop::Invalid("not univariate".into()));
            };
            if e > 2 {
                return Err(Stop::Unsupported("higher-degree root extraction".into()));
            }
            c[e] = self.copy_scalar(a)?;
        }
        if c[2].is_zero() {
            if c[1].is_zero() {
                return Err(Stop::Invalid("constant extraction".into()));
            }
            let neg = self.sub(&0u32.into(), &c[0])?;
            let inv = self.inv(&c[1])?;
            self.reserve(32)?;
            return Ok(vec![self.mul(&neg, &inv)?]);
        }
        let b2 = self.mul(&c[1], &c[1])?;
        let ac = self.mul(&c[2], &c[0])?;
        let fourac = self.mul(&4u32.into(), &ac)?;
        let d = self.sub(&b2, &fourac)?;
        let Some(sqrt) = self.sqrt(&d)? else {
            return Ok(vec![]);
        };
        let twoa = self.mul(&2u32.into(), &c[2])?;
        let inv = self.inv(&twoa)?;
        let minus_b = self.sub(&0u32.into(), &c[1])?;
        let plus = self.add(&minus_b, &sqrt)?;
        let minus = self.sub(&minus_b, &sqrt)?;
        let r1 = self.mul(&plus, &inv)?;
        let r2 = self.mul(&minus, &inv)?;
        self.reserve(64)?;
        Ok(if r1 == r2 {
            vec![r1]
        } else if r1 < r2 {
            vec![r1, r2]
        } else {
            vec![r2, r1]
        })
    }
    fn copy_assignment(&mut self, a: &[Option<BigUint>]) -> R<Vec<Option<BigUint>>> {
        self.reserve(24 + 64 * a.len() as u64)?;
        let mut out = Vec::with_capacity(a.len());
        for x in a {
            self.visit(1)?;
            out.push(match x {
                None => None,
                Some(x) => Some(self.copy_scalar(x)?),
            });
        }
        Ok(out)
    }
    fn node(
        &mut self,
        polys: &[Polynomial],
        mut assignment: Vec<Option<BigUint>>,
        depth: usize,
    ) -> R<Option<Vec<BigUint>>> {
        self.work(1)?;
        self.c.branch_nodes += 1;
        let mut polys = self.restrict(polys, &assignment)?;
        let mut basis_used = false;
        loop {
            let mut selected = None;
            for (i, p) in polys.iter().enumerate() {
                self.visit(1)?;
                if p.0.len() == 1 && p.0[0].0 .0.is_empty() {
                    return Ok(None);
                }
                if let Some((v, d)) = self.univariate(p)? {
                    if d <= 2 && selected.is_none_or(|(_, _, old)| d < old) {
                        selected = Some((i, v, d));
                    }
                }
            }
            if polys.is_empty() {
                self.reserve(24 + 64 * assignment.len() as u64)?;
                let mut values = Vec::with_capacity(assignment.len());
                for x in assignment {
                    self.visit(1)?;
                    self.scalar()?;
                    values.push(x.unwrap_or_default());
                }
                return Ok(Some(values));
            }
            if let Some((i, v, _)) = selected {
                let roots = self.roots(&polys[i], v)?;
                if roots.is_empty() {
                    return Ok(None);
                }
                if roots.len() == 1 {
                    assignment[v] = Some(roots.into_iter().next().unwrap());
                    polys = self.restrict(&polys, &assignment)?;
                    basis_used = false;
                    continue;
                }
                if depth >= self.limits.depth {
                    return Err(Stop::Cap("branch depth".into()));
                }
                let mut unsupported = None;
                for root in roots {
                    let mut next = self.copy_assignment(&assignment)?;
                    next[v] = Some(root);
                    match self.node(&polys, next, depth + 1) {
                        Ok(Some(values)) => return Ok(Some(values)),
                        Ok(None) => {}
                        Err(Stop::Unsupported(s)) => unsupported = Some(s),
                        Err(e) => return Err(e),
                    }
                }
                return match unsupported {
                    Some(s) => Err(Stop::Unsupported(s)),
                    None => Ok(None),
                };
            }
            if basis_used {
                return Err(Stop::Unsupported(
                    "no linear/quadratic rational extraction in computed basis".into(),
                ));
            }
            polys = self.buchberger(&polys)?;
            basis_used = true;
        }
    }
}

pub fn solve(system: &System, limits: Limits) -> Report {
    if !(5..=512).contains(&system.p.bits()) || !system.p.bit(0) {
        return Report {
            answer: Answer::Stopped(Stop::Invalid(
                "odd modulus admission5..512bits; primality external".into(),
            )),
            counts: Counts::default(),
        };
    }
    if limits.work == 0 {
        return Report {
            answer: Answer::Stopped(Stop::Cap("work".into())),
            counts: Counts::default(),
        };
    }
    let setup_bytes = 128 + 2 * system.p.bits().div_ceil(8);
    if setup_bytes > limits.bytes {
        return Report {
            answer: Answer::Stopped(Stop::Cap("represented storage".into())),
            counts: Counts::default(),
        };
    }
    let make = Field::new(system.p.clone());
    let f = match make {
        Ok(f) if (5..=512).contains(&f.p.bits()) => f,
        _ => {
            return Report {
                answer: Answer::Stopped(Stop::Invalid(
                    "odd modulus admission5..512bits; primality external".into(),
                )),
                counts: Counts::default(),
            }
        }
    };
    let mut context = Context {
        scalar_bytes: f.p.bits().div_ceil(8),
        f,
        limits,
        c: Counts {
            work: 1,
            visits_reserved: 1,
            represented_bytes_reserved: setup_bytes,
            ..Counts::default()
        },
    };
    let result = (|| {
        let input = context.input(system)?;
        context.reserve(24 + 64 * system.variables.len() as u64)?;
        context.visit(system.variables.len())?;
        let assignment = vec![None; system.variables.len()];
        let root = context.node(&input, assignment, 0)?;
        if let Some(values) = root {
            context.c.original_replay_calls += 1;
            system
                .check(&values)
                .map_err(|e| Stop::Invalid(format!("original replay: {e}")))?;
            Ok(Answer::Root(
                values.into_iter().map(|x| x.to_string()).collect(),
            ))
        } else {
            Ok(Answer::NoRootObservation)
        }
    })();
    Report {
        answer: result.unwrap_or_else(Answer::Stopped),
        counts: context.c,
    }
}
