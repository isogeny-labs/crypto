use std::{
    cell::{Cell, RefCell},
    collections::BTreeMap,
};
pub type Counts = BTreeMap<u64, [u64; 16]>;
thread_local! {static ACTIVE:Cell<bool>=const{Cell::new(false)};static COUNTS:RefCell<Counts>=RefCell::new(BTreeMap::new());}
#[inline]
pub fn hit(p: u64, j: usize, n: u64) {
    if ACTIVE.with(Cell::get) {
        COUNTS.with(|c| c.borrow_mut().entry(p).or_insert([0; 16])[j] += n)
    }
}
pub fn reset() {
    COUNTS.with(|c| c.borrow_mut().clear());
    ACTIVE.with(|c| c.set(true))
}
pub fn disable() {
    ACTIVE.with(|c| c.set(false))
}
pub fn snapshot() -> Counts {
    COUNTS.with(|c| c.borrow().clone())
}
