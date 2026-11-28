# Lightest-row floor at genus 4 and 5: results (2026-11-22)

Status: `[pcc:unverified]` (shared 4-vCPU host, three concurrent processes, no independent replay, no `VALIDATION.json`: not a leaderboard or `benchmarks/README.md` row).
Protocol [`PROTOCOL.md`](PROTOCOL.md) (frozen before data with `analyze.py`, `run_all.sh`, `binary.sha256`: `frozen.sha256`). Binary: copies of the clean-tree build (HEAD d046bb88ac) in
`scratchpad/r5/`; sha256 `genus_ladder` b47dbfa3...0e35 and `genus_ladder_analyze` 943d7f65...9b23, both **matching** the build marker `BUILD_DONE3` (`binary.sha256`).
Data `g4_2053.jsonl g4_4099.jsonl g5_1031.jsonl` (+ the earlier studies' rho rows, joined in `joined.jsonl`), `analyze.py` -> `analysis.json`, `render.py` -> `tables.md`, `fig_light_floor_g45.svg`;
`data.sha256` holds hashes of the decompressed files (files over 60 KB are stored `.gz`).

## Headline
**The floor selection does not carry over.** At genus 4 it is worth 0 to 4% of FPM_mid and at genus 5 it costs 7 to 22%. Best E per cell (lowest paired FPM ratio): 0.1, 0.2, 0.1. FPM_mid of the best E over plain SGE(8,3):
**0.984 [0.958, 1.002] at g4 p = 2053, 0.963 [0.950, 0.971] at g4 p = 4099, 1.072 [1.063, 1.082] at g5 p = 1031**; `la_mul` ratios 0.88 / 0.80 / 0.99 at those E (0.81 / 0.76 / 0.99 at E = 0.3).
G1 held; G2, G3, G4, G5 all falsified. The premise that the LA share is higher at genus >= 4 is false in these cells: the plain arm's LA share (g^2 la_mul / FPM_mid) is **0.519, 0.596 and 0.225**
(genus 3 at 262147: 0.67), and the relation phase, which the excess multiplies by 1 + E, takes the rest. Not an improvement over the baseline except a 3.7% one at g4 p = 4099 (E = 0.1 or 0.2, interval excludes 1), which is unvalidated and not claimed.
The plain arm reproduces the earlier ratios to rho (0.0252 at 2053, 0.0198 at 4099).

## Per-cell table (full table with intervals in [`tables.md`](tables.md); means over 6 runs per arm at genus 4, 9 at genus 5)
| cell | arm | rows collected -> kept | weight before -> after | la_mul | relation phase | FPM_mid | FPM / plain [95%] | la_mul / plain [95%] | FPM / rho [95%] |
|---|---|--|--|--:|--:|--:|--|--|--|
| g4 2053 | unmerged | 1.02e3 / 598 | - | 4.21e6 | 2.20e7 | 8.93e7 | - | - | 0.0493 [0.0406, 0.0627] |
| g4 2053 | plain | 1.02e3 / 598 | 7.98 | 1.48e6 | 2.20e7 | 4.57e7 | 1 | 1 | 0.0252 [0.0209, 0.0322] |
| g4 2053 | ex(0.1) | 1.12e3 -> 586 | 7.96 -> 7.35 | 1.30e6 | 2.42e7 | 4.50e7 | **0.984 [0.958, 1.002]** | 0.878 [0.830, 0.915] | 0.0248 |
| g4 2053 | ex(0.2) | 1.22e3 -> 585 | 7.94 -> 7.05 | 1.22e6 | 2.63e7 | 4.58e7 | 1.001 [0.983, 1.012] | 0.821 [0.795, 0.847] | 0.0253 |
| g4 2053 | ex(0.3) | 1.32e3 -> 588 | 7.91 -> 6.82 | 1.20e6 | 2.84e7 | 4.77e7 | 1.043 [1.028, 1.060] | 0.812 [0.784, 0.853] | 0.0263 |
| g4 4099 | unmerged | 1.98e3 / 1004 | - | 1.18e7 | 5.11e7 | 2.40e8 | - | - | 0.0375 [0.0220, 0.0779] |
| g4 4099 | plain | 1.98e3 / 1004 | 9.01 | 4.73e6 | 5.11e7 | 1.27e8 | 1 | 1 | 0.0198 [0.0116, 0.0411] |
| g4 4099 | ex(0.1) | 2.18e3 -> 979 | 8.95 -> 8.18 | 4.11e6 | 5.64e7 | 1.22e8 | 0.963 [0.955, 0.975] | 0.869 [0.854, 0.891] | 0.0190 |
| g4 4099 | ex(0.2) | 2.38e3 -> 984 | 8.92 -> 7.78 | 3.79e6 | 6.14e7 | 1.22e8 | **0.963 [0.950, 0.971]** | 0.803 [0.785, 0.815] | 0.0190 [0.0112, 0.0396] |
| g4 4099 | ex(0.3) | 2.58e3 -> 982 | 8.87 -> 7.47 | 3.58e6 | 6.64e7 | 1.24e8 | 0.976 [0.960, 0.989] | 0.757 [0.736, 0.779] | 0.0193 |
| g5 1031 | unmerged | 530 / 514 | - | 3.11e6 | 7.26e7 | 1.50e8 | - | - | 0.0066 [0.0039, 0.0120] |
| g5 1031 | plain | 530 / 514 | 6.02 | 8.45e5 | 7.26e7 | 9.37e7 | 1 | 1 | 0.0041 [0.0024, 0.0075] |
| g5 1031 | ex(0.1) | 584 -> 509 | 6.02 -> 6.00 | 8.36e5 | 7.96e7 | 1.00e8 | **1.072 [1.063, 1.082]** | 0.990 [0.975, 1.005] | 0.0044 |
| g5 1031 | ex(0.2) | 637 -> 509 | 6.03 -> 6.00 | 8.35e5 | 8.63e7 | 1.07e8 | 1.143 [1.131, 1.157] | 0.989 [0.977, 1.004] | 0.0047 |
| g5 1031 | ex(0.3) | 690 -> 509 | 6.03 -> 6.00 | 8.35e5 | 9.33e7 | 1.14e8 | 1.219 [1.205, 1.231] | 0.989 [0.976, 1.003] | 0.0050 |

Ratios to rho use the earlier studies' rho rows (6 per cell; at g5 only curves 39 and 42 have rho rows, so all g5 ratios to rho use those two curves). Intervals to rho are wide (2 or 3 curves); paired intervals to plain come from 3 curves and are coarse, not tests.
Relation phase of the excess arms / plain: 1.10 / 1.20 / 1.29-1.30 in every cell. No selection fallback, no second attempt in any run.
Mechanism (interpretation from the table, not separately tested): the plain peeled core already has few rows beyond the columns (rows 598 vs 577 columns at 2053, 1004 vs 972 at 4099, 514 vs 497 at g5), so "keep cols + margin rows" removes only 2 to 2.5% of
the rows (1% at g5, where the weight falls from 6.02 to 6.00).

## Scorecard (as requested, frozen before data)
| id | prediction | outcome |
|---|---|---|
| G1 | plain and unmerged rows equal the earlier studies' for the same (curve, seed) on every shared field except `wall_ns`; instances equal; every run certifies | **held**: 12 + 12 + 18 = 42 plain/unmerged rows compared (0 mismatches on the earlier rows' fields), 9 instance lines equal (the `curve_seed` lists are the preregistered ones; earlier rows' fields equal except wall), 5 arms x (6 + 6 + 9) = 105 runs, all 105 certify, 0 fallbacks, 0 second attempts; the rho rows' `r`, `h`, `g`, `p` equal the new instances' |
| G2 | FPM_mid ratio of best E <= 0.80 at g4 4099 and g5 1031 | **falsified** at both: 0.963 [0.950, 0.971] (E = 0.2) and 1.072 [1.063, 1.082] (E = 0.1); g4 2053 (no bound): 0.984 [0.958, 1.002] |
| G3 | `la_mul` ratio (best E) <= 0.65 in every cell | **falsified** in all three: 0.878 / 0.803 / 0.990 (best-FPM E); the lowest `la_mul` ratio is at E = 0.3: 0.812 / 0.757 / 0.989, still above 0.65 |
| G4 | best E = 0.3 in every cell | **falsified**: 0.1 / 0.2 / 0.1 (E = 0.3 is the second-worst or worst in the FPM ranking at every cell, 1.043 / 0.976 / 1.219) |
| G5 | best arm over rho at g4 4099 <= 0.016 | **falsified**: best arm ex(0.2,0.01) 0.0190 [0.0112, 0.0396] (ex(0.1) 0.0190; plain 0.0198 [0.0116, 0.0411]; unmerged 0.0375); the paired gain over plain is 3.7% |
Plain + unmerged rows compared against earlier files: 2 x 21 = 42 (the table's 12 + 12 + 18 counts the same rows).

## Outside the scorecard
* LA share of the plain arm: g4 2053 **0.519**, g4 4099 **0.596**, g5 1031 **0.225** (genus 3 at 262147 in R4: 0.67). I predicted this before the run from the earlier files (PROTOCOL.md); the request's premise (larger share at genus >= 4) did not hold.
* Would E > 0.3 help? Not run. A model fitted to the three measured E (ln `la_mul` linear in E, relation phase (1 + E) x plain's) gives, in tables.md: at g4 4099 the curve is flat to rising (0.975 at 0.3, 0.985 at 0.4, 0.997 at 0.5, 1.03 at 0.7), at g4 2053 and at g5 it rises (la_mul does not fall further:
  slope -0.39 and -0.007 per unit E). So E > 0.3 would plausibly not help at any of the three cells; the plausible optimum is E about 0.1 at genus 4 and 0 at genus 5. Extrapolation, not a measurement.

## Deviations
1. `--curve-seeds` was used in all three cells so the earlier studies' curves are re-admitted without their seed searches (the request asked for the C2 config with `--seed-search 4000`, which is kept in the command but is not exercised by `--curve-seeds`); instance lines match the earlier ones (G1). The 4099 run was therefore not a 20-minute job (the three cells ran in 10 s, 18 s and 77 s of wall).
2. Three processes ran concurrently on the shared 4-vCPU host (1, 2, 1 threads); counts do not depend on it and walls are not used.
3. Rho rows were reused (matched by instance fields); `--arms R` was not run. At g5, curve 3 has no rho row (skipped at its cap in the earlier run), so ratios to rho at g5 cover two of the three curves.
4. The `.resume/RESUME.md` checkpoint, `benchrec` record, Conductor task and session log required by the repository's CLAUDE.md were not made by this run (no write scope outside this directory).

## Limits
Shared 4-vCPU host, 3 curves per cell (6 or 9 runs per arm, coarse intervals), rho intervals wide, one SGE setting, instances not fresh (the earlier studies' own), the best E is selected on the data it is scored on, no independent replay, no validation gate.
