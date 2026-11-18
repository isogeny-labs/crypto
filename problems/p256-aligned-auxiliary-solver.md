# P-256 aligned auxiliary relation solver and measurement admission

- **Status:** open
- **Kind:** implementation/qualification gap, access/tool limit, proof gap
- **Affected work:** [aligned auxiliary study](../research/p256_aux_smooth_20261114/REPORT.md),
  [prospective protocol](../research/p256_aux_smooth_20261114/PROTOCOL.md)
- **Next action owner:** continuing P-256 relation-stage task
- **Last updated:** 2026-11-16

## Problem

The requested P-256 m=2..7 matched 2x relation-solving comparison cannot yet
be established: native construction and small-domain CAS qualification
are implemented, while the intended depth8 complete solver measurement
has not passed EKS/L2 admission. Structured regularity remains unmeasured.

Target is registered P-256, new auxiliary smooth-subgroup base, intended
timing depth8. Development depth4 FB1h2704fe49495e is not a replacement
for that experiment. Novelty and lower structured degree of regularity
remain separate proof obligations, not consequences of cubic inputs.

## Evidence and uncertainty

- **Observed:** checkpoint gate exits1 with no upload transport; [raw log](../research/p256_aux_smooth_20261114/evidence/checkpoint-gate.log).
  No EKS job was launched. The environment has no Magma executable; root
  Cargo validation exits127 because cargo is absent. The current GitHub
  `isolab-experiment.yml` accepts an ecbench.spec/v1 and its build bundle
  packages ecbench/fresh-owner, not the new native Magma adapter.
- **Observed construction scope:** native assertions and raw manifests are
  bound by PCC-20261114-4a3ab6, unsigned/unverified [pcc:unverified].
- **Derived or proved:** no regularity reduction or 2x cost result established.
  Paired/oriented encodings are prospective alternatives with extra variables.
- **Suspected explanation:** unknown whether auxiliary-sign pruning pays for
  the added ordinate-chain variables and constraints.
- **Missing capability:** qualified big-prime CAS execution, source/image
  binding and its measured stage adapter, successful checkpoint upload and
  live capacity/L2 admission. Native export/replay alone supplies none of these.
- **Not established:** this gap is not a mathematical impossibility, a
  safety restriction, or evidence that every solver would fail. An unsigned
  CAS completion marker is not independent proof of complete enumeration.

## What would unblock it

- [ ] Qualify exported dense/tower/paired/oriented Magma programs on actual
  P-256, first matching complete d1 m2..7 and d2 m2..4 references including
  exceptional charts, sign multiplicity and native exact replay.
- [ ] Admit checkpoint and an exact-source CAS image through the maintained
  EKS route, with capacity/L2 controls and all frozen m2..7 d8 inputs.
- [ ] Run the prospective A/A/warmup/five-round matched panel, report all
  censored cells and setup/conversion/replay costs. A 2x stage gain needs
  complete matching Fp solution sets and the threshold in the protocol.
- [ ] For a regularity claim, bind the homogeneous/structured definition,
  order and complete Macaulay/rank certificate; affine output degree or
  cubic inputs do not satisfy this condition.

These conditions establish the corresponding scoped result, not end-to-end
index-calculus cost, natural P-256 yield or novelty.

## Next discriminating step

Qualify the big-prime CAS adapter against the tiny complete P-256 references,
then admit the original d8 matrix under 60-second/4-GiB single-cell caps,
without retries. Dense is the same-base control; tower and paired isolate
orientation overhead. If orientation wins, retain it for the full matched
panel; if it loses or censors, preserve that outcome and test the cheaper
tower presentation. Infrastructure failure remains an admission failure.

Shared admission context: [arity launch gates](arity-completion-launch-gates.md).
Recheck on available checkpoint transport, CAS/image qualification or a
new matched run. Do not repeat the failed gate without changed conditions.

## Resolution and history

2026-11-14: opened with construction evidence and executable prospective
CAS export/replay. Unresolved; merging this record does not complete the
original performance or degree-of-regularity request.

2026-11-16: [continuation](../research/p256_aux_smooth_20261114/continuation_20261116/REPORT.md)
adds actual public Magma m2 depth1/2 qualification for all raw/reduced
presentations, complete-count controls and exact point replay. Native
affine elimination has72 reconstruction cases. PCC-20261116-2335a8 is
unsigned [pcc:unverified]. This supersedes the earlier "Magma not run"
status only for those small units. Exact-source EKS/L2 admission, intended
depth8 m2..7 measurements,2x stage gain and structured regularity remain
open. No other study's record was closed and no gate was bypassed.
