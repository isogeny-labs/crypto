# Input-selected oriented evaluation in the CM Weber field

- **Status:** `open`
- **Kind:** implementation gap; formal bridge gap; bounded recognition failures
- **Affected work:** [round06](../research/prime_field_transfer_goal_20261116/round-06/RESULT.md), PR4399
- **Next action owner:** prime-field-transfer lane
- **Last updated:** 2026-11-16

## Problem

The current coordinate-input route returns a sixfold Weber class and has bounded
recognition failures; an oriented target point before target DLP is not yet
implemented, and the full CM/specialization interface is not formally proved.
[pcc:unverified]

The concrete scope is the generated j=0 CM fixtures F79/order19, F1531/order61
and F3181/order127, with anomalous targets over the subgroup-order prime.
The original acceptance criterion also requires arbitrary-curve applicability,
no source scalar or hidden DLP preprocessing, and complete useful costs.
Those obligations remain distinct from this fixed-family implementation gap.

## Evidence and uncertainty

- **Observed:** round06 public PCC-20261116-e0d875, unsigned:4/4 order61
  inputs accepted at64/128 digits and3/4 order127 at32/64. All four order127
  processes exit124 during128 work with no completed128 row. Precisions and
  step cap were declared before calls; old unsuccessful bins stay unchanged.
  [pcc:unverified]
- **Formal component:** twelve SplitPrime199V1 endpoints concern finite
  coefficient reconstruction only. Gate PCC-20261116-cff31a is unsigned;
  it reports77 audited theorems and0 failing/trusted-native exceptions.
  No full native/CM transfer theorem follows. [pcc:unverified]
- **Suspected explanation:** recognition height/geometry may explain uncovered
  inputs; the existing all-screen-rejected traces exclude the denominator-unit
  filter as the explanation for the old missing order61 bins, not universally.
- **Missing capability:** source-selected oriented x/y evaluation, a faithful
  CM kernel/specialization interface and a priori representative-height/precision
  bounds. Dense field degree21/lattice44 are actual representation sizes, not
  a compressed-evaluation lower bound.
- **Limits:** local finite checks are not isolated performance or independent
  execution evidence; a timeout does not prove nonexistence or a lower bound.

## What would unblock it

- [ ] A versioned map taking encoded Q and public model parameters, returning
  S=[k]R before any target solver or source scalar-candidate search; O is handled.
- [ ] Exact field/source certificates and negation/addition/chimera controls,
  including genuine unrelated global-root rejection, on fixed auditor and public inputs.
- [ ] A faithful formal CM/kernel/specialization interface, and documented
  setup/recognition/recovery/failure costs and coverage assumptions.

Meeting these criteria would settle the oriented evaluator for its stated
family and tested coverage. Arbitrary-curve applicability, completeness and
efficient full costs would still need their own evidence.

## Next discriminating step

- **Action:** review and implement the saved
  [coordinate-ratio lead](../research/prime_field_transfer_goal_20261116/round-06/ORIENTED-RATIO-LEAD.md)
  in a fresh prospective version. Review the separate unproved uniform-bound
  draft before using it to choose a precision guarantee.
- **Inputs/controls:** fixed all18-point order19 auditor and the existing
  four public inputs at each larger fixture; no scalar, table or orbit label
  selects candidates. Charge both ratio recognizers and all failed candidates.
- **Limits:** declare precision, cap and point-process limits before calls;
  preserve existing frozen sources and every timeout. This record authorizes
  no changed historical run or spending campaign.
- **Outcomes:** oriented accepted points justify relation tests and a fresh
  proof interface; class-only or partial ratio matches remain partial; bounded
  rejection changes the candidate diagnosis rather than proving impossibility.
- **Recheck trigger:** a completed versioned evaluator or new generic CM laws.

## Resolution and history

Unresolved. This record preserves the concrete next obligation; merging PR4399
does not resolve it or end the original goal. KB/Conductor remain unavailable;
source review and prospective execution requirements remain binding.
