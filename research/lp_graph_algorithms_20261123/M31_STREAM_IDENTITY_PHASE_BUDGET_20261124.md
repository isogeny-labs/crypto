# M31 archived stream identity and graph phase budget

2026-11-24. Scope notice #5122 comment 7138069577. Read-only audit of existing native receipts, not a new scientific run or benchmark. [pcc:unverified]

## Inputs and identity

Producer revision d8efd1dcaaa8b5e6d285a4406e1cdae7718992e7 (#5097), path research/prime31_graph_ecdlp_20261123/evidence/mitm-b677-control-01/:
- ten-query.json, Git blob 7fd27b9888058399d8814cef373d373243e6a1e5;
- forty-cap-full-cost.json, blob 0d53e2f787eafa7c422e369983d6b04ea275e859;
- repeat-mitm.json, blob 6d16be977ce05252c5b36f26b006e74f5a499f8c;
- matched-rho.json, blob 4efa684278f41fe5ac5ec77aa3ed3ed4fb4705ff;
- repeat-rho.json, blob a691fb5ab264798e29f5c0d73d04bc3b12b4a83e.
Native timer source research/prime31_graph_ecdlp_20261123/native/mitm_four.rs, blob 8095178e98ee8b26c40bf5450f25c108f76eecde. Compare prior PAIR_SOURCE_BOUND.md and PROTOCOL.md at the same revision. Current audit-lane parent e54f44d8d57116dcd7afd111f7735b70f9990973.

M31 p=2147483647, a=847514846, b=2087444724, r=2147446831, h=1, G=(625037715,2100627251), Q=(976979610,1228979761). Historical producer seed 0x7033315f6d340101 unchanged. No new random source, experiment, timing or replicate. Exact Git blob identities were retrieved; no independent SHA-256 recomputation of the raw bytes in this connector audit.

Parse all three source receipts as JSON and compare curve, public target, fixture_certificate, base_size, lp_pool_size, pair_entries, distinct_pair_sums, seed, and ordered (label,a,b,query) tuples. Compare each stored relation row (label and ordered signed term list) against the same ordinal in the longer receipt; compare all 707 rows in the repeat. Compare lp_rank, projected_rows, factor_base_target_rank at every shared query boundary. Equality is exact, no tolerance; no point arithmetic or native solver replay executed. This is reproducible as a receipt comparison, not an independent producer validation.

All metadata checks agree. All 10/10 shared queries and 645/645 ordered prefix rows agree, all 11/11 full/repeat queries and 707/707 full/repeat rows agree, and all shared rank metadata agree. Labels 0 through 10 contain 52,54,71,64,72,75,68,54,69,66,62 rows. The final 62 rows constitute the archived extension. The support/fixture metadata agree, including the archived Y7 CSV hash; the actual CSV and binary were not independently reconstructed. [pcc:unverified]

This establishes exact stored prefix identity, not merely seed equality. It permits #5076 to reuse the original ten-query certificate/tape for the archived eleven-query continuation, subject to native group admission and actual source hashes. The repeated full run is same-stream repeatability, not an independent seed or a held-out workload.

## Phase arithmetic and decision

The native source starts the graph timer after relation sorting and stops it before per-query logging and final target_scalar/scalar replay. Its graph_and_replay_ms includes signed point replay, row assembly, JSON record construction and rank.add, not just graph filtering. Every phase is elapsed().as_millis(), truncated individually. Query generation, sorting, final recovery, setup outside the table timer and other overhead cannot be silently assigned to this phase.

| Retained full run | table_ms | sum source_ms | sum graph_and_replay_ms | cold_wall_ms |
|---|---:|---:|---:|---:|
| forty-cap-full-cost | 336 | 3006 | 13 | 3367 |
| repeat-mitm | 381 | 2806 | 13 | 3215 |

Reported table+source subtotals are 3342 and 3187 ms (99.26% and 99.13% of displayed cold time). With 11 truncated graph-phase records, their actual sum is strictly below 24 ms in each original run. Under the deliberately optimistic counterfactual that the entire timed graph+replay phase vanishes and all other original durations remain unchanged, Amdahl accounting T/(T-g) bounds speedup by 3367/(3367-24)=1.00718 and 3215/(3215-24)=1.00753 (rounded upward). These are conditional accounting ceilings, not rerun timings, universal graph bounds or permission to omit replay. See Amdahl, AFIPS 1967, DOI 10.1145/1465482.1465560.

There is also a fixed-prefix stage-cost floor stronger in scope than simply citing the full eleven-query count: the prior M31_TARGET_NONIDENTIFIABILITY_20261124 certificate rules out a target-only linear witness from ANY subset of the same first 645 signed rows. For the unchanged exhaustive-pair source proceeding through complete queries in archived order, a graph-only consumer cannot succeed before the eleventh query. Even granting a zero-cost eleventh query and zero-cost graph work, constructing 915304 pairs and scanning the first ten full queries charges 915304*(1+10)=10068344 group-addition-scale operations, 100.71 times the retained unfolded rho control's 99970 charged group additions. This is conditional on the previous unsigned negative certificate, unchanged source/order and full-query admission; does not bound changed source algorithms, skipped/reordered queries, other target information or a different stream. The previous report already gives the full eleven-query pair-source floor; do not count that old result as new.

## Seed and performance qualification

The retained exact prefix and repeat check resolves the stored stream identity requirement only. Missing: prospectively frozen independent replicate seed panel and complete domain-separated PRNG stream mapping for these historical producer runs, original source binary/rejection trace custody, isolated counterbalanced timing, fully charged Y7 support creation, tuned successful negation-map rho. The repeat's recorded same numeric seed is intentionally reused. No historical seed data fabricated; no new independent trial, uncertainty estimate or speedup claim. Archived cost comparison remains unqualified for general reproducible A/B under experiment-seeds/v1.

## Coordination and handoff

Read current AGENTS/ACTIVE, main and lane 2026-11-24 journal, catalog and SOURCES, current large_prime_filter, large_prime_filter_mst, gaudry_cubic_graph, jv_large_prime_rank_bound and pq_wiedemann, #4917/#4905 closed merged and #5122/#5076/#5120/#5097/#5118 current metadata, recent lane comments and bounded open cross-repo PR search. Research-KB MCP, local repo/cargo/rustc, Conductor and independent worker tools not exposed; bounded source/primary-literature fallback only. One coordinator, zero workers. No global outstanding job census or lease, no compute dispatch, paid compute or merge. Phase elapsed/CPU/RSS of this audit not measured. Existing active native lane #5120 has source/ledger reconciliation blocker and #5076 retains extension ownership. No duplicate producer, solver or production edits.

Decision: stop repeating static negative-matrix solver optimizations for the generic source. Keep them as native correctness fixtures; prioritize actual extension admission and the source bottleneck on already-owned lanes. No leaderboard changes. Preserve mandatory seed/M31/parallel/resource/publication/stop/impact instructions. No further user decision needed.

Impact score: 3/10 — Established exact archived ten-to-eleven query prefix identity and same-stream repeatability, and a rounding-aware phase-budget ceiling that makes further graph-only work low priority on this particular generic source. Evidence is an unsigned read-only receipt audit, not a new native test or general speedup.
