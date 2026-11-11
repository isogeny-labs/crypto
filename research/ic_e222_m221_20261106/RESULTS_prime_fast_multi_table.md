# prime_fast multi-target IC: table-lookup collector vs sweep, against measured BSGS (frozen evaluation)

Status: [pcc:unverified] producer's own measurement; not yet independently replayed; same machine, 1 thread, toy sizes, wall clock.

Change: `ic_multi_solve(..., table = true)` uses the O(1) pair-table lookup collector (`collect_relations_table`, 2-term decompositions R = +-F_j +-F_k and 1-term rows) for both the shared factor-base solve and the per-target searches, with a larger factor base (c = 2) because the table makes each walk point O(1) instead of O(fb). The sweep arm uses 3-term decompositions (c = 1). Every returned log is verified `k G = Q_t`.

Protocol: frozen in `results/pf_multi2/frozen.json` (commit 671cec184f, pushed and checked on the remote before the run; abandon condition stated: table/sweep median >= 1.0). c = 2.0 was chosen from exploratory runs on the 26- and 30-bit curves (`explore_table_*.jsonl`); the evaluation curves (25, 27, 29, 32 bits; residues 3 and 1) are disjoint from them. T = 1000 planted targets per instance, identical across the three arms (sweep3, table2, BSGS). 14 instance-rows per arm, all 14 000 targets of every arm verified, no failed run. Provenance: `results/pf_multi2/provenance.txt`. Timed: factor base, pair table, collection, elimination, per-target searches, extraction, verification (planted targets outside the timed region).

Result (medians of per-row ratios; total time, same targets):

| bits | rows | table2 ms | table2 / sweep3 | table2 / BSGS | sweep3 / BSGS |
|---|---|---|---|---|---|
| 25 | 4 | 97 | 0.61 | 2.9 | 4.8 |
| 27 | 4 | 253 | 0.46 | 3.7 | 8.6 |
| 29 | 4 | 586 | 0.60 | 3.6 | 6.3 |
| 32 | 2 | 2486 | 0.62 | 5.0 | 8.1 |

Pooled table2 / sweep3: median 0.58, range 0.36 to 0.67 over the 14 rows (every row below 1.0): the table collector is 1.5x to 2.8x faster than the sweep multi-target IC, a reproducible improvement of the multi-target IC solver on fresh curves. It is an improvement over the project's own multi-target variant and, by the earlier result, over repeated single-target `ic_solve` (134x to 570x); it is not an improvement over BSGS: the measured multi-target BSGS is still 2.9x to 5.0x faster, a smaller gap than the sweep variant's 4.8x to 8.6x.

Limits: the table is fb^2 entries (about 2 million at 32 bits) so memory grows as fb^2 = c^2 p^(2/3), compared with BSGS's sqrt(T n / 4); no §5a pricing band; one thread; T = 1000 only; 2 rows at 32 bits; c not tuned per size; the BSGS arm is simple (no tuning) so its measured speed is a floor; not independently replayed (a validator has been requested); no `VALIDATION.json`, bench record, S3 checkpoint, null control or certificate file.
