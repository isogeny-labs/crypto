# P3 census rho walk: bimodal, curve-reproducible cost above √n

- **Status:** `open`
- **Kind:** unknown cause (instrument defect); limits an empirical outcome
- **Affected work:** [`research/split_prime_ecdlp_20261107/followups/p3_cross_trace/RESULTS.md`](../research/split_prime_ecdlp_20261107/followups/p3_cross_trace/RESULTS.md) (outcome Y1, rho cost against Frobenius-field covariates)
- **Next action owner:** unassigned
- **Last updated:** 2026-11-08

## Problem

We cannot read P3's rho outcome as a statement about Pollard rho, because the
study's own walk (`p3census rho`) has a per-curve cost that is bimodal, grows
more quickly than √n, and reproduces across independent seeds.

Instances: 2,000 random prime-order curves y² = x³ + ax + b over each of
p24 = 16,777,213 and p28 = 268,435,399; 256 targets per curve per run; runs A and
B with disjoint seeds (card `CARD-p3-cross-trace-20261107-2`, run-20261107T2220Z).

## Evidence and uncertainty

- **Observed:** mean ops/√n per curve clusters at about 1.6 and 2.6 (p24) and 2.7
  and 5.1 (p28); r(A, B) = 0.974 and 0.986; between-curve variance 41 and 72 times
  the within-curve expectation; 0 unsolved targets, 0 mismatches, setup cost
  identical across curves (`runs/.../p24|p28/analysis.json`,
  `analysis/derived/p3_descriptive.txt`). A sound negation-map walk costs about
  0.9 · √n, flat in n.
- **Derived:** the jump table is seeded per target (`p3census/src/rho.rs`
  line 66 onward), so a shared table does not explain the A/B correlation.
- **Suspected explanation (conjecture):** 32 jumps with small coefficients
  (u_j, v_j ∈ [0, 16)) under the negation map give many short linear relations
  among jumps, hence recurrent fruitless cycles; some curve-dependent quantity
  (for example through the mix64(x) partition or the canonical-sign choice) sends
  each curve into one of two regimes. Not separated by trace parity, t mod 4,
  n mod 3, n mod 4 or a mod 2.
- **Missing capability:** the lab's `ic_wide::rho_negation` could not be linked
  offline (the main crate's dependencies did not resolve).
- **What this evidence does not establish:** any property of the curves; the P3
  coverage outcome (exact, rho-free) is unaffected.

## What would unblock it

- [ ] On 50 curves at p ≤ 2^20, per-curve fruitless-cycle escape counts and cycle
  lengths tabulated against the two clusters, identifying the binary property.
- [ ] A corrected walk (full-size random jump coefficients) whose per-curve cost is
  flat in n and unimodal (A/A between-curve variance ratio ≤ 2), then P3's Y1
  re-run with the frozen covariates and seeds.

## Next discriminating step

- **Action:** the 50-curve diagnosis above; then the corrected walk at p24.
- **Outcomes:** a named property → fix and re-run Y1; no property found → replace
  the walk with the lab's implementation.
- **Recheck trigger:** the main crate building offline, or a fixed walk.

## Resolution and history

- 2026-11-08: opened from P3's A/A control.
