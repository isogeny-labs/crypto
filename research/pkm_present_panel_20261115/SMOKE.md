# pkm_present_panel smoke (not a measurement)

`[pcc:unverified]` instrument smoke: it shows that every presentation exports, solves, lifts and verifies,
and what the tool reports. It is one seed, one or two targets per cell, no controls, no interleaving, no
replay certificate, no matched rho. Nothing here is a speed or degree claim.

## What ran

* Cells: the seed-777 pilot cells of the m8 instrument, `pkm_m8_panel cell --t 2 --ratio 9 --master-seed 777`
  (p = 262153, B_real = 2) and `--t 3 --ratio 6` (p = 262153, B_real = 4), written under `smoke/`.
  Because the symmetric arms need `B_real >= m + 1` and the pilot cells give B_real = 2 (t = 2) and 4 (t = 3,
  so no symmetric arm at m = 4), a third cell `t3b` was made with the new wrapper
  `pkm_present_panel cell --t 3 --ratio 6 --seed 777 --require-breal 5 --slug smoke-t3b`: the rule stepped over
  p = 262153 and 262193 (B_real = 4 each, recorded) to p = 262217, B_real = 6.
* Targets: `pkm_m8_panel targets` (pilot cells) / `pkm_present_panel targets --planted 0 --planted-distinct 1`
  (t3b), one SAT and one UNSAT per (cell, m), seed 777.
* Driver: `bash research/pkm_present_panel_20261115/smoke.sh` (tables: `bash .../tables.sh`). Per arm:
  `export --presentation all`, `solve --deadline 100 --extract` under `timeout 120` with
  `RAYON_NUM_THREADS=1` (the engine honours it: `threads_used = 1`, cpu/wall about 1 in every row: 0.90 to 1.45, the high values only on sub-10 ms runs),
  max degree bound the non-binding default 64, `--extract-max-q` the default 4096 for every arm; then
  `msolve -t 1 -v 2 -P 1` under `timeout 120` with `/usr/bin/time -v`, and `verify-msolve` on its output.
  The F4 run and the extraction share the 100 s `--deadline`; every call stayed under 120 s.
* m = 5 monolithic was not run (S_6 cannot be expanded in semaev_higher). Nothing was raised past the caps.

## Columns

`constr target / cell` are field multiplications counted by `algebra.rs` (every `Pol::mul` / `Pol::scale`,
including the `p-1` scaling inside `Pol::sub`); the cell part is the target-independent work (S_{m+1} by
resultants split by power of x_R, its elementary-symmetric rewrite, X^(2^t) mod P, the R_j tower), shared by every
target of the cell and metered once per export process. `solve mults` is `f4_fp`'s `field_ops` of the solving run,
`extract mults` the second F4 run plus substitution tree, root finding and the lift (0 when not extracted:
UNSAT/inconsistent, or the run was censored). `D` = `solving_degree_max`; for a `timeout` row it is the highest
degree reached, a lower bound. `last-productive deg` = `solving_degree`. `verified / unlifted / fp sols` = real
decompositions verified by counted point additions / F_p solutions (x-multisets) that did not lift /
F_p-rational assignments; `matches classifier` compares with `real_count` (symmetric arms: `real_count_distinct`).
msolve columns: max F4 degree of its `-v 2` table, CPU seconds of `/usr/bin/time -v` (0.00 when killed).

## Reading, with the limits

* Prior study (`research/pkm_symmetric_tower_20261026`, m = 3, N = m t = 9, p = 786433, random targets): plain chain
  D = 6, plain one S_4 D = 12, symmetric D = 5. Here (t = 3, m = 3, other prime, SAT and UNSAT targets, all three
  cells identical): chain-tower D = 6, **mono-tower D = 12** (the prior "plain, one S_4" arm), sym-tower D = 5. So the
  prior D values are reproduced arm by arm. **mono-direct at m = 3 is not that arm and has D = 16** (no tower: the
  membership `x^8 = c` is a degree-8 equation next to a degree-12 S_4); the prior "worst arm, D = 12" is
  mono-tower. `sym-tower` equality with the prior study's system is a test (see the test list), on its p = 786433 cells.
