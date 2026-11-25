# M31 target-only dual: recursive and nonfill degree-three pivots

Status: deterministic exact coefficient audit on frozen public input, [pcc:unverified]. Scope reserved by [#5122 comment](https://github.com/isogeny-labs/crypto/pull/5122#issuecomment-7124183041). No new collector, randomness, native Rust execution, timing, paid compute, merge, or production edits. #5076 owns extension admission; #5120 owns native graph comparison; #5118 owns source-order repair.

## Input and acceptance

Identical ten-query B677 source `577d6c2c1c4a527604d965e6fb62c44df7902f36:research/prime31_graph_ecdlp_20261123/evidence/mitm-b677-control-01/ten-query.json`, blob 7fd27b9888058399d8814cef373d373243e6a1e5, historical SHA256 e2407b695d4df7a7c9d71eae00895a643aebe3afab50d1c338e58778884dabab (not recomputed). p=2147483647, a=847514846, b=2087444724, order/subgroup ell=2147446831, h=1, G=(625037715,2100627251), Q=(976979610,1228979761), historical seed 0x7033315f6d340101. Negative certificate M u=b from 094215cea340742fc7009905db8e26941af9cb8e, blob ba90579abfe8f2a9f0817790060316390d737aa8. No new random sources or statistical performance A/B; one fixed deterministic case, no independent sample. Prior admission/source cost and seed custody gaps unchanged. Source group identities not rerun.

Hypothesis: after recursive degree-one/two elimination, a degree-three pivot can further reduce residual dimension without increasing total coefficient nonzeros. Control: same recursion stopping at degree two. Candidate: degree at most three, accept only exact net coefficient-nonzero delta <=0. Protect both public query functionals a,b. Frozen pre-execution scope comment above, no timings. Accept only if exact lifted coefficient maps and negative certificate agree; refuse nonunit or failed replay.

## Exact transformation

For a point constraint with d terms, select a source-weight variable i with coefficient c_i !=0, write y_i=sum_{k!=i}(-c_k/c_i)y_k. Substitute in every other point constraint and both public query functionals. Delete pivot equation and variable; retain its sparse substitution on a tape. This establishes an injective kernel lift, one unit of rank offset, and preservation of target recovery. For any future residual witness reverse the tape; verify M^T y=0 and both original functionals. For a negative certificate d=T^T b=C^T u, the original u restricted to surviving point equations remains a certificate: substitution is linear and each eliminated equation maps to zero. This is an elementary written derivation over F_ell, not a formal proof.

If variable i occurs in f point constraints including pivot, each other constraint loses one entry and adds at most d-1. Thus delta Z <= -d+(f-1)(d-2). For d=3 and f<=4, nonincrease is guaranteed without pairwise support screening. For f>4, compute exact cancellations or refuse; this bound ignores protected functional and provenance costs, which must be counted separately. No guarantee of linear-time recursive contraction: dynamic support scans and coefficient updates need charging. A queue keyed by current degree and variable occurrence can remove full rescans; high-degree rejected candidates still require bounded screening and reconsideration after incidence changes. No asymptotic solver improvement follows.

## Executed deterministic reference

The connector-side BigInt reference used exact residues mod ell and Euclidean inverses. Initialize 677 sparse point equations in point-ID order, source weights in original 645-row order, and two separate protected rows from queries[label].a,b. Build incidence sets per source weight (excluding protected rows). Repeatedly scan d=1..cap, point ID ascending; at each degree-d equation visit its live variables sorted by (point-incidence size, original source ID). Compute expression and exact delta using copies of incident point rows, including removal of the pivot row; commit first delta<=0, restart the scan. Stop when no candidate passes. Each copy removes selected weight, then adds coefficient times expression with exact cancellation. Apply same substitution to protected rows but do not use them for acceptance. Count row scans, screened candidates, old-support-plus-expression visits including pivot support, accepted visits, and refusals. Record ordered pivot point, original variable, substitution terms and delta in [raw tapes](./M31_DUAL_NONFILL_PIVOTS_20261124.json) at b1af35f3822160bd6e57cf2d1458f4ab0b8d2c10.

| Metric | Recursive degree <=2 | Nonfill degree <=3 |
|---|---:|---:|
| Residual variables | 445 | 342 |
| Nonempty point constraints | 466 | 363 |
| Point coefficient nonzeros | 2042 | 2034 |
| Rank offset | 200 | 303 |
| Nonzero target / constant coefficients | 443 / 443 | 337 / 337 |
| Expanded source-lift nonzeros | 579 | 682 |
| Candidate screens | 200 | 1386 |
| Rejected fill | 0 | 1083 |
| Screened term visits | 3553 | 67538 |
| Accepted term visits | 3553 | 6142 |
| Point-row scans | 156997 | 331103 |

Raw original 645 variables/2580 nonzeros; the earlier one-pass signed compression had 448 variables/2050 nonzeros. The recursive control gains three extra pivots. The new degree-three candidate gains 103 further pivots (23.15% fewer residual variables vs recursive control) while reducing point nonzeros by eight (0.39%). Conditional on previous rank 645, residual point rank is 445 and 342 respectively, so no new useful target relation. Negative certificate holds for both without relying on that previous rank assertion.

Verification: build each original source weight's expression in residual variables by reverse tape, then independently substitute that expanded lift into all 677 original point constraints and both protected functionals; compare exact sparse maps to forward-reduced maps. 679/679 each, zero differences. Compute residual target coefficient independently as sum of original u_j times surviving point row j: 445/445 and 342/342 coordinate checks, zero differences. This is same-coordinator reference checking, not independent native/PCC validation. Both arms were evaluated only once, no elapsed/RSS observations.

## Decision and cost implications

The candidate is a useful structural option but not a qualified speedup. As a dimension-times-sparsity proxy for a hypothetical scalar sparse solver with iterations proportional to residual dimension, 342*2034/(445*2042) is about 0.766. This is not a Krylov iteration or time prediction; actual preconditioner, polynomial generator, transpose, reconstruction, protected rows, and retries matter. Screening term visits grew approximately 19x, expanded lift support grew 17.8%, and the full scans were expensive even in abstract counts. Do not integrate brute-force screening. First use f<=4 sufficient bound with dynamic degree buckets; cap or refuse costly exact screens and retain a charged fallback. Native comparison must charge every rejected screen, inversion, allocation, coefficient update and witness lift; use the same source/input identities, resource authorization and experiment-seeds/v1 protocol. No new timing or held-out data were obtained. Avoid duplicate native work; pass the tape/counts to existing owners.

## Coordination and handoff

Current AGENTS/ACTIVE/newest UTC journal, main graph catalog and SOURCES, original large_prime_filter, large_prime_filter_mst, gaudry_cubic_graph, jv_large_prime_rank_bound and pq_wiedemann source headers read. Open #5122 starting head 094215ce, #5076 584b4baf, #5120 573d7045, #5118 b68a119d, #5097 a3a55a5d. Cross-repo code search 'dual signed contraction singleton target' and open PR search 'graph M31' returned related lane and autoresearcher inventory #3461, no absence claim. Sources checked (KB): unavailable; no KB MCP or checkout wrapper, bounded repository fallback and Cavallar CWI 2000 publication record https://ir.cwi.nl/pub/4456 (GF(2) filtering context, not an odd-field theorem). Local command discovery: git present, no rustc/cargo/conductor or /workspace. No independent workers available; one coordinator, zero workers, elapsed/CPU/RSS unavailable. No job dispatch, no claim of global job capacity. Do not change frozen inputs or historical seeds; preserve M31, reproducible seeds/paired A/B, worker/resource, publication/merge and impact instructions. Next executable step: existing native owner review of cheap degree-three guard and exact tape check before deciding on a bounded native implementation. No collector rerun needed.

Impact score: 4/10 — Exact same-input reference replay establishes 103 extra rank-preserving nonfill pivots beyond recursive degree-two control, with all 679 coefficient/functional maps matching. The significant screening overhead and lack of native timings or independent validation prevent a speedup claim.
