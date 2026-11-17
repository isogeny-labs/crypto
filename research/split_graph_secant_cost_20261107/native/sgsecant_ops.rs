//! Exact operation ledger. Every F_p operation of the instrument goes through
//! these functions so that a count is a count of what was executed.
//! Kinds: mul, add (add and sub), inv (one modular inversion, Fermat or
//! extended gcd costs are NOT folded in; see PROTOCOL for the weighted variant),
//! native (non-field work: monomial compares, divisibility tests, hash/set
//! probes, trial divisions, hash calls). Phases separate construction from
//! per-target work so no phase can hide.
use std::cell::Cell;

pub const PHASES: [&str; 11] = [
    "construct", "target", "build", "groebner", "fglm", "roots", "replay", "dedup", "la", "rho",
    "verify",
];
pub const PH_CONSTRUCT: usize = 0;
pub const PH_TARGET: usize = 1;
pub const PH_BUILD: usize = 2;
pub const PH_GB: usize = 3;
pub const PH_FGLM: usize = 4;
pub const PH_ROOTS: usize = 5;
pub const PH_REPLAY: usize = 6;
pub const PH_DEDUP: usize = 7;
pub const PH_LA: usize = 8;
pub const PH_RHO: usize = 9;
pub const PH_VERIFY: usize = 10;

thread_local! {
    static CT: [Cell<u64>; 44] = const { [const { Cell::new(0) }; 44] };
    static PH: Cell<usize> = const { Cell::new(0) };
    // 0 = unlimited. Total mul+add+inv allowed since `set_budget`.
    static LIMIT: Cell<u64> = const { Cell::new(0) };
}

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct Led {
    pub mul: u64,
    pub add: u64,
    pub inv: u64,
    pub native: u64,
}
impl Led {
    pub fn field(&self) -> u64 {
        self.mul + self.add + self.inv
    }
    pub fn plus(&self, o: &Led) -> Led {
        Led { mul: self.mul + o.mul, add: self.add + o.add, inv: self.inv + o.inv, native: self.native + o.native }
    }
    pub fn minus(&self, o: &Led) -> Led {
        Led { mul: self.mul - o.mul, add: self.add - o.add, inv: self.inv - o.inv, native: self.native - o.native }
    }
}
pub type Snap = [Led; 11];

pub fn phase(i: usize) -> usize {
    PH.with(|c| c.replace(i))
}
pub fn reset() {
    CT.with(|c| c.iter().for_each(|x| x.set(0)));
}
#[inline]
fn bump(kind: usize, n: u64) {
    let ph = PH.with(|c| c.get());
    CT.with(|c| {
        let cell = &c[ph * 4 + kind];
        cell.set(cell.get() + n)
    });
}
#[inline]
pub fn mul_n(n: u64) {
    bump(0, n)
}
#[inline]
pub fn add_n(n: u64) {
    bump(1, n)
}
#[inline]
pub fn inv_n(n: u64) {
    bump(2, n)
}
#[inline]
pub fn native_n(n: u64) {
    bump(3, n)
}
pub fn snapshot() -> Snap {
    let mut s = [Led::default(); 11];
    CT.with(|c| {
        for (i, l) in s.iter_mut().enumerate() {
            *l = Led { mul: c[i * 4].get(), add: c[i * 4 + 1].get(), inv: c[i * 4 + 2].get(), native: c[i * 4 + 3].get() };
        }
    });
    s
}
pub fn diff(a: &Snap, b: &Snap) -> Snap {
    let mut s = [Led::default(); 11];
    for i in 0..11 {
        s[i] = a[i].minus(&b[i]);
    }
    s
}
pub fn total(s: &Snap) -> Led {
    s.iter().fold(Led::default(), |a, l| a.plus(l))
}
pub fn field_total() -> u64 {
    CT.with(|c| (0..11).map(|i| c[i * 4].get() + c[i * 4 + 1].get() + c[i * 4 + 2].get()).sum())
}
/// Arm a budget of `n` additional field operations (mul+add+inv). 0 clears.
pub fn set_budget(n: u64) {
    LIMIT.with(|l| l.set(if n == 0 { 0 } else { field_total() + n }));
}
#[inline]
pub fn over_budget() -> bool {
    LIMIT.with(|l| {
        let v = l.get();
        v != 0 && field_total() > v
    })
}

// ---- counted F_p arithmetic (p < 2^31) ----
#[inline]
pub fn fmul(a: u64, b: u64, p: u64) -> u64 {
    mul_n(1);
    a * b % p
}
#[inline]
pub fn fadd(a: u64, b: u64, p: u64) -> u64 {
    add_n(1);
    let s = a + b;
    if s >= p { s - p } else { s }
}
#[inline]
pub fn fsub(a: u64, b: u64, p: u64) -> u64 {
    add_n(1);
    if a >= b { a - b } else { a + p - b }
}
#[inline]
pub fn fneg(a: u64, p: u64) -> u64 {
    fsub(0, a, p)
}
/// Modular inverse; one `inv` is charged (extended Euclid; the Euclid steps are
/// not individually counted, see PROTOCOL's inversion-weight sensitivity).
pub fn finv(a: u64, p: u64) -> u64 {
    inv_n(1);
    assert!(a % p != 0, "inverse of zero");
    let (mut t, mut nt, mut r, mut nr) = (0i64, 1i64, p as i64, (a % p) as i64);
    while nr != 0 {
        let q = r / nr;
        (t, nt) = (nt, t - q * nt);
        (r, nr) = (nr, r - q * nr);
    }
    assert_eq!(r, 1);
    if t < 0 { (t + p as i64) as u64 } else { t as u64 }
}
pub fn fpow(mut a: u64, mut e: u64, p: u64) -> u64 {
    let mut r = 1u64;
    while e > 0 {
        if e & 1 == 1 {
            r = fmul(r, a, p);
        }
        e >>= 1;
        if e > 0 {
            a = fmul(a, a, p);
        }
    }
    r
}
