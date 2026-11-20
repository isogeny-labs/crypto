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
        // charge() already checks the deadline. On the normal in-budget
        // path a separate clock() here would read Instant twice per field
        // operation. Preserve Deadline precedence on FieldLimit itself.
        if self.metrics.field_work >= self.limits.field_work {
            self.clock()?;
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
    // All inputs are canonical field elements, and the setup restricts
    // p <= 2^61-1, so addition/subtraction fit in one native word.
    // Gate charging and deadline checks must remain exactly one per op.
    fn add(&mut self, a: u64, b: u64) -> Result<u64> {
        self.gate.scalar()?;
        let sum = a + b;
        Ok(if sum >= self.p { sum - self.p } else { sum })
    }
    fn sub(&mut self, a: u64, b: u64) -> Result<u64> {
        self.gate.scalar()?;
        Ok(if a >= b { a - b } else { a + self.p - b })
    }
    fn mul(&mut self, a: u64, b: u64) -> Result<u64> {
        self.gate.scalar()?;
        let p = self.p;
        Ok(if p == (1u64 << 61) - 1 {
            // Mersenne reduction: 2^61 = 1 (mod p), with two folds
            // sufficient for a product of canonical 61-bit operands.
            let product = a as u128 * b as u128;
            let folded = ((product as u64) & p) + (product >> 61) as u64;
            let folded = (folded & p) + (folded >> 61);
            if folded >= p { folded - p } else { folded }
        } else if p <= u32::MAX as u64 {
            // (p-1)^2 fits in u64, including p=2^32-5.
            (a * b) % p
        } else {
            ((a as u128 * b as u128) % p as u128) as u64
        })
    }
    // R53_PRODUCER_INVERSE_BEGIN
    // Public-input Euclidean inversion: quotient steps replace repeated
    // Fermat powers. Use i128 coefficients so q*t cannot overflow; for
    // 0 < value < p <= 2^61-1 their magnitudes remain below p.
    // Each quotient step is independently budgeted and deadline checked.
    fn inverse(&mut self, value: u64) -> Result<u64> {
        if value == 0 {
            return Err(Stop::InvalidInput);
        }
        self.gate.charge(32)?;
        if value == 1 {
            return Ok(1);
        }
        let (mut r0, mut r1) = (self.p, value);
        let (mut t0, mut t1) = (0i128, 1i128);
        while r1 != 0 {
            self.gate.scalar()?;
            let q = r0 / r1;
            let next_r = r0 - q * r1;
            let next_t = t0 - i128::from(q) * t1;
            r0 = r1;
            r1 = next_r;
            t0 = t1;
            t1 = next_t;
        }
        debug_assert_eq!(r0, 1);
        let result = if t0 < 0 { t0 + i128::from(self.p) } else { t0 };
        Ok(result as u64)
    }
    // R53_PRODUCER_INVERSE_END
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
        // R51_PRODUCER_PRODUCT_BEGIN
        // Multiplication by a scalar, x, or a monic linear polynomial can
        // bypass whole rows of zero additions and unit multiplications.
        // The preceding COPY_WORK charge still checks the deadline.
        if a.len == 1 || b.len == 1 {
            let (scalar, poly) = if a.len == 1 { (a, b) } else { (b, a) };
            let c = scalar.coefficients[0];
            if c == 1 {
                return Ok(*poly);
            }
            out.len = poly.len;
            // R62_PRODUCER_UNIT_SCALAR_BEGIN
            // Multiplication by -1 is field negation, not a wide product.
            if c == self.p - 1 {
                for i in 0..poly.len as usize {
                    out.coefficients[i] = self.sub(0, poly.coefficients[i])?;
                }
                out.trim();
                return Ok(out);
            }
            // R62_PRODUCER_UNIT_SCALAR_END
            for i in 0..poly.len as usize {
                out.coefficients[i] = self.mul(c, poly.coefficients[i])?;
            }
            out.trim();
            return Ok(out);
        }
        let linear = if a.len == 2 && a.coefficients[1] == 1 {
            Some((a, b))
        } else if b.len == 2 && b.coefficients[1] == 1 {
            Some((b, a))
        } else {
            None
        };
        if let Some((linear, poly)) = linear {
            let c = linear.coefficients[0];
            let size = poly.len as usize;
            if c == 0 {
                // x * f(x) is an exact coefficient shift.
                out.coefficients[1..=size]
                    .copy_from_slice(&poly.coefficients[..size]);
                return Ok(out);
            }
            // R62_PRODUCER_UNIT_LINEAR_BEGIN
            // (x+1)f and (x-1)f require only coefficient additions or
            // subtractions. Keep the ordinary generic linear path intact.
            if c == 1 || c == self.p - 1 {
                out.coefficients[0] = if c == 1 {
                    poly.coefficients[0]
                } else {
                    self.sub(0, poly.coefficients[0])?
                };
                for i in 1..size {
                    out.coefficients[i] = if c == 1 {
                        self.add(poly.coefficients[i - 1], poly.coefficients[i])?
                    } else {
                        self.sub(poly.coefficients[i - 1], poly.coefficients[i])?
                    };
                }
                out.coefficients[size] = poly.coefficients[size - 1];
                out.trim();
                return Ok(out);
            }
            // R62_PRODUCER_UNIT_LINEAR_END
            out.coefficients[0] = self.mul(c, poly.coefficients[0])?;
            for i in 1..size {
                let scaled = self.mul(c, poly.coefficients[i])?;
                out.coefficients[i] = self.add(poly.coefficients[i - 1], scaled)?;
            }
            out.coefficients[size] = poly.coefficients[size - 1];
            out.trim();
            return Ok(out);
        }
        // R51_PRODUCER_PRODUCT_END
        // A zero constant term is a cheap indicator of structured/sparse
        // factors. Preserve the original dense loop on ordinary inputs.
        // R61_PRODUCT_LINEAR_ZERO_BEGIN
        // A missing linear coefficient is a common parity/sparse signature
        // even when the constant coefficient is nonzero. Keep the original
        // dense kernel for ordinary dense inputs.
        if a.coefficients[0] == 0 || b.coefficients[0] == 0
            || (a.len > 2 && a.coefficients[1] == 0)
            || (b.len > 2 && b.coefficients[1] == 0) {
        // R61_PRODUCT_LINEAR_ZERO_END
            for i in 0..a.len as usize {
                let ai = a.coefficients[i];
                if ai == 0 { continue; }
                for j in 0..b.len as usize {
                    let bj = b.coefficients[j];
                    if bj == 0 { continue; }
                    let value = self.mul(ai, bj)?;
                    out.coefficients[i + j] = self.add(out.coefficients[i + j], value)?;
                }
            }
        } else {
            for i in 0..a.len as usize {
                for j in 0..b.len as usize {
                    let value = self.mul(a.coefficients[i], b.coefficients[j])?;
                    // R54_PRODUCER_DENSE_PRODUCT_BEGIN
                    // The first row and final column introduce each output
                    // coefficient exactly once. Assign rather than add zero.
                    if i == 0 || j + 1 == b.len as usize {
                        out.coefficients[i + j] = value;
                    } else {
                        out.coefficients[i + j] = self.add(out.coefficients[i + j], value)?;
                    }
                    // R54_PRODUCER_DENSE_PRODUCT_END
                }
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
        // R53_CONSTANT_DIVISION_BEGIN
        // A nonzero constant divides the input exactly. Aggregate the
        // fixed per-coefficient copy-work budget once, while the scalar
        // multiplication still checks each field deadline independently.
        if b.len == 1 {
            self.gate.charge(COPY_WORK * u64::from(a.len))?;
            if b.coefficients[0] == 1 {
                return Ok((*a, Poly17::ZERO));
            }
            let inv = self.inverse(b.coefficients[0])?;
            for i in 0..a.len as usize {
                quotient.coefficients[i] = self.mul(a.coefficients[i], inv)?;
            }
            quotient.trim();
            return Ok((quotient, Poly17::ZERO));
        }
        // R53_CONSTANT_DIVISION_END
        // R52_DIVISION_BEGIN
        // Synthetic division by x is an exact coefficient shift.
        // No finite-field operations or repeated trimming are needed.
        if b.len == 2 && b.coefficients[0] == 0 && b.coefficients[1] == 1 {
            let n = a.len as usize;
            quotient.coefficients[..n - 1].copy_from_slice(&a.coefficients[1..n]);
            return Ok((quotient, Poly17::constant(a.coefficients[0])));
        }
        // R52_DIVISION_END
        // R56_PRODUCER_MONOMIAL_DIVISION_BEGIN
        // Division by x^d is a coefficient split. Preserve the original
        // quotient/remainder and account for each nonzero high cancellation
        // in one bounded deadline/work check, without any field operations.
        if b.len >= 3
            && b.coefficients[..b.len as usize - 1].iter().all(|&c| c == 0)
            && b.coefficients[b.len as usize - 1] == 1
        {
            let n = a.len as usize;
            let d = b.len as usize - 1;
            let cancellations = a.coefficients[d..n].iter().filter(|&&c| c != 0).count();
            self.gate.charge(COPY_WORK * cancellations as u64)?;
            quotient.coefficients[..n - d].copy_from_slice(&a.coefficients[d..n]);
            remainder.coefficients[d..n].fill(0);
            remainder.len = d as u8;
            quotient.trim();
            remainder.trim();
            return Ok((quotient, remainder));
        }
        // R56_PRODUCER_MONOMIAL_DIVISION_END
        // R59_BINOMIAL_DIVISION_BEGIN
        // Synthetic division by x^d+c: x^d == -c, so cancel each
        // descending high coefficient with one field update. For c=+/-1
        // this is just subtraction/addition, not multiplication.
        if b.len >= 2 && b.coefficients[b.len as usize - 1] == 1
            && (b.coefficients[0] == 1 || b.coefficients[0] == self.p - 1)
            && b.coefficients[1..b.len as usize - 1].iter().all(|&v| v == 0)
        {
            let d = b.len as usize - 1;
            let c = b.coefficients[0];
            for high in (d..a.len as usize).rev() {
                let factor = remainder.coefficients[high];
                if factor == 0 { continue; }
                self.gate.charge(COPY_WORK)?;
                let low = high - d;
                quotient.coefficients[low] = factor;
                remainder.coefficients[low] = if c == 1 {
                    self.sub(remainder.coefficients[low], factor)?
                } else if c == self.p - 1 {
                    self.add(remainder.coefficients[low], factor)?
                } else {
                    let scaled = self.mul(factor, c)?;
                    self.sub(remainder.coefficients[low], scaled)?
                };
                remainder.coefficients[high] = 0;
            }
            remainder.len = d as u8;
            quotient.trim();
            remainder.trim();
            return Ok((quotient, remainder));
        }
        // R59_BINOMIAL_DIVISION_END
        // Most GCD and splitting factors are monic. In that case no
        // reciprocal, leading multiplication, or leading subtraction is needed.
        // For a general divisor, the leading term also cancels exactly.
        let monic = b.coefficients[b.len as usize - 1] == 1;
        let inv = if monic { 1 } else { self.inverse(b.coefficients[b.len as usize - 1])? };
        while remainder.len >= b.len {
            self.gate.charge(COPY_WORK)?;
            let high = remainder.len as usize - 1;
            let shift = (remainder.len - b.len) as usize;
            let coefficient = if monic {
                remainder.coefficients[high]
            } else {
                self.mul(remainder.coefficients[high], inv)?
            };
            quotient.coefficients[shift] = coefficient;
            for j in 0..b.len as usize - 1 {
                let bj = b.coefficients[j];
                if bj == 0 { continue; }
                let v = self.mul(coefficient, bj)?;
                remainder.coefficients[shift + j] =
                    self.sub(remainder.coefficients[shift + j], v)?;
            }
            remainder.coefficients[high] = 0;
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
        // R52_REDUCE_BEGIN
        // Polynomial remainder modulo x is its constant coefficient.
        if modulus.len == 2 && modulus.coefficients[0] == 0 && value.len >= 2 {
            *value = Poly17::constant(value.coefficients[0]);
            return Ok(());
        }
        // R52_REDUCE_END
        // R56_PRODUCER_MONOMIAL_REDUCTION_BEGIN
        // f mod x^d is exactly its low d coefficients. Aggregated
        // metering charges the same nonzero leading cancellations as the
        // former trim-driven loop, with a single deadline check.
        if modulus.len >= 3
            && value.len >= modulus.len
            && modulus.coefficients[..modulus.len as usize - 1].iter().all(|&c| c == 0)
        {
            let n = value.len as usize;
            let d = modulus.len as usize - 1;
            let cancellations = value.coefficients[d..n].iter().filter(|&&c| c != 0).count();
            self.gate.charge(COPY_WORK * cancellations as u64)?;
            value.coefficients[d..n].fill(0);
            value.len = d as u8;
            value.trim();
            return Ok(());
        }
        // R56_PRODUCER_MONOMIAL_REDUCTION_END
        // R59_BINOMIAL_REDUCTION_BEGIN
        // Reduce modulo monic x^d+c by a single descending coefficient
        // fold. This avoids scanning d zero coefficients and trimming per
        // cancellation. Each nonzero term retains a deadline/work check.
        if modulus.len >= 2 && value.len >= modulus.len
            && (modulus.coefficients[0] == 1 || modulus.coefficients[0] == self.p - 1)
            && modulus.coefficients[1..modulus.len as usize - 1].iter().all(|&v| v == 0)
        {
            let d = modulus.len as usize - 1;
            let c = modulus.coefficients[0];
            for high in (d..value.len as usize).rev() {
                let factor = value.coefficients[high];
                if factor == 0 { continue; }
                self.gate.charge(COPY_WORK)?;
                let low = high - d;
                value.coefficients[low] = if c == 1 {
                    self.sub(value.coefficients[low], factor)?
                } else if c == self.p - 1 {
                    self.add(value.coefficients[low], factor)?
                } else {
                    let scaled = self.mul(factor, c)?;
                    self.sub(value.coefficients[low], scaled)?
                };
                value.coefficients[high] = 0;
            }
            value.len = d as u8;
            value.trim();
            return Ok(());
        }
        // R59_BINOMIAL_REDUCTION_END
        // x^d+constant and x^d+... with a zero low coefficient appear
        // in sparse factor bases; avoid paying the sparse branch on dense f.
        let sparse_modulus = modulus.coefficients[0] == 0
            || (modulus.len > 2 && modulus.coefficients[1] == 0);
        while value.len >= modulus.len {
            self.gate.charge(COPY_WORK)?;
            let high = value.len as usize - 1;
            let coefficient = value.coefficients[high];
            let shift = (value.len - modulus.len) as usize;
            if sparse_modulus {
                for j in 0..modulus.len as usize - 1 {
                    let mj = modulus.coefficients[j];
                    if mj == 0 { continue; }
                    let product = self.mul(coefficient, mj)?;
                    value.coefficients[shift + j] =
                        self.sub(value.coefficients[shift + j], product)?;
                }
            } else {
                for j in 0..modulus.len as usize - 1 {
                    let product = self.mul(coefficient, modulus.coefficients[j])?;
                    value.coefficients[shift + j] =
                        self.sub(value.coefficients[shift + j], product)?;
                }
            }
            value.coefficients[high] = 0;
            value.trim();
        }
        Ok(())
    }
    // Symmetric polynomial square for repeated Frobenius powering.
    // Off-diagonal products occur twice and can be doubled after one mul.
    // Skip exact-zero terms in sparse powers; all outputs stay canonical.
    fn square(&mut self, a: &Poly17) -> Result<Poly17> {
        self.gate.charge(3 * COPY_WORK)?;
        let mut out = Poly17::ZERO;
        if a.len == 0 { return Ok(out); }
        let n = 2 * a.len as usize - 1;
        if n > 17 { return Err(Stop::InvalidInput); }
        out.len = n as u8;
        // R55_PRODUCER_SMALL_SQUARE_BEGIN
        // Scalar squares and (x+c)^2 need only one multiplication and,
        // for monic linear inputs, one modular doubling. In particular,
        // squaring x is a coefficient shift instead of a generic loop.
        if a.len == 1 {
            out.coefficients[0] = self.mul(a.coefficients[0], a.coefficients[0])?;
            return Ok(out);
        }
        if a.len == 2 && a.coefficients[1] == 1 {
            let c = a.coefficients[0];
            out.coefficients[0] = self.mul(c, c)?;
            out.coefficients[1] = self.add(c, c)?;
            out.coefficients[2] = 1;
            return Ok(out);
        }
        // R55_PRODUCER_SMALL_SQUARE_END
        // R62_PRODUCER_BINOMIAL_SQUARE_BEGIN
        // (x^d+c)^2=x^(2d)+2cx^d+c^2. No sparse convolution
        // is necessary, and the highest coefficient remains monic.
        if a.len >= 3 && a.coefficients[a.len as usize - 1] == 1
            && a.coefficients[1..a.len as usize - 1].iter().all(|&v| v == 0)
        {
            let d = a.len as usize - 1;
            let c = a.coefficients[0];
            out.coefficients[2 * d] = 1;
            if c != 0 {
                out.coefficients[0] = if c == 1 || c == self.p - 1 {
                    1
                } else {
                    self.mul(c, c)?
                };
                out.coefficients[d] = self.add(c, c)?;
            }
            return Ok(out);
        }
        // R62_PRODUCER_BINOMIAL_SQUARE_END
        // R43_PRODUCER_SQUARE_BEGIN
        // In dense squares, double each row coefficient once rather than
        // doubling all off-diagonal products. Each term remains canonical.
        // R55_PRODUCER_DENSE_THRESHOLD_BEGIN
        // For dense degree-two through degree-four inputs, the already
        // tested symmetric algorithm avoids redundant cross-products and
        // additions just as it does for larger inputs.
        if a.len >= 2 && !a.coefficients[..a.len as usize].contains(&0) {
        // R55_PRODUCER_DENSE_THRESHOLD_END
            for i in 0..a.len as usize {
                let ai = a.coefficients[i];
                let diag = self.mul(ai, ai)?;
                // R54_PRODUCER_SQUARE_DIAG_BEGIN
                // Only the first and last diagonal are new coefficients.
                if i == 0 || i + 1 == a.len as usize {
                    out.coefficients[2 * i] = diag;
                } else {
                    out.coefficients[2 * i] = self.add(out.coefficients[2 * i], diag)?;
                }
                // R54_PRODUCER_SQUARE_DIAG_END
                if i + 1 < a.len as usize {
                    let twice_ai = self.add(ai, ai)?;
                    for j in i + 1..a.len as usize {
                        let cross = self.mul(twice_ai, a.coefficients[j])?;
                        // R54_PRODUCER_SQUARE_CROSS_BEGIN
                        // At row zero or the last column, this degree
                        // has not yet been populated by an earlier pair.
                        if i == 0 || j + 1 == a.len as usize {
                            out.coefficients[i + j] = cross;
                        } else {
                            out.coefficients[i + j] =
                                self.add(out.coefficients[i + j], cross)?;
                        }
                        // R54_PRODUCER_SQUARE_CROSS_END
                    }
                }
            }
            out.trim();
            return Ok(out);
        }
        // R43_PRODUCER_SQUARE_END
        for i in 0..a.len as usize {
            let ai = a.coefficients[i];
            if ai == 0 { continue; }
            let diag = self.mul(ai, ai)?;
            out.coefficients[2 * i] = self.add(out.coefficients[2 * i], diag)?;
            for j in i + 1..a.len as usize {
                let aj = a.coefficients[j];
                if aj == 0 { continue; }
                let cross = self.mul(ai, aj)?;
                let twice = self.add(cross, cross)?;
                out.coefficients[i + j] = self.add(out.coefficients[i + j], twice)?;
            }
        }
        out.trim();
        Ok(out)
    }

    // R44_PRODUCER_WINDOW_HELPER_BEGIN
    // Width-three left-to-right sliding window for non-Mersenne primes.
    // Odd powers b,b^3,b^5,b^7 cost one square and three products,
    // but reduce the number of full products in dense 30-bit powers.
    fn window3_power(&mut self, base: &Poly17, exponent: u64, monic: &Poly17) -> Result<Poly17> {
        debug_assert!(exponent > 0);
        let mut odd = [Poly17::ZERO; 4];
        odd[0] = *base;
        let mut b2 = self.square(base)?;
        self.reduce_monic(&mut b2, monic)?;
        for index in 1..odd.len() {
            self.gate.charge(4 * COPY_WORK)?;
            let mut next = self.product(&odd[index - 1], &b2)?;
            self.reduce_monic(&mut next, monic)?;
            odd[index] = next;
        }
        let mut acc = Poly17::constant(1);
        let mut initialized = false;
        let mut bit = 63i32 - exponent.leading_zeros() as i32;
        while bit >= 0 {
            self.gate.charge(4 * COPY_WORK)?;
            if exponent & (1u64 << bit) == 0 {
                if initialized {
                    let mut next = self.square(&acc)?;
                    self.reduce_monic(&mut next, monic)?;
                    acc = next;
                }
                bit -= 1;
                continue;
            }
            let mut low = (bit - 2).max(0);
            while exponent & (1u64 << low) == 0 {
                low += 1;
            }
            let width = bit - low + 1;
            let digit = ((exponent >> low) & ((1u64 << width) - 1)) as usize;
            debug_assert!(digit & 1 == 1 && digit <= 7);
            if initialized {
                for _ in 0..width {
                    let mut next = self.square(&acc)?;
                    self.reduce_monic(&mut next, monic)?;
                    acc = next;
                }
                let mut next = self.product(&acc, &odd[digit >> 1])?;
                self.reduce_monic(&mut next, monic)?;
                acc = next;
            } else {
                acc = odd[digit >> 1];
                initialized = true;
            }
            bit = low - 1;
        }
        Ok(acc)
    }
    // R44_PRODUCER_WINDOW_HELPER_END
    // R42_PRODUCER_HELPER_BEGIN
    // For e=2^k-1, let U_k=b^(2^k-1).  Then
    // U_(2m)=(U_m)^(2^m)*U_m and U_(2m+1)=U_(2m)^2*b.
    // This removes most full polynomial multiplications for Mersenne exponents.
    fn mersenne_bits_power(&mut self, base: &Poly17, bits: u32, monic: &Poly17) -> Result<Poly17> {
        debug_assert!(bits > 0);
        if bits == 1 {
            return Ok(*base);
        }
        let half = bits / 2;
        let lower = self.mersenne_bits_power(base, half, monic)?;
        let mut shifted = lower;
        for _ in 0..half {
            self.gate.charge(4 * COPY_WORK)?;
            let mut squared = self.square(&shifted)?;
            self.reduce_monic(&mut squared, monic)?;
            shifted = squared;
        }
        self.gate.charge(4 * COPY_WORK)?;
        let mut combined = self.product(&shifted, &lower)?;
        self.reduce_monic(&mut combined, monic)?;
        if bits & 1 != 0 {
            self.gate.charge(4 * COPY_WORK)?;
            let mut squared = self.square(&combined)?;
            self.reduce_monic(&mut squared, monic)?;
            self.gate.charge(4 * COPY_WORK)?;
            let mut extended = self.product(&squared, base)?;
            self.reduce_monic(&mut extended, monic)?;
            combined = extended;
        }
        Ok(combined)
    }
    // R42_PRODUCER_HELPER_END
    fn power(&mut self, base: &Poly17, mut exponent: u64, modulus: &Poly17) -> Result<Poly17> {
        self.gate.charge(6 * COPY_WORK)?;
        // Every polynomial reduces to zero modulo a nonzero constant;
        // avoid even normalizing the constant (which would invert it).
        if modulus.len == 1 {
            return Ok(Poly17::ZERO);
        }
        let monic = self.power_modulus(modulus)?;

        // R57_MONOMIAL_FROBENIUS_PRODUCER_BEGIN
        // Frobenius: (x+delta)^p = x^p+delta in characteristic p.
        // Modulo x^d, the x^p term vanishes for p>=d; otherwise it
        // is already reduced. No exponentiation, convolution or GCD
        // shortcut is needed, and the LinearPart event stays unchanged.
        if exponent == self.p && monic.len >= 3
            && base.len == 2 && base.coefficients[1] == 1
            && monic.coefficients[..monic.len as usize - 1].iter().all(|&c| c == 0)
        {
            let degree = monic.len as usize - 1;
            let mut result = Poly17::constant(base.coefficients[0]);
            if self.p < degree as u64 {
                result.coefficients[self.p as usize] = 1;
                result.len = (self.p as usize + 1) as u8;
            }
            return Ok(result);
        }
        // R57_MONOMIAL_FROBENIUS_PRODUCER_END
        // R58_BINOMIAL_FROBENIUS_PRODUCER_BEGIN
        // In Fp[x]/(x^d+c), x^p=(-c)^(p/d)*x^(p%d).
        // This is valid for affine x+delta by Frobenius, including p<=d,
        // after normalizing the leading coefficient. The scalar exponent
        // needs O(log p) field operations instead of polynomial convolution.
        if exponent == self.p && monic.len >= 4
            && base.len == 2 && base.coefficients[1] == 1
            && monic.coefficients[0] != 0
            && monic.coefficients[1..monic.len as usize - 1].iter().all(|&c| c == 0)
        {
            let degree = monic.len as usize - 1;
            let quotient = self.p / degree as u64;
            let remainder = (self.p % degree as u64) as usize;
            let scaled = if quotient == 0 {
                1
            } else {
                let neg_c = self.sub(0, monic.coefficients[0])?;
                self.split_scalar_pow(neg_c, quotient)?
            };
            let delta = base.coefficients[0];
            let mut result = Poly17::constant(delta);
            if remainder == 0 {
                result = Poly17::constant(self.add(delta, scaled)?);
            } else {
                result.coefficients[remainder] = scaled;
                result.len = (remainder + 1) as u8;
            }
            return Ok(result);
        }
        // R58_BINOMIAL_FROBENIUS_PRODUCER_END
        // Over Fp, r^p == r. In Fp[x]/(x-r), x^p is therefore r.
        // The canonical input x is the only base eligible for this path.
        if monic.len == 2 && exponent == self.p
            && base.len == 2 && base.coefficients[0] == 0
            && base.coefficients[1] == 1
        {
            return Ok(Poly17::constant(self.sub(0, monic.coefficients[0])?));
        }
        // R45_QUADRATIC_FROBENIUS_PRODUCER_BEGIN
        // In Fp[x]/(x^2+b*x+c), x^p equals x when the discriminant
        // is a nonzero square, -b-x when irreducible, and -b/2 when
        // the discriminant vanishes. Euler's criterion uses scalar
        // field operations rather than degree-two polynomial powering.
        // Prime p >= 3 is guaranteed by Fp64Arithmetic::try_new.
        if monic.len == 3 && exponent == self.p
            && base.len == 2 && base.coefficients[0] == 0
            && base.coefficients[1] == 1
        {
            let linear = monic.coefficients[1];
            let doubled = self.add(monic.coefficients[0], monic.coefficients[0])?;
            let four_c = self.add(doubled, doubled)?;
            let b_squared = self.mul(linear, linear)?;
            let discriminant = self.sub(b_squared, four_c)?;
            if discriminant == 0 {
                let half = (self.p + 1) / 2;
                let root = self.mul(linear, half)?;
                return Ok(Poly17::constant(self.sub(0, root)?));
            }
            // R49_FROBENIUS_PRODUCER_BEGIN
            let symbol = self.quadratic_character(discriminant)?;
            // R49_FROBENIUS_PRODUCER_END
            if symbol == 1 {
                return Ok(*base);
            }
            if symbol == -1 {
                let mut value = Poly17::ZERO;
                value.len = 2;
                value.coefficients[0] = self.sub(0, linear)?;
                value.coefficients[1] = self.sub(0, 1)?;
                return Ok(value);
            }
            return Err(Stop::InvalidInput);
        }
        // R45_QUADRATIC_FROBENIUS_PRODUCER_END
        let mut b = *base;
        self.reduce_monic(&mut b, &monic)?;
        // R42_PRODUCER_FASTPATH_BEGIN
        // Both x^p and split's (x+delta)^((p-1)/2) have all-one
        // exponents for p=2^61-1. Avoid 51-53 redundant poly products.
        if self.p == (1u64 << 61) - 1
            && (exponent == self.p || exponent == (self.p - 1) / 2)
        {
            return self.mersenne_bits_power(&b, 64 - exponent.leading_zeros(), &monic);
        }
        // R42_PRODUCER_FASTPATH_END
        // R44_PRODUCER_WINDOW_SWITCH_BEGIN
        // Avoid table setup on small degrees, sparse-bit exponents, and
        // the 61-bit prime already handled by the Mersenne addition chain.
        if monic.len >= 5 && exponent >= (1u64 << 20)
            && exponent <= u32::MAX as u64 && exponent.count_ones() >= 12
            && (exponent == self.p || exponent == (self.p - 1) / 2)
        {
            return self.window3_power(&b, exponent, &monic);
        }
        // R44_PRODUCER_WINDOW_SWITCH_END
        // For 61-bit exponents, the extra first-bit state test was slower
        // than one identity multiply in local regression screening.
        if exponent > u32::MAX as u64 {
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
                    let mut product = self.square(&b)?;
                    self.reduce_monic(&mut product, &monic)?;
                    b = product;
                }
            }
            return Ok(acc);
        }
        let mut acc = Poly17::constant(1);
        // The identity is reduced modulo any positive-degree polynomial.
        // A degree-zero modulus is exceptional: 1 mod c = 0.
        if monic.len == 1 {
            self.reduce_monic(&mut acc, &monic)?;
        }
        let mut acc_initialized = false;
        while exponent != 0 {
            self.gate.charge(4 * COPY_WORK)?;
            if exponent & 1 != 0 {
                if acc_initialized {
                    let mut product = self.product(&acc, &b)?;
                    self.reduce_monic(&mut product, &monic)?;
                    acc = product;
                } else {
                    // First set bit: 1 * b mod m == b, already reduced.
                    acc = b;
                    acc_initialized = true;
                }
            }
            exponent >>= 1;
            if exponent != 0 {
                let mut product = self.square(&b)?;
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
            // A nonzero constant is a unit in Fp[x]. The GCD is already 1;
            // polynomial division and two Fermat inversions are unnecessary.
            // Keep the loop charge/deadline check before this early return.
            if b.len == 1 {
                return Ok(Poly17::constant(1));
            }
            let r = self.division(&a, &b)?.1;
            a = b;
            b = r;
        }
        // Avoid an inverse and field multiplications for already-monic
        // results, including a unit leading coefficient.
        if a.len != 0 && a.coefficients[a.len as usize - 1] != 1 {
            let inv = self.inverse(a.coefficients[a.len as usize - 1])?;
            for i in 0..a.len as usize {
                a.coefficients[i] = self.mul(a.coefficients[i], inv)?;
            }
        }
        Ok(a)
    }
    // R49_BINARY_JACOBI_PRODUCER_BEGIN
    // The binary Jacobi symbol equals the Legendre symbol for odd prime p.
    // This replaces ~log2(p) modular squarings/products during quadratic
    // Frobenius and quadratic factor selection. Never used for square roots.
    // Charge integer work/deadline each step; do not charge field operations.
    fn quadratic_character(&mut self, mut a: u64) -> Result<i8> {
        let mut n = self.p;
        let mut sign = 1i8;
        self.gate.charge(32)?;
        while a != 0 {
            self.gate.charge(32)?;
            let twos = a.trailing_zeros();
            a >>= twos;
            if twos & 1 != 0 && (n & 7 == 3 || n & 7 == 5) {
                sign = -sign;
            }
            if a & 3 == 3 && n & 3 == 3 {
                sign = -sign;
            }
            let next = n % a;
            n = a;
            a = next;
        }
        Ok(if n == 1 { sign } else { 0 })
    }
    // R49_BINARY_JACOBI_PRODUCER_END
    // R46_PRODUCER_HELPER_BEGIN
    // In a split squarefree quadratic, evaluating the quadratic character
    // at both roots is much cheaper than polynomial Euler powering + GCD.
    // For p == 3 (mod 4), sqrt(D) is D^((p+1)/4). Return None for any
    // unsupported/degenerate case, retaining the generic path.
    fn split_scalar_pow(&mut self, base: u64, mut exponent: u64) -> Result<u64> {
        // R61_PRODUCER_SCALAR_IDENTITY_BEGIN
        // 1 * factor is factor: the first set bit needs no field product.
        // Retain the original metered operation for exponent one, so the
        // single-operation fast path still checks its deadline.
        if exponent == 1 {
            return self.mul(1, base);
        }
        let mut acc = 1u64;
        let mut factor = base;
        let mut initialized = false;
        while exponent != 0 {
            if exponent & 1 != 0 {
                if initialized {
                    acc = self.mul(acc, factor)?;
                } else {
                    acc = factor;
                    initialized = true;
                }
            }
            exponent >>= 1;
            if exponent != 0 {
                factor = self.mul(factor, factor)?;
            }
        }
        Ok(acc)
        // R61_PRODUCER_SCALAR_IDENTITY_END
    }
    fn quadratic_split_factor(&mut self, h: &Poly17, delta: u64) -> Result<Option<Poly17>> {
        if h.len != 3 || (self.p & 3 != 3 && self.p & 7 != 5 && self.p != 1_000_000_009) || h.coefficients[2] != 1 {
            return Ok(None);
        }
        let b = h.coefficients[1];
        let c = h.coefficients[0];
        let double_c = self.add(c, c)?;
        let squared_b = self.mul(b, b)?;
        let four_c = self.add(double_c, double_c)?;
        let discriminant = self.sub(squared_b, four_c)?;
        if discriminant == 0 {
            return Ok(None);
        }
        // R47_SQRT_PRODUCER_BEGIN
        // For p = 5 mod 8, Atkin's square root avoids polynomial factoring.
        // For p = 1e9+9, v2(p-1)=3; 11 is a fixed quadratic nonresidue.
        // All candidates are checked before use; unsupported cases fall back.
        let sqrt_d = if self.p & 3 == 3 {
            self.split_scalar_pow(discriminant, (self.p + 1) / 4)?
        } else if self.p & 7 == 5 {
            let candidate = self.split_scalar_pow(discriminant, (self.p + 3) / 8)?;
            if self.mul(candidate, candidate)? == discriminant {
                candidate
            } else {
                let sqrt_minus_one = self.split_scalar_pow(2, (self.p - 1) / 4)?;
                self.mul(candidate, sqrt_minus_one)?
            }
        } else {
            debug_assert_eq!(self.p, 1_000_000_009);
            // R48_TONELLI_PRODUCER_BEGIN
            // 11^((p-1)/8) mod p = 172252358 for p=1_000_000_009.
            // Its fourth power is -1 and eighth power is 1. Avoid
            // recomputing a fixed nonresidue power on every split attempt.
            const NONRESIDUE_Q_POWER: u64 = 172_252_358;
            let q = (self.p - 1) / 8;
            let mut c = NONRESIDUE_Q_POWER;
            // Obtain d^q and d^((q+1)/2) with a shared square ladder.
            // Separate binary powers square the same base twice at each bit.
            let mut t = 1u64;
            let mut r = 1u64;
            let mut base = discriminant;
            let mut exp_t = q;
            let mut exp_r = (q + 1) / 2;
            while exp_t != 0 || exp_r != 0 {
                if exp_t & 1 != 0 {
                    t = self.mul(t, base)?;
                }
                if exp_r & 1 != 0 {
                    r = self.mul(r, base)?;
                }
                exp_t >>= 1;
                exp_r >>= 1;
                if exp_t != 0 || exp_r != 0 {
                    base = self.mul(base, base)?;
                }
            }
            // R48_TONELLI_PRODUCER_END
            let mut m = 3usize;
            while t != 1 {
                let mut v = t;
                let mut i = 0usize;
                while v != 1 && i < m {
                    v = self.mul(v, v)?;
                    i += 1;
                }
                if i == m { return Ok(None); }
                let mut b = c;
                for _ in 1..(m - i) {
                    b = self.mul(b, b)?;
                }
                r = self.mul(r, b)?;
                c = self.mul(b, b)?;
                t = self.mul(t, c)?;
                m = i;
            }
            r
        };
        // R47_SQRT_PRODUCER_END
        if self.mul(sqrt_d, sqrt_d)? != discriminant {
            return Ok(None);
        }
        // R50_QUADRATIC_FINISH_PRODUCER_BEGIN
        // For odd p, half(a) = (a >> 1) + (a & 1) * ((p+1)/2).
        // Avoid two modular multiplications by inv(2), and use Vieta's
        // r0+r1=-b to reconstruct the other root. Canonical input only.
        let neg_b = self.sub(0, b)?;
        let numerator_a = self.add(neg_b, sqrt_d)?;
        self.gate.charge(32)?;
        let root_a = (numerator_a >> 1)
            + ((numerator_a & 1) * ((self.p + 1) / 2));
        let root_b = self.sub(neg_b, root_a)?;
        // R50_QUADRATIC_FINISH_PRODUCER_END
        if root_a == root_b {
            return Ok(None);
        }
        let shifted_a = self.add(root_a, delta)?;
        let shifted_b = self.add(root_b, delta)?;
        // R49_SPLIT_PRODUCER_BEGIN
        let a_selected = self.quadratic_character(shifted_a)? == 1;
        let b_selected = self.quadratic_character(shifted_b)? == 1;
        // R49_SPLIT_PRODUCER_END
        if a_selected && b_selected {
            return Ok(Some(*h));
        }
        if !a_selected && !b_selected {
            return Ok(Some(Poly17::constant(1)));
        }
        let root = if a_selected { root_a } else { root_b };
        let mut factor = Poly17::ZERO;
        factor.len = 2;
        factor.coefficients[0] = self.sub(0, root)?;
        factor.coefficients[1] = 1;
        Ok(Some(factor))
    }
    // R46_PRODUCER_HELPER_END

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
        // Every degree-one factor of x^p-x over Fp is exactly the
        // normalized input. A nonzero constant is coprime to x^p-x.
        // Preserve the same LinearPart event, but omit generic Euclid.
        let g = if f.len == 1 {
            Poly17::constant(1)
        } else if f.len == 2 {
            self.power_modulus(&f)?
        // R45_QUADRATIC_GCD_PRODUCER_BEGIN
        } else if f.len == 3 && xp.len == 2
            && xp.coefficients[0] == 0 && xp.coefficients[1] == 1
        {
            // x^p == x mod f: the quadratic is squarefree and split.
            self.power_modulus(&f)?
        } else if f.len == 3 && xp.len == 2
            && xp.coefficients[1] == self.p - 1
        {
            // x^p == -b-x: the quadratic is irreducible.
            Poly17::constant(1)
        } else if f.len == 3 && xp.len <= 1 {
            // A repeated quadratic has one distinct root r=x^p.
            let mut linear = Poly17::ZERO;
            linear.len = 2;
            linear.coefficients[0] = self.sub(0, xp.coefficients[0])?;
            linear.coefficients[1] = 1;
            linear
        // R45_QUADRATIC_GCD_PRODUCER_END
        } else {
            self.gcd(f, difference)?
        };
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
                let root = if h.coefficients[1] == 1 {
                    // Split factors are normally monic: x + c has root -c.
                    self.sub(0, h.coefficients[0])?
                } else {
                    let inv = self.inverse(h.coefficients[1])?;
                    let value = self.mul(h.coefficients[0], inv)?;
                    self.sub(0, value)?
                };
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
            // R46_PRODUCER_SPLIT_BEGIN
            let d = if let Some(d) = self.quadratic_split_factor(&h, delta)? {
                d
            } else {
                let mut s = self.power(&base, (self.p - 1) / 2, &h)?;
                s.len = s.len.max(1);
                s.coefficients[0] = self.sub(s.coefficients[0], 1)?;
                s.trim();
                self.gcd(h, s)?
            };
            // R46_PRODUCER_SPLIT_END
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
    // Replay uses independently implemented canonical arithmetic. In
    // particular it never invokes Producer, and the raw-wide oracle in
    // the regression tests checks each fast reduction separately.
    fn times(&mut self, x: u64, y: u64) -> Result<u64> {
        self.gate.scalar()?;
        let p = self.p;
        Ok(if p == (1u64 << 61) - 1 {
            let wide = u128::from(x) * u128::from(y);
            let low = wide as u64 & p;
            let high = (wide >> 61) as u64;
            let combined = low + high;
            let reduced = (combined & p) + (combined >> 61);
            if reduced == p { 0 } else { reduced }
        } else if p <= u32::MAX as u64 {
            (x * y) % p
        } else {
            (u128::from(x) * u128::from(y) % u128::from(p)) as u64
        })
    }
    fn plus(&mut self, x: u64, y: u64) -> Result<u64> {
        self.gate.scalar()?;
        let sum = x + y;
        Ok(if sum < self.p { sum } else { sum - self.p })
    }
    fn minus(&mut self, x: u64, y: u64) -> Result<u64> {
        self.gate.scalar()?;
        Ok(if x >= y { x - y } else { self.p - (y - x) })
    }
    // R53_REPLAY_RECIPROCAL_BEGIN
    // Independently implemented binary almost-inverse. The producer uses
    // quotient Euclid; replay uses shifts/subtractions and never invokes it.
    // All intermediates fit u64 because p <= 2^61-1. Public input only:
    // this is variable time, not a secret-key field inversion.
    fn reciprocal(&mut self, x: u64) -> Result<u64> {
        if x == 0 {
            return Err(Stop::InvalidCertificate);
        }
        self.gate.charge(32)?;
        if x == 1 {
            return Ok(1);
        }
        let (mut a, mut b) = (x, self.p);
        let (mut u, mut v) = (1u64, 0u64);
        while a != 1 && b != 1 {
            while a & 1 == 0 {
                self.gate.scalar()?;
                a >>= 1;
                u = if u & 1 == 0 { u >> 1 } else { (u + self.p) >> 1 };
            }
            while b & 1 == 0 {
                self.gate.scalar()?;
                b >>= 1;
                v = if v & 1 == 0 { v >> 1 } else { (v + self.p) >> 1 };
            }
            self.gate.scalar()?;
            if a >= b {
                a -= b;
                u = if u >= v { u - v } else { u + self.p - v };
            } else {
                b -= a;
                v = if v >= u { v - u } else { v + self.p - u };
            }
        }
        Ok(if a == 1 { u } else { v })
    }
    // R53_REPLAY_RECIPROCAL_END
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
        // R51_REPLAY_CONVOLUTION_BEGIN
        // Independent verifier path for scalar, x, and monic-linear
        // convolution. Traverse output coefficients rather than producer
        // rows; each nontrivial field operation retains its gate check.
        if x.len == 1 || y.len == 1 {
            let (scalar, poly) = if x.len == 1 { (x, y) } else { (y, x) };
            if scalar.coefficients[0] == 1 {
                return Ok(*poly);
            }
            z.len = poly.len;
            // R62_REPLAY_UNIT_SCALAR_BEGIN
            // Replay uses its independent subtraction routine for -f.
            if scalar.coefficients[0] == self.p - 1 {
                for k in 0..poly.len as usize {
                    self.gate.charge(32)?;
                    z.coefficients[k] = self.minus(0, poly.coefficients[k])?;
                }
                z.trim();
                return Ok(z);
            }
            // R62_REPLAY_UNIT_SCALAR_END
            for k in 0..poly.len as usize {
                self.gate.charge(32)?;
                z.coefficients[k] = self.times(scalar.coefficients[0], poly.coefficients[k])?;
            }
            z.trim();
            return Ok(z);
        }
        let linear = if x.len == 2 && x.coefficients[1] == 1 {
            Some((x, y))
        } else if y.len == 2 && y.coefficients[1] == 1 {
            Some((y, x))
        } else {
            None
        };
        if let Some((linear, poly)) = linear {
            let size = poly.len as usize;
            let c = linear.coefficients[0];
            if c == 0 {
                // No field operations are needed to multiply by x.
                z.coefficients[1..=size]
                    .copy_from_slice(&poly.coefficients[..size]);
                return Ok(z);
            }
            // R62_REPLAY_UNIT_LINEAR_BEGIN
            // Independently compute diagonals of (x+/-1)f by one
            // canonical field sum/difference per interior coefficient.
            if c == 1 || c == self.p - 1 {
                for k in 0..=size {
                    self.gate.charge(32)?;
                    z.coefficients[k] = if k == 0 {
                        if c == 1 { poly.coefficients[0] }
                        else { self.minus(0, poly.coefficients[0])? }
                    } else if k == size {
                        poly.coefficients[size - 1]
                    } else if c == 1 {
                        self.plus(poly.coefficients[k - 1], poly.coefficients[k])?
                    } else {
                        self.minus(poly.coefficients[k - 1], poly.coefficients[k])?
                    };
                }
                z.trim();
                return Ok(z);
            }
            // R62_REPLAY_UNIT_LINEAR_END
            for k in 0..=size {
                self.gate.charge(32)?;
                z.coefficients[k] = if k == 0 {
                    self.times(c, poly.coefficients[0])?
                } else if k == size {
                    poly.coefficients[size - 1]
                } else {
                    let scaled = self.times(c, poly.coefficients[k])?;
                    self.plus(poly.coefficients[k - 1], scaled)?
                };
            }
            z.trim();
            return Ok(z);
        }
        // R51_REPLAY_CONVOLUTION_END
        // Keep the independent output-diagonal verifier traversal; only
        // select the zero-eliding version for structured inputs.
        // R61_REPLAY_LINEAR_ZERO_BEGIN
        // Detect missing x coefficients without a full polynomial scan.
        // Replay remains output-diagonal rather than producer row-major.
        let sparse = x.coefficients[0] == 0 || y.coefficients[0] == 0
            || (x.len > 2 && x.coefficients[1] == 0)
            || (y.len > 2 && y.coefficients[1] == 0);
        // R61_REPLAY_LINEAR_ZERO_END
        for k in 0..n {
            self.gate.charge(32)?;
            let start = (k + 1).saturating_sub(usize::from(y.len));
            let end = usize::from(x.len).min(k + 1);
            for i in start..end {
                self.gate.charge(32)?;
                let xi = x.coefficients[i];
                let yj = y.coefficients[k - i];
                if sparse && (xi == 0 || yj == 0) { continue; }
                let v = self.times(xi, yj)?;
                // R54_REPLAY_CONVOLUTION_BEGIN
                // On a dense output diagonal the first term has no
                // predecessor. Sparse diagonals keep the old accumulation.
                if !sparse && i == start {
                    z.coefficients[k] = v;
                } else {
                    z.coefficients[k] = self.plus(z.coefficients[k], v)?;
                }
                // R54_REPLAY_CONVOLUTION_END
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
        // A nonzero constant divides every polynomial exactly. This also
        // avoids a redundant reciprocal in the final Euclidean iteration.
        if denominator.len == 1 {
            return Ok(Poly17::ZERO);
        }
        let d = usize::from(denominator.len);
        // R52_REMAINDER_BEGIN
        // Independent verifier: f(x) mod x is the constant coefficient.
        if d == 2 && denominator.coefficients[0] == 0
            && denominator.coefficients[1] == 1 {
            return Ok(Poly17::constant(numerator.coefficients[0]));
        }
        // R52_REMAINDER_END
        // R56_REPLAY_MONOMIAL_REMAINDER_BEGIN
        // Any nonzero scalar multiple of x^d has the same remainder.
        // Replay independently charges each eliminated degree in aggregate.
        if d >= 3 && denominator.coefficients[..d - 1].iter().all(|&c| c == 0) {
            let n = numerator.len as usize;
            let degree = d - 1;
            self.gate.charge(COPY_WORK * (n - degree) as u64)?;
            r.coefficients[degree..n].fill(0);
            r.len = degree as u8;
            r.trim();
            return Ok(r);
        }
        // R56_REPLAY_MONOMIAL_REMAINDER_END
        // R59_BINOMIAL_REMAINDER_BEGIN
        // Independent descending-index polynomial remainder: for monic
        // x^d+c the only surviving cancellation lies d slots below.
        // Treat +/-1 as additive units, independently of producer code.
        if d >= 2 && denominator.coefficients[d - 1] == 1
            && (denominator.coefficients[0] == 1 || denominator.coefficients[0] == self.p - 1)
            && denominator.coefficients[1..d - 1].iter().all(|&v| v == 0)
        {
            let c = denominator.coefficients[0];
            let degree = d - 1;
            for high in (degree..usize::from(numerator.len)).rev() {
                self.gate.charge(COPY_WORK)?;
                let factor = r.coefficients[high];
                if factor == 0 { continue; }
                let low = high - degree;
                r.coefficients[low] = if c == 1 {
                    self.minus(r.coefficients[low], factor)?
                } else if c == self.p - 1 {
                    self.plus(r.coefficients[low], factor)?
                } else {
                    let scaled = self.times(factor, c)?;
                    self.minus(r.coefficients[low], scaled)?
                };
                r.coefficients[high] = 0;
            }
            r.len = degree as u8;
            r.trim();
            return Ok(r);
        }
        // R59_BINOMIAL_REMAINDER_END
        let leading = denominator.coefficients[d - 1];
        let monic = leading == 1;
        let inverse = if monic { 1 } else { self.reciprocal(leading)? };
        // Unlike Producer's trim-driven division, replay traverses a fixed
        // descending range. The high coefficient cancels identically and
        // must not consume a field multiplication/subtraction.
        for high in (d - 1..usize::from(numerator.len)).rev() {
            self.gate.charge(COPY_WORK)?;
            let high_coefficient = r.coefficients[high];
            if high_coefficient == 0 {
                continue;
            }
            let c = if monic {
                high_coefficient
            } else {
                self.times(high_coefficient, inverse)?
            };
            let shift = high + 1 - d;
            for j in 0..d - 1 {
                let term = denominator.coefficients[j];
                if term == 0 {
                    continue;
                }
                let product = self.times(c, term)?;
                r.coefficients[shift + j] =
                    self.minus(r.coefficients[shift + j], product)?;
            }
            r.coefficients[high] = 0;
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
        // R52_CANCEL_BEGIN
        // Independently cancel modulo x without field arithmetic.
        if divisor_len == 2 && denominator.coefficients[0] == 0
            && numerator.len >= 2 {
            *numerator = Poly17::constant(numerator.coefficients[0]);
            return Ok(());
        }
        // R52_CANCEL_END
        // R56_REPLAY_MONOMIAL_CANCEL_BEGIN
        // In Fp[x]/(x^d), canceling every high term is just truncation.
        // Unlike producer's trim-driven path, replay's original loop
        // checks every eliminated degree, including zero coefficients.
        if divisor_len >= 3 && numerator.len >= denominator.len
            && denominator.coefficients[..divisor_len - 1].iter().all(|&c| c == 0)
        {
            let n = numerator.len as usize;
            let d = divisor_len - 1;
            self.gate.charge(COPY_WORK * (n - d) as u64)?;
            numerator.coefficients[d..n].fill(0);
            numerator.len = d as u8;
            numerator.trim();
            return Ok(());
        }
        // R56_REPLAY_MONOMIAL_CANCEL_END
        // R59_BINOMIAL_CANCEL_BEGIN
        // Replay keeps its independent descending coefficient traversal
        // and the same per-degree gate checks as the general cancel loop.
        if divisor_len >= 2 && numerator.len >= denominator.len
            && (denominator.coefficients[0] == 1 || denominator.coefficients[0] == self.p - 1)
            && denominator.coefficients[1..divisor_len - 1].iter().all(|&v| v == 0)
        {
            let degree = divisor_len - 1;
            let c = denominator.coefficients[0];
            for high in (degree..usize::from(numerator.len)).rev() {
                self.gate.charge(COPY_WORK)?;
                let factor = numerator.coefficients[high];
                if factor == 0 { continue; }
                let low = high - degree;
                numerator.coefficients[low] = if c == 1 {
                    self.minus(numerator.coefficients[low], factor)?
                } else if c == self.p - 1 {
                    self.plus(numerator.coefficients[low], factor)?
                } else {
                    let scaled = self.times(factor, c)?;
                    self.minus(numerator.coefficients[low], scaled)?
                };
                numerator.coefficients[high] = 0;
            }
            numerator.len = degree as u8;
            numerator.trim();
            return Ok(());
        }
        // R59_BINOMIAL_CANCEL_END
        // Independent descending-index traversal rather than producer trim loop.
        let sparse = denominator.coefficients[0] == 0
            || (divisor_len > 2 && denominator.coefficients[1] == 0);
        for high in (divisor_len - 1..usize::from(numerator.len)).rev() {
            self.gate.charge(COPY_WORK)?;
            let factor = numerator.coefficients[high];
            let offset = high + 1 - divisor_len;
            if sparse && factor == 0 {
                numerator.coefficients[high] = 0;
                continue;
            }
            for low in 0..divisor_len - 1 {
                let term = denominator.coefficients[low];
                if sparse && term == 0 { continue; }
                let term = self.times(factor, term)?;
                numerator.coefficients[offset + low] =
                    self.minus(numerator.coefficients[offset + low], term)?;
            }
            numerator.coefficients[high] = 0;
        }
        numerator.trim();
        Ok(())
    }
    // Independent output-diagonal symmetric square for certificate replay.
    // Deliberately use a different traversal from the producer's row-major
    // square, while skipping the mirrored half of each convolution diagonal.
    fn square_diagonals(&mut self, x: &Poly17) -> Result<Poly17> {
        self.gate.charge(4 * COPY_WORK)?;
        let mut z = Poly17::ZERO;
        if x.len == 0 { return Ok(z); }
        let len = usize::from(x.len);
        let n = 2 * len - 1;
        if n > 17 { return Err(Stop::InvalidCertificate); }
        z.len = n as u8;
        // R55_REPLAY_SMALL_SQUARE_BEGIN
        // Independent closed-form squaring for scalar and monic linear
        // polynomials; replay still uses its own metered field operations.
        if len == 1 {
            z.coefficients[0] = self.times(x.coefficients[0], x.coefficients[0])?;
            return Ok(z);
        }
        if len == 2 && x.coefficients[1] == 1 {
            let c = x.coefficients[0];
            z.coefficients[0] = self.times(c, c)?;
            z.coefficients[1] = self.plus(c, c)?;
            z.coefficients[2] = 1;
            return Ok(z);
        }
        // R55_REPLAY_SMALL_SQUARE_END
        // R62_REPLAY_BINOMIAL_SQUARE_BEGIN
        // The replay square is constructed by coefficient degree,
        // independently of the producer's row-major sparse algorithm.
        if len >= 3 && x.coefficients[len - 1] == 1
            && x.coefficients[1..len - 1].iter().all(|&v| v == 0)
        {
            let d = len - 1;
            let c = x.coefficients[0];
            z.coefficients[2 * d] = 1;
            if c != 0 {
                z.coefficients[0] = if c == 1 || c == self.p - 1 {
                    1
                } else {
                    self.times(c, c)?
                };
                z.coefficients[d] = self.plus(c, c)?;
            }
            return Ok(z);
        }
        // R62_REPLAY_BINOMIAL_SQUARE_END
        // R43_REPLAY_SQUARE_BEGIN
        // Independent diagonal traversal. Cache twice the lower-index
        // coefficient so every off-diagonal term needs only one add.
        // R55_REPLAY_DENSE_THRESHOLD_BEGIN
        // For small dense polynomials use diagonal symmetric products
        // without repeated additions into zero-initialized coefficients.
        if len >= 2 && !x.coefficients[..len].contains(&0) {
        // R55_REPLAY_DENSE_THRESHOLD_END
            let mut doubled = [0u64; 17];
            for i in 0..len - 1 {
                doubled[i] = self.plus(x.coefficients[i], x.coefficients[i])?;
            }
            for k in 0..n {
                self.gate.charge(32)?;
                let start = (k + 1).saturating_sub(len);
                let end = len.min(k + 1);
                for i in start..end {
                    let j = k - i;
                    if i > j { break; }
                    self.gate.charge(32)?;
                    let left = if i == j { x.coefficients[i] } else { doubled[i] };
                    let product = self.times(left, x.coefficients[j])?;
                    // R54_REPLAY_SQUARE_BEGIN
                    // The first pair initializes this output degree.
                    if i == start {
                        z.coefficients[k] = product;
                    } else {
                        z.coefficients[k] = self.plus(z.coefficients[k], product)?;
                    }
                    // R54_REPLAY_SQUARE_END
                }
            }
            z.trim();
            return Ok(z);
        }
        // R43_REPLAY_SQUARE_END
        let sparse = x.coefficients[..len].contains(&0);
        if sparse {
            // Sparse Frobenius powers are common; omit products with an
            // exact-zero input while retaining diagonal replay order.
            for k in 0..n {
                self.gate.charge(32)?;
                let start = (k + 1).saturating_sub(len);
                let end = len.min(k + 1);
                for i in start..end {
                    let j = k - i;
                    if i > j { break; }
                    self.gate.charge(32)?;
                    if x.coefficients[i] == 0 || x.coefficients[j] == 0 {
                        continue;
                    }
                    let v = self.times(x.coefficients[i], x.coefficients[j])?;
                    let term = if i == j { v } else { self.plus(v, v)? };
                    z.coefficients[k] = self.plus(z.coefficients[k], term)?;
                }
            }
        } else {
            // Preserve the previous dense traversal without a per-product
            // zero check.
            for k in 0..n {
                self.gate.charge(32)?;
                let start = (k + 1).saturating_sub(len);
                let end = len.min(k + 1);
                for i in start..end {
                    let j = k - i;
                    if i > j { break; }
                    self.gate.charge(32)?;
                    let v = self.times(x.coefficients[i], x.coefficients[j])?;
                    let term = if i == j { v } else { self.plus(v, v)? };
                    z.coefficients[k] = self.plus(z.coefficients[k], term)?;
                }
            }
        }
        z.trim();
        Ok(z)
    }

    // R42_REPLAY_HELPER_BEGIN
    // Independent iteration over the binary expansion of the bit count k.
    // Keep diagonal squaring, descending cancellation and replay accounting.
    fn repeated_mersenne_bits_power(
        &mut self, base: &Poly17, bits: u32, monic: &Poly17,
    ) -> Result<Poly17> {
        debug_assert!(bits > 0);
        let mut length = 1u32;
        let mut acc = *base;
        let top = 31 - bits.leading_zeros();
        for bit in (0..top).rev() {
            let previous = acc;
            for _ in 0..length {
                self.gate.charge(4 * COPY_WORK)?;
                let mut square = self.square_diagonals(&acc)?;
                self.cancel_monic(&mut square, monic)?;
                acc = square;
            }
            self.gate.charge(4 * COPY_WORK)?;
            let mut product = self.convolution(&acc, &previous)?;
            self.cancel_monic(&mut product, monic)?;
            acc = product;
            length *= 2;
            if bits & (1u32 << bit) != 0 {
                self.gate.charge(4 * COPY_WORK)?;
                let mut square = self.square_diagonals(&acc)?;
                self.cancel_monic(&mut square, monic)?;
                self.gate.charge(4 * COPY_WORK)?;
                let mut product = self.convolution(&square, base)?;
                self.cancel_monic(&mut product, monic)?;
                acc = product;
                length += 1;
            }
        }
        debug_assert_eq!(length, bits);
        Ok(acc)
    }
    // R42_REPLAY_HELPER_END
    fn modular_power(&mut self, base: Poly17, exponent: u64, modulus: &Poly17) -> Result<Poly17> {
        self.gate.charge(6 * COPY_WORK)?;
        // Independent replay: modulo a nonzero constant every residue
        // is zero, so no reciprocal or convolution is necessary.
        if modulus.len == 1 {
            return Ok(Poly17::ZERO);
        }
        let monic = self.normalized_modulus(modulus)?;

        // R57_MONOMIAL_FROBENIUS_REPLAY_BEGIN
        // Independent certificate calculation: in Fp[x]/(x^d),
        // Frobenius maps x+delta to delta plus x^p when p<d.
        if exponent == self.p && monic.len >= 3
            && base.len == 2 && base.coefficients[1] == 1
            && monic.coefficients[..usize::from(monic.len) - 1].iter().all(|&v| v == 0)
        {
            let degree = usize::from(monic.len) - 1;
            let mut checked = Poly17::ZERO;
            if base.coefficients[0] != 0 {
                checked.len = 1;
                checked.coefficients[0] = base.coefficients[0];
            }
            if self.p < degree as u64 {
                let power = self.p as usize;
                checked.coefficients[power] = 1;
                checked.len = (power + 1) as u8;
            }
            return Ok(checked);
        }
        // R57_MONOMIAL_FROBENIUS_REPLAY_END
        // R58_BINOMIAL_FROBENIUS_REPLAY_BEGIN
        // Independent left-to-right scalar exponentiation checks the same
        // Frobenius identity without sharing producer arithmetic.
        if exponent == self.p && monic.len >= 4
            && base.len == 2 && base.coefficients[1] == 1
            && monic.coefficients[0] != 0
            && monic.coefficients[1..usize::from(monic.len) - 1].iter().all(|&v| v == 0)
        {
            let degree = usize::from(monic.len) - 1;
            let quotient = self.p / degree as u64;
            let remainder = (self.p % degree as u64) as usize;
            let scaled = if quotient == 0 {
                1
            } else {
                let neg_c = self.minus(0, monic.coefficients[0])?;
                self.split_character_power(neg_c, quotient)?
            };
            let delta = base.coefficients[0];
            let mut checked = Poly17::constant(delta);
            if remainder == 0 {
                checked = Poly17::constant(self.plus(delta, scaled)?);
            } else {
                checked.coefficients[remainder] = scaled;
                checked.len = (remainder + 1) as u8;
            }
            return Ok(checked);
        }
        // R58_BINOMIAL_FROBENIUS_REPLAY_END
        // Evaluate x^p at the unique root of the linear modulus using
        // Frobenius on Fp; do not call the producer's arithmetic.
        if monic.len == 2 && exponent == self.p
            && base.len == 2 && base.coefficients[0] == 0
            && base.coefficients[1] == 1
        {
            return Ok(Poly17::constant(self.minus(0, monic.coefficients[0])?));
        }
        // R45_QUADRATIC_FROBENIUS_REPLAY_BEGIN
        // Verify quadratic Frobenius using a separate left-to-right
        // Euler-criterion implementation. Do not invoke producer code.
        if monic.len == 3 && exponent == self.p
            && base.len == 2 && base.coefficients[0] == 0
            && base.coefficients[1] == 1
        {
            let linear = monic.coefficients[1];
            let twice = self.plus(monic.coefficients[0], monic.coefficients[0])?;
            let four = self.plus(twice, twice)?;
            let square = self.times(linear, linear)?;
            let discriminant = self.minus(square, four)?;
            if discriminant == 0 {
                let root = self.times(linear, (self.p + 1) / 2)?;
                return Ok(Poly17::constant(self.minus(0, root)?));
            }
            // R49_FROBENIUS_REPLAY_BEGIN
            let symbol = self.quadratic_character(discriminant)?;
            // R49_FROBENIUS_REPLAY_END
            if symbol == 1 {
                return Ok(base);
            }
            if symbol == -1 {
                let mut result = Poly17::ZERO;
                result.len = 2;
                result.coefficients[0] = self.minus(0, linear)?;
                result.coefficients[1] = self.minus(0, 1)?;
                return Ok(result);
            }
            return Err(Stop::InvalidCertificate);
        }
        // R45_QUADRATIC_FROBENIUS_REPLAY_END
        let mut base = base;
        self.cancel_monic(&mut base, &monic)?;
        // R42_REPLAY_FASTPATH_BEGIN
        // Replay uses a distinct, iterative addition chain and its own
        // polynomial arithmetic; certificate contents remain unchanged.
        if self.p == (1u64 << 61) - 1
            && (exponent == self.p || exponent == (self.p - 1) / 2)
        {
            return self.repeated_mersenne_bits_power(
                &base, 64 - exponent.leading_zeros(), &monic,
            );
        }
        // R42_REPLAY_FASTPATH_END
        // Keep the former loop for large (61-bit) exponents: native and
        // local A/B shows no reproducible gain from its first-bit shortcut.
        if exponent > u32::MAX as u64 {
            let mut acc = Poly17::constant(1);
            self.cancel_monic(&mut acc, &monic)?;
            for bit in (0..64 - exponent.leading_zeros()).rev() {
                self.gate.charge(4 * COPY_WORK)?;
                let mut square = self.square_diagonals(&acc)?;
                self.cancel_monic(&mut square, &monic)?;
                acc = square;
                if exponent & (1u64 << bit) != 0 {
                    let mut product = self.convolution(&acc, &base)?;
                    self.cancel_monic(&mut product, &monic)?;
                    acc = product;
                }
            }
            return Ok(acc);
        }
        // The top exponent bit is always 1. Initialize from the already
        // reduced base instead of squaring 1 and multiplying by base.
        let mut acc = if exponent == 0 { Poly17::constant(1) } else { base };
        if exponent == 0 && monic.len == 1 {
            self.cancel_monic(&mut acc, &monic)?;
        }
        let remaining_bits = (64 - exponent.leading_zeros()).saturating_sub(1);
        for bit in (0..remaining_bits).rev() {
            self.gate.charge(4 * COPY_WORK)?;
            let mut square = self.square_diagonals(&acc)?;
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
            // The independent replay can prove coprimality immediately when
            // Euclid reaches a nonzero constant; do not invert a unit merely
            // to normalize it after another remainder pass.
            if pair[1].len == 1 {
                return Ok(Poly17::constant(1));
            }
            let next = self.remainder(pair[0], &pair[1])?;
            pair = [pair[1], next];
        }
        let mut out = pair[0];
        // Many GCD inputs and constant-ending Euclidean sequences are
        // already monic. A monic result is normalized without field work.
        if out.len > 0 && out.coefficients[usize::from(out.len) - 1] != 1 {
            let inv = self.reciprocal(out.coefficients[usize::from(out.len) - 1])?;
            for i in 0..usize::from(out.len) {
                if out.coefficients[i] != 0 {
                    out.coefficients[i] = self.times(out.coefficients[i], inv)?;
                }
            }
        }
        Ok(out)
    }
    // R49_BINARY_JACOBI_REPLAY_BEGIN
    // Independent binary Jacobi implementation for certificate checks.
    // The verifier still checks the quadratic roots by substitution.
    fn quadratic_character(&mut self, mut value: u64) -> Result<i8> {
        let mut odd_modulus = self.p;
        let mut parity = false;
        self.gate.charge(32)?;
        while value > 0 {
            self.gate.charge(32)?;
            let factors_of_two = value.trailing_zeros();
            value >>= factors_of_two;
            parity ^= (factors_of_two & 1 != 0)
                && (odd_modulus & 7 == 3 || odd_modulus & 7 == 5);
            parity ^= (value & odd_modulus & 3) == 3;
            let remainder = odd_modulus % value;
            odd_modulus = value;
            value = remainder;
        }
        Ok(if odd_modulus != 1 { 0 } else if parity { -1 } else { 1 })
    }
    // R49_BINARY_JACOBI_REPLAY_END
    // R46_REPLAY_HELPER_BEGIN
    // Certificate replay evaluates quadratic split characters independently,
    // with left-to-right scalar powering and separate field arithmetic.
    fn split_character_power(&mut self, x: u64, exponent: u64) -> Result<u64> {
        // R61_REPLAY_SCALAR_IDENTITY_BEGIN
        // The top set bit contributes x, not (1^2)*x. Replay uses the
        // independently implemented left-to-right square/multiply order.
        if exponent == 1 {
            self.gate.clock()?;
            return Ok(x);
        }
        let mut acc = if exponent == 0 { 1u64 } else { x };
        let remaining = (64 - exponent.leading_zeros()).saturating_sub(1);
        for bit in (0..remaining).rev() {
            acc = self.times(acc, acc)?;
            if exponent & (1u64 << bit) != 0 {
                acc = self.times(acc, x)?;
            }
        }
        Ok(acc)
        // R61_REPLAY_SCALAR_IDENTITY_END
    }
    fn replay_quadratic_split(&mut self, f: Poly17, delta: u64) -> Result<Option<Poly17>> {
        if f.len != 3 || (self.p % 4 != 3 && self.p % 8 != 5 && self.p != 1_000_000_009) || f.coefficients[2] != 1 {
            return Ok(None);
        }
        let b = f.coefficients[1];
        let c = f.coefficients[0];
        let twice = self.plus(c, c)?;
        let quadruple = self.plus(twice, twice)?;
        let squared_b = self.times(b, b)?;
        let discriminant = self.minus(squared_b, quadruple)?;
        if discriminant == 0 {
            return Ok(None);
        }
        // R47_SQRT_REPLAY_BEGIN
        // Independent verifier: left-to-right scalar powers and a separately
        // structured Tonelli-Shanks loop. Validate the candidate by squaring.
        let sqrt = if self.p % 4 == 3 {
            self.split_character_power(discriminant, (self.p + 1) / 4)?
        } else if self.p % 8 == 5 {
            let mut value = self.split_character_power(discriminant, (self.p + 3) / 8)?;
            if self.times(value, value)? != discriminant {
                let correction = self.split_character_power(2, (self.p - 1) / 4)?;
                value = self.times(value, correction)?;
            }
            value
        } else {
            debug_assert_eq!(self.p, 1_000_000_009);
            // R48_TONELLI_REPLAY_BEGIN
            // The independently implemented verifier uses its own field
            // operations and a bit-indexed, low-to-high shared ladder.
            // The fixed nonresidue power is checked against a wide oracle
            // in the round-48 regression test.
            let odd = (self.p - 1) >> 3;
            let root_exponent = (odd + 1) / 2;
            let mut root = 1u64;
            let mut residue = 1u64;
            let mut factor = discriminant;
            let bits = 64 - odd.leading_zeros();
            for bit in 0..bits {
                let mask = 1u64 << bit;
                if odd & mask != 0 {
                    residue = self.times(residue, factor)?;
                }
                if root_exponent & mask != 0 {
                    root = self.times(root, factor)?;
                }
                if bit + 1 < bits {
                    factor = self.times(factor, factor)?;
                }
            }
            let mut nonresidue = 172_252_358u64;
            // R48_TONELLI_REPLAY_END
            let mut order = 3u32;
            while residue != 1 {
                let mut order_test = residue;
                let mut count = 0u32;
                while order_test != 1 && count < order {
                    order_test = self.times(order_test, order_test)?;
                    count += 1;
                }
                if count == order { return Ok(None); }
                let mut multiplier = nonresidue;
                for _ in 0..order - count - 1 {
                    multiplier = self.times(multiplier, multiplier)?;
                }
                root = self.times(root, multiplier)?;
                let multiplier_sq = self.times(multiplier, multiplier)?;
                residue = self.times(residue, multiplier_sq)?;
                nonresidue = multiplier_sq;
                order = count;
            }
            root
        };
        // R47_SQRT_REPLAY_END
        if self.times(sqrt, sqrt)? != discriminant {
            return Ok(None);
        }
        // R50_QUADRATIC_FINISH_REPLAY_BEGIN
        // Independently derive half(a) by parity, then recover the
        // complementary quadratic root using the sum of roots. Both
        // roots are still explicitly substituted into f below.
        let negative = self.minus(0, b)?;
        let numerator0 = self.plus(negative, sqrt)?;
        self.gate.charge(32)?;
        let r0 = if numerator0 & 1 == 0 {
            numerator0 / 2
        } else {
            (numerator0 / 2) + (self.p / 2) + 1
        };
        let r1 = self.minus(negative, r0)?;
        // R50_QUADRATIC_FINISH_REPLAY_END
        if r0 == r1 {
            return Ok(None);
        }
        // Independently verify both roots satisfy the actual supplied factor,
        // before using their quadratic characters for the claimed split.
        for root in [r0, r1] {
            let square = self.times(root, root)?;
            let linear = self.times(b, root)?;
            let sum = self.plus(square, linear)?;
            if self.plus(sum, c)? != 0 {
                return Err(Stop::InvalidCertificate);
            }
        }
        let shifted0 = self.plus(r0, delta)?;
        let shifted1 = self.plus(r1, delta)?;
        // R49_SPLIT_REPLAY_BEGIN
        let s0 = self.quadratic_character(shifted0)? == 1;
        let s1 = self.quadratic_character(shifted1)? == 1;
        // R49_SPLIT_REPLAY_END
        if s0 && s1 {
            return Ok(Some(f));
        }
        if !s0 && !s1 {
            return Ok(Some(Poly17::constant(1)));
        }
        let mut factor = Poly17::ZERO;
        factor.len = 2;
        factor.coefficients[0] = self.minus(0, if s0 { r0 } else { r1 })?;
        factor.coefficients[1] = 1;
        Ok(Some(factor))
    }
    // R46_REPLAY_HELPER_END

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
                // Independently verify the linear/constant Frobenius cases
                // without invoking general polynomial Euclid.
                let g = if input.len == 1 {
                    Poly17::constant(1)
                } else if input.len == 2 {
                    self.normalized_modulus(&input)?
                // R45_QUADRATIC_GCD_REPLAY_BEGIN
                } else if input.len == 3 && expected.len == 2
                    && expected.coefficients[0] == 0 && expected.coefficients[1] == 1
                {
                    self.normalized_modulus(&input)?
                } else if input.len == 3 && expected.len == 2
                    && expected.coefficients[1] == self.p - 1
                {
                    Poly17::constant(1)
                } else if input.len == 3 && expected.len <= 1 {
                    let mut linear = Poly17::ZERO;
                    linear.len = 2;
                    linear.coefficients[0] = self.minus(0, expected.coefficients[0])?;
                    linear.coefficients[1] = 1;
                    linear
                // R45_QUADRATIC_GCD_REPLAY_END
                } else {
                    self.monic_gcd(input, xm)?
                };
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
                // R46_REPLAY_SPLIT_BEGIN
                let expected = if let Some(factor) = self.replay_quadratic_split(polynomial, delta)? {
                    factor
                } else {
                    let mut value = self.modular_power(base, (self.p - 1) / 2, &polynomial)?;
                    value.len = value.len.max(1);
                    value.coefficients[0] = self.minus(value.coefficients[0], 1)?;
                    value.trim();
                    self.monic_gcd(polynomial, value)?
                };
                // R46_REPLAY_SPLIT_END
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
                let v = if polynomial.coefficients[1] == 1 {
                    root
                } else {
                    self.times(polynomial.coefficients[1], root)?
                };
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

    #[test]
    fn round62_unit_coefficients_and_binomial_squares_match_wide_oracle() {
        // Full 128-bit coefficient oracle is independent of producer/replay.
        // Include zero, +/-1 and dense controls at both 32- and 61-bit primes.
        fn oracle(a: &Poly17, b: &Poly17, p: u64) -> Poly17 {
            let mut coefficients = [0u64; 17];
            if a.len == 0 || b.len == 0 { return Poly17::ZERO; }
            let n = (a.len + b.len - 1) as usize;
            for i in 0..a.len as usize {
                for j in 0..b.len as usize {
                    coefficients[i + j] =
                        ((coefficients[i + j] as u128
                          + a.coefficients[i] as u128 * b.coefficients[j] as u128)
                         % p as u128) as u64;
                }
            }
            let mut result = Poly17 {
                coefficients,
                len: n as u8,
            };
            result.trim();
            result
        }
        let mut comparisons = 0usize;
        for p in [3u64, 5, 7, 101, 1_000_000_007, 1_000_000_009,
                  (1u64 << 61) - 1] {
            let f = field(p);
            let mut observer = |_: &BoundedRootEvent| Ok(());
            let mut producer = Producer {
                p, gate: Gate::new(budget(), deadline()), observer: &mut observer,
                unacknowledged: None, pending: [Poly17::ZERO; 8], pending_len: 0,
                roots: [0; 8], root_count: 0,
            };
            producer.gate.admit().unwrap();
            let mut replay =
                Replay::new(&f, &[1, 1], 42, budget(), budget(), deadline()).unwrap();
            for degree in [2usize, 3, 5, 8] {
                for seed in 0..64u64 {
                    let mut values = vec![0u64; degree];
                    let mut state = seed ^ p.wrapping_mul(0x9e3779b97f4a7c15);
                    for (i, coefficient) in values.iter_mut().enumerate() {
                        state = state.wrapping_mul(6364136223846793005)
                            .wrapping_add(1442695040888963407);
                        *coefficient = if seed % 5 == 0 && i % 2 == 0 {
                            0
                        } else {
                            state % p
                        };
                    }
                    values[degree - 1] = 1;
                    let polynomial = Poly17::from_coefficients(&f, &values).unwrap();
                    for factor in [
                        Poly17::from_coefficients(&f, &[p - 1]).unwrap(),
                        Poly17::from_coefficients(&f, &[1, 1]).unwrap(),
                        Poly17::from_coefficients(&f, &[p - 1, 1]).unwrap(),
                        Poly17::from_coefficients(&f, &[2 % p, 1]).unwrap(),
                    ] {
                        let expected = oracle(&polynomial, &factor, p);
                        assert_eq!(producer.product(&polynomial, &factor).unwrap(), expected);
                        assert_eq!(replay.convolution(&polynomial, &factor).unwrap(), expected);
                        comparisons += 2;
                    }
                    producer.gate.metrics.work = 0;
                    producer.gate.metrics.field_work = 0;
                    replay.gate.metrics.work = 0;
                    replay.gate.metrics.field_work = 0;
                }
                for c in [0, 1, p - 1, 2 % p, 17 % p] {
                    let mut coefficients = vec![0u64; degree + 1];
                    coefficients[0] = c;
                    coefficients[degree] = 1;
                    let polynomial = Poly17::from_coefficients(&f, &coefficients).unwrap();
                    let expected = oracle(&polynomial, &polynomial, p);
                    assert_eq!(producer.square(&polynomial).unwrap(), expected);
                    assert_eq!(replay.square_diagonals(&polynomial).unwrap(), expected);
                    comparisons += 2;
                    producer.gate.metrics.work = 0;
                    producer.gate.metrics.field_work = 0;
                    replay.gate.metrics.work = 0;
                    replay.gate.metrics.field_work = 0;
                }
            }
        }
        println!("ROUND62_ORACLE comparisons={comparisons}");
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
    fn round60_unit_linear_division_reduction_and_independent_replay() {
        // A polynomial modulo x+/-1 can be canceled with one field add/sub
        // per nonzero high coefficient. Compare all four implementations
        // against a separate wide-integer synthetic-division oracle.
        for p in [3u64, 5, 7, 101, 1_000_000_007, 1_000_000_009,
                  (1u64 << 61) - 1] {
            let f = field(p);
            let mut observer = |_: &BoundedRootEvent| Ok(());
            let mut producer = Producer {
                p, gate: Gate::new(budget(), deadline()), observer: &mut observer,
                unacknowledged: None, pending: [Poly17::ZERO; 8], pending_len: 0,
                roots: [0; 8], root_count: 0,
            };
            producer.gate.admit().unwrap();
            let mut replay =
                Replay::new(&f, &[1, 1], 42, budget(), budget(), deadline()).unwrap();
            for c in [1u64, p - 1] {
                let modulus = Poly17::from_coefficients(&f, &[c, 1]).unwrap();
                for n in [2usize, 3, 8, 17] {
                    for seed in 0..64u64 {
                        let mut input = vec![0u64; n];
                        let mut state = seed.wrapping_mul(0x9E3779B97F4A7C15);
                        for coefficient in &mut input {
                            state = state.wrapping_mul(6364136223846793005)
                                .wrapping_add(1442695040888963407);
                            *coefficient = state % p;
                        }
                        if seed & 3 == 0 { input[0] = 0; }
                        input[n - 1] = 1;
                        let numerator = Poly17::from_coefficients(&f, &input).unwrap();

                        let mut remainder = input.clone();
                        let mut quotient = vec![0u64; n - 1];
                        for high in (1..n).rev() {
                            let factor = remainder[high];
                            quotient[high - 1] = factor;
                            let current = remainder[high - 1];
                            let next = if c == 1 {
                                ((current as u128 + p as u128 - factor as u128)
                                    % p as u128) as u64
                            } else {
                                ((current as u128 + factor as u128) % p as u128) as u64
                            };
                            remainder[high - 1] = next;
                            remainder[high] = 0;
                        }
                        let expected_q =
                            Poly17::from_coefficients(&f, &quotient).unwrap();
                        let expected_r =
                            Poly17::from_coefficients(&f, &remainder[..1]).unwrap();
                        let (q, r) = producer.division(&numerator, &modulus).unwrap();
                        assert_eq!((q, r), (expected_q, expected_r));
                        let mut reduced = numerator;
                        producer.reduce_monic(&mut reduced, &modulus).unwrap();
                        assert_eq!(reduced, expected_r);
                        assert_eq!(replay.remainder(numerator, &modulus).unwrap(), expected_r);
                        let mut checked = numerator;
                        replay.cancel_monic(&mut checked, &modulus).unwrap();
                        assert_eq!(checked, expected_r);
                        producer.gate.metrics.work = 0;
                        producer.gate.metrics.field_work = 0;
                        replay.gate.metrics.work = 0;
                        replay.gate.metrics.field_work = 0;
                    }
                }
            }
        }
    }

    #[test]
    fn round59_binomial_folds_match_scalar_oracle_and_independent_replay() {
        // Oracle computes the coefficient reduction directly using u128.
        // Exercise all four affected paths, including the non-specialized
        // c=17 fallback, on full-capacity degree-8 polynomials.
        for p in [3u64, 5, 7, 101, 1_000_000_007, 1_000_000_009,
                  (1u64 << 61) - 1] {
            let f = field(p);
            let mut observer = |_: &BoundedRootEvent| Ok(());
            let mut producer = Producer {
                p, gate: Gate::new(budget(), deadline()), observer: &mut observer,
                unacknowledged: None, pending: [Poly17::ZERO; 8], pending_len: 0,
                roots: [0; 8], root_count: 0,
            };
            producer.gate.admit().unwrap();
            let mut replay =
                Replay::new(&f, &[1, 1], 42, budget(), budget(), deadline()).unwrap();
            for degree in [2usize, 3, 4, 6, 8] {
                for c0 in [1u64, p - 1, 17 % p] {
                    let c = if c0 == 0 { 1 } else { c0 };
                    let mut divisor = vec![0; degree + 1];
                    divisor[0] = c;
                    divisor[degree] = 1;
                    let modulus = Poly17::from_coefficients(&f, &divisor).unwrap();
                    let n = (2 * degree + 1).min(17);
                    for seed in 0..32u64 {
                        let mut input = vec![0; n];
                        let mut state = seed.wrapping_mul(0x9E3779B97F4A7C15);
                        for coefficient in &mut input {
                            state = state.wrapping_mul(6364136223846793005)
                                .wrapping_add(1442695040888963407);
                            *coefficient = state % p;
                        }
                        input[n - 1] = 1;
                        let numerator = Poly17::from_coefficients(&f, &input).unwrap();
                        let mut oracle = input;
                        let mut quotient = vec![0u64; n - degree];
                        for high in (degree..n).rev() {
                            let factor = oracle[high];
                            quotient[high - degree] = factor;
                            let sub = ((factor as u128 * c as u128) % p as u128) as u64;
                            let low = high - degree;
                            oracle[low] = ((oracle[low] as u128 + p as u128
                                - sub as u128) % p as u128) as u64;
                            oracle[high] = 0;
                        }
                        let expected_r = Poly17::from_coefficients(&f, &oracle[..degree]).unwrap();
                        let expected_q = Poly17::from_coefficients(&f, &quotient).unwrap();

                        let (q, r) = producer.division(&numerator, &modulus).unwrap();
                        assert_eq!((q, r), (expected_q, expected_r));
                        let mut reduced = numerator;
                        producer.reduce_monic(&mut reduced, &modulus).unwrap();
                        assert_eq!(reduced, expected_r);
                        let replay_r = replay.remainder(numerator, &modulus).unwrap();
                        assert_eq!(replay_r, expected_r);
                        let mut replay_reduced = numerator;
                        replay.cancel_monic(&mut replay_reduced, &modulus).unwrap();
                        assert_eq!(replay_reduced, expected_r);
                        producer.gate.metrics.work = 0;
                        producer.gate.metrics.field_work = 0;
                        replay.gate.metrics.work = 0;
                        replay.gate.metrics.field_work = 0;
                    }
                }
            }
        }
    }

    #[test]
    fn round58_binomial_frobenius_matches_independent_oracle_and_replay() {
        // Oracle uses only u128 scalar modular arithmetic and the quotient
        // relation x^d=-c; it never invokes either production root algorithm.
        fn oracle_power(mut a: u64, mut n: u64, p: u64) -> u64 {
            let mut result = 1u64;
            while n > 0 {
                if n & 1 != 0 {
                    result = ((result as u128 * a as u128) % p as u128) as u64;
                }
                n >>= 1;
                a = ((a as u128 * a as u128) % p as u128) as u64;
            }
            result
        }
        for p in [3u64, 5, 7, 101, 1_000_000_007, 1_000_000_009,
                  (1u64 << 61) - 1] {
            let f = field(p);
            let mut observer = |_: &BoundedRootEvent| Ok(());
            let mut producer = Producer {
                p, gate: Gate::new(budget(), deadline()), observer: &mut observer,
                unacknowledged: None, pending: [Poly17::ZERO; 8], pending_len: 0,
                roots: [0; 8], root_count: 0,
            };
            producer.gate.admit().unwrap();
            let mut replay =
                Replay::new(&f, &[1, 1], 42, budget(), budget(), deadline()).unwrap();
            for degree in 3..=8usize {
                for leading in [1u64, p - 1] {
                    for constant in [1u64, 2 % p, p - 1, 17 % p] {
                        let mut coefficients = vec![0u64; degree + 1];
                        coefficients[0] =
                            ((constant as u128 * leading as u128) % p as u128) as u64;
                        coefficients[degree] = leading;
                        let modulus =
                            Poly17::from_coefficients(&f, &coefficients).unwrap();
                        for delta in [0u64, 1, p - 1, 23 % p] {
                            let base = Poly17::from_coefficients(&f, &[delta, 1]).unwrap();
                            let q = p / degree as u64;
                            let r = (p % degree as u64) as usize;
                            let neg_c = if constant == 0 { 0 } else { p - constant };
                            let scaled = oracle_power(neg_c, q, p);
                            let mut expected = vec![0u64; degree];
                            expected[0] = delta;
                            if r == 0 {
                                expected[0] =
                                    ((delta as u128 + scaled as u128) % p as u128) as u64;
                            } else {
                                expected[r] = scaled;
                            }
                            let expected =
                                Poly17::from_coefficients(&f, &expected).unwrap();
                            assert_eq!(producer.power(&base, p, &modulus).unwrap(), expected,
                                "producer p={p} degree={degree} c={constant} delta={delta}");
                            assert_eq!(replay.modular_power(base, p, &modulus).unwrap(), expected,
                                "replay p={p} degree={degree} c={constant} delta={delta}");
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn round57_monomial_frobenius_matches_independent_oracle_and_replay() {
        // A polynomial in Fp[x]/(x^d) is stored canonically. The oracle
        // constructs the reduced x^p + delta directly, never invoking
        // production polynomial multiplication or powering.
        for p in [3u64, 5, 7, 1_000_000_007, 1_000_000_009,
                  (1u64 << 61) - 1] {
            let f = field(p);
            let mut observer = |_: &BoundedRootEvent| Ok(());
            let mut producer = Producer {
                p, gate: Gate::new(budget(), deadline()), observer: &mut observer,
                unacknowledged: None, pending: [Poly17::ZERO; 8], pending_len: 0,
                roots: [0; 8], root_count: 0,
            };
            producer.gate.admit().unwrap();
            let mut replay =
                Replay::new(&f, &[1, 1], 42, budget(), budget(), deadline()).unwrap();
            for degree in 2..=8usize {
                for leading in [1u64, p - 1] {
                    let mut modulus_coefficients = vec![0u64; degree + 1];
                    modulus_coefficients[degree] = leading;
                    let modulus =
                        Poly17::from_coefficients(&f, &modulus_coefficients).unwrap();
                    for delta in [0u64, 1, p - 1, 17 % p] {
                        let base = Poly17::from_coefficients(&f, &[delta, 1]).unwrap();
                        let mut coefficients = vec![0u64; degree];
                        coefficients[0] = delta;
                        if p < degree as u64 {
                            coefficients[p as usize] = 1;
                        }
                        let expected =
                            Poly17::from_coefficients(&f, &coefficients).unwrap();
                        assert_eq!(producer.power(&base, p, &modulus).unwrap(), expected,
                            "producer p={p} d={degree} delta={delta}");
                        assert_eq!(replay.modular_power(base, p, &modulus).unwrap(), expected,
                            "replay p={p} d={degree} delta={delta}");
                    }
                }
            }
        }
    }

    #[test]
    fn round57_monomial_root_stream_matches_replay() {
        for p in [3u64, 7, 1_000_000_007, (1u64 << 61) - 1] {
            for degree in 2..=8usize {
                for leading in [1u64, p - 1] {
                    let mut coefficients = vec![0u64; degree + 1];
                    coefficients[degree] = leading;
                    let (report, events) = run(&coefficients, p, budget());
                    assert_eq!(report.stop, Stop::Complete, "p={p} d={degree}");
                    assert_eq!(&report.roots[..report.root_count as usize], &[0u64]);
                    assert!(check(&coefficients, &report, &events).unwrap().finite_complete,
                        "p={p} d={degree}");
                }
            }
        }
    }

    #[test]
    fn round56_monomial_quotients_remainders_and_replay_match_oracle() {
        // The independent oracle splits coefficients at degree d rather
        // than invoking polynomial division, modular reduction or replay.
        for p in [7u64, 1_000_000_007, 1_000_000_009, (1u64 << 61) - 1] {
            let f = field(p);
            for d in 2..=8usize {
                let mut modulus_coeffs = vec![0u64; d + 1];
                modulus_coeffs[d] = 1;
                let modulus = Poly17::from_coefficients(&f, &modulus_coeffs).unwrap();
                let mut observer = |_: &BoundedRootEvent| Ok(());
                let mut producer = Producer {
                    p, gate: Gate::new(budget(), deadline()), observer: &mut observer,
                    unacknowledged: None, pending: [Poly17::ZERO; 8], pending_len: 0,
                    roots: [0; 8], root_count: 0,
                };
                producer.gate.admit().unwrap();
                let mut replay =
                    Replay::new(&f, &[1, 1], 42, budget(), budget(), deadline()).unwrap();
                let mut state = (p ^ 0xabcd_1234_d00d_9911).wrapping_add(d as u64);
                for n in [d - 1, d, d + 1, (d + 3).min(16), 16] {
                    for case in 0..32u64 {
                        let mut coefficients = vec![0u64; n];
                        for (i, coefficient) in coefficients.iter_mut().enumerate() {
                            state ^= state << 13;
                            state ^= state >> 7;
                            state ^= state << 17;
                            *coefficient = if case % 7 == 0 && i % 2 == 0 {
                                0
                            } else {
                                state % p
                            };
                        }
                        coefficients[n - 1] = 1 + (state % (p - 1));
                        let polynomial = Poly17::from_coefficients(&f, &coefficients).unwrap();
                        let expected_remainder =
                            Poly17::from_coefficients(&f, &coefficients[..n.min(d)]).unwrap();
                        let expected_quotient = if n <= d {
                            Poly17::ZERO
                        } else {
                            Poly17::from_coefficients(&f, &coefficients[d..]).unwrap()
                        };
                        let (q, r) = producer.division(&polynomial, &modulus).unwrap();
                        assert_eq!(q, expected_quotient, "division quotient p={p} d={d} n={n}");
                        assert_eq!(r, expected_remainder, "division remainder p={p} d={d} n={n}");
                        let mut reduced = polynomial;
                        producer.reduce_monic(&mut reduced, &modulus).unwrap();
                        assert_eq!(reduced, expected_remainder, "producer p={p} d={d} n={n}");
                        let replay_r = replay.remainder(polynomial, &modulus).unwrap();
                        assert_eq!(replay_r, expected_remainder, "replay remainder p={p} d={d}");
                        let mut replay_c = polynomial;
                        replay.cancel_monic(&mut replay_c, &modulus).unwrap();
                        assert_eq!(replay_c, expected_remainder, "replay cancel p={p} d={d}");
                    }
                }
            }
        }
    }

    #[test]
    fn round53_inverses_match_independent_modular_power_and_deadlines() {
        // Independent square-and-multiply oracle; neither inversion routine
        // uses this code path in production. Include edge cases and randoms.
        fn reference(mut base: u64, p: u64) -> u64 {
            let mut exponent = p - 2;
            let mut accumulator = 1u64;
            while exponent > 0 {
                if exponent & 1 == 1 {
                    accumulator =
                        ((u128::from(accumulator) * u128::from(base)) % u128::from(p)) as u64;
                }
                exponent >>= 1;
                if exponent != 0 {
                    base = ((u128::from(base) * u128::from(base)) % u128::from(p)) as u64;
                }
            }
            accumulator
        }
        for p in [5u64, 7, 17, 1_000_000_007, 1_000_000_009,
                  1_000_000_021, (1u64 << 61) - 1] {
            let f = field(p);
            let mut observer = |_: &BoundedRootEvent| Ok(());
            let mut producer = Producer {
                p, gate: Gate::new(budget(), deadline()), observer: &mut observer,
                unacknowledged: None, pending: [Poly17::ZERO; 8], pending_len: 0,
                roots: [0; 8], root_count: 0,
            };
            producer.gate.admit().unwrap();
            let mut replay = Replay::new(&f, &[1, 1], 42, budget(), budget(), deadline()).unwrap();
            assert_eq!(producer.inverse(0), Err(Stop::InvalidInput));
            assert_eq!(replay.reciprocal(0), Err(Stop::InvalidCertificate));
            let mut state = p ^ 0x1a21_55b1_d1e5_1aef;
            for i in 0..512u64 {
                state ^= state << 13;
                state ^= state >> 7;
                state ^= state << 17;
                let a = match i {
                    0 => 1,
                    1 => p - 1,
                    2 => 2,
                    _ => (state % (p - 1)) + 1,
                };
                let expected = reference(a, p);
                let actual = producer.inverse(a).unwrap();
                let checked = replay.reciprocal(a).unwrap();
                assert_eq!(actual, expected, "producer p={p} a={a}");
                assert_eq!(checked, expected, "replay p={p} a={a}");
                assert_eq!(
                    (u128::from(a) * u128::from(actual)) % u128::from(p),
                    1, "product p={p} a={a}"
                );
            }
            assert!(producer.gate.metrics.field_work <= budget().field_work);
            assert!(replay.gate.metrics.field_work <= budget().field_work);
        }
    }

    #[test]
    fn round51_scalar_and_monic_linear_products_match_wide_reference() {
        for p in [3u64, 7, 1_000_000_007, 1_000_000_009,
                  4_294_967_291, (1u64 << 61) - 1] {
            let f = field(p);
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
            let mut replay =
                Replay::new(&f, &[1, 1], 42, budget(), budget(), deadline()).unwrap();
            let mut seed = 0x2026_1120_5151_0001u64;
            let mut comparisons = 0usize;
            for size in [2usize, 4, 8] {
                for _ in 0..16 {
                    let mut coefficients = [0u64; 8];
                    for value in coefficients.iter_mut().take(size) {
                        seed = seed.wrapping_mul(6364136223846793005)
                            .wrapping_add(1442695040888963407);
                        *value = (seed ^ (seed >> 29)) % p;
                    }
                    coefficients[size - 1] = 1 + (seed % (p - 1));
                    let poly =
                        Poly17::from_coefficients(&f, &coefficients[..size]).unwrap();
                    for linear in [
                        vec![0u64, 1], vec![1, 1], vec![p - 1, 1],
                        vec![2 % p, 1], vec![1], vec![p - 1],
                    ] {
                        let factor = Poly17::from_coefficients(&f, &linear).unwrap();
                        let mut wide = [0u128; 17];
                        for (i, &a) in poly.coefficients().iter().enumerate() {
                            for (j, &b) in factor.coefficients().iter().enumerate() {
                                wide[i + j] += u128::from(a) * u128::from(b);
                            }
                        }
                        let n = size + factor.len as usize - 1;
                        let expected_coefficients: Vec<u64> = wide[..n].iter()
                            .map(|&v| (v % u128::from(p)) as u64).collect();
                        let expected =
                            Poly17::from_coefficients(&f, &expected_coefficients).unwrap();
                        assert_eq!(producer.product(&poly, &factor).unwrap(), expected);
                        assert_eq!(producer.product(&factor, &poly).unwrap(), expected);
                        assert_eq!(replay.convolution(&poly, &factor).unwrap(), expected);
                        assert_eq!(replay.convolution(&factor, &poly).unwrap(), expected);
                        comparisons += 4;
                    }
                }
            }
            assert_eq!(comparisons, 1_152, "p={p}");
        }
    }
    // Compare both independently implemented consumers with the exact wide
    // modulo reference, including carries, zero, p-1 and random inputs.
    #[test]
    fn round31_bounded_canonical_ops_match_wide_reference() {
        for p in [3u64, 7, 17, 487, 17_497, 1_000_000_007,
                  4_294_967_291, 140_737_488_355_213, (1u64 << 61) - 1] {
            let field = field(p);
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
            let mut replay =
                Replay::new(&field, &[1, 1], 42, budget(), budget(), deadline()).unwrap();
            let mut seed = 0x2026_1119_3131_0001u64;
            let mut comparisons = 0usize;
            for index in 0..2_048 {
                seed = seed.wrapping_mul(6364136223846793005)
                    .wrapping_add(1442695040888963407);
                let mut x = (seed ^ (seed >> 29)) % p;
                seed = seed.wrapping_mul(6364136223846793005)
                    .wrapping_add(1442695040888963407);
                let mut y = (seed ^ (seed >> 29)) % p;
                match index % 37 {
                    0 => { x = p - 1; y = p - 1; }
                    1 => { x = 0; y = p - 1; }
                    2 => { x = p - 1; y = 0; }
                    3 => { x = 0; y = 0; }
                    _ => {}
                }
                let expected_add = ((x as u128 + y as u128) % p as u128) as u64;
                let expected_sub = ((x as u128 + p as u128 - y as u128) % p as u128) as u64;
                let expected_mul = ((x as u128 * y as u128) % p as u128) as u64;
                assert_eq!(producer.add(x, y).unwrap(), expected_add, "producer add p={p}");
                assert_eq!(producer.sub(x, y).unwrap(), expected_sub, "producer sub p={p}");
                assert_eq!(producer.mul(x, y).unwrap(), expected_mul, "producer mul p={p}");
                assert_eq!(replay.plus(x, y).unwrap(), expected_add, "replay plus p={p}");
                assert_eq!(replay.minus(x, y).unwrap(), expected_sub, "replay minus p={p}");
                assert_eq!(replay.times(x, y).unwrap(), expected_mul, "replay times p={p}");
                comparisons += 6;
            }
            assert_eq!(comparisons, 12_288);
        }
    }

    // This oracle uses u128 arithmetic only; the producer and the replay
    // independently compute the binary Jacobi symbol without field powers.
    #[test]
    fn round49_binary_jacobi_matches_fermat_for_prime_fields() {
        let mut checked = 0usize;
        for p in [3u64, 7, 487, 1_000_000_007, 1_000_000_009,
                  1_000_000_021, (1u64 << 61) - 1] {
            let f = field(p);
            let mut observer = |_: &BoundedRootEvent| Ok(());
            let mut prod = Producer {
                p, gate: Gate::new(budget(), deadline()), observer: &mut observer,
                unacknowledged: None, pending: [Poly17::ZERO; 8],
                pending_len: 0, roots: [0; 8], root_count: 0,
            };
            prod.gate.admit().unwrap();
            let mut replay = Replay::new(&f, &[1, 1], 42, budget(), budget(), deadline()).unwrap();
            let mut rng = 0x2026_1119_4949_0001u64;
            for j in 0..512 {
                rng = rng.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
                let v = match j % 29 {
                    0 => 0, 1 => 1, 2 => p - 1, _ => rng % p,
                };
                let mut e = (p - 1) / 2;
                let mut base = v as u128;
                let mut acc = 1u128;
                while e > 0 {
                    if e & 1 != 0 { acc = acc * base % p as u128; }
                    e >>= 1;
                    if e > 0 { base = base * base % p as u128; }
                }
                let expected = if acc == 0 { 0 } else if acc == 1 { 1 }
                    else { assert_eq!(acc, (p - 1) as u128); -1 };
                assert_eq!(prod.quadratic_character(v).unwrap(), expected,
                    "producer p={p} v={v}");
                assert_eq!(replay.quadratic_character(v).unwrap(), expected,
                    "replay p={p} v={v}");
                checked += 2;
            }
        }
        assert_eq!(checked, 7_168);
    }
    #[test]
    fn round48_joint_tonelli_nonresidue_and_certificate_cases() {
        let p = 1_000_000_009u64;
        let q = (p - 1) / 8;
        // Oracle intentionally uses a plain u128 remainder, not the
        // producer or verifier's field multiplication implementations.
        let mut power = q;
        let mut base = 11u128;
        let mut value = 1u128;
        while power != 0 {
            if power & 1 != 0 {
                value = value * base % p as u128;
            }
            power >>= 1;
            if power != 0 {
                base = base * base % p as u128;
            }
        }
        assert_eq!(value as u64, 172_252_358);
        assert_eq!(value.pow(4) % p as u128, p as u128 - 1);
        assert_eq!((value.pow(4) % p as u128).pow(2) % p as u128, 1);

        let mut comparisons = 0;
        for (r0, r1) in [
            (0, 1), (1, 2), (2, 7), (17, 31), (31, 997),
            (101, 103), (1_000, 20_000), (p - 3, p - 1),
        ] {
            for leading in [1u64, p - 2] {
                let f = polynomial(&[r0, r1], p, leading);
                let (report, events) = run(&f, p, budget());
                assert_eq!(report.stop, Stop::Complete, "p={p} roots=({r0},{r1})");
                let mut expected = [r0, r1];
                expected.sort_unstable();
                assert_eq!(&report.roots[..report.root_count as usize], &expected);
                assert!(check(&f, &report, &events).unwrap().finite_complete);
                comparisons += 1;
            }
        }
        assert_eq!(comparisons, 16);
    }
    #[test]
    fn round47_quadratic_sqrt_congruence_classes_and_replay() {
        for p in [1_000_000_009u64, 1_000_000_021, 1_000_000_007] {
            for (r0, r1) in [(2u64, 7u64), (17, 31), (0, 1), (31, 997)] {
                let mut expected = [r0, r1];
                expected.sort_unstable();
                for leading in [1, p - 2] {
                    let f = polynomial(&expected, p, leading);
                    let (report, events) = run(&f, p, budget());
                    assert_eq!(report.stop, Stop::Complete,
                        "p={p} roots={expected:?} stop={:?}", report.stop);
                    assert_eq!(&report.roots[..usize::from(report.root_count)], &expected);
                    assert!(check(&f, &report, &events).unwrap().finite_complete);
                    assert!(events.iter().any(|event|
                        matches!(event.kind, EventKind::Split { .. })),
                        "expected split event for p={p}");
                }
            }
        }
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
    fn round40_linear_and_constant_frobenius_events_are_certified() {
        // Exercise monic/nonmonic linear factors, zero roots, and constants
        // across small, word-sized, and Mersenne prime fields.
        for p in [17u64, 487, 1_000_000_007, (1u64 << 61) - 1] {
            for a in [1, 2, p - 2] {
                for b in [0, 1, 3, p - 1] {
                    let f = [b, a];
                    let (report, events) = run(&f, p, budget());
                    assert_eq!(report.stop, Stop::Complete, "p={p} a={a} b={b}");
                    assert_eq!(report.root_count, 1);
                    let root = report.roots[0];
                    assert_eq!(
                        (u128::from(a) * u128::from(root) + u128::from(b))
                            % u128::from(p), 0,
                        "p={p} a={a} b={b}"
                    );
                    assert!(check(&f, &report, &events).unwrap().finite_complete);
                    let (xp, g) = events.iter().find_map(|e| {
                        if let EventKind::LinearPart { xp, polynomial } = e.kind {
                            Some((xp, polynomial))
                        } else { None }
                    }).unwrap();
                    assert_eq!(xp, Poly17::constant(root));
                    assert_eq!(g.len, 2);
                    assert_eq!(g.coefficients[1], 1);
                    assert_eq!(g.coefficients[0], if root == 0 { 0 } else { p - root });
                }
            }
            for c in [1, 2, p - 1] {
                let f = [c];
                let (report, events) = run(&f, p, budget());
                assert_eq!(report.stop, Stop::Complete);
                assert_eq!(report.root_count, 0);
                assert!(check(&f, &report, &events).unwrap().finite_complete);
                assert!(events.iter().any(|e| matches!(
                    e.kind, EventKind::LinearPart { xp: Poly17::ZERO, polynomial }
                    if polynomial == Poly17::constant(1)
                )));
            }
        }
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
