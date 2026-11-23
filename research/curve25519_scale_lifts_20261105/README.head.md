# Scale-up S3: cofactor lifts (lead 2) and the shared lift system (lead D) at k = 24 and 28

Protocol: [PROTOCOL.md](PROTOCOL.md), written and frozen before any data in this directory (cells, seeds, arms, truncation rule and predictions S0 to S9 unchanged).
Scope of every statement: **the ten measured cells (five shapes at k = 24 and k = 28), m = 2, mode full, unit F4 elimination multiply-adds, one local host, trace off**, plus one k = 20 reproduction cell. F4 multiply-adds are native, unpriced work (AGENTS.md section 5a). No rho arm, no distance to rho, no cross-method statement, no leaderboard row, no benchmark row. k = 32 is refused by the instrument (tower engine needs p < 2^31), so k = 32 is not measured.

## Result in one paragraph

In all ten cells the per-target F4 cost, the solving degree and the matrix sizes of every arm equal their k = 20 values (the k-scaling ratios of multiply-adds per target are 1.00 to 1.013, Table C), so the k = 20 conclusions of leads 2 and D carry over to k = 24 and 28 at these cells: `lifts=1` costs 8.0 times `lifts=0` per target (calls and multiply-adds), the per-relation ratio at exact coverage `lifts=1 / lifts=0` is 0.63 to 1.18 where defined (one cell has no `lifts=0` coverage), and `lifts=2` costs 30.7 to 628 times `lifts=1` on the same targets with a solving degree 2 to 5 above `lifts=0`. No arm produced a hit in any k = 24 or k = 28 cell (0 hits in all 30 cell-arm runs and all 12 prefix re-runs, Table A), so observed yield is not measured; the expected number of hits at the exact coverage is 0.08 (lifts=0) and 0.63 (lifts=1) summed over the ten full runs. All ten predictions S0 to S9 hold (none of their falsifiers occurred). No improvement over the index-calculus baseline is claimed or measured: the study shows that the per-relation cost of `lifts=1` is about that of `lifts=0` and that `lifts=2` is dearer than both, at these cells. [pcc:unverified]

## Instrument and run record

- Frozen binary `/home/user/frozen-bin/curve25519_family_scale`, sha256 `d66b85b0dcb60c2d1b551a2e3ce19de4cc76e6897ce359eafce1bf579606ca90`. The driver `run_cells.sh` compared the hash before every run (no mismatch); it was compared again at the end (equal). No code change, no build.
- Driver `run_cells.sh` (resumable, one process at a time, `ulimit -v 3500000`, `timeout 1200`, cap 600 s per cell, `uptime` per run); raw `analysis/f4cost_scale.jsonl` (45 lines, 63 KB), one JSON per run, `analysis/status.txt`, `analysis/driver.log`. **All 45 runs finished: none failed, none was killed, all stderr files are empty.**
- Runs: 3 reproduction runs (k = 20), 30 runs (10 cells x 3 arms), 12 prefix re-runs (paired targets in the six cells where lifts=2 was capped).
- The box was shared with other agents; the 1-minute load average before the runs was 6 to 15 (column `load`), and the number of targets that lifts=2 completes inside 600 s depends on it, so the truncation points (Table A) are not results in themselves. Wall times are not compared. Local runs, not isolab-eks, as instructed.
- Seeds (fresh): `2026110500000 + 100 k + 50 [bits] + t`. Targets are the same sequence across the arms of a cell and across prefix re-runs.
- Equal base size: the five-field base signature (`columns`, covered points, all-coset points, projected points) is identical across the arms of each cell (`analysis/base_check.txt`, one distinct signature per cell). Base sizes differ between k = 24 and k = 28 and between k and the k = 20 cells (see Coverage).

## Reproduction (S0)

k = 20, isogeny t = 4, G = 0, seed 2026111204, lifts 0, 1, 2, 200 targets: 14 top-level fields and 5 `solver_extra` fields per arm were compared with lead D's JSONs (`analysis/repro_check.sh`): **all equal** (F4 multiply-adds 104 574 112 / 836 095 076 / 59 024 055 111 in total, hits 0 / 1 / 1, max solving degree 5 / 5 / 8, matrices identical). The binary reproduces lead D's counts exactly.

## Coverage and the expected number of hits

`cov` = `covered_points` for lifts=0 (subgroup points that are a sum of two base points) and `covered_projected_points` for lifts 1 and 2 (points R with some R + K such a sum); `r` is the subgroup order, `cov/r` the exact coverage; the all-coset count `covered_points_all_cosets / r` is shown as an upper bound. Expected hits over the 200 targets are `200 cov / r` (the coverage is tiny at k = 24 and nearly nil at k = 28):

| k | cell | columns | covered (lifts=0) | projected (lifts=1,2) | all cosets | r | expected hits lifts=0 | expected hits lifts=1 |
|--:|:--|--:|--:|--:|--:|--:|--:|--:|
| 24 | bits t=5 G=3 | 14 | 58 | 392 | 393 | 2097349 | 0.00553 | 0.0374 |
| 24 | bits t=6 G=3 | 33 | 284 | 2178 | 2179 | 2097349 | 0.0271 | 0.208 |
| 24 | isogeny t=4 G=0 | 8 | 10 | 128 | 129 | 2097349 | 0.000954 | 0.0122 |
| 24 | isogeny t=5 G=0 | 17 | 76 | 578 | 579 | 2097349 | 0.00725 | 0.0551 |
| 24 | isogeny t=6 G=0 | 39 | 364 | 3036 | 3043 | 2097349 | 0.0347 | 0.29 |
| 28 | bits t=5 G=3 | 16 | 58 | 512 | 513 | 33557809 | 0.000346 | 0.00305 |
| 28 | bits t=6 G=3 | 33 | 262 | 2178 | 2179 | 33557809 | 0.00156 | 0.013 |
| 28 | isogeny t=4 G=0 | 2 | 0 | 8 | 9 | 33557809 | 0 | 4.77e-05 |
| 28 | isogeny t=5 G=0 | 10 | 22 | 200 | 201 | 33557809 | 0.000131 | 0.00119 |
| 28 | isogeny t=6 G=0 | 35 | 286 | 2450 | 2451 | 33557809 | 0.0017 | 0.0146 |
