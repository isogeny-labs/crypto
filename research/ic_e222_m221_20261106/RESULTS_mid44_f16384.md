# F = 16384 at a 44 bit subgroup: a factor-base cap of 16384 and a half-size table

Extends `RESULTS_mid44_f8192.md`. All figures [pcc:unverified], one run, producer-run.

## Changes

- `MAX_FB_SIZE` (`src/bin/icx/prime_external.rs`) and `PAIR_SUM_TABLE_MAX_FB` (`src/cryptanalysis/ec_index_calculus.rs`) are now 16384; the compact table packs 14-bit indices (the test checks the largest index).
- A first F = 16384 run was OOM-killed at 13.9 GB resident on the 16 GB machine: the table held both signs of every pair sum (`+S` and `-S` share x), 10.7 GB at this F, and the rest of the process the remainder. The table now keeps one slot per +/- pair and returns the first verified slot in insertion order, flipping both signs when its y is the other one; this is the entry the exact table (both signs, first insertion wins) returns. The table halves to about 5.4 GB at F = 16384.
- The counters keep the exact table's accounting (two insert attempts and the negations per sum), so the table's uncharged counts equal the earlier runs' (536,870,912 insert attempts at F = 16384 is 2 F^2); the implementation performs half as many inserts and no negations.
- Checks: the 52 `ec_index_calculus` library tests pass, including `compact_pair_sum_table_matches_the_exact_table` (same hit set and same decomposition as the exact table over every point of a tiny curve); E34 H1 at F = 128 still gives exactly 72,132,515 trials with the one-slot-per-pair table.

## Result (E44 H1, same instance as the F = 8192 row, F = 16384, 16386 relations)

| F | relations | trials observed | trials predicted | obs / pred | IC (GAE, w=0) | IC (GAE, w=1) | IC / rho, rho = 0.886 to 1.8 sqrt(n) | wall time | elimination |
|---|---|---|---|---|---|---|---|---|---|
| 4096 (other instance, `RESULTS_mid44.md`) | 4098 | 2.142B | 2.149B | 0.997 | 1.62e11 | 1.69e11 | 2.2e4 to 4.4e4 | 187 min | 0.6 s |
| 8192 (`RESULTS_mid44_f8192.md`) | 8194 | 1.063B | 1.074B | 0.990 | 8.05e10 | 8.42e10 | 1.1e4 to 2.2e4 | 86 min | 2.4 s |
| 16384 | 16386 | 0.536B | 0.537B | 0.999 | 4.08e10 | 4.44e10 | 5.4e3 to 1.1e4 | 49 min | 12.8 s |

- It recovered the planted logarithm and both matched-rho walks ended at it. The two rho runs (5.4e6 and 9.1e6 GAE) are the F = 8192 row's: same instance, same seeds.
- Observed over predicted trials 0.999, against a sampling spread of about 0.8% for 16386 relations. Doubling F from 8192 to 16384 halves the trials (1.063B to 0.536B) and the distance with it, as the model's n / F says.
- The matched rho is still cheaper at every weight (IC rises 8.8% at w = 1, the elimination and table terms growing with F); the distance is 5.4e3 to 1.1e4 times. The F = 4096 row is another instance, so its trial count and rho are not the same cell.
- Elimination now takes 12.8 s (about 5x for each doubling of F), still 0.4% of the run; memory in the run was 5.1 GB resident when sampled mid-run, not a recorded peak.

Limits: one instance, one run at this F; two rho runs; largest measured subgroup 44 bits against 218 to 220 native; F = 32768 would need the table's index width raised again (15 bits fit in 32 with the empty marker only if capped at 32767) and about 22 GB for the table plus the elimination at the current layout.
