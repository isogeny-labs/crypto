use hec_native_ic::counted;
use std::{cell::RefCell, collections::BTreeMap};
#[derive(Default)]
struct Ledger {
    name: &'static str,
    last: counted::Counts,
    totals: BTreeMap<(&'static str, u64), [u64; 16]>,
}
thread_local! {static STATE:RefCell<Ledger>=RefCell::new(Ledger::default());}
pub fn begin() {
    counted::reset();
    STATE.with(|s| {
        *s.borrow_mut() = Ledger {
            name: "eligibility",
            ..Ledger::default()
        }
    });
}
pub fn mark(next: &'static str) {
    let now = counted::snapshot();
    STATE.with(|s| {
        let mut l = s.borrow_mut();
        for (&p, v) in &now {
            let before = l.last.get(&p).copied().unwrap_or_default();
            let name = l.name;
            let sum = l.totals.entry((name, p)).or_default();
            for i in 0..16 {
                sum[i] += v[i] - before[i];
            }
        }
        l.last = now;
        l.name = next;
    })
}
pub fn finish() {
    mark("end");
    STATE.with(|s| {
        let l = s.borrow();
        let mut total = counted::Counts::new();
        for (&(name, p), v) in &l.totals {
            let sum = total.entry(p).or_default();
            for i in 0..16 {
                sum[i] += v[i];
            }
            println!("PHASE name={name} modulus={p} vector={v:?}");
        }
        assert_eq!(total, l.last);
        for (p, v) in total {
            println!("TOTAL modulus={p} vector={v:?}");
        }
    });
    counted::disable();
}
