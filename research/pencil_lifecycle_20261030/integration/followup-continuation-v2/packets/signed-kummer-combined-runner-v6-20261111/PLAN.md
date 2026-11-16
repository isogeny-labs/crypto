# Combined qualification v6 overlay-closure successor

Preserves the v5 monitor correction, mathematics, trace checks, locks, panels,
and caps. The source-only correction restores all eight exact overlay bytes
declared by `overlays.json` from the retained v3 packet. Every restored byte
matches its declared post-hash, and all eight overlay files are included in the
v6 semantic manifest rather than depending on ignored/untracked discovery.

Source-only scope. No Rust/Cargo build, test, resolver or scientific payload
was invoked for v6. The v5 materialization failure is preserved: no compiler
ran because the packet omitted every declared overlay file. The v5 `lstat`
correction remains unchanged.

Combined: 1800 build + 600 execution seconds, 2 CPU / 16 GiB, one attempt, no automatic retry. Lock gate: one attempt, 60 seconds total, 45 shared resolver seconds, 1 CPU / 2 GiB, private network and pinned version-only rustc proxy.

Pending: independent v6 source review, publication, a current terminal ledger,
live owner, serial reconciliation, and a new release. The admission draft stays
false with null current ledger/lease fields. No execution is admitted here.

KB tools unavailable; pinned local primary source and independent F1/F2/F3 review used. No novelty claim.

The prior v3 and v5 terminal evidence remain preserved. The exact
published driver and adapter lock bytes remain unchanged and external to this
source-only lifecycle correction.
