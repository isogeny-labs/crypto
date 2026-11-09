# Prospective C1 trace-line comparison, version 1

2026-10-27. **No affected measurement data have been collected.** The JSON
record and this design are committed prospectively. Execution readiness and
the canonical protocol freeze remain blocked, as listed below. Development
unit tests are correctness checks, not the registered statistical sample.

## Question, prediction and boundaries

Does choosing x in u*F_(2^d) rather than x in F_(2^d) reduce cold, completely
priced one-target C1 four-summand 2LP cost on N=4d binary curves? The upstream
yield heuristic is about 16 times as many relations under a density-halving
approximation, while its optimised cost model predicts only about 0.50 bits.
Both are predictions, not acceptance thresholds or measurements.

The generic floor is sqrt(pi/(2A)) in S=cost/sqrt(r). Reference is matched
single-target rho with negation on the same general binary instance. Any
additional verified automorphism must be included prospectively in a new
reference spec; do not silently weaken rho. The fastest external implementation
comparison is pending; no fastest-implementation claim is admissible.

## Frozen design

- Field degrees: N=8,12,16,20 (d=2,3,4,5); ordinary binary_random constructor,
  seeds 20261027 and independent holdout 20261127, max_cofactor=16. Keep
  every instance, including an unexpectedly all-lift standard line or a failed
  candidate scan. Exact instances/EC1 identities must be frozen before timing.
- Eight independent public targets per curve, target_seed=20261027. Each
  target is a separate one-target workload; no cross-target relation reuse.
- Standard, selected, standard A/A (matched seed, fresh child process), and rho-negation
  arms. Same binary, one thread, same target, caps and inclusion rules; the
  standard base is also the negative/no-treatment control. Correctness tests
  plant known-answer targets through the same collector as positive controls;
  require those tests before every experiment image is admitted.
- Small-base binary dimension d-1, full envelope dimension d, four summands,
  two LP columns; at most 10000 queries, at most 1000000 MITM states for the
  reference; u scan inclusive cap 2^N-1. No tuning from these data. An equal
  column-count study is a separate experiment, not a replacement.
- Six measured rounds and one warmup; alternating arm order. Warmups remain
  in raw records. Algorithm seeds follow ecbench's frozen spec/plan derivation.
  Main and holdout instances are declared before observing any measurements.
- Iso Lab EKS Spot, c8a.2xlarge, L2, four reserved CPUs, single algorithm thread,
  job deadline 1800 seconds including provisioning; no quota or cap changes.
  No new financial campaign cap is inferred from this protocol.

The supplied `reference-d*.json` files implement **only the exact MITM
correctness-reference design**, with both constructor seeds at each size.
They are not launch-ready algebraic C1 specs. N=8 and N=12 unit tests do not
replace the four-size comparison. Numerical power for a 0.50-bit cost effect
is unknown until an authorised separate pilot supplies variance; preregister
any sample-size amendment before confirmatory data, retaining the pilot.

## Complete accounting and evidence

Cold cost includes field/tower setup, multiplier search, both lift signs,
cofactor projection, factor-base indexes and oracle setup. Keep reusable setup
separate as well. Online starts with the first aG+bQ query and includes every
failed trial, algebraic solve, extraction and lift, relation check, LP
combination, rank work, LA, scalar recovery and independent verification.
Keep the five exclusive target phases and verify that they sum to the online
interval. Rho includes walk, collision processing and recovery check.

Record construction counters/time, attempts, accepted full/1LP/2LP relations,
duplicates/rejections, graph vertices/edges/cycles/merges, rank trajectory,
LA work, memory, native field and word operations, calibrated conversion to
GAE, total cold priced cost, S, and IC/rho at weights 0,0.1,1 GAE. Missing
prices or phases force total cost and speedup to null, not zero. No wall-time
claim below L2 or within the A/A interval. Do not substitute the existing GAE
lower bound for the fully priced metric.

Preserve every raw record with protocol/spec/source/image/binary/host hashes,
input EC1/UID and FB identity, resource envelope, algorithm seed, exit status,
clock admission and interruption status. Retain all relation transcripts and
independent replay receipts. Verify raw sums, projected rows, LP certificates,
rank and recovered [k]G=Q. A failed verify invalidates the comparison but is
retained. A cap, timeout or OOM is a failure/censored row, never a win.

Before launch, complete and push the S3 resume checkpoint and pass its gate.
These ordinary request-v1 jobs do not resume; interruption retains counts,
invalidates the clock and requires a new prospectively identified epoch.
Do not retry a submitted or ambiguous request under a new name. Confirm its
original run/Job/Pod identity first. Download and hash-check S3 payloads.

## Analysis and decision

Freeze analysis before data; mask labels in the analysis export until the
scoring implementation has run. Use per-target cold-cost ratios, paired
bootstrap resampling workloads (not rows from shared targets), 95% intervals,
Holm correction over four size comparisons. Report per-target rows, A/A spread,
holdouts and all failures, without selecting favourable sizes. A pipeline gain
requires every phase priced, every answer and relation verified, independent
host replay, interval excluding 1, and effect exceeding A/A noise. A yield-only
change remains a stage diagnostic. Failure to satisfy this is inconclusive or
null within the measured scope; it is not an impossibility result.

Use `experiment.export/v1`, an independent `experiment.review/v1`, validation
gate receipts, source-linked report, diagrams and PDF. No export is written
for executions that never happened. Planned flow is 8 curves * 8 targets *
4 arms * (1 warmup + 6 measured rounds) = 1792 rows; attempted/completed
comparison rows are currently **0/0**.

## Required before confirmatory execution

1. Native algebraic C1 x-line oracle for both bases, checked against the exact
   reference at toy sizes; full native phase pricing and exclusive intervals.
2. Exact instance registry/EC1 records, tested source/image/binary digests,
   native protocol/schema validation and freeze, frozen blinded analysis.
3. Existing authorised Iso Lab submission identity and supported route; valid
   checkpoint upload token or existing authorised AWS role. Do not create
   identities, broaden access or replace the requested experiment with a fixture.
4. Passing development and positive-control tests; independently audited replay
   tooling. Final requirement remains an authorised matched C1 measurement.
