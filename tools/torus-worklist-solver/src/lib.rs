//! Classical dependency scheduling for deliberately partial polynomial extraction.
//! Diagnostic event/reservation counters are not complete cold cost or live RAM.
use num_bigint::BigUint;
use num_traits::{One, Zero};
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use torus_s4::{Field, System};

pub mod controls;

#[derive(Clone, Copy, Debug, Serialize, PartialEq, Eq)]
pub enum Schedule {
    Sweep,
    Worklist,
}
#[derive(Clone, Debug, Serialize)]
pub struct Limits {
    pub variables: usize,
    pub equations: usize,
    pub terms: usize,
    pub degree: usize,
    pub nodes: u64,
    pub depth: usize,
    pub events: u64,
    pub modeled_reservations: u64,
}
impl Default for Limits {
    fn default() -> Self {
        Self {
            variables: 4096,
            equations: 8192,
            terms: 32768,
            degree: 2,
            nodes: 1024,
            depth: 128,
            events: 10_000_000,
            modeled_reservations: 64 << 20,
        }
    }
}
#[derive(Clone, Default, Debug, Serialize)]
pub struct Counts {
    pub events: u64,
    pub cumulative_modeled_reservations: u64,
    pub input_terms: u64,
    pub dependency_incidences: u64,
    pub equation_revisions: u64,
    pub term_visits: u64,
    pub selection_scans: u64,
    pub queue_pushes: u64,
    pub field_mul: u64,
    pub field_add_sub: u64,
    pub field_inverse: u64,
    pub field_sqrt: u64,
    pub assignments: u64,
    pub undos: u64,
    pub nodes: u64,
    pub conflicts: u64,
    pub original_checks: u64,
}
#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub enum Stop {
    Invalid(String),
    Cap(String),
    Unsupported(String),
}
type Result<T> = std::result::Result<T, Stop>;
#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub enum Answer {
    Root(Vec<String>),
    NoRootObservation,
    Stopped(Stop),
}
#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct Decision {
    pub depth: usize,
    pub variable: usize,
    pub value: String,
}
#[derive(Debug, Serialize)]
pub struct Report {
    pub answer: Answer,
    pub counts: Counts,
    pub decisions: Vec<Decision>,
    pub schedule: Schedule,
    pub accounting_complete: bool,
}

