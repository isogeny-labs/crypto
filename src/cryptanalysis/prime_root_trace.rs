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
/// Accelerated arithmetic for canonical public field elements emitted by
/// the trace engine. Keep the general u128 fallback so event values remain
/// identical even for noncanonical internal callers.
#[inline]
fn field_value(op: &FieldOp, a: u64, b: u64, p: u64) -> u64 {
    match op {
        FieldOp::Add => {
            if a < p && b < p {
                let sum = a + b; // p <= 2^61-1: no u64 overflow
                if sum >= p { sum - p } else { sum }
            } else {
                ((a as u128 + b as u128) % p as u128) as u64
            }
        }
        FieldOp::Sub => {
            if a < p && b < p {
                if a >= b { a - b } else { a + p - b }
            } else {
                ((a as u128 + p as u128 - b as u128) % p as u128) as u64
            }
        }
        FieldOp::Mul | FieldOp::Square => {
            if p == MAX_PRIME && a < p && b < p {
                // 2^61 = 1 mod p: end-around carry rather than u128 divide.
                let product = a as u128 * b as u128;
                let folded = ((product as u64) & MAX_PRIME) + (product >> 61) as u64;
                let folded = (folded & MAX_PRIME) + (folded >> 61);
                if folded >= MAX_PRIME { folded - MAX_PRIME } else { folded }
            } else if p <= u32::MAX as u64 && a < p && b < p {
                (a * b) % p
            } else {
                ((a as u128 * b as u128) % p as u128) as u64
            }
        }
    }
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
        let value = field_value(&op, a, b, self.report.p);
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
#[inline]
fn vmul(a: u64, b: u64, p: u64) -> u64 {
    // This verifier remains independent of the Engine's field arithmetic.
    // The trace enforces canonical coefficients, but keep the wide fallback
    // for standalone verifier helpers and malformed public inputs.
    if p == MAX_PRIME && a < p && b < p {
        let product = a as u128 * b as u128;
        let folded = ((product as u64) & MAX_PRIME) + (product >> 61) as u64;
        let folded = (folded & MAX_PRIME) + (folded >> 61);
        if folded >= p { folded - p } else { folded }
    } else if p <= u32::MAX as u64 && a < p && b < p {
        (a * b) % p
    } else {
        ((a as u128 * b as u128) % p as u128) as u64
    }
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
// Independent extended Euclid inversion for public root-certificate inputs.
// p is an odd prime bounded by 2^61-1; the signed Bezout coefficients remain
// within the modulus and fit i64. Variable-time: NOT for secret scalar data.
// Keep the original Fermat vpow above as a frozen cross-check reference.
#[inline]
fn vinv(a: u64, p: u64) -> u64 {
    let mut r0 = p;
    let mut r1 = if a < p { a } else { a % p };
    if r1 == 0 {
        return 0; // preserve vpow(0, p-2, p) behavior
    }
    let (mut t0, mut t1) = (0i64, 1i64);
    while r1 != 0 {
        let q = r0 / r1;
        let r2 = r0 - q * r1;
        let t2 = t0 - (q as i64) * t1;
        r0 = r1;
        r1 = r2;
        t0 = t1;
        t1 = t2;
    }
    if t0 < 0 {
        (t0 + p as i64) as u64
    } else {
        t0 as u64
    }
}
// Recognize multiplication by 1 or a monic power of x, common when
// replaying polynomial root certificates. Restrict the shortcut to monic
// operands: canonical coefficient copying is then sufficient.
fn vmonomial_product(m: &[u64], other: &[u64]) -> Option<Vec<u64>> {
    if m.len() == 1 && m[0] == 1 {
        return Some(trim(other.to_vec()));
    }
    if m.len() < 2 || m[0] != 0 || *m.last().unwrap() != 1
        || m[1..m.len()-1].iter().any(|&x| x != 0)
    {
        return None;
    }
    let shift = m.len() - 1;
    let mut result = vec![0; shift + other.len()];
    result[shift..].copy_from_slice(other);
    Some(trim(result))
}
fn vproduct(a: &[u64], b: &[u64], p: u64) -> Vec<u64> {
    if a.is_empty() || b.is_empty() {
        return vec![];
    }
    // Scaling by a non-unit constant is a single coefficient pass, not
    // polynomial convolution. Preserve monic/zero scalar shortcuts.
    if a.len() == 1 || b.len() == 1 {
        let (scalar, other) = if a.len() == 1 { (a[0], b) } else { (b[0], a) };
        if scalar == 0 { return vec![]; }
        if scalar == 1 { return trim(other.to_vec()); }
        return trim(other.iter().map(|&x| vmul(x, scalar, p)).collect());
    }
    if let Some(result) = vmonomial_product(a, b) { return result; }
    if let Some(result) = vmonomial_product(b, a) { return result; }
    // A wide dense convolution can accumulate each output coefficient in u64
    // and perform ONE modular reduction per coefficient, not one per pair.
    // The bound is checked in u128, so no intermediate u64 overflow is possible.
    // Keep the sparse and small-product paths untouched: their zero skipping
    // beats the extra column and density checks on typical short certificates.
    if a.len() >= 8 && b.len() >= 8 && p <= u32::MAX as u64
        && a[1] != 0 && b[1] != 0 && a[2] != 0 && b[2] != 0
        && (a.len().min(b.len()) as u128) * ((p - 1) as u128).pow(2)
            <= u64::MAX as u128
        && a.iter().chain(b).all(|&x| x != 0 && x < p)
    {
        let mut result = vec![0; a.len() + b.len() - 1];
        for (k, coefficient) in result.iter_mut().enumerate() {
            let low = (k + 1).saturating_sub(b.len());
            let high = (k + 1).min(a.len());
            let mut sum = 0u64;
            for i in low..high {
                sum += a[i] * b[k - i];
            }
            *coefficient = sum % p;
        }
        return trim(result);
    }
    let mut r = vec![0; a.len() + b.len() - 1];
    for (i, &x) in a.iter().enumerate() {
        // Sparse root polynomials often have zero middle coefficients.
        // Multiplying those coefficients cannot change the accumulator.
        if x == 0 { continue; }
        for (j, &y) in b.iter().enumerate() {
            if y == 0 { continue; }
            // Fused multiply-accumulate: no u128 division for canonical
            // small fields or the 61-bit Mersenne prime.
            r[i + j] = if p == MAX_PRIME {
                let product = x as u128 * y as u128;
                let folded = ((product as u64) & MAX_PRIME)
                    + (product >> 61) as u64 + r[i + j];
                let folded = (folded & MAX_PRIME) + (folded >> 61);
                if folded >= p { folded - p } else { folded }
            } else if p <= u32::MAX as u64 {
                // (p-1)^2 + (p-1) < 2^64 when p <= 2^32-1.
                (r[i + j] + x * y) % p
            } else {
                ((r[i + j] as u128 + x as u128 * y as u128) % p as u128) as u64
            };
        }
    }
    trim(r)
}
// Squaring uses symmetry to avoid multiplying the same off-diagonal
// coefficient pair twice. Both inputs and the result are canonical mod p.
#[inline]
fn vsquare(a: &[u64], p: u64) -> Vec<u64> {
    // The general fused product wins for short 32-bit-field inputs in
    // actual Rust benchmarks; use symmetric squaring for larger polynomials,
    // and for 61-bit arithmetic where its crossover occurs earlier.
    if a.len() <= 2 || (a.len() <= 4 && p <= u32::MAX as u64) {
        return vproduct(a, a, p);
    }
    // Dense symmetric squaring: collect every diagonal coefficient once,
    // reducing modulo p once per output coefficient. For off-diagonal
    // terms, add 2*a[i]*a[j]. The same bound as convolution covers all
    // ordered products and therefore guarantees u64 accumulation safety.
    if a.len() >= 8 && p >= 1_000_000 && p <= u32::MAX as u64
        && a[1] != 0 && a[2] != 0
        && (a.len() as u128) * ((p - 1) as u128).pow(2)
            <= u64::MAX as u128
        && a.iter().all(|&x| x != 0 && x < p)
    {
        let mut result = vec![0; 2 * a.len() - 1];
        for (k, coefficient) in result.iter_mut().enumerate() {
            let low = (k + 1).saturating_sub(a.len());
            let high = (k + 1).min(a.len());
            let mut sum = 0u64;
            for i in low..high {
                let j = k - i;
                if i > j { break; }
                let term = a[i] * a[j];
                sum += if i == j { term } else { term + term };
            }
            *coefficient = sum % p;
        }
        return trim(result);
    }
    // (c + d*x^k)^2 needs exactly three field multiplications, and
    // its nonzero leading coefficient makes output trimming unnecessary.
    // This binomial shape occurs frequently in root certificates.
    if a.len() >= 6 && a[0] != 0 && a[1] == 0 && a[2] == 0
        && *a.last().unwrap() != 0
        && a[3..a.len() - 1].iter().all(|&x| x == 0)
    {
        let k = a.len() - 1;
        let mut result = vec![0; 2 * k + 1];
        result[0] = vmul(a[0], a[0], p);
        let cross = vmul(a[0], a[k], p);
        let twice = cross + cross;
        result[k] = if twice >= p { twice - p } else { twice };
        result[2 * k] = vmul(a[k], a[k], p);
        return result;
    }
    // Preserve the previous two-term fast path exactly; it is frequent in
    // certificates and a generic three-slot scan regressed ~4% in Rust.
    // Only inspect the remainder for an additional term after a third
    // nonzero is encountered.
    if a.len() >= 6 && a[1] == 0 && a[2] == 0 {
        let mut terms = [(0usize, 0u64); 2];
        let mut count = 0usize;
        let mut third = None;
        for (i, &x) in a.iter().enumerate() {
            if x == 0 { continue; }
            if count == 2 {
                third = Some((i, x));
                break;
            }
            terms[count] = (i, x);
            count += 1;
        }
        if third.is_none() {
            if count == 0 { return vec![]; }
            let mut result = vec![0; 2 * a.len() - 1];
            let (i, x) = terms[0];
            result[2 * i] = vmul(x, x, p);
            if count == 2 {
                let (j, y) = terms[1];
                result[2 * j] = vmul(y, y, p);
                let cross = vmul(x, y, p);
                let twice = cross + cross;
                result[i + j] = if twice >= p { twice - p } else { twice };
            }
            return trim(result);
        }
        if let Some((k, z)) = third {
            // The existing three-term fast path already scans the tail.
            // Reuse that scan to recognize four-term squares without
            // charging another pass on the common dense path.
            let mut fourth = None;
            let mut too_many = false;
            for (offset, &w) in a[k + 1..].iter().enumerate() {
                if w == 0 { continue; }
                if fourth.is_some() {
                    too_many = true;
                    break;
                }
                fourth = Some((k + 1 + offset, w));
            }
            if !too_many && fourth.is_none() {
                let (i, x) = terms[0];
                let (j, y) = terms[1];
                let mut result = vec![0; 2 * a.len() - 1];
                result[2 * i] = vmul(x, x, p);
                result[2 * j] = vmul(y, y, p);
                result[2 * k] = vmul(z, z, p);
                let cross = vmul(x, y, p);
                let twice = cross + cross;
                result[i + j] = if twice >= p { twice - p } else { twice };
                let cross = vmul(x, z, p);
                let twice = cross + cross;
                let twice = if twice >= p { twice - p } else { twice };
                let sum = result[i + k] + twice;
                result[i + k] = if sum >= p { sum - p } else { sum };
                let cross = vmul(y, z, p);
                let twice = cross + cross;
                let twice = if twice >= p { twice - p } else { twice };
                let sum = result[j + k] + twice;
                result[j + k] = if sum >= p { sum - p } else { sum };
                return trim(result);
            }
            if !too_many {
                if let Some((l, w)) = fourth {
                    // Four nonzero coefficients cost ten field products;
                    // the dense symmetric path would use n(n+1)/2.
                    // Accumulate, rather than assign, cross terms because
                    // exponent sums can collide (e.g. 0+6 == 3+3).
                    let nonzero = [terms[0], terms[1], (k, z), (l, w)];
                    let mut result = vec![0; 2 * a.len() - 1];
                    for (i, &(degree, x)) in nonzero.iter().enumerate() {
                        let diagonal = vmul(x, x, p);
                        let sum = result[2 * degree] + diagonal;
                        result[2 * degree] = if sum >= p { sum - p } else { sum };
                        for &(other_degree, y) in &nonzero[i + 1..] {
                            let cross = vmul(x, y, p);
                            let twice = cross + cross;
                            let twice = if twice >= p { twice - p } else { twice };
                            let sum = result[degree + other_degree] + twice;
                            result[degree + other_degree] = if sum >= p { sum - p } else { sum };
                        }
                    }
                    return trim(result);
                }
            }
        }
    }
    // Root certificates sometimes contain five or more nonzero terms
    // separated by gaps. For these, skip zero pairs during symmetric
    // squaring, without perturbing the dense path (or existing <=4-term
    // specializations) for the common all-nonzero case.
    if a.len() >= 6 && (a[1] == 0 || a[2] == 0) {
        let mut zeros = 0;
        for &value in a {
            if value == 0 {
                zeros += 1;
                if zeros == 2 { break; }
            }
        }
        if zeros == 2 {
            let mut result = vec![0; 2 * a.len() - 1];
            for (i, &x) in a.iter().enumerate() {
                if x == 0 { continue; }
                let diagonal = vmul(x, x, p);
                let sum = result[2 * i] + diagonal;
                result[2 * i] = if sum >= p { sum - p } else { sum };
                for (j, &y) in a.iter().enumerate().skip(i + 1) {
                    if y == 0 { continue; }
                    let cross = vmul(x, y, p);
                    let twice = cross + cross;
                    let twice = if twice >= p { twice - p } else { twice };
                    let sum = result[i + j] + twice;
                    result[i + j] = if sum >= p { sum - p } else { sum };
                }
            }
            return trim(result);
        }
    }
    let mut result = vec![0; 2 * a.len() - 1];
    for (i, &x) in a.iter().enumerate() {
        let diagonal = vmul(x, x, p);
        let sum = result[2 * i] + diagonal;
        result[2 * i] = if sum >= p { sum - p } else { sum };
        for (j, &y) in a.iter().enumerate().skip(i + 1) {
            let cross = vmul(x, y, p);
            let twice = cross + cross;
            let twice = if twice >= p { twice - p } else { twice };
            let sum = result[i + j] + twice;
            result[i + j] = if sum >= p { sum - p } else { sum };
        }
    }
    trim(result)
}
fn vdiv(a: &[u64], b: &[u64], p: u64) -> Result<(Vec<u64>, Vec<u64>), String> {
    if b.is_empty() {
        return Err("division by zero polynomial".into());
    }
    // Degree-short dividends need neither a quotient allocation nor a
    // costly field inversion. The input is already canonical here.
    if a.len() < b.len() {
        return Ok((vec![], a.to_vec()));
    }
    // A monic constant divisor leaves the dividend unchanged. Preserve
    // canonical quotient trimming even if a standalone caller supplies
    // trailing zero coefficients.
    if b.len() == 1 {
        if b[0] == 0 { return Err("division by zero polynomial".into()); }
        if b[0] == 1 { return Ok((trim(a.to_vec()), vec![])); }
        // Degree-zero division has no remainder. Avoid allocating a
        // temporary dividend and trimming after every quotient digit.
        let inverse = vinv(b[0], p);
        let quotient = a.iter().map(|&x| vmul(x, inverse, p)).collect();
        return Ok((trim(quotient), vec![]));
    }
    // Division by x^k is a coefficient split, not a polynomial long
    // division. The guard avoids a full scan for dense divisors.
    if b.len() > 1 && b[0] == 0 && *b.last().unwrap() == 1
        && b[1..b.len()-1].iter().all(|&x| x == 0)
    {
        let shift = b.len() - 1;
        return Ok((trim(a[shift..].to_vec()), trim(a[..shift].to_vec())));
    }
    // Synthetic division by x + c needs only one running remainder and
    // one quotient allocation, rather than a mutable dividend and a
    // trim pass on every quotient digit.
    if b.len() == 2 && b[1] == 1 {
        let mut q = vec![0; a.len() - 1];
        let mut carry = *a.last().unwrap();
        for i in (1..a.len()).rev() {
            q[i - 1] = carry;
            let product = vmul(carry, b[0], p);
            let value = a[i - 1];
            carry = if value >= product { value - product }
                    else { value + p - product };
        }
        return Ok((trim(q), if carry == 0 { vec![] } else { vec![carry] }));
    }
    // Synthetic division by a nonmonic linear divisor needs just a
    // running remainder and no repeated temporary-polynomial trimming.
    if b.len() == 2 {
        let inverse = vinv(b[1], p);
        let mut quotient = vec![0; a.len() - 1];
        let mut carry = *a.last().unwrap();
        for index in (1..a.len()).rev() {
            let coefficient = vmul(carry, inverse, p);
            quotient[index - 1] = coefficient;
            let product = vmul(coefficient, b[0], p);
            let value = a[index - 1];
            carry = if value >= product { value - product }
                    else { value + p - product };
        }
        return Ok((trim(quotient), if carry == 0 { vec![] } else { vec![carry] }));
    }
    // Retain the established monic-binomial fast path for k >= 3:
    // native Rust regression screening found the fixed-degree approach
    // about 2-4% slower on this already-specialized case.
    if b.len() >= 4 && b[0] != 0 && *b.last().unwrap() == 1
        && b[1..b.len() - 1].iter().all(|&x| x == 0)
    {
        let mut r = a.to_vec();
        let mut q = vec![0; a.len() - b.len() + 1];
        while !r.is_empty() && r.len() >= b.len() {
            let shift = r.len() - b.len();
            let c = *r.last().unwrap();
            q[shift] = c;
            r.pop();
            let product = vmul(c, b[0], p);
            let old = r[shift];
            r[shift] = if old >= product { old - product }
                       else { old + p - product };
            r = trim(r);
        }
        return Ok((trim(q), r));
    }
    // A monic sparse divisor x^k + c*x^j + d (at most one nonzero
    // interior coefficient) changes at most two remainder positions per
    // quotient digit. Iterate a fixed descending degree to avoid repeated
    // trimming and to skip zero quotient digits.
    if *b.last().unwrap() == 1 {
        let degree = b.len() - 1;
        let mut middle = None;
        let mut too_many = false;
        for (index, &value) in b[1..degree].iter().enumerate() {
            if value != 0 {
                if middle.is_some() {
                    too_many = true;
                    break;
                }
                middle = Some((index + 1, value));
            }
        }
        if !too_many {
            let mut remainder = a.to_vec();
            let mut quotient = vec![0; a.len() - degree];
            for shift in (0..quotient.len()).rev() {
                let coefficient = remainder[shift + degree];
                if coefficient == 0 { continue; }
                quotient[shift] = coefficient;
                remainder[shift + degree] = 0;
                if b[0] != 0 {
                    let product = vmul(coefficient, b[0], p);
                    let old = remainder[shift];
                    remainder[shift] = if old >= product { old - product }
                                       else { old + p - product };
                }
                if let Some((index, value)) = middle {
                    let product = vmul(coefficient, value, p);
                    let old = remainder[shift + index];
                    remainder[shift + index] = if old >= product { old - product }
                                               else { old + p - product };
                }
            }
            remainder.truncate(degree);
            return Ok((trim(quotient), trim(remainder)));
        }
    }
    // General long division: a descending fixed degree removes both the
    // per-iteration pop/trim scan and multiplications by zero divisor
    // coefficients. The top term cancels algebraically for every step.
    let mut remainder = a.to_vec();
    let mut quotient = vec![0; a.len() - b.len() + 1];
    let lead = *b.last().unwrap();
    let inverse = if lead == 1 { 1 } else { vinv(lead, p) };
    for shift in (0..quotient.len()).rev() {
        let highest = remainder[shift + b.len() - 1];
        if highest == 0 { continue; }
        let coefficient = if lead == 1 { highest } else { vmul(highest, inverse, p) };
        quotient[shift] = coefficient;
        remainder[shift + b.len() - 1] = 0;
        for (index, &value) in b[..b.len() - 1].iter().enumerate() {
            if value == 0 { continue; }
            let product = vmul(coefficient, value, p);
            let old = remainder[shift + index];
            remainder[shift + index] = if old >= product { old - product }
                                       else { old + p - product };
        }
    }
    remainder.truncate(b.len() - 1);
    Ok((trim(quotient), trim(remainder)))
}
fn vpower(base: &[u64], mut e: u64, m: &[u64], p: u64) -> Result<Vec<u64>, String> {
    let mut r = vdiv(&[1], m, p)?.1;
    let mut b = vdiv(base, m, p)?.1;
    while e > 0 {
        if e & 1 == 1 {
            r = vdiv(&vproduct(&r, &b, p), m, p)?.1;
        }
        e >>= 1;
        // No subsequent iteration reads b after the final exponent bit.
        // Skipping the last modular square saves a polynomial product
        // and a polynomial division in the verifier's independent replay.
        if e != 0 {
            b = vdiv(&vsquare(&b, p), m, p)?.1;
        }
    }
    Ok(r)
}
fn vmonic(mut a: Vec<u64>, p: u64) -> Vec<u64> {
    if let Some(&lead) = a.last() {
        // The typical GCD result is already monic. No Fermat inversion
        // or coefficient multiplies are needed to normalize it again.
        if lead != 1 {
            let inv = vinv(lead, p);
            for c in &mut a {
                *c = vmul(*c, inv, p);
            }
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
                    FieldOp::Add => {
                        let sum = *a + *b; // canonical, p <= 2^61-1
                        if sum >= p { sum - p } else { sum }
                    }
                    FieldOp::Sub => {
                        if a >= b { *a - *b } else { *a + p - *b }
                    },
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
                let (verified_q, verified_r) = vdiv(a, b, p)?;
                if verified_q != *quotient || verified_r != *remainder {
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
                // Squaring a polynomial is algebraically identical to multiplying
                // it by itself, but the independent verifier's specialized
                // arithmetic avoids computing symmetric off-diagonal pairs twice.
                state.base = vdiv(&vsquare(&state.base, p), &state.modulus, p)?.1;
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
                let (verified_q, verified_r) = vdiv(&state.0, &state.1, p)?;
                if verified_q != *quotient || verified_r != *remainder {
                    return Err("Euclidean step failed".into());
                }
                // Move the prior divisor into the new dividend; a deep clone
                // of that polynomial was previously performed every GCD step.
                // The exact remainder has already been independently computed
                // and compared to the certificate. Move that owned vector into
                // the state instead of allocating a second copy of the event.
                let previous_b = std::mem::replace(&mut state.1, verified_r);
                state.0 = previous_b;
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
                    || { let sum = vmul(polynomial[1], *root, p) + polynomial[0]; sum != 0 && sum != p }
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

    #[test]
    fn round22_fast_trace_field_exact_under_frozen_arithmetic() {
        let mut seed = 0x524f_4f54_2026_1118u64;
        for p in [3, 487, 998244353, 1000000007, MAX_PRIME - 30, MAX_PRIME] {
            for _ in 0..4096 {
                seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
                let a = seed % p;
                seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
                let b = seed % p;
                for op in [FieldOp::Add, FieldOp::Sub, FieldOp::Mul, FieldOp::Square] {
                    let reference = match op {
                        FieldOp::Add => ((a as u128 + b as u128) % p as u128) as u64,
                        FieldOp::Sub => ((a as u128 + p as u128 - b as u128) % p as u128) as u64,
                        FieldOp::Mul | FieldOp::Square =>
                            ((a as u128 * b as u128) % p as u128) as u64,
                    };
                    assert_eq!(field_value(&op, a, b, p), reference,
                               "p={p} a={a} b={b} op={op:?}");
                    if a < p - 2 && b < p - 2 {
                        assert_eq!(
                            field_value(&op, a + p, b, p),
                            match op {
                                FieldOp::Add => ((a as u128 + p as u128 + b as u128) % p as u128) as u64,
                                FieldOp::Sub => ((a as u128 + p as u128 + p as u128 - b as u128) % p as u128) as u64,
                                FieldOp::Mul | FieldOp::Square =>
                                    (((a + p) as u128 * b as u128) % p as u128) as u64,
                            }
                        );
                    }
                }
            }
        }
    }

    #[test]
    #[ignore = "paired production-Rust trace field microbench"]
    fn bench_round22_trace_field_arithmetic() {
        use std::hint::black_box;
        use std::time::Instant;
        fn old(op: &FieldOp, a: u64, b: u64, p: u64) -> u64 {
            match op {
                FieldOp::Add => ((a as u128 + b as u128) % p as u128) as u64,
                FieldOp::Sub => ((a as u128 + p as u128 - b as u128) % p as u128) as u64,
                FieldOp::Mul | FieldOp::Square =>
                    ((a as u128 * b as u128) % p as u128) as u64,
            }
        }
        fn sample(xs: &[(u64,u64)], op: &FieldOp, p: u64, fast: bool) -> f64 {
            let start = Instant::now();
            let mut checksum = 0u64;
            for _ in 0..32 {
                for &(a,b) in xs {
                    let (a,b) = (black_box(a), black_box(b));
                    checksum ^= if fast {
                        field_value(&op, a, b, p)
                    } else {
                        old(op, a, b, p)
                    };
                }
            }
            black_box(checksum);
            start.elapsed().as_nanos() as f64 / (32 * xs.len()) as f64
        }
        let mut seed = 0x2549_2026_1118u64;
        for p in [487, 1000000007, MAX_PRIME] {
            let mut xs = Vec::new();
            for _ in 0..4096 {
                seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
                let a = seed % p;
                seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
                let b = seed % p;
                xs.push((a,b));
            }
            for op in [FieldOp::Add, FieldOp::Sub, FieldOp::Mul, FieldOp::Square] {
                let (mut before, mut after) = (Vec::new(), Vec::new());
                for i in 0..13 {
                    if i % 2 == 0 {
                        before.push(sample(&xs, &op, p, false));
                        after.push(sample(&xs, &op, p, true));
                    } else {
                        after.push(sample(&xs, &op, p, true));
                        before.push(sample(&xs, &op, p, false));
                    }
                }
                before.sort_by(f64::total_cmp);
                after.sort_by(f64::total_cmp);
                println!("round22 field p={p} op={op:?}: {:.3} -> {:.3} ns ({:.3}x)",
                         before[6], after[6], before[6] / after[6]);
            }
        }
    }
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

#[cfg(test)]
mod round89_column_accumulators {
    use super::*;

    // Independent wide-arithmetic oracle. Never uses vproduct or vsquare.
    fn reference_product(a: &[u64], b: &[u64], p: u64) -> Vec<u64> {
        if a.is_empty() || b.is_empty() { return vec![]; }
        let mut out = vec![0; a.len() + b.len() - 1];
        for (i, &x) in a.iter().enumerate() {
            for (j, &y) in b.iter().enumerate() {
                out[i + j] =
                    ((out[i + j] as u128 + x as u128 * y as u128) % p as u128) as u64;
            }
        }
        trim(out)
    }

    #[test]
    fn column_convolution_and_squaring_match_u128_reference() {
        use rand::Rng;
        let mut rng = StdRng::seed_from_u64(0x2026_1121_0089);
        for p in [65_521, 1_000_000_007, 1_400_000_009,
                  4_294_967_291, MAX_PRIME] {
            for width in [2, 3, 5, 8, 12] {
                for case in 0..120 {
                    let mut a: Vec<u64> = (0..width).map(|_| rng.gen_range(0..p)).collect();
                    let mut b: Vec<u64> = (0..width).map(|_| rng.gen_range(0..p)).collect();
                    if case % 3 == 0 {
                        for index in (0..width).step_by(3) { a[index] = 0; }
                        for index in (1..width).step_by(3) { b[index] = 0; }
                    } else if case % 3 == 1 {
                        a[0] = p - 1;
                        b[width - 1] = p - 1;
                    }
                    assert_eq!(vproduct(&a, &b, p), reference_product(&a, &b, p),
                        "product: p={p}, width={width}, case={case}");
                    assert_eq!(vsquare(&a, p), reference_product(&a, &a, p),
                        "square: p={p}, width={width}, case={case}");
                }
            }
        }
    }

    #[test]
    fn certificate_replay_matches_solver_after_clone_elimination() {
        let limits = RootLimits {
            field_work: MAX_WORK,
            split_attempts: 1000,
            events: MAX_EVENTS,
            degree: MAX_DEGREE,
        };
        for p in [65_521, 1_000_000_007, MAX_PRIME] {
            for polynomial in [vec![p - 3, 1], vec![6, p - 5, 1]] {
                let report = solve(&polynomial, p, 13, limits.clone(),
                    &mut |_| Ok(())).unwrap();
                assert!(report.complete(), "incomplete p={p} poly={polynomial:?}");
                verify(&report).unwrap();
            }
        }
    }
}
