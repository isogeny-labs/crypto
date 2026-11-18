# P-256 mixed eigenline: cycle-safe solver and net cost

- **Status:** open
- **Kind:** implementation gap and missing measurement
- **Affected work:** [transfer search](../research/p256_genus_dlp_20261115/transfer_search/REPORT.md) and [folded solver](../research/p256_genus_dlp_20261115/folded_walk/REPORT.md), PCC-20261115-e111c2 and PCC-20261115-30dcbf
- **Next action owner:** unassigned; no background run is active
- **Last updated:** 2026-11-15

## Problem

The mixed P-256 subgroup in J(C3) has a checked order-three action, but no
cycle-safe quotient solver and complete-cost comparison establish a runtime
advantage over the strongest direct P-256 solver. The user's speedup objective
remains open. This is a specific constant-factor case related to
[chained prime-field transfers](prime-field-chained-transfer-no-advantage.md).
[pcc:unverified]

## Evidence and uncertainty

- Observed producer checks: nonzero order-n mixed image, eigenvalue relation,
  six distinct signed images, addition and public scalar checks over the actual
  P-256 field. Source and exact divisor are in audit.json. [pcc:unverified]
- Derived: a uniform quotient-walk heuristic saves sqrt(3) in steps relative
  to negation-only rho. It does not change the square-root exponent. [pcc:unverified]
- Cost control: the same eigenline in E x E needs two component additions;
  its ideal free-fold model costs 2/sqrt(3) source-addition units. This is
  not a wall-time lower bound for optimized implementations. [pcc:unverified]
- Implemented continuation: independent jumps, exact coefficient tracking and
  bounded short-cycle handling; complete public toy solves and an actual-field
  transition trace. Four producer tests pass; signed standing is absent.
  [pcc:unverified]
- Missing: optimized destination arithmetic; matched transport/setup/folding/solve/
  recovery/verification costs; strongest available source control and replay.
- New component: [minimal transport](../research/p256_genus_dlp_20261115/minimal_transport/REPORT.md)
  compiles the fixed-order integer norm floor and coefficient bound. Its
  norm-minimal map implementation is source-only/unqualified; full geometry
  and runtime advantage are not inferred. [pcc:unverified]
- No timing, speedup or global nonexistence claim follows from these checks.

## What would unblock it

- [x] Implement the bounded quotient walk, with identity/degenerate-collision and
  short-cycle tests; recover and replay every successful public toy scalar.
  Producer evidence only; long cycles can reach the explicit budget.
  [pcc:unverified]
- [ ] Measure addition plus folding/hash/cycle cost against the matched source
  step on the same subgroup/workload. The preliminary threshold is sqrt(3)
  times source cost, with transport/setup/recovery additionally charged.
- [ ] If that threshold survives, run the full three-arm transfer protocol
  under the repository isolation and independent-replay rules. A net gain
  needs source-total / destination-total > 1 within stated uncertainty.

These conditions would establish only the measured/modelled regime. A toy
solve or stage saving does not establish an actual P-256 scalar-recovery gain.

## Next discriminating step

Prepare complete step pricing for the implemented bounded walk and
compare it to the strongest applicable source step. Use exact P-256
arithmetic for the component calibration and fully specified small prime-order
fixtures for complete solves, with source negation, destination generic and
folded controls. Freeze the workload and finite budget before any benchmark;
use RunPod as explicitly selected by the user. The documented proxy API route
returned HTTP401; authenticated RunPod access is pending. A source-frozen replay
payload exists; no pod or timed comparison is running. The new candidate also
needs the mandatory S3 resume-checkpoint upload route, currently missing.
[pcc:unverified]

If costs exceed the threshold, close that implementation and examine another
representation; if they beat it, execute a full matched solve protocol; if
solver or infrastructure checks fail, retain the failure and fix that exact
dependency. Recheck when an optimized adapter or solver implementation lands.
[pcc:unverified]

## Resolution and history

2026-11-15: initial exact image checks and bounded exclusions recorded. Open.
No benchmark is running, and a PR merge does not resolve this cost problem.

2026-11-15 continuation: bounded folded solver and producer controls implemented,
PCC-20261115-30dcbf. RunPod replay package prepared; authenticated API access and
the subsequent admitted complete-cost comparison remain open. [pcc:unverified]

2026-11-15 minimal-transport continuation: integer-basis norm floor and
conditional128-bit coefficient bound are kernel-checked, PCC-20261115-9ff011
unsigned. Shared generic laws merged in crypto-lean PR12. The separate native
candidate compiles but has no executed qualification cases; checkpoint and
RunPod access remain required. This does not resolve the complete-cost problem.
[pcc:unverified]