struct Arithmetic {
    f: Field,
    limits: Limits,
    counts: Counts,
}
impl Arithmetic {
    fn event(&mut self, n: u64) -> Result<()> {
        if n > self.limits.events.saturating_sub(self.counts.events) {
            return Err(Stop::Cap("diagnostic events".into()));
        }
        self.counts.events += n;
        Ok(())
    }
    fn reserve(&mut self, n: u64) -> Result<()> {
        if n > self
            .limits
            .modeled_reservations
            .saturating_sub(self.counts.cumulative_modeled_reservations)
        {
            return Err(Stop::Cap("cumulative modeled reservations".into()));
        }
        self.counts.cumulative_modeled_reservations += n;
        Ok(())
    }
    fn scalar(&mut self) -> Result<()> {
        self.reserve(64 + self.f.p.bits().div_ceil(8))
    }
    fn mul(&mut self, a: &BigUint, b: &BigUint) -> Result<BigUint> {
        self.event(1)?;
        self.scalar()?;
        self.counts.field_mul += 1;
        Ok(self.f.mul(a, b))
    }
    fn add(&mut self, a: &BigUint, b: &BigUint) -> Result<BigUint> {
        self.event(1)?;
        self.scalar()?;
        self.counts.field_add_sub += 1;
        Ok(self.f.add(a, b))
    }
    fn sub(&mut self, a: &BigUint, b: &BigUint) -> Result<BigUint> {
        self.event(1)?;
        self.scalar()?;
        self.counts.field_add_sub += 1;
        Ok(self.f.sub(a, b))
    }
    fn pow(&mut self, a: &BigUint, e: &BigUint) -> Result<BigUint> {
        self.scalar()?;
        let mut r = BigUint::one();
        for i in (0..e.bits()).rev() {
            self.event(1)?;
            r = self.mul(&r, &r)?;
            if e.bit(i) {
                r = self.mul(&r, a)?;
            }
        }
        Ok(r)
    }
    fn inv(&mut self, a: &BigUint) -> Result<BigUint> {
        self.event(1)?;
        self.counts.field_inverse += 1;
        if a.is_zero() {
            return Err(Stop::Invalid("inverse of zero".into()));
        }
        let e = &self.f.p - 2u32;
        let v = if a.is_one() {
            self.scalar()?;
            BigUint::one()
        } else {
            self.pow(a, &e)?
        };
        if !self.mul(a, &v)?.is_one() {
            return Err(Stop::Invalid("field inverse contract".into()));
        }
        Ok(v)
    }
    fn sqrt(&mut self, a: &BigUint) -> Result<Option<BigUint>> {
        self.event(1)?;
        self.counts.field_sqrt += 1;
        if a.is_zero() {
            self.scalar()?;
            return Ok(Some(BigUint::zero()));
        }
        if &self.f.p % 8u32 != BigUint::from(5u32) {
            return Err(Stop::Unsupported("sqrt requires p=5 mod8".into()));
        }
        let mut r = self.pow(a, &((&self.f.p + 3u32) >> 3))?;
        if self.mul(&r, &r)? != *a {
            let i = self.pow(&2u32.into(), &((&self.f.p - 1u32) >> 2))?;
            r = self.mul(&r, &i)?;
        }
        Ok((self.mul(&r, &r)? == *a).then_some(r))
    }
    fn roots(&mut self, c: &[BigUint; 3]) -> Result<Vec<BigUint>> {
        self.reserve(128 + 2 * self.f.p.bits().div_ceil(8))?;
        if c[2].is_zero() {
            let n = self.sub(&BigUint::zero(), &c[0])?;
            let d = self.inv(&c[1])?;
            return Ok(vec![self.mul(&n, &d)?]);
        }
        // Factor x*(a*x+b) directly, including all Boolean equations.
        if c[0].is_zero() {
            let n = self.sub(&BigUint::zero(), &c[1])?;
            let d = self.inv(&c[2])?;
            let r = self.mul(&n, &d)?;
            return Ok(if r.is_zero() {
                vec![r]
            } else {
                vec![BigUint::zero(), r]
            });
        }
        let b2 = self.mul(&c[1], &c[1])?;
        let ac = self.mul(&c[2], &c[0])?;
        let fourac = self.mul(&4u32.into(), &ac)?;
        let disc = self.sub(&b2, &fourac)?;
        let Some(s) = self.sqrt(&disc)? else {
            return Ok(vec![]);
        };
        let den = self.mul(&2u32.into(), &c[2])?;
        let inv = self.inv(&den)?;
        let n = self.sub(&BigUint::zero(), &c[1])?;
        let plus = self.add(&n, &s)?;
        let minus = self.sub(&n, &s)?;
        let x = self.mul(&plus, &inv)?;
        let y = self.mul(&minus, &inv)?;
        Ok(if x == y {
            vec![x]
        } else if x < y {
            vec![x, y]
        } else {
            vec![y, x]
        })
    }
}

