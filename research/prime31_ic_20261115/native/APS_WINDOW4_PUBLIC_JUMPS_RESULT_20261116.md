# EXP-R31-APS-WIN4: exact-source completed Rust result

2026-11-16 UTC; outcome: **24/24 matched targets in every arm, 120/120 correct recovered logarithms, preregistered ≥5% cold improvement gate FAILED.** [pcc:unverified]

## Sources, pinned execution, and scope

- [Protocol frozen before code](APS_WINDOW4_PUBLIC_JUMPS_PROTOCOL_20261116.md), commit `1b56e7b19508e02a878f8719e6db008e7191f3d3`; code commits `1a07066c` and `67241caf`; new CI workflow commit `115f9391`. [PR #4454](https://github.com/isogeny-labs/crypto/pull/4454).
- [Actual-source GitHub Actions 43969851443](https://github.com/isogeny-labs/crypto/actions/runs/43969851443), job `133658825956`, source head `d5d7c5bf03676454515ac7a4547a6d08f0a69230`, Ubuntu 24.04, Rust 1.100 release. Job **SUCCESS** 2026-11-16 22:27 UTC: 40/40 reused native library tests, 2/2 targeted Rust scalar/window/partition tests (19 other tests filtered), 24-Q five-arm full cold program and all summary assertions. Source-specific CI is **not independent signed PCC**.
- [Complete raw 24-target × five-arm JSON artifact, ID 14277789822](https://github.com/isogeny-labs/crypto/actions/runs/43969851443/artifacts/14277789822). GitHub Actions reported ZIP SHA-256 `5237d73c181d5714af03d9c55822401a187a42af032ef68a758a7a8806f150ba`. The artifact retains per-round source timing, pair queries, signed witnesses and scalar outputs, zero-filter and rho counters. All four source snapshots in the repo remain intact.

The *same* ordinary full-prime-order curve `p=2147483647`, `y²=x³+847514846x+2087444724`, `n=2147446831`, cofactor one and shared `G=(625037715,2100627251)`; identical public Q for every arm, 24 distinct caller-known-only scalars from fixed seed `57494e3441505331`. Partition `[384,384,16384]`, six complete rounds cap. Each arm performs setup, partition generation, every root query, signed verification, failed rounds and final original-curve scalar replay inside its cold wall.

## Exact recorded total results

| Complete single-target solver (24 Q) | Correct calls | Fully cold total (ms) |
|---|---:|---:|
| Existing joint-binary Shamir + packed S3 APS | 24/24 | 67.549090 |
| **New 4-bit joint-window + same packed S3 APS** | **24/24** | **66.708464** |
| Window4 identical-source A/A repeat | 24/24 | 67.167406 |
| Unfolded 32-lane rho, 32 jumps | 24/24 | 73.031064 |
| Unfolded 32-lane rho, 64 jumps | 24/24 | **68.488801** |

Window4/binary cost ratio `0.98755533198` (1.2445% lower fully cold); **19/24 paired wins**, versus preregistered *at least 5%* improvement and at least 18 wins. Ratio gate **FAIL** because 0.98756 >0.95, even though paired-win gate passes. Window4 vs faster of the two *included* rho aggregate ratios `0.97400542900` (only 2.60% nominal lower), with **12/24** paired wins vs rho32, **13/24** vs rho64. A/A aggregate drift `0.00683280816` (0.683%, passing the 5% stability criterion). Neither aggregate rho comparison supports a claim of outperformance: target-level variation is substantial and the separate tuned WZ rho/external frontier are not included.

Source-generation time including 30-point precomputation and 64 jumps was `25.225735 ms` (binary) versus `24.592164 ms` (window4), a **2.5116% reduction at that phase**; the larger oracle and partition costs dilute whole-call gains. The A/A window4 source was `24.736994 ms`. Source work is ~36.9% of the new total.

An independent post-CI script decoded the archived JSON with Python standard-library arithmetic **only as verifier** (the solver remains entirely existing Rust): recomputed all five totals from 120 case-level times, all 120 `[k]G=Q` source-curve equations, 72/72 signed `alpha+beta*k ≡0 mod n` APS congruences, and **432/432** pairwise source-transcript field comparisons over 24 matched rounds (nine fields, candidate and candidate A/A). No discrepancies were found in these checks. This does not verify the physical timing independently or give a signed PCC verdict. Deterministic 10,000-sample target-paired bootstrap 95% percentile intervals for *aggregate ratios* were: window4/binary **[0.97298,0.99966]**; window4/rho32 **[0.69891,1.20158]**; window4/rho64 **[0.75626,1.26622]**. These are descriptive unisolated-producer intervals, not a guarantee of true performance.

## Per-target fully cold times (milliseconds)

| # | Caller verifier k | Binary | Window4 | Window4 A/A | Rho32 | Rho64 | Δ(W4−binary) |
|--:|--:|--:|--:|--:|--:|--:|--:|
| 0 | 1770200685 | 4.238 | 3.853 | 3.992 | 3.128 | 2.151 | -0.385 |
| 1 | 1230961087 | 3.499 | 3.494 | 3.495 | 3.942 | 1.245 | -0.005 |
| 2 | 59302271 | 2.774 | 2.751 | 2.850 | 1.073 | 4.053 | -0.024 |
| 3 | 788089436 | 1.856 | 1.687 | 1.699 | 2.174 | 5.130 | -0.170 |
| 4 | 325684050 | 3.142 | 3.085 | 3.094 | 1.480 | 3.495 | -0.057 |
| 5 | 1436841898 | 2.410 | 2.388 | 2.388 | 2.567 | 3.386 | -0.022 |
| 6 | 813648145 | 2.513 | 2.571 | 2.798 | 2.083 | 4.689 | +0.058 |
| 7 | 1419173378 | 1.812 | 1.785 | 1.829 | 5.150 | 3.033 | -0.027 |
| 8 | 383730928 | 3.377 | 3.354 | 3.355 | 2.646 | 1.085 | -0.023 |
| 9 | 1631510873 | 3.064 | 2.960 | 2.998 | 2.582 | 1.458 | -0.103 |
| 10 | 1587281664 | 1.755 | 1.724 | 1.719 | 6.087 | 1.596 | -0.030 |
| 11 | 712484892 | 4.854 | 4.945 | 4.895 | 4.820 | 1.561 | +0.090 |
| 12 | 1121479462 | 1.521 | 1.500 | 1.546 | 3.729 | 4.364 | -0.022 |
| 13 | 823610875 | 5.302 | 5.328 | 5.305 | 1.145 | 5.070 | +0.026 |
| 14 | 1684543715 | 1.534 | 1.526 | 1.475 | 2.852 | 1.789 | -0.007 |
| 15 | 769778568 | 1.582 | 1.523 | 1.501 | 1.700 | 3.875 | -0.058 |
| 16 | 1123934572 | 2.875 | 2.883 | 2.822 | 3.167 | 3.227 | +0.008 |
| 17 | 1606242856 | 3.092 | 3.052 | 3.091 | 5.052 | 3.353 | -0.040 |
| 18 | 2083305426 | 4.521 | 4.471 | 4.478 | 3.734 | 1.871 | -0.050 |
| 19 | 1888494135 | 2.245 | 2.215 | 2.229 | 1.230 | 1.559 | -0.030 |
| 20 | 1853358807 | 1.741 | 1.667 | 1.658 | 6.285 | 4.256 | -0.074 |
| 21 | 2026252735 | 2.057 | 2.199 | 2.079 | 0.768 | 1.431 | +0.142 |
| 22 | 589921852 | 3.379 | 3.349 | 3.440 | 1.859 | 1.744 | -0.029 |
| 23 | 457997904 | 2.408 | 2.398 | 2.432 | 3.777 | 3.069 | -0.010 |

## Interpretation, stopped lines, and next experiment

This is a **real source-specific correctness result** and a small measured engineering improvement in an existing index-calculus-derived Rust solver. It did **not** achieve the frozen 5% end-to-end cost gate, nor prove a fastest generic comparison. No variance-based redefinition of the failed gate is permitted. It is **not** an asymptotically non-generic prime-field ECDLP algorithm because its current factorized S3 pair-root oracle explicitly enumerates O(F²) support, equivalent to generic MITM.

Read-back of `algebraic_mode_factorized_index_variant` confirms it **already** uses Fp fibre roots and batched denominator inversion before computing both signed algebraic S3 roots. Replacing it with an unlabelled group-join or assuming the inversion is still one per pair would duplicate existing work or change the research classification. Keep previous frozen negative records: anchored m6 F4 has low natural support and censored matrices; m8 chained F4 is capped; zero-fibre eight-leaf search is mathematically obstructed conditional on D4 injectivity; Dickson31 has two explicit missing eight-leaf targets. The dominant **new scientific gate** remains a target-conditioned, original-ideal-certified S9/K961-S5 coefficient root extractor that returns `f` and signs directly, without a generic four-plus-four table. Use existing Rust root/quotient code from #4228/#4349, blinded B4/B8 controls and exact Dickson31 positives/negatives, then full B31 unknown-scalar rank with all costs charged. For strongest benchmark completion, independently test the tuned Wiener–Zuccherato rho and mature PKM/APS/F4 on a quiet matched host, with signed PCC and validation receipts. No claims are promoted to leaderboards here.
