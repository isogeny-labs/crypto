# JV weak-class cover: next leads after the #3286 review (2026-11-02)

**Scope.** This applies only to the weak class: full rational 2-torsion, `#E = 4l`, and the genus-3 cover over `F_{p^2}`. Nothing here bears on prime-order curves.

**Labels.** *Measured* means read from a record. *Derived* means computed from records. *Model* means an extrapolation.

**Lead atlas.** The atlas (`crypto-autoresearcher/knowledge/leads/ecdlp/`, `tools/lead_atlas.py`) is not in this container's checkout of the sibling repository. Its sparse checkout and tree have neither. The prior-work check below therefore uses this repository's ledger and studies instead.

## Where the frontier is (measured)

The cheapest route for this class is the JV12 §3.2 **sieve** variant (`jv_sieve.rs`), not the Nagao-only arm that #3194, #3278 and #3286 measured. The first record of the V2 study (`research/jv_sieve_strong_rho_v2_20261020/raw/ic_1511_sparse.json`, p = 1511) breaks down as follows:

| phase | F_p mults | share of mults | F_p adds / lookups (native) |
|---|---|---|---|
| line enumeration | 8.64e8 | 25% | 3.98e9 adds |
| sieve | 8.09e8 | 24% | 4.82e9 adds, 2.57e8 lookups |
| descent (2 successes × 736 tests × 9.8e5) | 1.44e9 | 42% | (not emitted) |
| linear algebra | 3.01e8 | 9% | — |
| **total** | **3.43e9 (S = 0.0060)** | | S⁺ = 0.0219 with adds and lookups charged |

**Across the V2 study** (T1, 16 seeds), S = 0.00615 at p = 1511. In `bitop/v1` the ic_acct1 study measured 0.0098 of the strong rho. However, V2's rho A/A rule A4 failed, so V2's rho comparisons are withheld (leaderboard Table D).

## Candidate leads

