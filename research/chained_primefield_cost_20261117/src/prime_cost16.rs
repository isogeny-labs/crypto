//! Prospective17-site source-operation accounting, not a timing measurement.
use num_bigint::BigUint;
use num_traits::ToPrimitive;
use std::cell::RefCell;

pub(crate) const WEIGHTS: [[u64; 6]; 17] = [
    [0, 0, 0, 1, 0, 0], // constant reduction
    [0, 0, 0, 1, 0, 0], // coefficient reduction
    [0, 1, 0, 1, 0, 0], // polynomial addition
    [0, 1, 1, 1, 0, 0], // polynomial subtraction: lhs+p-rhs
    [0, 0, 1, 1, 0, 0], // nonzero coefficient negation
    [1, 0, 0, 1, 0, 0], // polynomial coefficient product
    [0, 1, 0, 1, 0, 0], // polynomial product accumulation
    [0, 0, 0, 1, 0, 0], // scalar reduction
    [1, 0, 0, 1, 0, 0], // scalar product
    [1, 0, 0, 1, 0, 0], // division coefficient
    [0, 1, 0, 1, 0, 0], // division quotient accumulation
    [1, 0, 0, 1, 0, 0], // division subproduct
    [1, 1, 0, 1, 0, 0], // Horner step
    [0, 0, 0, 1, 0, 0], // inverse argument reduction
    [0, 0, 1, 0, 0, 0], // inverse exponent p-2
    [0, 0, 0, 0, 1, 0], // atomic actual BigUint::modpow
    [0, 0, 0, 0, 0, 1], // CubicField::c u64 remainder
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Budget {
    pub(crate) ops: [u64; 6],
    pub(crate) inv_inputs: [u64; 101],
    pub(crate) sites: [u64; 17],
}

impl Budget {
    pub(crate) const fn zero() -> Self {
        Self {
            ops: [0; 6],
            inv_inputs: [0; 101],
            sites: [0; 17],
        }
    }

    fn event(&mut self, site: usize, weights: [u64; 6], input: Option<usize>) {
        self.sites[site] = self.sites[site].checked_add(1).expect("site overflow");
        for (out, weight) in self.ops.iter_mut().zip(weights) {
            *out = out.checked_add(weight).expect("operator overflow");
        }
        if let Some(a) = input {
            self.inv_inputs[a] = self.inv_inputs[a]
                .checked_add(1)
                .expect("histogram overflow");
        }
    }

    pub(crate) fn add(&mut self, other: &Self) {
        for (a, b) in self.ops.iter_mut().zip(other.ops) {
            *a = a.checked_add(b).expect("aggregate overflow");
        }
        for (a, b) in self.inv_inputs.iter_mut().zip(other.inv_inputs) {
            *a = a.checked_add(b).expect("aggregate histogram overflow");
        }
        for (a, b) in self.sites.iter_mut().zip(other.sites) {
            *a = a.checked_add(b).expect("aggregate site overflow");
        }
    }

    pub(crate) fn validate(&self) {
        let mut derived = [0_u64; 6];
        for (count, weights) in self.sites.iter().zip(WEIGHTS) {
            for (out, weight) in derived.iter_mut().zip(weights) {
                *out = out
                    .checked_add(count.checked_mul(weight).expect("weight overflow"))
                    .expect("derived overflow");
            }
        }
        assert_eq!(self.ops, derived, "source-site weight accounting");
        assert_eq!(self.inv_inputs.iter().sum::<u64>(), self.ops[4]);
        assert_eq!(self.sites[15], self.ops[4]);
    }

    pub(crate) fn sparse_inverses(&self) -> Vec<(usize, u64)> {
        self.inv_inputs
            .iter()
            .enumerate()
            .filter_map(|(a, n)| (*n != 0).then_some((a, *n)))
            .collect()
    }

    /// Index: equal, candidate smaller, baseline smaller, incomparable.
    pub(crate) fn comparison(&self, candidate: &Self, expanded: bool) -> usize {
        let mut lower = false;
        let mut higher = false;
        for (i, (base, new)) in self.ops.iter().zip(candidate.ops).enumerate() {
            if expanded && i == 4 {
                continue;
            }
            lower |= new < *base;
            higher |= new > *base;
        }
        if expanded {
            for (base, new) in self.inv_inputs.iter().zip(candidate.inv_inputs) {
                lower |= new < *base;
                higher |= new > *base;
            }
        }
        match (lower, higher) {
            (false, false) => 0,
            (true, false) => 1,
            (false, true) => 2,
            (true, true) => 3,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Snapshot {
    pub(crate) all: Budget,
    pub(crate) inverse: Budget,
}

impl Snapshot {
    pub(crate) const fn zero() -> Self {
        Self {
            all: Budget::zero(),
            inverse: Budget::zero(),
        }
    }

    pub(crate) fn validate(&self) {
        self.all.validate();
        self.inverse.validate();
        assert!(self
            .inverse
            .ops
            .iter()
            .zip(self.all.ops)
            .all(|(inside, all)| *inside <= all));
        assert!(self
            .inverse
            .inv_inputs
            .iter()
            .zip(self.all.inv_inputs)
            .all(|(inside, all)| *inside <= all));
    }

    pub(crate) fn add(&mut self, other: &Self) {
        self.all.add(&other.all);
        self.inverse.add(&other.inverse);
    }
}

struct State {
    enabled: bool,
    corrupt_product: bool,
    depth: u32,
    value: Snapshot,
}

thread_local! {
    static STATE: RefCell<State> = const { RefCell::new(State {
        enabled: false,
        corrupt_product: false,
        depth: 0,
        value: Snapshot::zero(),
    }) };
}

fn event(state: &mut State, site: usize, input: Option<usize>) {
    let mut weights = WEIGHTS[site];
    if state.corrupt_product && site == 5 {
        weights[0] = 0;
    }
    state.value.all.event(site, weights, input);
    if state.depth != 0 {
        state.value.inverse.event(site, weights, input);
    }
}

pub(crate) fn tick(site: usize) {
    STATE.with(|cell| {
        let mut state = cell.borrow_mut();
        if state.enabled {
            assert_ne!(site, 15, "modpow must record its actual argument");
            event(&mut state, site, None);
        }
    });
}

pub(crate) fn modpow_input(a: &BigUint, p: &BigUint) {
    STATE.with(|cell| {
        let mut state = cell.borrow_mut();
        if state.enabled {
            let p = p.to_u64().expect("bounded calibration modulus");
            let a = a.to_usize().expect("bounded reduced inversion argument");
            assert!((p == 7 || p == 101) && a != 0 && (a as u64) < p);
            event(&mut state, 15, Some(a));
        }
    });
}

pub(crate) struct InverseScope(bool);

impl InverseScope {
    pub(crate) fn enter() -> Self {
        Self(STATE.with(|cell| {
            let mut state = cell.borrow_mut();
            if state.enabled {
                state.depth = state.depth.checked_add(1).expect("inverse depth overflow");
                true
            } else {
                false
            }
        }))
    }
}

impl Drop for InverseScope {
    fn drop(&mut self) {
        if self.0 {
            STATE.with(|cell| {
                let mut state = cell.borrow_mut();
                state.depth = state.depth.checked_sub(1).expect("inverse depth underflow");
            });
        }
    }
}

struct CaptureScope;

impl Drop for CaptureScope {
    fn drop(&mut self) {
        STATE.with(|cell| {
            let mut state = cell.borrow_mut();
            state.enabled = false;
            state.corrupt_product = false;
        });
    }
}

pub(crate) fn assert_idle() {
    STATE.with(|cell| {
        let state = cell.borrow();
        assert!(!state.enabled && !state.corrupt_product && state.depth == 0);
    });
}

pub(crate) fn capture<T>(corrupt_product: bool, operation: impl FnOnce() -> T) -> (T, Snapshot) {
    assert_idle();
    STATE.with(|cell| {
        let mut state = cell.borrow_mut();
        state.enabled = true;
        state.corrupt_product = corrupt_product;
        state.value = Snapshot::zero();
    });
    let guard = CaptureScope;
    let out = operation();
    let value = STATE.with(|cell| {
        let state = cell.borrow();
        assert_eq!(state.depth, 0, "all inverse scopes must close");
        state.value
    });
    drop(guard);
    assert_idle();
    (out, value)
}
