# T20 lacks the protocol-matched full-cost reference required for publication

- **Status:** open
- **Kind:** missing measurements; protocol deviation
- **Affected work:** [T20 protocol](../research/t20_n73_k600_e2e_20261019/protocol.json), PR #2318, its benchmark and leaderboard publication
- **Next action owner:** unassigned, T20 producer/validator lane
- **Last updated:** 2026-10-22

## Problem

The retained n=73, K=600 comparison does not supply complete charged IC
operation counts or the preregistered interleaved rho timing reference. It
cannot satisfy the [publication validation gate](../docs/VALIDATION_GATE.md)
by relabelling its probe proxy as total cost.

The instance is the frozen Koblitz curve a=0, b=1 over GF(2^73), polynomial
x^73+x^4+x^3+x^2+1, subgroup order 86020738150056119. The protocol binds the
base, target and validation-only sidecar by SHA-256 and names a signed
Frobenius rho reference. These identities must remain fixed in any comparison.

## Evidence and uncertainty

- **Observed:** the [retained-record review](../docs/ic/T20_REVIEW_20261020.md)
  documents duplicate rho receipts for draws 1–12 and that all retained rho
  timings follow IC. Two N-arm measurements bracket one O-arm measurement.
- **Rechecked:** [the 2026-10-22 receipt](../docs/ic/T20_RECHECK_20261022.json)
  records independent scalar replay of all three IC rows, frozen-input hash
  checks and exact native recomputation of the original analysis JSON.
- **Observed:** the raw records expose probes/attempts and relations but not
  the complete IC group-addition and operation-kind counts needed for the
  0, 0.1 and 1 GAE pricing band. The reported interval is conditional on the
  retained runs and rank-column costs.
- **Missing capability:** a protocol-matched reference and complete cost
  accounting for the comparison being admitted. Neither a successful replay
  nor the reproducible wall ratio supplies those missing measurements.
- **Not established:** a bound on run-to-run noise from the two N runs,
  complete-cost equality, or a full-cost improvement.

## What would unblock it

- [ ] Recover original, identity-bound records that establish the named
      reference and complete accounting; otherwise preregister an amended
      comparison before collecting replacement measurements.
- [ ] Record the strongest matched rho reference and IC costs in the same
      unit and binary/process, with the complete required GAE band.
- [ ] Independently replay and recompute the admitted comparison, then bind
      the replay, validator report and reference in `VALIDATION.json`.

These conditions would permit the validation gate to assess publication.
They do not predetermine an improvement or replace current-head CI and the
parent PR #2156 review. Historical reports and measurements remain intact.

## Next discriminating step

Inventory the original producer's raw counter and reference artifacts against
the frozen input hashes. If complete matching records exist, independently
recompute them. If they do not, retain the current result as a qualified
historical comparison and design an amended controlled run. Do not infer
missing counters from elapsed time or reuse the existing data as a prospective
preregistration. Any new execution follows the repository's experiment rules.

## Resolution and history

Unresolved. The 2026-10-22 recheck verifies correctness and reproducibility of
the retained analysis, while leaving the publication requirement open.
