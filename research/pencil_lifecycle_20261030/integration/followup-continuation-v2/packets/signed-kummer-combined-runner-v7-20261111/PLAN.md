# Combined qualification v7 cache-mode successor

Preserves the v6 overlay closure, mathematics, trace checks, locks, panels,
monitor correction, and caps. The source-only correction binds the regular-file
permission mode of every one of the 2,165 donor cache files to the exact frozen
cache byte manifest, rejects special permission bits, restores each mode with
`fchmod` on the private copy, and verifies the donor and copied modes after the
copy. This covers all 82 executable cache inputs, not only the first build
script that exposed the defect.

Source-only scope. No Rust/Cargo build, test, resolver or scientific payload
was invoked for v7. The v6 failure is preserved: materialization passed, then
Cargo stopped before source compilation because the cached `proc-macro2` build
script had equal bytes but copied mode 0600 instead of donor mode 0700. The v6
eight-overlay and 50-driver-source closure remains unchanged.

Combined: 1800 build + 600 execution seconds, 2 CPU / 16 GiB, one attempt, no automatic retry. Lock gate: one attempt, 60 seconds total, 45 shared resolver seconds, 1 CPU / 2 GiB, private network and pinned version-only rustc proxy.

Pending: independent v7 source review, publication, a current terminal ledger,
live owner, serial reconciliation, and a new release. The admission draft stays
false with null current ledger/lease fields. No execution is admitted here.

KB tools unavailable; pinned local primary source and independent F1/F2/F3 review used. No novelty claim.

The prior v3, v5, and v6 terminal evidence remain preserved. The exact
published driver and adapter lock bytes remain unchanged and external to this
source-only lifecycle correction.
