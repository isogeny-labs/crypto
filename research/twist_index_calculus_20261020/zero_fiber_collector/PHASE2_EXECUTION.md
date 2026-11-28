# Phase 2: stopped zero precomputation and actual target descent

Prospective execution supplement, 2026-10-24. The original
[protocol](PROTOCOL.md#phase-2-complete-one-target-pipeline) remains binding.
This supplement fixes implementation boundaries before any Phase 2 solve.
The completed 189+21 panel and Phase 1 physical censuses are not resubmitted.
New executions compare changed complete one-target pipelines and their
matched controls; old records supply post-run physical references only.

## Registered workload and controls

Keep all seven primes and seeds 0,1,2, with the original sealed 13-key
public inputs and entire original query caps. The known scalar remains 271;
these are seven curves in three presentations, not independent secrets.

For every case execute zero-precomputation plus descent for phi and doubling,
and matched plain phi and doubling without zero precomputation. Also freeze
the best admitted successful cold pipeline in the parent's ten-arm study,
using its existing `results/cold-costs.csv`, SHA-256
`7d47e6fc77d6dcb631b0bfd0cab0b5e93cd328ad9a9e716e35fdb8d5399f3919`.
The Rust selector requires every original case/arm, excludes censored or
unverified rows, compares exact integer nanoseconds from the recorded
five-round median, and breaks ties by the original arm order. Its output
and runner hashes must be committed before comparison runs.

This is the best complete cold control in that verified parent family.
Other documented methods, including thinned bases with incomplete cold
prices, and a globally strongest IC/rho price remain explicit obligations;
this family selection does not discharge them. No new timing admission is
inferred from old admitted measurements.

Use first/A/A for every distinct registered `(case, mode, arm)`. If the
frozen best plain arm equals the registered plain phi or doubling arm, use
that exact execution as both controls rather than launch it twice. Fix that
reuse from the frozen selection, never from new candidate outcomes. Keep
all losses, timeouts, partial results and independent audit failures.

## Candidate and accounting boundaries

The candidate receives public input, expected hash, arm, mode and fresh
output only. It reads no reference, old physical set, factor logs, BSGS,
baseline cost table or known scalar. Separate reusable base reconstruction
from target validation: the former must not use target coordinates or query
values to choose factors, images, coefficients or action representatives.
Charge target curve/subgroup validation when the online phase starts.

For the zero mode, construct and check the same canonical signed-pair
quotient as Phase 1, solve S4 in canonical pair order, process the complete
system's roots/signs and insert its transported physical rows. Stop at the
first completed system whose factor-log matrix has full factor rank;
otherwise exhaust the registered pair set. This is a system-boundary stop,
not a claim of an optimally interrupted root decoder. Verify determined
factor logs by actual point multiplication. All zero target coefficients
must remain zero.

Carry the zero rows into the full anchored target matrix. Generate each
original target residual, solve all signed peels for that query, and stop
at the first complete query attaining full target/factor rank, or its
original cap. Infinity branches remain explicit. Verify the target and
every allowed physical factor point. Plain controls use the same decoder,
query order, journaling and final verification policy.

Each finite solve receives the remaining cooperative deadline, including
failed work. Durable input journals precede F4 and result journals follow
decoding. A killed case leaves its completed prefix and pending system.
Each finite input is bound by its context, remaining point and SHA-256 of
the exact normalized F4 polynomials. The post-run verifier reconstructs
those polynomials and checks that signature before checking roots. Full
deterministic coefficients need not be duplicated in every journal/result.
Separate setup, zero precomputation, target-query generation, decomposition,
matrix work, back substitution, point verification and journal intervals.
Record allocation calls/bytes and peak live Rust bytes separately from
process RSS. Include action construction, trees/hashes, failed systems and
verification in the operational interval. Output serialization, input
decoding and process startup remain explicitly identified.

The migrated shared host permits correctness/counter diagnostics only.
Elapsed values, including separately recorded journal overhead, do not
establish an admitted speedup. Field/group counters plus F4 row-reduction
counters remain partial until missing primitives and 0/0.1/1 GAE are priced.

## Independent replay and gates

Post-run verification binds public input and immutable reference hashes.
Compare every processed zero system with its complete Phase 1 counterpart,
reconstruct transports using scalar-multiplication action maps, and compare
every processed target query with the parent's full-stream physical set.
Use separate RREF for all rank checkpoints, first stopping positions and
solutions. Verify physical factor points and the target independently.

Reject changed public/reference seals, omitted or duplicate systems,
altered work sums, wrong stopping positions, altered rows/signs, missing
target coefficients and forged verification flags. A/A compares deterministic
records while preserving each run's diagnostic clock and memory telemetry.
An incomplete solver remains incomplete even if later rows recover a scalar.

Keep one science worker, 8 GiB, the 1,200-second outer watchdog and
1,180-second cooperative case deadline. Compile separately under the shared
busy reservation without the address-space cap. Freeze the actual runner,
source, protocol, public inputs and references before running. Drivers
reserve each phase and must reject a nested outer busy wrapper.

Native tests and small positive/regression checks precede the full panel.
Every executed research round retains raw records, a report, diagram and
inspected PDF. Publish a new versioned raw archive; do not rewrite either
completed study's archive. All applicable CI, review, Conductor, store and
VALIDATION rules remain. A verified pipeline or a count reduction alone does
not close the broader twist-symmetry or complete-price goal.