* At m = 4, t = 3 the F4 runs of `chain-tower` and `mono-tower` hit the 100 s deadline on both targets
  (`censored = timeout`, D 7 and 32 are lower bounds, ops are lower bounds); msolve finished chain-tower and
  mono-tower on the UNSAT target (82 s and 64 s CPU) and timed out at 120 s on the SAT one.
  `tree-tower` finished (D 6, 2.3e9 mults) where `chain-tower` did not (3.5e10+): same variables and degrees,
  different box structure. `mono-direct` finished (D 32, 1.8e10 mults, 29 to 60 s).
* Symmetric arms: on the pilot cells they are skipped by `export --presentation all` (recorded reason in the
  manifest). On t3b they solve in milliseconds (D 8 at m = 4). The SAT target of t3b at m = 4 has
  `real_count = 6`, `real_count_distinct = 0` (every decomposition repeats an abscissa), so the symmetric arms
  correctly find nothing (inconsistent or 0 verified, `matches = true`); on the planted-distinct target `plantedD0`
  (`planted_abscissae` in `targets.json`) sym-direct, sym-tower, mono-direct and tree-tower each verify exactly the
  planted decomposition (`smoke/native_t3b_m4/plantedD0.*.solve.json`).
* **Semantic differences seen in the data.** (1) `tree-tower` at t3b m = 4, SAT target verifies 5 of the 6 real
  decompositions (`matches classifier = false`): the missing one is {-P,-P,-P,+P} (R = -2P); every pairing of the tree
  puts +P next to a -P (identity partial sum), while chain and mono find it. (2) mono arms admit extra F_p solutions
  that do not lift (`unlifted` > 0, e.g. 2 at t3b m = 4 mono-direct); chain/tree/sym showed none. (3) the chain/tower/mono arms count ordered tuples (`fp sols` 3, 6, 12, 40),
  the symmetric arms unordered ones (`fp sols` 1).
* t3b m = 4 SAT: `chain-direct` finished F4 (D 13) but its extraction ran into the shared deadline
  (`extraction censored`, no decompositions reported, so no `matches`); the UNSAT target finished.
* Not exact / caveats: the `construction` counter is `algebra.rs`'s and counts the `p-1` scaling inside
  `Pol::sub` as a multiplication (so it over-counts signs), additions are not counted, and cpu seconds are
  process CPU. `describe` and `.system.json` carry the same fields. The count of msolve's F4 degree is the
  largest `deg` of its round table, not an msolve "solving degree".


### cell t2, m = 3

| target | arm | nvars | neq | max deg | constr target / cell (mults) | solve mults | extract mults | D | last-productive deg | cpu_s | wall_s | censored | verified / unlifted / fp sols | matches classifier | msolve max F4 deg | msolve cpu_s | msolve status |
|---|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|---|---|---|--:|--:|---|
| unsat | chain-tower | 7 | 8 | 4 | 89 / 0 | 475495 | 0 | 5 | 2 | 0.01 | 0.01 | no | 0 / 0 / 0 | true | 5 | 0.00 | ok |
| unsat | tree-tower | 7 | 8 | 4 | 77 / 0 | 475495 | 0 | 5 | 2 | 0 | 0.01 | no | 0 / 0 / 0 | true | 5 | 0.00 | ok |
| unsat | mono-tower | 6 | 7 | 12 | 1794 / 2740 | 235513 | 0 | 12 | 2 | 0 | 0.01 | no | 0 / 0 / 0 | true | 12 | 0.00 | ok |
| unsat | chain-direct | 4 | 5 | 4 | 78 / 0 | 58493 | 0 | 7 | 3 | 0 | 0 | no | 0 / 0 / 0 | true | 7 | 0.00 | ok |
| unsat | mono-direct | 3 | 4 | 12 | 448 / 2740 | 109610 | 0 | 12 | 2 | 0 | 0 | no | 0 / 0 / 0 | true | 12 | 0.00 | ok |
| unsat | sym-direct | skipped (B_real < m+1: arm not applicable) | | | | | | | | | | | | | | | |
| unsat | sym-tower | skipped (B_real < m+1: arm not applicable) | | | | | | | | | | | | | | | |
| sat | chain-tower | 7 | 8 | 4 | 89 / 0 | 494326 | 497609 | 5 | 2 | 0.01 | 0.01 | no | 1 / 0 / 3 | true | 5 | 0.00 | ok |
| sat | tree-tower | 7 | 8 | 4 | 77 / 0 | 494326 | 497609 | 5 | 2 | 0.01 | 0.01 | no | 1 / 0 / 3 | true | 5 | 0.00 | ok |
| sat | mono-tower | 6 | 7 | 12 | 1794 / 2740 | 234481 | 236361 | 12 | 3 | 0 | 0.01 | no | 1 / 0 / 3 | true | 12 | 0.00 | ok |
| sat | chain-direct | 4 | 5 | 4 | 78 / 0 | 66215 | 67818 | 7 | 3 | 0 | 0 | no | 1 / 0 / 3 | true | 7 | 0.00 | ok |
| sat | mono-direct | 3 | 4 | 12 | 448 / 2740 | 109501 | 110610 | 12 | 3 | 0 | 0 | no | 1 / 0 / 3 | true | 12 | 0.00 | ok |
| sat | sym-direct | skipped (B_real < m+1: arm not applicable) | | | | | | | | | | | | | | | |
| sat | sym-tower | skipped (B_real < m+1: arm not applicable) | | | | | | | | | | | | | | | |

