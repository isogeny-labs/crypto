//! Fixed-storage distinct Fp roots and independent streaming identity replay.
//! Boundary certificates do not certify a complete operation trajectory.
use super::super::fp_backend::Fp64Arithmetic;
use std::time::Instant;

pub const STORAGE_BYTES: u64 = 16 * 1024;
pub const MAX_WORK: u64 = 16_777_216;
pub const MAX_FIELD_WORK: u64 = 1_000_000;
pub const MAX_EVENTS: u64 = 100_000;
const COPY_WORK: u64 = 17 * 32;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RootBudget {
    pub work: u64,
    pub field_work: u64,
    pub bytes: u64,
    pub events: u64,
    pub split_attempts: u64,
    pub degree: usize,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stop {
    Complete,
    AllFieldElements,
    InvalidInput,
    WorkLimit,
    FieldLimit,
    StorageLimit,
    EventLimit,
    SplitLimit,
    Deadline,
    ObserverFailure,
    InvalidCertificate,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Metrics {
    pub work: u64,
    pub field_work: u64,
    pub bytes: u64,
    pub splits: u64,
    pub attempted_events: u64,
    pub acknowledged_events: u64,
    pub arithmetic_invoked: bool,
}
/// Canonical low-to-high coefficients. Unused entries are zero.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Poly17 {
    coefficients: [u64; 17],
    len: u8,
}
impl Poly17 {
    const ZERO: Self = Self {
        coefficients: [0; 17],
        len: 0,
    };
    fn constant(c: u64) -> Self {
        let mut result = Self::ZERO;
        if c != 0 {
            result.coefficients[0] = c;
            result.len = 1;
        }
        result
    }
    fn trim(&mut self) {
        while self.len > 0 && self.coefficients[self.len as usize - 1] == 0 {
            self.len -= 1;
        }
    }
    pub fn coefficients(&self) -> &[u64] {
        &self.coefficients[..self.len as usize]
    }
    /// Fixed-size decoding helper. The caller owns admission of encoded input;
    /// this performs at most seventeen comparisons/copies and no allocation.
    pub fn from_coefficients(field: &Fp64Arithmetic, values: &[u64]) -> Result<Self> {
        if values.len() > 17 || values.iter().any(|&c| c >= field.p()) {
            return Err(Stop::InvalidInput);
        }
        let mut out = Self::ZERO;
        out.len = values.len() as u8;
        out.coefficients[..values.len()].copy_from_slice(values);
        out.trim();
        Ok(out)
    }
    fn valid(&self, p: u64, max_len: usize) -> bool {
        let n = self.len as usize;
        n <= max_len
            && n <= 17
            && self.coefficients[..n].iter().all(|&c| c < p)
            && (n == 0 || self.coefficients[n - 1] != 0)
            && self.coefficients[n..].iter().all(|&c| c == 0)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventKind {
    Begin {
        p: u64,
        polynomial: Poly17,
        seed: u64,
    },
    LinearPart {
        xp: Poly17,
        polynomial: Poly17,
    },
    Split {
        polynomial: Poly17,
        delta: u64,
        factor: Poly17,
        quotient: Poly17,
        accepted: bool,
    },
    LinearRoot {
        polynomial: Poly17,
        root: u64,
    },
    Finish {
        roots: [u64; 8],
        count: u8,
        all_field_elements: bool,
    },
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BoundedRootEvent {
    pub sequence: u64,
    pub kind: EventKind,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BoundedRootReport {
    pub p: u64,
    pub polynomial: Poly17,
    pub seed: u64,
    pub limits: RootBudget,
    pub stop: Stop,
    pub metrics: Metrics,
    pub roots: [u64; 8],
    pub root_count: u8,
    pub unresolved_factors: u8,
    pub unacknowledged: Option<BoundedRootEvent>,
}
impl BoundedRootReport {
    pub fn root_set_complete(&self) -> bool {
        self.stop == Stop::Complete
    }
    pub fn all_field_elements(&self) -> bool {
        self.stop == Stop::AllFieldElements
    }
}
type Result<T> = std::result::Result<T, Stop>;
struct Gate {
    limits: RootBudget,
    deadline: Instant,
    metrics: Metrics,
}
impl Gate {
    fn new(limits: RootBudget, deadline: Instant) -> Self {
        Self {
            limits,
            deadline,
            metrics: Metrics::default(),
        }
    }
    fn clock(&self) -> Result<()> {
        if Instant::now() >= self.deadline {
            Err(Stop::Deadline)
        } else {
            Ok(())
        }
    }
    fn admit(&mut self) -> Result<()> {
        self.clock()?;
        if self.limits.degree > 8
            || self.limits.work > MAX_WORK
            || self.limits.field_work > MAX_FIELD_WORK
            || self.limits.events > MAX_EVENTS
            || self.limits.split_attempts > MAX_EVENTS
            || self.limits.bytes > 64 * 1024 * 1024
        {
            return Err(Stop::InvalidInput);
        }
        if self.limits.bytes < STORAGE_BYTES {
            return Err(Stop::StorageLimit);
        }
        self.charge(4096)?;
        self.metrics.bytes = STORAGE_BYTES;
        Ok(())
    }
    fn charge(&mut self, amount: u64) -> Result<()> {
        self.clock()?;
        let next = self
            .metrics
            .work
            .checked_add(amount)
            .ok_or(Stop::WorkLimit)?;
        if next > self.limits.work {
            return Err(Stop::WorkLimit);
        }
        self.metrics.work = next;
        Ok(())
    }
    fn scalar(&mut self) -> Result<()> {
        self.clock()?;
        if self.metrics.field_work >= self.limits.field_work {
            return Err(Stop::FieldLimit);
        }
        self.charge(32)?;
        self.metrics.field_work += 1;
        self.metrics.arithmetic_invoked = true;
        Ok(())
    }
}
struct Producer<'a> {
    p: u64,
    gate: Gate,
    observer: &'a mut dyn FnMut(&BoundedRootEvent) -> std::result::Result<(), ()>,
    unacknowledged: Option<BoundedRootEvent>,
    pending: [Poly17; 8],
    pending_len: usize,
    roots: [u64; 8],
    root_count: usize,
}
impl Producer<'_> {
    fn prepare_event(&mut self) -> Result<()> {
        if self.gate.metrics.attempted_events >= self.gate.limits.events {
            return Err(Stop::EventLimit);
        }
        self.gate.charge(256 + 4 * COPY_WORK)?;
        Ok(())
    }
    fn emit(&mut self, kind: EventKind) -> Result<()> {
        self.gate.clock()?;
        let event = BoundedRootEvent {
            sequence: self.gate.metrics.acknowledged_events,
            kind,
        };
        self.gate.metrics.attempted_events += 1;
        if (self.observer)(&event).is_err() {
            self.unacknowledged = Some(event);
            return Err(Stop::ObserverFailure);
        }
        self.gate.metrics.acknowledged_events += 1;
        self.gate.clock()
    }
    fn add(&mut self, a: u64, b: u64) -> Result<u64> {
        self.gate.scalar()?;
        Ok(((a as u128 + b as u128) % self.p as u128) as u64)
    }
    fn sub(&mut self, a: u64, b: u64) -> Result<u64> {
        self.gate.scalar()?;
        Ok(((a as u128 + self.p as u128 - b as u128) % self.p as u128) as u64)
    }
    fn mul(&mut self, a: u64, b: u64) -> Result<u64> {
        self.gate.scalar()?;
        Ok((a as u128 * b as u128 % self.p as u128) as u64)
    }
    fn inverse(&mut self, value: u64) -> Result<u64> {
        if value == 0 {
            return Err(Stop::InvalidInput);
        }
        self.gate.charge(32)?;
        if value == 1 {
            return Ok(1);
        }
        let (mut base, mut result, mut exponent) = (value, 1, self.p - 2);
        while exponent != 0 {
            self.gate.charge(32)?;
            if exponent & 1 != 0 {
                result = self.mul(result, base)?;
            }
            exponent >>= 1;
            if exponent != 0 {
                base = self.mul(base, base)?;
            }
        }
        Ok(result)
    }
    fn product(&mut self, a: &Poly17, b: &Poly17) -> Result<Poly17> {
        self.gate.charge(3 * COPY_WORK)?;
        let mut out = Poly17::ZERO;
        if a.len == 0 || b.len == 0 {
            return Ok(out);
        }
        let n = a.len as usize + b.len as usize - 1;
        if n > 17 {
            return Err(Stop::InvalidInput);
        }
        out.len = n as u8;
        for i in 0..a.len as usize {
            for j in 0..b.len as usize {
                let value = self.mul(a.coefficients[i], b.coefficients[j])?;
                out.coefficients[i + j] = self.add(out.coefficients[i + j], value)?;
            }
        }
        out.trim();
        Ok(out)
    }
    fn division(&mut self, a: &Poly17, b: &Poly17) -> Result<(Poly17, Poly17)> {
        self.gate.charge(5 * COPY_WORK)?;
        if b.len == 0 {
            return Err(Stop::InvalidInput);
        }
        let mut remainder = *a;
        let mut quotient = Poly17::ZERO;
        if a.len < b.len {
            return Ok((quotient, remainder));
        }
        quotient.len = a.len - b.len + 1;
        let inv = self.inverse(b.coefficients[b.len as usize - 1])?;
        while remainder.len >= b.len {
            self.gate.charge(COPY_WORK)?;
            let shift = (remainder.len - b.len) as usize;
            let coefficient = self.mul(remainder.coefficients[remainder.len as usize - 1], inv)?;
            quotient.coefficients[shift] = coefficient;
            for j in 0..b.len as usize {
                let v = self.mul(coefficient, b.coefficients[j])?;
                remainder.coefficients[shift + j] =
                    self.sub(remainder.coefficients[shift + j], v)?;
            }
            remainder.trim();
        }
        quotient.trim();
        Ok((quotient, remainder))
    }
    fn power_modulus(&mut self, modulus: &Poly17) -> Result<Poly17> {
        self.gate.charge(2 * COPY_WORK)?;
        if modulus.len == 0 {
            return Err(Stop::InvalidInput);
        }
        let mut monic = *modulus;
        let leading = monic.coefficients[monic.len as usize - 1];
        if leading != 1 {
            let inverse = self.inverse(leading)?;
            for coefficient in &mut monic.coefficients[..monic.len as usize] {
                *coefficient = self.mul(*coefficient, inverse)?;
            }
        }
        Ok(monic)
    }
    // Powering requires only a remainder; no quotient or inverse per reduction.
    fn reduce_monic(&mut self, value: &mut Poly17, modulus: &Poly17) -> Result<()> {
        self.gate.charge(COPY_WORK)?;
        if modulus.len == 0 || modulus.coefficients[modulus.len as usize - 1] != 1 {
            return Err(Stop::InvalidInput);
        }
        while value.len >= modulus.len {
            self.gate.charge(COPY_WORK)?;
            let high = value.len as usize - 1;
            let coefficient = value.coefficients[high];
            let shift = (value.len - modulus.len) as usize;
            for j in 0..modulus.len as usize - 1 {
                let product = self.mul(coefficient, modulus.coefficients[j])?;
                value.coefficients[shift + j] = self.sub(value.coefficients[shift + j], product)?;
            }
            value.coefficients[high] = 0;
            value.trim();
        }
        Ok(())
    }
    fn power(&mut self, base: &Poly17, mut exponent: u64, modulus: &Poly17) -> Result<Poly17> {
        self.gate.charge(6 * COPY_WORK)?;
        let monic = self.power_modulus(modulus)?;
        let mut b = *base;
        self.reduce_monic(&mut b, &monic)?;
        let mut acc = Poly17::constant(1);
        self.reduce_monic(&mut acc, &monic)?;
        while exponent != 0 {
            self.gate.charge(4 * COPY_WORK)?;
            if exponent & 1 != 0 {
                let mut product = self.product(&acc, &b)?;
                self.reduce_monic(&mut product, &monic)?;
                acc = product;
            }
            exponent >>= 1;
            if exponent != 0 {
                let mut product = self.product(&b, &b)?;
                self.reduce_monic(&mut product, &monic)?;
                b = product;
            }
        }
        Ok(acc)
    }
    fn gcd(&mut self, mut a: Poly17, mut b: Poly17) -> Result<Poly17> {
        self.gate.charge(5 * COPY_WORK)?;
        while b.len != 0 {
            self.gate.charge(3 * COPY_WORK)?;
            let r = self.division(&a, &b)?.1;
            a = b;
            b = r;
        }
        if a.len != 0 {
            let inv = self.inverse(a.coefficients[a.len as usize - 1])?;
            for i in 0..a.len as usize {
                a.coefficients[i] = self.mul(a.coefficients[i], inv)?;
            }
        }
        Ok(a)
    }
    fn execute(&mut self, f: Poly17, seed: u64) -> Result<Stop> {
        self.prepare_event()?;
        self.emit(EventKind::Begin {
            p: self.p,
            polynomial: f,
            seed,
        })?;
        if f.len == 0 {
            self.prepare_event()?;
            self.emit(EventKind::Finish {
                roots: [0; 8],
                count: 0,
                all_field_elements: true,
            })?;
            return Ok(Stop::AllFieldElements);
        }
        let mut x = Poly17::ZERO;
        x.len = 2;
        x.coefficients[1] = 1;
        let xp = self.power(&x, self.p, &f)?;
        self.gate.charge(3 * COPY_WORK)?;
        let mut difference = xp;
        difference.len = difference.len.max(2);
        difference.coefficients[1] = self.sub(difference.coefficients[1], 1)?;
        difference.trim();
        let g = self.gcd(f, difference)?;
        self.prepare_event()?;
        self.emit(EventKind::LinearPart { xp, polynomial: g })?;
        self.pending[0] = g;
        self.pending_len = 1;
        let mut state = seed ^ 0x524f_4f54;
        while self.pending_len > 0 {
            self.gate.charge(4 * COPY_WORK)?;
            // Keep current factor on the stack until its event is acknowledged.
            let h = self.pending[self.pending_len - 1];
            if h.len <= 1 {
                self.pending_len -= 1;
                continue;
            }
            if h.len == 2 {
                let inv = self.inverse(h.coefficients[1])?;
                let value = self.mul(h.coefficients[0], inv)?;
                let root = self.sub(0, value)?;
                self.prepare_event()?;
                self.emit(EventKind::LinearRoot {
                    polynomial: h,
                    root,
                })?;
                if self.root_count >= 8 {
                    return Err(Stop::InvalidInput);
                }
                self.roots[self.root_count] = root;
                self.root_count += 1;
                self.pending_len -= 1;
                continue;
            }
            if self.gate.metrics.splits >= self.gate.limits.split_attempts {
                return Err(Stop::SplitLimit);
            }
            self.gate.charge(256)?;
            self.gate.metrics.splits += 1;
            state = state.wrapping_add(0x9e3779b97f4a7c15);
            let mut z = state;
            z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
            let delta = (z ^ (z >> 31)) % self.p;
            let mut base = x;
            base.coefficients[0] = delta;
            let mut s = self.power(&base, (self.p - 1) / 2, &h)?;
            s.len = s.len.max(1);
            s.coefficients[0] = self.sub(s.coefficients[0], 1)?;
            s.trim();
            let d = self.gcd(h, s)?;
            let accepted = d.len > 1 && d.len < h.len;
            let q = if accepted {
                self.division(&h, &d)?.0
            } else {
                Poly17::ZERO
            };
            self.prepare_event()?;
            self.emit(EventKind::Split {
                polynomial: h,
                delta,
                factor: d,
                quotient: q,
                accepted,
            })?;
            if accepted {
                if self.pending_len >= 8 {
                    return Err(Stop::InvalidInput);
                }
                self.pending[self.pending_len - 1] = d;
                self.pending[self.pending_len] = q;
                self.pending_len += 1;
            }
        }
        self.gate.charge(8 * COPY_WORK)?;
        self.roots[..self.root_count].sort_unstable();
        if self.roots[..self.root_count]
            .windows(2)
            .any(|w| w[0] == w[1])
        {
            return Err(Stop::InvalidInput);
        }
        self.prepare_event()?;
        self.emit(EventKind::Finish {
            roots: self.roots,
            count: self.root_count as u8,
            all_field_elements: false,
        })?;
        Ok(Stop::Complete)
    }
}

pub fn solve_bounded(
    field: &Fp64Arithmetic,
    coefficients: &[u64],
    seed: u64,
    limits: RootBudget,
    original_deadline: Instant,
    observer: &mut dyn FnMut(&BoundedRootEvent) -> std::result::Result<(), ()>,
) -> BoundedRootReport {
    let mut engine = Producer {
        p: field.p(),
        gate: Gate::new(limits, original_deadline),
        observer,
        unacknowledged: None,
        pending: [Poly17::ZERO; 8],
        pending_len: 0,
        roots: [0; 8],
        root_count: 0,
    };
    let mut polynomial = Poly17::ZERO;
    let stop = (|| {
        engine.gate.admit()?;
        if coefficients.len() > limits.degree + 1 || coefficients.iter().any(|&c| c >= field.p()) {
            return Err(Stop::InvalidInput);
        }
        engine.gate.charge(4 * COPY_WORK)?;
        polynomial.len = coefficients.len() as u8;
        polynomial.coefficients[..coefficients.len()].copy_from_slice(coefficients);
        polynomial.trim();
        engine.execute(polynomial, seed)
    })()
    .unwrap_or_else(|stop| stop);
    let complete = stop == Stop::Complete;
    BoundedRootReport {
        p: field.p(),
        polynomial,
        seed,
        limits,
        stop,
        metrics: engine.gate.metrics,
        roots: if complete { engine.roots } else { [0; 8] },
        root_count: if complete { engine.root_count as u8 } else { 0 },
        unresolved_factors: engine.pending_len as u8,
        unacknowledged: engine.unacknowledged,
    }
}

/// Independent identity checker. No producer algebra routine is called here.
/// Caller retains the stream; this object retains only bounded factor state.
pub struct Replay {
    p: u64,
    input: Poly17,
    seed: u64,
    producer_limits: RootBudget,
    gate: Gate,
    sequence: u64,
    begun: bool,
    linear_seen: bool,
    finished: bool,
    pending: [Poly17; 8],
    pending_len: usize,
    g: Poly17,
    roots: [u64; 8],
    root_count: usize,
    splits: u64,
    rng: u64,
    failure: Option<Stop>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CheckedRootSummary {
    pub finite_complete: bool,
    pub all_field_elements: bool,
    pub checked_roots: [u64; 8],
    pub checked_root_count: u8,
    pub acknowledged_prefix_events: u64,
    pub replay_metrics: Metrics,
    pub producer_operation_trajectory_verified: bool,
}
impl Replay {
    pub fn new(
        field: &Fp64Arithmetic,
        coefficients: &[u64],
        seed: u64,
        producer_limits: RootBudget,
        replay_limits: RootBudget,
        deadline: Instant,
    ) -> Result<Self> {
        let mut gate = Gate::new(replay_limits, deadline);
        gate.admit()?;
        if producer_limits.degree > 8
            || producer_limits.work > MAX_WORK
            || producer_limits.field_work > MAX_FIELD_WORK
            || producer_limits.events > MAX_EVENTS
            || producer_limits.split_attempts > MAX_EVENTS
            || producer_limits.bytes > 64 * 1024 * 1024
            || coefficients.len() > producer_limits.degree + 1
            || coefficients.len() > replay_limits.degree + 1
            || coefficients.iter().any(|&v| v >= field.p())
        {
            return Err(Stop::InvalidInput);
        }
        gate.charge(4 * COPY_WORK)?;
        let mut input = Poly17::ZERO;
        input.len = coefficients.len() as u8;
        input.coefficients[..coefficients.len()].copy_from_slice(coefficients);
        input.trim();
        Ok(Self {
            p: field.p(),
            input,
            seed,
            producer_limits,
            gate,
            sequence: 0,
            begun: false,
            linear_seen: false,
            finished: false,
            pending: [Poly17::ZERO; 8],
            pending_len: 0,
            g: Poly17::ZERO,
            roots: [0; 8],
            root_count: 0,
            splits: 0,
            rng: seed ^ 0x524f_4f54,
            failure: None,
        })
    }
    fn times(&mut self, x: u64, y: u64) -> Result<u64> {
        self.gate.scalar()?;
        Ok((u128::from(x) * u128::from(y) % u128::from(self.p)) as u64)
    }
    fn plus(&mut self, x: u64, y: u64) -> Result<u64> {
        self.gate.scalar()?;
        Ok((x + y) % self.p)
    }
    fn minus(&mut self, x: u64, y: u64) -> Result<u64> {
        self.gate.scalar()?;
        Ok((x + self.p - y) % self.p)
    }
    // Left-to-right scalar exponentiation, separate from producer inverse.
    fn reciprocal(&mut self, x: u64) -> Result<u64> {
        if x == 0 {
            return Err(Stop::InvalidCertificate);
        }
        self.gate.charge(32)?;
        if x == 1 {
            return Ok(1);
        }
        let exponent = self.p - 2;
        let mut acc = 1;
        for bit in (0..64 - exponent.leading_zeros()).rev() {
            self.gate.charge(32)?;
            acc = self.times(acc, acc)?;
            if exponent & (1u64 << bit) != 0 {
                acc = self.times(acc, x)?;
            }
        }
        Ok(acc)
    }
    fn convolution(&mut self, x: &Poly17, y: &Poly17) -> Result<Poly17> {
        self.gate.charge(4 * COPY_WORK)?;
        let mut z = Poly17::ZERO;
        if x.len == 0 || y.len == 0 {
            return Ok(z);
        }
        let n = usize::from(x.len) + usize::from(y.len) - 1;
        if n > 17 {
            return Err(Stop::InvalidCertificate);
        }
        z.len = n as u8;
        // Output-diagonal order differs from producer's input-row order.
        for k in 0..n {
            self.gate.charge(32)?;
            let start = (k + 1).saturating_sub(usize::from(y.len));
            let end = usize::from(x.len).min(k + 1);
            for i in start..end {
                self.gate.charge(32)?;
                let v = self.times(x.coefficients[i], y.coefficients[k - i])?;
                z.coefficients[k] = self.plus(z.coefficients[k], v)?;
            }
        }
        z.trim();
        Ok(z)
    }
    fn remainder(&mut self, numerator: Poly17, denominator: &Poly17) -> Result<Poly17> {
        self.gate.charge(4 * COPY_WORK)?;
        if denominator.len == 0 {
            return Err(Stop::InvalidCertificate);
        }
        let mut r = numerator;
        if r.len < denominator.len {
            return Ok(r);
        }
        let inverse =
            self.reciprocal(denominator.coefficients[usize::from(denominator.len) - 1])?;
        for high in (usize::from(denominator.len) - 1..usize::from(numerator.len)).rev() {
            self.gate.charge(COPY_WORK)?;
            let c = self.times(r.coefficients[high], inverse)?;
            let shift = high + 1 - usize::from(denominator.len);
            for j in 0..usize::from(denominator.len) {
                let v = self.times(c, denominator.coefficients[j])?;
                r.coefficients[shift + j] = self.minus(r.coefficients[shift + j], v)?;
            }
        }
        r.trim();
        Ok(r)
    }
    fn normalized_modulus(&mut self, modulus: &Poly17) -> Result<Poly17> {
        self.gate.charge(2 * COPY_WORK)?;
        if modulus.len == 0 {
            return Err(Stop::InvalidCertificate);
        }
        let mut result = *modulus;
        let lead = result.coefficients[usize::from(result.len) - 1];
        if lead != 1 {
            let reciprocal = self.reciprocal(lead)?;
            for index in 0..usize::from(result.len) {
                result.coefficients[index] = self.times(result.coefficients[index], reciprocal)?;
            }
        }
        Ok(result)
    }
    fn cancel_monic(&mut self, numerator: &mut Poly17, denominator: &Poly17) -> Result<()> {
        self.gate.charge(COPY_WORK)?;
        let divisor_len = usize::from(denominator.len);
        if divisor_len == 0 || denominator.coefficients[divisor_len - 1] != 1 {
            return Err(Stop::InvalidCertificate);
        }
        // Independent descending-index traversal rather than producer trim loop.
        for high in (divisor_len - 1..usize::from(numerator.len)).rev() {
            self.gate.charge(COPY_WORK)?;
            let factor = numerator.coefficients[high];
            let offset = high + 1 - divisor_len;
            for low in 0..divisor_len - 1 {
                let term = self.times(factor, denominator.coefficients[low])?;
                numerator.coefficients[offset + low] =
                    self.minus(numerator.coefficients[offset + low], term)?;
            }
            numerator.coefficients[high] = 0;
        }
        numerator.trim();
        Ok(())
    }
    fn modular_power(&mut self, base: Poly17, exponent: u64, modulus: &Poly17) -> Result<Poly17> {
        self.gate.charge(6 * COPY_WORK)?;
        let monic = self.normalized_modulus(modulus)?;
        let mut base = base;
        self.cancel_monic(&mut base, &monic)?;
        let mut acc = Poly17::constant(1);
        self.cancel_monic(&mut acc, &monic)?;
        for bit in (0..64 - exponent.leading_zeros()).rev() {
            self.gate.charge(4 * COPY_WORK)?;
            let mut square = self.convolution(&acc, &acc)?;
            self.cancel_monic(&mut square, &monic)?;
            acc = square;
            if exponent & (1u64 << bit) != 0 {
                let mut product = self.convolution(&acc, &base)?;
                self.cancel_monic(&mut product, &monic)?;
                acc = product;
            }
        }
        Ok(acc)
    }
    fn monic_gcd(&mut self, left: Poly17, right: Poly17) -> Result<Poly17> {
        self.gate.charge(5 * COPY_WORK)?;
        let mut pair = [left, right];
        while pair[1].len != 0 {
            self.gate.charge(4 * COPY_WORK)?;
            let next = self.remainder(pair[0], &pair[1])?;
            pair = [pair[1], next];
        }
        let mut out = pair[0];
        if out.len > 0 {
            let inv = self.reciprocal(out.coefficients[usize::from(out.len) - 1])?;
            for i in 0..usize::from(out.len) {
                out.coefficients[i] = self.times(out.coefficients[i], inv)?;
            }
        }
        Ok(out)
    }
    fn top(&mut self) -> Result<Poly17> {
        self.gate.charge(8 * COPY_WORK)?;
        while self.pending_len > 0 && self.pending[self.pending_len - 1].len <= 1 {
            self.pending_len -= 1;
        }
        if self.pending_len == 0 {
            return Err(Stop::InvalidCertificate);
        }
        Ok(self.pending[self.pending_len - 1])
    }
    fn check_event(&mut self, event: &BoundedRootEvent) -> Result<()> {
        self.gate.clock()?;
        if self.finished || event.sequence != self.sequence {
            return Err(Stop::InvalidCertificate);
        }
        if self.sequence >= self.gate.limits.events || self.sequence >= self.producer_limits.events
        {
            return Err(Stop::EventLimit);
        }
        self.gate.charge(256 + 8 * COPY_WORK)?;
        match event.kind {
            EventKind::Begin {
                p,
                polynomial,
                seed,
            } => {
                if self.begun || p != self.p || seed != self.seed || polynomial != self.input {
                    return Err(Stop::InvalidCertificate);
                }
                self.begun = true;
            }
            EventKind::LinearPart { xp, polynomial } => {
                if !self.begun
                    || self.linear_seen
                    || self.input.len == 0
                    || !xp.valid(self.p, 9)
                    || !polynomial.valid(self.p, 9)
                {
                    return Err(Stop::InvalidCertificate);
                }
                let mut x = Poly17::ZERO;
                x.len = 2;
                x.coefficients[1] = 1;
                let input = self.input;
                let expected = self.modular_power(x, self.p, &input)?;
                if xp != expected {
                    return Err(Stop::InvalidCertificate);
                }
                let mut xm = expected;
                xm.len = xm.len.max(2);
                xm.coefficients[1] = self.minus(xm.coefficients[1], 1)?;
                xm.trim();
                let g = self.monic_gcd(input, xm)?;
                if g != polynomial {
                    return Err(Stop::InvalidCertificate);
                }
                self.g = g;
                self.pending[0] = g;
                self.pending_len = 1;
                self.linear_seen = true;
            }
            EventKind::Split {
                polynomial,
                delta,
                factor,
                quotient,
                accepted,
            } => {
                if !self.linear_seen
                    || !polynomial.valid(self.p, 9)
                    || !factor.valid(self.p, 9)
                    || !quotient.valid(self.p, 9)
                    || delta >= self.p
                    || polynomial != self.top()?
                    || polynomial.len <= 2
                {
                    return Err(Stop::InvalidCertificate);
                }
                if self.splits >= self.gate.limits.split_attempts
                    || self.splits >= self.producer_limits.split_attempts
                {
                    return Err(Stop::SplitLimit);
                }
                self.gate.charge(256)?;
                self.rng = self.rng.wrapping_add(0x9e3779b97f4a7c15);
                let mut word = self.rng;
                word = (word ^ (word >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
                word = (word ^ (word >> 27)).wrapping_mul(0x94d049bb133111eb);
                if (word ^ (word >> 31)) % self.p != delta {
                    return Err(Stop::InvalidCertificate);
                }
                let mut base = Poly17::ZERO;
                base.len = 2;
                base.coefficients[0] = delta;
                base.coefficients[1] = 1;
                let mut value = self.modular_power(base, (self.p - 1) / 2, &polynomial)?;
                value.len = value.len.max(1);
                value.coefficients[0] = self.minus(value.coefficients[0], 1)?;
                value.trim();
                let expected = self.monic_gcd(polynomial, value)?;
                let proper = expected.len > 1 && expected.len < polynomial.len;
                if expected != factor || proper != accepted {
                    return Err(Stop::InvalidCertificate);
                }
                if proper {
                    if quotient.len == 0
                        || self.convolution(&factor, &quotient)? != polynomial
                        || self.pending_len >= 8
                    {
                        return Err(Stop::InvalidCertificate);
                    }
                    self.pending[self.pending_len - 1] = factor;
                    self.pending[self.pending_len] = quotient;
                    self.pending_len += 1;
                } else if quotient != Poly17::ZERO {
                    return Err(Stop::InvalidCertificate);
                }
                self.splits += 1;
                self.gate.metrics.splits += 1;
            }
            EventKind::LinearRoot { polynomial, root } => {
                if !self.linear_seen
                    || !polynomial.valid(self.p, 2)
                    || polynomial.len != 2
                    || root >= self.p
                    || polynomial != self.top()?
                    || self.root_count >= 8
                {
                    return Err(Stop::InvalidCertificate);
                }
                let v = self.times(polynomial.coefficients[1], root)?;
                if self.plus(v, polynomial.coefficients[0])? != 0 {
                    return Err(Stop::InvalidCertificate);
                }
                self.roots[self.root_count] = root;
                self.root_count += 1;
                self.pending_len -= 1;
            }
            EventKind::Finish {
                roots,
                count,
                all_field_elements,
            } => {
                if !self.begun
                    || usize::from(count) > 8
                    || roots[usize::from(count)..].iter().any(|&r| r != 0)
                {
                    return Err(Stop::InvalidCertificate);
                }
                if all_field_elements {
                    if self.input.len != 0 || self.linear_seen || count != 0 {
                        return Err(Stop::InvalidCertificate);
                    }
                } else {
                    if !self.linear_seen
                        || self.input.len == 0
                        || usize::from(count) != self.root_count
                    {
                        return Err(Stop::InvalidCertificate);
                    }
                    self.gate.charge(8 * COPY_WORK)?;
                    while self.pending_len > 0 && self.pending[self.pending_len - 1].len <= 1 {
                        self.pending_len -= 1;
                    }
                    if self.pending_len != 0 {
                        return Err(Stop::InvalidCertificate);
                    }
                    self.roots[..self.root_count].sort_unstable();
                    if roots != self.roots
                        || roots[..self.root_count].iter().any(|&r| r >= self.p)
                        || roots[..self.root_count].windows(2).any(|w| w[0] >= w[1])
                    {
                        return Err(Stop::InvalidCertificate);
                    }
                    let mut product = Poly17::constant(1);
                    for &r in &roots[..self.root_count] {
                        let mut factor = Poly17::ZERO;
                        factor.len = 2;
                        factor.coefficients[0] = if r == 0 { 0 } else { self.p - r };
                        factor.coefficients[1] = 1;
                        product = self.convolution(&product, &factor)?;
                    }
                    if product != self.g {
                        return Err(Stop::InvalidCertificate);
                    }
                }
                self.finished = true;
            }
        }
        self.sequence += 1;
        self.gate.metrics.attempted_events += 1;
        self.gate.metrics.acknowledged_events += 1;
        self.gate.clock()
    }
    pub fn push(&mut self, event: &BoundedRootEvent) -> Result<()> {
        if let Some(failure) = self.failure {
            return Err(failure);
        }
        match self.check_event(event) {
            Ok(()) => Ok(()),
            Err(stop) => {
                self.failure = Some(stop);
                Err(stop)
            }
        }
    }
    /// Check an acknowledged stream against its report. Unacknowledged event
    /// contents are explicitly NOT certified; they carry no root-set claim.
    pub fn finish(mut self, report: &BoundedRootReport) -> Result<CheckedRootSummary> {
        if let Some(stop) = self.failure {
            return Err(stop);
        }
        self.gate.charge(8 * COPY_WORK)?;
        let m = report.metrics;
        if !self.begun
            || report.p != self.p
            || report.polynomial != self.input
            || report.seed != self.seed
            || report.limits != self.producer_limits
            || m.acknowledged_events != self.sequence
            || m.attempted_events != self.sequence + u64::from(report.unacknowledged.is_some())
            || m.work > report.limits.work
            || m.field_work > report.limits.field_work
            || m.bytes != STORAGE_BYTES
            || m.bytes > report.limits.bytes
            || m.splits > report.limits.split_attempts
            || m.attempted_events > report.limits.events
            || m.splits < self.splits
            || m.splits > self.splits + 1
            || m.arithmetic_invoked != (m.field_work > 0)
        {
            return Err(Stop::InvalidCertificate);
        }
        if let Some(event) = report.unacknowledged {
            if report.stop != Stop::ObserverFailure || event.sequence != self.sequence {
                return Err(Stop::InvalidCertificate);
            }
        } else if report.stop == Stop::ObserverFailure {
            return Err(Stop::InvalidCertificate);
        }
        let complete = report.root_set_complete();
        let all = report.all_field_elements();
        if complete || all {
            if !self.finished
                || all != (self.input.len == 0)
                || report.unacknowledged.is_some()
                || report.roots != self.roots
                || usize::from(report.root_count) != self.root_count
                || report.unresolved_factors != 0
                || m.splits != self.splits
            {
                return Err(Stop::InvalidCertificate);
            }
        } else {
            if report.root_count != 0
                || report.roots != [0; 8]
                || report.unresolved_factors > 8
                || (self.finished && report.stop != Stop::Deadline)
                || matches!(
                    report.stop,
                    Stop::InvalidInput | Stop::StorageLimit | Stop::InvalidCertificate
                )
            {
                return Err(Stop::InvalidCertificate);
            }
        }
        self.gate.clock()?;
        Ok(CheckedRootSummary {
            finite_complete: complete,
            all_field_elements: all,
            checked_roots: self.roots,
            checked_root_count: self.root_count as u8,
            acknowledged_prefix_events: self.sequence,
            replay_metrics: self.gate.metrics,
            producer_operation_trajectory_verified: false,
        })
    }
}

// Native code has no heap allocations. Bound all explicitly retained state plus
// 64 polynomial equivalents for the maximum nonrecursive arithmetic call stack,
// event construction/copies and scalar temporaries. Refusal ABI metadata is not
// an allocation or a claim about measured stack usage/RSS.
const _: () = assert!(
    64 * std::mem::size_of::<Poly17>()
        + std::mem::size_of::<Producer<'static>>()
        + std::mem::size_of::<Replay>()
        + std::mem::size_of::<BoundedRootReport>()
        + 2 * std::mem::size_of::<BoundedRootEvent>()
        <= STORAGE_BYTES as usize
);

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    fn deadline() -> Instant {
        Instant::now() + Duration::from_secs(30)
    }
    fn budget() -> RootBudget {
        RootBudget {
            work: MAX_WORK,
            field_work: MAX_FIELD_WORK,
            bytes: STORAGE_BYTES,
            events: MAX_EVENTS,
            split_attempts: 1000,
            degree: 8,
        }
    }
    fn field(p: u64) -> Fp64Arithmetic {
        Fp64Arithmetic::try_new(p, 131072, 4096, deadline(), &mut |_| Ok(()))
            .unwrap()
            .field
    }
    fn run(f: &[u64], p: u64, limits: RootBudget) -> (BoundedRootReport, Vec<BoundedRootEvent>) {
        let field = field(p);
        let mut events = Vec::new();
        let report = solve_bounded(&field, f, 42, limits, deadline(), &mut |e| {
            events.push(*e);
            Ok(())
        });
        (report, events)
    }
    fn check(
        f: &[u64],
        report: &BoundedRootReport,
        events: &[BoundedRootEvent],
    ) -> Result<CheckedRootSummary> {
        let field = field(report.p);
        let mut replay = Replay::new(&field, f, 42, report.limits, budget(), deadline())?;
        for event in events {
            replay.push(event)?;
        }
        replay.finish(report)
    }
    fn polynomial(roots: &[u64], p: u64, leading: u64) -> Vec<u64> {
        // Test oracle has independent canonical arithmetic, no producer calls.
        let mut out = vec![leading];
        for &root in roots {
            let mut next = vec![0; out.len() + 1];
            for (i, &c) in out.iter().enumerate() {
                next[i] = ((u128::from(next[i]) + u128::from(c) * u128::from((p - root) % p))
                    % u128::from(p)) as u64;
                next[i + 1] = (next[i + 1] + c) % p;
            }
            out = next;
        }
        out
    }
    fn evaluate(f: &[u64], x: u64, p: u64) -> u64 {
        f.iter().rev().fold(0, |a, &c| {
            ((u128::from(a) * u128::from(x) + u128::from(c)) % u128::from(p)) as u64
        })
    }
    #[test]
    fn bounded_prime_roots_exhaustive_quadratics_and_degeneracies() {
        for a in 0..7 {
            for b in 0..7 {
                for c in 0..7 {
                    let f = [a, b, c];
                    let (report, events) = run(&f, 7, budget());
                    let checked = check(&f, &report, &events).unwrap();
                    if a == 0 && b == 0 && c == 0 {
                        assert!(checked.all_field_elements);
                        assert!(!checked.finite_complete);
                        assert!(!report.metrics.arithmetic_invoked);
                    } else {
                        let roots: Vec<_> = (0..7).filter(|&x| evaluate(&f, x, 7) == 0).collect();
                        assert_eq!(report.stop, Stop::Complete);
                        assert!(checked.finite_complete);
                        assert_eq!(&report.roots[..usize::from(report.root_count)], roots);
                    }
                }
            }
        }
        // Derivative-zero repeated polynomial X^3-1 in characteristic3.
        let f = [2, 0, 0, 1];
        let (report, events) = run(&f, 3, budget());
        assert_eq!(&report.roots[..usize::from(report.root_count)], &[1]);
        assert!(check(&f, &report, &events).unwrap().finite_complete);
        let (report, events) = run(&[0, 0, 0], 17, budget());
        assert!(
            check(&[0, 0, 0], &report, &events)
                .unwrap()
                .all_field_elements
        );
        let (report, events) = run(&[3], 17, budget());
        assert_eq!(report.root_count, 0);
        assert!(check(&[3], &report, &events).unwrap().finite_complete);
        // X^2+1 is irreducible over F7; no full factorization is claimed.
        let (report, events) = run(&[1, 0, 1], 7, budget());
        assert_eq!(report.root_count, 0);
        assert!(check(&[1, 0, 1], &report, &events).unwrap().finite_complete);
        // (X-1)(X^2+1): return the sole field root, not a claimed complete
        // irreducible factorization of the cubic.
        let f = [6, 1, 6, 1];
        let (report, events) = run(&f, 7, budget());
        assert_eq!(report.root_count, 1);
        assert_eq!(report.roots[0], 1);
        assert!(check(&f, &report, &events).unwrap().finite_complete);
    }
    #[test]
    fn bounded_prime_roots_monic_remainders_match_general_division() {
        for p in [7, 140737488355213, (1u64 << 61) - 1] {
            let field = field(p);
            for degree in [0usize, 1, 8] {
                for leading in [1, p - 2] {
                    let mut coefficients = [0u64; 9];
                    for (index, value) in coefficients[..degree].iter_mut().enumerate() {
                        *value = (index as u64 + 2) % p;
                    }
                    coefficients[degree] = leading;
                    let divisor =
                        Poly17::from_coefficients(&field, &coefficients[..degree + 1]).unwrap();
                    for numerator_len in [0usize, 1, 5, 17] {
                        let numerator: [u64; 17] = std::array::from_fn(|i| (3 * i as u64 + 1) % p);
                        let numerator =
                            Poly17::from_coefficients(&field, &numerator[..numerator_len]).unwrap();
                        let mut observer = |_: &BoundedRootEvent| Ok(());
                        let mut producer = Producer {
                            p,
                            gate: Gate::new(budget(), deadline()),
                            observer: &mut observer,
                            unacknowledged: None,
                            pending: [Poly17::ZERO; 8],
                            pending_len: 0,
                            roots: [0; 8],
                            root_count: 0,
                        };
                        producer.gate.admit().unwrap();
                        let expected = producer.division(&numerator, &divisor).unwrap().1;
                        let monic = producer.power_modulus(&divisor).unwrap();
                        let mut actual = numerator;
                        producer.reduce_monic(&mut actual, &monic).unwrap();
                        assert_eq!(
                            actual, expected,
                            "producer p={p} degree={degree} length={numerator_len}"
                        );
                        let mut replay =
                            Replay::new(&field, &[1, 1], 42, budget(), budget(), deadline())
                                .unwrap();
                        let reference = replay.remainder(numerator, &divisor).unwrap();
                        let normalized = replay.normalized_modulus(&divisor).unwrap();
                        let mut checked = numerator;
                        replay.cancel_monic(&mut checked, &normalized).unwrap();
                        assert_eq!(checked, reference);
                        assert_eq!(checked, expected);
                        assert!(matches!(
                            producer.power_modulus(&Poly17::ZERO),
                            Err(Stop::InvalidInput)
                        ));
                        assert!(matches!(
                            replay.normalized_modulus(&Poly17::ZERO),
                            Err(Stop::InvalidCertificate)
                        ));
                        if degree == 0 {
                            assert_eq!(
                                producer.power(&numerator, 0, &divisor).unwrap(),
                                Poly17::ZERO
                            );
                            assert_eq!(
                                replay.modular_power(numerator, 0, &divisor).unwrap(),
                                Poly17::ZERO
                            );
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn bounded_prime_roots_fixed_wide_nonmonic_repeated_and_degree_eight() {
        for p in [140737488355213, 9007199254740881, (1u64 << 61) - 1] {
            for roots in [
                &[0, 1][..],
                &[0, 1, 2][..],
                &[0, 1, 2, 3][..],
                &[0, 1, 2, 3, 4][..],
                &[0, 1, 2, 3, 4, 5][..],
                &[0, 1, 2, 3, 4, 5, 6][..],
                &[2, 2, 3, 5][..],
                &[0, 1, 2, 3, 4, 5, 6, 7][..],
            ] {
                let f = polynomial(roots, p, p - 2);
                let (report, events) = run(&f, p, budget());
                assert_eq!(
                    report.stop,
                    Stop::Complete,
                    "p={p} degree={} stop={:?} metrics={:?} last_event={:?}",
                    roots.len(),
                    report.stop,
                    report.metrics,
                    events.last()
                );
                let mut expected = roots.to_vec();
                expected.sort_unstable();
                expected.dedup();
                assert_eq!(&report.roots[..usize::from(report.root_count)], expected);
                assert!(report.metrics.arithmetic_invoked);
                assert!(report.metrics.splits > 0);
                let checked = check(&f, &report, &events).unwrap_or_else(|stop| {
                    panic!(
                        "replay p={p} degree={} stop={stop:?} producer_metrics={:?}",
                        roots.len(),
                        report.metrics
                    )
                });
                assert!(checked.finite_complete);
                let split_events = events
                    .iter()
                    .filter(|event| matches!(event.kind, EventKind::Split { .. }))
                    .count() as u64;
                assert_eq!(checked.replay_metrics.splits, split_events);
                assert!(checked.replay_metrics.splits > 0);
                assert!(
                    !check(&f, &report, &events)
                        .unwrap()
                        .producer_operation_trajectory_verified
                );
            }
        }
    }
    #[test]
    fn bounded_prime_roots_expiry_between_event_reservation_and_callback() {
        let mut calls = 0;
        let mut observer = |_: &BoundedRootEvent| {
            calls += 1;
            Ok(())
        };
        let mut producer = Producer {
            p: 7,
            gate: Gate::new(budget(), deadline()),
            observer: &mut observer,
            unacknowledged: None,
            pending: [Poly17::ZERO; 8],
            pending_len: 0,
            roots: [0; 8],
            root_count: 0,
        };
        producer.gate.admit().unwrap();
        producer.prepare_event().unwrap();
        producer.gate.deadline = Instant::now() - Duration::from_secs(1);
        assert_eq!(
            producer.emit(EventKind::Begin {
                p: 7,
                polynomial: Poly17::constant(1),
                seed: 42,
            }),
            Err(Stop::Deadline)
        );
        assert_eq!(producer.gate.metrics.attempted_events, 0);
        assert_eq!(producer.gate.metrics.acknowledged_events, 0);
        assert!(producer.unacknowledged.is_none());
        assert_eq!(calls, 0);
    }

    #[test]
    fn bounded_prime_roots_caller_preserves_cumulative_work_balance() {
        let f = [16, 1];
        let (baseline, _) = run(&f, 17, budget());
        assert_eq!(baseline.stop, Stop::Complete);
        let field = field(17);
        let original_deadline = deadline();
        let total_work = baseline.metrics.work + 4096;
        let mut remaining = budget();
        remaining.work = total_work;
        remaining.bytes = 2 * STORAGE_BYTES;
        let first = solve_bounded(
            &field,
            &f,
            42,
            remaining,
            original_deadline,
            &mut |_| Ok(()),
        );
        assert_eq!(first.stop, Stop::Complete);
        remaining.work -= first.metrics.work;
        remaining.field_work -= first.metrics.field_work;
        remaining.bytes -= first.metrics.bytes;
        remaining.events -= first.metrics.attempted_events;
        remaining.split_attempts -= first.metrics.splits;
        assert_eq!(remaining.work, 4096);
        let mut callbacks = 0;
        let second = solve_bounded(&field, &f, 42, remaining, original_deadline, &mut |_| {
            callbacks += 1;
            Ok(())
        });
        assert_eq!(second.stop, Stop::WorkLimit);
        assert!(!second.root_set_complete());
        assert_eq!(second.root_count, 0);
        assert_eq!(callbacks, 0);
        assert_eq!(first.metrics.work + second.metrics.work, total_work);
        assert_eq!(
            first.metrics.bytes + second.metrics.bytes,
            2 * STORAGE_BYTES
        );
    }

    #[test]
    fn bounded_prime_roots_split_refusal_and_exact_resource_boundaries() {
        let f = polynomial(&[1, 2, 3], 17, 3);
        let (full, events) = run(&f, 17, budget());
        assert_eq!(full.stop, Stop::Complete);
        assert!(events
            .iter()
            .any(|e| matches!(e.kind, EventKind::Split { accepted: true, .. })));
        let mut exact = budget();
        exact.work = full.metrics.work;
        exact.field_work = full.metrics.field_work;
        exact.events = full.metrics.attempted_events;
        exact.split_attempts = full.metrics.splits;
        let (same, _) = run(&f, 17, exact);
        assert_eq!(same.stop, Stop::Complete);
        for which in 0..5 {
            let mut limited = exact;
            let expected = match which {
                0 => {
                    limited.work -= 1;
                    Stop::WorkLimit
                }
                1 => {
                    limited.field_work -= 1;
                    Stop::FieldLimit
                }
                2 => {
                    limited.events -= 1;
                    Stop::EventLimit
                }
                3 => {
                    limited.bytes -= 1;
                    Stop::StorageLimit
                }
                _ => {
                    limited.split_attempts -= 1;
                    Stop::SplitLimit
                }
            };
            let (report, stream) = run(&f, 17, limited);
            assert_eq!(report.stop, expected);
            assert_eq!(report.root_count, 0);
            assert!(!report.root_set_complete());
            if expected == Stop::StorageLimit {
                assert!(stream.is_empty());
                assert!(!report.metrics.arithmetic_invoked);
            } else {
                assert!(!check(&f, &report, &stream).unwrap().finite_complete);
            }
            if expected == Stop::SplitLimit {
                assert!(report.unresolved_factors > 0);
            }
        }
        // Preserve the distinct zero-attempt refusal as well as the exact
        // one-less split boundary above. Three distinct roots need >=2 splits.
        assert!(full.metrics.splits >= 2);
        let mut zero_splits = budget();
        zero_splits.split_attempts = 0;
        let (refused, prefix) = run(&f, 17, zero_splits);
        assert_eq!(refused.stop, Stop::SplitLimit);
        assert_eq!(refused.metrics.splits, 0);
        assert!(!prefix
            .iter()
            .any(|e| matches!(e.kind, EventKind::Split { .. })));
        assert!(!check(&f, &refused, &prefix).unwrap().finite_complete);
        let mut forged = refused;
        forged.stop = Stop::Complete;
        assert!(check(&f, &forged, &prefix).is_err());

        let mut limited = budget();
        limited.work = 4096 + 4 * COPY_WORK;
        let (report, stream) = run(&f, 17, limited);
        assert_eq!(report.stop, Stop::WorkLimit);
        assert!(!report.metrics.arithmetic_invoked);
        assert!(stream.is_empty());
        // Second call must consume a remaining balance, not refresh the first.
        let mut remaining = budget();
        remaining.work = full.metrics.work - 1;
        assert_eq!(run(&f, 17, remaining).0.stop, Stop::WorkLimit);
    }
    #[test]
    fn bounded_prime_roots_fixed_rejected_split_is_observed_and_replayed() {
        let field = field(5);
        let coefficients = [1, 0, 1]; // (X-2)(X-3) over F5.
                                      // Analytically fixed, not searched: first SplitMix state/word is zero.
                                      // Thus delta=0 and X^((5-1)/2)-1 = X^2-1 = 3 mod X^2+1,
                                      // giving monic gcd=1 and a rejected split.
        let seed = 0u64.wrapping_sub(0x9e3779b97f4a7c15) ^ 0x524f_4f54;
        let mut limits = budget();
        limits.split_attempts = 1;
        let mut events = Vec::new();
        let report = solve_bounded(&field, &coefficients, seed, limits, deadline(), &mut |e| {
            events.push(*e);
            Ok(())
        });
        assert_eq!(report.stop, Stop::SplitLimit);
        assert_eq!(report.metrics.splits, 1);
        let splits: Vec<_> = events
            .iter()
            .filter_map(|e| match e.kind {
                EventKind::Split {
                    delta,
                    factor,
                    quotient,
                    accepted,
                    ..
                } => Some((delta, factor, quotient, accepted)),
                _ => None,
            })
            .collect();
        assert_eq!(splits, vec![(0, Poly17::constant(1), Poly17::ZERO, false)]);
        assert_eq!(report.root_count, 0);
        assert!(report.unresolved_factors > 0);
        assert!(!report.root_set_complete());
        let mut replay =
            Replay::new(&field, &coefficients, seed, limits, budget(), deadline()).unwrap();
        for event in &events {
            replay.push(event).unwrap();
        }
        assert!(!replay.finish(&report).unwrap().finite_complete);
    }

    #[test]
    fn bounded_prime_roots_invalid_admission_and_sticky_observer() {
        let field = field(17);
        for input in [&[17][..], &[0; 10][..]] {
            let mut calls = 0;
            let report = solve_bounded(&field, input, 42, budget(), deadline(), &mut |_| {
                calls += 1;
                Ok(())
            });
            assert_eq!(report.stop, Stop::InvalidInput);
            assert_eq!(calls, 0);
            assert!(!report.metrics.arithmetic_invoked);
        }
        let f = polynomial(&[1, 2, 3], 17, 1);
        let (_, full) = run(&f, 17, budget());
        for refusal in [0, 1, full.len() - 1] {
            let mut acknowledged = Vec::new();
            let mut calls = 0;
            let report = solve_bounded(&field, &f, 42, budget(), deadline(), &mut |event| {
                calls += 1;
                if event.sequence as usize == refusal {
                    Err(())
                } else {
                    acknowledged.push(*event);
                    Ok(())
                }
            });
            assert_eq!(report.stop, Stop::ObserverFailure);
            assert_eq!(calls, refusal + 1);
            assert_eq!(
                report.metrics.attempted_events,
                report.metrics.acknowledged_events + 1
            );
            assert_eq!(report.root_count, 0);
            assert!(report.unacknowledged.is_some());
            if refusal > 0 {
                assert!(!check(&f, &report, &acknowledged).unwrap().finite_complete);
            }
        }
    }
    #[test]
    fn bounded_prime_roots_acknowledged_deadline_at_begin_and_finish() {
        let field = field(17);
        let f = polynomial(&[1, 2], 17, 1);
        for at_finish in [false, true] {
            let expires = Instant::now() + Duration::from_secs(1);
            let mut reached = false;
            let mut events = Vec::new();
            let report = solve_bounded(&field, &f, 42, budget(), expires, &mut |event| {
                events.push(*event);
                if matches!(event.kind, EventKind::Finish { .. }) == at_finish
                    && (at_finish || matches!(event.kind, EventKind::Begin { .. }))
                {
                    reached = true;
                    std::thread::sleep(
                        expires.saturating_duration_since(Instant::now())
                            + Duration::from_millis(1),
                    );
                }
                Ok(())
            });
            assert!(reached);
            assert_eq!(report.stop, Stop::Deadline);
            assert_eq!(report.root_count, 0);
            assert_eq!(
                report.metrics.attempted_events,
                report.metrics.acknowledged_events
            );
            assert!(!check(&f, &report, &events).unwrap().finite_complete);
        }
        let mut calls = 0;
        let report = solve_bounded(&field, &f, 42, budget(), Instant::now(), &mut |_| {
            calls += 1;
            Ok(())
        });
        assert_eq!(report.stop, Stop::Deadline);
        assert_eq!(calls, 0);
        assert_eq!(report.metrics.bytes, 0);
    }
    #[test]
    fn bounded_prime_roots_independent_replay_tamper_and_budget_controls() {
        let f = polynomial(&[1, 2, 3], 17, 3);
        let (report, events) = run(&f, 17, budget());
        assert!(check(&f, &report, &events).unwrap().finite_complete);
        let mut bad = events.clone();
        let i = bad
            .iter()
            .position(|e| matches!(e.kind, EventKind::LinearPart { .. }))
            .unwrap();
        if let EventKind::LinearPart {
            ref mut polynomial, ..
        } = bad[i].kind
        {
            polynomial.coefficients[0] = (polynomial.coefficients[0] + 1) % 17;
        }
        assert!(check(&f, &report, &bad).is_err());
        let mut bad = events.clone();
        let i = bad
            .iter()
            .position(|e| matches!(e.kind, EventKind::Split { .. }))
            .unwrap();
        if let EventKind::Split { ref mut delta, .. } = bad[i].kind {
            *delta = (*delta + 1) % 17;
        }
        assert!(check(&f, &report, &bad).is_err());
        let mut bad = events.clone();
        bad.remove(1);
        assert!(check(&f, &report, &bad).is_err());
        let mut bad = events.clone();
        bad.insert(1, bad[1]);
        assert!(check(&f, &report, &bad).is_err());
        // Mutate a local root certificate directly, not only final roots.
        let mut bad = events.clone();
        let i = bad
            .iter()
            .position(|e| matches!(e.kind, EventKind::LinearRoot { .. }))
            .unwrap();
        if let EventKind::LinearRoot { ref mut root, .. } = bad[i].kind {
            *root = (*root + 1) % 17;
        }
        assert!(check(&f, &report, &bad).is_err());
        // Removing Finish must reject even after adjusting event counters;
        // Complete cannot be inferred from the preceding LinearRoot events.
        let mut missing_finish = events.clone();
        assert!(matches!(
            missing_finish.pop().unwrap().kind,
            EventKind::Finish { .. }
        ));
        let mut missing_finish_report = report;
        missing_finish_report.metrics.acknowledged_events -= 1;
        missing_finish_report.metrics.attempted_events -= 1;
        assert!(check(&f, &missing_finish_report, &missing_finish).is_err());
        let mut changed = report;
        changed.roots[0] = 4;
        assert!(check(&f, &changed, &events).is_err());
        let mut wrong = f.clone();
        wrong[0] = (wrong[0] + 1) % 17;
        assert!(check(&wrong, &report, &events).is_err());
        let field = field(17);
        let mut limits = budget();
        limits.work = 4096;
        assert!(matches!(
            Replay::new(&field, &f, 42, budget(), limits, deadline()),
            Err(Stop::WorkLimit)
        ));
        let mut replay = Replay::new(&field, &f, 42, budget(), budget(), deadline()).unwrap();
        let mut bad = events[0];
        bad.sequence = 1;
        assert_eq!(replay.push(&bad), Err(Stop::InvalidCertificate));
        assert_eq!(replay.push(&events[0]), Err(Stop::InvalidCertificate));
        // Canonical but forged trailing coefficient cannot bypass validation.
        let mut bad = events.clone();
        if let EventKind::LinearPart {
            ref mut polynomial, ..
        } = bad[1].kind
        {
            polynomial.coefficients[16] = 1;
        }
        assert!(check(&f, &report, &bad).is_err());
    }
}
