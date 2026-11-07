# Pair-sum table relation collector (local, unvalidated)

**Status:** unvalidated (pre-gate). Local unisolated host, solver stage timer; no replay certificate, validator report or matched-rho band. Not a leaderboard row.

**Mechanism.** The baseline relation trial samples R = aG + bQ and sweeps every factor-base point F_i, solving the Semaev S_3 quadratic by a modular square root, to find F_j with R = +-F_i +-F_j. That sweep costs O(fb) square roots per trial and is about 90% of a trial at fb 256. The table variant (`IC_SUM_TABLE=1`, `src/cryptanalysis/ec_index_calculus.rs`, `build_pair_sum_table`, `find_one_relation_table_with_rng`) builds once per solve a hash table of all points +-F_i +-F_j (i <= j) and tests each R with one lookup. The set of decomposable R is the same, so with the same RNG stream the sampled trials, the number of relations and the recovered logs agree (below). Construction of the table is charged to the relation stage (`relations_ms`). Default path unchanged.

**Design.** 30 fresh W instances (v3 indices 20 to 29 at three sizes, not used in any earlier panel), solver seeds 1, 2, extra = 2, fb 16 to 256, sweep vs table, serial, interleaved seeded order (`run_table.py`). 600 runs, all verified. Data: `results/table/`.

| fb | sweep ms | table ms | table / sweep | median trials sweep / table |
|---|---|---|---|---|
| 16 | 60.8 | 28.5 | 0.469 | 1170 / 1170 |
| 32 | 42.1 | 14.6 | 0.348 | 508 / 508 |
| 64 | 36.2 | 17.8 | 0.490 | 282 / 282 |
| 128 | 30.4 | 44.7 | 1.471 | 206 / 206 |
| 256 | 31.5 | 159.3 | 5.062 | 262 / 262 |

Best sweep: fb 128 (30.4 ms). Best table: fb 32 (14.6 ms). **Paired best-vs-best ratio 0.482, bootstrap 95% CI 0.425 to 0.543 (n = 60).** The trial counts are identical per seed, which is the equivalence check: the table does not change which relations are found, only what a trial costs. The table loses at fb >= 128 because its construction is O(fb^2) point additions.

**Caveats.** The best fb for each arm was chosen on this same panel, so a confirmation on a fresh set with frozen choices follows (next entry). Toy sizes (p up to 2^18); at cryptographic p the optimal fb and the table's memory (about 2 fb^2 entries) change, so nothing is claimed beyond this range. This compares against the already retuned sweep (extra = 2); against the shipped default the gains compose. Local host, not isolab-eks; no validation-gate evidence. Goal not met until independently checked.

## Confirmation on a third fresh set, settings frozen beforehand

30 further fresh W instances (v3 indices 30 to 39 at three sizes, never used in a panel), solver seeds 1, 2, 240 runs, **all verified**. Settings were frozen from the earlier panels before this run (`run_confirm.py`): A shipped default (sweep, extra 64, fb 8); B tuned baseline (sweep, extra 64, fb 256); C retuned sweep (extra 2, fb 128); D pair-sum table (extra 2, fb 32). Serial, interleaved seeded order, solver `total_ms` (table construction inside). Data: `results/confirm/`, `analysis.txt`.

| arm | median total ms | median trials |
|---|---|---|
| A shipped default | 688.7 | 17,546 |
| B tuned baseline | 38.9 | 326 |
| C retuned sweep | 29.9 | 197 |
| D pair-sum table | 16.3 | 567.5 |

| paired ratio | median | bootstrap 95% CI |
|---|---|---|
| D / B (table vs tuned baseline) | 0.411 | 0.392 to 0.459 |
| D / C (table vs retuned sweep) | 0.521 | 0.491 to 0.583 |
| C / B (retune alone vs tuned baseline) | 0.743 | 0.691 to 0.813 |
| D / A (table vs shipped default) | 0.023 | 0.022 to 0.025 |

D beat B in 60 of 60 pairs; per-size D/B medians 0.424, 0.409, 0.445, 0.443 for r bits 14 to 17 (no size trend within the range). The earlier exploratory estimates (best-vs-best 0.482 table vs retuned sweep, 0.83 retune vs tuned baseline) are consistent: the frozen-setting confirmation gives 0.521 and 0.743.

**What is established, and what is not.** Established locally, on three disjoint fresh instance sets: replacing the per-trial square-root sweep by a pair-sum table lookup, together with the lower relation surplus, solves the same prime-order toy instances in about 0.41 of the time of the baseline tuned over factor-base size (0.02 of the shipped default), with every log recovered, identical relations per seed, and the table construction counted. Not established: validation-gate evidence (replay certificate, validator report, matched-rho band at the 0 / 0.1 / 1 GAE band); an isolab-eks run; any behaviour at cryptographic p (the table needs about 2 fb^2 entries and an O(fb^2) build, and the number of relations needed grows with fb, so the optimum and the advantage move with p); any change in the exponent. Within this range the gap to Pollard rho is unchanged in kind and is not closed. This is a measured constant-factor improvement over the current implementation, not independently checked, and the goal is not met.
