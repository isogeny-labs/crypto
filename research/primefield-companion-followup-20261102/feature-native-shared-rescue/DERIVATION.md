# Shared interval-product setup across coefficient strides

T-1880. Derived cache equivalence and producer checks; independent signed verdict pending. [pcc:unverified]

## Cache identity versus stream identity

The coefficient stream retains its existing identity (r,m,cap). Its initial binomial coefficient, signed m-stride recurrence and cursor are unchanged. A stream with stride 2m must not masquerade as a stride-m stream; the existing mismatched-key check remains.

The underlying IntervalProducts engine computes products product_(j=0)^(L-1)(a+j) in F_r. Its cached polynomial is P_w(X)=product_(j=0)^(w-1)(X+j). Its progression table depends on this polynomial, r and step w. Neither contains m. The constructor's max_length parameter chooses w=min(ceil(sqrt(max_length)),cap); it is not a later maximum-length admission check. Existing products accepts any L<r, with at most 2cap requests and at most w buffered evaluation points.

For any such request, partition L=q*w+t and evaluate P_w(a+i*w) for i=0..q-1, then multiply the t tail factors. This identity holds regardless of whether the engine width was selected from m or 2m. Therefore the order-2m stream can use the existing engine without changing any coefficient value, while retaining its own (r,2m,cap) wrapper identity.

## Implementation and ownership

In this additive instrument, RecoveryProducts stores an Rc<RefCell<IntervalProducts>>. Fresh recovery contexts still create a new engine. The explicit for_doubled constructor checks field, doubled stride and unchanged block cap, and checks the engine field, request cap and bounded positive width under a nonpanicking borrow. It then clones the Rc, not the polynomial or progression arrays. Coefficient readers continue borrowing the wrapper with their own stride key.

The shared rescue creates a separately keyed doubled RecoveryModes value using that wrapper. All eta/order checks and recovery arithmetic are retained. The original conditional helper remains the fresh-engine reference. A conflicting mutable borrow returns Error::Arithmetic; no unsafe code, global cache or cross-target sharing is introduced. Dropping the rescue wrapper reduces the owner count while the original context retains the engine and any lazily initialized cache.

Rc is a single-threaded owner; this instrument makes no concurrent-worker sharing claim. RefCell preserves dynamic borrow checking. The original RefCell lived inline, so Rc introduces a heap allocation for the original engine and reference-count operations. Do not equate fewer engine constructions or polynomial setups with fewer allocations overall.

## Width tradeoff and controls

Retaining w(m) can use more block evaluations than a fresh w(2m). The check has three arms:

1. Shared engine at w(m), used by both streams.
2. Separate engines, both at w(m); the second engine's width is set before any use. This isolates sharing from width changes.
3. Separate engines at their native widths w(m) and w(2m).

All arms evaluate the same complete e=0,1,2 streams in the same order. Values must agree. Equal-width evaluation-batch totals must agree; shared lazy polynomial/progression setups occur at most once. Native-width batches are reported separately rather than credited to sharing. Additional interleaving tests compare stream cursor outputs against independent coefficient readers; field/stride/cap mismatches, borrow conflicts and wrapper cleanup are checked.

Public comparisons preserve the predecessor's six fixed cap panels. The small m=5,2m=10 fixture uses direct interval multiplication (length below 32), so it cannot demonstrate avoidance of an already-built polynomial/progression cache. Report those shares as cold. Larger scalar-stream controls exercise actual warm-cache reuse. The original cohort remains unchanged.

## Costs and next step

Count engine shares, warm/cold status, retained widths, polynomial setups, progression setups and evaluation batches. Retain coefficient initialization/advancement, inversions, exact-order checks, source work, polynomial quotient operations, Rc allocation/reference-count operations, copies, caps and final point verification. No timing or aggregate field-operation advantage is claimed from setup counts alone.

Next investigate borrowing the engine without an Rc allocation, or choosing shared width before the first stream from a declared possible-doubling bound. The latter must compare initial extra work with subsequent batch savings. Neither is implemented here.

## Sources checked

Exact feature-native-doubled-rescue/{interval.rs,context.rs,context-checks.rs,doubled.rs}; official Rust Rc documentation https://doc.rust-lang.org/std/rc/struct.Rc.html and RefCell documentation https://doc.rust-lang.org/std/cell/struct.RefCell.html for shared ownership and borrow semantics. KB MCP is absent; scripts/research-kb status reports no executable. The interval factorization is derived above, with no novelty claim.
