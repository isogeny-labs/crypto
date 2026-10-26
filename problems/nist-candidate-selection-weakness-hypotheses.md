# NIST candidate selection: weakness hypotheses remain unresolved

- **Status:** `investigating`
- **Kind:** unknown selection cause; missing applicable attack construction and validation
- **Affected work:** [selected/skipped comparison and protocol](../research/curve_candidates_dissect_20261021/hypotheses/REPORT.md)
- **Next action owner:** unassigned
- **Last updated:** 2026-10-26
- **Latest search:** [25–26 October full-census criterion search](../research/nist_skip_criterion_20261025/REPORT.md)

## Problem

We cannot yet establish that the published P-224 or P-256 curve is vulnerable
to an ordinary-input attack, or that weaknesses in earlier eligible curves
explain why they were skipped. The historical skip rule and any applicable
hidden-attack witness remain unknown.

The initial six exact models, generators and full ICV1 identities remain
frozen in the study's `hypotheses/inputs.tsv`. The latest linked study expands
the recorded population to 21 eligible prime curves: ten published and eleven
skipped across six streams. Its source and normalized parameter tables remain
separate. Eligibility uses the recorded known checks; it is not a historical
decision log.

## Evidence and uncertainty

- **Observed:** the earlier eligible indices match the four supplied curves.
  The native diagnostic rechecks seeds, orders, twists, nearby-order factors
  and 3-division polynomials on the six exact models.
- **Observed:** P-224 index 1485 has a larger largest twist prime than the
  published P-224 curve. Published P-256 has a stronger twist but a smoother
  `n-1` than its skipped counterpart. Published P-224 has Frobenius-order
  conductor 3 and a fully split 3-division polynomial, unlike its three
  skipped counterparts. Exact values and scope are in the linked report.
- **Observed:** the [native factorization follow-up](../research/nist_skip_criterion_20261025/factor_followup_20261026/REPORT.md)
  has proved p239 index 2491 `n+1`, P-384's discriminant magnitude and
  P-521 `n-1` completely. This raises admitted full factorizations to 83/84.
  The all-21 search can now use complete `n-1` and discriminant features;
  its best structural conjunction still has three tests. P-521 `n+1`
  retains a 125-digit composite and the full `n+1` family stays disabled.
- **Derived or proved:** primality and product checks certify the recorded
  factorizations; generator and Hasse checks certify the six prime full
  orders. These are computational checks, without Lean formalization.
- **Suspected explanation:** unknown. Deliberate weak-curve selection,
  deliberate avoidance of weak candidates, a family-wide unpublished
  method, and engineering or bookkeeping filters remain distinct hypotheses.
- **Missing capability:** an explicit applicable attack or retained-witness
  route, its full cost, and a prospective selection-rule test.
- **What this evidence does not establish:** a trait difference is not an
  ECDLP attack. Conditional auxiliary-input or twist attacks need actual
  protocol preconditions. Routine checks and bounded negative searches do
  not prove security or exclude an expensive intermediate advantage.

## What would unblock it

- [ ] Freeze the attack contract: exact subgroup, public and auxiliary inputs,
      oracle access, any retained witness, construction and recovery costs.
- [ ] Supply and verify an executable route, including subgroup preservation
      and independent recovery, against the matched strongest reference.
- [ ] For a selection explanation, specify a rule and confirm it on held-out
      matched candidates after conditioning on all known acceptance filters;
      retain failures and apply the declared multiple-testing policy.

An attack can establish vulnerability without establishing anyone's intent.
A predictive filter can explain selection without establishing an attack or
proving historical use. Neither hypothesis is disproved by failure to find
a route within the present diagnostic.

## Next discriminating step

For the selection question, obtain an original generator/acceptance log,
additional genuine historical choices outside the fitted dataset, or a
specified mathematical mechanism with independent predictions. Freeze the
rule before examining new labels, and replay any recovered code against the
entire recorded stream. Random new streams alone cannot validate a claim
about historical choices. Group-order-dependent RNG schedules and an
unrecovered old SEA implementation remain outside the tested finite family.

The 21-curve diagnostic, short-rule calibration and 52,520-model generation
search ran. No attack, explicit map or new historical holdout campaign has
been scheduled. For the vulnerability tracks, the original explicit-map and
auxiliary-input obligations remain. New runs retain their own protocol,
limits, failures and exact witness obligations.

## Resolution and history

2026-10-21: connected the supplied curves to the pinned earlier-candidate
census and added a native six-curve diagnostic. Both weakness hypotheses
and the historical skip explanation remain unresolved.

2026-10-26: expanded to all 21 eligible prime curves and found independently
verified empirical two-test and three-test separators. No historical
mechanism is established: 4,385 of 4,480 permitted shuffled labelings admit
an exact one- or two-test separator somewhere in the compared search families.
None of 52,520 defined RNG/generation models fits all labels. All 21 twist
orders are fully factored and proved. The raw records, finite search scope,
root-convention counterexample and remaining evidence obligations are retained
in the linked report. An exact fit does not resolve the selection explanation
or establish a vulnerability. Unresolved.

2026-10-26 follow-up: three of the four remaining factorization targets
passed independent native prime/product proofs. The unchanged full-21
search replay has 1,118 features per row and retains the existing fits.
Factor completion and the historical explanation remain separate obligations.
