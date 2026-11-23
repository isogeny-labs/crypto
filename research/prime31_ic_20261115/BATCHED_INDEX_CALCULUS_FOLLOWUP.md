# Prime-field index calculus: batched inversions and full-solver cost on F_(2^31−1)

**Date:** 2026-11-15. **Standing:** [pcc:unverified]. This is a reproducible producer-local component and complete-cold toy-scale measurement, **not** a cheaper-than-rho attack, security claim, or new asymptotic index-calculus algorithm.

## Curve, exclusions and baseline

Use the prior *ordinary*, cofactor-one, prime-order curve, not the excluded smooth-order control:

- p = 2147483647, E: y^2 = x^3 + 847514846 x + 2087444724
- #E(F_p) = 2147446831 (prime), G = (625037715,2100627251)
- Prior [curve certificate](curve_certificate.py), [original complete index calculus](prime31_ic.cpp), [nine old records](RUNS.jsonl), [prior report](README.md)
- The tested group is only about 31 bits, **not** a cryptographically sized challenge. The subgroup order is prime, and the benchmark did not exploit smoothness.

The existing algorithm has a coordinate-restricted B-point factor base, a signed-pair table and a three-summand natural relation query, then rank-B modular elimination and target individual log recovery. This is index calculus **with a group-arithmetic meet-in-the-middle oracle**; its relation search is not a novel subgeneric polynomial solver.

## Optimization and algebra

For two curve points P=(x,y) and Q=(u,v), x(P+Q) and x(P−Q) share the denominator (u−x)^2, as one sees directly from Semaev S3(x,u,z), quadratic in z. Using the known ordinates, its two roots can be written

    z_± = [((x+u)(xu+a)+2b) ± 2 y v] / (x−u)^2.

The optimized table builder computes both signed sums from one denominator inverse and uses **Montgomery batch inversion** across up to 8192 nonzero denominators. It handles i=j separately. This is a standard field-arithmetic optimization, not a new algebraic attack.

The query stage R ± P_k likewise shares x(P_k)−x(R), and batch inversion computes candidates for all k with one inversion except guarded collisions. It then checks the original signed-pair hash in the same k/sign order, preserving the first returned decomposition; R=O falls back to the reference query. Candidate relation equality is checked by group addition.

The separate [batch_pairs.cpp](batch_pairs.cpp) and [batch_decompose.cpp](batch_decompose.cpp) perform exact keyset comparison, signed-representation replay and natural target query comparisons against their reference counterparts. All source and recorded data blobs below were checked against locally compiled bytes; a producer-generated hash is **not** independent PCC certification.

## Factor-base pair setup: exact output, measured cost

Three runs each at B=384,640,1024. Every key (294895 / 818979 / 2095997) was present in both tables, and **every stored optimized pair representation** was independently recomputed with the ordinary group law. All 9 setup trials passed; [raw records](batch_pairs.results.jsonl).

| B | Reference inversions | Batched inversions | Median wall improvement in this local sample |
|--:|--:|--:|--:|
| 384 | 294912 | 393 | 1.74x |
| 640 | 819200 | 665 | 1.65x |
| 1024 | 2097152 | 1088 | 1.44x |

Timing is noisy and hash-table insertion/memory dominates some runs.

## Natural relation-query comparisons

For each B, 301 test inputs consist of 300 randomly selected public subgroup points R=[a]G (the oracle sees R only) and one exact-x collision control. Reference and batched queries returned byte-identical first witnesses or both returned no relation; every positive witness was replayed by group addition. [Raw query records](batch_decompose.results.jsonl).

| B | Positive / 301 | Reference ms | Batched ms | Relative query improvement |
|--:|--:|--:|--:|--:|
| 384 | 8 | 59.2222 | 13.5015 | 4.39x |
| 640 | 47 | 120.135 | 16.0544 | 7.48x |
| 1024 | 141 | 177.306 | 26.3241 | 6.74x |

At B=1024 this changes 387219 inversion calls into 302, but increases counted field multiplications from 1161657 to 2774014. The count of **group-add API calls** is no longer comparable to an algorithm computing the same outputs directly in field coordinates.

## Complete cold index-calculus pipeline

[batch_full.cpp](batch_full.cpp) is a separate self-contained C++17 complete pipeline, including factor-base generation, relation table, naturally sampled [a]G relation targets, full missed-call accounting, exact modular rank and back-substitution, all factor-base-log verification, target recovery and final [d]G=Q check. It includes a same-process BSGS control on each target. No hidden target scalar is used by the index calculus solver. The program samples its public target from a known scalar **for test verification only**, and checks the recovered value.

