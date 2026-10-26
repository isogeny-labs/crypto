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
  has proved all four requested integers, including P-521 `n+1`, completely.
  There are 84/84 complete records and 1,127 features per row in the replay.
  All four complete-factor families are admitted over all 21 controls.
  The shortest structural conjunction still has three tests; generation
  features retain the two-test fit. Neither establishes historical use.
- **Observed:** an [exact native SEA-feature replay](../research/nist_skip_criterion_20261025/sea_replay_20261026/REPORT.md)
  matches all 21 recorded bounds. A uniform preference for a larger bound
  fails on P-256 (published191, skipped211); the reverse fails on P-224
  (published257, skipped139/179/233). The proxy includes ramified primes
  and omits Atkin information. No new p-value, measured SEA cost or
  historical interpretation is admitted.
- **Observed:** [exact embedding degrees](https://github.com/isogeny-labs/crypto/pull/2648)
  were certified for all21 orders. Maximal degree accepts0/10 published and
  2/11 skipped; the modern RFC7748 bound accepts10/10 published and10/11
  skipped. Opposite-label P256 orders and several P224 orders share
  embedding index0x3. These fixed criteria do not explain all skips.
- **Observed:** the [period ECPP artifact trial](../research/nist_skip_criterion_20261025/period_ecpp_20261026/REPORT.md)
  executed all21 known-prime orders with one fixed full-catalog configuration.
  All21 raw CLI labels were prime, with no composite label. Corrected saved
  certificate admission accepts3 published and6 skipped, retaining7 published
  and5 skipped as indeterminate because of actual failed legacy theorem rows.
  All three originally supplied skipped P224 curves pass admission. The
  twelve unknowns are not weak/composite/rejected labels. This configuration
  cannot provide a universal rejection explanation; historical use is unknown.
- **Derived or proved:** primality and product checks certify the recorded
  factorizations; generator and Hasse checks certify the six prime full
  orders. These are computational checks, without Lean formalization.
- **Observed:** the [exact embedding-degree test](../research/nist_skip_criterion_20261025/exact_embedding_20261026/REPORT.md)
  computed and separately certified all21 minimal degrees. None of ten
  published curves has maximal degree; two skipped curves do. The fixed
  modern RFC7748 bound accepts ten published and ten skipped curves,
  rejecting only skipped p239 index1562. Published/skipped P-256 both have
  `(n-1)/k=0x3`, precluding separation by a filter of that normalized degree
  and their common field alone. These are exact counterexamples to those
  filters, not historical attribution or a practical pairing attack.
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
search, full84 factorization replay, exact-degree checks and fixed period-ECPP
trial ran. The ECPP source mirror is not authenticated as the original
generator's bytes, and no original caller/acceptance record links it to those
choices. Further artifact/configuration variants are untested; this bounded
negative trial does not exclude all certificate or historical resource filters.
No attack, explicit map or new historical holdout campaign has been scheduled.
For the vulnerability tracks, the original explicit-map and
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

2026-10-26 follow-up: all four remaining factorization targets passed
independent native prime/product proofs. The unchanged full-21 search replay
has 1,127 features per row and retains the existing conjunction lengths.
Generation-family as-good calibration is 1,973/4,480 for an exact fit of
at most two tests; the earlier 15-family union remains frozen separately.
The [factorization obstruction](nist-p521-nplus1-composite-cofactor.md) is
resolved. The historical explanation and applicable attacks remain unproved.

2026-10-26 continuation: all21 embedding degrees were checked exactly, and
all21 period ECPP prover/checker pairs actually ran. Six admitted skipped
certificates contradict the fixed artifact's reject-all-skips hypothesis.
The native CM metadata reader was corrected from original source and every
saved packet revalidated without a legacy rerun. Twelve genuine failed
theorem transcripts remain indeterminate. All failed preparation/runtime
attempts and the initial restrictive-reader results are retained. This is
additional negative evidence, not completion of the historical selector goal.
