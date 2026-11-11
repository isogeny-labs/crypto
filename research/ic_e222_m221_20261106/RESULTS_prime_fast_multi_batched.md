# Multi-target IC with a lock-step batched per-target phase (frozen evaluation)

Status: [pcc:unverified] producer's own measurement; not independently replayed; same machine, 1 thread, wall clock, toy sizes.

Change: `ic_multi_solve_ex(..., table = true, batched = true)`. After the shared factor-base solve, all targets walk in lock-step (R_t <- R_t + G for every active lane with one Montgomery inversion per round, the trick used by the tuned BSGS), each point is looked up in the pair table, and a lane retires when a relation using only determined logs is found; every log is verified `k G = Q_t`. The unbatched table mode and the sweep mode are unchanged.

Protocol: frozen in `results/pf_batched/frozen.json` (commit 82a6b30f96, pushed and checked on the remote before the evaluation), with the prediction and abandon rule: batched/table <= 0.7 at T = 10 000; batched/BSGS between 2x and 5x at T = 10 000; abandon if batched/table >= 1.0 at T = 10 000. c = 2.0 inherited from the table-mode freeze; evaluation curves 25, 29, 31 bits were not used for any tuning of this arm (the arm was explored only at 27 bits, `results/pf_tscale/`). Three arms on identical planted targets: batched, table (unbatched), tuned BSGS; T = 1000 and 10 000; 20 rows per arm, every one of the 60 arm-runs verified (all targets `k G = Q`), no failed run. Provenance in `provenance.txt`. Timed: factor base, pair table, collection, elimination, per-target searches, extraction, verification.

Results (medians of per-row ratios; ranges in brackets; total time on the same targets):

| T | rows | batched / table | batched / tuned BSGS | table / tuned BSGS |
|---|---|---|---|---|
| 1000 | 10 | 0.86 (0.73 to 1.09) | 6.9 (3.6 to 10.3) | 7.8 |
| 10 000 | 10 | 0.55 (0.45 to 0.63) | 4.9 (2.5 to 6.2) | 8.3 |

By size at T = 10 000: batched / BSGS is 2.7 at 25 bits, 5.3 at 29 bits, 6.1 at 31 bits; at T = 1000 it is 4.1, 7.2 and 9.6.

Prediction check: batched/table <= 0.7 at T = 10 000 holds (median 0.55, every row below 0.7). The batched/BSGS prediction (2x to 5x) holds at the median (4.9) and at 25 bits but is exceeded at 29 and 31 bits (5.3, 6.1): the gap grows with size, as before. The abandon rule was not triggered. At T = 1000 the gain is small (one of ten rows above 1.0, 1.09) because the shared setup (about 110 ms at 27 bits) dominates.

Finding: batching the per-target phase is a reproducible improvement of the multi-target IC (1.8x at T = 10 000; about 1.2x at T = 1000 and not reliable there). It narrows the distance to the tuned BSGS from about 8x to about 5x at the median, but the distance still grows with the field size (2.7x to 6.1x at T = 10 000 over 25 to 31 bits; 4.1x to 9.6x at T = 1000), so no crossover is in sight. The remaining floor is the per-target relation search (about 0.04 ms per target at 27 bits against BSGS's 0.012 ms at T = 10 000) and the shared setup.

T-scaling at 27 bits for the unbatched table mode (`results/pf_tscale/`): table-IC / tuned BSGS is 33x, 21x, 7.5x, 10.4x at T = 10, 100, 1000, 10 000 (4 rows each, all verified); my prediction that the ratio is smallest at small T was falsified, the minimum is near T = 1000.

Limits: toy sizes (25 to 31 bits), one thread, c not tuned per size, T in {1000, 10 000}, 2 rows at 31 bits, tuned BSGS not frozen with the same rigor, memory not priced (table arm's pair table is fb^2 entries against BSGS's sqrt(T n / 4)), no §5a band, not independently replayed, no `VALIDATION.json`, certificate file or bench record.
