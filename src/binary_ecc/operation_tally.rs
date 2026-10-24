//! Opt-in, thread-local field-operation ledger. Inversion is informational:
//! its multiplications and squarings are already counted, so never price it
//! again. Existing arithmetic and historical group ledgers are unchanged.
use std::{cell::RefCell, collections::BTreeMap, marker::PhantomData, rc::Rc};

thread_local! {
    static TALLIES: RefCell<Vec<BTreeMap<String,u64>>> = const { RefCell::new(Vec::new()) };
}

pub(crate) fn tick(m: u32, operation: &str) {
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
        Self {
            active: true,
            _thread: PhantomData,
        }
    }
    pub fn finish(mut self) -> BTreeMap<String, u64> {
        self.active = false;
        TALLIES.with(|s| s.borrow_mut().pop().expect("active tally"))
    }
}

impl Drop for OperationTally {
    fn drop(&mut self) {
        if self.active {
            TALLIES.with(|s| {
                s.borrow_mut().pop();
            });
        }
    }
}
