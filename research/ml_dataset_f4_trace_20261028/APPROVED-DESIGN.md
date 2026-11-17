# Review refinement: exact first native slice and resource admission

This 2026-10-28 follow-up narrows the first implementation unit in DESIGN.md. It is preparation only: no implementation, build, measurement or solver execution is authorized by this document. The reviewed source remains d79f658f7a0a83e2f75f4cb6f0d1f7542d0e6b83. Later integration must reconcile the then-current producer source and rerun ownership checks.

## First slice: one Boolean F4 matrix call, two native files

Recommend exactly these implementation files, on a newly assigned isolated branch:

1. `src/cryptanalysis/koblitz_groebner.rs`: add an opt-in typed traced entry point alongside `matrix_f4_f2_counted`, and thread a fallible sink through the existing candidate visitor, nested packing and suffix elimination path.
2. New `src/cryptanalysis/algebra_trace.rs`: strict event/header types, explicit limits, bounded sink abstraction and stop/result types. It can initially be included with a path module inside koblitz_groebner, following the existing sibling-module pattern, so this unit does not need to edit `src/cryptanalysis/mod.rs`.

Proposed API shape: `matrix_f4_f2_traced(polys, n_vars, degree, binding, limits, sink) -> Result<TracedMatrixOutcome, TraceError>`. Keep the current non-traced API and its behavior intact. `TracedMatrixOutcome` distinguishes complete matrix, censored prefix, unsupported profile and native size refusal; an I/O/integrity failure is not a completed outcome. The new API takes explicit binding/limits and a caller-owned fallible sink. It does not install a global dataset sink or expose a CLI yet.

The first profile is `boolean-macaulay-f4-suffix/v1`:

- One call, q <= 8 Boolean variables and D <= 4, no recursive search, no degree ladder, no inherited basis, no algebra cache, no F5 criterion, no prime F4.
- At most 64 ordered input polynomials, each normalized in the Boolean quotient and with at most 256 terms. Reject noncanonical terms/masks at this API boundary. Candidate enumeration includes zero products and has a hard cumulative 4,096-candidate cap; exceeding it is censored rather than equivalent to a smaller complete matrix.
- All q variables are admitted multipliers and full reduced-row readback is requested. Use the existing nested candidate/packing path. Any environment-selected alternate layout or criterion that the trace cannot cover is rejected as unsupported before the native operation; do not silently choose a different path.
- The column universe has at most sum(k=0..4) C(8,k)=163 monomials. At pinned lines 4375-4401, fewer than 256 columns selects the existing native column-at-a-time suffix kernel automatically, regardless of row count. This restricts input shape rather than switching kernels. No gf2_elim.rs/M4RI changes are needed in this first unit.
- Events cover actual candidate order, source/multiplier/parity normalization, zero rows, complete column/packing map, pivot search result, row swap, suffix XOR and final rows/rank. No auxiliary scalar elimination may be presented as the native trace. A suffix XOR certificate checks the full-row identity, including the omitted leading-word region.
- Each event uses immutable row versions. Emit actual post-operation data only after the operation, acknowledge it before the next bounded block, and retain only the last acknowledged prefix after failure. A sticky sink error exits the native call. Existing aggregate counters remain separately identified observations.

This deliberately does not claim to cover every admitted dataset Boolean input: the adapter currently permits q <= 12 and D <= 6. Those larger rows remain in the campaign with their existing coverage status; this unit advertises only its own smaller trace profile. The next native step can expand actual M4RI hooks, then F5 witnesses. Adapter integration and independent public S3/selector reconstruction follow separately after ownership and source reconciliation.

## What its checker can establish

Implement a separate small checker in the independent consumer owner’s branch or a dedicated standalone review crate. It may be prepared from the schema without editing these two native files, but its final schema bindings must match exactly. It must not import producer row normalization/elimination functions. This is a separate ownership unit, not an unnoticed third file in the first producer patch.

For this slice, the checker verifies F2 mask-union/parity arithmetic, ordered candidate enumeration, column/padding map, supplied pivot/XOR operations, complete final matrix and rank. It can establish equality of the declared finite matrix row spaces when the whole bounded call is present. Its root input is an exact ordered polynomial artifact and binding; public S3/field/selector reconstruction is outside this slice. Therefore `input_encoding_checked=false` for an unaccompanied native trace, global UNSAT and solver-completion flags stay false, and no PDP/scalar/fall-degree label is produced. This does not weaken the broader design’s eventual encoding checks.

