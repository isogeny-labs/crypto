# Bounded Koblitz m3 dataset adapter: integration design

Design checkpoint, 2026-10-30. Task T-1024. Parent source
`3c690cc24a5964c87f031e0342f9a92c16485897`. This checkpoint authorizes no
experiment, starts no generator, and changes no runtime capability. It advances
the saved adapter proposal to the native APIs now present in that source.

## Reconciled prerequisites

The source contains `koblitz_groebner::bounded_s4::build`,
`koblitz_groebner::bounded_matrix::matrix_f4_f2_bounded`, and
`matrix_f5_f2::matrix_f5_f2_bounded`. Their construction and matrix entry points
already have checked work/byte reservations, original absolute deadlines and
fallible boundary observers. Reuse these implementations. The old proposal's
requirement to implement bounded native matrix entry points has been satisfied;
its remaining adapter, semantic replay, and original-input binding obligations
have not.

The parent source's focused validation retained 620 passing, three failing and
seven ignored tests. The three failures are S3 publication/restore integration
checks, recorded in the immutable failure packet referenced by the companion
reconciliation record. They remain a delivery dependency under T-954. No passing
parent full-suite claim is made. Earlier b4f9757 full-suite evidence applies to
that earlier source only.

## Increment and ownership

Implement a private `src/ml_dataset/koblitz_m3.rs` under T-1024. Minimal module,
dispatch and verification registration belongs to the same root's T-887 scope.
Do not edit the T-828 exporter or T-954 publication implementation.

The initial useful matrix profile admits actual Koblitz b=1 curves, m=3,
n in 23..31 or 47..61, two distinct canonical original x coordinates, affine
targets, and an admitted original execution context. The three selector bits give
at most eight monomials per coordinate equation. Both actual twists remain
required. General-binary and larger-factor-base corpus axes remain in the parent
plan; unsupported initial profile cells must retain explicit outcomes.

Weil construction, F4 matrix step and F5 matrix step have distinct identities.
A construction result cannot set `native_backend_invoked`. A native matrix
report cannot set complete-Groebner-basis or exhaustive-decomposition claims.
F5's `SkippedConstantInput` is retained as such; invocation alone is not an
active-criterion or pruning result. Do not route through the old unbounded
recursive Boolean solver.

## Resource contract to implement

The original `SolverExecutionContext` is authoritative. Validate its binding to
original case, attempt, run configuration and capture before curve arithmetic.
Use its single absolute deadline for all phases. The construction subdeadline is
computed once at adapter entry as min(original deadline, entry+5 seconds), and
includes setup. Do not reset it after setup or a refused allocation.

The saved proposal's conversion from configured work to native modeled work is
`min(checked(work_limit * 1024), 16_777_216)`. Record both the original configured
limit and this conversion explicitly. It is a reservation convention, not a
CPU-instruction count. The adapter must establish a single remaining modeled
allowance across construction and matrix reservations, rather than grant the
converted limit afresh to each phase. Preserve native counters in their original
units. Original signed-domain extraction gets an explicit separately recorded
limit in actual group additions; reserve it before native work. Its conversion
and allocation are still an implementation-design obligation, not an already
validated total-work model.

Construction's byte cap remains min(original memory/4, 64 MiB). Its cumulative
reservation is retained when computing matrix headroom. Native matrix caps are
bounded by remaining admitted bytes and the native 64 MiB maximum. Account for
callback-owned data, original equations, input copies, serialized payloads,
returned reports, extraction storage and the first certificate outside the
native matrix's own allocation model. Do not infer complete memory coverage
from the small matrix dimensions alone. Select and document exact adapter-side
reservation constants from the implemented types and serialization paths before
calling the native backends. No guessed blanket memory envelope is approved by
this design.

Pre-admit retained artifact slots and trace bytes. Use a bounded fallible writer
and sticky failures. Stage original equations before any flush that could fail;
never invoke the matrix after a failed input flush. Preserve an already durable
first verified witness across a later cap or observer error. A supervisor-killed
raw prefix is not a sealed result. Reuse existing capture and parent flush paths.

## Artifact identities and replay

