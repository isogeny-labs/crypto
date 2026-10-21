# Quadratic-twist geometric formalization and shared coordination

- **Status:** open; native parity and identity work completed
- **Kind:** remaining formal implementation gap; coordination tool/access limit
- **Affected work:** [quadratic-twist study](../research/quadratic_twist_evidence_20261021/README.md)
- **Next action owner:** authoring session for PR #2381; shared coordination unassigned
- **Last updated:** 2026-10-21

## Problem

The full historical finite workload now has a native replay, but the full
geometric theorems still lack checked bridges from formal elliptic-curve point
groups and twist/pairing constructions to all the proved algebraic lemmas.
Actual Frobenius point maps and rational lifts are now checked.
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
- **Checked implications:** [18 Lean theorems and proof map](../research/quadratic_twist_evidence_20261021/lean/README.md)
  pass the repository proof gate on standard foundations, with no extra trusted
  native axioms. This proves their explicit group/ring statements, not all the
  historical geometric identifications. Four declarations use the actual
  elliptic point group, q-power Frobenius and rational base-change map.
- **Identity records:** three IDC1 records linked to the actual Lean source,
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
- [ ] Prove the twist embedding's Frobenius negation property and full rational-eigenspace identifications,
  summation-polynomial lift statements, and nondegenerate pairing DLP reduction.
- [ ] Complete the universal trace-zero/counting bridge under exact field,
  degree, separability, and rationality hypotheses.
- [ ] Run the actual shared Conductor check with its configured task state.
  Do not substitute a new local coordination instance.

Satisfying these conditions would establish the stated formal parity. It would
still not prove general ECDLP hardness or an asymptotic solver advantage.

## Next discriminating step

- **Action:** construct the twist point embedding and prove that actual Frobenius
  negates its image, then discharge `hB` in `FrobeniusBridge.rational_mixed_odd`.
- **Inputs/control:** use the same readable theorem hypotheses; retain the
  nonzero 2-torsion exception as a control against dropping the oddness condition.
- **Acceptance:** a kernel-checked twist embedding and its negation theorem,
  with no assumed eigen-identification conclusion, and an axiom-audit receipt.
- **Outcomes:** a passing bridge closes that specific universal proof gap;
  missing definitions or failed builds remain explicit implementation obligations.
- **Recheck trigger:** the next formal bridge or real shared coordination access.

## Resolution and history

The initial import supplied historical evidence and a certificate-only auditor.
The 2026-10-21 continuation closes the full native replay and identity-record gaps,
installs the missing proof/test toolchains, and proves 14 algebraic implications
and four theorems using actual Frobenius maps on elliptic-curve points.
This record remains open for the specific bridges and shared access listed above.
Frozen historical evidence and earlier failed attempts remain unchanged.
