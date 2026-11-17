# Combined qualification v5 lifecycle successor

Preserves the unexecuted v4 mathematics, source inventory, trace checks, locks,
panels, and caps. The only executable correction replaces the v4 monitor's
`Path.is_symlink()`/`Path.exists()` classification with one `lstat()` whose
`FileNotFoundError` is allowed only while the materializer is creating the
private cache. Every other lookup error fails closed. A present target must be
a real directory before traversal, and every post-materialization phase still
requires it.

Source-only scope. No Rust/Cargo build, test, resolver or scientific payload
was invoked for v5; a local Python bytecode syntax check passed and its
generated cache was removed. The v3 terminal failure and unexecuted v4 packet remain
unchanged. Exact executable differences are in `successor.patch`.

Combined: 1800 build + 600 execution seconds, 2 CPU / 16 GiB, one attempt, no automatic retry. Lock gate: one attempt, 60 seconds total, 45 shared resolver seconds, 1 CPU / 2 GiB, private network and pinned version-only rustc proxy.

Pending: independent v5 source review, publication, a post-pivot current ledger,
live owner, serial reconciliation, and a new release. The admission draft stays
false with null current ledger/lease fields. No execution is admitted here.

KB tools unavailable; pinned local primary source and independent F1/F2/F3 review used. No novelty claim.

The prior v3 terminal evidence and unexecuted v4 source remain preserved. The exact
published driver and adapter lock bytes remain unchanged and external to this
source-only lifecycle correction.
