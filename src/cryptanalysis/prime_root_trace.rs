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
    // Append a (2^bits-1) exponent block by repeated verified squares
    // followed by a verified product. All operations remain in the trace.
    fn inverse_mersenne_append(&mut self, mut power: u64, factor: u64, bits: usize) -> Work<u64> {
        for _ in 0..bits {
            power = self.field(FieldOp::Square, power, power)?;
        }
        self.field(FieldOp::Mul, power, factor)
    }
    #[inline(never)]
    fn inv(&mut self, value: u64) -> Work<u64> {
        debug_assert_ne!(value, 0);
        self.event(RootEvent::InverseBegin { value })?;
        let p = self.report.p;
        // Exact odd-prime identities. No field operations are performed, so
        // none should be added to the trace or field-work budget.
        let trivial = if value == 1 { Some(1) }
            else if value == 2 { Some((p + 1) / 2) }
            else if value == p - 1 { Some(p - 1) }
            else if value == p - 2 { Some((p - 1) / 2) }
            else { None };
        if let Some(inverse) = trivial {
            self.event(RootEvent::InverseEnd { value, inverse })?;
            return Ok(inverse);
        }
        if p == MAX_PRIME {
            // p-2 = 2^61-3 = 4*(2^59-1)+1. Build the exponents
            // 2^k-1 for k=2,4,8,16,32,48,56,58,59, then append
            // two squares and a multiplication by the original value.
            // This uses 60 traced squares + 10 traced multiplies instead
            // of 60 squares + 60 multiplies in the binary Fermat chain.
            // Unlike the independent verifier, these operations are
            // accounted for and remain interruptible at every field event.
            let x2 = self.inverse_mersenne_append(value, value, 1)?;
            let x4 = self.inverse_mersenne_append(x2, x2, 2)?;
            let x8 = self.inverse_mersenne_append(x4, x4, 4)?;
            let x16 = self.inverse_mersenne_append(x8, x8, 8)?;
            let x32 = self.inverse_mersenne_append(x16, x16, 16)?;
            let x48 = self.inverse_mersenne_append(x32, x16, 16)?;
            let x56 = self.inverse_mersenne_append(x48, x8, 8)?;
            let x58 = self.inverse_mersenne_append(x56, x2, 2)?;
            let x59 = self.inverse_mersenne_append(x58, value, 1)?;
            let inverse = self.inverse_mersenne_append(x59, value, 2)?;
            self.event(RootEvent::InverseEnd { value, inverse })?;
            return Ok(inverse);
        }
        let mut exponent = p - 2;
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
        // The source-exact root benchmarks found a 2-3% gain on 65,521
        // but repeatable larger-prime full-solver regressions. Restrict this
        // small-field identity specialization to its measured benefit region;
        // keep the original general producer convolution for larger fields.
        if self.report.p <= 65_521 && (a.len() <= 2 || b.len() <= 2) {
            if let Some(result) = self.mul_shortcut(a, b)? {
                return Ok(result);
            }
        }
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
                // Multiplication by zero cannot contribute to the polynomial.
                // Omitting it avoids two trace events and their sink calls.
                if y == 0 {
                    continue;
                }
                // Multiplication by one and addition to an empty coefficient
                // are exact identities. Their omission changes only the
                // operation trace/counters, not the Product certificate.
                let z = if x == 1 { y } else if y == 1 { x }
                        else { self.field(FieldOp::Mul, x, y)? };
                if product[i + j] == 0 {
                    product[i + j] = z;
                } else {
                    product[i + j] = self.field(FieldOp::Add, product[i + j], z)?;
                }
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
    // An opt-in coefficient-copy shortcut for the identities 1*f and X*f.
    // Keep it out of the dense-by-dense convolution code path.
    #[inline(never)]
    fn mul_shortcut(&mut self, a: &[u64], b: &[u64]) -> Work<Option<Vec<u64>>> {
        let special = if a == [1] {
            Some(trim(b.to_vec()))
        } else if b == [1] {
            Some(trim(a.to_vec()))
        } else if a == [0, 1] && b.len() >= 4 {
            let mut shifted = Vec::with_capacity(b.len() + 1);
            shifted.push(0);
            shifted.extend_from_slice(b);
            Some(trim(shifted))
        } else if b == [0, 1] && a.len() >= 4 {
            let mut shifted = Vec::with_capacity(a.len() + 1);
            shifted.push(0);
            shifted.extend_from_slice(a);
            Some(trim(shifted))
        } else {
            None
        };
        if let Some(product) = special {
            // Preserve the durable Product event and independent verification.
            self.event(RootEvent::Product {
                a: a.to_vec(),
                b: b.to_vec(),
                product: product.clone(),
            })?;
            Ok(Some(product))
        } else {
            Ok(None)
        }
    }
    fn div(&mut self, a: &[u64], b: &[u64]) -> Work<(Vec<u64>, Vec<u64>)> {
        debug_assert!(!b.is_empty());
        let mut r = a.to_vec();
        let mut q = vec![0; a.len().saturating_sub(b.len()) + 1];
        if a.len() >= b.len() {
            // The leading coefficient cancels by construction: c = lead(r)/lead(b).
            // In the monic case c is already the top remainder coefficient, so
            // no inverse or traced multiplication is necessary. Zero divisor
            // coefficients also leave lower remainder coefficients unchanged.
            // These are omitted arithmetic operations, not unreported work.
            let lead = *b.last().unwrap();
            let inverse = if lead == 1 { None } else { Some(self.inv(lead)?) };
            while r.len() >= b.len() {
                let shift = r.len() - b.len();
                let highest = *r.last().unwrap();
                let c = if let Some(inv) = inverse {
                    self.field(FieldOp::Mul, highest, inv)?
                } else {
                    highest
                };
                q[shift] = c;
                for (k, &v) in b[..b.len() - 1].iter().enumerate() {
                    if v == 0 { continue; }
                    let product = self.field(FieldOp::Mul, c, v)?;
                    r[shift + k] = self.field(FieldOp::Sub, r[shift + k], product)?;
                }
                // lead(r) - c * lead(b) is zero in F_p, so removing this
                // coefficient is exact for both monic and nonmonic divisors.
                r.pop();
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
        // A smaller-degree polynomial is already reduced. Recording a
        // zero-quotient division allocates a certificate and runs the event
        // sink without doing any field work. PowerBegin already binds the
        // exact modulus and base, and the independent verifier recomputes
        // both initial residues, so these no-op divisions are unnecessary.
        let mut b = if base.len() < modulus.len() {
            base.to_vec()
        } else {
            self.div(base, modulus)?.1
        };
        let mut acc = if modulus.len() > 1 {
            vec![1]
        } else {
            self.div(&[1], modulus)?.1
        };
        while exponent != 0 {
            if exponent & 1 == 1 {
                let product = self.mul(&acc, &b)?;
                // If the product degree is below the modulus degree, its
                // remainder is already the product. Avoid allocating a
                // quotient and emitting a redundant Division event.
                acc = if product.len() < modulus.len() {
                    product
                } else {
                    self.div(&product, modulus)?.1
                };
            }
            // Round 92: the final modular square is never consumed after
            // the exponent becomes zero. Save one traced multiplication and
            // division per exponentiation, and keep the final PowerStep's
            // base equal to the last useful base. The verifier supports both
            // this shorter trace and legacy traces with the terminal square.
            exponent >>= 1;
            if exponent != 0 {
                let product = self.mul(&b, &b)?;
                b = if product.len() < modulus.len() {
                    product
                } else {
                    self.div(&product, modulus)?.1
                };
            }
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
        // A monic Euclidean result is already normalized. Avoid the
        // otherwise redundant inverse ceremony and coefficient multiplies.
        if let Some(&lead) = a.last() {
            if lead != 1 {
                let inv = self.inv(lead)?;
                for c in &mut a {
                    *c = self.field(FieldOp::Mul, *c, inv)?;
                }
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
        // The first Frobenius power is the only call using the caller's
        // possibly nonmonic input. Every later split uses a monic gcd
        // factor. Dispatch here, leaving the monic power implementation
        // byte-for-byte unchanged for the hot paths and their codegen.
        let xp = if f.last().is_some_and(|&lead| lead != 1) {
            self.pow_nonmonic(&[0, 1], p, &f)?
        } else {
            self.pow(&[0, 1], p, &f)?
        };
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
                    // Splitting a monic polynomial leaves monic linear
                    // factors; their root is simply -constant (mod p).
                    let negated_constant = if h[1] == 1 {
                        h[0]
                    } else {
                        let inv = self.inv(h[1])?;
                        self.field(FieldOp::Mul, h[0], inv)?
                    };
                    let root = self.field(FieldOp::Sub, 0, negated_constant)?;
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
    #[inline(never)]
    fn pow_nonmonic(&mut self, base: &[u64], mut exponent: u64, modulus: &[u64]) -> Work<Vec<u64>> {
        let id = self.next_id;
        self.next_id += 1;
        self.event(RootEvent::PowerBegin {
            id,
            base: base.to_vec(),
            exponent,
            modulus: modulus.to_vec(),
        })?;
        // Repeated polynomial reductions are now monic. Tracing the one
        // Fermat inverse of the original leading coefficient is sufficient
        // for the entire modular exponentiation. All skipped inverses are
        // identities because the normalized divisor's leading term is one.
        let lead = *modulus.last().expect("power modulus is nonzero");
        let inverse = self.inv(lead)?;
        let mut normalized_modulus = Vec::with_capacity(modulus.len());
        for &coefficient in &modulus[..modulus.len() - 1] {
            normalized_modulus.push(if coefficient == 0 {
                0
            } else if coefficient == lead {
                1
            } else {
                self.field(FieldOp::Mul, coefficient, inverse)?
            });
        }
        normalized_modulus.push(1);
        let modulus = normalized_modulus.as_slice();
        // A smaller-degree polynomial is already reduced. Recording a
        // zero-quotient division allocates a certificate and runs the event
        // sink without doing any field work. PowerBegin already binds the
        // original modulus, and the independent verifier compares residues
        // modulo its monic normalization.
        let mut b = if base.len() < modulus.len() {
            base.to_vec()
        } else {
            self.div(base, modulus)?.1
        };
        let mut acc = if modulus.len() > 1 {
            vec![1]
        } else {
            self.div(&[1], modulus)?.1
        };
        while exponent != 0 {
            if exponent & 1 == 1 {
                let product = self.mul(&acc, &b)?;
                // If the product degree is below the modulus degree, its
                // remainder is already the product. Avoid allocating a
                // quotient and emitting a redundant Division event.
                acc = if product.len() < modulus.len() {
                    product
                } else {
                    self.div(&product, modulus)?.1
                };
            }
            // Round 92: the final modular square is never consumed after
            // the exponent becomes zero. Save one traced multiplication and
            // division per exponentiation, and keep the final PowerStep's
            // base equal to the last useful base. The verifier supports both
            // this shorter trace and legacy traces with the terminal square.
            exponent >>= 1;
            if exponent != 0 {
                let product = self.mul(&b, &b)?;
                b = if product.len() < modulus.len() {
                    product
                } else {
                    self.div(&product, modulus)?.1
                };
            }
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
        e >>= 1;
        // The final square is never consumed.
        if e != 0 {
            a = vmul(a, a, p);
        }
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
// Fast path for c + d*x^k (k >= 5), with c,d both nonzero. This is
// separate from monomial multiplication, which is handled earlier.
#[cold]
#[inline(never)]
fn vbinomial_product(binomial: &[u64], other: &[u64], p: u64) -> Option<Vec<u64>> {
    if binomial.len() < 6 || binomial[0] == 0
        || binomial[1] != 0 || binomial[2] != 0
        || *binomial.last().unwrap() == 0
        || binomial[3..binomial.len() - 1].iter().any(|&x| x != 0)
    {
        return None;
    }
    let shift = binomial.len() - 1;
    let low = binomial[0];
    let high = binomial[shift];
    let mut out = vec![0; binomial.len() + other.len() - 1];
    for (i, &x) in other.iter().enumerate() {
        if x == 0 { continue; }
        // All caller coefficients are canonical. Two additions cannot
        // overflow u64 because p <= 2^61 - 1.
        let product = vmul(x, low, p);
        let sum = out[i] + product;
        out[i] = if sum >= p { sum - p } else { sum };
        if high == 1 {
            let sum = out[i + shift] + x;
            out[i + shift] = if sum >= p { sum - p } else { sum };
        } else {
            let product = vmul(x, high, p);
            let sum = out[i + shift] + product;
            out[i + shift] = if sum >= p { sum - p } else { sum };
        }
    }
    Some(trim(out))
}
// Used only for independently replaying recorded Product events. Do not
// alter the generic vproduct hot path: production Rust A/B showed 5-12%
// regressions from even guarded binomial dispatch on ordinary operands.
fn vverified_product(a: &[u64], b: &[u64], p: u64) -> Vec<u64> {
    if a.len() >= 6 && a[1] == 0 && a[2] == 0 {
        if let Some(result) = vbinomial_product(a, b, p) { return result; }
    }
    if b.len() >= 6 && b[1] == 0 && b[2] == 0 {
        if let Some(result) = vbinomial_product(b, a, p) { return result; }
    }
    vproduct(a, b, p)
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
// Quotient-free ownership for a residue already below the modulus degree.
// In particular, initial 1 and x+delta need no allocation or division when
// their degrees are smaller. A nonempty modulus is required in both paths.
// Keep the general cubic-and-higher reduction out of the linear/quadratic
// modular-power hot path. The verifier has many repeated calls with tiny
// moduli; inlining this loop there enlarges the entire exponentiation body.
#[inline(never)]
fn vpower_monic_remainder(mut remainder: Vec<u64>, modulus: &[u64], p: u64) -> Vec<u64> {
    debug_assert!(modulus.len() >= 4 && modulus.last() == Some(&1));
    let degree = modulus.len() - 1;
    for shift in (0..remainder.len() - degree).rev() {
        let high = remainder[shift + degree];
        if high == 0 { continue; }
        remainder[shift + degree] = 0;
        for (i, &coefficient) in modulus[..degree].iter().enumerate() {
            if coefficient == 0 { continue; }
            let term = vmul(high, coefficient, p);
            let old = remainder[shift + i];
            remainder[shift + i] = if old >= term { old - term } else { old + p - term };
        }
    }
    remainder.truncate(degree);
    trim(remainder)
}

// For x^d +/- 1 the quotient-ring remainder needs one coefficient
// addition/subtraction per cancelled high term, not degree modular products.
// Keep this verifier-only fast path out of the existing generic remainder
// loop so dense/non-unit moduli retain their original code.
#[inline(never)]
fn vpower_unit_binomial_remainder(
    mut remainder: Vec<u64>, degree: usize, positive_reduction: bool, p: u64,
) -> Vec<u64> {
    debug_assert!(degree >= 4 && remainder.len() > degree);
    for shift in (0..remainder.len() - degree).rev() {
        let high = remainder[shift + degree];
        if high == 0 { continue; }
        remainder[shift + degree] = 0;
        let old = remainder[shift];
        remainder[shift] = if positive_reduction {
            // The modulus is x^d - 1.
            let sum = old + high; // canonical residues, 2*p-2 <= 2^62-4
            if sum >= p { sum - p } else { sum }
        } else {
            // The modulus is x^d + 1.
            if old >= high { old - high } else { old + p - high }
        };
    }
    remainder.truncate(degree);
    trim(remainder)
}

fn vpower_reduce(product: Vec<u64>, modulus: &[u64], p: u64) -> Result<Vec<u64>, String> {
    if !modulus.is_empty() && product.len() < modulus.len() {
        return Ok(product);
    }
    // The normalized modulus is monic. Modular powering and replay need
    // the remainder only: avoid allocating a quotient for every reduction.
    // Preserve the existing synthetic linear/quadratic division fast paths.
    if modulus.len() >= 4 && modulus.last() == Some(&1) {
        // Check the unit constant before scanning interior coefficients.
        // Avoid the scan for the overwhelmingly common dense modulus.
        let degree = modulus.len() - 1;
        if degree >= 4 && (modulus[0] == 1 || modulus[0] == p - 1)
            && modulus[1..degree].iter().all(|&x| x == 0)
        {
            return Ok(vpower_unit_binomial_remainder(
                product, degree, modulus[0] == p - 1, p));
        }
        return Ok(vpower_monic_remainder(product, modulus, p));
    }
    Ok(vdiv(&product, modulus, p)?.1)
}
// A monic quadratic quotient ring has only two coefficients. Fusing the
// product/square with reduction avoids temporary polynomial products,
// quotient vectors, and repeated length/trimming scans during verifier
// modular exponentiation. Independent of Engine::pow and its event trace.
// Variable-time arithmetic: coefficients here are public certificate data.
#[inline]
fn quadratic_sub(a: u64, b: u64, p: u64) -> u64 {
    if a >= b { a - b } else { a + p - b }
}
#[inline]
fn quadratic_mul(a: (u64, u64), b: (u64, u64), c: u64, d: u64, p: u64)
    -> (u64, u64)
{
    // x^2 = -d*x-c in F_p[x]/(x^2+d*x+c).
    let high = vmul(a.1, b.1, p);
    let low = quadratic_sub(vmul(a.0, b.0, p), vmul(c, high, p), p);
    let cross = vmul(a.0, b.1, p) + vmul(a.1, b.0, p);
    let cross = if cross >= p { cross - p } else { cross };
    let linear = quadratic_sub(cross, vmul(d, high, p), p);
    (low, linear)
}
#[inline]
fn quadratic_square(a: (u64, u64), c: u64, d: u64, p: u64) -> (u64, u64) {
    let high = vmul(a.1, a.1, p);
    let low = quadratic_sub(vmul(a.0, a.0, p), vmul(c, high, p), p);
    let cross = vmul(a.0, a.1, p);
    let cross = if cross + cross >= p { cross + cross - p } else { cross + cross };
    (low, quadratic_sub(cross, vmul(d, high, p), p))
}
// Use fused replay only for fields >= 2^24: two replicated native Rust
// A/B runs found 2-4% complete-solver regressions over p=65521 despite
// favorable isolated C++ arithmetic; preserve the generic small-field path.
// Replay PowerStep over a degree-two quotient ring without constructing a
// temporary polynomial product and quotient. The Engine records independent
// field-operation events; these helpers are verifier-only arithmetic.
#[inline]
fn vquadratic_replay_mul(a: &[u64], b: &[u64], m: &[u64], p: u64) -> Vec<u64> {
    debug_assert_eq!(m.len(), 3);
    let lhs = (a.first().copied().unwrap_or(0), a.get(1).copied().unwrap_or(0));
    let rhs = (b.first().copied().unwrap_or(0), b.get(1).copied().unwrap_or(0));
    let (constant, linear) = quadratic_mul(lhs, rhs, m[0], m[1], p);
    trim(vec![constant, linear])
}
#[inline]
fn vquadratic_replay_square(a: &[u64], m: &[u64], p: u64) -> Vec<u64> {
    debug_assert_eq!(m.len(), 3);
    let lhs = (a.first().copied().unwrap_or(0), a.get(1).copied().unwrap_or(0));
    let (constant, linear) = quadratic_square(lhs, m[0], m[1], p);
    trim(vec![constant, linear])
}

// Specialized arithmetic in F_p[x]/(x^3+c2*x^2+c1*x+c0).
// Polynomial multiplication has five coefficients; reduce the x^4 and x^3
// terms directly. All additions are of canonical residues and fit u64 for
// the public p <= 2^61-1 bound. This is verifier-only (no Engine trace edits).
#[inline]
fn cubic_add(a: u64, b: u64, p: u64) -> u64 {
    let sum = a + b;
    if sum >= p { sum - p } else { sum }
}
#[inline]
fn cubic_mul(a: [u64; 3], b: [u64; 3], c: [u64; 3], p: u64) -> [u64; 3] {
    let high = vmul(a[2], b[2], p);
    let t3 = cubic_add(vmul(a[1], b[2], p), vmul(a[2], b[1], p), p);
    let t2 = cubic_add(
        cubic_add(vmul(a[0], b[2], p), vmul(a[1], b[1], p), p),
        vmul(a[2], b[0], p), p);
    let t1 = cubic_add(vmul(a[0], b[1], p), vmul(a[1], b[0], p), p);
    let t0 = vmul(a[0], b[0], p);
    // x^4 = -c2*x^3-c1*x^2-c0*x; then remove x^3.
    let t3 = quadratic_sub(t3, vmul(c[2], high, p), p);
    let t2 = quadratic_sub(t2, vmul(c[1], high, p), p);
    let t1 = quadratic_sub(t1, vmul(c[0], high, p), p);
    [
        quadratic_sub(t0, vmul(c[0], t3, p), p),
        quadratic_sub(t1, vmul(c[1], t3, p), p),
        quadratic_sub(t2, vmul(c[2], t3, p), p),
    ]
}
#[inline]
fn cubic_square(a: [u64; 3], c: [u64; 3], p: u64) -> [u64; 3] {
    // Symmetry reduces the nine convolution terms to six products.
    let high = vmul(a[2], a[2], p);
    let cross = vmul(a[1], a[2], p);
    let t3 = cubic_add(cross, cross, p);
    let cross = vmul(a[0], a[2], p);
    let t2 = cubic_add(cubic_add(cross, cross, p), vmul(a[1], a[1], p), p);
    let cross = vmul(a[0], a[1], p);
    let t1 = cubic_add(cross, cross, p);
    let t0 = vmul(a[0], a[0], p);
    let t3 = quadratic_sub(t3, vmul(c[2], high, p), p);
    let t2 = quadratic_sub(t2, vmul(c[1], high, p), p);
    let t1 = quadratic_sub(t1, vmul(c[0], high, p), p);
    [
        quadratic_sub(t0, vmul(c[0], t3, p), p),
        quadratic_sub(t1, vmul(c[1], t3, p), p),
        quadratic_sub(t2, vmul(c[2], t3, p), p),
    ]
}
#[inline]
fn cubic_coordinates(a: &[u64]) -> [u64; 3] {
    [a.first().copied().unwrap_or(0), a.get(1).copied().unwrap_or(0),
        a.get(2).copied().unwrap_or(0)]
}
#[inline]
fn cubic_trim(a: [u64; 3]) -> Vec<u64> {
    trim(a.to_vec())
}
fn vpower_cubic(base: &[u64], mut exponent: u64, m: &[u64], p: u64)
    -> Result<Vec<u64>, String>
{
    debug_assert_eq!(m.len(), 4);
    let lead = m[3];
    if lead == 0 { return Err("division by zero leading coefficient".into()); }
    let inverse = if lead == 1 { 1 } else { vinv(lead, p) };
    let c = if lead == 1 { [m[0], m[1], m[2]] }
            else { [vmul(m[0], inverse, p), vmul(m[1], inverse, p),
                    vmul(m[2], inverse, p)] };
    // Horner evaluation of an arbitrary-degree base in the cubic quotient.
    // Multiplication by x maps (a,b,d) to (-c0*d,a-c1*d,b-c2*d).
    let mut power = [0u64; 3];
    for &coefficient in base.iter().rev() {
        let high = power[2];
        power = [
            quadratic_sub(coefficient, vmul(high, c[0], p), p),
            quadratic_sub(power[0], vmul(high, c[1], p), p),
            quadratic_sub(power[1], vmul(high, c[2], p), p),
        ];
    }
    let mut accumulator = [1, 0, 0];
    while exponent != 0 {
        if exponent & 1 == 1 {
            accumulator = cubic_mul(accumulator, power, c, p);
        }
        exponent >>= 1;
        if exponent != 0 {
            power = cubic_square(power, c, p);
        }
    }
    Ok(cubic_trim(accumulator))
}
#[inline]
fn vcubic_replay_mul(a: &[u64], b: &[u64], m: &[u64], p: u64) -> Vec<u64> {
    debug_assert_eq!(m.len(), 4);
    cubic_trim(cubic_mul(cubic_coordinates(a), cubic_coordinates(b),
                         [m[0], m[1], m[2]], p))
}
#[inline]
fn vcubic_replay_square(a: &[u64], m: &[u64], p: u64) -> Vec<u64> {
    debug_assert_eq!(m.len(), 4);
    cubic_trim(cubic_square(cubic_coordinates(a), [m[0], m[1], m[2]], p))
}

// Quartic quotient-ring arithmetic for the independent certificate verifier.
// Unlike generic Vec polynomial multiplication, coefficients stay on the stack,
// and x^6, x^5 and x^4 are reduced directly by a monic degree-four modulus.
// All field additions and doubled cross terms fit u64 because p <= 2^61-1.
#[inline]
fn quartic_reduce(mut t: [u64; 7], c: [u64; 4], p: u64) -> [u64; 4] {
    // Sparse quartic moduli occur during splitting. Avoid modular products
    // by zero, and use coefficient copying for unit coefficients. Gate the
    // extra tests once so dense moduli retain their original inner loop.
    if c[1] == 0 || c[2] == 0 {
        for degree in (4..=6).rev() {
            let high = t[degree];
            if high == 0 { continue; }
            if c[0] != 0 {
                let x = if c[0] == 1 { high } else { vmul(high, c[0], p) };
                t[degree - 4] = quadratic_sub(t[degree - 4], x, p);
            }
            if c[1] != 0 {
                let x = if c[1] == 1 { high } else { vmul(high, c[1], p) };
                t[degree - 3] = quadratic_sub(t[degree - 3], x, p);
            }
            if c[2] != 0 {
                let x = if c[2] == 1 { high } else { vmul(high, c[2], p) };
                t[degree - 2] = quadratic_sub(t[degree - 2], x, p);
            }
            if c[3] != 0 {
                let x = if c[3] == 1 { high } else { vmul(high, c[3], p) };
                t[degree - 1] = quadratic_sub(t[degree - 1], x, p);
            }
        }
        return [t[0], t[1], t[2], t[3]];
    }
    for degree in (4..=6).rev() {
        let high = t[degree];
        if high == 0 { continue; }
        for j in 0..4 {
            t[degree - 4 + j] = quadratic_sub(
                t[degree - 4 + j], vmul(high, c[j], p), p);
        }
    }
    [t[0], t[1], t[2], t[3]]
}
// A quartic coefficient is the sum of at most four products of canonical
// residues. For p=2^61-1, this sum is < 4*(p-1)^2 < 2^124: it fits u128.
// Two Mersenne folds suffice: after the first, the high part is at
// most four; the second is at most p+4, so one subtraction canonicalizes.
// Accumulate before reduction to avoid 16 modular reductions per
// multiplication (or 10 per symmetric square).
#[inline]
fn quartic_mersenne_reduce(value: u128) -> u64 {
    let p = MAX_PRIME as u128;
    let folded = (value & p) + (value >> 61);
    let folded = (folded & p) + (folded >> 61);
    if folded >= p { (folded - p) as u64 } else { folded as u64 }
}
#[inline]
fn quartic_mul(a: [u64; 4], b: [u64; 4], c: [u64; 4], p: u64) -> [u64; 4] {
    let mut t = [0u64; 7];
    if p <= 2_000_000_000 {
        // Every coefficient contains at most four products. Therefore
        // 4*(p-1)^2 <= 4*(2e9-1)^2 < 2^64: one reduction per column is safe.
        // This replaces 16 divisions/modular reductions with 7.
        for i in 0..4 {
            for j in 0..4 {
                t[i + j] += a[i] * b[j];
            }
        }
        for x in &mut t { *x %= p; }
    } else if p == MAX_PRIME {
        let mut columns = [0u128; 7];
        for i in 0..4 {
            for j in 0..4 {
                columns[i + j] += a[i] as u128 * b[j] as u128;
            }
        }
        for (result, value) in t.iter_mut().zip(columns) {
            *result = quartic_mersenne_reduce(value);
        }
    } else {
        for i in 0..4 {
            for j in 0..4 {
                t[i + j] = cubic_add(t[i + j], vmul(a[i], b[j], p), p);
            }
        }
    }
    quartic_reduce(t, c, p)
}
#[inline]
fn quartic_square(a: [u64; 4], c: [u64; 4], p: u64) -> [u64; 4] {
    let mut t = [0u64; 7];
    if p <= 2_000_000_000 {
        // Symmetry: 4 squares + 6 doubled cross terms. The accumulated
        // value for any column is bounded by 4*(p-1)^2 < 2^64.
        for i in 0..4 {
            t[2 * i] += a[i] * a[i];
            for j in (i + 1)..4 {
                t[i + j] += 2 * a[i] * a[j];
            }
        }
        for x in &mut t { *x %= p; }
    } else if p == MAX_PRIME {
        let mut columns = [0u128; 7];
        for i in 0..4 {
            columns[2 * i] += a[i] as u128 * a[i] as u128;
            for j in (i + 1)..4 {
                columns[i + j] += 2 * a[i] as u128 * a[j] as u128;
            }
        }
        for (result, value) in t.iter_mut().zip(columns) {
            *result = quartic_mersenne_reduce(value);
        }
    } else {
        for i in 0..4 {
            t[2 * i] = cubic_add(t[2 * i], vmul(a[i], a[i], p), p);
            for j in (i + 1)..4 {
                let product = vmul(a[i], a[j], p);
                t[i + j] = cubic_add(
                    t[i + j], cubic_add(product, product, p), p);
            }
        }
    }
    quartic_reduce(t, c, p)
}
#[inline]
fn quartic_coordinates(a: &[u64]) -> [u64; 4] {
    [a.first().copied().unwrap_or(0), a.get(1).copied().unwrap_or(0),
     a.get(2).copied().unwrap_or(0), a.get(3).copied().unwrap_or(0)]
}
#[inline]
fn quartic_trim(a: [u64; 4]) -> Vec<u64> {
    trim(a.to_vec())
}
// This verifier-only addition chain is for the two exact exponents used by
// Frobenius and quadratic-character splitting over p = 2^61 - 1.
// Concatenating (2^a-1) and (2^b-1) requires b squarings and one multiply:
// (2^a-1)*2^b + (2^b-1) = 2^(a+b)-1.
// Compared with ordinary binary exponentiation, 60/61-bit exponents retain
// 59/60 squares but require only 8/9 quotient-ring multiplies, not 59/60.
// The producer's recorded field-operation trace and certificate stay intact.
#[inline]
fn quartic_mersenne_exponent_chain(
    base: [u64; 4], exponent: u64, modulus: [u64; 4],
) -> [u64; 4] {
    debug_assert!(exponent == MAX_PRIME || exponent == MAX_PRIME >> 1);
    #[inline]
    fn append(a: [u64; 4], b: [u64; 4], bits: usize, c: [u64; 4]) -> [u64; 4] {
        let mut squared = a;
        for _ in 0..bits {
            squared = quartic_square(squared, c, MAX_PRIME);
        }
        quartic_mul(squared, b, c, MAX_PRIME)
    }
    let x2 = append(base, base, 1, modulus);
    let x4 = append(x2, x2, 2, modulus);
    let x8 = append(x4, x4, 4, modulus);
    let x16 = append(x8, x8, 8, modulus);
    let x32 = append(x16, x16, 16, modulus);
    let x48 = append(x32, x16, 16, modulus);
    let x56 = append(x48, x8, 8, modulus);
    let x60 = append(x56, x4, 4, modulus);
    if exponent == MAX_PRIME {
        append(x60, base, 1, modulus)
    } else {
        x60
    }
}
fn vpower_quartic(base: &[u64], mut exponent: u64, modulus: &[u64], p: u64)
    -> Result<Vec<u64>, String>
{
    debug_assert_eq!(modulus.len(), 5);
    let lead = modulus[4];
    if lead == 0 { return Err("division by zero leading coefficient".into()); }
    let inverse = if lead == 1 { 1 } else { vinv(lead, p) };
    let c = if lead == 1 {
        [modulus[0], modulus[1], modulus[2], modulus[3]]
    } else {
        [vmul(modulus[0], inverse, p), vmul(modulus[1], inverse, p),
         vmul(modulus[2], inverse, p), vmul(modulus[3], inverse, p)]
    };
    // Horner reduction of an arbitrary-degree base without a quotient.
    let mut power = [0u64; 4];
    for &coefficient in base.iter().rev() {
        let high = power[3];
        power = [
            quadratic_sub(coefficient, vmul(high, c[0], p), p),
            quadratic_sub(power[0], vmul(high, c[1], p), p),
            quadratic_sub(power[1], vmul(high, c[2], p), p),
            quadratic_sub(power[2], vmul(high, c[3], p), p),
        ];
    }
    // The Mersenne field produces these exponents in the principal
    // Frobenius and Cantor-Zassenhaus split checks. For other fields or
    // exponent shapes, retain the measured binary-power path.
    if p == MAX_PRIME && (exponent == MAX_PRIME || exponent == MAX_PRIME >> 1) {
        return Ok(quartic_trim(quartic_mersenne_exponent_chain(power, exponent, c)));
    }
    let mut accumulator = [1, 0, 0, 0];
    while exponent != 0 {
        if exponent & 1 == 1 {
            accumulator = quartic_mul(accumulator, power, c, p);
        }
        exponent >>= 1;
        if exponent != 0 {
            power = quartic_square(power, c, p);
        }
    }
    Ok(quartic_trim(accumulator))
}
#[inline]
fn vquartic_replay_mul(a: &[u64], b: &[u64], modulus: &[u64], p: u64) -> Vec<u64> {
    debug_assert_eq!(modulus.len(), 5);
    quartic_trim(quartic_mul(
        quartic_coordinates(a), quartic_coordinates(b),
        [modulus[0], modulus[1], modulus[2], modulus[3]], p))
}
#[inline]
fn vquartic_replay_square(a: &[u64], modulus: &[u64], p: u64) -> Vec<u64> {
    debug_assert_eq!(modulus.len(), 5);
    quartic_trim(quartic_square(
        quartic_coordinates(a),
        [modulus[0], modulus[1], modulus[2], modulus[3]], p))
}

fn vpower_quadratic(base: &[u64], mut exponent: u64, modulus: &[u64], p: u64)
    -> Result<Vec<u64>, String>
{
    debug_assert_eq!(modulus.len(), 3);
    let lead = modulus[2];
    if lead == 0 { return Err("division by zero leading coefficient".into()); }
    let inverse = if lead == 1 { 1 } else { vinv(lead, p) };
    let c = if lead == 1 { modulus[0] } else { vmul(modulus[0], inverse, p) };
    let d = if lead == 1 { modulus[1] } else { vmul(modulus[1], inverse, p) };
    // Horner reduction also handles degree >2 bases, without an allocated
    // quotient: (a+b*x)*x+v = (v-b*c)+(a-b*d)*x.
    let (mut a, mut b) = (0, 0);
    for &value in base.iter().rev() {
        let next_a = quadratic_sub(value, vmul(b, c, p), p);
        let next_b = quadratic_sub(a, vmul(b, d, p), p);
        a = next_a;
        b = next_b;
    }
    let mut residue = (1, 0);
    let mut power = (a, b);
    while exponent != 0 {
        if exponent & 1 == 1 {
            residue = quadratic_mul(residue, power, c, d, p);
        }
        exponent >>= 1;
        if exponent != 0 {
            power = quadratic_square(power, c, d, p);
        }
    }
    Ok(trim(vec![residue.0, residue.1]))
}

fn vpower(base: &[u64], mut e: u64, m: &[u64], p: u64) -> Result<Vec<u64>, String> {
    // F_p[x] / (c + d*x) is F_p. Reduce the base by Horner evaluation at
    // -c/d, then use a scalar power rather than repeatedly allocating
    // polynomials and doing synthetic division at every exponent bit.
    // This remains independent of the producer's field-operation trace.
    if m.len() == 2 {
        if m[1] == 0 {
            return Err("division by zero polynomial".into());
        }
        let minus_constant = if m[0] == 0 { 0 } else { p - m[0] };
        let root = if m[1] == 1 {
            minus_constant
        } else {
            vmul(minus_constant, vinv(m[1], p), p)
        };
        let mut value = 0u64;
        for &coefficient in base.iter().rev() {
            let sum = vmul(value, root, p) + coefficient;
            // Coefficients and products are canonical; 2*p - 2 fits u64.
            value = if sum >= p { sum - p } else { sum };
        }
        let result = vpow(value, e, p);
        return Ok(if result == 0 { vec![] } else { vec![result] });
    }
    // Specialized quadratic modular arithmetic; producer and verifier remain
    // independent. Applies to both monic and nonmonic quadratic certificates.
    if m.len() == 3 {
        return vpower_quadratic(base, e, m, p);
    }
    if m.len() == 4 {
        return vpower_cubic(base, e, m, p);
    }
    // Preserve the validated small-field generic path; p=2^61-1 uses
    // bounded u128 Mersenne column accumulation in the quartic helpers.
    if m.len() == 5 && ((p >= (1u64 << 24) && p <= u32::MAX as u64)
        || p == MAX_PRIME) {
        return vpower_quartic(base, e, m, p);
    }
    // Polynomial remainder is invariant under scaling the modulus by a
    // nonzero field element. Normalize a nonmonic modulus ONCE rather than
    // redoing extended-Euclid inversion at every vdiv in the exponent loop.
    // Leave monic inputs borrowed, avoiding an extra clone on the hot path.
    let normalized;
    let m = if m.last().is_some_and(|&lead| lead != 1) {
        normalized = vmonic(m.to_vec(), p);
        normalized.as_slice()
    } else {
        m
    };
    let mut r = vpower_reduce(vec![1], m, p)?;
    let mut b = vpower_reduce(base.to_vec(), m, p)?;
    while e > 0 {
        if e & 1 == 1 {
            r = vpower_reduce(vproduct(&r, &b, p), m, p)?;
        }
        e >>= 1;
        // Preserve the omitted terminal square and the independent
        // symmetric verifier squaring algorithm.
        if e != 0 {
            b = vpower_reduce(vsquare(&b, p), m, p)?;
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
    // A verified PowerEnd with this binding can also certify Finish's
    // X^p mod f without repeating the entire exponentiation.
    initial_frobenius: bool,
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
    // These are independently reconstructed results of fully replayed
    // PowerStep/GcdStep events, not assertions accepted from the producer.
    // Retain only computations whose original inputs exactly match the
    // final certificate's Frobenius and linear-part obligations.
    let mut verified_frobenius: Option<Vec<u64>> = None;
    let mut verified_linear_part: Option<Vec<u64>> = None;
    let mut initial_gcd_id: Option<u64> = None;
    // A prior Division event has already been checked using
    // independent arithmetic. Keep its verified remainder for the following
    // matching GcdStep and its verified quotient for an accepted SplitEnd. Borrow
    // certificate operands, and own only the computed remainder.
    let mut verified_division: Option<(&[u64], &[u64], &[u64], &[u64], Vec<u64>)> = None;
    // Source-exact Rust A/B identified two cache-overhead crossovers:
    // linear/quadratic cases (too little work to amortize matching), and
    // degree-eight certificates over small prime fields (fast division).
    // Keep the original independent replay there, and reuse independently
    // checked division results for the remaining quartic/higher workloads.
    let reuse_divisions = report.polynomial.len() >= 5
        && !(report.polynomial.len() >= 9 && p <= 65_521);
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
        if index == 0 {
            match event {
                RootEvent::Begin { p: event_p, polynomial, seed }
                    if *event_p == p
                        && polynomial == &report.polynomial
                        && *seed == report.seed => {}
                _ => return Err("root trace header differs".into()),
            }
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
                if vverified_product(a, b, p) != *product {
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
                // A matching GcdStep or accepted SplitEnd can reuse this
                // independently verified identity even across other events.
                if reuse_divisions {
                    verified_division = Some((
                        a.as_slice(), b.as_slice(),
                        quotient.as_slice(), remainder.as_slice(), verified_r,
                    ));
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
                // Replay with the same residue classes and original event
                // values; a unit-scaled monic modulus reduces the same
                // polynomials, but avoids repeated lead inversions.
                let monic = vmonic(modulus.clone(), p);
                let acc = vpower_reduce(vec![1], &monic, p)?;
                let reduced_base = vpower_reduce(base.clone(), &monic, p)?;
                powers.insert(
                    *id,
                    PowerState {
                        acc,
                        base: reduced_base,
                        exponent: *exponent,
                        modulus: monic,
                        initial_frobenius: base.as_slice() == [0, 1]
                            && *exponent == p && modulus == &report.polynomial,
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
                    state.acc = if state.modulus.len() == 3 && p >= (1u64 << 24) {
                        vquadratic_replay_mul(&state.acc, &state.base, &state.modulus, p)
                    } else if state.modulus.len() == 4 && p >= (1u64 << 24) {
                        vcubic_replay_mul(&state.acc, &state.base, &state.modulus, p)
                    } else {
                        vpower_reduce(vproduct(&state.acc, &state.base, p), &state.modulus, p)?
                    };
                }
                let next_exponent = state.exponent >> 1;
                // Either the original fully traced terminal square, or the
                // shortened equivalent with an unused square omitted. The
                // latter is accepted only when the recorded terminal base
                // exactly matches the previously verified base. Every
                // nonterminal square is still independently recomputed.
                if next_exponent != 0 || base.as_slice() != state.base.as_slice() {
                    state.base = if state.modulus.len() == 3 && p >= (1u64 << 24) {
                        vquadratic_replay_square(&state.base, &state.modulus, p)
                    } else if state.modulus.len() == 4 && p >= (1u64 << 24) {
                        vcubic_replay_square(&state.base, &state.modulus, p)
                    } else {
                        vpower_reduce(vproduct(&state.base, &state.base, p), &state.modulus, p)?
                    };
                }
                state.exponent = next_exponent;
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
                if state.initial_frobenius {
                    verified_frobenius = Some(state.acc);
                }
            }
            RootEvent::GcdBegin { id, a, b } => {
                canonical(a, p)?;
                canonical(b, p)?;
                if *id != next_id {
                    return Err("nonconsecutive computation id".into());
                }
                next_id += 1;
                if let Some(xp) = verified_frobenius.as_ref() {
                    let mut expected_xpx = xp.clone();
                    expected_xpx.resize(expected_xpx.len().max(2), 0);
                    expected_xpx[1] = if expected_xpx[1] == 0 {
                        p - 1
                    } else {
                        expected_xpx[1] - 1
                    };
                    if a == &report.polynomial && b == &trim(expected_xpx) {
                        initial_gcd_id = Some(*id);
                    }
                }
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
                let cached = if reuse_divisions {
                    verified_division.take().and_then(
                        |(dividend, divisor, checked_q, checked_r, computed_r)| {
                            if dividend == state.0.as_slice()
                                && divisor == state.1.as_slice()
                                && checked_q == quotient.as_slice()
                                && checked_r == remainder.as_slice()
                            {
                                Some(computed_r)
                            } else {
                                None
                            }
                        },
                    )
                } else {
                    None
                };
                let verified_r = if let Some(remainder) = cached {
                    remainder
                } else {
                    // Missing or nonmatching identities must still be independently
                    // checked, with no reliance on the producer's assertion.
                    let (verified_q, verified_r) = vdiv(&state.0, &state.1, p)?;
                    if verified_q != *quotient || verified_r != *remainder {
                        return Err("Euclidean step failed".into());
                    }
                    verified_r
                };
                // Transfer the independently calculated remainder instead
                // of recomputing division and cloning the event data.
                let previous_b = std::mem::replace(&mut state.1, verified_r);
                state.0 = previous_b;
            }
            RootEvent::GcdEnd { id, value } => {
                canonical(value, p)?;
                let (a, b) = gcds.remove(id).ok_or("missing gcd begin")?;
                if !b.is_empty() || vmonic(a, p) != *value {
                    return Err("incomplete Euclidean chain".into());
                }
                if initial_gcd_id == Some(*id) {
                    verified_linear_part = Some(value.clone());
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
                let split_quotient_valid = if valid {
                    if reuse_divisions {
                        match verified_division.take() {
                            Some((dividend, divisor, checked_q, checked_r, _))
                                if dividend == h.as_slice()
                                    && divisor == g.as_slice()
                                    && checked_q == quotient.as_slice()
                                    && checked_r.is_empty() => true,
                            _ => vdiv(&h, &g, p)? == (quotient.clone(), vec![]),
                        }
                    } else {
                        vdiv(&h, &g, p)? == (quotient.clone(), vec![])
                    }
                } else {
                    true
                };
                if h != *polynomial
                    || g != *factor
                    || valid != *accepted
                    || !split_quotient_valid
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
                    // When replay has verified the exact original
                    // (X, exponent=p, modulus=f) computation, reusing its
                    // independently checked terminal value is equivalent
                    // to repeating the same arithmetic here.
                    let xp = if let Some(cached) = verified_frobenius.as_ref() {
                        cached.clone()
                    } else {
                        vpower(&[0, 1], p, &report.polynomial, p)?
                    };
                    if xp != *xp_mod_f {
                        return Err("X^p residue certificate failed".into());
                    }
                    let mut xm = xp;
                    xm.resize(xm.len().max(2), 0);
                    xm[1] = (xm[1] + p - 1) % p;
                    let g = if let Some(cached) = verified_linear_part.as_ref() {
                        cached.clone()
                    } else {
                        vgcd(report.polynomial.clone(), trim(xm), p)?
                    };
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
    fn round111_reused_frobenius_and_gcd_reject_modified_certificates() {
        let p = 65_521u64;
        let f = vproduct(
            &vproduct(&[p - 3, 1], &[p - 7, 1], p),
            &vproduct(&[p - 11, 1], &[p - 19, 1], p), p);
        let report = run(&f, p);
        assert!(report.complete());
        verify(&report).unwrap();
        assert_eq!(report.roots, vec![3, 7, 11, 19]);

        let mut forged = report.clone();
        for event in &mut forged.events {
            if let RootEvent::PowerEnd { value, .. } = event {
                if !value.is_empty() {
                    value[0] = (value[0] + 1) % p;
                    break;
                }
            }
        }
        assert!(verify(&forged).is_err(), "forged replayed power accepted");

        let mut forged = report.clone();
        for event in &mut forged.events {
            if let RootEvent::GcdEnd { value, .. } = event {
                if !value.is_empty() {
                    value[0] = (value[0] + 1) % p;
                    break;
                }
            }
        }
        assert!(verify(&forged).is_err(), "forged replayed gcd accepted");

        let mut forged = report.clone();
        for event in &mut forged.events {
            if let RootEvent::Finish { xp_mod_f, .. } = event {
                xp_mod_f[0] = (xp_mod_f[0] + 1) % p;
                break;
            }
        }
        assert!(verify(&forged).is_err(), "forged Frobenius finish accepted");

        let mut forged = report.clone();
        for event in &mut forged.events {
            if let RootEvent::Finish { linear_part, .. } = event {
                linear_part[0] = (linear_part[0] + 1) % p;
                break;
            }
        }
        assert!(verify(&forged).is_err(), "forged linear-part finish accepted");
    }

    #[test]
    fn round106_nonmonic_power_uses_single_normalization_and_monic_divisions() {
        // 2*(x-3)*(x-7) has exactly the roots 3 and 7 over each field.
        // PowerBegin must continue binding the *original* nonmonic input;
        // the producer's divisions inside that power must use a monic
        // equivalent, and the independent certificate verifier must accept it.
        for p in [65_521u64, 1_000_000_007, MAX_PRIME] {
            let input = vec![42, p - 20, 2];
            let report = solve(
                &input, p, 7,
                RootLimits {
                    field_work: MAX_WORK,
                    split_attempts: 1000,
                    events: MAX_EVENTS,
                    degree: MAX_DEGREE,
                },
                &mut |_| Ok(()),
            ).unwrap();
            assert!(report.complete(), "p={p} stop={:?}", report.stop);
            verify(&report).unwrap();
            assert_eq!(report.roots, vec![3, 7], "p={p}");
            let mut in_original_power = false;
            let mut checked_divisions = 0;
            let mut normalizations = 0;
            for event in &report.events {
                match event {
                    RootEvent::PowerBegin { modulus, .. } if modulus == &input => {
                        in_original_power = true;
                    }
                    RootEvent::PowerEnd { .. } if in_original_power => {
                        in_original_power = false;
                    }
                    RootEvent::InverseBegin { value } if in_original_power && *value == 2 => {
                        normalizations += 1;
                    }
                    RootEvent::Division { b, .. } if in_original_power => {
                        assert_eq!(b.last(), Some(&1), "nonmonic power division p={p}");
                        checked_divisions += 1;
                    }
                    _ => {}
                }
            }
            assert!(checked_divisions > 0, "missing power division p={p}");
            assert_eq!(normalizations, 1, "repeated power inversion p={p}");
        }
    }


    #[test]
    fn round102_cubic_quotient_power_matches_independent_division() {
        let mut rng = StdRng::seed_from_u64(0x726f_6f74_3130_3231);
        for p in [7u64, 65_521, 1_000_000_007, MAX_PRIME] {
            for nonmonic in [false, true] {
                for _case in 0..48 {
                    let lead = if nonmonic { rng.gen_range(2..p) } else { 1 };
                    let modulus = vec![rng.gen_range(0..p), rng.gen_range(0..p),
                                       rng.gen_range(0..p), lead];
                    let width = rng.gen_range(0..12);
                    let base: Vec<u64> = (0..width).map(|_| rng.gen_range(0..p)).collect();
                    let exponent = rng.gen_range(0..4096u64);
                    let normalized = vmonic(modulus.clone(), p);
                    let mut reference = vec![1];
                    let mut power = vdiv(&base, &normalized, p).unwrap().1;
                    let mut bits = exponent;
                    while bits != 0 {
                        if bits & 1 == 1 {
                            reference = vdiv(&vproduct(&reference, &power, p),
                                             &normalized, p).unwrap().1;
                        }
                        bits >>= 1;
                        if bits != 0 {
                            power = vdiv(&vproduct(&power, &power, p),
                                         &normalized, p).unwrap().1;
                        }
                    }
                    assert_eq!(vpower(&base, exponent, &modulus, p).unwrap(), reference);
                    for i in 0..3 {
                        let a: Vec<u64> = (0..i+1).map(|_| rng.gen_range(0..p)).collect();
                        let b: Vec<u64> = (0..i+1).map(|_| rng.gen_range(0..p)).collect();
                        let old_mul = vdiv(&vproduct(&a, &b, p), &normalized, p).unwrap().1;
                        let old_sq = vdiv(&vproduct(&a, &a, p), &normalized, p).unwrap().1;
                        assert_eq!(vcubic_replay_mul(&a, &b, &normalized, p), old_mul);
                        assert_eq!(vcubic_replay_square(&a, &normalized, p), old_sq);
                    }
                }
            }
        }
    }

    #[test]
    fn round101_verified_division_cache_rejects_modified_gcd_step() {
        for p in [65_521u64, 1_000_000_007, MAX_PRIME] {
            let polynomial = vec![p - 1, 0, 1];
            let report = run(&polynomial, p);
            verify(&report).unwrap();
            let mut tampered = run(&polynomial, p);
            let step = tampered.events.iter_mut().find_map(|event| {
                if let RootEvent::GcdStep { quotient, .. } = event {
                    Some(quotient)
                } else {
                    None
                }
            }).expect("root solver must perform a GCD step");
            if step.is_empty() {
                step.push(1);
            } else {
                step[0] = (step[0] + 1) % p;
                if step.last() == Some(&0) { step.pop(); }
            }
            assert!(verify(&tampered).is_err(), "forged quotient p={p}");
        }
    }

    #[test]
    fn round99_quadratic_power_step_replay_matches_generic_polynomials() {
        // Independent generic multiply/divide is the oracle; exercise sparse
        // and zero residues, arbitrary quadratic coefficients, and Mersenne
        // arithmetic. Both helpers must preserve canonical trimming.
        for p in [3u64, 65_521, 1_000_000_007, MAX_PRIME] {
            let mut seed = 0x494e_4445_5045_4e44u64 ^ p;
            for trial in 0..8192 {
                let mut next = || {
                    seed = seed.wrapping_mul(6364136223846793005)
                        .wrapping_add(1442695040888963407);
                    seed % p
                };
                let mut a = trim(vec![next(), next()]);
                let mut b = trim(vec![next(), next()]);
                if trial % 17 == 0 { a.clear(); }
                if trial % 23 == 0 { b = vec![1]; }
                let modulus = vec![next(), next(), 1];
                assert_eq!(
                    vquadratic_replay_mul(&a, &b, &modulus, p),
                    vpower_reduce(vproduct(&a, &b, p), &modulus, p).unwrap(),
                    "quadratic replay multiply p={p} trial={trial}"
                );
                assert_eq!(
                    vquadratic_replay_square(&a, &modulus, p),
                    vpower_reduce(vproduct(&a, &a, p), &modulus, p).unwrap(),
                    "quadratic replay square p={p} trial={trial}"
                );
            }
        }
    }

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
    fn round98_quadratic_power_matches_independent_long_division() {
        fn reference(base: &[u64], mut e: u64, m: &[u64], p: u64)
            -> Result<Vec<u64>, String>
        {
            let mut acc = vdiv(&[1], m, p)?.1;
            let mut value = vdiv(base, m, p)?.1;
            while e > 0 {
                if e & 1 == 1 {
                    acc = vdiv(&vproduct(&acc, &value, p), m, p)?.1;
                }
                e >>= 1;
                if e > 0 {
                    value = vdiv(&vsquare(&value, p), m, p)?.1;
                }
            }
            Ok(acc)
        }
        let mut state = 0x524f_4f54_2026_1198u64;
        for p in [7u64, 65_521, 1_000_000_007, MAX_PRIME] {
            for case in 0..160usize {
                let mut next = || {
                    state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
                    state % p
                };
                let constant = if case % 7 == 0 { 0 } else { next() };
                let linear = if case % 11 == 0 { 0 } else { next() };
                let lead = if case % 2 == 0 { 1 } else { 1 + next() % (p - 1) };
                let modulus = vec![constant, linear, lead];
                let mut base: Vec<u64> = (0..case % 10).map(|_| next()).collect();
                if case % 13 == 0 { base.fill(0); }
                for exponent in [0, 1, 2, 17, (p - 1) / 2, p, u64::MAX] {
                    let before = reference(&base, exponent, &modulus, p).unwrap();
                    let after = vpower(&base, exponent, &modulus, p).unwrap();
                    assert_eq!(after, before,
                               "p={p} case={case} e={exponent} m={modulus:?} base={base:?}");
                }
            }
        }
        assert!(vpower(&[1], 5, &[1, 2, 0], 7).is_err());
    }
    #[test]
    fn round98_quadratic_certificate_replay() {
        for p in [7u64, 65_521, 1_000_000_007, MAX_PRIME] {
            for polynomial in [vec![p - 1, 0, 1], vec![1, 1, 1], vec![2, 1, 2]] {
                let report = run(&polynomial, p);
                verify(&report).unwrap();
            }
        }
    }

    #[test]
    fn round96_linear_modulus_scalar_power_matches_independent_polynomial_arithmetic() {
        // Freeze the polynomial-ring method as an independent oracle for
        // every linear-modulus specialization, including zero and 0^0.
        fn reference(base: &[u64], mut exponent: u64, modulus: &[u64], p: u64)
            -> Result<Vec<u64>, String>
        {
            let mut r = vdiv(&[1], modulus, p)?.1;
            let mut b = vdiv(base, modulus, p)?.1;
            while exponent > 0 {
                if exponent & 1 == 1 {
                    r = vdiv(&vproduct(&r, &b, p), modulus, p)?.1;
                }
                exponent >>= 1;
                if exponent != 0 {
                    b = vdiv(&vsquare(&b, p), modulus, p)?.1;
                }
            }
            Ok(r)
        }
        let mut seed = 0x524f_4f54_2026_1196u64;
        for p in [7u64, 65_521, 1_000_000_007, MAX_PRIME] {
            for case in 0..128u64 {
                let constant = if case % 7 == 0 { 0 } else { seed % p };
                seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
                let leading = if case & 1 == 0 { 1 } else { 1 + seed % (p - 1) };
                let modulus = vec![constant, leading];
                let mut base = vec![0; 1 + case as usize % 9];
                for value in &mut base {
                    seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
                    *value = if case % 5 == 0 { 0 } else { seed % p };
                }
                if case % 11 == 0 { base.clear(); }
                for exponent in [0, 1, 2, 17, (p - 1) / 2, p] {
                    assert_eq!(vpower(&base, exponent, &modulus, p).unwrap(),
                               reference(&base, exponent, &modulus, p).unwrap(),
                               "p={p} case={case} exponent={exponent}");
                }
            }
        }
        assert!(vpower(&[1], 7, &[1, 0], 7).is_err());
    }

    #[test]
    fn round96_linear_root_certificate_replay() {
        for p in [7u64, 65_521, 1_000_000_007, MAX_PRIME] {
            for root in [0, 1, 2, 6, p - 1] {
                let polynomial = vec![if root == 0 { 0 } else { p - root }, 1];
                let report = run(&polynomial, p);
                assert!(report.complete());
                verify(&report).unwrap();
            }
        }
    }

    #[test]
    fn round95_nonmonic_power_reduction_matches_legacy_reference() {
        // Frozen reference performs every reduction using the original
        // (possibly nonmonic) modulus and always materializes quotient pairs.
        fn legacy(base: &[u64], mut exponent: u64, modulus: &[u64], p: u64)
            -> Result<Vec<u64>, String>
        {
            let mut acc = vdiv(&[1], modulus, p)?.1;
            let mut b = vdiv(base, modulus, p)?.1;
            while exponent != 0 {
                if exponent & 1 == 1 {
                    acc = vdiv(&vproduct(&acc, &b, p), modulus, p)?.1;
                }
                exponent >>= 1;
                if exponent != 0 {
                    b = vdiv(&vsquare(&b, p), modulus, p)?.1;
                }
            }
            Ok(acc)
        }
        let mut seed = 0x524f_4f54_2026_1195u64;
        for p in [7u64, 65_521, 1_000_000_007, MAX_PRIME] {
            for degree in [1usize, 2, 4, 8] {
                for case in 0..24u64 {
                    let mut modulus = vec![0u64; degree + 1];
                    for coefficient in &mut modulus {
                        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
                        *coefficient = seed % p;
                    }
                    modulus[degree] = if case & 1 == 0 { 1 } else { 2 + seed % (p - 2) };
                    if case % 3 == 0 {
                        for coefficient in &mut modulus[1..degree] {
                            *coefficient = 0;
                        }
                    }
                    let base = vec![seed.wrapping_add(case) % p, 1];
                    for exponent in [0, 1, 17, (p - 1) / 2] {
                        assert_eq!(
                            vpower(&base, exponent, &modulus, p).unwrap(),
                            legacy(&base, exponent, &modulus, p).unwrap(),
                            "p={p}, degree={degree}, case={case}, exponent={exponent}"
                        );
                    }
                }
            }
        }
        assert!(vpower(&[1], 13, &[], 7).is_err());
    }

    #[test]
    fn round95_verify_nonmonic_root_certificates() {
        for p in [7u64, 13, 65_521, 1_000_000_007, MAX_PRIME] {
            for mut polynomial in [vec![1, 2, 3], vec![2, 0, 3, 2], vec![3, 1, 0, 2]] {
                for coefficient in &mut polynomial {
                    *coefficient %= p;
                }
                let report = run(&polynomial, p);
                verify(&report).unwrap();
                assert!(report.complete(), "p={p}, polynomial={polynomial:?}");
            }
        }
    }

    #[test]
    fn round93_traced_division_matches_independent_quotient_remainder() {
        // Exhaust both monic and nonmonic cases, with sparse and dense
        // divisors. The verifier arithmetic is independent of Engine::div.
        let mut seed = 0x524f_4f54_2026_1193u64;
        for p in [7u64, 65_521, 1_000_000_007, MAX_PRIME] {
            for case in 0..384usize {
                let alen = 1 + case % 11;
                let blen = 1 + (case / 3) % 9;
                let mut a = vec![0; alen];
                let mut b = vec![0; blen];
                for coeff in a.iter_mut().chain(b.iter_mut()) {
                    seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
                    *coeff = seed % p;
                }
                a[alen - 1] = 1 + a[alen - 1] % (p - 1);
                b[blen - 1] = if case & 1 == 0 { 1 } else { p - 1 };
                if blen > 2 && case % 3 == 0 {
                    for coefficient in &mut b[1..blen - 1] {
                        *coefficient = 0;
                    }
                }
                let reference = vdiv(&a, &b, p).unwrap();
                let mut sink = |_event: &RootEvent| Ok(());
                let mut engine = Engine {
                    report: RootReport {
                        schema: "prime-root-trace/v1".into(),
                        p,
                        polynomial: vec![1, 1],
                        seed: 7,
                        limits: limits(),
                        stop: RootStop::Complete,
                        counters: RootCounters::default(),
                        events: vec![],
                        unacknowledged: None,
                        sink_error: None,
                        roots: vec![],
                    },
                    sink: &mut sink,
                    next_id: 0,
                };
                let actual = engine.div(&a, &b).unwrap();
                assert_eq!(actual, reference, "p={p} case={case} a={a:?} b={b:?}");
                assert!(matches!(
                    engine.report.events.last(),
                    Some(RootEvent::Division { .. })
                ));
            }
        }
    }

    #[test]
    fn round92_skips_terminal_power_squares_but_preserves_complete_certificates() {
        for p in [7u64, 13u64, 65_521, 1_000_000_007, MAX_PRIME] {
            for mut f in [vec![1, 1], vec![1, 0, 1], vec![4, 9, 1]] {
                for coefficient in &mut f { *coefficient %= p; }
                let report = run(&f, p);
                assert!(report.complete(), "incomplete p={p}, f={f:?}");
                verify(&report).unwrap();
                let final_steps = report.events.iter().filter(
                    |e| matches!(e, RootEvent::PowerStep { exponent: 0, .. })
                ).count();
                assert!(final_steps > 0, "no final power step p={p}");
            }
        }
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
    fn binomial_products_match_independent_arithmetic_oracle() {
        let mut rng = StdRng::seed_from_u64(0x2026_1121_0091);
        for p in [65_521, 1_000_000_007, MAX_PRIME] {
            for width in [6, 8, 12, 16] {
                for case in 0..160 {
                    let mut binomial = vec![0; width];
                    binomial[0] = rng.gen_range(1..p);
                    binomial[width - 1] =
                        if case % 2 == 0 { 1 } else { rng.gen_range(1..p) };
                    let mut other: Vec<u64> =
                        (0..width).map(|_| rng.gen_range(0..p)).collect();
                    if case % 3 == 0 {
                        for i in (0..width).step_by(3) { other[i] = 0; }
                    }
                    let expected = reference_product(&binomial, &other, p);
                    assert_eq!(vverified_product(&binomial, &other, p), expected,
                        "left binomial p={p} width={width} case={case}");
                    assert_eq!(vverified_product(&other, &binomial, p), expected,
                        "right binomial p={p} width={width} case={case}");
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

#[cfg(test)]
mod round94_noop_reductions {
    use super::*;

    #[test]
    fn monic_roots_and_power_shortcuts_preserve_independent_certificates() {
        let limits = RootLimits {
            field_work: MAX_WORK,
            split_attempts: 1000,
            events: MAX_EVENTS,
            degree: MAX_DEGREE,
        };
        for p in [7u64, 13, 65_521, 1_000_000_007, MAX_PRIME] {
            let linear = vec![p - 3, 1];
            let quadratic = vproduct(&linear, &[p - 5, 1], p);
            for (polynomial, expected) in [
                (linear, vec![3]),
                (quadratic, vec![3, 5]),
            ] {
                for seed in [7u64, 19u64] {
                    let report = solve(&polynomial, p, seed, limits.clone(),
                        &mut |_| Ok(())).unwrap();
                    assert!(report.complete(),
                        "p={p} seed={seed} stop={:?}", report.stop);
                    verify(&report).unwrap();
                    assert_eq!(report.roots, expected,
                        "p={p} seed={seed} polynomial={polynomial:?}");
                }
            }
        }
    }

    #[test]
    fn short_modular_power_products_need_no_division_events() {
        let limits = RootLimits {
            field_work: MAX_WORK,
            split_attempts: 1000,
            events: MAX_EVENTS,
            degree: MAX_DEGREE,
        };
        // For degree-eight moduli, the first several binary powering
        // products have smaller degree than the modulus and need no
        // polynomial reduction. The verifier independently recomputes them.
        for p in [13u64, 65_521] {
            let polynomial = vec![1, 1, 0, 0, 0, 0, 0, 0, 1];
            let report = solve(&polynomial, p, 11, limits.clone(),
                &mut |_| Ok(())).unwrap();
            verify(&report).unwrap();
            assert!(report.events.iter().any(|event|
                matches!(event, RootEvent::PowerBegin { .. })));
        }
    }
}

#[cfg(test)]
mod round97_remainder_tests {
    use super::*;
    #[test]
    fn round97_quotient_free_power_reduction_matches_division_remainder() {
        let mut seed=0x97f0_2026_1121_0001u64;
        for p in [65_521u64, 1_000_000_007, MAX_PRIME] {
            for degree in [3usize,4,8] {
                for case in 0..1536 {
                    let mut modulus=vec![0u64;degree+1];
                    for c in &mut modulus[..degree] {
                        seed=seed.wrapping_mul(6364136223846793005).wrapping_add(1);
                        *c=seed%p;
                    }
                    modulus[degree]=1;
                    if case%4==0 { for c in &mut modulus[1..degree] { *c=0; } }
                    if case%4==1 { for c in &mut modulus[..degree] { *c=0; } }
                    let mut product=vec![0u64;case%(2*degree+3)];
                    for c in &mut product {
                        seed=seed.wrapping_mul(6364136223846793005).wrapping_add(1);
                        *c=seed%p;
                    }
                    if case%5==0 && !product.is_empty() { *product.last_mut().unwrap()=0; }
                    let expected=vdiv(&product,&modulus,p).unwrap().1;
                    let actual=vpower_reduce(product,&modulus,p).unwrap();
                    assert_eq!(actual,expected,"p={p} degree={degree} case={case}");
                }
            }
        }
    }
}


#[cfg(test)]
mod round103_quartic_quotient {
    use super::*;

    // Independent oracle: long division, generic multiplication and squaring.
    // It does not call vpower, quartic_mul, quartic_square, or quartic_reduce.
    fn reference_power(base: &[u64], mut e: u64, modulus: &[u64], p: u64) -> Vec<u64> {
        let m = vmonic(modulus.to_vec(), p);
        let mut r = vdiv(&[1], &m, p).unwrap().1;
        let mut b = vdiv(base, &m, p).unwrap().1;
        while e != 0 {
            if e & 1 == 1 { r = vdiv(&vproduct(&r, &b, p), &m, p).unwrap().1; }
            e >>= 1;
            if e != 0 { b = vdiv(&vproduct(&b, &b, p), &m, p).unwrap().1; }
        }
        r
    }

    #[test]
    fn round105_mersenne_quartic_addition_chain_matches_division() {
        // Independent polynomial multiplication/long division oracle.
        let p = MAX_PRIME;
        let mut rng = StdRng::seed_from_u64(0x726f_6f74_3130_3531);
        for nonmonic in [false, true] {
            for width in [0usize, 1, 2, 4, 7] {
                for _ in 0..16 {
                    let modulus = vec![
                        rng.gen_range(0..p), rng.gen_range(0..p),
                        rng.gen_range(0..p), rng.gen_range(0..p),
                        if nonmonic { rng.gen_range(2..p) } else { 1 },
                    ];
                    let base: Vec<_> = (0..width).map(|_| rng.gen_range(0..p)).collect();
                    for exponent in [p, (p - 1) / 2] {
                        assert_eq!(
                            vpower(&base, exponent, &modulus, p).unwrap(),
                            reference_power(&base, exponent, &modulus, p),
                            "nonmonic={nonmonic} width={width} exponent={exponent}",
                        );
                    }
                }
            }
        }
        // Extreme residues ensure intermediate products cannot be
        // specialized using unchecked assumptions about coefficient size.
        for coefficient in [0, 1, 2, p / 2, p - 2, p - 1] {
            let modulus = vec![p - 1, coefficient, 0, p - 1, 1];
            let base = vec![coefficient, p - 1, 0, coefficient];
            for exponent in [p, (p - 1) / 2] {
                assert_eq!(
                    vpower(&base, exponent, &modulus, p).unwrap(),
                    reference_power(&base, exponent, &modulus, p),
                );
            }
        }
    }
    #[test]
    fn round104_mersenne_quartic_column_extremes() {
        let p = MAX_PRIME;
        let values = [0u64, 1, 2, p / 2, p - 2, p - 1];
        for &x in &values {
            for &y in &values {
                for &z in &values {
                    let a = [x, y, z, p - 1];
                    let b = [y, p - 1, x, z];
                    let c = [z, x, y, p - 1];
                    let mut modulus = c.to_vec(); modulus.push(1);
                    let expected_product = vdiv(&vproduct(&a, &b, p), &modulus, p).unwrap().1;
                    let expected_square = vdiv(&vsquare(&a, p), &modulus, p).unwrap().1;
                    assert_eq!(quartic_trim(quartic_mul(a, b, c, p)), expected_product);
                    assert_eq!(quartic_trim(quartic_square(a, c, p)), expected_square);
                }
            }
        }
        for k in 0..16u128 {
            let value = 4 * (p as u128 - 1).pow(2) - k;
            assert_eq!(quartic_mersenne_reduce(value), (value % p as u128) as u64);
        }
    }
    #[test]
    fn quartic_multiply_square_horner_and_power_match_independent_long_division() {
        let mut state = 0x1030_aaaa_bbbb_ccccu64;
        for p in [7u64, 65_521, 1_000_000_007, MAX_PRIME] {
            for iteration in 0..512 {
                let mut next = || {
                    state ^= state << 13; state ^= state >> 7;
                    state ^= state << 17; state % p
                };
                let mut c = [0u64; 4];
                let mut a = [0u64; 4];
                let mut b = [0u64; 4];
                for j in 0..4 { c[j] = next(); a[j] = next(); b[j] = next(); }
                let mut modulus = c.to_vec();
                modulus.push(1);
                let a_poly = trim(a.to_vec());
                let b_poly = trim(b.to_vec());
                let direct = vdiv(&vproduct(&a_poly, &b_poly, p), &modulus, p).unwrap().1;
                assert_eq!(quartic_trim(quartic_mul(a, b, c, p)), direct, "mul p={p}");
                let direct_sq = vdiv(&vproduct(&a_poly, &a_poly, p), &modulus, p).unwrap().1;
                assert_eq!(quartic_trim(quartic_square(a, c, p)), direct_sq, "square p={p}");
                assert_eq!(vquartic_replay_mul(&a_poly, &b_poly, &modulus, p),
                    direct, "replay multiply p={p}");
                assert_eq!(vquartic_replay_square(&a_poly, &modulus, p),
                    direct_sq, "replay square p={p}");
                let base: Vec<u64> = (0..(iteration % 12)).map(|_| next()).collect();
                let exponent = (next() & 255) + (iteration % 7) as u64;
                let lead = if iteration % 2 == 0 { 1 } else { 2 };
                let mut nonmonic = c.iter().map(|&x| vmul(x, lead, p)).collect::<Vec<_>>();
                nonmonic.push(lead);
                let actual = vpower_quartic(&base, exponent, &nonmonic, p).unwrap();
                let expected = reference_power(&base, exponent, &nonmonic, p);
                assert_eq!(actual, expected,
                    "power p={p} iteration={iteration} e={exponent}");
                assert_eq!(vpower(&base, exponent, &nonmonic, p).unwrap(), expected);
            }
        }
    }
}

#[cfg(test)]
mod round100_traced_multiplication {
    use super::*;

    #[test]
    fn sparse_unit_and_zero_accumulator_elision_matches_wide_oracle() {
        // Engine arithmetic is tested independently of the verifier's
        // polynomial multiplication. The certificate Product still binds
        // the full result, even when identity operations are untraced.
        let mut seed = 0x726f_6f74_3130_3030u64;
        for p in [3u64, 65_521, 1_000_000_007, MAX_PRIME] {
            for width in [1usize, 2, 4, 8, 12] {
                for case in 0..128 {
                    let mut next = || {
                        seed = seed.wrapping_mul(6364136223846793005)
                            .wrapping_add(1442695040888963407);
                        seed % p
                    };
                    let mut a = vec![0; width];
                    let mut b = vec![0; width];
                    for i in 0..width {
                        a[i] = next();
                        b[i] = next();
                        if case % 4 == 0 && i % 2 == 0 { a[i] = 0; }
                        if case % 4 == 1 && i % 3 != 0 { b[i] = 0; }
                        if case % 4 == 2 && i % 2 == 0 { a[i] = 1; }
                        if case % 4 == 3 && i % 3 == 0 { b[i] = 1; }
                    }
                    let mut expected = vec![0u64; 2 * width - 1];
                    for (i, &x) in a.iter().enumerate() {
                        for (j, &y) in b.iter().enumerate() {
                            expected[i + j] = ((expected[i + j] as u128
                                + x as u128 * y as u128) % p as u128) as u64;
                        }
                    }
                    let expected = trim(expected);
                    let mut observer = |_: &RootEvent| -> Result<(), String> { Ok(()) };
                    let mut engine = Engine {
                        report: RootReport {
                            schema: "prime-root-trace/v1".into(),
                            p,
                            polynomial: vec![],
                            seed: 0,
                            limits: RootLimits {
                                field_work: MAX_WORK,
                                split_attempts: MAX_EVENTS,
                                events: MAX_EVENTS,
                                degree: MAX_DEGREE,
                            },
                            stop: RootStop::WorkLimit,
                            counters: RootCounters::default(),
                            events: vec![],
                            unacknowledged: None,
                            sink_error: None,
                            roots: vec![],
                        },
                        sink: &mut observer,
                        next_id: 0,
                    };
                    assert_eq!(engine.mul(&a, &b).unwrap(), expected,
                        "p={p}, width={width}, case={case}");
                    assert!(matches!(engine.report.events.last(),
                        Some(RootEvent::Product { product, .. }) if product == &expected));
                    for ev in &engine.report.events {
                        match ev {
                            RootEvent::Field { op: FieldOp::Mul, a, b, .. } =>
                                assert!(*a != 0 && *b != 0 && *a != 1 && *b != 1),
                            RootEvent::Field { op: FieldOp::Add, a, .. } =>
                                assert_ne!(*a, 0),
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn optimized_trace_still_verifies_end_to_end_sparse_and_dense_roots() {
        let limits = RootLimits {
            field_work: MAX_WORK, split_attempts: 1000,
            events: MAX_EVENTS, degree: MAX_DEGREE,
        };
        for p in [13u64, 65_521, 1_000_000_007, MAX_PRIME] {
            for f in [
                vec![p - 3, 1],
                vec![6, p - 5, 1],
                vec![1, 0, 1],
            ] {
                let report = solve(&f, p, 17, limits.clone(),
                    &mut |_| Ok(())).unwrap();
                assert!(report.complete(), "p={p} f={f:?} stop={:?}", report.stop);
                verify(&report).unwrap();
            }
        }
    }
}

#[cfg(test)]
mod round107_inverse_chain {
    use super::*;

    #[test]
    fn mersenne_inverse_chain_matches_independent_fermat_oracle() {
        let p = MAX_PRIME;
        let mut seed = 0x726f_6f74_3130_3731u64;
        let mut samples = vec![1, 2, 3, 5, p - 2, p - 1, p / 2];
        for _ in 0..1024 {
            seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
            samples.push(1 + seed % (p - 1));
        }
        for value in samples {
            let mut sink = |_: &RootEvent| Ok(());
            let report = RootReport {
                schema: "prime-root-trace/v1".into(),
                p,
                polynomial: vec![1, 1],
                seed: 7,
                limits: RootLimits { field_work: MAX_WORK, split_attempts: MAX_EVENTS,
                                     events: MAX_EVENTS, degree: MAX_DEGREE },
                stop: RootStop::Complete,
                counters: RootCounters::default(),
                events: vec![],
                unacknowledged: None,
                sink_error: None,
                roots: vec![],
            };
            let mut engine = Engine { report, sink: &mut sink, next_id: 0 };
            let actual = engine.inv(value).unwrap();
            assert_eq!(actual, vpow(value, p - 2, p), "value={value}");
            assert_eq!(vmul(actual, value, p), 1, "value={value}");
            if [1, 2, p - 1, p - 2].contains(&value) {
                assert_eq!(engine.report.counters.field_work(), 0);
            } else {
                assert_eq!(engine.report.counters.squares, 60);
                assert_eq!(engine.report.counters.multiplications, 10);
            }
        }
    }

    #[test]
    fn inverse_shortcuts_preserve_verified_root_certificates() {
        let limits = RootLimits { field_work: MAX_WORK, split_attempts: 1000,
                                  events: MAX_EVENTS, degree: MAX_DEGREE };
        for p in [7, 65_521, 1_000_000_007, MAX_PRIME] {
            for polynomial in [
                vec![6, p - 5, 1],       // monic (x-2)(x-3)
                vec![12 % p, p - (10 % p), 2], // twice that polynomial mod p
                vec![p - 2, 1],         // linear root 2
            ] {
                let report = solve(&polynomial, p, 13, limits.clone(),
                                   &mut |_| Ok(())).unwrap();
                assert!(report.complete(), "p={p}, stop={:?}", report.stop);
                verify(&report).unwrap();
            }
        }
    }
}

#[cfg(test)]
mod round108_unit_monomial_fastpaths {
    use super::*;

    #[test]
    fn identity_and_x_shift_products_preserve_independent_certificates() {
        // Direct producer-vs-independent-verifier oracle across field sizes,
        // narrow break-even cases, and both operand orders.
        let mut seed = 0x726f_6f74_3130_3831u64;
        for p in [7, 65_521, 1_000_000_007, MAX_PRIME] {
            for width in [1usize, 2, 3, 4, 8] {
                for case in 0..64u64 {
                    let mut polynomial = vec![0; width];
                    for coefficient in &mut polynomial {
                        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
                        *coefficient = seed % p;
                    }
                    polynomial[width - 1] = 1 + case % (p - 1);
                    for monomial in [&[1u64][..], &[0, 1][..]] {
                        for (a, b) in [(monomial, polynomial.as_slice()),
                                       (polynomial.as_slice(), monomial)] {
                            let mut sink = |_: &RootEvent| Ok(());
                            let report = RootReport {
                                schema: "prime-root-trace/v1".into(), p,
                                polynomial: vec![1, 1], seed: 7,
                                limits: RootLimits {
                                    field_work: MAX_WORK, split_attempts: MAX_EVENTS,
                                    events: MAX_EVENTS, degree: MAX_DEGREE,
                                },
                                stop: RootStop::Complete,
                                counters: RootCounters::default(),
                                events: vec![], unacknowledged: None,
                                sink_error: None, roots: vec![],
                            };
                            let mut engine = Engine { report, sink: &mut sink, next_id: 0 };
                            let actual = engine.mul(a, b).unwrap();
                            assert_eq!(actual, vproduct(a, b, p),
                                "p={p} width={width} case={case} a={a:?} b={b:?}");
                            assert_eq!(engine.report.counters.field_work(), 0);
                            assert!(matches!(engine.report.events.last(),
                                Some(RootEvent::Product { .. })));
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn quartic_mersenne_two_fold_and_sparse_reduction_oracle() {
        // Independent u128 division, not the optimized vmul/folding code.
        let p = MAX_PRIME;
        let mut seed = 0xa319_5432_fba9_87de_u64;
        let mut next = || {
            seed ^= seed << 7;
            seed ^= seed >> 9;
            seed ^= seed << 8;
            seed % p
        };
        let oracle = |a: [u64; 4], b: [u64; 4], c: [u64; 4]| {
            let mut t = [0u64; 7];
            for i in 0..4 {
                for j in 0..4 {
                    let sum = t[i + j] as u128 + a[i] as u128 * b[j] as u128;
                    t[i + j] = (sum % p as u128) as u64;
                }
            }
            for degree in (4..=6).rev() {
                let high = t[degree];
                for j in 0..4 {
                    let product = (high as u128 * c[j] as u128 % p as u128) as u64;
                    let old = t[degree - 4 + j];
                    t[degree - 4 + j] = if old >= product {
                        old - product
                    } else {
                        old + p - product
                    };
                }
            }
            [t[0], t[1], t[2], t[3]]
        };
        for shape in 0..6 {
            for case in 0..2000 {
                let mut a = [next(), next(), next(), next()];
                let mut b = [next(), next(), next(), next()];
                let mut c = [next(), next(), next(), next()];
                if shape & 1 != 0 { c[1] = 0; }
                if shape & 2 != 0 { c[2] = 0; }
                if shape == 4 { c[0] = 0; c[1] = 0; c[2] = 1; }
                if shape == 5 { c[0] = 1; c[1] = 0; c[2] = 0; }
                if case % 19 == 0 { a = [p - 1; 4]; }
                if case % 23 == 0 { b = [p - 1; 4]; }
                assert_eq!(quartic_mul(a, b, c, p), oracle(a, b, c),
                    "shape={shape} case={case} multiply");
                assert_eq!(quartic_square(a, c, p), oracle(a, a, c),
                    "shape={shape} case={case} square");
            }
        }
        for value in [
            0u128, 1, p as u128, 4 * (p as u128 - 1).pow(2),
            4 * (p as u128 - 1).pow(2) - 1,
        ] {
            assert_eq!(quartic_mersenne_reduce(value),
                (value % p as u128) as u64);
        }
    }

}

#[cfg(test)]
mod round110_unit_binomial_quotient {
    use super::*;

    #[test]
    fn unit_binomial_remainder_matches_independent_u128_long_division() {
        // The reference performs full polynomial long division with u128
        // arithmetic; it never calls the specialized reduction routine.
        for p in [7u64, 65_521, 1_000_000_007, MAX_PRIME] {
            for degree in [4usize, 5, 8] {
                for constant in [1, p - 1] {
                    let mut modulus = vec![0u64; degree + 1];
                    modulus[0] = constant;
                    modulus[degree] = 1;
                    let mut state = 0x726f_6f74_3131_3030u64 ^ p;
                    for iteration in 0..384usize {
                        let width = degree + 1 + (iteration % (degree * 2 + 1));
                        let mut product = vec![0u64; width];
                        for value in &mut product {
                            state ^= state << 7;
                            state ^= state >> 9;
                            state ^= state << 8;
                            *value = state % p;
                        }
                        if iteration % 11 == 0 {
                            for value in &mut product { *value = p - 1; }
                        } else if iteration % 13 == 0 {
                            product[width - 1] = 0;
                        }
                        let mut expected = product.clone();
                        for shift in (0..expected.len() - degree).rev() {
                            let high = expected[shift + degree];
                            let term = (high as u128 * constant as u128
                                        % p as u128) as u64;
                            expected[shift] = ((expected[shift] as u128
                                + p as u128 - term as u128) % p as u128) as u64;
                            expected[shift + degree] = 0;
                        }
                        expected.truncate(degree);
                        assert_eq!(
                            vpower_reduce(product, &modulus, p).unwrap(),
                            trim(expected),
                            "p={p} degree={degree} constant={constant} trial={iteration}"
                        );
                    }
                }
            }
        }
    }
}