#[derive(Clone)]
enum Status {
    Satisfied,
    Conflict,
    Candidate(usize, Vec<BigUint>),
    Blocked,
}
struct Engine<'a> {
    system: &'a System,
    arithmetic: Arithmetic,
    schedule: Schedule,
    allowed: Vec<bool>,
    dependencies: Vec<Vec<usize>>,
    assignment: Vec<Option<BigUint>>,
    trail: Vec<usize>,
    cache: Vec<Option<Status>>,
    queue: VecDeque<usize>,
    queued: Vec<bool>,
    decisions: Vec<Decision>,
}
impl Engine<'_> {
    fn dirty(&mut self, v: usize) -> Result<()> {
        for &e in &self.dependencies[v] {
            self.arithmetic.event(1)?;
            self.cache[e] = None;
            if self.schedule == Schedule::Worklist && !self.queued[e] {
                self.arithmetic.event(1)?;
                self.queue.push_back(e);
                self.queued[e] = true;
                self.arithmetic.counts.queue_pushes += 1;
            }
        }
        Ok(())
    }
    fn assign(&mut self, v: usize, value: BigUint) -> Result<()> {
        self.arithmetic.event(1)?;
        self.arithmetic.scalar()?;
        if self.assignment[v].is_some() {
            return Err(Stop::Invalid("assignment already set".into()));
        }
        self.assignment[v] = Some(value);
        self.trail.push(v);
        self.arithmetic.counts.assignments += 1;
        self.dirty(v)
    }
    fn undo(&mut self, mark: usize) -> Result<()> {
        while self.trail.len() > mark {
            self.arithmetic.event(1)?;
            let v = self.trail.pop().unwrap();
            self.assignment[v] = None;
            self.arithmetic.counts.undos += 1;
            self.dirty(v)?;
        }
        Ok(())
    }
    fn revise(&mut self, e: usize) -> Result<Status> {
        let ctx = &mut self.arithmetic;
        ctx.event(1)?;
        ctx.counts.equation_revisions += 1;
        let mut residual: BTreeMap<Vec<usize>, BigUint> = BTreeMap::new();
        for (m, coefficient) in &self.system.equations[e].terms {
            ctx.event(1)?;
            ctx.counts.term_visits += 1;
            ctx.reserve(96 + 8 * m.len() as u64 + ctx.f.p.bits().div_ceil(8))?;
            let mut c = coefficient.clone();
            let mut unknown = Vec::with_capacity(m.len());
            for &v in m {
                ctx.event(1)?;
                if let Some(x) = &self.assignment[v] {
                    c = ctx.mul(&c, x)?;
                } else {
                    unknown.push(v);
                }
            }
            if !c.is_zero() {
                let a = residual.entry(unknown).or_default();
                *a = ctx.add(a, &c)?;
            }
        }
        residual.retain(|_, c| !c.is_zero());
        if residual.is_empty() {
            return Ok(Status::Satisfied);
        }
        if residual.len() == 1 && residual.keys().next().unwrap().is_empty() {
            return Ok(Status::Conflict);
        }
        let mut variable = None;
        let mut coefficients = [BigUint::zero(), BigUint::zero(), BigUint::zero()];
        for (m, c) in residual {
            for &v in &m {
                ctx.event(1)?;
                if variable.is_some_and(|old| v != old) {
                    return Ok(Status::Blocked);
                }
                variable = Some(v);
            }
            if m.len() > 2 {
                return Ok(Status::Blocked);
            }
            coefficients[m.len()] = c;
        }
        let v = variable.ok_or_else(|| Stop::Invalid("constant residual".into()))?;
        let roots = ctx.roots(&coefficients)?;
        Ok(if roots.is_empty() {
            Status::Conflict
        } else {
            Status::Candidate(v, roots)
        })
    }
    fn refresh(&mut self) -> Result<()> {
        if self.schedule == Schedule::Sweep {
            for e in 0..self.cache.len() {
                self.cache[e] = Some(self.revise(e)?);
            }
        } else {
            while let Some(e) = self.queue.pop_front() {
                self.arithmetic.event(1)?;
                self.queued[e] = false;
                self.cache[e] = Some(self.revise(e)?);
            }
        }
        Ok(())
    }
    fn search(&mut self, depth: usize) -> Result<Option<Vec<BigUint>>> {
        if self.arithmetic.counts.nodes >= self.arithmetic.limits.nodes {
            return Err(Stop::Cap("nodes".into()));
        }
        self.arithmetic.event(1)?;
        self.arithmetic.counts.nodes += 1;
        loop {
            self.refresh()?;
            let mut best: Option<(usize, usize, usize)> = None;
            let mut all_satisfied = true;
            for (e, status) in self.cache.iter().enumerate() {
                self.arithmetic.event(1)?;
                self.arithmetic.counts.selection_scans += 1;
                let status = status
                    .as_ref()
                    .ok_or_else(|| Stop::Invalid("stale residual cache".into()))?;
                match status {
                    Status::Conflict => {
                        self.arithmetic.counts.conflicts += 1;
                        return Ok(None);
                    }
                    Status::Satisfied => {}
                    Status::Blocked => {
                        all_satisfied = false;
                    }
                    Status::Candidate(v, roots) => {
                        all_satisfied = false;
                        if roots.len() == 1 || self.allowed[*v] {
                            let key = (roots.len(), *v, e);
                            if best.is_none_or(|old| key < old) {
                                best = Some(key);
                            }
                        }
                    }
                }
            }
            if all_satisfied {
                self.arithmetic
                    .reserve(64 + 64 * self.assignment.len() as u64)?;
                return Ok(Some(
                    self.assignment
                        .iter()
                        .map(|x| x.clone().unwrap_or_default())
                        .collect(),
                ));
            }
            let Some((_, v, e)) = best else {
                return Err(Stop::Unsupported(
                    "no admitted linear/quadratic input extraction".into(),
                ));
            };
            self.arithmetic.reserve(192)?;
            let Status::Candidate(_, roots) = self.cache[e].as_ref().unwrap().clone() else {
                unreachable!()
            };
            if roots.len() == 1 {
                self.assign(v, roots.into_iter().next().unwrap())?;
                continue;
            }
            if depth >= self.arithmetic.limits.depth {
                return Err(Stop::Cap("depth".into()));
            }
            let mut incomplete = None;
            for value in roots {
                self.arithmetic.event(1)?;
                self.arithmetic.reserve(128)?;
                self.decisions.push(Decision {
                    depth,
                    variable: v,
                    value: value.to_string(),
                });
                let mark = self.trail.len();
                self.assign(v, value)?;
                let answer = self.search(depth + 1);
                match answer {
                    Ok(Some(values)) => return Ok(Some(values)),
                    Ok(None) => {}
                    Err(Stop::Unsupported(s)) => {
                        incomplete = Some(s);
                    }
                    Err(other) => return Err(other),
                }
                self.undo(mark)?;
            }
            return match incomplete {
                Some(s) => Err(Stop::Unsupported(s)),
                None => Ok(None),
            };
        }
    }
}

