//! Bounded ordinary-polynomial F4 over a borrowed Fp64Arithmetic context.
//!
//! Field construction/primality certification and observer-owned storage belong
//! to the caller. This engine does not inherit tower quotient rules. Reservations
//! are cumulative modeled capacities/work, never RSS or retired instructions.
use crate::cryptanalysis::fp_backend::Fp64Arithmetic;
use std::{
    cmp::Ordering,
    mem::size_of,
    time::{Duration, Instant},
};

pub const MAX_VARIABLES: usize = 28;
pub const MAX_WORK: u64 = 16_777_216;
pub const MAX_BYTES: u64 = 64 * 1024 * 1024;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Mono28 {
    exponents: [u16; MAX_VARIABLES],
}
impl Mono28 {
    pub fn new(exponents: &[u16], variables: usize) -> Option<Self> {
        if !(1..=MAX_VARIABLES).contains(&variables) || exponents.len() != variables {
            return None;
        }
        let mut out = Self {
            exponents: [0; MAX_VARIABLES],
        };
        out.exponents[..variables].copy_from_slice(exponents);
        Some(out)
    }
    pub fn exponents(&self, variables: usize) -> Option<&[u16]> {
        (1..=MAX_VARIABLES)
            .contains(&variables)
            .then(|| &self.exponents[..variables])
    }
    fn valid(&self, v: usize) -> bool {
        self.exponents[v..].iter().all(|&e| e == 0)
    }
    pub fn degree(&self) -> u32 {
        self.exponents.iter().map(|&e| u32::from(e)).sum()
    }
    fn active_degree(&self, v: usize) -> u32 {
        self.exponents[..v].iter().map(|&e| u32::from(e)).sum()
    }
    fn cmp(&self, other: &Self, v: usize) -> Ordering {
        self.active_degree(v)
            .cmp(&other.active_degree(v))
            .then_with(|| {
                (0..v)
                    .rev()
                    .find_map(|i| {
                        let c = other.exponents[i].cmp(&self.exponents[i]);
                        (c != Ordering::Equal).then_some(c)
                    })
                    .unwrap_or(Ordering::Equal)
            })
    }
    fn divides(&self, other: &Self, v: usize) -> bool {
        (0..v).all(|i| self.exponents[i] <= other.exponents[i])
    }
    fn product(self, b: Self, v: usize) -> Option<Self> {
        let mut out = self;
        for i in 0..v {
            out.exponents[i] = out.exponents[i].checked_add(b.exponents[i])?;
        }
        Some(out)
    }
    fn quotient(self, b: Self, v: usize) -> Option<Self> {
        let mut out = self;
        for i in 0..v {
            out.exponents[i] = out.exponents[i].checked_sub(b.exponents[i])?;
        }
        Some(out)
    }
    fn lcm(self, b: Self, v: usize) -> Self {
        let mut out = self;
        for i in 0..v {
            out.exponents[i] = out.exponents[i].max(b.exponents[i]);
        }
        out
    }
    fn coprime(self, b: Self, v: usize) -> bool {
        (0..v).all(|i| self.exponents[i] == 0 || b.exponents[i] == 0)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CanonicalTerm {
    pub mono: Mono28,
    pub coefficient: u64,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InputPolynomial {
    pub terms: Vec<CanonicalTerm>,
}
// The private engine uses the same storage with Montgomery coefficients. Only
// canonical coefficients cross the public API boundary, including partial bases.
type Poly = InputPolynomial;
#[derive(Clone, Copy, Debug)]
pub struct Limits {
    pub work: u64,
    pub bytes: u64,
    pub rows: usize,
    pub columns: usize,
    pub basis: usize,
    pub pairs: usize,
    pub steps: usize,
    pub events: usize,
    pub max_pair_degree: u32,
}
impl Default for Limits {
    fn default() -> Self {
        Self {
            work: MAX_WORK,
            bytes: MAX_BYTES,
            rows: 8192,
            columns: 8192,
            basis: 256,
            pairs: 32768,
            steps: 64,
            events: 4096,
            max_pair_degree: 12,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stop {
    Complete,
    UnitDerived,
    PairDegreeLimit,
    WorkLimit,
    StorageLimit,
    RowLimit,
    ColumnLimit,
    BasisLimit,
    PairLimit,
    StepLimit,
    EventLimit,
    ExponentOverflow,
    Allocation,
    Deadline,
    Observer,
    InvalidInput,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stage {
    Begin,
    Initial,
    Basis,
    Batch,
    Terminal,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Reserved {
    pub work: u64,
    pub bytes: u64,
}
#[derive(Clone, Copy, Debug)]
pub struct Event {
    pub stage: Stage,
    pub sequence: usize,
    pub reserved: Reserved,
}
#[derive(Debug)]
pub struct Report {
    pub basis: Vec<InputPolynomial>,
    pub stop: Stop,
    pub reserved: Reserved,
    pub acknowledged_events: usize,
    pub native_backend_invoked: bool,
    pub native_statistics_complete: bool,
    pub matrix_rows_reduced: u64,
    pub pivot_updates: u64,
    pub pairs_pending: usize,
    pub pairs_selected_unfinished: usize,
    pub maximum_rows_admitted: usize,
    pub maximum_columns_admitted: usize,
    pub maximum_pending_pairs: usize,
    pub completed_steps: usize,
    pub maximum_input_degree: u32,
    pub maximum_processed_row_degree: u32,
}
struct Gate<'a, O> {
    limits: Limits,
    reserved: Reserved,
    deadline: Instant,
    events: usize,
    observer: &'a mut O,
}
// Requested helper state, not measured stack/RSS. The Vec's own gate.vec
// allocation tariff remains separate from this 128-byte envelope.
#[derive(Default)]
struct FrontierScratch {
    hash: u64,
    slot: u64,
    probes: u64,
    decoded_front_index: u64,
    axis: u64,
    left_exponent: u64,
    right_exponent: u64,
    init_position: u64,
    init_end: u64,
    publish_code: u64,
    active_variables: u64,
    logical_column_limit: u64,
}
struct FrontierIndex {
    slots: Vec<u32>,
    mask: usize,
    scratch: FrontierScratch,
}
const _: () = assert!(size_of::<usize>() == 8);
const _: () = assert!(size_of::<Mono28>() == 56);
const _: () = assert!(size_of::<FrontierScratch>() == 96);
const _: () = assert!(size_of::<FrontierIndex>() <= 128);
const _: () = assert!(size_of::<Option<FrontierIndex>>() <= 128);

impl FrontierIndex {
    fn new<O: FnMut(Event) -> Result<(), ()>>(
        gate: &mut Gate<'_, O>,
        variables: usize,
    ) -> Result<Self, Stop> {
        if gate.limits.columns == 0 {
            return Err(Stop::ColumnLimit);
        }
        if gate.limits.columns > 8192 || !(1..=MAX_VARIABLES).contains(&variables) {
            return Err(Stop::InvalidInput);
        }
        // These bounded admission calculations precede creation of helper state.
        let slots = gate
            .limits
            .columns
            .checked_mul(2)
            .and_then(usize::checked_next_power_of_two)
            .ok_or(Stop::StorageLimit)?;
        if !(2..=16384).contains(&slots) {
            return Err(Stop::InvalidInput);
        }
        gate.permit(0, 128)?;
        let mut index = Self {
            slots: Vec::new(),
            mask: slots - 1,
            scratch: FrontierScratch {
                active_variables: variables as u64,
                logical_column_limit: gate.limits.columns as u64,
                init_end: slots as u64,
                ..FrontierScratch::default()
            },
        };
        index.slots = gate.vec::<u32>(index.scratch.init_end as usize)?;
        gate.permit(
            index
                .scratch
                .init_end
                .checked_mul(16)
                .ok_or(Stop::WorkLimit)?,
            0,
        )?;
        while index.scratch.init_position < index.scratch.init_end {
            #[cfg(test)]
            frontier_test_expiry(
                gate,
                FrontierCheckpoint::Initialize,
                index.scratch.init_position,
            );
            gate.check()?;
            index.scratch.axis = (index.scratch.init_position + 64).min(index.scratch.init_end);
            while index.scratch.init_position < index.scratch.axis {
                // Capacity S was admitted. Exactly S logical stores; allocator
                // overcapacity is neither initialized nor a legal column grant.
                index.slots.push(0);
                index.scratch.init_position += 1;
            }
        }
        gate.check()?;
        Ok(index)
    }
    fn add<O: FnMut(Event) -> Result<(), ()>>(
        &mut self,
        gate: &mut Gate<'_, O>,
        frontier: &mut Vec<Mono28>,
        mono: &Mono28,
    ) -> Result<(), Stop> {
        gate.permit(
            self.scratch
                .active_variables
                .checked_add(1)
                .and_then(|v| v.checked_mul(64))
                .ok_or(Stop::WorkLimit)?,
            0,
        )?;
        self.scratch.hash = 0xcbf29ce484222325 ^ self.scratch.active_variables;
        self.scratch.axis = 0;
        while self.scratch.axis < self.scratch.active_variables {
            #[cfg(test)]
            frontier_test_expiry(gate, FrontierCheckpoint::Hash, self.scratch.axis);
            gate.check()?;
            self.scratch.hash = (self.scratch.hash
                ^ u64::from(mono.exponents[self.scratch.axis as usize]))
            .wrapping_mul(0x00000100000001b3);
            self.scratch.axis += 1;
        }
        self.scratch.hash ^= self.scratch.hash >> 32;
        #[cfg(test)]
        FRONTIER_TEST_HASH.with(|override_hash| {
            if let Some(hash) = override_hash.get() {
                self.scratch.hash = hash;
            }
        });
        self.scratch.slot = (self.scratch.hash as usize & self.mask) as u64;
        self.scratch.probes = 0;
        while self.scratch.probes < self.slots.len() as u64 {
            #[cfg(test)]
            frontier_test_expiry(gate, FrontierCheckpoint::Probe, self.scratch.probes);
            gate.check()?;
            gate.permit(
                self.scratch
                    .active_variables
                    .checked_add(1)
                    .and_then(|v| v.checked_mul(64))
                    .and_then(|v| v.checked_add(32))
                    .ok_or(Stop::WorkLimit)?,
                0,
            )?;
            self.scratch.decoded_front_index = u64::from(self.slots[self.scratch.slot as usize]);
            if self.scratch.decoded_front_index == 0 {
                if frontier.len() as u64 >= self.scratch.logical_column_limit {
                    return Err(Stop::ColumnLimit);
                }
                gate.permit(16 * ((size_of::<Mono28>() as u64).div_ceil(8) + 1) + 32, 0)?;
                self.scratch.publish_code = u64::from(
                    u32::try_from(frontier.len().checked_add(1).ok_or(Stop::InvalidInput)?)
                        .map_err(|_| Stop::InvalidInput)?,
                );
                // Commit as a pair: gate.push may refuse before append. Once it
                // succeeds, publication has no intervening fallible operation.
                gate.push(frontier, *mono)?;
                self.slots[self.scratch.slot as usize] = self.scratch.publish_code as u32;
                return Ok(());
            }
            self.scratch.decoded_front_index -= 1;
            if self.scratch.decoded_front_index as usize >= frontier.len() {
                return Err(Stop::InvalidInput);
            }
            self.scratch.axis = 0;
            while self.scratch.axis < self.scratch.active_variables {
                #[cfg(test)]
                frontier_test_expiry(gate, FrontierCheckpoint::Compare, self.scratch.axis);
                gate.check()?;
                self.scratch.left_exponent = u64::from(
                    frontier[self.scratch.decoded_front_index as usize].exponents
                        [self.scratch.axis as usize],
                );
                self.scratch.right_exponent = u64::from(mono.exponents[self.scratch.axis as usize]);
                if self.scratch.left_exponent != self.scratch.right_exponent {
                    break;
                }
                self.scratch.axis += 1;
            }
            if self.scratch.axis == self.scratch.active_variables {
                return Ok(());
            }
            self.scratch.slot = (self.scratch.slot + 1) & self.mask as u64;
            self.scratch.probes += 1;
        }
        Err(Stop::InvalidInput)
    }
}

#[cfg(test)]
#[derive(Clone, Copy, PartialEq, Eq)]
enum FrontierCheckpoint {
    Initialize,
    Hash,
    Probe,
    Compare,
}
#[cfg(test)]
thread_local! {
    static INVERSE_TEST_LEGACY: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
    static INVERSE_TEST_OPS: std::cell::Cell<(u64,u64)> = const { std::cell::Cell::new((0,0)) };
    static FRONTIER_TEST_HASH: std::cell::Cell<Option<u64>> = const { std::cell::Cell::new(None) };
    static FRONTIER_TEST_EXPIRY: std::cell::Cell<Option<(FrontierCheckpoint,u64)>> = const { std::cell::Cell::new(None) };
    static FRONTIER_TEST_LEGACY: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
    static FRONTIER_TEST_BATCHES: std::cell::RefCell<Option<Vec<Vec<(usize,usize)>>>> = const { std::cell::RefCell::new(None) };
}
#[cfg(test)]
fn frontier_test_expiry<O: FnMut(Event) -> Result<(), ()>>(
    gate: &mut Gate<'_, O>,
    checkpoint: FrontierCheckpoint,
    index: u64,
) {
    if FRONTIER_TEST_EXPIRY.with(|target| target.get() == Some((checkpoint, index))) {
        gate.deadline = Instant::now();
    }
}
impl<O: FnMut(Event) -> Result<(), ()>> Gate<'_, O> {
    fn check(&self) -> Result<(), Stop> {
        if Instant::now() >= self.deadline {
            Err(Stop::Deadline)
        } else {
            Ok(())
        }
    }
    fn permit(&mut self, work: u64, bytes: u64) -> Result<(), Stop> {
        self.check()?;
        let w = self
            .reserved
            .work
            .checked_add(work)
            .ok_or(Stop::WorkLimit)?;
        let b = self
            .reserved
            .bytes
            .checked_add(bytes)
            .ok_or(Stop::StorageLimit)?;
        if w > self.limits.work {
            return Err(Stop::WorkLimit);
        }
        if b > self.limits.bytes {
            return Err(Stop::StorageLimit);
        }
        self.reserved = Reserved { work: w, bytes: b };
        Ok(())
    }
    fn event(&mut self, stage: Stage) -> Result<(), Stop> {
        if self.events >= self.limits.events {
            return Err(Stop::EventLimit);
        }
        self.permit(16, 256)?;
        (self.observer)(Event {
            stage,
            sequence: self.events,
            reserved: self.reserved,
        })
        .map_err(|_| Stop::Observer)?;
        self.events += 1;
        self.check()
    }
    fn vec<T>(&mut self, n: usize) -> Result<Vec<T>, Stop> {
        std::alloc::Layout::array::<T>(n).map_err(|_| Stop::StorageLimit)?;
        let bytes = (n as u64)
            .checked_mul(size_of::<T>() as u64)
            .and_then(|b| b.checked_add(128))
            .ok_or(Stop::StorageLimit)?;
        self.permit(0, bytes)?;
        let mut out = Vec::new();
        out.try_reserve_exact(n).map_err(|_| Stop::Allocation)?;
        Ok(out)
    }
    fn push<T>(&mut self, out: &mut Vec<T>, item: T) -> Result<(), Stop> {
        if out.len() == out.capacity() {
            let cap = out.capacity().saturating_mul(2).max(4);
            std::alloc::Layout::array::<T>(cap).map_err(|_| Stop::StorageLimit)?;
            let bytes = (cap as u64)
                .checked_mul(size_of::<T>() as u64)
                .and_then(|b| b.checked_add(128))
                .ok_or(Stop::StorageLimit)?;
            self.permit(out.len() as u64, bytes)?;
            out.try_reserve_exact(cap - out.len())
                .map_err(|_| Stop::Allocation)?;
        }
        out.push(item);
        Ok(())
    }
}
#[derive(Clone, Copy)]
struct Pair {
    a: usize,
    b: usize,
    lcm: Mono28,
}
struct Row {
    cols: Vec<u32>,
    vals: Vec<u64>,
}
struct Engine<'a, 'o, O> {
    field: &'a Fp64Arithmetic,
    v: usize,
    gate: Gate<'o, O>,
    basis: Vec<Poly>,
    pairs: Vec<Pair>,
    selected: usize,
    invoked: bool,
    matrix_rows: u64,
    updates: u64,
    max_row_degree: u32,
    peak_rows: usize,
    peak_columns: usize,
    peak_pairs: usize,
    steps: usize,
}
impl<O: FnMut(Event) -> Result<(), ()>> Engine<'_, '_, O> {
    fn mono_cost(&self) -> u64 {
        8 * (self.v as u64 + 1)
    }
    fn copy_poly(&mut self, p: &Poly) -> Result<Poly, Stop> {
        self.gate.permit(16 * p.terms.len() as u64, 0)?;
        let mut terms = self.gate.vec(p.terms.len())?;
        terms.extend_from_slice(&p.terms);
        Ok(Poly { terms })
    }
    fn inverse(&mut self, a: u64) -> Result<u64, Stop> {
        if a == 0 {
            return Err(Stop::InvalidInput);
        }
        #[cfg(test)]
        let dispatch = !INVERSE_TEST_LEGACY.with(|flag| flag.get());
        #[cfg(not(test))]
        let dispatch = true;
        if dispatch {
            // Load, compare, branch/control and return: four scalar actions.
            // Montgomery identity is field.one(), not canonical integer 1.
            self.gate.permit(32, 0)?;
            let one = self.field.one();
            if a == one {
                return Ok(one);
            }
        }
        // General inversion retains its full original tariff and algorithm.
        self.gate.permit(4096, 0)?;
        let mut e = self.field.p() - 2;
        let mut x = a;
        let mut r = self.field.one();
        while e != 0 {
            self.gate.check()?;
            if e & 1 != 0 {
                r = self.field.mul(r, x);
                #[cfg(test)]
                INVERSE_TEST_OPS.with(|ops| {
                    let (m, s) = ops.get();
                    ops.set((m + 1, s));
                });
            }
            x = self.field.sqr(x);
            #[cfg(test)]
            INVERSE_TEST_OPS.with(|ops| {
                let (m, s) = ops.get();
                ops.set((m, s + 1));
            });
            e >>= 1;
        }
        Ok(r)
    }
    fn monic(&mut self, p: &mut Poly) -> Result<(), Stop> {
        if p.terms.is_empty() {
            return Ok(());
        }
        let inv = self.inverse(p.terms[0].coefficient)?;
        self.gate.permit(32 * p.terms.len() as u64, 0)?;
        for chunk in p.terms.chunks_mut(256) {
            self.gate.check()?;
            for t in chunk {
                t.coefficient = self.field.mul(t.coefficient, inv);
            }
        }
        Ok(())
    }
    fn shift(&mut self, which: usize, m: Mono28, scale: u64) -> Result<Poly, Stop> {
        let n = self.basis[which].terms.len();
        self.gate.permit(n as u64 * (self.mono_cost() + 32), 0)?;
        let mut terms = self.gate.vec(n)?;
        for (i, t) in self.basis[which].terms.iter().enumerate() {
            if i % 256 == 0 {
                self.gate.check()?;
            }
            terms.push(CanonicalTerm {
                mono: t.mono.product(m, self.v).ok_or(Stop::ExponentOverflow)?,
                coefficient: self.field.mul(t.coefficient, scale),
            });
        }
        Ok(Poly { terms })
    }
    fn subtract(&mut self, a: &Poly, b: &Poly) -> Result<Poly, Stop> {
        let count = a
            .terms
            .len()
            .checked_add(b.terms.len())
            .ok_or(Stop::StorageLimit)?;
        self.gate
            .permit(32 * (self.v as u64 + 1) * (count as u64 + 1), 0)?;
        let mut terms = self.gate.vec(count)?;
        let (mut i, mut j) = (0, 0);
        while i < a.terms.len() || j < b.terms.len() {
            if (i + j) % 256 == 0 {
                self.gate.check()?;
            }
            let order = if i == a.terms.len() {
                Ordering::Less
            } else if j == b.terms.len() {
                Ordering::Greater
            } else {
                a.terms[i].mono.cmp(&b.terms[j].mono, self.v)
            };
            match order {
                Ordering::Greater => {
                    terms.push(a.terms[i]);
                    i += 1;
                }
                Ordering::Less => {
                    let t = b.terms[j];
                    terms.push(CanonicalTerm {
                        mono: t.mono,
                        coefficient: self.field.neg(t.coefficient),
                    });
                    j += 1;
                }
                Ordering::Equal => {
                    let c = self
                        .field
                        .sub(a.terms[i].coefficient, b.terms[j].coefficient);
                    if c != 0 {
                        terms.push(CanonicalTerm {
                            mono: a.terms[i].mono,
                            coefficient: c,
                        });
                    }
                    i += 1;
                    j += 1;
                }
            }
        }
        Ok(Poly { terms })
    }
    fn normal_reduce(&mut self, mut p: Poly) -> Result<Poly, Stop> {
        loop {
            let work = (p.terms.len() as u64)
                .checked_mul(self.basis.len() as u64 + 1)
                .and_then(|x| x.checked_mul(16 * (self.v as u64 + 1)))
                .ok_or(Stop::WorkLimit)?;
            self.gate.permit(work, 0)?;
            let mut chosen = None;
            'outer: for (i, t) in p.terms.iter().enumerate() {
                self.gate.check()?;
                for (j, b) in self.basis.iter().enumerate() {
                    if b.terms[0].mono.divides(&t.mono, self.v) {
                        chosen = Some((i, j));
                        break 'outer;
                    }
                }
            }
            let Some((i, j)) = chosen else {
                return Ok(p);
            };
            self.gate.permit(self.mono_cost(), 0)?;
            let q = p.terms[i]
                .mono
                .quotient(self.basis[j].terms[0].mono, self.v)
                .ok_or(Stop::InvalidInput)?;
            let multiple = self.shift(j, q, p.terms[i].coefficient)?;
            p = self.subtract(&p, &multiple)?;
        }
    }
    fn insert(&mut self, mut p: Poly) -> Result<bool, Stop> {
        if p.terms.is_empty() {
            return Ok(false);
        }
        if self.basis.len() >= self.gate.limits.basis {
            return Err(Stop::BasisLimit);
        }
        self.monic(&mut p)?;
        // Reserve eventual canonical output conversion before retaining this basis.
        self.gate.permit(32 * p.terms.len() as u64, 0)?;
        let new = self.basis.len();
        let lm = p.terms[0].mono;
        self.gate.push(&mut self.basis, p)?;
        // Admission may stop midway: all created pairs remain pending; no complete claim.
        for j in 0..new {
            self.gate.permit(32 * (self.v as u64 + 1), 0)?;
            let other = self.basis[j].terms[0].mono;
            if other.coprime(lm, self.v) {
                continue;
            }
            if self.pairs.len() >= self.gate.limits.pairs {
                return Err(Stop::PairLimit);
            }
            let pair = Pair {
                a: j,
                b: new,
                lcm: other.lcm(lm, self.v),
            };
            self.gate.push(&mut self.pairs, pair)?;
            self.peak_pairs = self.peak_pairs.max(self.pairs.len());
        }
        self.gate.event(Stage::Basis)?;
        Ok(lm.degree() == 0)
    }
    fn add_frontier(
        &mut self,
        index: &mut Option<FrontierIndex>,
        front: &mut Vec<Mono28>,
        m: &Mono28,
    ) -> Result<(), Stop> {
        #[cfg(test)]
        if FRONTIER_TEST_LEGACY.with(|legacy| legacy.get()) {
            // Exact former membership path, test-only differential oracle.
            self.gate
                .permit(16 * (self.v as u64 + 1) * (front.len() as u64 + 1), 0)?;
            if front.contains(m) {
                return Ok(());
            }
            if front.len() >= self.gate.limits.columns {
                return Err(Stop::ColumnLimit);
            }
            return self.gate.push(front, *m);
        }
        if index.is_none() {
            *index = Some(FrontierIndex::new(&mut self.gate, self.v)?);
        }
        index
            .as_mut()
            .ok_or(Stop::InvalidInput)?
            .add(&mut self.gate, front, m)
    }
    fn batch(&mut self, selected: &[Pair]) -> Result<(), Stop> {
        #[cfg(test)]
        FRONTIER_TEST_BATCHES.with(|batches| {
            if let Some(batches) = batches.borrow_mut().as_mut() {
                batches.push(selected.iter().map(|p| (p.a, p.b)).collect());
            }
        });
        self.gate.event(Stage::Batch)?;
        let mut srows = self.gate.vec(selected.len())?;
        let mut reducers = self.gate.vec(0)?;
        let mut frontier = self.gate.vec(0)?;
        let mut frontier_index = None;
        for pair in selected {
            self.gate.permit(2 * self.mono_cost(), 0)?;
            let a = pair
                .lcm
                .quotient(self.basis[pair.a].terms[0].mono, self.v)
                .ok_or(Stop::InvalidInput)?;
            let b = pair
                .lcm
                .quotient(self.basis[pair.b].terms[0].mono, self.v)
                .ok_or(Stop::InvalidInput)?;
            let pa = self.shift(pair.a, a, self.field.one())?;
            let pb = self.shift(pair.b, b, self.field.one())?;
            let s = self.subtract(&pa, &pb)?;
            for t in &s.terms {
                self.add_frontier(&mut frontier_index, &mut frontier, &t.mono)?;
            }
            srows.push(s);
        }
        let mut processed = 0;
        while processed < frontier.len() {
            self.gate
                .permit(16 * (self.v as u64 + 1) * (self.basis.len() as u64 + 1), 0)?;
            let m = frontier[processed];
            processed += 1;
            let reducer = self
                .basis
                .iter()
                .position(|b| b.terms[0].mono.divides(&m, self.v));
            if let Some(index) = reducer {
                if reducers.len() + srows.len() >= self.gate.limits.rows {
                    return Err(Stop::RowLimit);
                }
                self.gate.permit(self.mono_cost(), 0)?;
                let q = m
                    .quotient(self.basis[index].terms[0].mono, self.v)
                    .ok_or(Stop::InvalidInput)?;
                let row = self.shift(index, q, self.field.one())?;
                // Each processed monomial has exactly one deterministic reducer multiple.
                for t in &row.terms {
                    self.add_frontier(&mut frontier_index, &mut frontier, &t.mono)?;
                }
                self.gate.push(&mut reducers, row)?;
            }
        }
        // Table entries name append positions, so retire the index before sort.
        // Requested reservations are cumulative and are never refunded here.
        drop(frontier_index);
        let c = frontier.len();
        self.peak_columns = self.peak_columns.max(c);
        self.peak_rows = self.peak_rows.max(reducers.len() + srows.len());
        let log = c.max(2).next_power_of_two().trailing_zeros() as u64;
        self.gate
            .permit(32 * (self.v as u64 + 1) * c as u64 * (log + 1), 0)?;
        frontier.sort_unstable_by(|a, b| b.cmp(a, self.v));
        self.gate.check()?;
        let mut pivot_of = self.gate.vec(c)?;
        pivot_of.resize(c, None::<usize>);
        let mut pivots: Vec<Row> = self.gate.vec(0)?;
        let mut acc = self.gate.vec(c)?;
        acc.resize(c, 0u64);
        let mut candidates = self.gate.vec(0)?;
        let reducer_count = reducers.len();
        for (number, row) in reducers.into_iter().chain(srows).enumerate() {
            self.gate.permit(
                64 * c as u64 + 16 * (self.v as u64 + 1) * (log + 1) * row.terms.len() as u64,
                0,
            )?;
            self.max_row_degree = self.max_row_degree.max(
                row.terms
                    .iter()
                    .map(|t| t.mono.active_degree(self.v))
                    .max()
                    .unwrap_or(0),
            );
            acc.fill(0);
            for t in row.terms {
                let j = frontier
                    .binary_search_by(|m| t.mono.cmp(m, self.v))
                    .map_err(|_| Stop::InvalidInput)?;
                acc[j] = t.coefficient;
            }
            let mut lead = None;
            for j in 0..c {
                if j % 256 == 0 {
                    self.gate.check()?;
                }
                let value = acc[j];
                if value == 0 {
                    continue;
                }
                if let Some(index) = pivot_of[j] {
                    let pivot = &pivots[index];
                    self.gate.permit(64 * (pivot.cols.len() as u64 + 1), 0)?;
                    for chunk in pivot.cols.chunks(256).zip(pivot.vals.chunks(256)) {
                        self.gate.check()?;
                        for (&col, &coefficient) in chunk.0.iter().zip(chunk.1) {
                            acc[col as usize] = self
                                .field
                                .sub(acc[col as usize], self.field.mul(value, coefficient));
                        }
                    }
                    self.updates += 1;
                } else {
                    lead = Some(j);
                    break;
                }
            }
            self.matrix_rows += 1;
            if let Some(j) = lead {
                let inv = self.inverse(acc[j])?;
                self.gate.permit(32 * c as u64, 0)?;
                let count = acc[j..].iter().filter(|&&x| x != 0).count();
                let mut cols = self.gate.vec(count)?;
                let mut vals = self.gate.vec(count)?;
                let mut terms = if number >= reducer_count {
                    Some(self.gate.vec(count)?)
                } else {
                    None
                };
                for k in j..c {
                    if k % 256 == 0 {
                        self.gate.check()?;
                    }
                    if acc[k] != 0 {
                        let coefficient = self.field.mul(acc[k], inv);
                        cols.push(k as u32);
                        vals.push(coefficient);
                        if let Some(t) = &mut terms {
                            t.push(CanonicalTerm {
                                mono: frontier[k],
                                coefficient,
                            });
                        }
                    }
                }
                pivot_of[j] = Some(pivots.len());
                self.gate.push(&mut pivots, Row { cols, vals })?;
                if let Some(terms) = terms {
                    self.gate.push(&mut candidates, Poly { terms })?;
                }
            }
        }
        for candidate in candidates {
            let p = self.normal_reduce(candidate)?;
            if self.insert(p)? {
                return Err(Stop::UnitDerived);
            }
        }
        Ok(())
    }
    fn run_inner(&mut self, input: &[InputPolynomial]) -> Result<Stop, Stop> {
        if input.len() > self.gate.limits.rows {
            return Err(Stop::RowLimit);
        }
        self.peak_rows = input.len();
        self.gate.event(Stage::Initial)?;
        for raw in input {
            let mut p = self.copy_poly(raw)?;
            self.gate.permit(32 * p.terms.len() as u64, 0)?;
            for chunk in p.terms.chunks_mut(256) {
                self.gate.check()?;
                for t in chunk {
                    t.coefficient = self.field.from_u64(t.coefficient);
                    // An acknowledged Initial event or admitted copy alone is
                    // not native arithmetic. Empty generators do not set this.
                    self.invoked = true;
                    self.max_row_degree = self.max_row_degree.max(t.mono.active_degree(self.v));
                }
            }
            let p = self.normal_reduce(p)?;
            if self.insert(p)? {
                return Ok(Stop::UnitDerived);
            }
        }

        while !self.pairs.is_empty() {
            self.gate
                .permit(96 * (self.v as u64 + 1) * (self.pairs.len() as u64 + 1), 0)?;
            let degree = self
                .pairs
                .iter()
                .map(|p| p.lcm.active_degree(self.v))
                .min()
                .ok_or(Stop::InvalidInput)?;
            if degree > self.gate.limits.max_pair_degree {
                return Ok(Stop::PairDegreeLimit);
            }
            if self.steps >= self.gate.limits.steps {
                return Err(Stop::StepLimit);
            }
            if self.gate.limits.rows == 0 {
                return Err(Stop::RowLimit);
            }
            // Deterministic stable insertion order is ascending new basis ID then old ID.
            let count = self
                .pairs
                .iter()
                .filter(|p| p.lcm.active_degree(self.v) == degree)
                .count()
                .min(self.gate.limits.rows);
            let mut selected = self.gate.vec(count)?;
            for pair in &self.pairs {
                if pair.lcm.degree() == degree && selected.len() < count {
                    selected.push(*pair);
                }
            }
            self.selected = count;
            match self.batch(&selected) {
                Err(Stop::UnitDerived) => return Ok(Stop::UnitDerived),
                Err(e) => return Err(e),
                Ok(()) => {}
            }
            self.gate.permit(
                16 * (self.pairs.len() as u64 + 1) * (selected.len() as u64 + 1),
                0,
            )?;
            self.pairs
                .retain(|p| !selected.iter().any(|s| s.a == p.a && s.b == p.b));
            self.selected = 0;
            self.steps += 1;
        }
        Ok(Stop::Complete)
    }
}

/// Run bounded ordinary F4. The borrowed field must be prime-certified and its
/// construction admitted by the caller. No square-root setup occurs here.
pub fn run<O: FnMut(Event) -> Result<(), ()>>(
    field: &Fp64Arithmetic,
    variables: u8,
    input: &[InputPolynomial],
    limits: &Limits,
    original_deadline: Instant,
    observer: &mut O,
) -> Report {
    let v = usize::from(variables);
    let defaults = Limits::default();
    let empty = |stop| Report {
        basis: Vec::new(),
        stop,
        reserved: Reserved::default(),
        acknowledged_events: 0,
        native_backend_invoked: false,
        native_statistics_complete: false,
        matrix_rows_reduced: 0,
        pivot_updates: 0,
        pairs_pending: 0,
        pairs_selected_unfinished: 0,
        maximum_rows_admitted: 0,
        maximum_columns_admitted: 0,
        maximum_pending_pairs: 0,
        completed_steps: 0,
        maximum_input_degree: 0,
        maximum_processed_row_degree: 0,
    };
    if !(1..=28).contains(&v)
        || !(5..=(1u64 << 61) - 1).contains(&field.p())
        || input.len() > 256
        || limits.work > MAX_WORK
        || limits.bytes > MAX_BYTES
        || limits.rows > defaults.rows
        || limits.columns > defaults.columns
        || limits.basis > defaults.basis
        || limits.pairs > defaults.pairs
        || limits.steps > defaults.steps
        || limits.events > defaults.events
        || limits.max_pair_degree > 12
    {
        return empty(Stop::InvalidInput);
    }
    let Some(terms) = input
        .iter()
        .try_fold(0usize, |n, p| n.checked_add(p.terms.len()))
    else {
        return empty(Stop::InvalidInput);
    };
    if terms > 8192 {
        return empty(Stop::InvalidInput);
    }
    let deadline = original_deadline.min(Instant::now() + Duration::from_secs(5));
    let mut e = Engine {
        field,
        v,
        gate: Gate {
            limits: *limits,
            reserved: Reserved::default(),
            deadline,
            events: 0,
            observer,
        },
        basis: Vec::new(),
        pairs: Vec::new(),
        selected: 0,
        invoked: false,
        matrix_rows: 0,
        updates: 0,
        max_row_degree: 0,
        peak_rows: 0,
        peak_columns: 0,
        peak_pairs: 0,
        steps: 0,
    };
    let mut maximum_input_degree = 0;
    let result = (|| {
        e.gate.permit(
            32 * (v as u64 + 1) * terms as u64 + 32 * (input.len() as u64 + 1),
            4096,
        )?;
        for p in input {
            e.gate.check()?;
            let mut previous: Option<Mono28> = None;
            for chunk in p.terms.chunks(256) {
                e.gate.check()?;
                for t in chunk {
                    if !t.mono.valid(v)
                        || t.coefficient == 0
                        || t.coefficient >= field.p()
                        || t.mono.active_degree(v) > 32
                        || previous.is_some_and(|m| m.cmp(&t.mono, v) != Ordering::Greater)
                    {
                        return Err(Stop::InvalidInput);
                    }
                    maximum_input_degree = maximum_input_degree.max(t.mono.active_degree(v));
                    previous = Some(t.mono);
                }
            }
        }
        e.gate.event(Stage::Begin)?;
        e.run_inner(input)
    })();
    let mut stop = match result {
        Ok(s) | Err(s) => s,
    };
    // Every retained term's conversion was reserved at insertion. No allocation
    // or observer call occurs in this bounded partial-report cleanup.
    for p in &mut e.basis {
        for t in &mut p.terms {
            t.coefficient = field.to_u64(t.coefficient);
        }
    }
    if matches!(
        stop,
        Stop::Complete | Stop::UnitDerived | Stop::PairDegreeLimit
    ) {
        if let Err(s) = e.gate.event(Stage::Terminal) {
            stop = s;
        }
    }
    Report {
        basis: e.basis,
        stop,
        reserved: e.gate.reserved,
        acknowledged_events: e.gate.events,
        native_backend_invoked: e.invoked,
        native_statistics_complete: matches!(stop, Stop::Complete | Stop::UnitDerived),
        matrix_rows_reduced: e.matrix_rows,
        pivot_updates: e.updates,
        pairs_pending: e.pairs.len(),
        pairs_selected_unfinished: e.selected,
        maximum_rows_admitted: e.peak_rows,
        maximum_columns_admitted: e.peak_columns,
        maximum_pending_pairs: e.peak_pairs,
        completed_steps: e.steps,
        maximum_input_degree,
        maximum_processed_row_degree: e.max_row_degree,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cryptanalysis::{groebner_f4 as reference, symmetrized_semaev::MPoly};
    use crate::ecc::field::FieldElement;
    use num_bigint::BigUint;
    struct FrontierTestReset;
    impl Drop for FrontierTestReset {
        fn drop(&mut self) {
            FRONTIER_TEST_HASH.with(|v| v.set(None));
            FRONTIER_TEST_EXPIRY.with(|v| v.set(None));
            FRONTIER_TEST_LEGACY.with(|v| v.set(false));
            FRONTIER_TEST_BATCHES.with(|v| *v.borrow_mut() = None);
        }
    }
    fn frontier_gate<O: FnMut(Event) -> Result<(), ()>>(
        observer: &mut O,
        columns: usize,
    ) -> Gate<'_, O> {
        Gate {
            limits: Limits {
                columns,
                ..Limits::default()
            },
            reserved: Reserved::default(),
            deadline: deadline(),
            events: 0,
            observer,
        }
    }
    fn assert_frontier_index(index: &FrontierIndex, frontier: &[Mono28]) {
        let mut seen = vec![false; frontier.len()];
        for &code in &index.slots {
            if code != 0 {
                let id = code as usize - 1;
                assert!(id < frontier.len());
                assert!(!seen[id]);
                seen[id] = true;
            }
        }
        assert!(seen.into_iter().all(|v| v));
    }
    #[test]
    fn dynamic_f4_frontier_append_order_collision_and_all_coordinates() {
        let _reset = FrontierTestReset;
        for forced in [None, Some(127)] {
            FRONTIER_TEST_HASH.with(|v| v.set(forced));
            let mut observer = |_: Event| Ok(());
            let mut gate = frontier_gate(&mut observer, 64);
            let mut index = FrontierIndex::new(&mut gate, 28).unwrap();
            let mut frontier = Vec::new();
            let mut legacy = Vec::new();
            let mut sequence = Vec::new();
            for axis in (0..28).rev() {
                sequence.push(mono(28, &[(axis, u16::MAX)]));
                sequence.push(mono(28, &[(axis, 1)]));
                sequence.push(mono(28, &[(axis, u16::MAX)]));
            }
            sequence.extend([
                mono(28, &[]),
                mono(28, &[(0, 1), (27, u16::MAX)]),
                mono(28, &[]),
            ]);
            for m in sequence {
                index.add(&mut gate, &mut frontier, &m).unwrap();
                if !legacy.contains(&m) {
                    legacy.push(m);
                }
                assert_eq!(frontier, legacy);
                assert_frontier_index(&index, &frontier);
            }
            if forced.is_some() {
                assert_ne!(index.slots[127], 0);
                assert_ne!(index.slots[0], 0);
            }
        }
    }
    #[test]
    fn dynamic_f4_frontier_column_zero_one_max_and_full_duplicates() {
        let _reset = FrontierTestReset;
        let mut observer = |_: Event| Ok(());
        let mut gate = frontier_gate(&mut observer, 0);
        assert!(matches!(
            FrontierIndex::new(&mut gate, 1),
            Err(Stop::ColumnLimit)
        ));
        assert_eq!(gate.reserved, Reserved::default());
        for columns in [1, 8192] {
            let mut gate = frontier_gate(&mut observer, columns);
            let mut index = FrontierIndex::new(&mut gate, 1).unwrap();
            assert_eq!(index.slots.len(), (2 * columns).next_power_of_two());
            let mut frontier = Vec::new();
            for value in 0..columns {
                index
                    .add(&mut gate, &mut frontier, &mono(1, &[(0, value as u16)]))
                    .unwrap();
            }
            assert_frontier_index(&index, &frontier);
            let old = frontier.clone();
            let slots = index.slots.clone();
            index
                .add(&mut gate, &mut frontier, &mono(1, &[(0, 0)]))
                .unwrap();
            assert_eq!(
                index.add(&mut gate, &mut frontier, &mono(1, &[(0, u16::MAX)])),
                Err(Stop::ColumnLimit)
            );
            assert_eq!(frontier, old);
            assert_eq!(index.slots, slots);
        }
        // Spare allocator capacity does not grant a second logical column.
        let mut gate = frontier_gate(&mut observer, 1);
        let mut index = FrontierIndex::new(&mut gate, 1).unwrap();
        let mut frontier = Vec::with_capacity(32);
        index.add(&mut gate, &mut frontier, &mono(1, &[])).unwrap();
        assert_eq!(
            index.add(&mut gate, &mut frontier, &mono(1, &[(0, 1)])),
            Err(Stop::ColumnLimit)
        );
        assert_eq!(frontier.len(), 1);
    }
    #[test]
    fn dynamic_f4_frontier_exact_initialization_and_insertion_tickets() {
        let _reset = FrontierTestReset;
        FRONTIER_TEST_HASH.with(|v| v.set(Some(0)));
        let mut observer = |_: Event| Ok(());
        // Cmax1 => S2, helper128 + gate.vec(128+8), initialization32.
        for (work, bytes, expected, paid_work, paid_bytes) in [
            (32, 127, Stop::StorageLimit, 0, 0),
            (32, 263, Stop::StorageLimit, 0, 128),
            (31, 264, Stop::WorkLimit, 0, 264),
        ] {
            let mut gate = frontier_gate(&mut observer, 1);
            gate.limits.work = work;
            gate.limits.bytes = bytes;
            assert!(matches!(FrontierIndex::new(&mut gate,1),Err(s) if s==expected));
            assert_eq!(
                gate.reserved,
                Reserved {
                    work: paid_work,
                    bytes: paid_bytes
                }
            );
        }
        let mut gate = frontier_gate(&mut observer, 1);
        gate.limits.work = 32;
        gate.limits.bytes = 264;
        let index = FrontierIndex::new(&mut gate, 1).unwrap();
        assert_eq!(index.slots, [0, 0]);
        assert_eq!(
            gate.reserved,
            Reserved {
                work: 32,
                bytes: 264
            }
        );
        // Hash128, each probe160, append160. All prior reservations persist.
        for (available, paid) in [
            (127, 0),
            (128, 128),
            (128 + 159, 128),
            (128 + 160, 288),
            (128 + 160 + 159, 288),
        ] {
            let mut gate = frontier_gate(&mut observer, 1);
            let mut index = FrontierIndex::new(&mut gate, 1).unwrap();
            gate.limits.work = 32 + available;
            let mut frontier = Vec::new();
            assert_eq!(
                index.add(&mut gate, &mut frontier, &mono(1, &[])),
                Err(Stop::WorkLimit)
            );
            assert!(frontier.is_empty());
            assert_eq!(index.slots, [0, 0]);
            assert_eq!(
                gate.reserved,
                Reserved {
                    work: 32 + paid,
                    bytes: 264
                }
            );
        }
        for bytes in [615, 616] {
            let mut gate = frontier_gate(&mut observer, 1);
            let mut index = FrontierIndex::new(&mut gate, 1).unwrap();
            gate.limits.work = 480;
            gate.limits.bytes = bytes;
            let mut frontier = Vec::new();
            let result = index.add(&mut gate, &mut frontier, &mono(1, &[]));
            assert_eq!(gate.reserved.work, 480);
            if bytes == 615 {
                assert_eq!(result, Err(Stop::StorageLimit));
                assert!(frontier.is_empty());
                assert_eq!(index.slots, [0, 0]);
                assert_eq!(gate.reserved.bytes, 264);
            } else {
                assert_eq!(result, Ok(()));
                assert_eq!(gate.reserved.bytes, 616);
                assert_frontier_index(&index, &frontier);
            }
        }
    }
    #[test]
    fn dynamic_f4_frontier_growth_refusal_is_transactional_and_probe_bounded() {
        let _reset = FrontierTestReset;
        FRONTIER_TEST_HASH.with(|v| v.set(Some(7)));
        let mut observer = |_: Event| Ok(());
        let mut gate = frontier_gate(&mut observer, 4);
        let mut index = FrontierIndex::new(&mut gate, 1).unwrap();
        let mut frontier = Vec::with_capacity(1);
        index.add(&mut gate, &mut frontier, &mono(1, &[])).unwrap();
        // Fill precisely the actual reported capacity, without assuming allocator behavior.
        while frontier.len() < frontier.capacity() {
            let value = frontier.len() as u16;
            index
                .add(&mut gate, &mut frontier, &mono(1, &[(0, value)]))
                .unwrap();
        }
        assert!(frontier.len() < 4);
        let before = frontier.clone();
        let slots = index.slots.clone();
        let before_work = gate.reserved.work;
        let lookup_and_append = 128 + 160 * (frontier.len() as u64 + 1) + 160;
        gate.limits.work = before_work + lookup_and_append + frontier.len() as u64 - 1;
        assert_eq!(
            index.add(&mut gate, &mut frontier, &mono(1, &[(0, 31)])),
            Err(Stop::WorkLimit)
        );
        assert_eq!(gate.reserved.work, before_work + lookup_and_append);
        assert_eq!(frontier, before);
        assert_eq!(index.slots, slots);
        gate.limits.work = MAX_WORK;
        gate.limits.bytes = gate.reserved.bytes;
        assert_eq!(
            index.add(&mut gate, &mut frontier, &mono(1, &[(0, 31)])),
            Err(Stop::StorageLimit)
        );
        assert_eq!(frontier, before);
        assert_eq!(index.slots, slots);
        gate.limits.bytes = MAX_BYTES;
        index
            .add(&mut gate, &mut frontier, &mono(1, &[(0, 31)]))
            .unwrap();
        assert_frontier_index(&index, &frontier);
        // Corrupt diagnostic state must fail finitely, not reseed or fall back.
        index.slots.fill(1);
        let before_work = gate.reserved.work;
        assert_eq!(
            index.add(&mut gate, &mut frontier, &mono(1, &[(0, 99)])),
            Err(Stop::InvalidInput)
        );
        assert_eq!(gate.reserved.work - before_work, 128 + 8 * 160);
        index.slots[7] = u32::MAX;
        assert_eq!(
            index.add(&mut gate, &mut frontier, &mono(1, &[(0, 99)])),
            Err(Stop::InvalidInput)
        );
    }
    #[test]
    fn dynamic_f4_frontier_deadlines_cover_initialization_hash_probe_and_equality() {
        let _reset = FrontierTestReset;
        let mut observer = |_: Event| Ok(());
        let mut gate = frontier_gate(&mut observer, 64);
        gate.deadline = Instant::now();
        assert!(matches!(
            FrontierIndex::new(&mut gate, 28),
            Err(Stop::Deadline)
        ));
        assert_eq!(gate.reserved, Reserved::default());
        FRONTIER_TEST_EXPIRY.with(|v| v.set(Some((FrontierCheckpoint::Initialize, 64))));
        let mut gate = frontier_gate(&mut observer, 64);
        assert!(matches!(
            FrontierIndex::new(&mut gate, 28),
            Err(Stop::Deadline)
        ));
        assert_eq!(
            gate.reserved,
            Reserved {
                work: 16 * 128,
                bytes: 128 + 128 + 4 * 128
            }
        );
        FRONTIER_TEST_EXPIRY.with(|v| v.set(None));
        FRONTIER_TEST_HASH.with(|v| v.set(Some(0)));
        for (checkpoint, at, distinct) in [
            (FrontierCheckpoint::Hash, 27, false),
            (FrontierCheckpoint::Probe, 1, true),
            (FrontierCheckpoint::Compare, 27, false),
        ] {
            let mut gate = frontier_gate(&mut observer, 64);
            let mut index = FrontierIndex::new(&mut gate, 28).unwrap();
            let mut frontier = Vec::new();
            let m = mono(28, &[(27, u16::MAX)]);
            index.add(&mut gate, &mut frontier, &m).unwrap();
            let old = frontier.clone();
            let slots = index.slots.clone();
            FRONTIER_TEST_EXPIRY.with(|v| v.set(Some((checkpoint, at))));
            let next = if distinct { mono(28, &[(0, 1)]) } else { m };
            assert_eq!(
                index.add(&mut gate, &mut frontier, &next),
                Err(Stop::Deadline)
            );
            assert_eq!(frontier, old);
            assert_eq!(index.slots, slots);
            FRONTIER_TEST_EXPIRY.with(|v| v.set(None));
        }
    }
    #[test]
    fn dynamic_f4_frontier_collision_probe_exact_one_less_and_full_duplicate() {
        let _reset = FrontierTestReset;
        FRONTIER_TEST_HASH.with(|v| v.set(Some(3)));
        let mut observer = |_: Event| Ok(());
        let mut gate = frontier_gate(&mut observer, 2);
        let mut index = FrontierIndex::new(&mut gate, 1).unwrap();
        let mut frontier = Vec::with_capacity(4);
        let first = mono(1, &[]);
        let second = mono(1, &[(0, 1)]);
        index.add(&mut gate, &mut frontier, &first).unwrap();
        let slots = index.slots.clone();
        let before = gate.reserved.work;
        gate.limits.work = before + 128 + 160 + 159;
        assert_eq!(
            index.add(&mut gate, &mut frontier, &second),
            Err(Stop::WorkLimit)
        );
        assert_eq!(gate.reserved.work, before + 128 + 160);
        assert_eq!(index.slots, slots);
        assert_eq!(frontier, [first]);
        let before = gate.reserved.work;
        gate.limits.work = before + 128 + 2 * 160 + 160;
        index.add(&mut gate, &mut frontier, &second).unwrap();
        assert_eq!(gate.reserved.work, gate.limits.work);
        let slots = index.slots.clone();
        let before = gate.reserved.work;
        gate.limits.work = before + 128 + 2 * 160 - 1;
        assert_eq!(
            index.add(&mut gate, &mut frontier, &second),
            Err(Stop::WorkLimit)
        );
        assert_eq!(gate.reserved.work, before + 128 + 160);
        let before = gate.reserved.work;
        gate.limits.work = before + 128 + 2 * 160;
        index.add(&mut gate, &mut frontier, &second).unwrap();
        assert_eq!(gate.reserved.work, gate.limits.work);
        assert_eq!(index.slots, slots);
        assert_eq!(frontier, [first, second]);
    }
    #[test]
    fn dynamic_f4_frontier_native_completed_differential_preserves_batches_and_basis() {
        let _reset = FrontierTestReset;
        for v in [2, 4, 8] {
            let f = field(17);
            let input = fixed(v, 17);
            let limits = Limits::default();
            let mut outcomes = Vec::new();
            for legacy in [true, false] {
                FRONTIER_TEST_LEGACY.with(|flag| flag.set(legacy));
                FRONTIER_TEST_BATCHES.with(|batches| *batches.borrow_mut() = Some(Vec::new()));
                let mut stages = Vec::new();
                let report = run(&f, v as u8, &input, &limits, deadline(), &mut |event| {
                    stages.push((event.stage, event.sequence));
                    Ok(())
                });
                let batches = FRONTIER_TEST_BATCHES.with(|b| b.borrow_mut().take().unwrap());
                assert!(matches!(report.stop, Stop::Complete | Stop::UnitDerived));
                assert!(report.native_statistics_complete);
                assert!(report.matrix_rows_reduced > 0);
                assert!(report.pivot_updates > 0);
                check_linear_ideal(&report, v, 17);
                outcomes.push((report, stages, batches));
            }
            let (a, stages_a, batches_a) = &outcomes[0];
            let (b, stages_b, batches_b) = &outcomes[1];
            assert_eq!(a.basis, b.basis);
            assert_eq!(a.stop, b.stop);
            assert_eq!(stages_a, stages_b);
            assert_eq!(batches_a, batches_b);
            assert_eq!(
                (
                    a.matrix_rows_reduced,
                    a.pivot_updates,
                    a.pairs_pending,
                    a.pairs_selected_unfinished,
                    a.maximum_rows_admitted,
                    a.maximum_columns_admitted,
                    a.maximum_pending_pairs,
                    a.completed_steps,
                    a.maximum_input_degree,
                    a.maximum_processed_row_degree
                ),
                (
                    b.matrix_rows_reduced,
                    b.pivot_updates,
                    b.pairs_pending,
                    b.pairs_selected_unfinished,
                    b.maximum_rows_admitted,
                    b.maximum_columns_admitted,
                    b.maximum_pending_pairs,
                    b.completed_steps,
                    b.maximum_input_degree,
                    b.maximum_processed_row_degree
                )
            );
            assert_ne!(a.reserved, b.reserved); // different reservation model, not measured speed
        }
    }
    fn deadline() -> Instant {
        Instant::now() + Duration::from_secs(5)
    }
    fn field(p: u64) -> Fp64Arithmetic {
        Fp64Arithmetic::try_new(p, MAX_WORK, MAX_BYTES, deadline(), &mut |_| Ok(()))
            .unwrap()
            .field
    }
    fn mono(v: usize, powers: &[(usize, u16)]) -> Mono28 {
        let mut exponents = vec![0; v];
        for &(i, e) in powers {
            exponents[i] = e;
        }
        Mono28::new(&exponents, v).unwrap()
    }
    fn poly(v: usize, raw: Vec<(Mono28, u64)>) -> Poly {
        let mut terms: Vec<_> = raw
            .into_iter()
            .filter(|(_, c)| *c != 0)
            .map(|(mono, coefficient)| CanonicalTerm { mono, coefficient })
            .collect();
        terms.sort_by(|a, b| b.mono.cmp(&a.mono, v));
        Poly { terms }
    }
    fn fixed(v: usize, p: u64) -> Vec<Poly> {
        let t = v - 1;
        let mut out = vec![
            poly(
                v,
                vec![(mono(v, &[(0, 1), (t, 1)]), 1), (mono(v, &[]), p - 1)],
            ),
            poly(
                v,
                vec![(mono(v, &[(t, 2)]), 1), (mono(v, &[(t, 1)]), p - 1)],
            ),
        ];
        for i in 1..t {
            out.push(poly(
                v,
                vec![(mono(v, &[(i, 1)]), 1), (mono(v, &[(t, 1)]), p - 1)],
            ));
        }
        out
    }
    fn reference_poly(p: &Poly, v: usize, modulus: u64) -> MPoly {
        let modulus = BigUint::from(modulus);
        let mut out = MPoly::zero(v, modulus.clone());
        for t in &p.terms {
            out.terms.insert(
                t.mono
                    .exponents(v)
                    .unwrap()
                    .iter()
                    .map(|&e| u32::from(e))
                    .collect(),
                FieldElement::new(BigUint::from(t.coefficient), modulus.clone()),
            );
        }
        out
    }
    fn check_linear_ideal(report: &Report, v: usize, p: u64) {
        let returned: Vec<_> = report
            .basis
            .iter()
            .map(|f| reference_poly(f, v, p))
            .collect();
        let expected: Vec<_> = (0..v)
            .map(|i| {
                reference_poly(
                    &poly(v, vec![(mono(v, &[(i, 1)]), 1), (mono(v, &[]), p - 1)]),
                    v,
                    p,
                )
            })
            .collect();
        for g in &returned {
            assert!(reference::reduce(g, &expected, reference::Ordering::Grevlex).is_zero());
        }
        for g in &expected {
            assert!(reference::reduce(g, &returned, reference::Ordering::Grevlex).is_zero());
        }
        for a in 0..returned.len() {
            for b in a + 1..returned.len() {
                let s = reference::s_polynomial(
                    &returned[a],
                    &returned[b],
                    reference::Ordering::Grevlex,
                );
                assert!(reference::reduce(&s, &returned, reference::Ordering::Grevlex).is_zero());
            }
        }
    }
    #[test]
    fn dynamic_f4_every_arity_has_useful_matrix_work_and_exact_reference_ideal() {
        for p in [
            17,
            140_737_488_355_213,
            9_007_199_254_740_881,
            2_305_843_009_213_693_951,
        ] {
            let f = field(p);
            for m in 2..=15 {
                let v = 2 * m - 2;
                let input = fixed(v, p);
                let limits = Limits {
                    work: MAX_WORK - 131_072,
                    bytes: MAX_BYTES - 4096,
                    ..Limits::default()
                };
                let result = run(&f, v as u8, &input, &limits, deadline(), &mut |_| Ok(()));
                assert_eq!(result.stop, Stop::Complete, "m={m},p={p},report={result:?}");
                assert!(result.native_statistics_complete && result.native_backend_invoked);
                assert!(
                    result.matrix_rows_reduced > 0 && result.pivot_updates > 0,
                    "m={m},p={p}"
                );
                assert_eq!(result.pairs_pending, 0);
                assert_eq!(result.pairs_selected_unfinished, 0);
                check_linear_ideal(&result, v, p);
                if p == 17 {
                    let original: Vec<_> = input.iter().map(|g| reference_poly(g, v, p)).collect();
                    let oracle = reference::buchberger(&original, reference::Ordering::Grevlex);
                    for g in &result.basis {
                        assert!(reference::reduce(
                            &reference_poly(g, v, p),
                            &oracle,
                            reference::Ordering::Grevlex
                        )
                        .is_zero());
                    }
                }
                let mut negative = input;
                negative.push(poly(v, vec![(mono(v, &[(v - 1, 1)]), 1)]));
                let result = run(
                    &f,
                    v as u8,
                    &negative,
                    &Limits::default(),
                    deadline(),
                    &mut |_| Ok(()),
                );
                assert_eq!(result.stop, Stop::UnitDerived, "negative m={m},p={p}");
                assert!(result.basis.iter().any(|g| g.terms.len() == 1
                    && g.terms[0].mono.degree() == 0
                    && g.terms[0].coefficient == 1));
            }
        }
    }
    #[test]
    fn dynamic_f4_all_coordinates_order_products_and_overflow_are_checked() {
        for v in 1..=28 {
            let a = mono(v, &[(v - 1, 2)]);
            let b = mono(v, &[(0, 1)]);
            let av: Vec<_> = a
                .exponents(v)
                .unwrap()
                .iter()
                .map(|&e| u32::from(e))
                .collect();
            let bv: Vec<_> = b
                .exponents(v)
                .unwrap()
                .iter()
                .map(|&e| u32::from(e))
                .collect();
            assert_eq!(
                a.cmp(&b, v),
                reference::cmp_monomial(&av, &bv, reference::Ordering::Grevlex)
            );
            let product = a.product(b, v).unwrap();
            assert_eq!(product.degree(), 3);
            assert_eq!(product.quotient(a, v), Some(b));
            assert!(a.divides(&product, v));
            assert!(mono(v, &[(v - 1, u16::MAX)])
                .product(mono(v, &[(v - 1, 1)]), v)
                .is_none());
            assert!(Mono28::new(&vec![0; v + 1], v).is_none());
        }
        let f = field(17);
        let mut bad = fixed(2, 17);
        bad[0].terms[0].mono.exponents[27] = 1;
        let mut calls = 0;
        let result = run(&f, 2, &bad, &Limits::default(), deadline(), &mut |_| {
            calls += 1;
            Ok(())
        });
        assert_eq!(result.stop, Stop::InvalidInput);
        assert_eq!(calls, 0);
    }
    #[test]
    fn dynamic_f4_exact_work_storage_and_counters_match_terminal() {
        let f = field(17);
        let input = fixed(4, 17);
        let mut terminal = None;
        let base = run(&f, 4, &input, &Limits::default(), deadline(), &mut |e| {
            if e.stage == Stage::Terminal {
                terminal = Some(e.reserved);
            }
            Ok(())
        });
        assert_eq!(base.stop, Stop::Complete);
        assert_eq!(terminal, Some(base.reserved));
        let limits = Limits {
            work: base.reserved.work,
            bytes: base.reserved.bytes,
            ..Limits::default()
        };
        let exact = run(&f, 4, &input, &limits, deadline(), &mut |_| Ok(()));
        assert_eq!(exact.stop, Stop::Complete);
        assert_eq!(exact.reserved, base.reserved);
        for (limits, stop) in [
            (
                Limits {
                    work: limits.work - 1,
                    ..limits
                },
                Stop::WorkLimit,
            ),
            (
                Limits {
                    bytes: limits.bytes - 1,
                    ..limits
                },
                Stop::StorageLimit,
            ),
        ] {
            let short = run(&f, 4, &input, &limits, deadline(), &mut |_| Ok(()));
            assert_eq!(short.stop, stop);
            assert!(!short.native_statistics_complete);
        }
    }
    #[test]
    fn dynamic_f4_exact_structural_caps_and_one_less() {
        let f = field(17);
        let input = fixed(4, 17);
        let base = run(&f, 4, &input, &Limits::default(), deadline(), &mut |_| {
            Ok(())
        });
        assert_eq!(base.stop, Stop::Complete);
        let exact = Limits {
            rows: base.maximum_rows_admitted,
            columns: base.maximum_columns_admitted,
            basis: base.basis.len(),
            pairs: base.maximum_pending_pairs,
            steps: base.completed_steps,
            events: base.acknowledged_events,
            max_pair_degree: 3,
            ..Limits::default()
        };
        assert_eq!(
            run(&f, 4, &input, &exact, deadline(), &mut |_| Ok(())).stop,
            Stop::Complete
        );
        for (limits, stop) in [
            (
                Limits {
                    rows: exact.rows - 1,
                    ..exact
                },
                Stop::RowLimit,
            ),
            (
                Limits {
                    columns: exact.columns - 1,
                    ..exact
                },
                Stop::ColumnLimit,
            ),
            (
                Limits {
                    basis: exact.basis - 1,
                    ..exact
                },
                Stop::BasisLimit,
            ),
            (
                Limits {
                    pairs: exact.pairs - 1,
                    ..exact
                },
                Stop::PairLimit,
            ),
            (
                Limits {
                    steps: exact.steps - 1,
                    ..exact
                },
                Stop::StepLimit,
            ),
            (
                Limits {
                    events: exact.events - 1,
                    ..exact
                },
                Stop::EventLimit,
            ),
            (
                Limits {
                    max_pair_degree: 2,
                    ..exact
                },
                Stop::PairDegreeLimit,
            ),
        ] {
            let r = run(&f, 4, &input, &limits, deadline(), &mut |_| Ok(()));
            assert_eq!(r.stop, stop);
            assert!(!r.native_statistics_complete);
        }
    }
    #[test]
    fn dynamic_f4_limits_never_promote_partial_completion() {
        let f = field(17);
        let input = fixed(4, 17);
        for (limits, stop) in [
            (
                Limits {
                    rows: 0,
                    ..Limits::default()
                },
                Stop::RowLimit,
            ),
            (
                Limits {
                    columns: 0,
                    ..Limits::default()
                },
                Stop::ColumnLimit,
            ),
            (
                Limits {
                    basis: 0,
                    ..Limits::default()
                },
                Stop::BasisLimit,
            ),
            (
                Limits {
                    pairs: 0,
                    ..Limits::default()
                },
                Stop::PairLimit,
            ),
            (
                Limits {
                    steps: 0,
                    ..Limits::default()
                },
                Stop::StepLimit,
            ),
            (
                Limits {
                    events: 0,
                    ..Limits::default()
                },
                Stop::EventLimit,
            ),
            (
                Limits {
                    max_pair_degree: 0,
                    ..Limits::default()
                },
                Stop::PairDegreeLimit,
            ),
        ] {
            let r = run(&f, 4, &input, &limits, deadline(), &mut |_| Ok(()));
            assert_eq!(r.stop, stop);
            assert!(!r.native_statistics_complete);
            if matches!(stop, Stop::ColumnLimit) {
                assert!(r.pairs_selected_unfinished > 0 && r.pairs_pending > 0);
            }
        }
    }
    #[test]
    fn dynamic_f4_sticky_observer_deadline_and_invalid_input() {
        let f = field(17);
        let input = fixed(4, 17);
        for stage in [
            Stage::Begin,
            Stage::Initial,
            Stage::Basis,
            Stage::Batch,
            Stage::Terminal,
        ] {
            let mut failed = false;
            let r = run(&f, 4, &input, &Limits::default(), deadline(), &mut |e| {
                assert!(!failed, "callback after refusal");
                if e.stage == stage {
                    failed = true;
                    Err(())
                } else {
                    Ok(())
                }
            });
            assert!(failed);
            assert_eq!(r.stop, Stop::Observer);
            assert!(!r.native_statistics_complete);
        }
        let mut calls = 0;
        let r = run(
            &f,
            4,
            &input,
            &Limits::default(),
            Instant::now(),
            &mut |_| {
                calls += 1;
                Ok(())
            },
        );
        assert_eq!(r.stop, Stop::Deadline);
        assert_eq!(calls, 0);
        assert!(!r.native_backend_invoked);
        for coefficient in [0, 17, u64::MAX] {
            // All malformed cases are checked before any callback.
            let mut bad = input.clone();
            bad[0].terms[0].coefficient = coefficient;
            let r = run(&f, 4, &bad, &Limits::default(), deadline(), &mut |_| {
                panic!("invalid callback")
            });
            assert_eq!(r.stop, Stop::InvalidInput);
        }
        let mut bad = input.clone();
        bad[0].terms.reverse();
        assert_eq!(
            run(&f, 4, &bad, &Limits::default(), deadline(), &mut |_| Ok(())).stop,
            Stop::InvalidInput
        );
    }
    #[test]
    fn dynamic_f4_invocation_requires_actual_admitted_arithmetic() {
        let f = field(17);
        let input = fixed(4, 17);
        for limits in [
            Limits {
                work: 1472,
                ..Limits::default()
            },
            Limits {
                work: 1504,
                ..Limits::default()
            },
            Limits {
                bytes: 4608,
                ..Limits::default()
            },
        ] {
            let mut stages = Vec::new();
            let r = run(&f, 4, &input, &limits, deadline(), &mut |e| {
                stages.push(e.stage);
                Ok(())
            });
            assert!(matches!(r.stop, Stop::WorkLimit | Stop::StorageLimit));
            assert_eq!(stages, vec![Stage::Begin, Stage::Initial]);
            assert!(!r.native_backend_invoked);
            assert_eq!(r.matrix_rows_reduced, 0);
            assert!(!r.native_statistics_complete);
        }
        let r = run(
            &f,
            4,
            &input,
            &Limits {
                work: 1568,
                ..Limits::default()
            },
            deadline(),
            &mut |_| Ok(()),
        );
        assert_eq!(r.stop, Stop::WorkLimit);
        assert!(r.native_backend_invoked);
        assert_eq!(r.matrix_rows_reduced, 0);
        assert!(!r.native_statistics_complete);
        for input in [vec![], vec![Poly { terms: vec![] }]] {
            let r = run(&f, 1, &input, &Limits::default(), deadline(), &mut |_| {
                Ok(())
            });
            assert_eq!(r.stop, Stop::Complete);
            assert!(!r.native_backend_invoked);
        }
    }
    #[test]
    fn dynamic_f4_validation_preserves_adjacency_across_chunk_boundary() {
        let f = field(17);
        let raw = (0..17)
            .flat_map(|a| (0..17).map(move |b| (mono(2, &[(0, a), (1, b)]), 1)))
            .collect();
        let mut p = poly(2, raw);
        p.terms.truncate(257);
        let r = run(
            &f,
            2,
            &[p.clone()],
            &Limits {
                events: 0,
                ..Limits::default()
            },
            deadline(),
            &mut |_| panic!("event cap"),
        );
        assert_eq!(r.stop, Stop::EventLimit); // All 257 canonical terms were admitted.
        p.terms.swap(255, 256);
        let r = run(&f, 2, &[p], &Limits::default(), deadline(), &mut |_| {
            panic!("invalid cross-chunk order")
        });
        assert_eq!(r.stop, Stop::InvalidInput);
        assert!(!r.native_backend_invoked);
    }
    #[test]
    fn dynamic_f4_acknowledged_callback_expiry_overrides_success() {
        let f = field(17);
        let input = fixed(4, 17);
        for target in [Stage::Initial, Stage::Terminal] {
            let original = Instant::now() + Duration::from_secs(1);
            let mut reached = false;
            let mut calls = 0;
            let mut accepted = None;
            let r = run(&f, 4, &input, &Limits::default(), original, &mut |e| {
                assert!(!reached, "callback after expired acknowledged boundary");
                calls += 1;
                if e.stage == target {
                    reached = true;
                    accepted = Some(e.reserved);
                    while Instant::now() < original {
                        std::thread::yield_now();
                    }
                }
                Ok(())
            });
            assert!(
                reached,
                "intended boundary {target:?} was not reached: {r:?}"
            );
            assert_eq!(r.stop, Stop::Deadline);
            assert!(!r.native_statistics_complete);
            assert_eq!(r.acknowledged_events, calls);
            assert_eq!(Some(r.reserved), accepted);
            assert_eq!(r.native_backend_invoked, target == Stage::Terminal);
            if target == Stage::Terminal {
                assert!(r.pivot_updates > 0);
            }
        }
    }
    #[test]
    fn dynamic_f4_zero_constant_nonmonic_and_high_input_degree() {
        let f = field(17);
        let zero = run(
            &f,
            1,
            &[Poly { terms: vec![] }],
            &Limits::default(),
            deadline(),
            &mut |_| Ok(()),
        );
        assert_eq!(zero.stop, Stop::Complete);
        assert!(zero.basis.is_empty());
        let unit = run(
            &f,
            1,
            &[poly(1, vec![(mono(1, &[]), 16)])],
            &Limits::default(),
            deadline(),
            &mut |_| Ok(()),
        );
        assert_eq!(unit.stop, Stop::UnitDerived);
        assert_eq!(unit.basis[0].terms[0].coefficient, 1);
        let p = poly(1, vec![(mono(1, &[(0, 32)]), 16), (mono(1, &[]), 1)]);
        let high = run(&f, 1, &[p], &Limits::default(), deadline(), &mut |_| Ok(()));
        assert_eq!(high.stop, Stop::Complete);
        assert_eq!(high.maximum_input_degree, 32);
        assert_eq!(high.maximum_processed_row_degree, 32);
        assert_eq!(high.basis[0].terms[0].coefficient, 1);
    }
    struct InverseTestReset;
    impl Drop for InverseTestReset {
        fn drop(&mut self) {
            INVERSE_TEST_LEGACY.with(|f| f.set(false));
            INVERSE_TEST_OPS.with(|o| o.set((0, 0)));
        }
    }
    fn inverse_control(
        field: &Fp64Arithmetic,
        a: u64,
        work: u64,
        until: Instant,
    ) -> (Result<u64, Stop>, Reserved, (u64, u64)) {
        let mut observer = |_: Event| Ok(());
        let mut e = Engine {
            field,
            v: 1,
            gate: Gate {
                limits: Limits {
                    work,
                    ..Limits::default()
                },
                reserved: Reserved::default(),
                deadline: until,
                events: 0,
                observer: &mut observer,
            },
            basis: Vec::new(),
            pairs: Vec::new(),
            selected: 0,
            invoked: false,
            matrix_rows: 0,
            updates: 0,
            max_row_degree: 0,
            peak_rows: 0,
            peak_columns: 0,
            peak_pairs: 0,
            steps: 0,
        };
        INVERSE_TEST_OPS.with(|o| o.set((0, 0)));
        let result = e.inverse(a);
        (result, e.gate.reserved, INVERSE_TEST_OPS.with(|o| o.get()))
    }
    #[test]
    fn dynamic_f4_unit_inverse_exact_limits_deadline_and_montgomery_identity() {
        let _reset = InverseTestReset;
        for p in [101, (1u64 << 61) - 1] {
            let f = field(p);
            assert_ne!(f.one(), 1);
            assert_eq!(f.to_u64(f.one()), 1);
            for (work, expected, charged) in [(31, Err(Stop::WorkLimit), 0), (32, Ok(f.one()), 32)]
            {
                let (result, reserved, ops) = inverse_control(&f, f.one(), work, deadline());
                assert_eq!(result, expected);
                assert_eq!(reserved.work, charged);
                assert_eq!(reserved.bytes, 0);
                assert_eq!(ops, (0, 0));
            }
            let a = f.from_u64(2);
            let (result, reserved, ops) = inverse_control(&f, a, 4127, deadline());
            assert_eq!(result, Err(Stop::WorkLimit));
            assert_eq!(reserved.work, 32);
            assert_eq!(ops, (0, 0));
            let (result, reserved, ops) = inverse_control(&f, a, 4128, deadline());
            assert_eq!(f.mul(a, result.unwrap()), f.one());
            assert_eq!(reserved.work, 4128);
            assert_eq!(reserved.bytes, 0);
            assert!(ops.0 > 0 && ops.1 > 0);
            let (result, reserved, ops) = inverse_control(&f, 1, 4128, deadline());
            assert_eq!(f.mul(1, result.unwrap()), f.one());
            assert_eq!(reserved.work, 4128);
            assert!(ops.1 > 0);
            let (result, reserved, ops) = inverse_control(&f, f.one(), MAX_WORK, Instant::now());
            assert_eq!(result, Err(Stop::Deadline));
            assert_eq!(reserved.work, 0);
            assert_eq!(ops, (0, 0));
            let (result, reserved, ops) = inverse_control(&f, 0, 0, Instant::now());
            assert_eq!(result, Err(Stop::InvalidInput));
            assert_eq!(reserved.work, 0);
            assert_eq!(ops, (0, 0));
        }
    }
    #[test]
    fn dynamic_f4_inverse_general_path_matches_legacy_operations() {
        let _reset = InverseTestReset;
        for p in [101, (1u64 << 61) - 1] {
            let f = field(p);
            for a in [1, f.from_u64(2), f.from_u64(7), f.from_u64(p - 1), f.one()] {
                INVERSE_TEST_LEGACY.with(|v| v.set(true));
                let old = inverse_control(&f, a, MAX_WORK, deadline());
                INVERSE_TEST_LEGACY.with(|v| v.set(false));
                let new = inverse_control(&f, a, MAX_WORK, deadline());
                assert_eq!(old.0, new.0);
                assert_eq!(f.mul(a, new.0.unwrap()), f.one());
                assert_eq!(old.1.work, 4096);
                assert_eq!(new.1.bytes, 0);
                if a == f.one() {
                    assert_eq!(new.1.work, 32);
                    assert_eq!(new.2, (0, 0));
                    assert!(old.2 .1 > 0);
                } else {
                    assert_eq!(new.1.work, 4128);
                    assert_eq!(new.2, old.2);
                }
            }
        }
    }
    #[test]
    fn dynamic_f4_unit_inverse_preserves_basis_batches_and_pivot_semantics() {
        let _reset = InverseTestReset;
        let _frontier_reset = FrontierTestReset;
        for v in [2, 4] {
            let f = field(17);
            let input = fixed(v, 17);
            let mut results = Vec::new();
            for legacy in [true, false] {
                INVERSE_TEST_LEGACY.with(|b| b.set(legacy));
                FRONTIER_TEST_BATCHES.with(|b| *b.borrow_mut() = Some(Vec::new()));
                let mut stages = Vec::new();
                let report = run(
                    &f,
                    v as u8,
                    &input,
                    &Limits::default(),
                    deadline(),
                    &mut |event| {
                        stages.push((event.stage, event.sequence));
                        Ok(())
                    },
                );
                let batches = FRONTIER_TEST_BATCHES.with(|b| b.borrow_mut().take().unwrap());
                assert!(matches!(report.stop, Stop::Complete | Stop::UnitDerived));
                check_linear_ideal(&report, v, 17);
                results.push((report, stages, batches));
            }
            let (a, sa, ba) = &results[0];
            let (b, sb, bb) = &results[1];
            assert_eq!(a.basis, b.basis);
            assert_eq!(a.stop, b.stop);
            assert_eq!(sa, sb);
            assert_eq!(ba, bb);
            assert_eq!(
                (
                    a.matrix_rows_reduced,
                    a.pivot_updates,
                    a.pairs_pending,
                    a.pairs_selected_unfinished,
                    a.completed_steps
                ),
                (
                    b.matrix_rows_reduced,
                    b.pivot_updates,
                    b.pairs_pending,
                    b.pairs_selected_unfinished,
                    b.completed_steps
                )
            );
            assert_eq!(a.reserved.bytes, b.reserved.bytes);
        }
    }
}
