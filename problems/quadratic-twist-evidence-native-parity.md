# Quadratic-twist geometric formalization and shared coordination

- **Status:** open; native parity and identity work completed
- **Kind:** remaining formal implementation gap; coordination tool/access limit
- **Affected work:** [quadratic-twist study](../research/quadratic_twist_evidence_20261021/README.md)
- **Next action owner:** authoring session for PR #2381; shared coordination unassigned
- **Last updated:** 2026-10-24

## Problem

The full historical finite workload now has a native replay, but the full
geometric theorems still lack checked bridges from formal elliptic-curve point
groups and twist/pairing constructions to all the proved algebraic lemmas.
Actual Frobenius point maps and rational lifts are now checked.
The explicit injective short-model twist image and its Frobenius-negation
property are now checked too, without the earlier hB assumption for that image.
Addition and scalar compatibility, an actual isomorphism onto the entire
short-model minus eigenspace, and the rational fixed-point identification are
now checked. Source annihilators are transported by the proved group map.
The explicit F43 Miller-function reduction is now checked on all 121 generator
pairs, with unique exponent transport. General geometric pairing theory and a
uniform reduction remain open; the finite IC algorithm is not formalized.
The actual shared Conductor task state is also unavailable in this environment.
Neither gap is a mathematical nonexistence result.

The new actual S3 comparison checks a finite F43 useful-row/closure improvement
and an injective scalar-preserving base-field map for that example. Its leading
homogeneous ideal is identical at fixed size2/3; a strict regularity improvement
is refuted for this encoding. A different leading ideal/solver bridge, uniform
useful yield and compatible complete work bounds remain unresolved. The gain is
reproduced by an exact pulled-back factor base on the main curve.

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
- **Checked implications:** [106 named Lean theorems and proof map](../research/quadratic_twist_evidence_20261021/lean/README.md)
  pass the repository proof gate on standard foundations, with no extra trusted
  native axioms. This proves their explicit group/ring statements, not all the
  historical geometric identifications. Four declarations use the actual
  elliptic point group, q-power Frobenius and rational base-change map.
- **Explicit image bridge (2026-10-22):** ten declarations prove u^q=-u from the
  nonsquare hypothesis, construct the actual injective twist-point image,
  prove Frobenius negation, and derive odd-order single-image/finite-sum
  obstructions without assuming hB. Those original statements put the odd
  annihilator on the image/sum; their source and certificate remain frozen.
- **Group and eigenspace bridge (2026-10-23):** [new proof and report](../research/quadratic_twist_evidence_20261021/reports/group-map-20261023/README.md)
  add 21 named theorems. The actual point map preserves addition, natural and
  integer scalars and finite sums. Its injection gives equivalence of source
  and image annihilators. Lean proves the q-power fixed field is the image of
  k in any field algebra K, descends the coordinates of every minus point,
  and constructs the actual group isomorphism onto all of the short-model
  minus eigenspace. Rational base-change points give exactly the plus space.
  No fixed-field or surjectivity conclusion is assumed. The unmodified gate
  audits 57 declarations, zero failures/extra trusted axioms. General-model
  normalization and the further geometry below remain open.
- **Continued falsification:** [new run and inspected report](../research/quadratic_twist_evidence_20261021/reports/mixed-20261022T0718Z/README.md)
  covers 74,693,760 ordered signed words on 274 odd-prime subgroup pairs,
  with independent Rust point arithmetic and DP replay. The 1,718,592 mixed
  zero words give no rank gain over pure rows. All 260,064 rational/twist
  pairs satisfy the obstruction, with 14,688 nonzero two-torsion exceptions.
- **Positive pairing angle (2026-10-24):** [new run and inspected report](../research/quadratic_twist_evidence_20261021/reports/pairing-20261024T0442Z/README.md)
  kernel-checks actual F43 twist/source/Frobenius bindings and all 121 explicit
  Miller generator pairs. The finite transport does not use a log lookup.
  Independent native arithmetic checks 4,616 auxiliary points across 274
  shared-prime cases, all 649 nonshared-prime cases, and eleven native IC logs.
  Degenerate and undefined evaluations remain distinct; wrong-subgroup and
  corrupted-payload controls reject. Nineteen new named statements bring the
  unmodified gate to 85 audited declarations, zero failing/extra trusted.
- **Identity records:** three original IDC1 records, the source-bound
  cleared-monic-map certificate, and four new group-coordinate/discriminant
  records, the norm-product IDC1h23c0696a12358b08 and cleared-budget
  IDC1h1fac46ceb41801ac, linked to the actual Lean sources;
  1,921 registered ICV1 models and 27 exact selected EC1 representations. Models
  without an experimental cyclic generator retain that unknown.
- **Yield/rank/cost continuation:** [new proof and report](../research/quadratic_twist_evidence_20261021/reports/yield-20261024/README.md)
  add38 named statements. Actual matched twist bases have identical coefficient
  relation sets and row rank. Uniform whole-group raw yields are1/N, with exact
  twist/main ratio N/Nt. Conditional useful-yield and solver-degree cost models
  have checked strict-improvement and overhead thresholds. A kernel-checked F5
  control has higher raw twist zero count but no nonzero rows modulo3. The
  unmodified gate audits127 declarations, zero failing/extra trusted. No actual
  useful-yield advantage, lower regularity certificate, solver work bound or
  main-to-twist DLP reduction is silently supplied. See the
  [lead-atlas handoff](../research/quadratic_twist_evidence_20261021/validation/yield-20261024/atlas-handoff.json)
  for existing related leads; their dispositions are not changed by this proof.
