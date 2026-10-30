//! Per-phase tally of the work a solve does, for the ecbench arm
//! `ic.tensor_contraction_v1` (Rust only; the Go prototype has no tally).
//!
//! The tally is off unless [`start`] was called on this thread, so the
//! port's behaviour and its tests are unchanged. While it is on:
//!
//! - every non-trivial point addition and doubling in [`crate::Curve::add`]
//!   (neither input is O) is counted under `<phase>.adds` or
//!   `<phase>.doubles`;
//! - native work that is not a group operation is counted under a name
//!   ending `_uncharged`: rows produced by the sparse contraction, points
//!   enumerated and membership hashes evaluated when a factor base is
//!   listed, and modular multiplications in the elimination.
//!
//! Every count depends only on the inputs and the solver's seed, never on
//! hash-map iteration order, so replays reproduce it exactly.

use std::cell::RefCell;
use std::collections::BTreeMap;

struct Tally {
    phase: &'static str,
    counts: BTreeMap<String, u64>,
}

thread_local! {
    static TALLY: RefCell<Option<Tally>> = const { RefCell::new(None) };
}

/// Starts an empty tally on this thread, in phase `setup`.
pub fn start() {
    TALLY.with(|t| {
        *t.borrow_mut() = Some(Tally {
            phase: "setup",
            counts: BTreeMap::new(),
        })
    });
}

/// Stops the tally and returns its counts (empty if none was running).
pub fn take() -> BTreeMap<String, u64> {
    TALLY.with(|t| t.borrow_mut().take().map(|t| t.counts).unwrap_or_default())
}

/// Reports whether a tally is running on this thread. Work that would be
/// dispatched to other threads must stay sequential while it is true, since
/// the tally is thread-local.
pub fn active() -> bool {
    TALLY.with(|t| t.borrow().is_some())
}

/// Names the phase that later counts belong to.
pub fn set_phase(phase: &'static str) {
    TALLY.with(|t| {
        if let Some(t) = t.borrow_mut().as_mut() {
            t.phase = phase;
        }
    });
}

/// Adds `n` to `<phase>.<name>`; a no-op while no tally is running.
pub(crate) fn bump(name: &str, n: u64) {
    TALLY.with(|t| {
        if let Some(t) = t.borrow_mut().as_mut() {
            *t.counts.entry(format!("{}.{}", t.phase, name)).or_default() += n;
        }
    });
}
