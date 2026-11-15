//! Counts-only build for the point-table pilot. The ordinary example installs
//! no allocator wrapper and is the only supported source of batch clocks.
#![deny(unsafe_op_in_unsafe_fn)]

#[path = "native_point_hash_study/driver.rs"]
mod study;
use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;
use study::accounting::{AllocationHooks, AllocationStats};

struct CountingSystem;
#[global_allocator]
static ALLOCATOR: CountingSystem = CountingSystem;

const ZERO: AllocationStats = AllocationStats {
    alloc_calls: 0,
    alloc_zeroed_calls: 0,
    realloc_calls: 0,
    dealloc_calls: 0,
    alloc_requested_bytes: 0,
    zeroed_requested_bytes: 0,
    realloc_old_layout_bytes: 0,
    realloc_new_requested_bytes: 0,
    dealloc_layout_bytes: 0,
    failed_allocation_calls: 0,
    overflow: false,
};
thread_local! {static COUNTERS: Cell<(bool, AllocationStats)> = const {Cell::new((false, ZERO))};}

fn add(value: &mut u64, n: usize, overflow: &mut bool) {
    match u64::try_from(n).ok().and_then(|n| value.checked_add(n)) {
        Some(n) => *value = n,
        None => {
            *value = u64::MAX;
            *overflow = true;
        }
    }
}
fn observe(update: impl FnOnce(&mut AllocationStats)) {
    // Constant-initialized, destructor-free TLS and Copy counters only. No
    // allocation, formatting, lock or unwind is introduced in these callbacks.
    let _ = COUNTERS.try_with(|cell| {
        let (active, mut stats) = cell.get();
        if active {
            update(&mut stats);
            cell.set((true, stats));
        }
    });
}
// SAFETY: all pointer/layout contracts are forwarded unchanged to System.
// Observation touches only nonallocating TLS counters and never unwinds.
unsafe impl GlobalAlloc for CountingSystem {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // SAFETY: the caller supplies a valid allocation layout.
        let p = unsafe { System.alloc(layout) };
        observe(|s| {
            add(&mut s.alloc_calls, 1, &mut s.overflow);
            add(&mut s.alloc_requested_bytes, layout.size(), &mut s.overflow);
            if p.is_null() {
                add(&mut s.failed_allocation_calls, 1, &mut s.overflow);
            }
        });
        p
    }
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        // SAFETY: the caller supplies a valid allocation layout.
        let p = unsafe { System.alloc_zeroed(layout) };
        observe(|s| {
            add(&mut s.alloc_zeroed_calls, 1, &mut s.overflow);
            add(
                &mut s.zeroed_requested_bytes,
                layout.size(),
                &mut s.overflow,
            );
            if p.is_null() {
                add(&mut s.failed_allocation_calls, 1, &mut s.overflow);
            }
        });
        p
    }
    unsafe fn dealloc(&self, p: *mut u8, layout: Layout) {
        observe(|s| {
            add(&mut s.dealloc_calls, 1, &mut s.overflow);
            add(&mut s.dealloc_layout_bytes, layout.size(), &mut s.overflow);
        });
        // SAFETY: pointer and original layout are passed through unchanged.
        unsafe { System.dealloc(p, layout) };
    }
    unsafe fn realloc(&self, p: *mut u8, layout: Layout, size: usize) -> *mut u8 {
        // SAFETY: the original allocation/layout and valid new size are forwarded.
        let q = unsafe { System.realloc(p, layout, size) };
        observe(|s| {
            add(&mut s.realloc_calls, 1, &mut s.overflow);
            add(
                &mut s.realloc_old_layout_bytes,
                layout.size(),
                &mut s.overflow,
            );
            add(&mut s.realloc_new_requested_bytes, size, &mut s.overflow);
            if q.is_null() {
                add(&mut s.failed_allocation_calls, 1, &mut s.overflow);
            }
        });
        q
    }
}
fn begin() {
    COUNTERS.with(|c| c.set((true, ZERO)));
}
fn snapshot() -> AllocationStats {
    COUNTERS.with(|c| c.get().1)
}
fn end() {
    COUNTERS.with(|c| {
        let (_, s) = c.get();
        c.set((false, s));
    });
}
fn main() {
    let hooks = AllocationHooks {
        begin,
        snapshot,
        end,
    };
    if let Err(error) = study::entry(std::env::args_os().collect(), Some(hooks)) {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn raw_hooks_forward_layouts_and_do_not_measure_outside_scope() {
        begin();
        let layout = Layout::from_size_align(16, 8).unwrap();
        // SAFETY: every layout is valid, non-null results are checked before use,
        // and the final allocation is freed with its final layout.
        unsafe {
            let p = ALLOCATOR.alloc(layout);
            assert!(!p.is_null());
            let q = std::hint::black_box(ALLOCATOR.realloc(p, layout, 32));
            if q.is_null() {
                ALLOCATOR.dealloc(p, layout);
                end();
                panic!("development control could not reallocate 32 bytes");
            }
            ALLOCATOR.dealloc(q, Layout::from_size_align(32, 8).unwrap());
            let p = ALLOCATOR.alloc_zeroed(layout);
            assert!(!p.is_null());
            assert_eq!(std::hint::black_box(p).read(), 0);
            ALLOCATOR.dealloc(p, layout);
        }
        let measured = snapshot();
        end();
        assert_eq!(measured.alloc_calls, 1);
        assert_eq!(measured.alloc_zeroed_calls, 1);
        assert_eq!(measured.realloc_calls, 1);
        assert_eq!(measured.dealloc_calls, 2);
        assert_eq!(measured.alloc_requested_bytes, 16);
        assert_eq!(measured.zeroed_requested_bytes, 16);
        assert_eq!(measured.realloc_new_requested_bytes, 32);
        assert_eq!(measured.dealloc_layout_bytes, 48);
        assert!(!measured.overflow);
        observe(|s| s.alloc_calls = 999);
        assert_eq!(snapshot(), measured);
    }
}
