//! Bounded, opt-in distinct-root extraction over odd prime fields.
//!
//! The legacy GLV root APIs are unchanged. This implementation uses the same
//! gcd(X^p-X,f) / randomized linear-factor splitting algorithm (Shoup, 20.4).
//! A separate arithmetic checker verifies the certificate without root finding.
//! Counters measure native field operations, including inverse exponentiation;
//! primality validation, serialization, and certificate replay are excluded.
use std::collections::HashMap;

use rand::{rngs::StdRng, Rng, SeedableRng};
use serde::{Deserialize, Serialize};

use super::residual_walk::is_prime_u64;

/// Fixed-storage opt-in API; legacy solve/verify remain unchanged.
#[path = "prime_root_bounded.rs"]
pub mod bounded;

pub const MAX_PRIME: u64 = (1u64 << 61) - 1;
pub const MAX_DEGREE: usize = 8;
pub const MAX_EVENTS: u64 = 100_000;
pub const MAX_WORK: u64 = 1_000_000;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RootLimits {
    pub field_work: u64,
    pub split_attempts: u64,
    pub events: u64,
    pub degree: usize,
}
impl RootLimits {
    fn validate(&self) -> Result<(), String> {
        if self.field_work > MAX_WORK
            || self.events > MAX_EVENTS
            || self.split_attempts > MAX_EVENTS
            || self.degree > MAX_DEGREE
        {
            return Err("root limits exceed hard safety bounds".into());
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RootCounters {
    pub additions: u64,
    pub subtractions: u64,
    pub multiplications: u64,
    pub squares: u64,
    pub inverse_calls: u64,
    pub completed_poly_products: u64,
    pub completed_divisions: u64,
    pub gcd_steps: u64,
    pub split_attempts: u64,
    pub rejected_splits: u64,
}
impl RootCounters {
    pub fn field_work(&self) -> u64 {
        self.additions + self.subtractions + self.multiplications + self.squares
    }
    fn observe(&mut self, event: &RootEvent) {
        match event {
            RootEvent::Field { op, .. } => match op {
                FieldOp::Add => self.additions += 1,
                FieldOp::Sub => self.subtractions += 1,
                FieldOp::Mul => self.multiplications += 1,
                FieldOp::Square => self.squares += 1,
            },
            RootEvent::InverseBegin { .. } => self.inverse_calls += 1,
            RootEvent::Product { .. } => self.completed_poly_products += 1,
            RootEvent::Division { .. } => self.completed_divisions += 1,
            RootEvent::GcdStep { .. } => self.gcd_steps += 1,
            RootEvent::SplitBegin { .. } => self.split_attempts += 1,
            RootEvent::SplitEnd {
                accepted: false, ..
            } => self.rejected_splits += 1,
            _ => {}
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FieldOp {
    Add,
    Sub,
    Mul,
    Square,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum RootEvent {
    Begin {
        p: u64,
        polynomial: Vec<u64>,
        seed: u64,
    },
    Field {
        op: FieldOp,
        a: u64,
        b: u64,
        value: u64,
    },
    InverseBegin {
        value: u64,
    },
    InverseEnd {
        value: u64,
        inverse: u64,
    },
    Product {
        a: Vec<u64>,
        b: Vec<u64>,
        product: Vec<u64>,
    },
    Division {
        a: Vec<u64>,
        b: Vec<u64>,
        quotient: Vec<u64>,
        remainder: Vec<u64>,
    },
    PowerBegin {
        id: u64,
        base: Vec<u64>,
        exponent: u64,
        modulus: Vec<u64>,
    },
    PowerStep {
        id: u64,
        exponent: u64,
        accumulator: Vec<u64>,
        base: Vec<u64>,
    },
    PowerEnd {
        id: u64,
        value: Vec<u64>,
    },
    GcdBegin {
        id: u64,
        a: Vec<u64>,
        b: Vec<u64>,
    },
    GcdStep {
        id: u64,
        quotient: Vec<u64>,
        remainder: Vec<u64>,
    },
    GcdEnd {
        id: u64,
        value: Vec<u64>,
    },
    SplitBegin {
        polynomial: Vec<u64>,
        delta: u64,
    },
    SplitEnd {
        polynomial: Vec<u64>,
        factor: Vec<u64>,
        quotient: Vec<u64>,
        accepted: bool,
    },
    LinearRoot {
        polynomial: Vec<u64>,
        root: u64,
    },
    Finish {
        roots: Vec<u64>,
        xp_mod_f: Vec<u64>,
        linear_part: Vec<u64>,
        all_field_elements: bool,
    },
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RootStop {
    Complete,
    AllFieldElements,
    WorkLimit,
    SplitLimit,
    EventLimit,
    SinkFailure,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RootReport {
    pub schema: String,
    pub p: u64,
    pub polynomial: Vec<u64>,
    pub seed: u64,
    pub limits: RootLimits,
    pub stop: RootStop,
    pub counters: RootCounters,
    /// Only events acknowledged by the sink, in order.
    pub events: Vec<RootEvent>,
    /// A failed sink can leave one computed event unacknowledged. Its operations
    /// remain charged, but it is not part of the durable acknowledged prefix.
    pub unacknowledged: Option<RootEvent>,
    pub sink_error: Option<String>,
    pub roots: Vec<u64>,
}
impl RootReport {
    pub fn complete(&self) -> bool {
        matches!(self.stop, RootStop::Complete | RootStop::AllFieldElements)
    }
}
fn trim(mut v: Vec<u64>) -> Vec<u64> {
    while v.last() == Some(&0) {
        v.pop();
    }
    v
}
fn validate_input(f: &[u64], p: u64, limits: &RootLimits) -> Result<(), String> {
    limits.validate()?;
    if !(3..=MAX_PRIME).contains(&p) || !is_prime_u64(p) {
        return Err("root field must be an odd prime at most 2^61-1".into());
    }
    if f.len() > limits.degree + 1 || f.iter().any(|&c| c >= p) {
        return Err("noncanonical or oversized root polynomial".into());
    }
    Ok(())
}
struct Engine<'a> {
    report: RootReport,
    sink: &'a mut dyn FnMut(&RootEvent) -> Result<(), String>,
    next_id: u64,
}
type Work<T> = Result<T, RootStop>;
impl Engine<'_> {
    fn room(&self) -> Work<()> {
        if self.report.events.len() as u64 >= self.report.limits.events {
            Err(RootStop::EventLimit)
        } else {
            Ok(())
        }
    }
    fn event(&mut self, event: RootEvent) -> Work<()> {
        self.room()?;
        self.report.counters.observe(&event);
        if let Err(error) = (self.sink)(&event) {
            self.report.unacknowledged = Some(event);
            self.report.sink_error = Some(error);
            return Err(RootStop::SinkFailure);
        }
        self.report.events.push(event);
        Ok(())
    }
    fn field(&mut self, op: FieldOp, a: u64, b: u64) -> Work<u64> {
        self.room()?;
        if self.report.counters.field_work() >= self.report.limits.field_work {
            return Err(RootStop::WorkLimit);
        }
        let p = self.report.p;
        let value = match op {
            FieldOp::Add => ((a as u128 + b as u128) % p as u128) as u64,
            FieldOp::Sub => ((a as u128 + p as u128 - b as u128) % p as u128) as u64,
            FieldOp::Mul | FieldOp::Square => ((a as u128 * b as u128) % p as u128) as u64,
        };
        self.event(RootEvent::Field { op, a, b, value })?;
        Ok(value)
    }
    fn inv(&mut self, value: u64) -> Work<u64> {
        debug_assert_ne!(value, 0);
        self.event(RootEvent::InverseBegin { value })?;
        if value == 1 {
            self.event(RootEvent::InverseEnd { value, inverse: 1 })?;
            return Ok(1);
        }
        let mut exponent = self.report.p - 2;
        let (mut acc, mut base) = (1, value);
        while exponent > 0 {
            if exponent & 1 == 1 {
                acc = self.field(FieldOp::Mul, acc, base)?;
            }
            exponent >>= 1;
            if exponent != 0 {
                base = self.field(FieldOp::Square, base, base)?;
            }
        }
        self.event(RootEvent::InverseEnd {
            value,
            inverse: acc,
        })?;
        Ok(acc)
    }
    fn mul(&mut self, a: &[u64], b: &[u64]) -> Work<Vec<u64>> {
        let mut product = if a.is_empty() || b.is_empty() {
            vec![]
        } else {
            vec![0; a.len() + b.len() - 1]
        };
        for (i, &x) in a.iter().enumerate() {
            if x == 0 {
                continue;
            }
            for (j, &y) in b.iter().enumerate() {
                let z = self.field(FieldOp::Mul, x, y)?;
                product[i + j] = self.field(FieldOp::Add, product[i + j], z)?;
            }
        }
        let product = trim(product);
        self.event(RootEvent::Product {
            a: a.to_vec(),
            b: b.to_vec(),
            product: product.clone(),
        })?;
        Ok(product)
    }
    fn div(&mut self, a: &[u64], b: &[u64]) -> Work<(Vec<u64>, Vec<u64>)> {
        debug_assert!(!b.is_empty());
        let mut r = a.to_vec();
        let mut q = vec![0; a.len().saturating_sub(b.len()) + 1];
        if a.len() >= b.len() {
            let inv = self.inv(*b.last().unwrap())?;
            while r.len() >= b.len() {
                let shift = r.len() - b.len();
                let c = self.field(FieldOp::Mul, *r.last().unwrap(), inv)?;
                q[shift] = c;
                for (k, &v) in b.iter().enumerate() {
                    let product = self.field(FieldOp::Mul, c, v)?;
                    r[shift + k] = self.field(FieldOp::Sub, r[shift + k], product)?;
                }
                r = trim(r);
            }
        }
        let q = trim(q);
        self.event(RootEvent::Division {
            a: a.to_vec(),
            b: b.to_vec(),
            quotient: q.clone(),
            remainder: r.clone(),
        })?;
        Ok((q, r))
    }
    fn pow(&mut self, base: &[u64], mut exponent: u64, modulus: &[u64]) -> Work<Vec<u64>> {
        let id = self.next_id;
        self.next_id += 1;
        self.event(RootEvent::PowerBegin {
            id,
            base: base.to_vec(),
            exponent,
            modulus: modulus.to_vec(),
        })?;
        let mut b = self.div(base, modulus)?.1;
        let mut acc = self.div(&[1], modulus)?.1;
        while exponent != 0 {
            if exponent & 1 == 1 {
                let product = self.mul(&acc, &b)?;
                acc = self.div(&product, modulus)?.1;
            }
            let product = self.mul(&b, &b)?;
            b = self.div(&product, modulus)?.1;
            exponent >>= 1;
            self.event(RootEvent::PowerStep {
                id,
                exponent,
                accumulator: acc.clone(),
                base: b.clone(),
            })?;
        }
        self.event(RootEvent::PowerEnd {
            id,
            value: acc.clone(),
        })?;
        Ok(acc)
    }
    fn gcd(&mut self, a: &[u64], b: &[u64]) -> Work<Vec<u64>> {
        let id = self.next_id;
        self.next_id += 1;
        self.event(RootEvent::GcdBegin {
            id,
            a: a.to_vec(),
            b: b.to_vec(),
        })?;
        let (mut a, mut b) = (a.to_vec(), b.to_vec());
        while !b.is_empty() {
            let (q, r) = self.div(&a, &b)?;
            self.event(RootEvent::GcdStep {
                id,
                quotient: q,
                remainder: r.clone(),
            })?;
            a = b;
            b = r;
        }
        if let Some(&lead) = a.last() {
            let inv = self.inv(lead)?;
            for c in &mut a {
                *c = self.field(FieldOp::Mul, *c, inv)?;
            }
        }
        self.event(RootEvent::GcdEnd {
            id,
            value: a.clone(),
        })?;
        Ok(a)
    }
    fn run(&mut self) -> Work<RootStop> {
        let f = self.report.polynomial.clone();
        let p = self.report.p;
        self.event(RootEvent::Begin {
            p,
            polynomial: f.clone(),
            seed: self.report.seed,
        })?;
        if f.is_empty() {
            self.event(RootEvent::Finish {
                roots: vec![],
                xp_mod_f: vec![],
                linear_part: vec![],
                all_field_elements: true,
            })?;
            return Ok(RootStop::AllFieldElements);
        }
        let xp = self.pow(&[0, 1], p, &f)?;
        let mut xpx = xp.clone();
        xpx.resize(xpx.len().max(2), 0);
        xpx[1] = self.field(FieldOp::Sub, xpx[1], 1)?;
        let g = self.gcd(&f, &trim(xpx))?;
        let mut stack = vec![g.clone()];
        let mut rng = StdRng::seed_from_u64(self.report.seed ^ 0x524f_4f54);
        let mut roots = vec![];
        while let Some(h) = stack.pop() {
            match h.len() {
                0 | 1 => {}
                2 => {
                    let inv = self.inv(h[1])?;
                    let product = self.field(FieldOp::Mul, h[0], inv)?;
                    let root = self.field(FieldOp::Sub, 0, product)?;
                    self.event(RootEvent::LinearRoot {
                        polynomial: h,
                        root,
                    })?;
                    roots.push(root);
                }
                _ => {
                    if self.report.counters.split_attempts >= self.report.limits.split_attempts {
                        return Err(RootStop::SplitLimit);
                    }
                    let delta = rng.gen_range(0..p);
                    self.event(RootEvent::SplitBegin {
                        polynomial: h.clone(),
                        delta,
                    })?;
                    let mut s = self.pow(&[delta, 1], (p - 1) / 2, &h)?;
                    if s.is_empty() {
                        s.push(0);
                    }
                    s[0] = self.field(FieldOp::Sub, s[0], 1)?;
                    let d = self.gcd(&h, &trim(s))?;
                    let accepted = d.len() > 1 && d.len() < h.len();
                    let q = if accepted {
                        self.div(&h, &d)?.0
                    } else {
                        vec![]
                    };
                    self.event(RootEvent::SplitEnd {
                        polynomial: h.clone(),
                        factor: d.clone(),
                        quotient: q.clone(),
                        accepted,
                    })?;
                    if accepted {
                        stack.push(d);
                        stack.push(q);
                    } else {
                        stack.push(h);
                    }
                }
            }
        }
        roots.sort_unstable();
        roots.dedup();
        self.event(RootEvent::Finish {
            roots: roots.clone(),
            xp_mod_f: xp,
            linear_part: g,
            all_field_elements: false,
        })?;
        self.report.roots = roots;
        Ok(RootStop::Complete)
    }
}

pub fn solve(
    f: &[u64],
    p: u64,
    seed: u64,
    limits: RootLimits,
    sink: &mut dyn FnMut(&RootEvent) -> Result<(), String>,
) -> Result<RootReport, String> {
    validate_input(f, p, &limits)?;
    let mut engine = Engine {
        report: RootReport {
            schema: "prime-root-trace/v1".into(),
            p,
            polynomial: trim(f.to_vec()),
            seed,
            limits,
            stop: RootStop::WorkLimit,
            counters: RootCounters::default(),
            events: vec![],
            unacknowledged: None,
            sink_error: None,
            roots: vec![],
        },
        sink,
        next_id: 0,
    };
    engine.report.stop = match engine.run() {
        Ok(stop) | Err(stop) => stop,
    };
    Ok(engine.report)
}

// Independent verifier arithmetic. This never invokes Engine or solve and does
// not search for roots: only modular powering, Euclidean identities, and products.
fn vmul(a: u64, b: u64, p: u64) -> u64 {
    ((a as u128 * b as u128) % p as u128) as u64
}
fn vpow(mut a: u64, mut e: u64, p: u64) -> u64 {
    let mut r = 1;
    while e > 0 {
        if e & 1 == 1 {
            r = vmul(r, a, p);
        }
        a = vmul(a, a, p);
        e >>= 1;
    }
    r
}
fn vproduct(a: &[u64], b: &[u64], p: u64) -> Vec<u64> {
    if a.is_empty() || b.is_empty() {
        return vec![];
    }
    let mut r = vec![0; a.len() + b.len() - 1];
    for (i, &x) in a.iter().enumerate() {
        for (j, &y) in b.iter().enumerate() {
            r[i + j] = ((r[i + j] as u128 + x as u128 * y as u128) % p as u128) as u64;
        }
    }
    trim(r)
}
fn vdiv(a: &[u64], b: &[u64], p: u64) -> Result<(Vec<u64>, Vec<u64>), String> {
    if b.is_empty() {
        return Err("division by zero polynomial".into());
    }
    let mut r = a.to_vec();
    let mut q = vec![0; a.len().saturating_sub(b.len()) + 1];
    let inv = vpow(*b.last().unwrap(), p - 2, p);
    while !r.is_empty() && r.len() >= b.len() {
        let shift = r.len() - b.len();
        let c = vmul(*r.last().unwrap(), inv, p);
        q[shift] = c;
        for (i, &x) in b.iter().enumerate() {
            r[shift + i] =
                ((r[shift + i] as u128 + p as u128 - vmul(c, x, p) as u128) % p as u128) as u64;
        }
        r = trim(r);
    }
    Ok((trim(q), r))
}
fn vpower(base: &[u64], mut e: u64, m: &[u64], p: u64) -> Result<Vec<u64>, String> {
    let mut r = vdiv(&[1], m, p)?.1;
    let mut b = vdiv(base, m, p)?.1;
    while e > 0 {
        if e & 1 == 1 {
            r = vdiv(&vproduct(&r, &b, p), m, p)?.1;
        }
        b = vdiv(&vproduct(&b, &b, p), m, p)?.1;
        e >>= 1;
    }
    Ok(r)
}
fn vmonic(mut a: Vec<u64>, p: u64) -> Vec<u64> {
    if let Some(&lead) = a.last() {
        let inv = vpow(lead, p - 2, p);
        for c in &mut a {
            *c = vmul(*c, inv, p);
        }
    }
    a
}
fn vgcd(mut a: Vec<u64>, mut b: Vec<u64>, p: u64) -> Result<Vec<u64>, String> {
    while !b.is_empty() {
        let r = vdiv(&a, &b, p)?.1;
        a = b;
        b = r;
    }
    Ok(vmonic(a, p))
}
fn canonical(poly: &[u64], p: u64) -> Result<(), String> {
    if poly.len() > 2 * MAX_DEGREE + 3 || poly.last() == Some(&0) || poly.iter().any(|&x| x >= p) {
        return Err("noncanonical trace polynomial".into());
    }
    Ok(())
}
#[derive(Clone)]
struct PowerState {
    acc: Vec<u64>,
    base: Vec<u64>,
    exponent: u64,
    modulus: Vec<u64>,
}

/// Checks each supplied arithmetic state, the terminal distinct-root certificate,
/// canonical inputs, recorded-event counter tallies and caps. This is not a proof
/// that every operation in a native execution was recorded. Incomplete reports
/// never prove absence; complete root sets do not certify complete cost traces.
pub fn verify(report: &RootReport) -> Result<(), String> {
    validate_input(&report.polynomial, report.p, &report.limits)?;
    canonical(&report.polynomial, report.p)?;
    if report.schema != "prime-root-trace/v1"
        || report.events.len() as u64 + u64::from(report.unacknowledged.is_some())
            > report.limits.events
        || report.roots.len() > MAX_DEGREE
    {
        return Err("invalid root trace schema or event bound".into());
    }
    if (report.stop == RootStop::SinkFailure)
        != (report.unacknowledged.is_some() && report.sink_error.is_some())
        || (report.stop != RootStop::SinkFailure
            && (report.unacknowledged.is_some() || report.sink_error.is_some()))
    {
        return Err("inconsistent failed-sink state".into());
    }
    let p = report.p;
    let mut counters = RootCounters::default();
    let mut powers: HashMap<u64, PowerState> = HashMap::new();
    let mut gcds: HashMap<u64, (Vec<u64>, Vec<u64>)> = HashMap::new();
    let mut next_id = 0;
    let mut inverse = None;
    let mut split: Option<(Vec<u64>, u64)> = None;
    let mut rng = StdRng::seed_from_u64(report.seed ^ 0x524f_4f54);
    let mut finished = false;
    for (index, event) in report
        .events
        .iter()
        .chain(report.unacknowledged.iter())
        .enumerate()
    {
        if finished {
            return Err("events after root finish".into());
        }
        counters.observe(event);
        if index == 0
            && *event
                != (RootEvent::Begin {
                    p,
                    polynomial: report.polynomial.clone(),
                    seed: report.seed,
                })
        {
            return Err("root trace header differs".into());
        }
        match event {
            RootEvent::Begin { .. } => {
                if index != 0 {
                    return Err("repeated root header".into());
                }
            }
            RootEvent::Field { op, a, b, value } => {
                if *a >= p || *b >= p || *value >= p {
                    return Err("noncanonical field operation".into());
                }
                let expected = match op {
                    FieldOp::Add => ((*a as u128 + *b as u128) % p as u128) as u64,
                    FieldOp::Sub => ((*a as u128 + p as u128 - *b as u128) % p as u128) as u64,
                    FieldOp::Mul => vmul(*a, *b, p),
                    FieldOp::Square => {
                        if a != b {
                            return Err("square operands differ".into());
                        }
                        vmul(*a, *a, p)
                    }
                };
                if *value != expected {
                    return Err("field operation identity failed".into());
                }
            }
            RootEvent::InverseBegin { value } => {
                if *value == 0 || *value >= p || inverse.replace(*value).is_some() {
                    return Err("invalid inverse begin".into());
                }
            }
            RootEvent::InverseEnd {
                value,
                inverse: inv,
            } => {
                if inverse.take() != Some(*value) || *inv >= p || vmul(*value, *inv, p) != 1 {
                    return Err("inverse identity failed".into());
                }
            }
            RootEvent::Product { a, b, product } => {
                for v in [a, b, product] {
                    canonical(v, p)?;
                }
                if vproduct(a, b, p) != *product {
                    return Err("polynomial product identity failed".into());
                }
            }
            RootEvent::Division {
                a,
                b,
                quotient,
                remainder,
            } => {
                for v in [a, b, quotient, remainder] {
                    canonical(v, p)?;
                }
                if vdiv(a, b, p)? != (quotient.clone(), remainder.clone()) {
                    return Err("division identity failed".into());
                }
            }
            RootEvent::PowerBegin {
                id,
                base,
                exponent,
                modulus,
            } => {
                canonical(base, p)?;
                canonical(modulus, p)?;
                if *id != next_id {
                    return Err("nonconsecutive computation id".into());
                }
                next_id += 1;
                powers.insert(
                    *id,
                    PowerState {
                        acc: vdiv(&[1], modulus, p)?.1,
                        base: vdiv(base, modulus, p)?.1,
                        exponent: *exponent,
                        modulus: modulus.clone(),
                    },
                );
            }
            RootEvent::PowerStep {
                id,
                exponent,
                accumulator,
                base,
            } => {
                canonical(accumulator, p)?;
                canonical(base, p)?;
                let state = powers.get_mut(id).ok_or("missing power begin")?;
                if state.exponent == 0 {
                    return Err("power continued after completion".into());
                }
                if state.exponent & 1 == 1 {
                    state.acc = vdiv(&vproduct(&state.acc, &state.base, p), &state.modulus, p)?.1;
                }
                state.base = vdiv(&vproduct(&state.base, &state.base, p), &state.modulus, p)?.1;
                state.exponent >>= 1;
                if (*exponent, accumulator, base) != (state.exponent, &state.acc, &state.base) {
                    return Err("modular power step failed".into());
                }
            }
            RootEvent::PowerEnd { id, value } => {
                canonical(value, p)?;
                let state = powers.remove(id).ok_or("missing power begin")?;
                if state.exponent != 0 || state.acc != *value {
                    return Err("incomplete power chain".into());
                }
            }
            RootEvent::GcdBegin { id, a, b } => {
                canonical(a, p)?;
                canonical(b, p)?;
                if *id != next_id {
                    return Err("nonconsecutive computation id".into());
                }
                next_id += 1;
                gcds.insert(*id, (a.clone(), b.clone()));
            }
            RootEvent::GcdStep {
                id,
                quotient,
                remainder,
            } => {
                canonical(quotient, p)?;
                canonical(remainder, p)?;
                let state = gcds.get_mut(id).ok_or("missing gcd begin")?;
                if vdiv(&state.0, &state.1, p)? != (quotient.clone(), remainder.clone()) {
                    return Err("Euclidean step failed".into());
                }
                *state = (state.1.clone(), remainder.clone());
            }
            RootEvent::GcdEnd { id, value } => {
                canonical(value, p)?;
                let (a, b) = gcds.remove(id).ok_or("missing gcd begin")?;
                if !b.is_empty() || vmonic(a, p) != *value {
                    return Err("incomplete Euclidean chain".into());
                }
            }
            RootEvent::SplitBegin { polynomial, delta } => {
                canonical(polynomial, p)?;
                if polynomial.len() < 3
                    || *delta != rng.gen_range(0..p)
                    || split.replace((polynomial.clone(), *delta)).is_some()
                {
                    return Err("invalid split proposal or RNG stream".into());
                }
            }
            RootEvent::SplitEnd {
                polynomial,
                factor,
                quotient,
                accepted,
            } => {
                for v in [polynomial, factor, quotient] {
                    canonical(v, p)?;
                }
                let (h, delta) = split.take().ok_or("missing split begin")?;
                let mut s = vpower(&[delta, 1], (p - 1) / 2, &h, p)?;
                if s.is_empty() {
                    s.push(0);
                }
                s[0] = (s[0] + p - 1) % p;
                let g = vgcd(h.clone(), trim(s), p)?;
                let valid = g.len() > 1 && g.len() < h.len();
                if h != *polynomial
                    || g != *factor
                    || valid != *accepted
                    || (valid && vdiv(&h, &g, p)? != (quotient.clone(), vec![]))
                    || (!valid && !quotient.is_empty())
                {
                    return Err("split identity failed".into());
                }
            }
            RootEvent::LinearRoot { polynomial, root } => {
                canonical(polynomial, p)?;
                if polynomial.len() != 2
                    || *root >= p
                    || !(vmul(polynomial[1], *root, p) + polynomial[0]).is_multiple_of(p)
                {
                    return Err("linear root identity failed".into());
                }
            }
            RootEvent::Finish {
                roots,
                xp_mod_f,
                linear_part,
                all_field_elements,
            } => {
                if roots.len() > MAX_DEGREE {
                    return Err("oversized root certificate".into());
                }
                for v in [xp_mod_f, linear_part] {
                    canonical(v, p)?;
                }
                if !powers.is_empty() || !gcds.is_empty() || inverse.is_some() || split.is_some() {
                    return Err("unfinished native computation".into());
                }
                let acknowledged = index < report.events.len();
                if acknowledged {
                    if !report.complete() || roots != &report.roots {
                        return Err("finish disagrees with root report".into());
                    }
                } else if report.stop != RootStop::SinkFailure {
                    return Err("unacknowledged finish is not completed".into());
                }
                if *all_field_elements {
                    if !report.polynomial.is_empty()
                        || !roots.is_empty()
                        || !xp_mod_f.is_empty()
                        || !linear_part.is_empty()
                        || (acknowledged && report.stop != RootStop::AllFieldElements)
                    {
                        return Err("invalid all-field certificate".into());
                    }
                } else {
                    if report.polynomial.is_empty()
                        || (acknowledged && report.stop != RootStop::Complete)
                    {
                        return Err("invalid finite-root certificate".into());
                    }
                    let xp = vpower(&[0, 1], p, &report.polynomial, p)?;
                    if xp != *xp_mod_f {
                        return Err("X^p residue certificate failed".into());
                    }
                    let mut xm = xp;
                    xm.resize(xm.len().max(2), 0);
                    xm[1] = (xm[1] + p - 1) % p;
                    let g = vgcd(report.polynomial.clone(), trim(xm), p)?;
                    let mut product = vec![1];
                    let mut previous = None;
                    for &r in roots {
                        if r >= p || previous.is_some_and(|a| a >= r) {
                            return Err("noncanonical or repeated root".into());
                        }
                        previous = Some(r);
                        product = vproduct(&product, &[(p - r) % p, 1], p);
                    }
                    if g != *linear_part || product != g {
                        return Err("distinct-root completeness failed".into());
                    }
                }
                finished = true;
            }
        }
    }
    if counters != report.counters
        || counters.field_work() > report.limits.field_work
        || counters.split_attempts > report.limits.split_attempts
    {
        return Err("root counters or limits differ".into());
    }
    if report.complete() != matches!(report.events.last(), Some(RootEvent::Finish { .. })) {
        return Err("missing acknowledged completion certificate".into());
    }
    if !report.complete() && !report.roots.is_empty() {
        return Err("censored report claims final roots".into());
    }
    match report.stop {
        RootStop::EventLimit if report.events.len() as u64 != report.limits.events => {
            return Err("event cap not reached".into())
        }
        RootStop::WorkLimit if counters.field_work() != report.limits.field_work => {
            return Err("work cap not reached".into())
        }
        RootStop::SplitLimit if counters.split_attempts != report.limits.split_attempts => {
            return Err("split cap not reached".into())
        }
        _ => {}
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    fn limits() -> RootLimits {
        RootLimits {
            field_work: MAX_WORK,
            split_attempts: 1000,
            events: MAX_EVENTS,
            degree: MAX_DEGREE,
        }
    }
    fn run(f: &[u64], p: u64) -> RootReport {
        solve(f, p, 7, limits(), &mut |_| Ok(())).unwrap()
    }
    fn evaluate(f: &[u64], x: u64, p: u64) -> u64 {
        f.iter().rev().fold(0, |a, &b| {
            ((a as u128 * x as u128 + b as u128) % p as u128) as u64
        })
    }
    #[test]
    fn exhaustive_quadratics_over_f7_match_direct_evaluation() {
        for a in 0..7 {
            for b in 0..7 {
                for c in 0..7 {
                    let r = run(&[a, b, c], 7);
                    verify(&r).unwrap();
                    if a | b | c == 0 {
                        assert_eq!(r.stop, RootStop::AllFieldElements);
                    } else {
                        assert_eq!(r.stop, RootStop::Complete);
                        assert_eq!(
                            r.roots,
                            (0..7)
                                .filter(|&x| evaluate(&[a, b, c], x, 7) == 0)
                                .collect::<Vec<_>>()
                        );
                    }
                }
            }
        }
    }
    #[test]
    fn repeated_roots_irreducibles_and_degenerate_polynomials() {
        for (f, p, roots) in [
            (vec![], 13, vec![]),
            (vec![9], 13, vec![]),
            (vec![1, 1], 13, vec![12]),
            (vec![1, 0, 1], 7, vec![]),
            (vec![1, 0, 1], 13, vec![5, 8]),
            (vec![4, 9, 1], 13, vec![2]),
        ] {
            let r = run(&f, p);
            verify(&r).unwrap();
            assert_eq!(r.roots, roots);
        }
        let f = vproduct(&[1, 0, 1], &[4, 3, 1], 7);
        let r = run(&f, 7);
        verify(&r).unwrap();
        assert_eq!(r.roots, vec![2]);
    }
    #[test]
    fn constructed_split_polynomial_at_prime61() {
        let p = MAX_PRIME;
        let f = vproduct(&[p - 2, 1], &[p - 17, 1], p);
        let r = run(&f, p);
        verify(&r).unwrap();
        assert_eq!(r.roots, vec![2, 17]);
        assert!(
            r.counters.inverse_calls > 0
                && r.counters.squares > 0
                && r.counters.multiplications > 0
        );
    }
    #[test]
    fn certificate_tampering_rejected() {
        let good = run(&[1, 0, 1], 13);
        verify(&good).unwrap();
        let mut r = good.clone();
        r.p = 17;
        assert!(verify(&r).is_err());
        let mut r = good.clone();
        r.events.pop();
        assert!(verify(&r).is_err());
        let mut r = good.clone();
        r.roots.pop();
        assert!(verify(&r).is_err());
        let mut r = good.clone();
        r.counters.multiplications += 1;
        assert!(verify(&r).is_err());
        let mut r = good.clone();
        if let Some(RootEvent::Field { value, .. }) = r
            .events
            .iter_mut()
            .find(|e| matches!(e, RootEvent::Field { .. }))
        {
            *value = (*value + 1) % 13;
        }
        assert!(verify(&r).is_err());
        let mut r = good.clone();
        if let Some(RootEvent::GcdStep { remainder, .. }) = r
            .events
            .iter_mut()
            .find(|e| matches!(e, RootEvent::GcdStep { .. }))
        {
            *remainder = vec![12];
        }
        assert!(verify(&r).is_err());
        let mut r = good;
        if let Some(RootEvent::Finish { roots, .. }) = r.events.last_mut() {
            roots.pop();
            r.roots = roots.clone();
        }
        assert!(verify(&r).is_err());
    }
    #[test]
    fn exact_caps_and_failed_sink_keep_censored_prefix() {
        for work in [0, 1, 20] {
            let mut l = limits();
            l.field_work = work;
            let r = solve(&[1, 0, 1], 13, 7, l, &mut |_| Ok(())).unwrap();
            assert_eq!(r.stop, RootStop::WorkLimit);
            verify(&r).unwrap();
        }
        for events in [0, 1, 10] {
            let mut l = limits();
            l.events = events;
            let r = solve(&[1, 0, 1], 13, 7, l, &mut |_| Ok(())).unwrap();
            assert_eq!(r.stop, RootStop::EventLimit);
            verify(&r).unwrap();
        }
        let mut l = limits();
        l.split_attempts = 0;
        let r = solve(&[1, 0, 1], 13, 7, l, &mut |_| Ok(())).unwrap();
        assert_eq!(r.stop, RootStop::SplitLimit);
        verify(&r).unwrap();
        let mut seen = 0;
        let r = solve(&[1, 0, 1], 13, 7, limits(), &mut |_| {
            seen += 1;
            if seen == 17 {
                Err("fixture sink failure".into())
            } else {
                Ok(())
            }
        })
        .unwrap();
        assert_eq!(seen, 17);
        assert_eq!(r.events.len(), 16);
        assert_eq!(r.stop, RootStop::SinkFailure);
        verify(&r).unwrap();
        let mut forged = r;
        forged.stop = RootStop::Complete;
        assert!(verify(&forged).is_err());
    }
    #[test]
    fn invalid_fields_and_inputs_fail_before_sink() {
        for p in [2, 15, 341_550_071_728_321, MAX_PRIME + 2] {
            assert!(solve(&[1, 1], p, 1, limits(), &mut |_| panic!(
                "invalid input reached sink"
            ))
            .is_err());
        }
        assert!(solve(&[13, 1], 13, 1, limits(), &mut |_| Ok(())).is_err());
        assert!(solve(&[1; MAX_DEGREE + 2], 13, 1, limits(), &mut |_| Ok(())).is_err());
    }
    #[test]
    fn root_certificate_is_not_a_complete_operation_trajectory_proof() {
        let mut report = run(&[1, 0, 1], 13);
        report
            .events
            .retain(|e| !matches!(e, RootEvent::Field { .. }));
        report.counters = RootCounters::default();
        for event in &report.events {
            report.counters.observe(event);
        }
        // The root-set proof still holds. This deliberately demonstrates why
        // dataset artifacts mark independent execution/cost completeness false.
        verify(&report).unwrap();
        assert_eq!(report.counters.field_work(), 0);
    }
    #[test]
    fn oversized_root_lists_and_impossible_unacknowledged_event_rejected() {
        let mut report = run(&[1, 1], 13);
        report.roots = vec![1; MAX_DEGREE + 1];
        assert!(verify(&report).is_err());
        let mut report = run(&[1, 1], 13);
        if let Some(RootEvent::Finish { roots, .. }) = report.events.last_mut() {
            *roots = vec![1; MAX_DEGREE + 1];
        }
        assert!(verify(&report).is_err());
        let mut count = 0;
        let mut report = solve(&[1, 1], 13, 7, limits(), &mut |_| {
            count += 1;
            if count == 5 {
                Err("sink".into())
            } else {
                Ok(())
            }
        })
        .unwrap();
        report.limits.events = report.events.len() as u64;
        assert!(verify(&report).is_err());
    }
}