### cell t2, m = 4

| target | arm | nvars | neq | max deg | constr target / cell (mults) | solve mults | extract mults | D | last-productive deg | cpu_s | wall_s | censored | verified / unlifted / fp sols | matches classifier | msolve max F4 deg | msolve cpu_s | msolve status |
|---|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|---|---|---|--:|--:|---|
| unsat | chain-tower | 10 | 11 | 4 | 131 / 0 | 61152276 | 0 | 6 | 2 | 0.16 | 0.16 | no | 0 / 0 / 0 | true | 6 | 0.06 | ok |
| unsat | tree-tower | 10 | 11 | 4 | 115 / 0 | 4657314 | 0 | 6 | 4 | 0.03 | 0.04 | no | 0 / 0 / 0 | true | 6 | 0.02 | ok |
| unsat | mono-tower | 8 | 9 | 32 | 180174 / 2188176 | 17379486 | 0 | 32 | 2 | 0.12 | 0.12 | no | 0 / 0 / 0 | true | 32 | 0.02 | ok |
| unsat | chain-direct | 6 | 7 | 4 | 115 / 0 | 12473170 | 0 | 7 | 5 | 0.04 | 0.04 | no | 0 / 0 / 0 | true | 7 | 0.02 | ok |
| unsat | mono-direct | 4 | 5 | 32 | 54790 / 2188176 | 7677551 | 0 | 32 | 2 | 0.02 | 0.03 | no | 0 / 0 / 0 | true | 32 | 0.02 | ok |
| unsat | sym-direct | skipped (B_real < m+1: arm not applicable) | | | | | | | | | | | | | | | |
| unsat | sym-tower | skipped (B_real < m+1: arm not applicable) | | | | | | | | | | | | | | | |
| sat | chain-tower | 10 | 11 | 4 | 131 / 0 | 61973760 | 61982891 | 6 | 3 | 0.15 | 0.32 | no | 1 / 0 / 4 | true | 6 | 0.36 | ok |
| sat | tree-tower | 10 | 11 | 4 | 115 / 0 | 4758901 | 4765445 | 6 | 3 | 0.04 | 0.07 | no | 1 / 0 / 4 | true | 6 | 0.23 | ok |
| sat | mono-tower | 8 | 9 | 32 | 180174 / 2188176 | 17374784 | 17378812 | 32 | 3 | 0.12 | 0.23 | no | 1 / 0 / 4 | true | 32 | 4.82 | ok |
| sat | chain-direct | 6 | 7 | 4 | 115 / 0 | 12674025 | 12678097 | 7 | 3 | 0.05 | 0.1 | no | 1 / 0 / 4 | true | 7 | 0.12 | ok |
| sat | mono-direct | 4 | 5 | 32 | 54790 / 2188176 | 7677130 | 7679388 | 32 | 3 | 0.03 | 0.06 | no | 1 / 0 / 4 | true | 32 | 0.07 | ok |
| sat | sym-direct | skipped (B_real < m+1: arm not applicable) | | | | | | | | | | | | | | | |
| sat | sym-tower | skipped (B_real < m+1: arm not applicable) | | | | | | | | | | | | | | | |

### cell t3, m = 3

