# Projective-law follow-up protocol

Frozen before the first collector target on 2026-10-22 UTC. This arm uses the
21 normalized Scholten genus-2 models and the 1,260 public known-answer targets
in `research/five_leads_e2e_20261019/sessions_projective_chart_v2.tar.zst`.
Each target is one cold solve with the full factor base, 16 jumps, the same
algorithm seed and candidate cap as the archived control. Source negation-rho
runs on the same input in the same executable and process. Both recovered
scalars are checked independently by multiplication on the source elliptic
curve. Failed or exhausted targets remain as explicit rows.

The candidate replaces the wrapped external genus-2 law in the frozen
factor-base/graph collector with weighted formulas from Ozbay Gurler and
Hisil, checked against independent Cantor arithmetic. The collector stores
affine Mumford divisors between walk steps, so it normalizes each projective
output. The charged operation counts include these inversions. This run
therefore measures the formula integration as implemented; it does not claim
an inversion-free *collector*.

After that wrapper run, the second arm retains weighted divisors in jump and
walk state. It tests the projective discriminant for splitting before an
affine boundary conversion; only split candidates are normalized for roots,
relation signs, and canonical duplicate checks. The walk hash and restart set
use the raw deterministic projective representation, so this arm follows a
different deterministic walk even with the same source seed. The 180-target
16-bit diagnostic completed before freezing the full state-arm cohort; its
scratch output was overwritten by the full 16-bit run and is not used as
evidence. The full state arm is 1,260 cold solves with the same
source rho process, workload, cap, full base, and 16 jumps. Every boundary
conversion is charged and counted. This is an inversion-free *law* with
charged split-boundary inversions, not an inversion-free end-to-end collector.

Before full execution, the arithmetic gate checks all supported branches on
the 21 models, including non-unit projective `Z` and `W`, same-support,
inverse, degree-drop, and exceptional doubling cases. The gate records
uncaught branches explicitly. A separate regression records a non-unit-W
counterexample to the author's Magma `N6` expression as transcribed and the
homogeneous variant used here. No claim about the 2024 paper's omitted common
doubling equation follows from that code comparison.

Every candidate record retains the frozen workload id/hash, target index,
seed, cap, recovered scalar, verification, phase counts, native logical counts,
and a same-process rho record. The representative price is `M+0.5S+10I` and
0.1 source-addition equivalents per native logical operation. The 36-cell
grid uses the same 12 arithmetic and 3 logical prices as the frozen cohort.
Cost is cold per target, including construction, transfer, initialization,
relation work, elimination, and source verification. Wall time is diagnostic
until isolation and fastest-external-tool checks are met. The standardized
summary will include raw JSONL, a machine summary, a price-grid CSV, an SVG
graph, and an inspected PDF report; missing targets and unsupported branches
remain explicit.
