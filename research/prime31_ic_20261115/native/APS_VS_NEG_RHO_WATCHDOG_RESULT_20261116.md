# Prospective prime31 cold APS versus completed batched-negation rho — measured 2026-11-16

**Standing:** `[pcc:unverified]`. **24/24 correct complete logs in each of four arms (96/96)** in a source-specific Rust 1.100.0 release GitHub Actions panel. APS beat both **implemented batched negation-rho** variants in aggregate. This is a specific measured comparison, **not** proof that these are the best obtainable rho implementations or a new non-generic ECDLP algorithm.

- GitHub Actions **[43909292156](https://github.com/isogeny-labs/crypto/actions/runs/43909292156)**, job `133446473796`, exact tested head `5af9b95d7aa229dcfaa246d53ad53ba2c5fae3d9`; focused Rust tests and both panel commands passed.
- Rust implementation: [prime31_aps_one_relation.rs](examples/prime31_aps_one_relation.rs), `--rho-watchdog-match`. Same existing `Arithmetic`, Mersenne field, joint-Jacobian public labelled generation, mixed-Jacobian group addition, batch normalization, exact modular scalar and existing optimized APS control were **reused**, not rewritten in another language.
- Prospective frozen design: [APS_VS_NEG_RHO_WATCHDOG_PROTOCOL_20261116.md](APS_VS_NEG_RHO_WATCHDOG_PROTOCOL_20261116.md), committed as `81aa275b7b2f1088924ae721e7b6232fdf56174c` before the source change.
- Entire original 24×4 emitted result, including each cold run, public target, caller-only scalar, source seeds, step counts, DP records, restart counts and collision answers: [APS_VS_NEG_RHO_WATCHDOG_EXACT_CI_20261116.json](APS_VS_NEG_RHO_WATCHDOG_EXACT_CI_20261116.json), committed as `6269da9f5047a8261e80dc560f55c937e0f73940`. GitHub-log timestamp wrappers were stripped; JSON formatting only changed.
- Same curve in all arms: `p=2147483647`, `E:y²=x³+847514846x+2087444724`, `n=2147446831` prime, cofactor one; same `PRIME31.g`. All 24 held-out public Q were derived from distinct verifier-only scalars using seed `0x52484f5f4e455732`. All calls include cold curve checking, jump and public-point construction, walk/group operations, DP lookup, useless collisions and scalar verification, including failed rounds where applicable.

## Frozen fully cold result

| Rust complete solver arm | Independently caller-verified | Aggregate cold wall (24 Q) | Relative to APS | Per-target APS wins |
| --- | ---: | ---: | ---: | ---: |
| **Optimized complete APS S3 index calculus**, joint-Jacobian labelled public source, [384,384,16384], packed x-index | **24/24** | **64.741649 ms** | 1.000× | — |
| Batched **negation-map rho**: 32 lanes, 4 DP bits, 16 r-adding jumps, 512-step stagnation escape | **24/24** | **125.261741 ms** | **1.93479× slower** | **20/24** |
| Batched negation rho: 64 lanes, 5 DP bits, otherwise same | **24/24** | **140.040979 ms** | **2.16307× slower** | **21/24** |
| 32-lane rho deterministic A/A repeat | **24/24** | **124.606229 ms** | — | — |

- Completed-workload APS/32-rho cold ratio **0.516851** (48.315% lower aggregate cold wall); APS/64-rho ratio **0.462305** (53.769% lower).
- 32-lane rho A/A aggregate drift **0.5261%**. Chronological arm ordering rotated through all four cyclic orders; no performance threshold or tuning was retroactively changed.
- 32-lane rho completed in **1,481,984** total charged walk additions across 24 targets; 64-lane in **1,802,944**. Stagnation watchdog triggered 617 and 708 times respectively (each restart also charged). Largest observed table 5,318 and 4,098 distinguished records.
- Independent report-side readback recomputed 4 aggregate sums from individual rows, 24 APS signed modular congruences `alpha+beta*k=0 (mod n)`, all 72 rho successful scalars against caller-only known answers, exact 24+24+24+24 success counts, 20/24 and 21/24 per-target APS wins; zero discrepancies. Rust additionally verifies each recovered `[k]G=Q` inside every solver and the caller repeats verification.
- The measured wall times are **producer development CI**, not isolated CPU experiments, signed external PCC, fastest known rho, or mature PKM/APS/CAS frontier. No risk assessment for full NIST curve sizes follows from one 31-bit instance.

## Crucial preliminary failure and repair — NOT silently erased

The exact earlier source **[Actions 43908360694](https://github.com/isogeny-labs/crypto/actions/runs/43908360694)**, `9c09b3ea76b68d1766b8aba5af960efe8d121402`, used the same negation-folded walk **without** a long-cycle escape watchdog. Although its CI process exited successfully, 32-lane rho completed **12/24**, 64-lane **20/24**, and the repeated 32-lane only **12/24**, all under 2m step cap. Pilot JSON: [APS_VS_OPT_NEG_RHO_PILOT_CI_20261116.json](APS_VS_OPT_NEG_RHO_PILOT_CI_20261116.json). Rho pilot totals (including censored calls) of 767.388ms and 325.781ms **cannot** be compared as fully complete solver costs. We froze the negative result, preregistered a 512-step no-*new*-distinguished-point restart, and used a **fresh** held-out panel to test the changed implementation.

The preliminary failure is consistent with fruitless cycles from negation folding and a small 16-jump table, a mechanism explicitly investigated in [rho_walk_leads_20261014/README.md](../../rho_walk_leads_20261014/README.md), and in the classical Wiener–Zuccherato work. The fixed watchdog completes each new target, but is a conservative fallback, not proof of an optimal map or of ideal collision distributions.

## Mathematical classification and previous-lead review

- **APS arm:** a correct complete published one-relation `S3`-based **index-calculus-derived** implementation. Its explicit pair-root search of finite supports is generic meet-in-the-middle equivalent; its faster CPU runtime is an **engineering** gain, not a new non-generic scaling law.
- **Generic rho:** canonical negation fold `P↦{P,-P}` with exact `a,b` labels, 16 independently public labelled jumps, 32- or 64-wide Jacobian batch inversions, x-based DP hash, exact collision-to-scalar modular recovery, explicit 1- and 2-step cycle repair, new-DP stagnation guard and full cold accounting. This is **a serious implemented optimized comparator** but not yet the fastest known: the repository's `src/cryptanalysis/ic_boundary.rs` separately has `rho.negation` with Wiener–Zuccherato look-ahead and cycle escape; it has **not** been run on these exact Q in a matched host/units panel.
- Previous negative axes reviewed rather than repeated: unequal APS partition alternatives (post-Jac 384 incumbent), 5.81% packed-index gain below prior 10% gate, original S3 index 1,956× slower than BSGS, anchored two-positive m6 F4 natural-support ceiling 3.7407%, F4 censored ordered m8, and B8 finite signed selected-root validator `given f` which has not selected `f` from `Q`.
- Literature: van Oorschot & Wiener, [CCS 1994](https://doi.org/10.1145/191177.191231); Wiener & Zuccherato, [SAC 1998](https://doi.org/10.1007/3-540-48892-8_15); [PKM 2016](https://doi.org/10.1007/978-3-662-49387-8_1); [APS FFA 2018](https://doi.org/10.1016/j.ffa.2018.01.009). Research-KB MCP and local wrapper were unavailable; review was repository GitHub evidence + publisher-primary fallback.

## Next executable experiments, ordered

1. **Stronger rho denominator:** run the library's existing tuned `rho.negation` with proper WZ look-ahead and cycle escape on exactly the same ordinary prime31 Q and host, and compare against new within-curve batch-optimized rho with look-ahead rather than the stagnation watchdog. Retain all 24 matched unknown-answer solves, A/A controls and full cold costs. Try existing best-of-two map only if measured cost per step remains charged; `research/rho_walk_leads_20261014/` warns that fewer steps can lose after batch arithmetic overhead.
2. **Independent reproduce:** on an isolated fixed-clock CPU, a fresh 48-Q panel and separately signed PCC checking of full logs, modular relations, cycle restart traces, source SHA and measurement conditions. Report confidence interval from target-level ratios rather than assuming perfect hardware.
3. **Compare index-calculus frontier:** same-host mature PKM/APS implementations and completed native F4/CAS, not just BSGS or one generic walk. Distinguish single cold target from multi-Q amortized relation setup.
4. **Highest-value non-generic mathematical lead:** construct **target-conditioned original-ideal-certified** symmetric eight-summand `S9` / `K/S5` coefficient elimination. Use the checked-in Rust #4228/#4349 rational-root/subresultant tools and exact B4/B8 finite-root validator; solve for a rational `f(T)` from public Q **without enumerating all B31 48,903,492 multisets**, then complete unknown B31 logarithms with real full costs. On oversize/censored F4, report an uncompleted oracle, not a success. This directly addresses the more ambitious cryptanalytic objective.

**No paid compute, PR merge, history rewrite, weaker CI gate, source-independent PCC promotion or leaderboard modification.**