| # | lead | mechanism | evidence before running | expected gain | cost | decision |
|---|---|---|---|---|---|---|
| L1 | Factor-base line search on the **sieve** route | Choose the line `a·F_p + b` with the largest base `B`. Relations needed `∝ B`, sieve hits per line rise with `B`, and the descent rate rises as `(2B/p)^6`. | #3286 (Nagao arm, ×1.71, B⁻⁵ model exact). Red-team pilot on the sieve route at p = 1511, seeds 17–24 (exploratory): ×1.75 route-only, ×1.42 [1.18, 1.73] net at n = 4096, ×0.92 net at n = 65536. | ×1.3–1.5 (model and pilot) | runs take ~10 s; cheap | **RUN (A)**. Frozen; n fixed by a rule set before any data. |
| L2 | Cheaper search: a χ(h) table over F_q, then per-line lookups | Each line costs `p` lookups and additions and no multiplications. That allows an exhaustive search, ceiling +×1.075 over n = 65536 (red-team census). | Red-team census: the exhaustive maximum is reachable in seconds. Table cost ≈ 38q multiplications. | +5–10% on L1, more if the lookups are cheap in the §5a band | small code change | **RUN, as arm C of A** (same instances and binary) |
| L3 | Matched negation-map rho on the sieve-route instances | V2's rho A/A failed. Re-measure `rhostrong --neg 1` on V2 instances and test homogeneity against the pooled #3278 reference (126 runs, 0.932 steps/√l). | Theory 0.888. #3278 homogeneity p = 0.31. The cover instance class is identical. | Makes the frontier row admissible (accounting, not an improvement) | ~470 s per run at p ≈ 1500 | **RUN (B)**. p = 1511, V2 seeds 17–24, 2 runs each. |
| L4 | Emit the Nagao arm's F_p additions (red-team control O3) | Settle U1 for the Nagao arm. | The arm is dominated by ×83–100. | none (accounting on a dominated arm) | small | **DEFER** |
| L5 | Tune F4 / the per-test cost of the Nagao arm | U2 lever for the Nagao arm. | `f4_fp` is at msolve parity (#1717). Trace replay is a regression; the staircase stop saves nothing (ic_cover_leads L3, L4). | ≤ ×1.08 (F5 criteria, heuristic) | large | **DROP** for now |
| L6 | Large-prime variation | Trade relations for linear algebra. | The jv_cover_large_prime study: ≤ 0.56% at p ≤ 12k (linear algebra is 2e-4 of the cost). | ~0 | — | **DROP** (already negative) |
| L7 | Cheaper descent (42% of sieve-route mults) | Only the B-rate lever applies (inside L1). Six-point tests need all roots in F_p by construction, so a large prime in the descent is impossible. | Measured share 42% at p = 1511. | inside L1 | — | **FOLDED into L1** (report the descent share per arm) |
| L8 | Cut the sieve route's additions (S⁺ is 3.6× S) | Enumeration takes 23.7k adds per line and the sieve 19 adds per step. Finite differences or batching might cut them. | Counters measured in V2. Nothing tried yet. | up to ×2 in S⁺ (model, speculative) | needs design | **NEXT ROUND**: design after L1, since its rows give the post-L1 breakdown |
| L9 | Push p to the u64 limit (1823–2039) for the sieve route | Scaling, not an improvement. | V2 has p = 1823. | none | — | **DROP** (V2 covers it) |

## What runs now (in parallel)

- **A: L1 + L2.** A frozen study of the line search on `run_cover_sieve_dlp_on`, using V2's instances and options (p ∈ {1009, 1511, 1823}, seeds 17–32, `--stop 64`, `JV_LIN=sparse`).
  - **Arm P (plain):** must reproduce V2's records bit-for-bit.
  - **Arm H:** Horner search with n set by a rule in the protocol, before any data.
  - **Arm T:** χ-table exhaustive search, with its lookups and additions emitted and priced in the §5a band.
  - **Outcome:** S and S⁺ paired against P.
  - **Decision rule:** H1 is supported if the geometric-mean upper bound of the net S ratio is below 1. Phase shares are reported.
- **B: L3.** A matched negation rho on V2 instances at p = 1511 (seeds 17–24, 2 runs each, `--neg 1 --batch 16 --r 256 --dp-bits 16`), run on a binary frozen before A's code changes.
  - **Homogeneity test:** against #3278's pooled reference, with the test frozen.
  - **Positive control:** within 3 standard errors of theory.
  - **Result:** the sieve route's S and S⁺ divided by rho, with the §5a band.

Both run on the local 4-core box, the same host as the earlier arms. The counted-operation metrics do not depend on the host, so running in parallel changes wall times only. Each lead leaves a README, graph and PDF in its own study directory. Neither adds a leaderboard row until a validator writes `VALIDATION.json`.

## Outcomes (2026-11-02, both leads run)

### A: line search on the sieve route (`research/jv_sieve_fbline_20261102/`)

**Design.** Frozen with `PROTOCOL.md`, `run.sh` and `rule.txt`, hashed at 19:29Z. They were committed unchanged in 73582ce306, by which time only arm-P and H runs at p = 1009 existed.

**Controls (all pass).**
- C1: plain = V2 bit-for-bit on 48/48.
- C2: the null control.
- C3: table and Horner agree.
- 192/192 runs correct.

**Paired ratio arm/plain, pooled over 48 instances** (95% bootstrap):

| arm | S (muls, search charged) | S⁺ (adds and lookups charged) |
|---|---|---|
| H (n = 4096) | **0.623 [0.562, 0.689]** (×1.61) | 0.550 (×1.82) |
| U (table, n = 32768) | 0.600 | **0.525 [0.491, 0.561]** (×1.90) |
| T (table, exhaustive) | 0.560 (×1.79) | 0.800, and > 1 at p = 1823 |

- **Cheapest arm:** the exhaustive search T wins in multiplications only. Its lookups cost too much in S⁺ and in the §5a band, so U is the arm to use.
- **Decay with p:** the gain falls as p grows (H: 0.495 / 0.644 / 0.759 at p = 1009 / 1511 / 1823), as the B model predicts.

### B: matched negation rho at p = 1511 (`research/jv_sieve_negrho_20261102/`)

**Reference.** 16 runs on V2 seeds 17–24. They are homogeneous with #3278's 126 runs, so the reference is the pooled 142.

**Plain sieve route against that rho** (V2 records, seeds 17–24):

| | U1 | U2 |
|---|---|---|
| S/rho | 0.0065 [0.0055, 0.0080] | 0.0156 |
| S⁺/rho | 0.027 | 0.065 |

The route is below rho at every §5a weight.

### Combined (derived, product of two measured ratios at p = 1511; not a single measured run)

| | U1 | U2 |
|---|---|---|
| sieve route with line search, S/rho | ≈ 0.0065 × 0.644 ≈ **0.0042** (about ×240 below rho) | ≈ 0.010 |
| arm U, S⁺/rho | ≈ 0.027 × 0.525 ≈ 0.014 | ≈ 0.034 (about ×29 below rho) |

### Next round

1. **Validator and red team on A and B.**
2. **Lead L8: cut the sieve route's additions.** Line enumeration and the sieve dominate S⁺.
3. **Combine the two measurements.** Run one study with the matched rho and the line-search arm on the same instances, instead of multiplying ratios.
