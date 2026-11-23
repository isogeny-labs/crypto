# Normalized recovery accounting: instrument and development checks

Native execution: 2026-11-05. Reconciliation/publication: 2026-11-07. T-1727.
The interrupted process handle was gone; the saved receipt records both build
and test exit0, with a complete test log. No native execution was repeated.

## Instrument contract

Fifteen source modules are adapted only by explicit module/include path routing,
helper counters and lexical stage guards. The recorded edit map reconstructs
each generated file from the frozen source and reverses it to the exact original
bytes. The read-only verifier checks both hashes and transformations and rejects
changed original/generated inputs for each module. The native dependency lock
includes Rust include! files and all inherited fixtures. [pcc:unverified]

Counters separate 14 stages and12 components. They record modular-product helper
calls, field and scalar inverse calls, Euclidean iterations, popcounts, correction
rule comparisons, residual calls and allocator events. A fixed thread-local array
avoids counter allocations; overflow and reentrant missed updates are rejected.
Lexical scopes restore on return/drop. Nested setup work remains in calibration,
preparation or compilation instead of leaking into query-phase buckets.
The allocator forwards alloc, alloc_zeroed, realloc and dealloc to System.
Requested bytes include realloc requests; freed bytes follow dealloc events.
Neither number is peak memory or net retained bytes. [pcc:unverified]

Initial transport includes the two direct initializations when needed and column
updates. Initial character includes geometry-only residuals, homogeneous factors
and calibration comparisons. Extra transport includes column updates even for a
skipped character; extra character is entered only when evaluation is requested.
Public verification has its own counted affine multiplication routine. Setup,
compilation, query and verification are printed separately. [pcc:unverified]

## Development checks and raw projection

Three accounting tests passed;414 imported tests were filtered. The instrumented
path agrees with the frozen uninstrumented normalized recovery on20 public-point
fixtures, and all20 recovered scalars pass forward multiplication. Stage checks
assert actual initial phase/value calls, absence of value initialization inside
residual evaluation, extra geometry inversions,24 transport products per
non-anchor extension in these fixtures, skipped-character behavior and scope
restoration. Allocator forwarding and thread isolation are checked. [pcc:unverified]

The transformation verifier accepts15 forward/reverse module mappings and rejects
30 byte corruptions. Fifty-three source/data hashes bind the build;60 compiler
warnings are retained. Projection contains1140 unique raw counter records,
including the first count record prefixed by Rust's test name. The projection
checker requires one row per raw marker and exact equality with counts.json.
No row is inferred from an earlier run. [pcc:unverified]

## Recorded example and next construction

For the fixed F509/r479 fixture Q=[25]P (P=(3,68), curve y^2=x^3+2x+10),
initial geometry records6818 product-helper calls. Initial transport records3100:
2250 phase,488 value,362 transport. Fourteen initial residuals record12138:
7714 polynomial and4424 source products. Two extra geometry steps record916;
extra column transport48; the one evaluated extra residual867 (551 polynomial,
316 source); forward verification122. These are disjoint helper-event counts,
not a timing model. [pcc:unverified]

The compiler records24076995 product-helper calls. Calibration records5974 and
preparation818, all separate from per-query work. Initial character allocation
for the Q=[25]P fixture requests218526 bytes; this is cumulative requested volume,
not simultaneous memory use. These records expose residual/polynomial work and
allocation alongside transport costs. [pcc:unverified]

Next test combined-monomial transport and geometry-only residual simplification,
then bind a matched complete-cost comparison. This packet has one instrumented
candidate and an uninstrumented correctness reference. It does not provide a
matched performance comparison against the batch-reciprocal implementation.
Adds, branches, moves, hashing, some inline arithmetic and machine costs are not
covered by the helper counters. No wall-clock speedup, complete machine-work
claim, statistical experiment, cloud job or timing panel is asserted.
The original4563-case research cohort remains unchanged.

## Sources checked

KB unavailable: relevant MCP tools absent; scripts/research-kb status reported no
executable. Read the previous batch/quartic accounting generators, meter,
transformation verifier, projection parser, and normalized recovery source.
The existing GlobalAlloc forwarding contract and stage patterns were retained;
new transport/include routing and calibration-preserving nested guards were
checked explicitly. This is instrumentation development under AGENTS.md, not a
new normalization theorem or a comparative experiment.
