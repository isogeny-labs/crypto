# Executable odd-isogeny transfer to a rational quartic

- **Status:** `open`
- **Kind:** implementation gap and proof gap
- **Affected work:** [five-leads follow-up](../research/five_leads_e2e_20261019/STATUS.md), L1 cubic Weil restriction and L2 genus-3 Smith-negative exits
- **Next action owner:** unassigned follow-up
- **Last updated:** 2026-10-19

## Problem

The cited odd-torsion constructions cannot yet be compared as complete DLP
pipelines in this repository because there is no checked quotient, rational
plane-quartic Jacobian map, and scalar-recovery adapter for either source.
For L1 the named input is Tian's `F_523^3` elliptic curve and 3-kernel. For
L2 the positive control is his genus-3 curve over `F_257`, followed by the
requested frozen 20 Smith-negative odd-characteristic inputs at ell 3 and 5.

## Evidence and uncertainty

- **Observed:** [native L1 kernel replay](../research/five_leads_e2e_20261019/correctness/tian-cubic-kernel-v1.json) checks the actual nine components, 27-element Frobenius-stable maximal isotropic subgroup, Weil pairings, and source group order by a prime-order point and Hasse uniqueness. [Native quartic checks](../research/five_leads_e2e_20261019/correctness/tian-quartic-smoothness-v1.jsonl) derive a unit gradient ideal in each affine chart for both published rational quartics, proving geometric smoothness for the copied equations. [L2 T1 replay](../research/five_leads_e2e_20261019/correctness/tian-genus3-f257-t1-result.json) checks one published rational divisor's order three. A [20-input holdout](../research/five_leads_e2e_20261019/fixtures/smith-negative-holdout-v1.json) is frozen from the earlier `F_2147483647` Smith census; all 20 degree-eight polynomials and archived negative verdicts [materialize and replay](../research/five_leads_e2e_20261019/correctness/smith-negative-materialized-v1.jsonl). No odd-kernel screen has run on those inputs. None of these outputs contains a quotient or image point.
- **Derived or proved:** A subgroup of order 27 isotropic for the product pairing is maximal isotropic in the six-dimensional 3-torsion module of the Weil restriction, under the checked field and group-law implementation. This alone does not construct its quotient.
- **Suspected explanation:** Unknown whether the holdout contains a rational odd-kernel exit; no negative search has been run.
- **Missing capability:** extension-field genus-3 torsion and Weil-pairing arithmetic for L2, including the holdout's degree-eight two-infinity model; theta/Weil-set quotient construction, rational descent, an executable map and dual-recovery check for both; a destination quartic solver adapter with complete cost accounting.
- **What this evidence does not establish:** It does not reproduce the quotient, verify the rational map, establish coverage on Smith-negative inputs, or price a single source-to-answer solve.

## What would unblock it

- [ ] Reconstruct the published `F_523` quotient and `F_257` positive-control quotient, with independent smoothness/descent and actual-kernel checks and executable image/dual maps, including exceptional points.
- [ ] Verify 100 public known-answer transfers on the cubic source, freeze 20 exact Smith-negative identities before testing ell 3 and 5, and retain every failed kernel/quotient attempt.
- [ ] Run complete cold one-target DLPs through each successful transfer, verify every answer on the source, and pair them with source rho and destination generic solves on identical targets.

The first two conditions establish construction and coverage only. The third
is needed for a solver-cost claim.

## Next discriminating step

- **Action or proof obligation:** Implement the published quotient/map for the exact `F_523` fixture, then independently check 100 transferred public targets and the dual composition.
- **Inputs and controls:** Published kernel/quartic, zero and exceptional points, deliberately non-isotropic kernel, extension-only descent rejection, source scalar checks.
- **Execution limits and stop condition:** Follow the frozen [lead specification](../research/new_leads_20261019/README.md) and record actual runtime/resource limits before a cohort run. Stop promotion on any map, descent or source scalar mismatch.
- **Outcomes and next actions:** A verified map enables complete cost testing; a failed published replay requires an arithmetic or transcription diagnosis; a resource failure remains an execution limit, not nonexistence.
- **Recheck trigger:** A checked odd-isogeny quotient/map implementation or a completed positive-control replay.

## Resolution and history

2026-10-19: Actual cubic kernel and source order certified; both published
rational quartics checked smooth; one genus-3 torsion generator checked; 20
Smith-negative inputs frozen with replayed identities. Odd-kernel screens,
quotients and DLP transfers remain unresolved.
