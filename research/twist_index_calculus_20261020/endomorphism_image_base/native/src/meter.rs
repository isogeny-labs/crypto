//! Explicit scope: native Fp arithmetic and subgroup arithmetic, excluding F4.
use serde::{Deserialize, Serialize};
use std::cell::Cell;
#[derive(Clone, Copy, Default, Debug, Serialize, Deserialize)]
pub(super) struct Counts {
    pub fp_muls: u64,
    pub subgroup_muls: u64,
    pub group_add_calls: u64,
}
thread_local! {
    static COUNTS: Cell<Counts> = Cell::new(Counts::default());
    static MODULI: Cell<(u64,u64)> = const { Cell::new((0,0)) };
}
pub(super) fn configure(p: u64, n: u64) {
    MODULI.with(|v| v.set((p, n)));
    reset();
}
pub(super) fn reset() {
    COUNTS.with(|v| v.set(Counts::default()));
}
pub(super) fn snapshot() -> Counts {
    COUNTS.with(Cell::get)
}
pub(super) fn difference(a: Counts, b: Counts) -> Counts {
    Counts {
        fp_muls: b.fp_muls - a.fp_muls,
        subgroup_muls: b.subgroup_muls - a.subgroup_muls,
        group_add_calls: b.group_add_calls - a.group_add_calls,
    }
}
pub(super) fn fp(n: u64) {
    if MODULI.with(|v| v.get().0) != 0 {
        COUNTS.with(|v| {
            let mut c = v.get();
            c.fp_muls += n;
            v.set(c);
        });
    }
}
pub(super) fn modular(modulus: u64, n: u64) {
    let (p, q) = MODULI.with(Cell::get);
    if modulus == p && p != 0 {
        fp(n);
    } else if modulus == q && q != 0 {
        COUNTS.with(|v| {
            let mut c = v.get();
            c.subgroup_muls += n;
            v.set(c);
        });
    }
}
pub(super) fn group_call() {
    if MODULI.with(|v| v.get().0) != 0 {
        COUNTS.with(|v| {
            let mut c = v.get();
            c.group_add_calls += 1;
            v.set(c);
        });
    }
}