| target | arm | nvars | neq | max deg | constr target / cell (mults) | solve mults | extract mults | D | last-productive deg | cpu_s | wall_s | censored | verified / unlifted / fp sols | matches classifier | msolve max F4 deg | msolve cpu_s | msolve status |
|---|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|---|---|---|--:|--:|---|
| unsat | chain-tower | 10 | 11 | 4 | 95 / 0 | 237582601 | 0 | 6 | 3 | 0.53 | 0.55 | no | 0 / 0 / 0 | true | 6 | 0.12 | ok |
| unsat | tree-tower | 10 | 11 | 4 | 77 / 0 | 237582601 | 0 | 6 | 3 | 0.55 | 0.56 | no | 0 / 0 / 0 | true | 6 | 0.15 | ok |
| unsat | mono-tower | 9 | 10 | 12 | 1794 / 2740 | 14182349 | 0 | 12 | 5 | 0.06 | 0.06 | no | 0 / 0 / 0 | true | 12 | 0.04 | ok |
| unsat | chain-direct | 4 | 5 | 8 | 79 / 0 | 18971214 | 0 | 12 | 2 | 0.05 | 0.06 | no | 0 / 0 / 0 | true | 12 | 0.02 | ok |
| unsat | mono-direct | 3 | 4 | 12 | 449 / 2740 | 4203190 | 0 | 16 | 2 | 0.02 | 0.03 | no | 0 / 0 / 0 | true | 16 | 0.01 | ok |
| unsat | sym-direct | 3 | 4 | 6 | 128 / 6002 | 125168 | 0 | 7 | 2 | 0 | 0 | no | 0 / 0 / 0 | true | 7 | 0.00 | ok |
| unsat | sym-tower | 9 | 10 | 4 | 326 / 5992 | 222860 | 0 | 5 | 3 | 0 | 0.01 | no | 0 / 0 / 0 | true | 5 | 0.00 | ok |
| sat | chain-tower | 10 | 11 | 4 | 95 / 0 | 238609932 | 238618285 | 6 | 3 | 0.54 | 1.17 | no | 1 / 0 / 6 | true | 6 | 3.05 | ok |
| sat | tree-tower | 10 | 11 | 4 | 77 / 0 | 238609932 | 238618285 | 6 | 3 | 0.62 | 1.2 | no | 1 / 0 / 6 | true | 6 | 3.03 | ok |
| sat | mono-tower | 9 | 10 | 12 | 1794 / 2740 | 15018088 | 15024991 | 12 | 5 | 0.06 | 0.12 | no | 1 / 0 / 6 | true | 12 | 0.86 | ok |
| sat | chain-direct | 4 | 5 | 8 | 79 / 0 | 19560393 | 36339376 | 12 | 3 | 0.05 | 7.35 | no | 1 / 0 / 6 | true | 12 | 0.10 | ok |
| sat | mono-direct | 3 | 4 | 12 | 449 / 2740 | 4763681 | 4765100 | 16 | 3 | 0.03 | 0.05 | no | 1 / 0 / 6 | true | 16 | 0.11 | ok |
| sat | sym-direct | 3 | 4 | 6 | 128 / 6002 | 126568 | 127204 | 7 | 3 | 0 | 0 | no | 1 / 0 / 1 | true | 7 | 0.00 | ok |
| sat | sym-tower | 9 | 10 | 4 | 326 / 5992 | 225646 | 226918 | 5 | 3 | 0 | 0.01 | no | 1 / 0 / 1 | true | 5 | 0.00 | ok |

### cell t3, m = 4

