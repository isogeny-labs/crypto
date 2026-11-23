# Original Prime31 RR9 four-plane graph-chart experiment — source-exact result

**Recorded 2026-11-19 UTC; draft [PR #4780](https://github.com/isogeny-labs/crypto/pull/4780), stacked on [#4776](https://github.com/isogeny-labs/crypto/pull/4776).** Preregistered [protocol](RR9_GRAPH_CHART_PROTOCOL_20261119.md) was committed **before** implementation and measurements. **PASS**: [source-specific GitHub Actions #44452737502](https://github.com/isogeny-labs/crypto/actions/runs/44452737502), source `122f6d9c830d4111c0c3261eb2209fc24743858c`, pinned original Rust arithmetic `1df714645aa683cf975bc76fcacb9842cb4780fd`, Rust 1.100.0. Original 40/40 Rust library tests + 4/4 example algebraic tests; independent Python BigInt EC truth replay and 62 exact RR9 determinant-residue checks passed. Independently signed PCC remains **[pcc:unverified]**.

Raw source-bound [four-arm JSON artifact #14496176710](https://github.com/isogeny-labs/crypto/actions/runs/44452737502/artifacts/14496176710) (SHA-256 `e3fe6eeee64d2b9bf21b913bcd6173788ed9b6b7daa164ff58a8d65c3b7cd1fa`). Frozen [machine-readable summary](RR9_GRAPH_CHART_SUMMARY_20261119.json).

## Mathematical construction and exact qualifications

On the **unchanged**, ordinary, cofactor-one curve `p=2147483647`, `a=847514846`, `b=2087444724`, `n=2147446831` (prime), `G=(625037715,2100627251)`, the public residual `R=(r,s)` and signed public leaf `Q=(x,y)` give a target-conditioned RR9 row

```text
[(x-r), (x-r)x, (x-r)x², (x-r)x³ | (y+s), (y+s)x, (y+s)x², (y+s)x³].
```

Partition the 8 rows into four-row halves `H_L=[A_L|B_L]`, `H_R=[A_R|B_R]`. For both `A` nonsingular, set `M=A^{-1}B`. Exact Schur determinant identity:

```text
det([A_L B_L; A_R B_R]) = det(A_L) det(A_R) det(M_R - M_L).
```

Here `det(A)=∏_i(x_i-r)·Vandermonde(x_0,x_1,x_2,x_3)`, hence the formula is admissible only when each four-leaf block has distinct x coordinates and x!=r. The source **fails closed** on a singular chart; neither a no-witness verdict nor claimed coverage on singular fibres follows. Repeated signed points and the leaf `-R` are rejected on acceptance; no Hermite extension is claimed. The Schur/graph-chart identity is **classical linear algebra**, not a novel theorem.

Compute the 4x4 determinant of `D=M_R-M_L` using complementary 2x2 minors: six products of two 2x2 determinants, hence **30 Fp multiplications per tested pair** (24 inside minors + 6 products), versus 70 Fp coordinate products for the exterior-4 contraction in #4776. This per-pair count excludes scalar chart construction and its field inversions, and excludes modular additions/subtractions and independent EC checking. Two half tables hold 16 elements per tuple instead of 70: **4.375x smaller coordinate storage**.

This is still **exhaustive** algebraic pair search, not unknown-index nonenumerative F4/XL/SAT extraction. It performs no EC group-sum table lookup or EC addition in the extraction function; the independent grading oracle is entirely out of band.

## Same-runner exact-source B4 paired experiment

All four runs use the same original curve, signed B4 supports, exact planted positive (first all-zero signed word) and independently verified natural negative `R=G`, with 4,096 tuples per half and **16,777,216** tested pairs for the negative. Order was exterior–graph–graph–exterior, charged with algebraic preprocessing and 2 full negative scans per arm.

| Diagnostic B4 candidate time | Exterior 4-plane 70-term | Graph-chart 30-product |
|---|---:|---:|
| Full negative R=G run 1 | 2025.929563 ms | 1444.170124 ms |
| Full negative R=G run 2 | 2022.829508 ms | 1446.774604 ms |
| **Full negative mean** | **2024.379536 ms** | **1445.472364 ms** |
| First-pair planted run 1 | 289.090905 ms | 5.426871 ms |
| First-pair planted run 2 | 289.880874 ms | 5.352730 ms |
| **First-pair planted mean** | **289.485890 ms** | **5.389801 ms** |
| Negative pairwise coordinate products | 1,174,405,120 | **503,316,480** |
| B4 both-half vector coordinate bytes | 4,587,520 | **1,048,576** |
| Negative witness | none (full scan) | none (full scan) |
| Positive accepted word | [0,0,0,0,0,0,0,0] | same |

**Measured 1.4005x faster** (28.60% less wall time) on the paired, fully exhaustive B4 **algebraic negative candidate**. Its exact per-pair Fp-product reduction is **2.3333x**, not its actual elapsed speedup. The first-pair planted time is intentionally only a diagnostic: it cannot approximate natural-target hit yield, because the witness is located at the very first attempted pair and preprocessing dominates. These numbers do **not** include independently graded EC group operation time or whole ECDLP setup/training/recovery.

## B8 exact work/space falsifier

| Exhaustive negative B8 per public R | Exterior-4 | Graph-chart |
|---|---:|---:|
| Half assignments | 65,536 | 65,536 |
| Pair checks | 4,294,967,296 | 4,294,967,296 |
| Pairwise coordinate products | 300,647,710,720 | **128,849,018,880** |
| Both-half coordinate storage | 73,400,320 bytes | **16,777,216 bytes** |

For 1,024 complete **negative** B8 scans, graph-chart alone would require **131,941,395,333,120 coordinate products** before chart inversions, setup, additive work or recovery. This is specific to the exhaustive implementation, and **not a lower bound on a nonenumerative extractor**.

The prior same-runner [#4774](https://github.com/isogeny-labs/crypto/pull/4774) signed shared-BSGS controller recovered 1,024 public logs in **3342.793 ms** with **1,084,071,298 Fp multiplications** over the *entire generic solver*. This external benchmark is a different run and workload; ratios across these counts are merely scaling warnings, not full wall-time comparisons. Four-window [#4728](https://github.com/isogeny-labs/crypto/pull/4728) obtained 31/31 rank and 1,024/1,024 logs **with generic relation grader tables**; no replacement of these tables occurred here.

## Previous areas searched / nonduplication

- **RR9 track**: [#4466](https://github.com/isogeny-labs/crypto/pull/4466) original RR9/Hermite, [#4728](https://github.com/isogeny-labs/crypto/pull/4728) zeta-Walsh and four-window generic index calculus, [#4776](https://github.com/isogeny-labs/crypto/pull/4776) 70-minor exterior pair contraction, [#4772](https://github.com/isogeny-labs/crypto/pull/4772) Jacobian, [#4773](https://github.com/isogeny-labs/crypto/pull/4773) two-prefix reduction; [#4692](https://github.com/isogeny-labs/crypto/pull/4692) already owns **nonenumerative unknown-index extraction** and [#4779](https://github.com/isogeny-labs/crypto/pull/4779) already owns fixed **linear Plücker-span rank**. Our graph chart is *nonlinear* in 16 coordinates, so fixed-linear rank 70 would not contradict it.
- **Competitive controls and failed selectors**: [#4774](https://github.com/isogeny-labs/crypto/pull/4774) native same-runner signed shared-BSGS; [#4548](https://github.com/isogeny-labs/crypto/pull/4548) degree-5 Macaulay inconclusive 56-dimensional remainder. `crypto-autoresearcher` [SA-04 Riemann–Roch norm](https://github.com/isogeny-labs/crypto-autoresearcher/blob/d54c01d325b9e4508d51f56dc113833be27ddc9a/experiments/semaev-alternatives-20261105/SUMMARY.md) and SA-18 determinant method killed under their criteria. `cryptanalysis` [prime-field Semaev/IC reference](https://github.com/isogeny-labs/cryptanalysis/blob/6b64ee6ca07f92db014cf24d277439730ac31f4b/suite/src/cryptanalysis/ec_index_calculus.rs) also checked.
- **Literature**: Petit–Kosters–Messeng PKC 2016 DOI `10.1007/978-3-662-49387-8_1`; Amadori–Pintore–Sala FFA 2018 DOI `10.1016/j.ffa.2018.01.009`; prior hybrid elimination/symmetry work of Kudo et al. 2018. This is a bounded novelty search; shared research KB unavailable in the connector runtime. No original-theorem novelty or asymptotic-security claim is made.

## Next concrete experiment / strict promotion gates

Try a **rank-one graph update** rather than additional all-pair contraction. Replacing one signed leaf row `[a_j|b_j]` with `[a'_j|b'_j]` in an admissible half induces
```text
M' - M = (A')^{-1} e_j [b'_j - a'_j M],
```
a matrix of rank at most one. This exact identity follows by substituting `A'M = B + e_j(a'_j M-b'_j)` and solving; it is an *algebraic proposed lever*, not yet separately executed in source-bound B8. Multiple replacements could admit structured low-rank updates. Preregister a small fixed B4/B8 row-replacement identity panel with independent Python, and test whether a **nonenumerative** incidence search can recover unknown signed eight-leaf indices directly from public `R`. A faster exhaustive enumerator is **not** an acceptable replacement.

Use the **frozen 46-positive / 18-negative natural B8 targets**, no supplied factor indices, no EC group-sum tables, no planted chart coefficients, count all failures, root reconstruction, F4/linear-algebra work and memory. Only if public-only extraction succeeds should it replace the generic oracle in #4728 four-B8-window rank31/1024 complete Rust pipeline. Compare fully charged cold-one, 64 and 1024 verified log batches on one runner against optimized shared BSGS, strongest existing index-calculus and negation-map rho, then an ordinary cofactor-one growing prime ladder.

**Result:** Reproducible algebraic component speed/memory improvement on the unchanged p=2^31−1 curve. **No** cheaper complete genuinely non-generic ECDLP solver is established by this exhaustive B4 result. Independent signed PCC review is outstanding.
