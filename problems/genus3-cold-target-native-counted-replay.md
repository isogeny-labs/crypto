# Historical genus-3 cold-target timings lack native counted-rho admission

- **Status:** open
- **Kind:** implementation gap; measurement/validation gap; access/tool limit
- **Affected work:** [historical study](../research/genus3_cover_cold_target_20261115/README.md), claim PCC-20261115-7c5845
- **Next action owner:** native cover-solver implementer, then independent runner
- **Last updated:** 2026-11-15

## Problem

The historical Python/BSGS wall-time audit cannot establish the repository's
primary cold one-target IC comparison because the complete native solver,
counted matched Pollard-rho arm, isolation admission and independent replay
are absent. This is an evidence/implementation gap, not a mathematical
obstruction to the restricted cubic-extension construction.

The exact fixture, subgroup, generator and original per-target rows remain
in the study. Its model has no registered ICV1/EC1 identity yet; preserve that
gap instead of guessing an alias from field size.

## Evidence and uncertainty

- **Observed:** original source and audit bytes are preserved. The native
  checker reports local scalar/summary checks; its unsigned record remains
  [pcc:unverified]. It does not authenticate the original clock readings.
- **Derived or proved:** no new complexity, fastest-implementation or
  rho-beating conclusion is established by this import.
- **Missing capability:** full native cover execution and counted phase
  counters, native reference/controls and signed replay. AWS/checkpoint/ecdb
  credentials were not configured in the import environment; no gated run was
  launched or replaced with an unauthorized local timing run.
- **Uncertainty:** the historical wall-time comparison may depend on language,
  arithmetic implementation and runtime conditions. A scalar replay alone
  cannot resolve those explanations.

## What would unblock it

- [ ] Implement and check the complete native cover path on the exact fixture,
  retaining generator normalization and the pole-divisor correction.
- [ ] Resolve exact curve/candidate/workload identity and independently certify
  the map and subgroup action; retain old names only in frozen evidence.
- [ ] Freeze counted cold/online phase contracts and a matched strong native
  rho reference before a new run; charge setup and all failed attempts to each
  single public target and price native work across the required band.
- [ ] Obtain the configured checkpoint and EKS route, a bounded preregistration,
  qualified controls/isolation, immutable raw publication and signed replay.
- [ ] Pass validation gates before adding any accepted dashboard/index row.

These are conditions for a new eligible evaluation, not a prediction that
the candidate will beat its native reference.

## Next discriminating step

Port the full execution path to the native harness and compare exact outputs
against the frozen known-answer fixtures as development checks first. Once
that contract holds, preregister the matched measurement under existing
admission/budget rules. No performance run or spending is authorized by this
problem record itself. A mismatch requires repair; a successful native replay
allows measurement admission work; a measured loss is retained as a loss.

## Resolution and history

Unresolved. This historical-import PR preserves the requested observation
while keeping the missing acceptance evidence visible. Related general
context: [cubic-extension admission](cubic-extension-transfer-admission.md).
