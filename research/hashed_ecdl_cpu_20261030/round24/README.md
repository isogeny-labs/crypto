# Round24: early residual rejection comparison

## What was done

The panel retains all three fixtures, seeds111/222/333, 16-trial and
1,000,000-work caps, one software thread, one warmup and nine measured
repetitions per cell: 180 executions, 162 measured, 18 warmups.
u/x are the original baseline A/A pair. v/w use early residual rejection from
b19d9b3beae66488c54ca50ec28b065e5683eccb at batch64/batch1. y/z reuse the
verified round23 candidate from 4939261c9fcc48bf86f17ac36fa458f52a155b2e,
with suffix lookup filtering only, at batch64/batch1. No profiler clocks.

The source delta contains residual x rejection, scratch flags and its affine
reference test. The batch inverse recurrence, exact lookup, witness order,
row/work accounting and full certificate replays remain mandatory. y/v and
z/w isolate early rejection at fixed batching, including extra branch and
scratch cost. u/v preserves comparison with the original baseline. No earlier
timing rows are pooled.

## Decisions made

Runner and analyzer are unchanged from round23. Fixed rotation/reversal,
positive and corrupted-scalar controls, complete timing and full certificate
byte equality remain. Native exporter admits round24 as an ordinary panel;
round22 profile handling is untouched. L0 clocks are descriptive, not isolated
tenfold acceptance. Allocation failure or missing cells invalidates the panel;
fixtures are never dropped or replaced.

Candidate is archived from its published commit and built standalone with
offline, locked release Cargo. The original baseline, suffix-only parent and
analyzer are reused from verified custody. Materializations and manifests bind
all executable identities. This does not build the root crypto/CUDA plug-in.

## Open threads

No upload, job or study timing data. Execution requires a new single-attempt
authorization; round23's authorization is consumed. Proposed bound: 2 vCPUs,
4 GiB, 300 seconds, zero retries, normal Spot autoscaling, one-node cap unchanged.
Before launch reconcile exact job hashed-ecdl-cpu-round24-20261030 and its
input/result prefixes and eligible capacity. Preserve any charged failure.
Publish raw records before a separate inspected report after execution.

## Errors

An early log inspection ran while source archival was still in progress and
found no build log yet. This was an observation only; the existing preparation
process was retained, not restarted. Validation outcomes are recorded below.

Reference basis: residual-filter/README.md (exact x rejection and affine
equations), oracle-filter/README.md (Bloom reference), and the completed round23
protocol and report. No new arithmetic, novelty or speedup claim is made.
