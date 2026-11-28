# Before-data amendment 3: measure the batched collector in wall time

Status: specified before any study execution or outcome row, 2026-11-15.
This amendment changes the primary decision metric of
[`PROTOCOL.md`](PROTOCOL.md) for the scalar-versus-batch study. It leaves
[`SPEC-K13.json`](SPEC-K13.json), the six arms, eight public targets, six
measured rounds, warmup, source-bound amendments 1 and 2, correctness
controls, resource limits and failure retention unchanged. The original
protocol and all publication receipts remain immutable. Execution under
this amendment requires a new pushed-source receipt that includes it.

## Reason for the correction

[`COST_MODEL_REVIEW_20261115.md`](COST_MODEL_REVIEW_20261115.md) traces the
current accounting boundary. `bitop/v1` applies the same fixed affine
`group_add` weight to every recorded binary addition. The scalar and
batched builders store the same ordered key-to-first-witness table and
charge the same abstract additions. Their native field work differs:
batch inversion shares an inversion, and lazy sums omit ordinates on
misses and during table construction. The current rootfinding adapter
does not serialize those field operations as distinct priced primitives.
For an exact matched m=2 trajectory, `bitop/v1` therefore ties; the m=3
candidate can charge extra additions after an early hit. Requiring a
strictly lower batch `bitop/v1` total would test the abstract-addition
model rather than the implementation change. This is a source-level
accounting deduction, not an observed performance result.

## Revised primary decision

The primary comparison is complete cold process wall time for
`batch-m3 / scalar-m3` on the exact matched targets and rounds. Admit it
only when all 48 measured pairs finish with correct `[d]G=Q` answers,
all required phases, the same release binary and host, L2 isolation for
both arms, and no timeout or censored member. Keep the original A/A
control (`scalar-m3-aa / scalar-m3`) on those same targets and rounds.
The A/A 95% target-cluster interval must contain 1. For each target,
the geometric mean of its six candidate/scalar ratios must be below 1.
The paired 95% target-cluster interval for the candidate ratio must have
an upper endpoint below both 1 and the lower endpoint of the A/A
interval. For each arm pair, take the geometric mean of its six ratios
within each target, then the geometric mean of the eight target values.
Construct each 95% interval from 20,000 percentile bootstrap draws of
eight target indices with replacement, keeping each target's six rounds
together, with analysis seed `202611151503`. Use the same draws for the
candidate and A/A intervals. This rule
tests a whole-solve advantage outside the observed A/A noise interval.
If any admission or decision condition fails, retain all rows and report
the failed threshold or opposite measured direction at this fixed panel;
do not replace targets or repeats.

Report relation-phase and table-build wall times as separately labelled
components. Report the complete cold `bitop/v1` totals and per-phase
primitives for every arm as model diagnostics, with the original strict
all-target count rule explicitly marked non-discriminating for batching.
The `batch-m2 / scalar-m2` comparison remains a table-build diagnostic:
report complete and setup wall times and exact relation controls, but do
not promote an m=2 wall advantage from this spec because it has no m=2
A/A arm. The strong rho arm remains a reference. This study alone still
cannot rank the batched candidate against F4/F5/F6; that requires an
exact same-target panel and its own preregistered decision rule.

## Outcome policy

A passing wall rule would support only a fixed K13 scalar-versus-batch
wall comparison after independent second-host replay and the repository's
progress-fidelity record. A failing rule preserves the exact measured
direction and uncertainty; it does not refute a field-arithmetic saving
or a larger-curve hypothesis. Infrastructure or budget refusal produces
no performance observation. The unchanged original F4/F5/F6 study keeps
its own source, spec and decision rule.
