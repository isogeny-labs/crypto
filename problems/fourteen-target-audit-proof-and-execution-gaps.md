# Fourteen-target audit: remaining formal bridges and execution prerequisites

- **Status:** open
- **Kind:** proof gaps, implementation gaps and access/tool limits; no impossibility claim
- **Affected work:** [fourteen-target audit](../research/theorem_audit_20261108/README.md)
- **Next action owner:** audit continuation; environment owner for credential/host provisioning
- **Last updated:** 2026-11-08

## Problem

The request for all fourteen source audits, experiments and faithful Lean proofs
is incomplete: the study has proof components and source corrections, while
new experiments cannot pass the required checkpoint upload and several actual
curve/Jacobian/implementation bridges have not been formalized.
[pcc:unverified]

## Evidence and uncertainty

- Observed: a complete resume document was supplied to `scripts/checkpoint.sh
  gate`; upload failed because neither the configured AWS profile nor an
  upload token exists. Local builds and fixed development tests continued.
  The same checkpoint dependency appears in
  [the existing launch-gates record](arity-completion-launch-gates.md).
  No paid cloud resources were requested or launched. [pcc:unverified]
- Observed: the selected runtime contains one execution environment. The
  Rust production controls and additional C++ fixture execute on it; this
  does not supply the requested other-machine validation. [pcc:unverified]
- Local mathematical components: `PROOF_MAP.md` describes the actual Lean
  statements. The S3 existence equivalence is general in its stated domain;
  the Frobenius, branch, pairing and subgroup lemmas expose their missing
  bridges. An arithmetic identity is not labeled a full Jacobian, pairing or
  algorithm-refinement theorem. [pcc:unverified]
- Missing formal bridges: E[r]'s Frobenius representation/characteristic
  polynomial; supersingularity and Hasse trace-zero result; actual Weil/Tate
  constructions and domains; Diem ramification and intersection geometry;
  reduced-divisor/Jacobian definitions and smoothness counting; msolve row
  arithmetic/provenance/degree refinement; probabilistic mixing/collision
  theorems on the appropriate walk ensemble. These are not infrastructure
  errors or assumed conclusions. [pcc:unverified]

## What would unblock it

- Configure a valid checkpoint upload mechanism in environment settings, or
  receive an explicit owner change to that repository requirement. Never
  paste a bearer token into a report or chat. This enables only the new local
  experiments; it does not settle any theorem. [pcc:unverified]
- Supply an independently controlled execution host, or a reviewed CI run
  of the appropriate independently implemented validator and original inputs;
  retain its real provenance and signed report. [pcc:unverified]
- Complete each missing formal bridge against the exact source hypotheses,
  build its actual module, audit axioms and map its conclusion to the corpus
  statement before promoting a result. [pcc:unverified]

## Next discriminating steps

1. With checkpoint access, resume the original native small-field census
   obligations in items1–3, with fixtures and output scope frozen before runs.
   The current production known-degree test remains the calibration control.
   Never convert finite agreement into a universal theorem. [pcc:unverified]
2. For item7, provide the concrete Jacobian/factor-base/subgroup family and a
   faithful Lean group/divisor bridge; the abstract confinement theorem is
   already available and is insufficient by itself. [pcc:unverified]
3. For item12, bind recorded C matrix rows to monomial/prior-polynomial
   certificates and total degrees; separate ordinary GB, trace replay and
   saturation. A source comment naming F4 is not a refinement proof.
   [pcc:unverified]

## Resolution and history

Unresolved. The proof-and-control increment does not close the original
fourteen-item request, and opening or merging its PR would not do so.
[pcc:unverified]
