# Separate native engine rollback contract

Frozen after the140-input core qualification and before this contract's code
or calls. This is a distinct one-system correctness control, not part of the
old35-system core, not performance/campaign evidence and not an edit to old
qualified source. Core PCC-20261117-1ab7fd and literature3187dc remain unchanged.

The core keeps an explicitly AUTHORED lease/LIFO reader fixture because its
quadratic span can eliminate the original rollback branch before search. That
fixture does not exercise the engine's conflict-to-Undo path. This separate
control closes that runtime coverage gap rather than treating authored counters
as measured behavior.

Public Fp29 System, variables z,u,v, branch role z:
z^2-z=0; z*u+u+v-1=0; z*v+u+v-2=0.
Prediction: z0 exposes two inconsistent affine rows; the engine must admit the
constant explanation, Drop before Undo, then reach a checked z1 root. At root,
the different nonlinear columns prevent the simple constant-weight cancellation
that eliminated the old rollback branch. These are predictions, not observed
results yet. No witness/value enters solve; expected behavior is checked outside.

Use Off and MonomialCancellation with exact core limits/default deterministic
32/64/256slice, not adjusted from observations. Require a returned original
root, actual nonzero engine Undo/Drop counts, valid full native coefficient/
lifetime readback, refusal of omitted Drop/wrong Undo, and full24389-assignment
truth enumeration (including independent original coefficient checking). Keep
complete reports/traces/truth bytes and any failure. No absence certificate,
timing, cost win or curve-pruning inference. Full cold/allocator costs remain open.

New isolated Rust control links the immutable core; freeze its own3 Cargo/Rust
inputs plus the old140context, copied executable and separate qualifier with
distinct build/pre-call/post-call records. This143-input composite context is
separate from core140; no claim of one monolithic archive. Preserve every
core source/fixture/result byte. Native result aggregation only. Publish with
the same report/regular lane PR and restore SAME checker afterward.
