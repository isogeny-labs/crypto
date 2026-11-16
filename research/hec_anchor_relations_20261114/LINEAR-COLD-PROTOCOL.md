# Frozen complete cold screen for linear-denominator rational relations

Freeze after LINEAR-PROTOCOL.md exactness gate, before cold worker/driver or run.
Question: does the complete specialized rational solver improve on the fastest
existing same-target implementation, with all method costs charged?

## Inputs, policy and controls

Reuse exactly six existing dense curve fixtures from heldout-run, family0:
g2p1009,p16381;g3p127,p1009;g4p31,p127. These are reused curves, not a new curve
holdout. The selector is not rerun and timings do not select fixtures. Retain
fixture bytes and SHA256. Use target seed indices5,6,7, with the existing
Fixture::target law and walk seed20260916+s. These18targets were not measured
in the preceding0..4cohort. Generate each target outside every method clock;
all arms receive identical curve,D1,D2,ell,seed. Scalar is harness-only.

For rational generic and linear arms choose d2 and requestedH as follows:
g2: smallestH>=6with32*C(H,6)>=4p, using bounded integer arithmetic;
g3/g4: ceil(p^(1-1/g)), the prior anchor policy. Both rational modes use the
same requestedH and must agree on effectiveH and every deterministic count.
EffectiveH includes both descent rows and is capped by available FB places.

Sixteen independently cold timed arms per target: the prior12native/table
controls (branches1/64,Fast/Hybrid, native reference/direct oracle); compact;
rational-generic; rational-linear; and a second rational-linear-aa. Thus288
timed cells. Do not change the old ARMS15or its prior cohort/stage outputs.
Rotate this separate arm list by(fixture_index*3+seed_index)%16 and reverse
for odd seed indices. The duplicate linear arm is an A/A diagnostic and is
excluded from baseline selection.

Primary ratio per target: min(time of14existing arms:12controls+compact+generic)
/ linear total. This includes the existing compact solver as a baseline.
Secondary: generic total / linear total, on exactly matching rational counts.
Do not multiply this ratio by earlier timing or rank-count ratios.

## Gates and complete costs

Before timing, fully gate rational-linear on each of the18targets in a separate
child, including all raw and projected identities and final scalar. Retain
gating cost separately. Gate failure censors rational candidate/generic timing
on that target; native/compact controls still run. Missing cells are not zeros.
Every timed arm verifies its scalar; final method verification is included.
Extra harness verification is separately measured. Fixture parsing, fixture
validation and known-answer target construction are excluded equally.

All cold tables/catalogue, descents, target support, anchor construction,
subset/Gray construction including unused outputs, primitive normalization,
monic normalization, factorization/orientation, dedup, forest, incremental
solve and method final verification are charged in total. Individual phase
clocks are explanatory and do not replace that total. No cross-target reuse.

Fresh child per cell, sequential local ARM, L0observational/no isolation claim,
no cloud spend. Gate child cap60s, timed child cap30s, aggregate child-wall cap
1800s. Native receipt runner pins scientific build, full args, input SHA and
owned child status; changed input must fail closed. Retain timeouts/errors/
unstarted cells and separate diagnostic control failures. Avoid pooled hosts.

## Acceptance, falsifiers and raw evidence

E6correctness: all18gates and288timed cells pass answer checks; generic/linear
and linear A/A counts/target fields agree exactly. Retain losses/censoring.
E7pilot cost: every fixture's three-target median primary ratio>=10. Failure
rejects this implementation's10xpilot claim; it is not a proof about all
rational interpolation algorithms. Even a pass needs larger independent
curves, controlled replay, external comparison and mathematical novelty review
before satisfying the full objective. No exponent fit from this six-curve panel.

Report every target and fixture median/min/max, generic/linear total and phase
ratios, A/A spread, effectiveH/constructed/consumed outputs and terminal statuses.
Native Rust collection/native jq reduction, raw stdout/stderr/receipts/fixtures,
source/library/binary hashes, closed bench.record/v1and S3custody, native report
and PDF. Existing shared progress metadata must be regenerated separately;
no Python scientific execution, collection, reduction or plot generation.
Source receipts and prior art are in LINEAR-PROTOCOL.md; no novelty assertion.
