# Borrowed interval ownership for doubled rescue

T-1881. Implementation argument and producer checks; independent signed verdict pending. [pcc:unverified]

## Ownership change

The preceding shared-engine construction used Rc<RefCell<IntervalProducts>>. This additive instrument replaces it with EngineStorage<'a,T>, whose variants are Owned(RefCell<T>) and Borrowed(&'a RefCell<T>). Deref exposes the same cell methods. The original recovery owns its engine inline; the doubled rescue stores a reference to that engine's cell. No ownership allocation, Rc clone, increment or decrement occurs in those constructors. The polynomial and progression vectors still allocate as before: this is not a zero-allocation arithmetic engine.

RecoveryProducts and RecoveryModes now carry the engine lifetime. SharedCoeffStream ties its wrapper reference to that lifetime. for_doubled returns RecoveryProducts<'_>, borrowing from its self argument instead of returning Self with an independent owner. Key checks, bounded width/cap checks, nonpanicking RefCell borrow conflicts and the arithmetic remain identical to the shared predecessor. No unsafe code, raw-pointer ownership, global engine or engine swap is used.

The reciprocal control always constructs its own engine. Its field is explicitly RecoveryProducts<'static>, meaning that this owned value contains no borrowed external engine. This annotation does not extend any reference lifetime; the constructor uses the Owned variant. The first compile exposed the previously unannotated field, and that failure is retained.

## Lifetime property and costs

A borrowed wrapper cannot outlive the owner it references. The negative fixture invokes the actual for_doubled API on a local owner and attempts to return RecoveryProducts<'static>. The dedicated compiler invocation must reject it with E0515 referencing owner. The ordinary native binary compiles the same source without this deliberately invalid cfg branch. A compiler rejection for an unrelated error is not accepted.

The generic enum stores enough inline space for its Owned variant even when it currently holds only a reference. Consequently wrapper size can be much larger than the prior Rc handle. Record size_of values for the actual compiler/target; they are layout observations, not peak-stack usage, peak memory or a timing result. Moves and copies may impose costs. The live polynomial/progression data still reside in their existing Vec allocations, while the engine metadata is inline.

Nested borrowed wrappers retain the same cell address. After their scopes end, the original owner remains usable. RefCell mutation rules still apply dynamically even though storage lifetime is enforced statically. The scalar-stream controls retain equal-width versus native-width comparisons, and public controls retain the prior six fixed cap panels. This ownership change does not alter the retained-width tradeoff or source-query policy.

## Evidence obligations

Preserve the first compile failure and its changed source snapshot. Require normal native compilation and all inherited checks to pass after the annotation correction. Verify the actual escaping-wrapper fixture fails for E0515. Compare all current source hashes against the successful native receipt before compiling the negative fixture. Retain rejected-key and borrow-conflict controls, nested borrowing, exact coefficients, source calls and unresolved targets.

No new cloud or timed workload is run. The original 4563-case cohort is immutable. Producer attestations and compiler checks do not replace an independent signed scientific verdict.

## Sources and next work

Sources checked: feature-native-shared-rescue/{context.rs,shared-rescue.rs,shared-rescue-phase.rs,shared-rescue-checks.rs}; official Rust lifetime documentation https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html and the preceding Rc/RefCell references. KB MCP absent and wrapper unavailable; these exact sources are the fallback. Lifetimes and owned/borrowed enums are standard Rust techniques, not a new ECDLP mechanism.

Next choose the interval width before the first coefficient stream under an explicit possible-doubling policy. The existing width controls show that retaining the smaller width adds batches; compare upfront setup and retained-memory costs against that batch reduction, with separate no-rescue controls.
