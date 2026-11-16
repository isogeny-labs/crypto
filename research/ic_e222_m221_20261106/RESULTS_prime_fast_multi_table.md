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

## Independent validation and required corrections (`results/pf_multi2/VALIDATOR_REPORT.md`)

Validated: the measurement. 42 000 of 42 000 logs (14 000 per arm) re-verified in independent Python code; mutation tests show the `k G = Q` guard stops every corrupted variant on both the table and the sweep path; the merge of origin/main changed no solver source; a clean rebuild reproduces the deterministic fields on 36 of 36 runs; the freeze commit predates every row (that it was on the remote before the run is not verifiable to the second). Same machine, not an independent-host audit.

Corrections that supersede the text above:
- "about 2 million" table entries at 32 bits is wrong: the table arm has **10 569 001 entries (169 MB), peaking at 244 MB**, against 36 MB for BSGS (2 million was the sweep arm's size).
- 27-bit sweep3/BSGS median is 8.5, not 8.6.
- "1.5x to 2.8x, every row below 1.0" is one pass: pooled table2/sweep3 medians over seven measurement sets are 0.50 to 0.65, so table2 is faster everywhere, but 1 of 92 row-level measurements reached 1.035; run-to-run variation per arm is about 10 to 15%. Both frozen c values sit at or next to their optimum in a c-grid probe at 28 and 31 bits.
- **The BSGS arm was untuned.** A BSGS with batched baby steps and a cheaper hash is 2.6x to 3.9x faster in the same session (validator probe, outside the frozen protocol), which puts the table arm **7.6x to 16.8x behind the tuned BSGS**, not 2.9x to 5.0x. The distance remaining to the best multi-target attack is therefore larger than reported above.
- The rows still carry `rho_*` columns from the mis-sized 1024-entry table; they must not be cited. No matched rho arm, §5a band, certificate file or bench record; not eligible for the leaderboard or `benchmarks/README.md`.
- Provenance: the BSGS binary hash in `provenance.txt` could not be matched (the git-ignored binary was relinked on 7 Nov); the code path is unchanged between the BSGS commit and the freeze.