| target | arm | nvars | neq | max deg | constr target / cell (mults) | solve mults | extract mults | D | last-productive deg | cpu_s | wall_s | censored | verified / unlifted / fp sols | matches classifier | msolve max F4 deg | msolve cpu_s | msolve status |
|---|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|---|---|---|--:|--:|---|
| unsat | chain-tower | 14 | 15 | 4 | 139 / 0 | 35128833437 | 0 | 7 | 6 | 99.41 | 100.45 | timeout | - | - | 7 | 82.09 | ok |
| unsat | tree-tower | 14 | 15 | 4 | 115 / 0 | 2340655171 | 0 | 6 | 3 | 5.38 | 5.48 | no | 0 / 0 / 0 | true | 6 | 2.54 | ok |
| unsat | mono-tower | 12 | 13 | 32 | 180174 / 2188176 | 39957128440 | 0 | 32 | 9 | 99.07 | 100.2 | timeout | - | - | 32 | 63.81 | ok |
| unsat | chain-direct | 6 | 7 | 8 | 116 / 0 | 1263036404 | 0 | 13 | 7 | 2.56 | 2.61 | no | 0 / 0 / 0 | true | 13 | 1.32 | ok |
| unsat | mono-direct | 4 | 5 | 32 | 54791 / 2188176 | 18082189159 | 0 | 32 | 2 | 28.52 | 28.93 | no | 0 / 0 / 0 | true | 32 | 11.75 | ok |
| unsat | sym-direct | skipped (B_real < m+1: arm not applicable) | | | | | | | | | | | | | | | |
| unsat | sym-tower | skipped (B_real < m+1: arm not applicable) | | | | | | | | | | | | | | | |
| sat | chain-tower | 14 | 15 | 4 | 139 / 0 | 39130636136 | 0 | 7 | 6 | 99.24 | 100.42 | timeout | - | - | 7 | 0.00 | timeout120 |
| sat | tree-tower | 14 | 15 | 4 | 115 / 0 | 2356460426 | 2356483332 | 6 | 3 | 5.6 | 11.21 | no | 1 / 0 / 12 | true | 6 | 59.41 | ok |
| sat | mono-tower | 12 | 13 | 32 | 180174 / 2188176 | 37740079729 | 0 | 32 | 9 | 98.94 | 100.22 | timeout | - | - | 32 | 0.00 | timeout120 |
| sat | chain-direct | 6 | 7 | 8 | 116 / 0 | 1276830341 | 1772569131 | 13 | 4 | 2.67 | 45.49 | no | 1 / 0 / 12 | true | 13 | 10.34 | ok |
| sat | mono-direct | 4 | 5 | 32 | 54791 / 2188176 | 18082063861 | 18082068423 | 32 | 3 | 29.69 | 60.17 | no | 1 / 0 / 12 | true | 32 | 72.92 | ok |
| sat | sym-direct | skipped (B_real < m+1: arm not applicable) | | | | | | | | | | | | | | | |
| sat | sym-tower | skipped (B_real < m+1: arm not applicable) | | | | | | | | | | | | | | | |

### cell t3b, m = 3

| target | arm | nvars | neq | max deg | constr target / cell (mults) | solve mults | extract mults | D | last-productive deg | cpu_s | wall_s | censored | verified / unlifted / fp sols | matches classifier | msolve max F4 deg | msolve cpu_s | msolve status |
|---|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|---|---|---|--:|--:|---|
| unsat | chain-tower | 10 | 11 | 4 | 95 / 0 | 237585276 | 0 | 6 | 3 | 0.6 | 0.63 | no | 0 / 0 / 0 | true | 6 | 0.16 | ok |
| unsat | tree-tower | 10 | 11 | 4 | 77 / 0 | 237585276 | 0 | 6 | 3 | 0.63 | 0.65 | no | 0 / 0 / 0 | true | 6 | 0.15 | ok |
| unsat | mono-tower | 9 | 10 | 12 | 1794 / 2740 | 14182548 | 0 | 12 | 5 | 0.06 | 0.06 | no | 0 / 0 / 0 | true | 12 | 0.02 | ok |
| unsat | chain-direct | 4 | 5 | 8 | 79 / 0 | 18970977 | 0 | 12 | 2 | 0.05 | 0.05 | no | 0 / 0 / 0 | true | 12 | 0.02 | ok |
| unsat | mono-direct | 3 | 4 | 12 | 449 / 2740 | 4203207 | 0 | 16 | 2 | 0.02 | 0.03 | no | 0 / 0 / 0 | true | 16 | 0.01 | ok |
| unsat | sym-direct | 3 | 4 | 6 | 128 / 6002 | 125168 | 0 | 7 | 2 | 0 | 0 | no | 0 / 0 / 0 | true | 7 | 0.00 | ok |
| unsat | sym-tower | 9 | 10 | 4 | 326 / 5992 | 222860 | 0 | 5 | 3 | 0 | 0.01 | no | 0 / 0 / 0 | true | 5 | 0.00 | ok |
| sat | chain-tower | 10 | 11 | 4 | 95 / 0 | 238128373 | 238129807 | 6 | 3 | 0.6 | 1.19 | no | 1 / 0 / 3 | true | 6 | 0.71 | ok |
| sat | tree-tower | 10 | 11 | 4 | 77 / 0 | 238128373 | 238129807 | 6 | 3 | 0.52 | 1.1 | no | 1 / 0 / 3 | true | 6 | 0.76 | ok |
| sat | mono-tower | 9 | 10 | 12 | 1794 / 2740 | 14484119 | 14485200 | 12 | 5 | 0.06 | 0.13 | no | 1 / 0 / 3 | true | 12 | 0.18 | ok |
| sat | chain-direct | 4 | 5 | 8 | 79 / 0 | 19233858 | 19235414 | 12 | 5 | 0.05 | 0.1 | no | 1 / 0 / 3 | true | 12 | 0.07 | ok |
| sat | mono-direct | 3 | 4 | 12 | 449 / 2740 | 4380056 | 4381259 | 16 | 3 | 0.02 | 0.05 | no | 1 / 0 / 3 | true | 16 | 0.04 | ok |
| sat | sym-direct | 3 | 4 | 6 | 128 / 6002 | 125168 | 0 | 7 | 2 | 0 | 0 | no | 0 / 0 / 0 | true | 7 | 0.00 | ok |
| sat | sym-tower | 9 | 10 | 4 | 326 / 5992 | 222860 | 0 | 5 | 3 | 0 | 0 | no | 0 / 0 / 0 | true | 5 | 0.00 | ok |

