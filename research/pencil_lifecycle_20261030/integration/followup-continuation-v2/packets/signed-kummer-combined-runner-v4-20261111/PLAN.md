# Combined qualification v4 lifecycle successor

Preserves the reviewed v3 mathematics, source inventory, trace checks, locks,
panels, and caps. The only executable correction allows the build monitor's
target directory to be absent while the materializer is creating the private
cache. Every post-materialization phase still requires that target directory,
and an existing target must be a real directory before traversal.

Source-only scope. No compiler, test, resolver or scientific payload was
invoked for v4. The v3 terminal failure remains unchanged: materialization was
killed after 0.115 seconds because the monitor traversed the not-yet-created
target directory. Exact executable differences are in `successor.patch`.

Combined: 1800 build + 600 execution seconds, 2 CPU / 16 GiB, one attempt, no automatic retry. Lock gate: one attempt, 60 seconds total, 45 shared resolver seconds, 1 CPU / 2 GiB, private network and pinned version-only rustc proxy.

Pending: independent v4 source review, publication, a post-pivot current ledger,
live owner, serial reconciliation, and a new release. The admission draft stays
false with null current ledger/lease fields. No execution is admitted here.

KB tools unavailable; pinned local primary source and independent F1/F2/F3 review used. No novelty claim.

The prior v3 source and its terminal evidence remain preserved. The exact
published driver and adapter lock bytes remain unchanged and external to this
source-only lifecycle correction.
