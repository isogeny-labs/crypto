use std::{
    cell::{Cell, RefCell},
    collections::BTreeMap,
};
pub type Counts = BTreeMap<u64, [u64; 16]>;
pub type Phases = BTreeMap<(&'static str, u64), [u64; 16]>;
thread_local! {static ACTIVE:Cell<bool>=const{Cell::new(false)};static TOTAL:RefCell<Counts>=RefCell::new(BTreeMap::new());static PHASES:RefCell<Phases>=RefCell::new(BTreeMap::new());static STAGE:Cell<&'static str>=const{Cell::new("entry")};}
pub fn stage(s: &'static str) {
    STAGE.with(|c| c.set(s))
}
pub fn hit(p: u64, j: usize, n: u64) {
    if ACTIVE.with(Cell::get) {
        TOTAL.with(|c| c.borrow_mut().entry(p).or_insert([0; 16])[j] += n);
        let s = STAGE.with(Cell::get);
        PHASES.with(|c| c.borrow_mut().entry((s, p)).or_insert([0; 16])[j] += n)
    }
}
pub fn reset() {
    TOTAL.with(|c| c.borrow_mut().clear());
    PHASES.with(|c| c.borrow_mut().clear());
    stage("entry");
    ACTIVE.with(|c| c.set(true))
}
pub fn disable() {
    ACTIVE.with(|c| c.set(false))
}
pub fn snapshot() -> Counts {
    TOTAL.with(|c| c.borrow().clone())
}
pub fn phases() -> Phases {
    PHASES.with(|c| c.borrow().clone())
}