## Finite resource accounting

The broad ceilings in DESIGN.md are independent maxima, not an assertion that they fit in one allocation. This first slice instead uses all of the following caps simultaneously, each reduced to fit the caller’s already admitted remaining budget:

| Item | First-slice ceiling |
|---|---:|
| Input polynomials / terms per polynomial | 64 / 256 |
| Candidate products including zero products | 4,096 |
| Columns / nonzero matrix rows | 163 / 4,096 |
| Packed row width / matrix bytes | 3 u64 words / 98,304 bytes |
| Algebra events / single serialized event | 32,768 / 64 KiB |
| Total event stream | 4 MiB |
| Matrix/source/temporary polynomial arena | 2 MiB |
| Provenance and immutable row-version arena | 8 MiB |
| Concurrent event batches | 1, on the calling thread |
| Component accounted allocation ceiling | min(caller remaining allowance, 32 MiB) |

The 32 MiB is a component allocation ceiling, not an RSS claim for the whole binary. Charge actual vector capacities, row-version storage and hash/serialization workspace before allocation. Reserve capture copies, chunk/terminal metadata and bounded sort/product scratch; do not assume sparse data or allocator capacity equals logical length. A 4 MiB acknowledged stream plus its immutable copy already consumes 8 MiB before other state. If the existing capture reservation is smaller, reduce the stream bound further or refuse admission. Consumer checking runs separately with its own explicit allocation and work limits; it does not silently share a producer allocation.

Use checked arithmetic for all products and counts. At most 4,096 products of at most 256 input terms bounds raw monomial visits by 1,048,576; parity normalization and sorting also need explicit bounded workspace and cancellation checks. With rank <= 163, an elimination has at most 163*4,095=667,485 target-row visits and at most 2,002,455 three-word XORs, before pivot scans and input work. These are conservative algebraic loop bounds, not machine instruction counts or predictions that the event budget completes them. The stream/event limit may censor well before those maxima.

Check the caller’s existing deadline at each candidate, pivot scan chunk of at most 256 rows and row-update chunk of at most 256 target rows, and propagate cancellation into source serialization. An event must not require serializing an unbounded matrix: emit bounded row/block records. Keep the existing supervisor’s hard deadline and terminal reserve. A cancellation check does not prove a wall-clock latency bound until tested under the selected native implementation and scheduler.

No new cloud launch, runtime allowance or USD allocation follows. The parent’s USD 200 campaign cap and cumulative dataset/S3 authority remain unchanged. This design archive itself is a small, separately receipted immutable artifact; it is not a generated scientific dataset cell.

## Ownership and current-source blockers

At the live Conductor check, T-751 held exclusive `src/ml_dataset/curves.rs`, `src/ml_dataset/solvers.rs` and `research/ml_dataset_exact_cofactor_20261028`. The read-only advisory returned only the solvers.rs overlap among the queried native/adapter paths. This does not reserve the apparently free native files for later edits: recheck and claim the exact two files immediately before any approved implementation.

Do not edit solvers.rs, curves.rs, capture/live/export files, consumer owner files or the producer’s current worktree in this slice. The parent reported an active producer Cargo test against source 1bee3b9...; a failed Conductor card does not authorize takeover. Root remains the Cargo owner. This reviewer did not inspect or interfere with that process.

Before implementation: obtain parent approval of this narrower schema/profile, reconcile the current native source against d79, inspect the actual optional layout/criterion resolution so unsupported profiles fail before work, assign ownership of the two native files and independent checker, and centralize meaningful tests under the current build owner. No mathematical blocker was identified for checking these finite row identities. The unresolved work is implementation, cancellation/resource enforcement, independent checking and eventual adapter/publication integration; none is claimed complete.

Minimal tests for the first unit are independent tiny matrix equality/rank checks, zero and constant inputs, q=0, quotient product collisions, candidate order, row swaps and suffix boundaries, rehashed malformed witnesses/maps, every sink failure boundary, limit/deadline censoring, and unchanged untraced behavior. No experiment or benchmark is required to establish this unit’s arithmetic contract; performance claims remain out of scope.
