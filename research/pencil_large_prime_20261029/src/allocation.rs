//! Observed System-allocator calls and requested layout bytes, not RSS.
//! Optimizers may remove allocations. Regions require a single-threaded,
//! nonnested observer; allocation itself is thread-safe and never allocates
//! or panics while updating counters.
use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicU64, Ordering::Relaxed};

pub struct ObservedSystem {
    allocs: AtomicU64,
    frees: AtomicU64,
    reallocs: AtomicU64,
    failures: AtomicU64,
    requested: AtomicU64,
    released: AtomicU64,
    live: AtomicU64,
    peak: AtomicU64,
    envelope_peak: AtomicU64,
}

#[derive(Clone, Copy)]
pub struct Boundary {
    events: [u64; 6],
    live: u64,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "reporting", derive(serde::Serialize))]
pub struct Region {
    pub alloc_calls: u64,
    pub dealloc_calls: u64,
    pub realloc_calls: u64,
    pub failures: u64,
    pub requested_bytes: u64,
    pub released_bytes: u64,
    pub peak_live_requested_bytes: u64,
    pub peak_growth_over_entry_bytes: u64,
}
impl Region {
    pub fn add_assign(&mut self, rhs: Self) {
        self.alloc_calls += rhs.alloc_calls;
        self.dealloc_calls += rhs.dealloc_calls;
        self.realloc_calls += rhs.realloc_calls;
        self.failures += rhs.failures;
        self.requested_bytes += rhs.requested_bytes;
        self.released_bytes += rhs.released_bytes;
        self.peak_live_requested_bytes = self
            .peak_live_requested_bytes
            .max(rhs.peak_live_requested_bytes);
        self.peak_growth_over_entry_bytes = self
            .peak_growth_over_entry_bytes
            .max(rhs.peak_growth_over_entry_bytes);
    }
}
impl Default for ObservedSystem {
    fn default() -> Self {
        Self::new()
    }
}
impl ObservedSystem {
    pub const fn new() -> Self {
        Self {
            allocs: AtomicU64::new(0),
            frees: AtomicU64::new(0),
            reallocs: AtomicU64::new(0),
            failures: AtomicU64::new(0),
            requested: AtomicU64::new(0),
            released: AtomicU64::new(0),
            live: AtomicU64::new(0),
            peak: AtomicU64::new(0),
            envelope_peak: AtomicU64::new(0),
        }
    }
    fn events(&self) -> [u64; 6] {
        [
            self.allocs.load(Relaxed),
            self.frees.load(Relaxed),
            self.reallocs.load(Relaxed),
            self.failures.load(Relaxed),
            self.requested.load(Relaxed),
            self.released.load(Relaxed),
        ]
    }
    pub fn begin(&self) -> Boundary {
        let live = self.live.load(Relaxed);
        self.peak.store(live, Relaxed);
        Boundary {
            events: self.events(),
            live,
        }
    }
    pub fn finish(&self, before: Boundary) -> Region {
        self.finish_at(before, self.peak.load(Relaxed))
    }
    /// One outer target envelope may contain sequential begin/finish regions.
    pub fn begin_envelope(&self) -> Boundary {
        let live = self.live.load(Relaxed);
        self.envelope_peak.store(live, Relaxed);
        Boundary {
            events: self.events(),
            live,
        }
    }
    pub fn finish_envelope(&self, before: Boundary) -> Region {
        self.finish_at(before, self.envelope_peak.load(Relaxed))
    }
    fn finish_at(&self, before: Boundary, peak: u64) -> Region {
        let e = self.events();
        let d: [u64; 6] = std::array::from_fn(|i| e[i].wrapping_sub(before.events[i]));
        Region {
            alloc_calls: d[0],
            dealloc_calls: d[1],
            realloc_calls: d[2],
            failures: d[3],
            requested_bytes: d[4],
            released_bytes: d[5],
            peak_live_requested_bytes: peak,
            peak_growth_over_entry_bytes: peak.saturating_sub(before.live),
        }
    }
    fn grew(&self, bytes: u64) {
        let live = self.live.fetch_add(bytes, Relaxed).wrapping_add(bytes);
        self.peak.fetch_max(live, Relaxed);
        self.envelope_peak.fetch_max(live, Relaxed);
    }
    fn allocated(&self, ptr: *mut u8, size: usize) {
        if ptr.is_null() {
            self.failures.fetch_add(1, Relaxed);
        } else {
            self.requested.fetch_add(size as u64, Relaxed);
            self.grew(size as u64);
        }
    }
}

// SAFETY: every pointer/layout call is forwarded unchanged to System. The
// observer only uses nonallocating atomic operations and touches no payload.
unsafe impl GlobalAlloc for ObservedSystem {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.allocs.fetch_add(1, Relaxed);
        let ptr = unsafe { System.alloc(layout) };
        self.allocated(ptr, layout.size());
        ptr
    }
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        self.allocs.fetch_add(1, Relaxed);
        let ptr = unsafe { System.alloc_zeroed(layout) };
        self.allocated(ptr, layout.size());
        ptr
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe {
            System.dealloc(ptr, layout);
        }
        self.frees.fetch_add(1, Relaxed);
        self.released.fetch_add(layout.size() as u64, Relaxed);
        self.live.fetch_sub(layout.size() as u64, Relaxed);
    }
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        self.reallocs.fetch_add(1, Relaxed);
        let result = unsafe { System.realloc(ptr, layout, new_size) };
        if result.is_null() {
            self.failures.fetch_add(1, Relaxed);
        } else {
            // Successful realloc replaces one requested layout by another;
            // this does not claim the allocator actually copied these bytes.
            self.requested.fetch_add(new_size as u64, Relaxed);
            self.released.fetch_add(layout.size() as u64, Relaxed);
            if new_size >= layout.size() {
                self.grew((new_size - layout.size()) as u64);
            } else {
                self.live
                    .fetch_sub((layout.size() - new_size) as u64, Relaxed);
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn zeroed_grow_shrink_preserves_payload_and_observations() {
        let a = ObservedSystem::new();
        let boundary = a.begin();
        let envelope = a.begin_envelope();
        unsafe {
            let initial = Layout::from_size_align(32, 8).unwrap();
            let p = a.alloc_zeroed(initial);
            assert!(!p.is_null());
            assert!((0..32).all(|i| *p.add(i) == 0));
            *p = 91;
            let q = a.realloc(p, initial, 64);
            assert!(!q.is_null());
            assert_eq!(*q, 91);
            let r = a.realloc(q, Layout::from_size_align(64, 8).unwrap(), 16);
            assert!(!r.is_null());
            assert_eq!(*r, 91);
            a.dealloc(r, Layout::from_size_align(16, 8).unwrap());
        }
        let s = a.finish(boundary);
        assert_eq!(
            (s.alloc_calls, s.realloc_calls, s.dealloc_calls, s.failures),
            (1, 2, 1, 0)
        );
        assert_eq!((s.requested_bytes, s.released_bytes), (112, 112));
        assert_eq!(
            (s.peak_live_requested_bytes, s.peak_growth_over_entry_bytes),
            (64, 64)
        );
        let next = a.begin();
        assert_eq!(a.finish(next).peak_live_requested_bytes, 0);
        assert_eq!(a.finish_envelope(envelope).peak_live_requested_bytes, 64);
    }
}
