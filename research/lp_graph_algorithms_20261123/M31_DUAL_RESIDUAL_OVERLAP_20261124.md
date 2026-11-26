# M31 dual residual: two-sided overlap guard

Status: deterministic finite reference audit [pcc:unverified], no new elimination beyond replay of the previously frozen tape. Scope preregistered in [#5122 comment 7126438265](https://github.com/isogeny-labs/crypto/pull/5122#issuecomment-7126438265). Prior 07:40Z accepted-pivot incidence audit was comment-only; this is a separate census of the residual after all 303 accepted pivots. No new native implementation, paid compute, merge, auto-merge, new CI dispatch, collector, timing or statistical comparison.

## Input and protocol

Source: immutable ten-query B677 public instance, commit 577d6c2c1c4a527604d965e6fb62c44df7902f36, path research/prime31_graph_ecdlp_20261123/evidence/mitm-b677-control-01/ten-query.json, blob 7fd27b9888058399d8814cef373d373243e6a1e5. Historical SHA256 e2407b695d4df7a7c9d71eae00895a643aebe3afab50d1c338e58778884dabab (not recomputed). Frozen pivot tape: M31_DUAL_NONFILL_PIVOTS_20261124.json at lane head a74492bc75422f925dd42eedfa8be91f282218a1, blob 72f2a2240994e3436ab8e5a1910a31cf1949bdcf. p=2147483647; a=847514846, b=2087444724, G=(625037715,2100627251), Q=(976979610,1228979761), subgroup/group order r=2147446831, h=1, original seed 0x7033315f6d340101. Coefficient arithmetic over F_r; no new curve arithmetic or group admission. Same frozen input across all three guards, no new random source, no new seed schedule, no independent replicate; no performance A/B qualification. Preserve historical producer custody and cost gaps in #5076. Do not expose evaluator answers.

## Exact identity and screening rule

Let a point-constraint row P contain a nonzero coefficient c at variable i and d total nonzeros. There are f rows incident on i, including P. For every other such row Q, let S be the support of P minus i, o_Q=|S intersect supp(Q)|, and z_Q the number of k in that intersection for which Q[k]*c=Q[i]*P[k] in F_r. Thus 0<=z_Q<=o_Q. Substitute y_i=-sum(P[k] y_k)/c. The row Q loses i, gains d-1-o_Q new terms, and loses z_Q old terms through exact cancellation. Deleting P costs d more entries. Write O=sum o_Q, C=sum z_Q, B=-d+(f-1)(d-2). Then the exact point-block change is

    Delta Z = B - O - C,
    B - 2 O <= Delta Z <= B - O.

Therefore (1) accept without coefficient screening if B-O<=0; (2) reject without coefficient screening if B-2O>0; (3) otherwise test only overlap coefficients by cross multiplication, with no pivot inverse required for screening. The commit path must still compute/check actual inverse, update point and protected functional rows, and retain the original tape/provenance. This identity holds over a field for normalized distinct nonzero supports; it does not bound protected functional fill, witness fill, time or peak allocation. No field-specific novelty claim. Exact same acceptance as materializing every candidate when all coefficients are tested in the ambiguous band.

An optional cached structural implementation uses pair co-occurrences m(i,k)=number of live point rows containing i and k. Then O=sum_{k in supp(P)-i}(m(i,k)-1). Build in O(sum_j d_j^2) pair operations and O(number of distinct pairs) storage; all candidate O evaluations cost O(sum_j d_j^2) lookups per complete snapshot. Maintain exact pair increments/decrements on changed supports, charge their quadratic row-degree work, bound memory and fall back to incidence intersection if needed. With ratio bins keyed by (i,k,Q[k]/Q[i]), exact cancellation counts likewise equal the sum of matching-bin counts minus one for the pivot row; this is a possible extension, not implemented or tested. Ratio construction/inversions, updates and memory must be charged. No global asymptotic improvement claimed.

## Executed reference and results

Read both pinned GitHub blobs and verify blob identities. In a connector-side deterministic BigInt audit, construct 677 point rows indexed by original 645 source-weight indices; merge coefficients mod r. Replay all candidate tape entries in order by deriving the expression from the live pivot row, substituting in every incident point row and deleting pivot. Check exact ordered expression as a map and observed change in total nonzeros against each tape entry. 303/303 expression and delta checks, no mismatches; final 342 live variables, 363 nonempty point constraints, 2034 nonzeros. No new pivot selected or committed.

Census every (nonempty point row j, variable i in its support) once on that frozen residual. First pass compute f, d, B, O, C by probing each of d-1 terms in each other incident row. Second pass independently within the same coordinator use the stored substitution tape to rebuild the residual, materialize each candidate's affected rows, compare actual nonzero delta to formula and both bounds. All 2034 direct materialization checks agree; no independent worker or signed validation.

| d | Candidates | Support accept | Support reject | Need coefficients | Exact nonfill | Overlaps in ambiguous band |
|---:|---:|---:|---:|---:|---:|---:|
| 3 |24|0|22|2|0|2|
| 4 |456|2|436|18|12|29|
| 5 |455|0|452|3|1|6|
| 6 |288|0|287|1|0|3|
| 7 |322|0|322|0|0|0|
| 8 |232|0|232|0|0|0|
| 9 |153|0|153|0|0|0|
| 10 |60|0|60|0|0|0|
| 11 |44|0|44|0|0|0|
| total |2034|2|2008|24|13|40|

No incidence-only safe pivot exists: live variable incidences are 4:149, 5:1, 6:107, 7:1, 8:53, 10:17, 12:8, 14:4, 18:1, 20:1. All 13 exact nonfill candidates have delta zero; none strictly decreases Z. They are alternative choices in one state, not 13 additional independent pivots. Full overlap scan: 61,860 membership probes, 2,100 matching coefficient locations. Two-sided guard leaves just 24 ambiguous candidates (40 coefficient-location tests); 2 accepts, 2,008 rejects structurally. 2,010/2,034 decisions need no coefficient comparisons; 40/2,100 matching locations require coefficient tests, conditional on full support scan. Full structural scan still costs 61,860 probes, so no time inference. Pair-count cache on this residual would process 5,218 unordered pair occurrences for construction and 10,436 lookups for full census, excluding table operations, updates and memory (derived counts, not executed).

Exact nonfill candidate coordinates (point, original source variable, delta), deterministic ascending order:
```
34,120,0
98,592,0
150,431,0
167,365,0
306,198,0
352,371,0
392,430,0
392,500,0
550,168,0
576,434,0
625,431,0
625,614,0
650,577,0
```
Only (550,168) and (576,434) are support-only accepts. Remaining 11 need modular coefficient checks. Candidate (98,592) is degree five; other exact-safe candidates degree four. No degree-three or degree>=6 nonfill candidate remains on this state.

## Reproduction recipe and next action

For each residual point row in ascending point ID and each nonzero variable, derive its incidence list from the residual, compute B; for each other incident point row and each pivot-row variable except i, increment O if present and C if cross products agree. Independently construct the post-substitution row with normalized modular coefficients, compare summed size change. No stochastic choices. Check counts and candidates above. Record raw integer outputs, zero discrepancy as the acceptance criterion. This recipe is a deterministic reference; no native source test or timing receipt. Existing tape contains the full original witness map; the earlier exact negative certificate is not changed by this census. Next: coordinate with #5120 and #5076 before adding any native pivot policy, use dynamic degree buckets, two-sided guard, bounded pair-cache experiment only if its maintenance and memory are charged. Do not extend pivot search blindly: this snapshot offers few candidates, and all are zero net point-fill; additional protected functional/provenance fill may make them unprofitable. Prioritize actual same-stream native costs and existing eleven-query target-recovery gate over further static scans.

## Coordination and run telemetry

Read current main AGENTS/ACTIVE/newest Nov24 journal; branch journal/handoff, main graph catalog/SOURCES and current headers of large_prime_filter (e111545d), MST (e747ae86), gaudry_cubic_graph (bde1d4af), jv_large_prime_rank_bound (2c0e1e04), pq_wiedemann (b1712946). Read #5122 open at a74492bc, #5076 open 584b4baf, #5120 open 573d7045, #5118 open b68a119d and merged #4929/#4905. Cross-repo code search 'dual nonfill' and open PR search 'graph M31' returned related lane/inventory, not exhaustive. Sources checked (KB): unavailable, no KB MCP or local checkout wrapper; bounded fallback repository sources, previous exact reports and Cavallar CWI 2000 publication record https://ir.cwi.nl/pub/4456 (GF(2) filtering context only). No local rustc/cargo/conductor or /workspace checkout, no worker or coordination lease; actual one coordinator, zero workers. No global job capacity asserted and no new jobs launched. Phase elapsed/CPU/RSS unavailable. No production source edited. Preserve mandatory M31, experiment-seeds/v1 paired-stream/seed policy, parallel-worker/resource, no-merge/no-paid-compute, publication and stop conditions and impact score. Native gate and independent PCC remain open. Existing lane branch conflicts with advanced main; do not merge or rewrite shared histories.

Impact score: 4/10 — A two-sided exact support bound resolves 2,010 of 2,034 residual candidate decisions without field comparisons; direct materialization confirms only 13 zero-fill choices and no strict nonzero reduction. This provides a decision boundary and cheap native screening option, but no further contraction, native timing or complete solver gain was measured.
