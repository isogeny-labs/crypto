# M31 relation deduplication and partition multiplicity

2026-11-24. Fixed-record/source audit, [pcc:unverified]. Scope notice #5122 comment 7145249255. No native execution or production edits.

## Source and input

Producer #5097 revision d8efd1dcaaa8b5e6d285a4406e1cdae7718992e7, research/prime31_graph_ecdlp_20261123/native/mitm_four.rs blob 8095178e98ee8b26c40bf5450f25c108f76eecde (lines 163–182, 209–269), forty-cap-full-cost.json blob 0d53e2f787eafa7c422e369983d6b04ea275e859 under evidence/mitm-b677-control-01. Existing public M31 curve p=2147483647, a=847514846, b=2087444724, r=2147446831, h=1, G=(625037715,2100627251), Q=(976979610,1228979761). Historical seed 0x7033315f6d340101 unchanged; no new PRNG or collection. 509 small and 168 LP points, 677 total. The stored full-run receipt records recovery; this audit does not rerun or independently certify it.

At #5120 head 573d704563ed1375bd4cfe2fd8b771f46861c6a1, src/cryptanalysis/large_prime_filter.rs blob e111545df0d2f0684759a1fac7cb4c07d0af72e0 defines duplicate_key = (small,large,rhs), after normalization; MST blob e747ae865b31e09b772769bf73b9279671d23346 reuses it. This is a sound exact-equation duplicate key when the protected target column is included. Admission blob e4868c650d788697187279fb8f2bf55e1268a413 checks the target map and point equation. No proposed change to that key.

## Exact partition multiplicity (written derivation)

The producer enumerates each unordered pair of distinct point indices with all four sign choices once, then visits every signed pair as left and every matching right from the index, rejects shared indices, sorts four signed leaves and inserts into a per-query HashSet. For a valid fixed signed four-distinct-index decomposition there are exactly six ordered left/right splits (choose two of four for left). Conversely each accepted pre-dedup match maps to one such row. Thus after complete enumeration the number of successful pre-dedup insert attempts equals six times the output count. This is independent of equal pair-sum values: the index retains every pair ID separately.

An optional source-owner optimization: after disjointness, accept only left.j < right.i (each pair already has i<j). For sorted distinct indices i<j<k<l this admits exactly (i,j)|(k,l) with the original signs. Thus the same complete sorted output can be constructed with one insertion per signed row instead of six. This condition must NOT be applied to arbitrary repeated-index, incomplete pair-table, pruned, or early-stop semantics without a separate proof. Keep the final original point replay, public a,b, deterministic sorted output and all counters; preserve historical HashSet baseline. A Vec may replace the HashSet only with proved unique pair IDs and complete domain. The candidate does not remove complement additions or table lookup/bucket traversal; it merely avoids redundant candidate materialization, sorting/hashing. No change in asymptotic source complexity.

## Deterministic receipt inspection

Read all 707 retained rows in original order. Require four distinct increasing indices and signs in {-1,1}; verify each label maps to a retained query. For each row enumerate the six choices of two positions, test the ordered guard above, require exactly one. Separately compare distinct keys on the first 645 and all 707 rows: support-only, signed point coefficients, projectively normalized point coefficients, exact affine equation (append -b and -a), projectively normalized affine equation. Normalization divides by the first nonzero coefficient over F_r. No field point replay or collection here.

| Prefix | Rows | Distinct keys in each of five schemes | Split checks | Canonical splits |
|---|---:|---:|---:|---:|
| Ten queries | 645 | 645 | 3870 | 645 |
| Eleven queries | 707 | 707 | 4242 | 707 |

All 707 terms lists met the four-distinct sorted-sign condition; all 11 stored query coordinates distinct. Query row counts: 52,54,71,64,72,75,68,54,69,66,62. Deterministic JS connector-side metadata/count audit, not a Rust implementation or a new timing. The sixfold insertion count follows from the inspected producer loop and complete-output assumption, not a retained internal counter. In the eleven-query receipt, 3535 successful duplicate materializations could be avoided in this model, versus 10068344 recorded complement additions. Do not convert the count ratio into a timing estimate. The much larger table/source costs remain untouched.

## Deduplication correctness boundary

A point-only projective key is not a safe general global dedup key. If two normalized point vectors agree but their public query coefficients differ, subtracting the two valid affine equations may yield a nonzero target coefficient; discarding one based solely on support or points loses that information. If both target and RHS match after normalization, dropping one is span-preserving for the full affine system; retain a canonical original source representative and account for duplicates in the raw journal. An exact duplicate of the whole equation cannot increment rank. Same point support is even weaker: differing signs can be independent. The fact that all five keys happen to agree in cardinality on this fixed archive does not justify substituting them in general.

## Next gate and limitations

Existing #5097 source owner may evaluate this optional canonical-pair gate on identical frozen query inputs after source synchronization. Accept only exact ordered signed-row, public query, projected rank, original witness and target-outcome equality against the unchanged source, including collision buckets, all sign patterns, shared-index rejects, identity handling, cap/refusal semantics, and separately charged preprocessing, lookup, allocation/sort and replay. Falsify if any row disappears or changes order. Do not duplicate #5076 extension, #5120 native comparison or #5118 source ordering. Given the prior graph-phase budget and the small affected count, this is a low-priority cleanup, not a reason to delay the native integration.

Sources checked (KB): no KB MCP exposed; local wrapper/Conductor/Rust/checkout absent. Bounded fallback: AGENTS, ACTIVE, Nov24 session log, catalog/SOURCES, lane handoff and current #5122/#5120/#5076/#5078; #5097 pinned native source/receipt; cross-repo open duplicate-relation and canonical-pair searches (not absence proof). Classical pair-sum background: Horowitz–Sahni, Computing Partitions with Applications to the Knapsack Problem, DOI 10.1145/321812.321823; primary-author abstract/repository record checked, not a full-text novelty review. This is an elementary source-specific counting identity, no novelty claim.

One coordinator, zero independent workers; no local Rust/cargo, private lease/job inventory or native build. No new randomness, field experiment, paid compute, CI dispatch or merge; no phase elapsed/CPU/RSS measured. Preserve historical seed evidence; original binary/rejection-trace and independent replicate panel remain missing. Existing historical receipts are unqualified for general reproducible A/B or speedup under experiment-seeds/v1. No leaderboard changes. Preserve all mandatory seed, M31, parallel/resource, publication/no-merge/no-paid-compute, stop and impact instructions.

Impact score: 3/10 — Established an exact six-to-one successful-match materialization multiplicity on the archived source model and verified canonical partition uniqueness on all 707 retained rows; clarified safe affine deduplication. This is a small source-level improvement opportunity with no native timing or complete-cost gain established.
