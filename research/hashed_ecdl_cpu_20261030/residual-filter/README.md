# Reject residuals before completing y

## What was done

Round23 measured lower complete-cost medians for the exact suffix filter at
both batch sizes on all three fixtures, with overlap on fixture0. This candidate
extends the same filter into residual construction. Once nx is known, test
both compressed keys 2*nx and 2*nx+1. If neither bit is set, no finite suffix
point at that x can be present, and the final y multiplication and two
subtractions can be skipped. A present bit remains only a candidate: compute
y, then perform the existing exact suffix lookup and checked tuple sum.

The two bits share a bitmap word because the index capacity is a power of two
at least two, so the first bit is even. Identity's sentinel may collide with
these bits; this causes only extra work. Identity and exceptional additions
retain their ordinary outputs and exact lookup. nx is canonical and below
2^62, so doubling it cannot overflow u64 in the admitted oracle domain.

## Decisions made

Every nonzero denominator updates the inverse recurrence before any early
rejection. A scratch boolean vector marks unfinished output slots; the query
loop skips those slots before exact lookup. It resets for every batch,
including identity-target and unfiltered preparation calls. Prefix order,
first witness, row counters, work caps and both replays remain unchanged.
Scratch remains excluded from the explicitly scoped retained-table statistic,
as it was before; this additional storage is real overhead to include in a
future complete-cost comparison. No new retained suffix table is allocated.

This saves one field multiplication and two modular subtractions per rejected
ordinary residual, but adds a two-bit check, reject flag writes and a query
branch. The tradeoff is unmeasured. No performance or tenfold claim follows
from source counts alone. No paid experiment has been launched.

## Checks and reference basis

Read oracle-filter/README.md and its Bloom 1970 primary-paper reference,
round23/EXECUTION.md and recorded complete-cost summaries, plus the existing
Scratch::subtract equations and independent affine-reference tests. This is
an extension of the exact prefilter construction, not a new curve formula.
The new test checks all pairs in the 199-point fixture group and sampled
65521/68719476731 fields with empty, sparse and full suffix tables. Rejected
rows must be finite and absent from an independent linear membership search;
completed rows must equal independent affine subtraction. Identity-target
unfiltered reuse must clear all rejection flags. Existing certificate and
wide-field witness tests remain mandatory.

## Open threads

After software validation, freeze a comparison against the measured round23
candidate before another bounded run. Round23's single-run authorization is
consumed. Preserve all fixtures, original baseline and complete-cost controls.

## Errors

Validation logs accompany this note. No historical bundle or source snapshot
was modified; software checks use a separate scratch source tree.

## Validation outcome

48 standalone release library tests pass, including all archived certificate
fixtures and the new affine-reference rejection test. Strict all-target release
Clippy, formatting and example target compilation/test invocations pass
(examples register zero tests). Authored-source whitespace checks pass. No
validation failure occurred. The source hash and software logs are retained;
these are correctness checks, not performance samples.
