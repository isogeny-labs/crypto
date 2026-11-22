# Lightest-row floor (`--excess E,0.01`) at genus 4 and 5 (frozen protocol, 2026-11-22)

R4 (`genus_ladder_light_floor_20261122`) gave FPM_mid 0.83-0.86 of the plain SGE(8,3) arm and `la_mul` 0.64-0.65 at genus 3. Question: does the
selection carry over to genus 4 (p = 2053, 4099) and genus 5 (p = 1031)? Measurement with an existing binary, no source change. Every figure `[pcc:unverified]`.

## Cells and instances (the earlier studies' own)
| cell | instances | earlier rows for the identity check | rho rows |
|---|---|---|---|
| g4, p = 2053 | curve seeds 40, 60, 109 (h 1, 2, 1) | `genus_ladder_cc_sweep_20261121/raw/g4_cc2.0.jsonl` | `.../raw/rho_g4.jsonl` (6 runs) |
| g4, p = 4099 | curve seeds 72, 247, 418 (h 1, 1, 2) | same file | same file (6 runs) |
| g5, p = 1031 | curve seeds 3, 39, 42 (h 1, 2, 8) | `genus_ladder_sge_depth_20261121/g5.jsonl` | `.../rho5.jsonl` (6 runs; **curve 3 has no rho rows**, skipped by the rho cap there) |
Rho rows' `r`, `h`, `g`, `p` equal the new instance lines' (checked for g4 before launch; checked in `analyze.py` for all), so `--arms R` is not run.
Seeds 1..2 (genus 4) and 1..3 (genus 5). Ratios to rho at g5 use only curves 39 and 42 (the curves with rho rows), for every arm.

## Run (`run_all.sh`; three concurrent processes on the 4-vCPU host)
```sh
A="--arms c --cc 2.0 --sge 8,3 --excess 0.1,0.01 --excess 0.2,0.01 --excess 0.3,0.01"
genus_ladder run --primes 2053 --genera 4 --curves 3 --curve-seeds 2053=40/60/109 --seeds 2 $A --max-h 2 --threads 1 --max-trials 5e7 --seed-search 4000 --out g4_2053.jsonl
genus_ladder run --primes 4099 --genera 4 --curves 3 --curve-seeds 4099=72/247/418 --seeds 2 $A --max-h 2 --threads 2 --max-trials 5e7 --seed-search 4000 --out g4_4099.jsonl
genus_ladder run --primes 1031 --genera 5 --curves 3 --curve-seeds 1031=3/39/42 --seeds 3 $A --max-h 8 --threads 1 --out g5_1031.jsonl
```
**Declared departure from the request:** `--curve-seeds` re-admits the earlier studies' curves without the seed search (which took 670 s at 2053, about 20 min at 4099
and 2187 s at genus 5 earlier); it is the same admission path, and the instance lines are checked field for field (G1). `--curves 3` is explicit. Binary: copies of
`genus_ladder` / `genus_ladder_analyze` built from clean HEAD d046bb88ac, hashes in `binary.sha256`, verified before use; only the copies are used.
Arms per (curve, seed): `C_double_lp+pre` (unmerged), `+sge(8,3)` (plain), `+sge(8,3)+ex(0.1,0.01)`, `+ex(0.2,0.01)`, `+ex(0.3,0.01)`.

## Estimands (fixed before data)
* FPM_mid = cantor_mul + 10 cantor_inv + rel_mul + g^2 la_mul (g^2 = 16 at genus 4, 25 at genus 5; rho: cantor_mul + 10 cantor_inv). Relation phase = cantor_mul + 10 cantor_inv + rel_mul.
* Paired ratio to the plain arm: mean over the (curve, seed) pairs of the per-pair ratio (6 pairs per genus-4 cell, 9 at genus 5); 95% percentile interval of 4000 hierarchical
  bootstrap draws (curves with replacement, then seeds within), seed 20261122. With 3 curves the interval is coarse and is not a test.
* Ratio to rho: mean FPM of the arm / mean FPM of the matching rho runs on the same curves; interval by the same bootstrap (rho resampled with its own curves).
* Best E per cell: the E with the lowest paired FPM ratio (point estimate), chosen after data from the three settings. "Best arm" for G5 = the arm with the lowest ratio to rho among all five.
* LA share of the plain arm = mean over runs of g^2 la_mul / FPM_mid. Extrapolation to E > 0.3 (outside the scorecard, not run): log mean `la_mul` linear in E over the three
  settings, relation phase = (1+E) x plain's times the measured factor; predicted FPM_mid over plain for E = 0.4, 0.5, 0.7, 1.0, reported as a model, not a measurement.

## Expected LA share and predictions (fixed before data)
The earlier plain SGE(8,3) rows give the LA share g^2 la_mul / FPM_mid: **0.52 at g4 p = 2053, 0.60 at g4 p = 4099, 0.23 at g5 p = 1031** (computed from the earlier files before
the run; genus 3 at 262147 was 0.67). So the premise "LA share higher at genus >= 4" does **not** hold in the earlier data: it is lower. With la_mul ratio about 0.6 and the
relation phase x(1+E), my expectation is FPM ratio about 0.86-0.88 at g4 4099 (G2 fails), and above 1 at g5 (the relation phase dominates; E = 0.1 least bad), so G2 fails there and G4 fails at g5.
The scorecard below is the one requested, kept as asked.

| id | prediction | falsified if |
|---|---|---|
| G1 | the plain and unmerged rows equal the earlier studies' rows for the same (curve, seed) on every field they share, except `wall_ns`; instance lines equal (except wall fields); every run of every arm certifies with the planted t | any mismatch or any non-certified run |
| G2 | FPM_mid of the best E over plain <= 0.80 at g4 p = 4099 and <= 0.80 at g5 p = 1031 | any of the two point estimates above 0.80 |
| G3 | `la_mul` ratio (best-FPM E) <= 0.65 in every cell (the E = 0.3 ratio is also reported) | any cell above 0.65 |
| G4 | the best E is 0.3 in every cell | any cell with another best E |
| G5 | best arm over rho at g4 p = 4099 <= 0.016 (plain SGE(8,3): 0.0198) | point estimate above 0.016 |
Failures are reported as failures.

## Limits (declared)
Shared 4-vCPU host with three concurrent processes (no walls used), 3 curves per cell and 6 (genus 4) or 9 (genus 5) runs per arm, rho rows reused (6 per cell; g5 curve 3 has none),
one SGE setting, instances not fresh (the earlier studies' own), the best E is selected on the data it is scored on, no independent replay, no validation gate.
