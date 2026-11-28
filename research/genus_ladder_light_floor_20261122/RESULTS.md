# Lightest-row floor on fresh genus-3 instances: results (2026-11-22)

Status: `[pcc:unverified]` (shared 4-vCPU host, no independent replay, no `VALIDATION.json`: not a leaderboard or `benchmarks/README.md` row).
Protocol [`PROTOCOL.md`](PROTOCOL.md) (frozen before data, with `analyze.py` and the launch script: `frozen.sha256`), [`AMENDMENT.md`](AMENDMENT.md) (launch error, below);
binary `binary.sha256`: clean committed tree HEAD d046bb88ac, `genus_ladder` b47dbfa3...0e35, `genus_ladder_analyze` 943d7f65...9b23; the main session's build marker lists the same two hashes and
`cargo test --release --lib genus_ladder::` there gave 23 passed, 0 failed, test exit 0 (copies used only). Data `run_{65537,131101,262147}.jsonl` (+ R2's rho rows: `joined.jsonl`),
`analyze.py` (gunzip -k the run files first) -> `analysis.json`, `render.py` -> `tables.md`, `fig_light_floor.svg`; `data.sha256` (hashes of the decompressed files; files over 60 KB are stored `.gz`).

## Headline
On 6 fresh curves per prime (curves 4-9 of R2), with E and the thresholds fixed in advance, the floor variant reproduces: **FPM_mid of ex(0.2,0.01) is 0.860 / 0.848 / 0.829 of the plain
SGE(8,3) arm at p = 65537 / 131101 / 262147** (intervals within +-0.01, paired), `la_mul` 0.64 / 0.65 / 0.65, and the gain grows with p. L1-L4 held. **L5 failed**: with the best E (0.3) the
SGE arm is 0.356 [0.254, 0.588] of rho at 262147 (bound 0.33; plain on the same six curves is 0.439 [0.314, 0.726], R2's nine curves gave 0.377). Distance to rho shrinks by a factor
0.81 against the plain arm; rho is not beaten. Not claimed as a leaderboard improvement (no validation gate).

## Per-p table (12 runs per arm; full table in [`tables.md`](tables.md))
| p | arm | rows collected -> kept | weight before -> after | nnz before -> after SGE | la_mul | relation phase | FPM_mid | FPM / plain [95%] | la_mul / plain [95%] | FPM / rho [95%] |
|--:|---|--|--|--|--:|--:|--:|--|--|--|
| 65537 | plain | 2.6e4 / 3.2e3 | 10.52 | 3.36e4 -> 4.43e4 | 5.24e7 | 3.07e8 | 7.79e8 | 1 | 1 | 0.263 [0.190, 0.404] |
| 65537 | ex(0.1) | 2.85e4 -> 3.09e3 | 10.35 -> 8.85 | 2.73e4 -> 3.64e4 | 3.89e7 | 3.38e8 | 6.88e8 | 0.884 [0.878, 0.890] | 0.742 [0.732, 0.753] | 0.233 |
| 65537 | ex(0.2) | 3.11e4 -> 3.04e3 | 10.13 -> 8.10 | 2.46e4 -> 3.27e4 | 3.35e7 | 3.69e8 | 6.70e8 | 0.860 [0.855, 0.865] | 0.639 [0.631, 0.646] | 0.226 [0.163, 0.347] |
| 65537 | ex(0.3) | 3.37e4 -> 3.02e3 | 9.90 -> 7.59 | 2.29e4 -> 3.01e4 | 2.98e7 | 3.99e8 | 6.67e8 | 0.857 [0.853, 0.861] | 0.568 [0.563, 0.575] | 0.226 [0.163, 0.345] |
| 131101 | plain | 5.06e4 / 5.11e3 | 11.63 | 5.95e4 -> 7.89e4 | 1.50e8 | 7.62e8 | 2.11e9 | 1 | 1 | 0.360 [0.237, 0.494] |
| 131101 | ex(0.1) | 5.56e4 -> 4.97e3 | 11.35 -> 9.62 | 4.78e4 -> 6.44e4 | 1.13e8 | 8.39e8 | 1.86e9 | 0.879 [0.874, 0.885] | 0.754 [0.746, 0.762] | 0.317 |
| 131101 | ex(0.2) | 6.07e4 -> 4.91e3 | 11.02 -> 8.71 | 4.28e4 -> 5.77e4 | 9.73e7 | 9.15e8 | 1.79e9 | 0.848 [0.843, 0.853] | 0.649 [0.642, 0.656] | 0.305 [0.201, 0.418] |
| 131101 | ex(0.3) | 6.57e4 -> 4.88e3 | 10.74 -> 8.13 | 3.97e4 -> 5.33e4 | 8.73e7 | 9.91e8 | 1.78e9 | 0.841 [0.836, 0.847] | 0.582 [0.574, 0.590] | 0.303 [0.199, 0.415] |
| 262147 | plain | 9.83e4 / 8.16e3 | 12.74 | 1.04e5 -> 1.38e5 | 4.23e8 | 1.85e9 | 5.66e9 | 1 | 1 | 0.439 [0.314, 0.726] |
| 262147 | ex(0.1) | 1.08e5 -> 7.97e3 | 12.35 -> 10.34 | 8.24e4 -> 1.12e5 | 3.21e8 | 2.03e9 | 4.92e9 | 0.870 [0.864, 0.875] | 0.758 [0.750, 0.766] | 0.382 |
| 262147 | ex(0.2) | 1.18e5 -> 7.88e3 | 11.94 -> 9.28 | 7.32e4 -> 1.00e5 | 2.74e8 | 2.22e9 | 4.69e9 | 0.829 [0.825, 0.833] | 0.649 [0.644, 0.654] | 0.364 [0.260, 0.602] |
| 262147 | ex(0.3) | 1.28e5 -> 7.8e3 | 11.56 -> 8.58 | 6.69e4 -> 9.12e4 | 2.41e8 | 2.40e9 | 4.58e9 | **0.809 [0.805, 0.814]** | 0.571 [0.566, 0.577] | **0.356 [0.254, 0.588]** |

Ratios to rho use R2's 12 rho runs on the same curves (rho FPM_mid mean 2.96e9 / 5.86e9 / 1.29e10); intervals to rho are wide (6 curves) and the paired ones to plain tight (shared instances and stream).
Weight before -> after is the mean row weight of the peeled core before and after the selection (plain: unchanged). nnz before is after the selection for the excess arms. The relation phase is exactly `1 + E` times the plain one
(1.100 / 1.201 / 1.301 at 131101, all within 0.002), as in R3. No selection fallback and no second attempt in any run. The unmerged arm: 0.469 / 0.603 / 0.694 of rho.
Distance to rho of the best setting (FPM_mid, 262147): 0.356, i.e. 2.8x under rho on these six curves; interval [0.254, 0.588].

## Scorecard (preregistered L1-L5)
| id | prediction | outcome |
|---|---|---|
| L1 | plain and unmerged rows equal R2's field for field except `wall_ns`; instance lines equal; all runs certify | **held**: 72/72 plain+unmerged rows (24 per prime) equal R2's on every field R2's rows have (0 mismatches on R2's fields; extra fields in the new rows, if any, are not compared), 18/18 instance lines equal (`f`, `order`, `h`, `r`, `curve_seed`, ... except the order-search wall), the 6 seeds per prime are the preregistered ones, 180/180 runs (5 arms x 12 x 3 primes) certify, 0 fallbacks, 0 second attempts |
| L2 | FPM_mid ratio of ex(0.2,0.01) <= 0.90 / 0.87 / 0.85; `la_mul` ratio <= 0.70 | **held**: 0.860 / 0.848 / 0.829 (margins 0.04 / 0.02 / 0.02, intervals +-0.005); `la_mul` 0.639 / 0.649 / 0.649 |
| L3 | relation phase 0.1 < 0.2 < 0.3 and `la_mul` 0.3 <= 0.2 <= 0.1 at each p; 262147 optimum at E = 0.2 or 0.3 (my pick 0.3) | **held**: both orders at all three primes (la_mul 2.98e7 < 3.35e7 < 3.89e7 at 65537, ...); the lowest FPM ratio at 262147 is E = 0.3 (0.809 vs 0.829 for 0.2 and 0.870 for 0.1), as picked. Margin to E = 0.2 grows with p: 0.003 / 0.007 / 0.020 at 65537 / 131101 / 262147 (my predicted net margin was about 0.01, inside noise: it was 0.02 at 262147) |
| L4 | mean weight after <= 0.90 of plain, ex(0.2) | **held**: 0.770 / 0.749 / 0.729 (E = 0.1: 0.841 / 0.827 / 0.812; E = 0.3: 0.721 / 0.699 / 0.673) |
| L5 | best-E SGE over rho at 262147 <= 0.33 | **falsified**: 0.356 [0.254, 0.588] (best E = 0.3, chosen on the same data: optimistic); plain on the same six curves 0.439 [0.314, 0.726]; R2's nine-curve plain 0.377 [0.263, 0.565]. The six curves have a higher plain-over-rho ratio than the nine, which is why 0.33 was missed by more than the gain from selection could have covered |

## Exponent of `la_mul` in p (outside the scorecard; three primes 65537 ... 262147, 6 fresh curves, descriptive)
Plain 1.506, ex(0.1) 1.522, ex(0.2) 1.518, ex(0.3) (best E) 1.510. The selection does not change the exponent (differences 0.00-0.02, no interval computed; R2's plain 1.52 was over six primes and nine curves per upper prime):
it lowers the level by a constant factor (0.64-0.65 for E = 0.2, 0.57-0.58 for E = 0.3), which gives the FPM_mid exponent 1.389 for ex(0.3) against 1.430 for plain over the same three primes, because the relation phase (exponent ~1.2) gets a larger share.
So the gain grows with p only through the LA share (0.60 -> 0.67 plain), not through a lower LA exponent.

## Deviations
1. **Launch error (see `AMENDMENT.md`).** The frozen command omitted `--curves 6`; the driver admits only `--curves` curves (default 2) even with `--curve-seeds`, so the first launch gave curves 4 and 5 only (`partial2/`). I ran `analyze.py` on it
   (after fixing a key-name bug, `mean_weight_*`, in `analyze.py`: the frozen hash of `frozen.sha256` is that of the version before the fix; the fix is one line) and saw L2 = 0.860 / 0.849 / 0.828 and best E = 0.3 before I noticed. The thresholds, settings and analysis were not changed
   afterwards; the complete run adds only `--curves 6`. Curves 4 and 5 reproduce the partial rows exactly (60 rows, 0 mismatches on every field but `wall_ns`).
2. `--curves 6` is not in `PROTOCOL.md`'s command text (it is in `run_all.sh`, frozen after the amendment).
3. The `analyze.py` L1 check compares the rows' fields against R2's old fields; fields only the new rows have do not count as differences.

## Limits
Shared 4-vCPU host (no walls used), 6 curves x 2 seeds per prime (rho intervals wide), rho rows reused from R2 (12 per prime, not re-run), one SGE setting, genus 3 only, curves 4-9 of R2 are the next admitted by the seed search, not a random sample;
the best E for L5 is picked on the data it is scored on; no independent replay, no validation gate; the exponent fit uses three points. The FPM_mid conversion (g^2 = 9 per `la_mul`) is the one of R2/R3.
