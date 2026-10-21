# NIST candidate selection: weakness hypotheses remain unresolved

- **Status:** `investigating`
- **Kind:** unknown selection cause; missing applicable attack construction and validation
- **Affected work:** [selected/skipped comparison and protocol](../research/curve_candidates_dissect_20261021/hypotheses/REPORT.md)
- **Next action owner:** unassigned
- **Last updated:** 2026-10-21

## Problem

We cannot yet establish that the published P-224 or P-256 curve is vulnerable
to an ordinary-input attack, or that weaknesses in earlier eligible curves
explain why they were skipped. The historical skip rule and any applicable
hidden-attack witness remain unknown.

The exact fields, models, prime subgroups, generators, full ICV1 identities
and stream positions are frozen in the study's `hypotheses/inputs.tsv`.
The two within-stream cohorts contain six eligible candidates: P-224
indices 777, 1485, 5736, 7026 and P-256 indices 876, 1842. The last index in
each cohort was published. Eligibility uses the pinned census's known
checks; it is not a historical decision log.

## Evidence and uncertainty

- **Observed:** the earlier eligible indices match the four supplied curves.
  The native diagnostic rechecks seeds, orders, twists, nearby-order factors
  and 3-division polynomials on the six exact models.
- **Observed:** P-224 index 1485 has a larger largest twist prime than the
  published P-224 curve. Published P-256 has a stronger twist but a smoother
  `n-1` than its skipped counterpart. Published P-224 has Frobenius-order
  conductor 3 and a fully split 3-division polynomial, unlike its three
  skipped counterparts. Exact values and scope are in the linked report.
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

Complete the native matched trait baseline, then resolve the published
P-224 local 3-isogeny/endomorphism structure using explicit coordinate and
map certificates. In parallel, make the nearby-order attack preconditions
explicit. The report defines controls and the broader 21-candidate census.
Only the six-row arithmetic diagnostic ran; future attack, map and fresh
control campaigns have not been scheduled. New runs retain their own
protocol, limits, failures and exact witness obligations.

## Resolution and history

2026-10-21: connected the supplied curves to the pinned earlier-candidate
census and added a native six-curve diagnostic. Both weakness hypotheses
and the historical skip explanation remain unresolved.
