//! Scoped, allocation-free event bookkeeping. The ordinary study binary does
//! not install an allocator wrapper; the separate accounting binary supplies
//! hooks. Requested layout bytes are not live bytes, allocator arenas or RSS.

use serde::{Deserialize, Serialize};
use std::cell::Cell;
use std::marker::PhantomData;
use std::rc::Rc;

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct AllocationStats {
    pub alloc_calls: u64,
    pub alloc_zeroed_calls: u64,
    pub realloc_calls: u64,
    pub dealloc_calls: u64,
    pub alloc_requested_bytes: u64,
    pub zeroed_requested_bytes: u64,
    pub realloc_old_layout_bytes: u64,
    pub realloc_new_requested_bytes: u64,
    pub dealloc_layout_bytes: u64,
    pub failed_allocation_calls: u64,
    pub overflow: bool,
}

#[derive(Clone, Copy)]
pub(crate) struct AllocationHooks {
    pub begin: fn(),
    pub snapshot: fn() -> AllocationStats,
    pub end: fn(),
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct KeyWork {
    pub key_constructions: u64,
    pub table_lookups: u64,
    pub key_hash_calls: u64,
    pub hash_builds: u64,
    pub hash_finishes: u64,
    pub hash_input_bytes: u64,
    pub full_key_equalities: u64,
    pub full_key_orderings: u64,
    pub overflow: bool,
}
impl KeyWork {
    const ZERO: Self = Self {
        key_constructions: 0,
        table_lookups: 0,
        key_hash_calls: 0,
        hash_builds: 0,
        hash_finishes: 0,
        hash_input_bytes: 0,
        full_key_equalities: 0,
        full_key_orderings: 0,
        overflow: false,
    };
}

pub(crate) enum Event {
    KeyConstruction,
    Lookup,
    KeyHash,
    HashBuild,
    HashFinish,
    HashBytes(usize),
    Equality,
    Ordering,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Snapshot {
    pub keys: KeyWork,
    pub allocations: Option<AllocationStats>,
    // std's public API does not expose SwissTable control-group probes.
    pub internal_probe_groups: Option<u64>,
    pub peak_live_bytes: Option<u64>,
}

#[derive(Clone, Copy)]
struct State {
    active: bool,
    keys: KeyWork,
    hooks: Option<AllocationHooks>,
}
thread_local! {
    static STATE: Cell<State> = const {Cell::new(State {
        active: false, keys: KeyWork::ZERO, hooks: None,
    })};
}

pub(crate) fn configure(hooks: Option<AllocationHooks>) -> Result<(), String> {
    STATE.with(|cell| {
        let mut state = cell.get();
        if state.active {
            return Err("cannot change an active accounting backend".into());
        }
        state.hooks = hooks;
        cell.set(state);
        Ok(())
    })
}
pub(crate) fn has_allocator() -> bool {
    STATE.with(|s| s.get().hooks.is_some())
}

pub(crate) fn event(event: Event) {
    STATE.with(|cell| {
        let mut state = cell.get();
        if !state.active {
            return;
        }
        let (value, amount) = match event {
            Event::KeyConstruction => (&mut state.keys.key_constructions, 1),
            Event::Lookup => (&mut state.keys.table_lookups, 1),
            Event::KeyHash => (&mut state.keys.key_hash_calls, 1),
            Event::HashBuild => (&mut state.keys.hash_builds, 1),
            Event::HashFinish => (&mut state.keys.hash_finishes, 1),
            Event::HashBytes(bytes) => (&mut state.keys.hash_input_bytes, bytes as u64),
            Event::Equality => (&mut state.keys.full_key_equalities, 1),
            Event::Ordering => (&mut state.keys.full_key_orderings, 1),
        };
        match value.checked_add(amount) {
            Some(n) => *value = n,
            None => {
                *value = u64::MAX;
                state.keys.overflow = true;
            }
        }
        cell.set(state);
    });
}

pub(crate) fn snapshot() -> Option<Snapshot> {
    STATE.with(|cell| {
        let state = cell.get();
        state.active.then(|| Snapshot {
            keys: state.keys,
            allocations: state.hooks.map(|h| (h.snapshot)()),
            internal_probe_groups: None,
            peak_live_bytes: None,
        })
    })
}

pub(crate) struct Scope {
    hooks: Option<AllocationHooks>,
    _thread: PhantomData<Rc<()>>,
}
impl Scope {
    pub(crate) fn start() -> Result<Self, String> {
        STATE.with(|cell| {
            let mut state = cell.get();
            if state.active {
                return Err("nested point accounting interval".into());
            }
            state.active = true;
            state.keys = KeyWork::ZERO;
            cell.set(state);
            if let Some(hooks) = state.hooks {
                (hooks.begin)();
            }
            Ok(Self {
                hooks: state.hooks,
                _thread: PhantomData,
            })
        })
    }
}
impl Drop for Scope {
    fn drop(&mut self) {
        if let Some(hooks) = self.hooks {
            (hooks.end)();
        }
        STATE.with(|cell| {
            let mut state = cell.get();
            state.active = false;
            cell.set(state);
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn scopes_reset_and_reject_nesting_without_zeroing_the_outer_one() {
        configure(None).unwrap();
        let scope = Scope::start().unwrap();
        event(Event::Lookup);
        assert!(Scope::start().is_err());
        assert_eq!(snapshot().unwrap().keys.table_lookups, 1);
        drop(scope);
        assert!(snapshot().is_none());
        let scope = Scope::start().unwrap();
        assert_eq!(snapshot().unwrap().keys, KeyWork::default());
        assert!(snapshot().unwrap().allocations.is_none());
        drop(scope);
    }
}
