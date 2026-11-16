//! Test-child observation of real durable boundaries. This module is cfg(test)
//! only. It cannot mint a receipt or change producer state, limits or verdicts.
use super::capture;
use std::cell::Cell;
use std::marker::PhantomData;
use std::rc::Rc;

#[derive(Clone, Copy)]
pub(in crate::ml_dataset::solvers) enum Boundary<'a> {
    /// Head installation and acknowledgement equality checks already succeeded.
    /// The producer's subsequent deadline check has not necessarily succeeded.
    ChunkAcknowledged {
        pattern: u32,
        snapshot: u64,
        chunk: u64,
        bytes: u64,
        receipt: &'a capture::CommittedEventPosition,
    },
    /// Encoder completion, exact acknowledgements, byte totals and original
    /// budget check already succeeded; native mathematical success is not implied.
    SnapshotComplete {
        pattern: u32,
        snapshot: u64,
        chunks: u64,
        bytes: u64,
        receipt: &'a capture::CommittedEventPosition,
    },
}
type Observer = for<'a> fn(Boundary<'a>);
thread_local! {
    static OBSERVER: Cell<Option<Observer>> = const { Cell::new(None) };
}
/// Not Send/Sync: unregister only on the installing producer thread.
pub(in crate::ml_dataset::solvers) struct Guard {
    _same_thread: PhantomData<Rc<()>>,
}
impl Drop for Guard {
    fn drop(&mut self) {
        OBSERVER.with(|slot| slot.set(None));
    }
}
/// Install once on the actual producer thread before authentic owner admission.
/// A test driver may copy bounded notification metadata to its preopened control
/// channel and park until supervisor termination. That instrumentation is outside
/// scientific timings and must be recorded separately. No owner renewal occurs.
/// Callback return resumes the ordinary producer checks; callback panic is a test
/// failure, never an accepted interruption envelope or a verified snapshot.
pub(in crate::ml_dataset::solvers) fn install(observer: Observer) -> Result<Guard, &'static str> {
    OBSERVER.with(|slot| {
        if slot.get().is_some() {
            return Err("publication test observer already installed on this thread");
        }
        slot.set(Some(observer));
        Ok(Guard { _same_thread: PhantomData })
    })
}
pub(super) fn notify(boundary: Boundary<'_>) {
    // Cell retrieval does not hold a RefCell borrow across a user callback.
    // The reference lifetime prevents a safe callback from retaining the receipt.
    if let Some(observer) = OBSERVER.with(Cell::get) {
        observer(boundary);
    }
}