/// Branch indices are PUBLIC semantic role metadata, never guessed values.
pub fn solve(
    system: &System,
    branch_indices: &[usize],
    schedule: Schedule,
    limits: Limits,
) -> Report {
    let mut ctx = Arithmetic {
        f: Field { p: BigUint::zero() },
        limits,
        counts: Counts::default(),
    };
    let mut decisions = vec![];
    let outcome = (|| -> Result<Option<Vec<BigUint>>> {
        ctx.event(1)?;
        if !(5..=512).contains(&system.p.bits()) || !system.p.bit(0) {
            return Err(Stop::Invalid(
                "odd5..512bit modulus; primality external".into(),
            ));
        }
        let n = system.variables.len();
        let m = system.equations.len();
        if n == 0 {
            return Err(Stop::Invalid("no variables".into()));
        }
        if n > ctx.limits.variables {
            return Err(Stop::Cap("variables".into()));
        }
        if m > ctx.limits.equations {
            return Err(Stop::Cap("equations".into()));
        }
        if branch_indices.len() > n {
            return Err(Stop::Invalid("role length".into()));
        }
        ctx.reserve(128 + 2 * system.p.bits().div_ceil(8) + 128 * n as u64 + 192 * m as u64)?;
        ctx.f.p = system.p.clone();
        let mut allowed = vec![false; n];
        for &v in branch_indices {
            ctx.event(1)?;
            if v >= n || allowed[v] {
                return Err(Stop::Invalid("duplicate/out-of-range role".into()));
            }
            allowed[v] = true;
        }
        let mut dependencies = vec![vec![]; n];
        for (e, p) in system.equations.iter().enumerate() {
            ctx.event(1)?;
            if p.terms.len() as u64
                > (ctx.limits.terms as u64).saturating_sub(ctx.counts.input_terms)
            {
                return Err(Stop::Cap("input terms".into()));
            }
            ctx.counts.input_terms += p.terms.len() as u64;
            let mut support = BTreeSet::new();
            for (monomial, coefficient) in &p.terms {
                ctx.event(1)?;
                if monomial.len() > ctx.limits.degree {
                    return Err(Stop::Cap("degree".into()));
                }
                if coefficient >= &system.p || coefficient.is_zero() {
                    return Err(Stop::Invalid("noncanonical coefficient".into()));
                }
                if !monomial.windows(2).all(|w| w[0] <= w[1]) {
                    return Err(Stop::Invalid("unsorted monomial".into()));
                }
                for &v in monomial {
                    ctx.event(1)?;
                    if v >= n {
                        return Err(Stop::Invalid("variable index".into()));
                    }
                    ctx.reserve(64)?;
                    support.insert(v);
                }
            }
            for v in support {
                ctx.event(1)?;
                ctx.reserve(16)?;
                dependencies[v].push(e);
                ctx.counts.dependency_incidences += 1;
            }
        }
        ctx.event(m as u64)?;
        ctx.counts.queue_pushes = if schedule == Schedule::Worklist {
            m as u64
        } else {
            0
        };
        let placeholder = Arithmetic {
            f: Field { p: BigUint::zero() },
            limits: ctx.limits.clone(),
            counts: Counts::default(),
        };
        let mut engine = Engine {
            system,
            arithmetic: std::mem::replace(&mut ctx, placeholder),
            schedule,
            allowed,
            dependencies,
            assignment: vec![None; n],
            trail: Vec::with_capacity(n),
            cache: vec![None; m],
            queue: (0..m).collect(),
            queued: vec![true; m],
            decisions: vec![],
        };
        let result = engine.search(0);
        decisions = engine.decisions;
        ctx = engine.arithmetic;
        let values = result?;
        if let Some(v) = &values {
            ctx.event(1)?;
            ctx.counts.original_checks += 1;
            if v.iter().any(|x| x >= &system.p) {
                return Err(Stop::Invalid("noncanonical root".into()));
            }
            system.check(v).map_err(Stop::Invalid)?;
        }
        Ok(values)
    })();
    let answer = match outcome {
        Ok(Some(v)) => Answer::Root(v.iter().map(ToString::to_string).collect()),
        Ok(None) => Answer::NoRootObservation,
        Err(s) => Answer::Stopped(s),
    };
    Report {
        answer,
        counts: ctx.counts,
        decisions,
        schedule,
        accounting_complete: false,
    }
}
