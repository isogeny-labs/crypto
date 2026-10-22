# Quadratic-twist geometric formalization and shared coordination

- **Status:** open; native parity and identity work completed
- **Kind:** remaining formal implementation gap; coordination tool/access limit
- **Affected work:** [quadratic-twist study](../research/quadratic_twist_evidence_20261021/README.md)
- **Next action owner:** authoring session for PR #2381; shared coordination unassigned
- **Last updated:** 2026-10-22

## Problem

The full historical finite workload now has a native replay, but the full
geometric theorems still lack checked bridges from formal elliptic-curve point
groups and twist/pairing constructions to all the proved algebraic lemmas.
Actual Frobenius point maps and rational lifts are now checked.
The explicit injective short-model twist image and its Frobenius-negation
property are now checked too, without the earlier hB assumption for that image.
The actual shared Conductor task state is also unavailable in this environment.
Neither gap is a mathematical nonexistence result.

## Evidence and uncertainty

- **Verified finite observations:** [native parity receipt](../research/quadratic_twist_evidence_20261021/native/runs/run-20261021T150125Z/parity.json)
  matches all 434 historical deterministic fields. It covers 942 models,
  4,188,162 S3 triples, 113,342 S4 quadruples, the four trace-zero fixtures,
  the subfield fixture, 81 complete toy IC recoveries and four relation collections.
- **Independent native replay:** all 84 IC certificate matrices (three headline
  runs and 81 complete-log runs), their factor-base logs, pairings, scalars, and
  ranks pass the separate auditor. False relations, missing coverage, and a false
  historical census count are rejected. The tower/simple-basis conversion checks
  2,401 elements, 2,475 points and 2,025 group-law identities.
- **Checked implications:** [28 named Lean theorems and proof map](../research/quadratic_twist_evidence_20261021/lean/README.md)
  pass the repository proof gate on standard foundations, with no extra trusted
  native axioms. This proves their explicit group/ring statements, not all the
  historical geometric identifications. Four declarations use the actual
  elliptic point group, q-power Frobenius and rational base-change map.
- **New explicit bridge:** ten further declarations prove u^q=-u from the
  nonsquare hypothesis, construct the actual injective twist-point image,
  prove Frobenius negation, and derive odd-order single-image/finite-sum
  obstructions without assuming hB. The odd annihilator is on the actual
  image or sum; an additive isomorphism and full eigenspace surjectivity are
  still separate unproved bridges. The audit checks 36 declarations, including
  generated helpers, with zero extra trusted axioms.
- **Continued falsification:** [new run and inspected report](../research/quadratic_twist_evidence_20261021/reports/mixed-20261022T0718Z/README.md)
  covers 74,693,760 ordered signed words on 274 odd-prime subgroup pairs,
  with independent Rust point arithmetic and DP replay. The 1,718,592 mixed
  zero words give no rank gain over pure rows. All 260,064 rational/twist
  pairs satisfy the obstruction, with 14,688 nonzero two-torsion exceptions.
- **Identity records:** three original IDC1 records plus the new source-bound
  cleared-monic-map certificate, linked to the actual Lean sources;
  1,921 registered ICV1 models and 27 exact selected EC1 representations. Models
  without an experimental cyclic generator retain that unknown.
- **Tool/access:** Rust and Lean were installed for this continuation; the former
  claim that they were unavailable no longer describes the current environment.
  The library suite and native backend tests are recorded in the study validation
  directory. Conductor still exits 127; its shared workflow/configuration is absent.
  No shared reservation is claimed, and no replacement local coordinator was started.
- **Not established:** general ECDLP hardness, an asymptotic IC advantage, or a
  general GHS algorithm. The finite observations and proved conditional
  implications do not establish those conclusions.

## What would unblock it

- [x] Port and replay every historical finite workload natively; preserve the
  original source, results and controls; compare all mathematical output fields.
- [x] Add independently checked new relation certificates and meaningful controls.
- [x] Issue identity certificates and exact model/subgroup records without
  inventing a generator for a whole-group census.
- [x] Finish feasible central Lean group/ring lemmas and audit their axioms.
- [x] Instantiate the central mixed-relation implication on the actual elliptic
  point group and Frobenius; derive fixedness of rational base-change points.
- [x] Prove the explicit short-model twist image's Frobenius negation, with no
  assumed hB; derive the actual-point odd-order obstruction and finite-sum form.
- [ ] Prove the additive isomorphism, general-model normalization and full rational-eigenspace identifications,
  summation-polynomial lift statements, and nondegenerate pairing DLP reduction.
- [ ] Complete the universal trace-zero/counting bridge under exact field,
  degree, separability, and rationality hypotheses.
- [ ] Run the actual shared Conductor check with its configured task state.
  Do not substitute a new local coordination instance.

Satisfying these conditions would establish the stated formal parity. It would
still not prove general ECDLP hardness or an asymptotic solver advantage.

## Next discriminating step

- **Action:** prove the explicit short-model image preserves addition and
  characterize its full minus eigenspace. Then attempt exceptional-case S3/S4
  lifting, so a purported polynomial relation is tied to actual lifted points.
- **Inputs/control:** keep characteristic, model, nonsquare and extension
  hypotheses; retain nonzero two-torsion and square-parameter countercontrols.
- **Acceptance:** kernel-checked group-map and eigenspace/lifting theorems,
  without assumed conclusions, with axiom audits. Any new bounded relation
  search must preregister its field/factor-base family and include pure-component
  and actual-point controls rather than infer impossibility from absent gains.
- **Outcomes:** a passing theorem closes only its stated bridge. A new valid
  mixed relation is retained and projected into both eigenspaces before claiming
  it adds information; missing definitions and failed builds remain explicit.
- **Recheck trigger:** the next formal bridge or real shared coordination access.

## Resolution and history

The initial import supplied historical evidence and a certificate-only auditor.
The 2026-10-21 continuation closes the full native replay and identity-record gaps,
installs the missing proof/test toolchains, and proves 14 algebraic implications
and four theorems using actual Frobenius maps on elliptic-curve points.
This record remains open for the specific bridges and shared access listed above.
Frozen historical evidence and earlier failed attempts remain unchanged.
The 2026-10-22 continuation closes hB for explicit short-model images and
records the new finite mixed-word null result, its exceptions and controls.
It does not close the remaining geometric, solver-cost or shared-access gaps.