### cell t3b, m = 4

| target | arm | nvars | neq | max deg | constr target / cell (mults) | solve mults | extract mults | D | last-productive deg | cpu_s | wall_s | censored | verified / unlifted / fp sols | matches classifier | msolve max F4 deg | msolve cpu_s | msolve status |
|---|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|---|---|---|--:|--:|---|
| unsat | chain-tower | 14 | 15 | 4 | 139 / 0 | 35711047542 | 0 | 7 | 6 | 99.1 | 100.47 | timeout | - | - | 7 | 77.21 | ok |
| unsat | tree-tower | 14 | 15 | 4 | 115 / 0 | 2340641018 | 0 | 6 | 3 | 5.48 | 5.58 | no | 0 / 0 / 0 | true | 6 | 2.55 | ok |
| unsat | mono-tower | 12 | 13 | 32 | 180174 / 2188176 | 39590233582 | 0 | 32 | 9 | 99.37 | 100.24 | timeout | - | - | 32 | 63.21 | ok |
| unsat | chain-direct | 6 | 7 | 8 | 116 / 0 | 1263036646 | 0 | 13 | 7 | 2.52 | 2.58 | no | 0 / 0 / 0 | true | 13 | 1.23 | ok |
| unsat | mono-direct | 4 | 5 | 32 | 54791 / 2188176 | 18082161794 | 0 | 32 | 2 | 28.91 | 29.42 | no | 0 / 0 / 0 | true | 32 | 11.66 | ok |
| unsat | sym-direct | 4 | 5 | 8 | 3986 / 4627826 | 391184 | 0 | 8 | 5 | 0 | 0 | no | 0 / 0 / 0 | true | 8 | 0.00 | ok |
| unsat | sym-tower | 12 | 13 | 8 | 7915 / 4627924 | 400958 | 0 | 8 | 5 | 0.01 | 0.01 | no | 0 / 0 / 0 | true | 8 | 0.00 | ok |
| sat | chain-tower | 14 | 15 | 4 | 139 / 0 | 41692644913 | 0 | 7 | 6 | 99.28 | 100.44 | timeout | - | - | 7 | 0.00 | timeout120 |
| sat | tree-tower | 14 | 15 | 4 | 115 / 0 | 2229912167 | 2327815200 | 6 | 4 | 5.44 | 72 | no | 5 / 0 / 40 | false | 6 | 72.77 | ok |
| sat | mono-tower | 12 | 13 | 32 | 180174 / 2188176 | 37813062752 | 0 | 32 | 9 | 98.94 | 100.23 | timeout | - | - | 32 | 0.00 | timeout120 |
| sat | chain-direct | 6 | 7 | 8 | 116 / 0 | 1635224365 | 1954939583 | 13 | 6 | 3.49 | 100 | no | - | - | 13 | 22.69 | ok |
| sat | mono-direct | 4 | 5 | 32 | 54791 / 2188176 | 18076354119 | 18076401636 | 32 | 4 | 30.98 | 61.82 | no | 6 / 2 / 43 | true | 32 | 88.33 | ok |
| sat | sym-direct | 4 | 5 | 8 | 3986 / 4627826 | 391184 | 0 | 8 | 5 | 0 | 0 | no | 0 / 0 / 0 | true | 8 | 0.00 | ok |
| sat | sym-tower | 12 | 13 | 8 | 7915 / 4627924 | 400958 | 0 | 8 | 5 | 0.01 | 0.01 | no | 0 / 0 / 0 | true | 8 | 0.00 | ok |
