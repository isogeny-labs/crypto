use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

static ENABLED: AtomicBool = AtomicBool::new(false);
static FP_ADD: AtomicU64 = AtomicU64::new(0);
static FP_MUL: AtomicU64 = AtomicU64::new(0);
static FP_INV: AtomicU64 = AtomicU64::new(0);
static SC_ADD: AtomicU64 = AtomicU64::new(0);
static SC_MUL: AtomicU64 = AtomicU64::new(0);
static MIX: AtomicU64 = AtomicU64::new(0);
static CMP: AtomicU64 = AtomicU64::new(0);
static ALLOCS: AtomicU64 = AtomicU64::new(0);
static BYTES: AtomicU64 = AtomicU64::new(0);
static FREED: AtomicU64 = AtomicU64::new(0);
static PEAK: AtomicU64 = AtomicU64::new(0);

fn bump(counter: &AtomicU64, n: u64) {
    if ENABLED.load(Ordering::Relaxed) {
        counter.fetch_add(n, Ordering::Relaxed);
    }
}
fn allocation(size: usize) {
    if ENABLED.load(Ordering::Relaxed) {
        ALLOCS.fetch_add(1, Ordering::Relaxed);
        let total = BYTES.fetch_add(u64::try_from(size).unwrap(), Ordering::Relaxed)
            + u64::try_from(size).unwrap();
        let live = total.saturating_sub(FREED.load(Ordering::Relaxed));
        PEAK.fetch_max(live, Ordering::Relaxed);
    }
}

struct AccountedSystem;
unsafe impl GlobalAlloc for AccountedSystem {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // SAFETY: the original allocator contract and layout are forwarded.
        let p = unsafe { System.alloc(layout) };
        if !p.is_null() {
            allocation(layout.size());
        }
        p
    }
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        // SAFETY: the original allocator contract and layout are forwarded.
        let p = unsafe { System.alloc_zeroed(layout) };
        if !p.is_null() {
            allocation(layout.size());
        }
        p
    }
    unsafe fn dealloc(&self, p: *mut u8, layout: Layout) {
        bump(&FREED, u64::try_from(layout.size()).unwrap());
        // SAFETY: p and its matching layout came from the system allocator.
        unsafe { System.dealloc(p, layout) };
    }
    unsafe fn realloc(&self, p: *mut u8, layout: Layout, size: usize) -> *mut u8 {
        // SAFETY: the original allocation and requested size are forwarded.
        let next = unsafe { System.realloc(p, layout, size) };
        if !next.is_null() {
            bump(&FREED, u64::try_from(layout.size()).unwrap());
            allocation(size);
        }
        next
    }
}
#[global_allocator]
static ALLOCATOR: AccountedSystem = AccountedSystem;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cost {
    pub fp_add: u64,
    pub fp_mul: u64,
    pub fp_inv: u64,
    pub scalar_add: u64,
    pub scalar_mul: u64,
    pub hash_mix: u64,
    pub coordinate_compare: u64,
    pub allocation_calls: u64,
    pub allocated_bytes: u64,
    pub freed_bytes: u64,
    pub peak_new_heap_bytes: u64,
}
pub fn sample<T>(f: impl FnOnce() -> T) -> (T, Cost) {
    assert!(!ENABLED.load(Ordering::Relaxed), "no nested accounting");
    for c in [
        &FP_ADD, &FP_MUL, &FP_INV, &SC_ADD, &SC_MUL, &MIX, &CMP, &ALLOCS,
        &BYTES, &FREED, &PEAK,
    ] {
        c.store(0, Ordering::Relaxed);
    }
    ENABLED.store(true, Ordering::Relaxed);
    let result = f();
    ENABLED.store(false, Ordering::Relaxed);
    let get = |c: &AtomicU64| c.load(Ordering::Relaxed);
    let cost = Cost {
        fp_add: get(&FP_ADD),
        fp_mul: get(&FP_MUL),
        fp_inv: get(&FP_INV),
        scalar_add: get(&SC_ADD),
        scalar_mul: get(&SC_MUL),
        hash_mix: get(&MIX),
        coordinate_compare: get(&CMP),
        allocation_calls: get(&ALLOCS),
        allocated_bytes: get(&BYTES),
        freed_bytes: get(&FREED),
        peak_new_heap_bytes: get(&PEAK),
    };
    (result, cost)
}
pub fn fp_add() {
    bump(&FP_ADD, 1);
}
pub fn fp_mul() {
    bump(&FP_MUL, 1);
}
pub fn fp_inv() {
    bump(&FP_INV, 1);
}
pub fn scalar_add() {
    bump(&SC_ADD, 1);
}
pub fn scalar_mul() {
    bump(&SC_MUL, 1);
}
pub fn hash_mix() {
    bump(&MIX, 1);
}
pub fn compare(n: usize) {
    bump(&CMP, u64::try_from(n).unwrap());
}
