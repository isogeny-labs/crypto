# P3 census rho walk: bimodal, curve-reproducible cost above √n

- **Status:** `resolved` (2026-11-08: root cause found, walk v2 passes the gate and the claim rerun)
- **Kind:** unknown cause (instrument defect); limits an empirical outcome
- **Affected work:** [`research/split_prime_ecdlp_20261107/followups/p3_cross_trace/RESULTS.md`](../research/split_prime_ecdlp_20261107/followups/p3_cross_trace/RESULTS.md) (outcome Y1, rho cost against Frobenius-field covariates)
- **Next action owner:** unassigned
- **Last updated:** 2026-11-08 (resolved)

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
- **Diagnosis (2026-11-08, round 2 builder; instrumented copy of v1, 50 prime-order
  curves over p = 1048573 = 2^20 - 3, 64 targets each) [pcc:unverified]:**
  - *Binary property: n >= 2^k.* The DP bit count was floor(log2 n / 4). All study primes
    sit just below a power of two (2^20 - 3, p24 = 2^24 - 3, p28 = 2^28 - 57), so the Hasse
    interval straddles 2^k and curves with n >= 2^k get one more DP bit. At 2^20: 36 curves
    with n >= 2^20 cost 1.698 · √n (45.0 steps per walk, 0.716 capped walks per target),
    14 with n < 2^20 cost 1.328 (17.4 steps per walk, 0.280 capped). Confirmation on the
    first 6 p24 claim curves (regenerated with the card's curves seed, 24 targets): the 5
    with n >= 2^24 cost 2.26 - 2.65 · √n, the one with n < 2^24 1.57: the two clusters.
  - *Mechanism: walks trapped in cycles longer than the 16-point window.* Capped walks end
    in cycles of period 17 - 63 (many odd, which needs integer relations among the jump
    logarithms u_j + v_j k with u_j, v_j < 16) and run to the cap 20 · 2^d + 64. Wasted
    ops per target 0.11 vs 0.49 · √n at 2^20, 0.48 vs 1.2 - 1.7 at p24; the waste scales
    with walk length and cap (both ∝ 2^d ∝ n^(1/4)), hence the growth from p24 to p28.
    Time to the first point collision between walks is 88 - 97 % of the cost.
  - *Ruled out:* per-curve DP density (1.00 - 1.01 × 2^-d), flip rate (0.50 - 0.52).
  - *Fix (walk version 2, `p3census/src/rho.rs`, README "Walk version 2"):* full-size
    G-only jumps in a per-(run, curve) table, a per-target partition salt, a
    point-dependent escape m + M_s + M_t (a fixed escape point gave a ping-pong
    involution), DP bits from bitlen(p). Validation at p = 2^16 - 15 ... 2^22 - 3, 50
    curves, T = 256, runs A and B: means 1.188, 1.057, 1.035, 0.989 · √n, A/A var ratios
    0.72 - 1.32, r(A, B) −0.16 ... +0.23, no n >= 2^k effect, 0 unsolved. This
    validates the instrument only; Y1 has not been re-run.
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
- 2026-11-08: resolved. Root cause: distinguished-point bits set per curve from
  floor(log₂ n / 4) at primes just below a power of two, plus long fruitless cycles from
  small jump coefficients. Walk v2 (`p3census`, src sha256 `4888b050…aed0`) passed the
  frozen gate and the claim rerun (`CARD-p3-cross-trace-20261108-r2`): A/A r = 0.027 / 0.009,
  variance ratios 0.95 to 1.02, every per-curve mean in 0.850 to 1.091·√n. See
  `research/split_prime_ecdlp_20261107/followups/p3_cross_trace/RESULTS.md`, round 2.
