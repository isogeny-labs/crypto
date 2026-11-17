# Preregistered compact-x-index APS experiment, before Rust edits

Date: 2026-11-16. [pcc:unverified], protocol only. This is a follow-up to the **measured** full-cold BSGS control in [APS_VS_BSGS_COLD_RESULT_20261116.md](APS_VS_BSGS_COLD_RESULT_20261116.md). No timing or speedup claimed here.

## Concrete changed mechanism

The existing Rust APS factorized-S3 one-relation solver makes a `HashMap<u64,Vec<usize>>` index over the third public partition, paying per-key `Vec` construction and allocation. The source already enforces **global x-coordinate uniqueness** across all three partitions; therefore, one `u64` abscissa maps to precisely **one** third-partition index. A `HashMap<u64,usize>` should be exact and may cut the per-query cold cost. This changes **only indexing**; field arithmetic, source points, candidate roots, signed relation and full scalar verification remain identical. It is an *engineering improvement of the existing published APS one-relation method*, still generic MITM-equivalent, **not** a new non-generic technique.

- Before choosing this approach, reviewed `AGENTS.md`, latest `docs/session-logs/2026-11-16.md`, [#4391](https://github.com/isogeny-labs/crypto/pull/4391), the previous [matched BSGS result](APS_VS_BSGS_COLD_RESULT_20261116.md), the negative post-Jacobian partition sweeps, #4383, #4373, #4228/#4349 and crypto-autoresearcher SA-02/SA-14b/SA-15b results. KB/Conductor unavailable in this GitHub connector environment; source-level audit only. Primary prior arts: [APS2018](https://doi.org/10.1016/j.ffa.2018.01.009), [PKM2016](https://doi.org/10.1007/978-3-662-49387-8_1). No novel algorithmic claim.

## Frozen test design

- Ordinary cofactor-one p=2^31-1: `E: y²=x³+847514846x+2087444724`, n=2147446831, G=(625037715,2100627251), same APS profile [384,384,16384], max six rounds.
- Caller-only deterministic 24 **fresh** nonzero Q from SplitMix64 seed `0x5849_4458_5046_3331`, exclude trivial k and duplicates. No known scalar passed into either solver.
- Four cold arms per Q, cyclically rotated (case mod 4): (0) existing `HashMap<u64,Vec<usize>>` APS, (1) new `HashMap<u64,usize>` APS, (2) same new APS A/A repeat, (3) independent batched Jacobian BSGS with a fresh baby table. Each arm rebuilds all its public sources/indexes, charges all failed rounds, records original signed candidate selection, and caller verifies exact k and [k]G=Q.
- Exact signed recovery transcript (solution indices/signs/alpha/beta) and visited S3 pair counts **must agree** between old/new, including rounds, failures and successes. A standalone Rust unit test asserts equality for a natural Q. If the compact map sees a duplicate x, error rather than silently overwrite.
- Predeclared **engineering admission**: all 96/96 cold solves correct and exact transcript agreement; new total cold ≤ 0.90 × old, ≥18/24 paired wins over old, new A/A repeat drift ≤5%. BSGS is priced in the same panel and reported **regardless of win/loss**; a narrower comparator must not be misrepresented as an optimized rho win. Source-specific GitHub Actions locked Rust 1.100 release; if CI fails, keep the error and repair explicitly.
- Preserve exact JSON per-case receipt with no dropped failures, and report aggregate cold ns, setup, algebraic ns, pair counts and pairwise comparisons. No paid compute, no leaderboard/PCC promotion or extrapolated complexity benefit.

## Separate mathematical high-priority lead

Still need actual **target-conditioned** original-ideal-certified B4/B8 and then B31 symmetric 3+3+2/S9 factor-base multiset polynomial extraction, without enumerating C(38,8) candidate multisets; reuse #4228/#4349 and existing B4/B8 signed/infinity validator. A cheap third-list index does not fulfill that non-generic scientific requirement.