At B=384/640/1024, each of the three seeds 20261114–20261116 completed at full factor-base rank and correct scalar recovery: **9/9**. Historical baseline [RUNS.jsonl](RUNS.jsonl) reported approximately 4.0 seconds cold per IC solve. The new scalar-target mode [raw nine runs](batch_full.results.jsonl) and a subsequent paired scalar-vs-walk [18-run panel](batch_full_modes.results.jsonl) show approximately **0.9–1.5 s** cold for new IC. The historical 4 s comparison is **not a controlled same-host A/B**, and should not be promoted as a verified overall speedup.

In the exact 18-run paired panel, mean times (ms):

| B | IC with fresh random-scalar targets | IC with public full-period walk | Matched BSGS (scalar panel) |
|--:|--:|--:|--:|
| 384 | 965.0 | 1031.3 | 9.9 |
| 640 | 1080.1 | 1041.1 | 11.7 |
| 1024 | 1205.8 | 1213.3 | 12.1 |

The **walk** is a separate non-IID target-generation control: choose random nonzero start a and stride s mod prime n, use R_{j+1}=R_j+[s]G and update its known scalar a_{j+1}=a_j+s mod n. It removes per-target random-scalar multiplication, but relation rank and number of positive witnesses change. It did not produce a consistent cold-cost win. The scalar mode is the primary comparison.

**Important limitation:** optimized BSGS is still roughly two orders of magnitude faster on this 31-bit benchmark. The existing rho control was not rerun with an optimized negation map. Memory/RSS, final validation under the lab's signed PCC gate, full workspace Rust suite and independent machine replay remain pending. The C++ reference uses a conventional pair table; this work is *not* a sparse PKM m=8 polynomial-system solver.

## Reproduction (repository root)

    g++ -O3 -std=c++17 -DNDEBUG -Wall -Wextra -Werror -o /tmp/batch_pairs research/prime31_ic_20261115/batch_pairs.cpp
    g++ -O3 -std=c++17 -DNDEBUG -Wall -Wextra -Werror -o /tmp/batch_decompose research/prime31_ic_20261115/batch_decompose.cpp
    g++ -O3 -std=c++17 -DNDEBUG -Wall -Wextra -Werror -o /tmp/batch_full research/prime31_ic_20261115/batch_full.cpp
    for B in 384 640 1024; do /tmp/batch_pairs "$B"; /tmp/batch_decompose "$B" 300; done
    for mode in scalar walk; do for B in 384 640 1024; do for seed in 20261114 20261115 20261116; do /tmp/batch_full "$B" "$seed" 200000 "$mode"; done; done; done

**Exact Git blob IDs locally matched on this PR:** batch_pairs.cpp 78147eac1af0ce595748b151e0490b3e8cf9f093; batch_decompose.cpp 894f5456edd55a7edcf59875d775d5364093e5cd; batch_full.cpp 31a3b686b6b95821b05b33eb1e51da91aab3be43; batch_full_modes.results.jsonl 71fd6550848119a71b2e653c45ed3401e4177202. These are integrity checks, **not PCC signatures**. Compilation and commands above passed locally with warnings as errors; not exact-head CI.

## Lead decisions and next experiments

1. **Sparse PKM m=8 natural-positive algebraic decomposition — top hypothesis.** Reuse the same ordinary prime-order curve and B=8,16,32, replace this meet-in-the-middle oracle with a bona fide sparse polynomial-system witness solver. Record *all* failed calls, exact witness extraction, independent rank increments, work per invocation, memory and target recovery. Jointly fit the optimistic gamma and delta gate gamma < 3+delta/2 on held-out larger sizes. This is the necessary missing *structural* advantage; batch inversion changes constants, not the exponent.
2. **Semaev S4 constrained root extraction.** Compare solver calls using the same public x-coordinate target/base instances, with external msolve/Magma controls and explicit certification. Do not count just the polynomial solving degree or planted solutions as positive relations.
3. **Large-prime variation / relation graph.** Admit one/two out-of-base points and test whether cycles increase *independent* full relations after full EC checks; measure graph memory, rank, failed trials and total cold cost. No generic-only graph expansion is an index-calculus breakthrough.
4. **Auxiliary covers and scalar-blind transfer.** Continue #3877 only with a public point map, an exception policy and a fully priced final DLP solve; do not assume a map exists from unrelated Jacobian torsion.
5. **Validation:** independent exact-head native replay (including the complete matrix rank evidence, not only saved answers), strongest available optimized rho benchmark and signed PCC standing. Do not alter leaderboard or merge before gates pass.

**Prior literature:** Petit–Kosters–Messeng, *Algebraic Approaches for the ECDLP over Prime Fields*, PKC 2016 (DOI 10.1007/978-3-662-49387-8_1); Semaev, summation polynomials; Shoup, generic-group lower bounds (EUROCRYPT 1997). Also #4077, #4091, #4188 and this repo's preceding prime31 source. Shared KB/Conductor/EKS were unavailable in this tool context; no novelty claim. All new findings [pcc:unverified].
