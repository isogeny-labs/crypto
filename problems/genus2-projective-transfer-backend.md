# Complete projective arithmetic for the Scholten genus-2 cohort

- **Status:** `open`
- **Kind:** proof/generalization gap; frozen cohort measurement is complete
- **Affected work:** [five-leads follow-up](../research/five_leads_e2e_20261019/STATUS.md), L5; [four follow-ups](../research/four_followups_20261021/REPORT.md)
- **Next action owner:** PR #2304 follow-up
- **Last updated:** 2026-10-25

## Problem

Two projective-law collector arms now have full, source-verified 1,260-target
operation-count comparisons. Exact coefficient certificates and written
divisor arguments now cover every reachable branch of the implemented
weighted law under the stated smooth-curve, odd-characteristic, valid-input,
and nonzero-weight assumptions. Independent proof review and faithful
formalization remain open; the frozen runs alone do not prove correctness
for arbitrary inputs.

## Evidence and uncertainty

- **Observed, 2026-10-19:** The [native model gate](../research/five_leads_e2e_20261019/correctness/genus2-model-gate-v1.jsonl) converts all 42 recorded models to monic quintics without an x^4 term, validates 4,200 transferred divisor samples and checks 4,200 generator-addition pairs commute with the coordinate map. The cited [complete group law](https://link.springer.com/article/10.1007/s13389-024-00351-7) covers that equation form in odd characteristic. That gate ran no projective additions.
- **Observed, 2026-10-22:** The [weighted-law follow-up](../research/projective_law_followup_20261022/REPORT.md)
  ran a normalized wrapper and a retained weighted-state collector on all
  1,260 original targets each. Every candidate and same-process source-rho
  scalar passed independent source multiplication. At the representative
  price, the pooled candidate/rho ratios are 3.793783 and 4.179297; the
  retained-state/wrapper ratio is 1.101617. The arithmetic gate checked
  24,192 weighted 1+2 cases, 26,880 sampled 2+2 pairs, 2,310 targeted
  exceptional 2+2 cases, 1,680 doubles, and 21,504 consecutive additions
  against independent Cantor arithmetic. The [native validator](../research/projective_law_followup_20261022/results/independent-validation.json)
  recomputed 576 arm/size/price cells and cold-phase sums.
- **Observed, 2026-10-22 proof audit:** A deterministic [42-model finite audit](../research/projective_law_followup_20261022/PROOF_AUDIT.md)
  compared 1,020,000 sampled 2+2 and 204,000 sampled 1+2 weighted
  additions with Cantor arithmetic. No mismatch or unresolved return was
  observed. The audit also records five algebraic branch-selection lemmas;
  output-formula identities remain unproved.
- **Derived or proved:** For a quintic with leading coefficient L and x^4 coefficient a4, `x=L*u-a4/(5L)` and `y=L^3*v` give a monic quintic without u^4 when L and 5 are invertible. All 42 finite fields meet these conditions in the executed gate.
- **Derived or proved, 2026-10-22 symbolic follow-up:** The
  [proof audit](../research/projective_law_followup_20261022/PROOF_AUDIT.md)
  now gives an exact integer-polynomial certificate and written divisor
  argument for the generic weighted-input 1+2 branch with `K≠0`. This
  establishes that branch's formula identity over the stated family,
  but not the remaining exceptional or 2+2 branches.
- **Derived or proved, 2026-10-22 exceptional follow-up:** The same
  [proof audit](../research/projective_law_followup_20261022/PROOF_AUDIT.md)
  now has exact integer-polynomial certificates and written divisor
  arguments for the `K=0` cancellation and coincident-positive
  nontripling 1+2 branches.
- **Derived or proved, 2026-10-22 tripling and coprime 2+2 follow-up:**
  The same [proof audit](../research/projective_law_followup_20261022/PROOF_AUDIT.md)
  now gives separate exact integer-polynomial certificates, weighted-scale
  checks, and written divisor arguments for tripling and for the generic
  `B≠0,C≠0` 2+2 branch. Exceptional 2+2 outputs remain open.
- **Derived or proved, 2026-10-22 coprime degree-drop follow-up:**
  The [proof audit](../research/projective_law_followup_20261022/PROOF_AUDIT.md)
  adds an exact integer-polynomial certificate and written divisor
  argument for the `B≠0,C=0` 2+2 degree-one branch. Same-`u` and
  shared-support 2+2 output branches remain unproved.
- **Derived or proved, 2026-10-22 same-`u` mixed follow-up:**
  The [proof audit](../research/projective_law_followup_20261022/PROOF_AUDIT.md)
  adds exact weighted point-extraction identities and a written
  valid-divisor cancellation argument for the mixed same-`u` branch.
  Equal-`v` same-`u` doubling and shared-support outputs remain open.
- **Derived or proved, 2026-10-22 same-`u` doubling follow-up:**
  The [proof audit](../research/projective_law_followup_20261022/PROOF_AUDIT.md)
  adds exact Hermite-interpolation, output, and weighted-scale
  certificates for same-`u`, equal-`v` doubling, including `C=0`
  degree one and `B=0` point extraction or identity.
  Shared-support outputs remain open.
- **Derived or proved, 2026-10-22 shared-support follow-up:**
  The [proof audit](../research/projective_law_followup_20261022/PROOF_AUDIT.md)
  adds exact weighted point-extraction and `add11` certificates
  and a written compositional argument for the distinct-`u`,
  `B=0` cancellation and positive branches. Every reachable
  implemented branch now has such a certificate/argument under
  the stated smooth-curve and valid-input assumptions. Faithful
  formalization and independent proof review remain outstanding.
- **Mechanized, 2026-10-25:** The pinned Lean
  [formal boundary](../research/projective_law_followup_20261022/formal/README.md)
  kernel-checks three branch-selection obligations, the generic 1+2 cleared
  output-ordinate identity, both generic 2+2 interpolation factorization
  identities, and a total manual runtime-label-to-certificate inventory.  Its
  axiom audit reports no unexpected axioms.  The runtime source and all eight
  native certificate schemas are SHA-256 bound.  Lean does not parse the Rust
  implementation, and the smooth-curve/Mumford/principal-divisor/`Pic^0`
  bridge is deliberately an explicit unsupplied hypothesis.  This is partial
  mechanization and does not close universal correctness.
- **Mechanized, 2026-10-25 N9:** The same pinned packet now includes a manual
  executable abstraction of the 1+2 branch decision order and selected
  proof-side intermediates associated with the generic 1+2/common 2+2 paths.
  Five additional audited theorems exclude the valid-ordinate mismatch branch
  under a supplied equal-square hypothesis, prove conditional identities for
  the selected intermediates, and prove the final residual-class equality
  through an explicit principal-divisor interface. The 2+2 `N` is a
  constructed proof witness; no derivation from a Rust `N` variable is claimed.
  `runtime-bindings.json` brace-scans and hashes the exact four Rust function
  bodies named by these translations, and the verifier recomputes those
  hashes. This is a narrower, reproducible manual abstraction boundary; it
  does not prove Rust-to-Lean semantic equivalence or construct the required
  smooth-curve/Mumford/principal-divisor instance.
- **Mechanized, 2026-10-25 infinity point:** Pinned Lean proves that the
  explicit infinity-chart complement of its `D(z)` overlap is exactly one
  point, the origin `(z,w)`, over any field with arbitrary coefficients.
  The [packet](../research/projective_law_followup_20261022/formal/COMMON22_COMPLETION_BRIDGE.md)
  records exit 0 and only standard axioms. It does not construct a chosen
  smooth projective completion or prove the divisor/`Pic⁰` bridge.
- **Open compiler/API gap, 2026-10-25:** The
  [weighted-Proj packet](../research/projective_law_followup_20261022/formal/WEIGHTED_PROJ_HEAD.md)
  specifies the proposed degree-`(1,3,1)` quotient, the missing graded
  quotient construction, and the required `D₊(Z)` chart isomorphism. Its
  bounded compiler attempts returned two timeouts and one missing-import
  error; no new theorem was checked.
- **Mechanized, 2026-10-25 weighted head:** A subsequent
  [pinned Lean packet](../research/projective_law_followup_20261022/formal/WEIGHTED_PROJ_HEAD_VERIFIED.md)
  proves the proposed weighted sextic has degree six and its principal ideal
  is homogeneous under weights `(1,3,1)`, with exit 0 and standard-axiom
  audits. Two additional imported-source Lean theorems now check that the
  `Z=1` and `X=1` substitutions yield the exact affine and infinity chart
  equations, also with exit 0 and standard-axiom audits. The earlier failed
  attempts remain preserved. A grading on the
  quotient, the `D₊(Z)` coordinate-ring isomorphism, completion and `Pic⁰`
  transport are still unproved.
- **Mechanized, 2026-10-25 cover/quotient rungs:** The
  [cover packet](../research/projective_law_followup_20261022/formal/WEIGHTED_PROJ_COVER.md)
  checks the sextic factorization and proves that any ambient prime containing
  `X`, `Z`, and the equation contains the coordinate ideal `(X,Y,Z)`; the
  native factorization certificate is `IDC1h2022ac86a412d974`. The
  [ordinary-quotient packet](../research/projective_law_followup_20261022/formal/WEIGHTED_PROJ_QUOTIENT.md)
  checks a surjective `Z=1` algebra map to the affine chart ring, with `Z-1`
  in its kernel. Neither packet constructs the weighted graded quotient or
  proves the exact localized `Proj` chart isomorphism; the smooth projective
  and `Pic⁰` bridge remains open.
- **Mechanized, 2026-10-25 ordinary exact kernel:** A later
  [pinned Lean packet](../research/projective_law_followup_20261022/formal/WEIGHTED_PROJ_KERNEL.md)
  constructs inverse algebra maps between the additional `Z=1` quotient of
  the ordinary sextic ring and the affine curve ring. It proves both
  compositions are identity and that the original specialization's kernel
  is exactly the principal ideal generated by `Z-1`. Four theorem audits
  report standard axioms only. This ordinary quotient result does not
  identify the degree-zero localization in weighted `Proj`, prove its chart
  cover, or supply smooth-projective/`Pic⁰` transport.
- **Mechanized, 2026-10-25 componentwise quotient:** A bounded
  [pinned Lean packet](../research/projective_law_followup_20261022/formal/weighted-proj-components/README.md)
  proves every ordinary sextic quotient class has a finite sum of
  weighted-homogeneous lifts and equality of classes is equivalent to
  equality in every weighted degree. It supplies a descent prerequisite for
  the graded quotient; the `GradedRing` instance, `HomogeneousLocalization.Away`
  identification, and `Proj` chart still have no theorem.
- **Finite source-bound check, 2026-10-25:** One smooth `F17` 2-torsion
  fixture makes both the pinned author public dispatcher and compiled Rust
  dispatcher return identity, matching Cantor. A direct author doubling
  subroutine has two zero weights on that input but is unreachable through
  the public inverse-first dispatcher. The [packet](../research/projective_law_followup_20261022/exceptional_source_bridge/README.md)
  preserves the initially failed assertion and corrected replay. No
  public-law discrepancy or cost change follows from this one input.
- **Finite source-bound check, 2026-10-25 full small-field fixture:** The
  [parity packet](../research/projective_law_followup_20261022/public_source_parity/README.md)
  checks 34,704 weighted ordered pairs over the specified `F7` and `F11`
  models. Source-evaluated author outputs match compiled Rust on all rows,
  and Rust matches independent Cantor on all rows; undefined divisions,
  invalid weights, and mismatches are zero. Magma itself was unavailable,
  and these two fields do not prove universal formula or `Pic⁰` correctness.
- **Suspected explanation:** None needed for the measured cost. Independent
  review of the written branch composition remains a proof obligation.
- **Missing capability:** A mechanically derived or proved Rust-to-Lean
  translation and a concrete smooth-curve/Mumford/principal-divisor instance
  for the abstract class interface, or independent mathematical review of
  the complete branch argument. Hardware speed additionally
  requires isolated timing and the applicable external arithmetic comparator.
- **What this evidence does not establish:** Finite branch checks and successful
  cohort solves do not prove universal formula completeness or wall speed.

## What would unblock it

- [ ] Independently review and finish formalizing the now-complete written
  addition/doubling case map, including the remaining Rust-to-theorem
  correspondence, exceptional compositions, and the concrete divisor-class
  bridge on the stated model family. The 2026-10-25 N9 packet adds manual
  executable abstractions and the abstract residual-class step but leaves the source
  equivalence and geometric construction explicit.
- [x] Replay the original 1,260 exact-order one-target workloads with unchanged identities/seeds and matched source rho; retain primitive/native counts and source-answer checks for both measured arms.
- [ ] If claiming wall speed, run matched L2/A/A timing and the fastest available external arithmetic comparator.

The first condition establishes correctness. The second establishes complete
operation-model cost. The third qualifies a hardware-speed claim.

## Next discriminating step

- **Action or proof obligation:** Have an independent reviewer check the
  exact certificates and written divisor arguments, then formalize the
  complete case split, including zero denominators and degree drops, in a
  pinned algebra/proof environment. Use exhaustive small models as separate
  regressions, not as the proof.
- **Inputs and controls:** Existing affine Jacobian, all converted quintics, origin and exceptional divisors, equal-cost source rho, unchanged target cohort.
- **Execution limits and stop condition:** Preserve the cohort cap, 12 arithmetic prices and native sensitivity band from the [frozen protocol](../research/five_leads_e2e_20261019/protocols/genus2-choice2-v1.md); stop promotion on any mismatched class or scalar.
- **Outcomes and next actions:** A valid proof would close universal
  completeness; a counterexample requires a corrected branch and renewed
  cohort validation. The existing finite-cohort cost result remains scoped to
  its recorded inputs unless a correction changes them.
- **Recheck trigger:** Complete projective arithmetic and matching rho source implementation.

## Resolution and history

2026-10-19: Formula-model compatibility checked; arithmetic and cohort replay unresolved.

2026-10-21: [The branch gate](../research/four_followups_20261021/results/complete-law-branches.jsonl)
now matches Cantor on 13,608 sampled 1+1, 12,096 sampled 1+2, and 6,551
sampled 2+2 cases across the 21 exact-order models. These include 1,175
targeted exceptional 2+2 cases, with a byte-identical second run. The
formulas are still a standalone arithmetic gate: exceptional branch coverage
is not exhaustive, the inversion-free law is not integrated into the
collector, and zero of the 1,260 required DLP solves used it. An affine
normalization with inversions is used only at the comparison boundary. The
original completion conditions remain open.

2026-10-22: The wrapper and retained-state collectors each solved and
source-verified all 1,260 original targets, with independent 36-cell pricing.
The measured-cohort and integration gaps are closed. Universal completeness
of the implemented law remains unproved, as stated in the
[follow-up report](../research/projective_law_followup_20261022/REPORT.md).

2026-10-22: All 42 mapped models received the additional finite audit.
Five branch-selection conditions were discharged
algebraically for valid reduced inputs in the linked proof audit. No
counterexample to the implemented law was found. The universal
output-formula proof remains open.

2026-10-22: The generic weighted-input 1+2 output received exact
symbolic polynomial and homogeneity certificates plus a written
divisor-class proof. Every other output branch remains open;
the universal-completeness status is unchanged.

2026-10-22: The `K=0` 1+2 cancellation and coincident-positive
nontripling paths received exact symbolic certificates and written
divisor-class arguments.

2026-10-22: Tripling and generic coprime 2+2 received exact native
polynomial certificates and written divisor-class arguments. The
unproved output identities are now the exceptional 2+2 branches.

2026-10-22: The coprime `B≠0,C=0` degree-one branch received an
exact native polynomial certificate and written divisor argument.
Same-`u` doubling/mixed composition and `B=0` shared-support
cancellation/positive composition, including nested degree drops,
remain open. Universal completeness remains open.

2026-10-22: Same-`u` mixed support received an exact weighted
extraction certificate and written cancellation argument, composed
with the previously certified non-Weierstrass point doubling.
Equal-`v` same-`u` doubling and `B=0` shared-support paths remain open.

2026-10-22: Equal-`v` same-`u` doubling received exact Hermite,
output, and weighted-scale certificates for its generic, degree-one,
and point-extraction or identity paths. Only `B=0` shared-support
composition paths remain open in the implemented 2+2 case split.

2026-10-22: Distinct-`u`, `B=0` shared support received an exact
weighted extraction/point-addition certificate and written
composition through the previously certified helpers. This completes
the branch-by-branch written proof map, but no faithful Lean theorem
or independent mathematical review has been recorded. The problem
remains open at that stronger proof boundary.

2026-10-25: A pinned Lean/Mathlib packet kernel-checked seven named algebraic
and inventory theorems with no unexpected axioms.  It intentionally leaves
the divisor-class bridge as a caller-supplied proposition and binds the manual
Rust/certificate mapping by hashes rather than claiming source equivalence.
The problem therefore remains open at the universal-correctness boundary.

2026-10-25 (N9): The packet kernel-checked five additional conditional
abstraction and abstract residual-class theorems, for 13 audited theorems total. Four exact
Rust function fragments are independently brace-scanned and hash-bound. The
remaining boundary is a proved source translation and a concrete
smooth-curve/Mumford/principal-divisor class instance; neither is implied by
the fragment hashes.
