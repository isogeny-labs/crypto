# Native filter trace increment

Design frozen before implementation on 2026-10-28, base runtime
`d79f658f7a0a83e2f75f4cb6f0d1f7542d0e6b83`. Root approved the conservative hard
ceilings from `PROGRAM_DESIGN.md`. Conductor T-748 reserves only the native
filter source, its new child-module directory and this research directory.

This increment implements the native API, independent source-combination
checker and native tests. Dataset v2, signed-Frobenius Koblitz reduction,
modular-rank internals, cloud generation and scientific results are excluded.
Root owns the session log and publication. No original budget or deadline is
extended. The accompanying source map records inspected primary literature.

The existing `filter_large_primes` body remains unchanged as the compatibility
baseline. A separate bounded traced runner reuses native row arithmetic but
adds admission and event boundaries. The independent checker uses separate
coefficient arithmetic and validates the recorded schedule. Maintaining two
producer schedules is an explicit risk: complete runs must match every legacy
report field and output row in tests. No legacy caller acquires trace limits.

Events describe normalized inputs, duplicate representatives, degree-one
peels, ordered core/static degrees, row starts, pivot choice, candidate AXPY
and its accepted/rejected fill decision, raw stored pivots, completed source
combinations and final report. An AXPY and its fill decision may share one
atomic event so a refused sink cannot commit either half of the decision.
Every row/pivot reference binds its producer input ID and reduction version.
The actual valid peel order is retained without promising HashMap order.

Admission caps: at most 4096 raw input rows, 1000000 raw input terms, 65536
terms in a captured or tentative row, 100000 events, 16777216 compact JSON
event bytes and 1000000 logical work units. Requests may lower these limits.
Input admission is checked before hashing/cloning. Conservative operand-size
reservations bound candidate construction before it occurs; a reservation
stop does not assert that the candidate's final nonzero size exceeds a cap.

Logical work charges only defined normalization-term visits, pivot-selection
term visits, peel degree updates, core degree visits and sparse AXPY operand
term visits. Inversions are separately informational. Hashing, validation,
source replay, coefficient arithmetic internals, map comparisons, allocation,
serialization, snapshots, queue scanning, outer control and group/rank work
are excluded. These are software admission units, not total field or CPU cost.
Initial incidence construction is also excluded. The input digest binds compact
JSON of the ordered typed input before coefficient normalization, rather than
original JSON whitespace. A raw input-admission stop deliberately has no digest
or emitted event: only size admission is checked, and no mathematical input
binding or row identity is claimed for that rejected input.
Computed work on a byte- or sink-refused transition remains separately visible;
its row/report changes are not committed. Callback failure stops immediately.

The verifier checks raw ordered input and options/limits binding, canonical
rows, each source identity, legal event order and transition, aggregate
accounting and the precise resource/failure reason. A complete run requires an
acknowledged Finish; a prefix or failed Finish is censored. Mathematical input
errors remain errors, with emitted bytes retained by the caller. Neither a
trace cap nor the filter's existing fill rejection proves UNSAT or recovery.

Tests cover original result/report equivalence, independent dense projected
row-space checks on bounded small examples, normalization/peeling/ties/fill,
every event and callback boundary, byte/work/state/input limits, tampering,
invalid/composite legacy modulus behavior and serialization round trips.
The parent coordinates one centralized Cargo owner. Full-library execution
will be coordinated separately after the targeted tests and strict lint.
