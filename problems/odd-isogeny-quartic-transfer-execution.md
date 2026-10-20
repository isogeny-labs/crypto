# Executable odd-isogeny transfer to a rational quartic

- **Status:** `open`
- **Kind:** implementation gap and proof gap
- **Affected work:** [five-leads follow-up](../research/five_leads_e2e_20261019/STATUS.md), L1 cubic Weil restriction and L2 genus-3 Smith-negative exits
- **Next action owner:** unassigned follow-up
- **Last updated:** 2026-10-20

## Problem

The cubic fixture now has a checked source-to-plane-quartic Jacobian map and
a complete mapped DLP run. Its published quotient has not been independently
constructed from the checked 3-kernel. The genus-3 Smith-negative route has
neither an odd-kernel screen nor a mapped DLP.
For L1 the named input is Tian's `F_523^3` elliptic curve and 3-kernel. For
L2 the positive control is his genus-3 curve over `F_257`, followed by the
requested frozen 20 Smith-negative odd-characteristic inputs at ell 3 and 5.

## Evidence and uncertainty

- **Observed:** [native L1 kernel replay](../research/five_leads_e2e_20261019/correctness/tian-cubic-kernel-v1.json) checks the actual nine components, 27-element Frobenius-stable maximal isotropic subgroup, Weil pairings, and source group order by a prime-order point and Hasse uniqueness. [Native quartic checks](../research/five_leads_e2e_20261019/correctness/tian-quartic-smoothness-v1.jsonl) derive a unit gradient ideal in each affine chart for both published rational quartics, proving geometric smoothness for the copied equations. A [published-class line-sieve DLP](../research/five_leads_e2e_20261019/correctness/tian-quartic-line-sieve-dlp-v1.json) recovers 1987 from Tian's destination classes S,T, checks its two rational divisor relations by a second Jacobian construction, and verifies the scalar on the destination and source. Its [36-price analysis](../research/five_leads_e2e_20261019/results/tian-quartic-line-sieve-summary.json) loses to optimized source rho in every cell; at the representative price the loss is 561.59×. These published classes are direct inputs to that control. [L2 T1 replay](../research/five_leads_e2e_20261019/correctness/tian-genus3-f257-t1-result.json) checks one published rational divisor's order three. A [20-input holdout](../research/five_leads_e2e_20261019/fixtures/smith-negative-holdout-v1.json) is frozen from the earlier `F_2147483647` Smith census; all 20 degree-eight polynomials and archived negative verdicts [materialize and replay](../research/five_leads_e2e_20261019/correctness/smith-negative-materialized-v1.jsonl). No odd-kernel screen has run on those inputs.
- **Mapped cubic result:** [101 source-point evaluations](../research/five_leads_e2e_20261019/correctness/tian-cubic-map-gate-v2.jsonl) of the six published rational functions have nonzero denominators and separable coordinate cubics. The quartic multiplication map has rank six in each nine-dimensional cubic-pair algebra, giving three geometric points of support. A corrupted coefficient fails the same check. Their [Frobenius-traced Jacobian classes](../research/five_leads_e2e_20261019/correctness/tian-cubic-trace-map-gate-v1.json) match all 100 source multiples and Q. The [mapped line-sieve DLP](../research/five_leads_e2e_20261019/correctness/tian-quartic-line-sieve-mapped-dlp-v1.json) recovers 1987 without it as solver input and verifies on source and destination. Its [full 36-price grid](../research/five_leads_e2e_20261019/results/tian-quartic-line-sieve-mapped-price-grid.csv) loses to optimized rho in every cell: 645.65× at the representative price, 47.14–787.55× over the band.
- **Derived or proved:** A subgroup of order 27 isotropic for the product pairing is maximal isotropic in the six-dimensional 3-torsion module of the Weil restriction, under the checked field and group-law implementation. This alone does not construct its quotient.
- **Suspected explanation:** Unknown whether the holdout contains a rational odd-kernel exit; no negative search has been run.
- **Missing capability:** extension-field genus-3 torsion and Weil-pairing arithmetic for L2, including the holdout's degree-eight two-infinity model; independent construction of the L1 quotient from the checked kernel, actual-kernel and dual-recovery checks, and exceptional-input handling; theta/Weil-set quotient construction and rational descent for L2; destination solver adaptation for L2 and full mapped-source cost accounting there.
- **What this evidence does not establish:** The mapped cubic example does not prove the independently checked 3-kernel is the kernel of the published map, cover exceptional inputs, or establish a growing-size advantage. It gives no Smith-negative coverage or mapped genus-3 cost.

## What would unblock it

- [ ] Reconstruct the published `F_523` quotient and `F_257` positive-control quotient, with independent smoothness/descent and actual-kernel checks and executable image/dual maps, including exceptional points.
- [x] Verify 100 public known-answer transfers on the cubic source and freeze 20 exact Smith-negative identities. The Smith odd-kernel test still needs to run.
- [x] Run a complete cold mapped one-target DLP on the published cubic fixture, verify on the source, and pair with source rho. Destination generic and larger-size comparisons remain separate checks.

The first two conditions establish construction and coverage only. The third
is needed for a solver-cost claim.

## Next discriminating step

- **Action or proof obligation:** Prove the published map's exact 3-kernel and dual composition, then run odd-kernel screens on the frozen genus-3 holdout.
- **Inputs and controls:** Published kernel/quartic, zero and exceptional points, deliberately non-isotropic kernel, extension-only descent rejection, source scalar checks.
- **Execution limits and stop condition:** Follow the frozen [lead specification](../research/new_leads_20261019/README.md) and record actual runtime/resource limits before a cohort run. Stop promotion on any map, descent or source scalar mismatch.
- **Outcomes and next actions:** A verified map enables complete cost testing; a failed published replay requires an arithmetic or transcription diagnosis; a resource failure remains an execution limit, not nonexistence.
- **Recheck trigger:** A checked odd-isogeny quotient/map implementation or a completed positive-control replay.

## Resolution and history

2026-10-19: Actual cubic kernel and source order certified; both published
rational quartics checked smooth; one genus-3 torsion generator checked; 20
Smith-negative inputs frozen with replayed identities. Odd-kernel screens,
quotients and DLP transfers remain unresolved.

2026-10-20: A genuine line-sieve index-calculus solve on Tian's *published
destination classes* recovered 1987 and source-verified it. Complete counted
field operations lose to source rho in every price cell. The executable
source-to-Jacobian map, quotient check and Smith-negative odd-kernel tests
remain open.

Later 2026-10-20: The six published map functions were extracted from
SHA-checked original TeX and evaluated on 101 source points. Their cubic
cuts meet the smooth quartic in exactly three geometric points in every
case, with a discriminating corrupted-coefficient control. Frobenius trace
classes match all 101 source points, and a mapped line-sieve DLP recovers
1987 and loses 645.65× to source rho at the representative price. Independent
quotient-from-kernel and Smith-negative transfer remain open.
