# Prime31 run57 — Rank-aware two-point relation-query gate

**2026-11-22 UTC. Scope:** a reproducible *conditional random-query model* applied to the exact frozen run56 public factor bases. This is **not** measured blind Semaev/F4 extraction, a full ECDLP solver, or an empirical solver speedup. [pcc:unverified]

## Prior-work and provenance audit

- Same ordinary prime-order curve: `E/F_(2147483647): y²=x³+847514846x+2087444724`, `#E=2147446831`.
- Frozen run55/r56 exact public points, signed pair sums and modular collision/triple ranks: `research/prime31_f4_admission_20261121/RUN55_DICKSON2_YIELD_RESULT.md`, `RUN56_HYBRID_DICKSON_Y7_RANK.md`; independent replay ZIP `prime31_run56_hybrid_rank_20261121.zip` used locally. Original-Rust Dickson census source and green native CI are already on PR #4901.
- `docs/ACTIVE.md` and PR #4692 (selector SAT / RR9), #4812 (Horner/F3), #4822 (quotient/RR9 Krylov), #4809/#4846 (two-summand S3/rootfinding). Do not clone these extractors or mislabel 2-point generic search as non-generic attack.
- Published comparator: Amadori–Pintore–Sala, *Finite Fields and Their Applications* 51 (2018), 168–182, DOI [10.1016/j.ffa.2018.01.009](https://doi.org/10.1016/j.ffa.2018.01.009), specifically the **one-relation** alternative to building a full factor-base log database; Kudo–Yokota–Takahashi–Yasuda, *Acceleration of index calculus for solving ECDLP over prime fields and its limitation* (2018), a hybrid/symmetry approach with explicit limitations. No claim this two-point rank graph analysis is novel across all prior work. The repository research-KB MCP was unavailable to this runtime; scope is in-repo sources and available papers.

## Mathematical correction to run56's simplified sensitivity

Run56 correctly computed the exact two-point coverage and the initial free **homogeneous** relation ranks, but its `496` and `589` useful-hit requirements are *optimistic mathematical lower bounds*, not expected full-rank stopping times.

Let `L_i=log_G(P_i)`. Every successful *known-scalar* random relation query with public `[k]G=P_i^{s_i}+P_j^{s_j}` contributes one sparse affine row `s_i L_i+s_j L_j=k (mod n)`. The identity and collision relations are known homogeneous rows, computed before queries. A successful call need not add rank: reused variables form small components and many variables remain unseen. For two-point rows, rank is governed by a signed graph, and the existing homogeneous three-point/four-point rows further constrain its free component parameters.

We therefore sampled random **successful signed pairs**, independently and uniformly over the fixed unsigned point indices and `±` signs (rejecting opposite-sign self-pairs giving identity). This models an *idealized uniformly random positive relation* conditional on a hit; it is not an actual algebraic oracle. Each trial incrementally builds the exact rank of coefficient rows over `F_n` via signed disjoint sets (with anchored odd-sign cycles and double-point rows), projects the frozen independent homogeneous rows to remaining component variables, and tests their rank by modular Gaussian elimination. The stopping rule is **full rank**, not merely number of hits.

For each base: 256 deterministic Python Mersenne-Twister `random.Random` seeds `0x13579bdf + 7919*t`, fixed source basis from the archived run56, 8,000 positive-hit cap (all trials completed), binary search over frozen pairs; exact modular arithmetic, no floating-point rank tests. An independently coded *dense* Gauss–Jordan validator agreed with the signed-DSU plus projected-rank implementation on **500 random small-field-dimension test cases**. Full independent original-EC replay for all free relations is inherited from run56; this model *does not* replay individual target DLPs.

| Conditional rank-completion metric | Dickson 509 | Hybrid 677 |
|---|---:|---:|
| Initial free homogeneous rank | 13 | 88 |
| Naive minimum useful hits `b-r_free` | 496 | 589 |
| **Mean residual rank after those many sampled hits (256 trials)** | **79.05** | **93.88** |
| **Mean hits to actually reach full rank (256 trials)** | **1,732.17** | **2,292.89** |
| 5th / 95th percentile hits | 1,306 / 2,369 | 1,697 / 3,364 |
| Mean uniform known-scalar queries, `hits*n/(distinct_sums-1)` | **7.180 million** | **5.373 million** |
| Ratio versus prior rank-only bound | **3.49×** | **3.89×** |

Mean-hit Monte Carlo standard errors are approximately 20.11 (Dickson) and 29.36 (hybrid). The hybrid offers **about 25.17% fewer *conditional uniform calls*** than Dickson, not the earlier 32.87% optimistic rank-only projection. Consequently the mean **same-cost-per-query threshold** is **1.33636×** Dickson-only, rather than 1.48965×. A deterministic 10,000-resample bootstrap of the mean ratio gave an exploratory 95% interval about **[1.292, 1.383]**. This statistical interval characterizes only the simulated idealized query distribution; it is not a benchmark confidence interval or proof.

A simple independent check of the coupon-collector effect: at 496 Dickson positive rows, an idealized uniform-endpoint model leaves `509*(1-1/509)^(992) ≈72.36` unseen vertices on average; at 589 hybrid positive rows, `677*(1-1/677)^(1178) ≈118.67` unseen. Existing free equations may cover some unseen vertices; exact rank simulation measures the remaining deficiency.

**Crucial limits:** The signed-pair generator is an **idealized uniform successful-pair** model. Real Semaev/F4 extractors can bias the returned decomposition and may find multiple representations per target. The 190 hybrid nonidentity collisions are a tiny fraction of 916k supported targets; one query seldom yields multiple rows, but exact run-specific multiplicities and failure costs matter. Precomputing the pair census, anchor acquisition, relation recovery, F4 failures, target decomposition and logarithm verification are all **excluded**. A one-relation method or higher-arity/hybrid index calculus need not fully populate this two-summand factor-base rank and is not excluded by these results.

## Reproducibility / executable evidence

- `run57_basis.json`: canonical 13 Dickson and 88 hybrid independent free-row *span bases*, from frozen 78 and 190 nonidentity collision records + 56 cubic triples. Source of truth remains the run56 archive; the compact fixture is an exact span-preserving reduction, not a new arithmetic assertion. Its source hash is recorded.
- `run57_rank_gate.py`: signed-DSU + exact modular projected-rank trial with nonidentity support probabilities, seeded positive samples and failure distinction.
- `run57_validate.py`: independently authored dense full-matrix rank comparison (500 trials).
- `RUN57_RESULTS.json`: committed exact per-arm summary and seed/provenance; `run57_rank_gate.py` regenerates `run57_compact_results.json` with all 512 stopping times. Local full results SHA256 `1aabb3b531009d0c6a90f2b2142a142f302dc91c0a7af7b89d18e99bfced3889`. [pcc:unverified].
- Source-linked existing-Rust `research/prime31_ic_20261115/native/examples/prime31_rank_completion_gate.rs` is committed and now has its own source-triggered cached CI step; it draws a separate 128-trial SplitMix64 cohort. It reuses the original crate's `check_curve`/`Arithmetic`, not a replacement EC engine. The separate 128-trial Rust cohort is now source-linked CI PASS #45003247773; see verified results below.

## Independent source-linked native Rust verification (2026-11-22, PASS)

The **original** `prime31-algebraic-ic` crate, original `Arithmetic` / `PRIME31` and the small dedicated cached runner compiled and passed [GitHub Actions #45003247773](https://github.com/isogeny-labs/crypto/actions/runs/45003247773) (job 137149155743). The source-bound runner completed **500/500 independent Python dense rank tests**, a 14.41-second release build, the existing full 509-point Rust signed-pair yield/rank census, and a **separate 128-trial native Rust SplitMix64 rank-saturation simulation** with the same frozen 13/88 free-row basis. This is *not* merely a predicted CI result.

| Rust idealized conditional model (128 seeds) | Dickson 509 | Hybrid 677 |
|---|---:|---:|
| Mean successful pair rows to full rank | **1,738.375** | **2,285.656** |
| Mean rank deficiency after naive minimum | **79.555** | **93.742** |
| Mean uniform queries (model extrapolation) | **7,205,526** | **5,355,697** |

Native hybrid/Dickson query ratio **0.743276** (25.67% fewer modeled calls), same-setup hybrid oracle-cost threshold **1.345395×**. This corroborates the independent 256-seed Python model's **1.33636×** gate despite different RNGs and cohort sizes. Only the model implementation, deterministic exact modular arithmetic and source-bound execution are verified; actual S3/F4 costs and blind ECDLP logs remain unmeasured. No independent signed PCC.

## Priority changes / falsifiable next tests

1. **R57-1: measure rank gain, not raw hits.** In the current original-Rust S3 collector (PRs #4809/#4846), log `(branch, query source, signed pair, independent_row, cost_ms, field_ops, full_rank_deficiency)` for every positive and negative unknown-index extraction. Compare Dickson×Dickson, Dickson×coset, coset×coset on held-out known-scalar relation targets, while excluding the hidden scalar from the candidate input.
2. **R57-2: one-relation/graph-cycle stop rule.** Implement a *separate* verified individual-target solve gate that stops on an anchored target equation or a valid cycle without collecting a full log database, as in the Amadori–Pintore–Sala family. Count scalar multiplications for public target shifts and check `[k]G=Q` independently.
3. **R57-3: do not rank MQ7/MQ8 input charts by degree alone.** Run real public-only small W9/14-var Horner/4-var RR9 root extraction with strict timeout/MEM, and separate `unknown` from proven UNSAT. A 64-variable degree-two chart remains purely a correctness input until bounded F4 solves it.
4. **R57-4: complete cost gate.** Freeze source SHA, exact curve/base/targets and same-runner Rust cold and 32/128/1024 batch cases versus the best optimized negation-BSGS, rho and prior IC. Include ALL successful and unsuccessful probes, factor-base construction, homogeneous enumeration, scalar shifts, LA, individual logs, exceptional branches, and memory. Do not claim a cheaper complete non-generic prime-field ECDLP until successful blind logs and total cost comparison.

**Verdict at this scope:** run56's exact base/rank arithmetic stands, but the hoped-for **32.9% query advantage** was too generous for full-rank two-point collection. The rank-aware *conditional* advantage is ~25%, with a stricter 1.336× per-call threshold. No measured ECDLP speedup is established.