- **Actual S3 continuation:** [report and proof map](../research/quadratic_twist_evidence_20261021/reports/s3-yield-20261024/README.md)
  independently checks124 factor-base comparisons /1,904 systems. The first F43
  prefix target gives main/twist rank0/2 and finite closure degree6/4. Lean checks
  the concrete main-DLP scalar equivalence and actual useful twist relations,
  the main's degree4 non-refutation /degree5 unit certificate, and exact accepted
  relation-set equality after pulling the twist base back to main. The actual
  leading ideal <x^2*y^2,x^b,y^b> has first-empty degree b+1 in both arms for
  b2/3. All-target mean closure is5.2 in both F43 arms, and the twist loses on
  targets4--7. This is a basis/target effect with no measured work advantage.
  The unmodified gate now passes144 named theorems /185 audited declarations,
  zero extra trusted. Both new polynomial certificates and rehashed false-rank,
  false-degree, omitted-case and false-scaling controls pass. Previous evidence
  and all ten prior study certificates remain unchanged. The newly used twist
  generator(24,26) is explicit and has no registered EC1 record; the existing
  generator(35,12) alias is never reused for it.
- **Tool/access:** Rust and Lean were installed for this continuation; the former
  claim that they were unavailable no longer describes the current environment.
  The library suite and native backend tests are recorded in the study validation
  directory. Conductor still exits 127; its shared workflow/configuration is absent.
  No shared reservation is claimed, and no replacement local coordinator was started.
- **Artifact publication hold:** the new inspected PDF, two PNG copies and raw
  archive require current-main S3 storage. Automatic approval review rejected
  uploading the specific raw payload/destination without explicit user
  authorization, citing possible non-public research data. No upload was
  performed or retried. The [exact four-file pending manifest](../research/quadratic_twist_evidence_20261021/validation/s3-yield-20261024/ARTIFACTS_UPLOAD_PENDING.json)
  records684,341 local bytes with hashes/intended keys; compact data/proofs and
  editable report source remain reviewable in the PR. This is an authorization
  gate, not a mathematical or test failure. Keep publication incomplete until
  approval, actual upload and remote verification succeed.
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
- [x] Prove actual addition/scalar compatibility and source-annihilator transport.
- [x] Construct the isomorphism onto the full minus eigenspace for the explicit
  short models; identify the rational fixed eigenspace on actual Weierstrass points.
- [x] Check the explicit F43 Miller-function table, twist/Frobenius bindings and
  exact finite logarithm transport; independently verify all eleven native IC logs.
- [x] Prove exact matched relation-set/rank transport, whole-group yield counts
  and explicit conditional useful-yield/degree/overhead comparisons.
- [x] Check a finite actual small-base useful-row/closure improvement and an
  applicable main-DLP reduction: F43 prefix fixture; verify its factor-base
  pullback control and all-target losses, without claiming uniform advantage.
- [ ] Establish a useful-yield improvement beyond a changed base/finite rank
  state, a genuinely lower regularity/solver-degree bound in an applicable
  encoding, and compatible complete work bounds before an actual speed claim.
  For the tested S3 encoding, equal leading ideals rule out a strict homogeneous
  regularity gain. A new encoding must name the changed leading ideal rather
  than silently substitute finite closure degree for regularity.
- [ ] Prove general-model normalization with compatible group maps,
  exceptional summation-polynomial lift statements, and the general geometric
  pairing correctness/nondegeneracy and uniform DLP reduction.
- [ ] Complete the universal trace-zero/counting bridge under exact field,
  degree, separability, and rationality hypotheses.
- [ ] Run the actual shared Conductor check with its configured task state.
  Do not substitute a new local coordination instance.
- [ ] Obtain explicit approval for the four-file pending artifact manifest,
  upload those immutable bytes to the named S3 store and verify remote hashes.

Satisfying these conditions would establish the stated formal parity. It would
still not prove general ECDLP hardness or an asymptotic solver advantage.

## Next discriminating step

- **Actual-system construction obligation:** specify a descent/rationality
  encoding whose actual leading ideal can differ between the two arms, an
  admissible factor-base distribution and a scalar-preserving subgroup map.
  Acceptance needs independent useful-rank progression, a certified relevant
  degree bound with the solver bridge stated, and a pullback/base-choice control.
  A mapped equal ideal is a scoped obstruction; a finite first-target gain is
  not a stationary yield law. Full work accounting is still required. No such
  new experiment is currently running; this does not assert all encodings fail.
- **Action:** extend the positive pairing angle beyond the checked k2 fixture:
  preregister denominator-safe subgroup families and price the destination
  solver before an advantage claim. General geometric pairing correctness is
  a separate formal obligation. Exceptional-case S3/S4 lifting and compatible
  general-model normalization remain additional obligations, not running jobs.
- **Inputs/control:** keep characteristic, model, nonsquare and extension
  hypotheses; retain nonzero two-torsion and square-parameter countercontrols.
- **Acceptance:** retain explicit subgroup/embedding-degree/nondegeneracy
  conditions and independent actual-point/field-log checks. General theorems
  need kernel proofs and axiom audits. Any new bounded relation
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
The 2026-10-23 continuation closes the explicit short-model addition, scalar,
source-annihilator and full eigenspace bridges. The same PR-caused stale
Jacobian catalog is repaired natively without changing prior model objects.
Current merge conflicts, inaccessible complete checks/statuses, external CI
service failures and shared Conductor access remain operational gates, not
mathematical obstructions. No broader hardness or no-connection claim follows.
The 2026-10-24 continuation follows the user's positive pairing request and
checks the explicit finite transfer, its native IC recoveries and degeneracy
controls. It closes that bounded bridge, retaining the generic geometric,
cost, coordination, review, CI/access and merge-conflict holds.
