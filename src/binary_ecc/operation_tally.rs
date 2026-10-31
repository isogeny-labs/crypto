//! Opt-in, thread-local field-operation ledger. Inversion is informational:
//! its multiplications and squarings are already counted, so never price it
//! again. Existing arithmetic and historical group ledgers are unchanged.
use std::{
    cell::RefCell,
    collections::BTreeMap,
    marker::PhantomData,
    rc::Rc,
    sync::atomic::{AtomicUsize, Ordering},
};

static ACTIVE: AtomicUsize = AtomicUsize::new(0);

thread_local! {
    static TALLIES: RefCell<Vec<BTreeMap<String,u64>>> = const { RefCell::new(Vec::new()) };
}

pub(crate) fn tick(m: u32, operation: &str) {
    if ACTIVE.load(Ordering::Relaxed) == 0 {
        return;
    }
    TALLIES.with(|stack| {
        if let Some(tally) = stack.borrow_mut().last_mut() {
            *tally.entry(format!("f2m{m}_{operation}")).or_default() += 1;
        }
    });
}

pub struct OperationTally {
    active: bool,
    _thread: PhantomData<Rc<()>>,
}

impl Default for OperationTally {
    fn default() -> Self {
        Self::begin()
    }
}

impl OperationTally {
    pub fn begin() -> Self {
        TALLIES.with(|s| s.borrow_mut().push(BTreeMap::new()));
        ACTIVE.fetch_add(1, Ordering::Relaxed);
        Self {
            active: true,
            _thread: PhantomData,
        }
    }
    pub fn finish(mut self) -> BTreeMap<String, u64> {
        let tally = TALLIES.with(|s| s.borrow_mut().pop().expect("active tally"));
        ACTIVE.fetch_sub(1, Ordering::Relaxed);
        self.active = false;
        tally
    }
}

impl Drop for OperationTally {
    fn drop(&mut self) {
        if self.active {
            TALLIES.with(|s| {
                s.borrow_mut().pop();
            });
            ACTIVE.fetch_sub(1, Ordering::Relaxed);
        }
    }
}

/// Bounded field callers refuse the allocating optional tally path. Any live
/// borrow also refuses admission: tick() needs an exclusive borrow even when
/// this thread's stack is empty. The probe never changes stack or tally contents.
pub(crate) fn current_thread_active_or_borrowed() -> bool {
    TALLIES.with(|stack| match stack.try_borrow_mut() {
        Ok(stack) => !stack.is_empty(),
        Err(_) => true,
    })
}

#[cfg(test)]
mod bounded_field_tally_guard_tests {
    use super::*;
    #[test]
    fn shared_and_mutable_empty_stack_borrows_refuse_without_mutation() {
        TALLIES.with(|stack| {
            assert!(stack.borrow().is_empty());
            assert!(!current_thread_active_or_borrowed());
            let shared = stack.borrow();
            assert!(current_thread_active_or_borrowed());
            assert!(shared.is_empty());
            drop(shared);
            let exclusive = stack.borrow_mut();
            assert!(current_thread_active_or_borrowed());
            assert!(exclusive.is_empty());
            drop(exclusive);
            assert!(!current_thread_active_or_borrowed());
            assert!(stack.borrow().is_empty());
        });
    }
    #[test]
    fn active_tally_probe_preserves_existing_entries() {
        let tally = OperationTally::begin();
        tick(3, "add");
        let before = TALLIES.with(|stack| stack.borrow().clone());
        assert!(current_thread_active_or_borrowed());
        TALLIES.with(|stack| assert_eq!(*stack.borrow(), before));
        let result = tally.finish();
        assert_eq!(result.len(), 1);
        assert_eq!(result.get("f2m3_add"), Some(&1));
        assert!(!current_thread_active_or_borrowed());
    }
}