Reserve the following distinct paths and roles. Every JSON object uses strict
typed decoding with unknown fields rejected. Events are JSONL; the other three
artifacts are single JSON objects (including group replay's bounded row array).

| Path | Role | Encoding |
| --- | --- | --- |
| koblitz-m3-input.json | restricted-koblitz-m3-original-equations/v1 | JSON |
| koblitz-m3-events.jsonl | restricted-koblitz-m3-boundary-events/v1 | JSONL |
| koblitz-m3-result.json | restricted-koblitz-m3-result/v1 | JSON |
| koblitz-m3-group-replay.json | restricted-koblitz-m3-group-replay/v1 | JSON |

Schemas use the corresponding `crypto.ml-dataset.koblitz-m3-<name>/v1` names,
with `events` for each event record and `group-replay` for the bounded replay
object. Freeze the precise typed field sets alongside their implementation before
producing fixtures. This design freezes identities and encoding, not nonexistent
field definitions.

Input provenance binds original modulus, actual curve coefficients, target,
canonical original factor-base codes, selector offsets and modulo padding, source
profile, execution attempt and original file digests. Certify modulus validity
using the existing native field admission; the bounded builder's structural
modulus checks alone are insufficient.

Historical verification does not read today's clock. Rebuild original equations
under the recorded bounded limits and compare canonical terms. Replay the small
original signed domain independently of S4 zeros: at most four signed points,
at most 64 ordered triples, lexicographic order, repetitions permitted, identity
intermediate sums permitted. Charge each actual group addition including identity
starts. A negative requires the exact end cursor and no failure. A positive
requires a sum equal to the original target and original membership checks.
Never infer completeness from `used >= limit` or matrix completion.

Checking returned rows on all Boolean assignments establishes finite-assignment
consistency only. Set `native_basis_derivation_verified=false` until an actual
derivation checker exists. Do not silently strengthen this claim to ideal
membership, full row-space equality or a complete solver trace.

The central verifier must require path/role/encoding bijection from original
profile/config, even if all producer completion flags are removed. Reject missing,
duplicate, relocated or aliased artifacts and rehashed semantic mutations.
Censored prefixes have an explicit admission path. Existing outer supervisor
outcomes continue to override worker completion labels.

## Validation and activation order

1. Implement typed contracts, checked aggregate resource planning and bounded
   serialization; test overflow, original binding, exact/one-less limits and
   sticky failure. These are software controls, not scientific draws.
2. Implement bounded construction plus independent original-equation replay.
3. Integrate each named native matrix step with actual report/counter retention.
4. Add independent signed-domain extraction, first-certificate persistence and
   central semantic verification, including exceptional infinity prefixes.
5. Freeze concrete fixture coordinates before running the proposed controls at
   n=23,31,47,53,61 on both actual twists: domain +/-P,+/-2P with ord(P)>13;
   targets 3P,7P,P. Verify preconditions without selecting replacement points.
   Require nonempty useful native reduction. Preserve a fixed-control failure;
   F5 active-criterion evidence is a separate gate.
6. Add mutation, sink, deadline, original-resource and supervisor-prefix controls;
   require focused validation and applicable repository gates before activation.
7. Coordinate exporter and consumer schema support with their owners. Until then,
   do not claim S4 graph export, consumer arithmetic replay, or ML-ready coverage.

No cloud execution or budget renewal is part of this increment. Preserve the
original total USD 200 ceiling and expired campaign deadline. Software evidence
and failures continue through the existing authorized S3 custody path, with
conditional immutable upload and fresh byte/hash readback.

## Implemented resource-ledger increment

`src/ml_dataset/koblitz_m3.rs` now implements the cumulative ledger; runtime
registration remains pending the original-input binder and adapter pipeline.
The initial standalone harness compiled that exact module with Rust 1.98,
edition 2021 and warnings denied: nine boundary controls passed. That harness
was retired when original-context admission introduced real dataset-module
dependencies; the module now registers under solvers for library tests only.
Its runtime profile remains inactive.

The extraction allowance is fixed at the original configured `work_limit` actual
group additions, matching the existing prime adapter's separate addition limit.
Native modeled work is capped at the checked 1024 conversion and shared across
construction/matrix. These are two explicitly different units and limits, not a
single retired-instruction budget. Byte and trace reservations never refund;
callback-owned memory reduces the remaining native headroom. Exact end-cursor
validation remains the extraction caller's responsibility.

The ledger accepts an already admitted original deadline and original limits.
It does not itself certify provenance. The next increment must bind these inputs
to `SolverExecutionContext` and validate original config/attempt/capture before
calling it. Adapter-specific allocation constants, typed serialization, native
observer wiring and independent replay are still required before activation.

Controls cover shared phase work/memory, repeated/regressing cumulative reports,
callback allocations after a native allowance is issued, setup time inside the
construction deadline, the unchanged original matrix deadline, exact versus
one-extra group additions, phase order, arithmetic overflow, zero limits, trace
caps, sticky observer errors, and rejection before mutation at native caps.


## Original-context admission increment

`admit_original` samples its construction entry time before reading original
files. It compares the passed config to saved config, invokes existing
`execution::admit` (including original run/attempt binding and boot/clock checks),
compares the passed launch context to that result, invokes existing
`admit_capture_context`, and checks the supplied actual `CaptureEmitter` against
that binding and those limits. The ledger deadline is the minimum of the passed
and freshly admitted absolute deadlines. Setup time therefore cannot be reclaimed
by rereading the context. Admission rechecks the deadline after file reads.

This increment binds resource inputs and sink identity. Actual curve, target and
factor-base mathematical admission remains the next adapter stage; the function
does not claim to validate those by reading configuration alone. It performs no
native construction, matrix solve or dataset generation. Integration controls
use frozen synthetic run files and the real context/capture APIs.

## Original mathematical input increment

`original_math` reuses existing public curve and factor-base replay, then checks
actual binary degree and b=1, extracts the affine target, and canonicalizes the
four signed points into exactly two sorted x codes. Both signs are checked with
the binary negation rule `(x,y) -> (x,x XOR y)`. Field exponents are sorted only
after the existing native irreducibility/duplicate checks have succeeded.
Private witness data is never accessed. Runtime registration remains test-only.

The original `factor_base_size` means signed-point count, not distinct-x count.
The default generator processes both lifts at a source x before checking its
size target. Thus a requested size of two can be exactly P,-P (one code).
The new mathematical controls use four signed points and assert two actual
codes; two signed points are a retained rejection control. This changes neither
an existing frozen corpus nor any existing data. The adapter must report the
actual signed and x-domain cardinalities separately when activated.

The new controls are software input-validation checks, not the later frozen
matrix-reduction experiment. They use fixed existing-generator parameters at
n=23,31,47,53,61 and both actual twists, compare canonical inputs with the
original case, alter private witness contents, and reject coefficient, target,
twist, duplicated-point, modulus, requested-size, family and arity mutations.
No native S4 or matrix work is performed by these controls.

The existing public/factor-base verifiers include construction replay and,
when the subgroup divides the cofactor, a cyclic-membership table. Their setup
cost is outside S4 reservation counters. This staged function is not a proof
that the whole adapter is resource-bounded. Runtime integration must admit and
account for those paths (or reuse independently verified original-input state
with an exact provenance binding) before calling them under the profile. Do not
silently count their work as zero or grant fresh phase budgets. Typed persistence
and independent equation/group replay are still outstanding.

## Bounded S4 bridge increment

`construct_original` now invokes the existing bounded native S4 builder with the
ledger's remaining construction allowance and its original construction deadline.
Every cumulative native reservation is observed by the same ledger before the
external callback. The external callback may reserve its own retained memory and
trace bytes through that ledger. Its error is sticky; even a callback that ignores
a failed reservation and returns success cannot suppress the ledger failure.

The returned construction record preserves native failures separately from
adapter failures. A native failure seals the ledger against later matrix work.
If the native builder returned original equations but the adapter's final
boundary fails, the returned equations remain available for typed persistence.
Construction alone never sets a matrix-invocation or decomposition claim.
The bridge does not yet implement artifact serialization or matrix dispatch.

The mathematical preflight now caps candidate replay at255 rows and2 lifts per
row before invoking generic verification. Controls include oversized arrays.
These are structural limits; they do not replace the remaining complete setup
accounting. Three bridge controls exercise actual construction and cumulative
headroom, one-less native work, callback refusal, swallowed trace-cap errors,
and an already expired construction deadline before any native invocation.

## Input persistence and named matrix bridge

`persist_original` serializes an explicit borrowed InputView through a fallible
writer, with at most64KiB of pre-admitted buffer capacity. Every emitted byte is
charged against the shared trace allowance before appending. It records the
attempt binding, original public/factor-base digest strings, actual curve/field,
target, sorted codes and signed domain, selector offsets, equations and original
resource ceilings. Integer coordinates are exact u64 JSON integers; downstream
consumers must not decode them through floating-point numbers. The caller must
bind digest strings to original files; format validation alone is not provenance
validation. A strict historical input decoder and semantic replay remain pending.

Only successful serialization, a successful persistence callback and a final
ledger check produce PersistedInput. This token borrows both the exact native
output and the same ledger exclusively. The runtime callback must publish the
reserved koblitz-m3-input.json role through bound capture; its actual binding is
an outer integration requirement. Unit controls inject callback failures.

A failed callback returns the complete serialized bytes for parent retention;
a serialization cap returns its bounded incomplete prefix marked incomplete.
Neither case grants a matrix token. The original native equations remain owned
by the construction caller. A successful token is consumed by matrix_step,
which invokes the selected existing bounded F4 or F5 engine with remaining work,
remaining memory, the original deadline, max488 main row attempts and4096 events.
Each callback shares the same ledger and sticky-failure behavior. Native report,
precise stop, actual engine identity and criterion status remain intact. Returned
original bytes survive matrix failure. These steps do not claim a complete
Groebner basis, recursive solver, or exhaustive PDP verdict.

New software controls exercise both named engines on original S4 equations,
nonempty native row/reduction work, cumulative reservations, exact/one-less input
trace capacity, input I/O failure with byte retention, and matrix observer
failure with no later extraction admission. They do not replace the independent
signed-domain replay and frozen matrix-control panel required before activation.
