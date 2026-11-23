# Memory and time of the capped F4 runs at `t = 12, 13, 14`, and what cap 7 at `t = 14` needs

Design input for the bigger-memory attempt (owner instruction 2026-11-21). Nothing here was run on a pod; the only
computation is `project.py` in this folder, which re-derives every derived number below from committed files.
**Every number in this file is `[pcc:unverified]`**: it is a pod's own log or row, or arithmetic on one, and no pcc
claim exists. Labels: **measured** = read from a log or row (cited `file:line`); **derived** = arithmetic or a model
applied to measured values (each assumption stated).

Paths are relative to `research/pkm_falldegree_unblock_20261107/` unless they start with `research/` or `src/`.
`SNAP:<pod>:<member>:<line>` means a member of the committed archive
`results/SNAPSHOT_20261117/<pod>/results.tar.gz`, read without changing it. The `results/X*` and `results/Y*` trees are
git-excluded (`.git/info/exclude:43` and neighbours; raw data in S3 through the benchmark records named in
`RESULTS_T12_BRACKET.md` and `RESULTS_T13_BRACKET.md`); where a committed copy of the same cell exists it is cited
instead.

## 1. What the code does (read at the frozen commit `f13a690df0`)

Both files were read at `f13a690df016d0a457bd40579fbb9d054f486adf` with `git show f13a690df0:<path>`, the commit the
frozen binaries were built from (`host.txt` of every run: `"source_revision": "f13a690df016…"`). HEAD differs from it
in both files; HEAD was not used. `[pcc:unverified]`

- **What `--max-dense` bounds.** `examples/pkm_tower_pilot.rs:2105` parses it; `:1449-1450` passes it as
  `TowerF4Options::with_max_dense`. In `src/cryptanalysis/f4_fp_tower.rs` the option bounds "the reduced reducer
  block alone, in place of `max_nnz`" (`:917-921`). The step's `eliminate` call receives `opts.max_dense.or(opts.max_nnz)`
  (`:2698`). Inside `eliminate`, `B'` is the set of pivot (reducer and promoted) rows, each reduced to a dense row over
  the step's columns without a divisor (`Q`), stored from its lead's position to the right. Its planned size is
  `dense_entries = Σ over pivot leads of tail_len(lead)` (`:1577`), and `if max_dense.is_some_and(|cap| st.dense_entries
  > cap) { return Err(Stop::Oversize) }` (`:1578-1580`) runs **before any row of `B'` is allocated**. So the bound is
  per step and covers that step's whole `B'` (every pivot row, not one 64-row block, not the process total). It does
  not cover the basis, the residue echelon, the symbolic data or the product cache. `[pcc:unverified]`
- **The symbolic-phase bound is off in these runs.** `entry.sh`/`entry_r2.sh` pass only `--max-dense`, so
  `opts.max_nnz` is `None` and the nonzero check at `f4_fp_tower.rs:2575` never fires. The pilot prints the fixed
  string `"oversize: --max-nnz"` for every size stop (`pkm_tower_pilot.rs:1185-1188`), whichever limit fired. The
  t = 14 cap-7 stop at step 35 came from `:1578`: its stopped step carries `residual_cols 50256` and
  `dense_entries 33961233968`, which only `eliminate` fills in (report notes `:1115-1122`). `[pcc:unverified]`
- **Bytes per dense entry: 4.** Each `B'` row is kept as `Vec<u32>` (`:1679`, `bred[k] = acc.iter().map(|&x|
  fp.reduce(x) as u32).collect()`); the transient `u64` accumulators exist for one block of 64 rows at a time (`:1629`,
  `:1637`), and the S-row accumulators for `per_task` rows per task (`:1693`, `:1717`): tens of MB. The residue echelon
  keeps each pivot as a dense `u32` row from its lead on (`:1688`), at most `4·nq²/2` bytes. The option text says the
  same: "The block is held in memory, 4 bytes an entry" (`:918-919`). `[pcc:unverified]`
- **Memory lines.** `memory R MB (peak P MB)` is `VmRSS` and `VmHWM` of the process divided by 1024, so MiB
  (`pkm_tower_pilot.rs:1166-1172`), printed when a step ends or a stop cuts one off. The stop line adds `VmSize`/`VmPeak`
  (`:1191-1193`). The binaries are static musl builds (`build-info.json` `target x86_64-unknown-linux-musl`), so the
  glibc-only "live heap" figure is absent (`:1204-1213`). `[pcc:unverified]`
- **`timed_out` in the rows.** The pilot writes `timed_out: r.timed_out || r.oversize` (`pkm_tower_pilot.rs:1504-1506`,
  comment "A run stopped for size did not finish either"). A row with `oversize true` therefore always has
  `timed_out true`. The F4 deadline itself is `Instant::now() + budget`, set when the options are built
  (`f4_fp_tower.rs:1013-1014`, `pkm_tower_pilot.rs:1442`): wall clock from the F4 start. A deadline stop is
  distinguished in the log by the word `budget` in place of `oversize: --max-nnz` (`:1186-1189`). `[pcc:unverified]`
- **Other limits of the pilot that could bind at degree 7 or 8: none found.** Column numbers are `u32` with
  `NONE = u32::MAX` (`f4_fp_tower.rs:1142`); monomials are `u128` with a 64-bit mask of squarefree tower variables
  (`:184-189`, `:333-335`), and the `m = 2` ring has 28 tower variables and no free variable (`pkm_tower_pilot.rs:1147-1157`,
  `n_free: m - 2`). The product cache is capped at 2²³ terms (`f4_fp_tower.rs:2160-2171`) and only bounds itself.
  `--hky-max-cols` (2·10⁶) applies to the closure engine (`--engine hky`), not to `--engine tower`. `dense_entries` and
  the nonzero counters are `u64`. The number of columns cannot exceed the number of monomials of degree `≤ d` in 28
  squarefree variables: 499,178 (`d = 6`), 1,683,218 (`d = 7`), 4,791,323 (`d = 8`) (derived, `Σ C(28, k)`), far below
  `u32::MAX`. `[pcc:unverified]`

## 2. The runs (measured)

All rows: Kummer `m = 2`, `P1 = 2013265921`, random target, `RAYON_NUM_THREADS = 12` (`host.txt`), frozen binaries
(`pkm_tower_pilot` SHA-256 `f78f3959…` in `binary.sha256`), `--budget 86400`. "max `B'`" is the row's
`max_dense_entries`, which counts a step stopped for it (`pkm_tower_pilot.rs:1525-1527`). "asked" is the `B'` of the
step the limit stopped. Peak is the largest `peak` in the log. Wall is the row's `wall_s`. `[pcc:unverified]`

| `t` | draw | cap | `--max-dense` (request) | host CPU (`host.txt`) | degree reached | refutes | stop | max `B'` (asked) | peak MiB (log line) | wall s | evidence |
|--:|--:|--:|--:|:--|--:|:--|:--|--:|--:|--:|:--|
| 12 | 0 | 5 | 12.8e9 (56Gi) | Xeon Platinum 8488C | 5 | no | none | 53,833,099 | 356 (`:21`) | 10.8 | `results/X1/9f2460de-…/runs/X-m2-t12-k0-c5.log:21`, `.jsonl:1` (git-excluded) |
| 12 | 1 | 5 | 12.8e9 | Xeon Platinum 8488C | 5 | no | none | 53,833,099 | 356 (`:21`) | 11.8 | `results/X2/16cf635f-…/runs/X-m2-t12-k1-c5.log:21` (git-excluded) |
| 12 | 0 | 6 | 12.8e9 | Xeon Platinum 8488C | 6 | yes | none | 1,357,804,954 | 6,506 (`:27`) | 1,824.0 | `results/X3/b6596acd-…/runs/X-m2-t12-k0-c6.log:27`; committed copy (EPYC 9575F, wall 1,172.8, peak 6,502): `results/KUMMER/pfd-x-t13-k0-c6-rp/fetch-20261115T0720Z/runs/X-m2-t12-k0-c6.log:27` |
| 12 | 1 | 6 | 12.8e9 | Xeon Platinum 8488C | 6 | yes | none | 1,357,804,954 | 6,510 (`:27`) | 1,762.8 | `results/X4/6cb84cac-…/runs/X-m2-t12-k1-c6.log:27` (git-excluded) |
| 13 | 0 | 5 | 27.9e9 (112Gi) | Xeon Platinum 8375C | 5 | no | none | 53,833,099 | 356 (`:21`) | 16.8 | `results/Y1/9253a575-…/runs/X-m2-t13-k0-c5.log:21` (git-excluded) |
| 13 | 1 | 5 | 27.9e9 | EPYC 7R13 | 5 | no | none | 53,833,099 | 356 (`:21`) | 16.8 | `results/Y2/e7e4ad4c-…/runs/X-m2-t13-k1-c5.log:21` (git-excluded) |
| 13 | 0 | 6 | 12.8e9 | EPYC 9575F | 6 | yes | none | 4,717,115,340 | 20,616 (`:28`) | 12,371.8 | `results/RPX/pfd-x-t13-k0-c6-rp/runs/X-m2-t13-k0-c6.log:28`, `.jsonl:1`; second copy (EPYC 9575F, wall 7,907.7): `results/KUMMER/pfd-x-t13-k0-c6-rp/fetch-20261115T0720Z/runs/X-m2-t13-k0-c6.log:28` |
| 13 | 1 | 6 | 12.8e9 | Xeon 6975P-C | 6 | yes | none | 4,717,115,340 | 20,618 (`:28`) | 11,570.0 | `results/Y4/2f855278-…/runs/X-m2-t13-k1-c6.log:28` (git-excluded) |
| 14 | 0 | 5 | (earlier study) | not recorded | 5 | no | none | 53,833,099 | 390 (`:22`) | 18.6 | `research/prime_fall_degree_t14_20261101/runs/T-t14-k0-c5.log:22`, `.jsonl:1` |
| 14 | 1 | 5 | (earlier study) | not recorded | 5 | no | none | 53,833,099 | 393 (`:22`) | 18.3 | `research/prime_fall_degree_t14_20261101/runs/T-t14-k1-c5.log:22` |
| 14 | 0 | 6 | 12.8e9 | EPYC 9575F | 6 | no | oversize at step 30, degree 6 | 15,213,507,673 (asked) | 25,562 (`:28`) | 5,150.3 | `results/RPX/pfd-x-t14-k0-c6-rp/runs/X-m2-t14-k0-c6.log:30` |
| 14 | 1 | 6 | 12.8e9 | Threadripper 7960X | 6 | no | oversize at step 30, degree 6 | 15,213,507,673 (asked) | 25,566 (`:28`) | 5,039.3 | `results/RPX/pfd-x-t14-k1-c6-rp/runs/X-m2-t14-k1-c6.log:30` |
| 14 | 0 | 6 | 27.9e9 (`-m112`) | EPYC 9575F | 6 | no | none (degree 6 complete, pairs above cap) | 15,213,507,673 | 64,816 (`:30`) | 17,680.9 | `results/RPX/pfd-x-t14-k0-c6-m112-rp/runs/X-m2-t14-k0-c6.log:30`, `:34` |
| 14 | 1 | 6 | 27.9e9 (`-m112`) | EPYC 9575F | 6 | no | none | 15,213,507,673 | 64,816 (`:30`) | 22,471.2 | `results/RPX/pfd-x-t14-k1-c6-m112-rp/runs/X-m2-t14-k1-c6.log:30` |
| 14 | 0 | 7 | 12.8e9 (Amendment 30) | EPYC 9575F | 6 | no | oversize at step 30, degree 6 | 15,213,507,673 (asked) | 25,564 (`:28`) | 5,192.9 | `results/RPX/pfd-x-t14-k0-c7-rp2/runs/X-m2-t14-k0-c7.log:30` |
| 14 | 1 | 7 | 12.8e9 (Amendment 30) | Threadripper 7960X | 6 | no | oversize at step 30, degree 6 | 15,213,507,673 (asked) | 25,563 (`:28`) | 5,020.4 | `results/RPX/pfd-x-t14-k1-c7-rp2/runs/X-m2-t14-k1-c7.log:30` |
| 14 | 0 | 7 | 27.9e9 (`-m112`, Amendment 31 F) | Threadripper 7960X | 7 | no | **oversize at step 35, degree 7** | **33,961,233,968 (asked)** | 64,823 (`:30`) | 22,598.3 | `SNAP:pfd-x-t14-k0-c7-m112-rp:results/runs/X-m2-t14-k0-c7.log:35`, `.jsonl:1` |
| 14 | 1 | 7 | 27.9e9 (`-m112`, Amendment 31 F) | EPYC 9575F | 7 | no | **oversize at step 35, degree 7** | **33,961,233,968 (asked)** | 64,820 (`:30`) | 21,586.9 | `SNAP:pfd-x-t14-k1-c7-m112-rp:results/runs/X-m2-t14-k1-c7.log:35` |

The `t = 13` draw 0 Spot attempts (`results/Y6/…`, partial log, no row) are not used, as `RESULTS_T13_BRACKET.md`
section 2a requires. `[pcc:unverified]`

**The traces do not depend on the draw, and barely on `t`.** Within each `t`, both draws give identical step lines
(every count, `B'` and nonzero figure) through the last step; `t = 12, 13, 14` agree step for step through step 26
(`B' 785,810,929`), and `t = 13, 14` through step 27 (`2,155,440,769`). Only the wall time differs, with the host. So a
draw-1 run of any new size is expected to repeat draw 0 count for count (derived from the table; not a rule).
`[pcc:unverified]`

### 2a. The stop at step 35 (measured, `SNAP:pfd-x-t14-k0-c7-m112-rp:results/runs/X-m2-t14-k0-c7.log`)

- Line 34 (last complete step, degree 6): `fresh 192 (lowest degree 6)`, `pairs left 673466`, `memory 11669 MB (peak
  64823 MB)`. Degree 6 is complete: the run had no pair of degree `≤ 6` left. `[pcc:unverified]`
- Line 35: `tower stop in step 35 (oversize: --max-nnz): degree 7, 435158 critical + 238308 tower pairs, 673770 S-rows
  + 564787 reducers + 121105 promoted x 736148 cols, nnz 41203521491, residue 0 x 50256, … 622553.8 ms (rows 622513, A
  0, B 0, update 0), B' 33961233968 entries, … memory 11821 MB (peak 64823 MB), address space 11908 MB (peak 64924
  MB)`. The symbolic phase finished (622.5 s), then the `B'` check refused 33,961,233,968 entries against the cap
  27,900,000,000. That is 135.8·10⁹ bytes, 126.5 GiB of `B'` alone (derived). `[pcc:unverified]`
- The row (`.jsonl:1`): `degree_reached 7`, `inconsistent false`, `oversize true`, `timed_out true`, `max_dense_entries
  33961233968`, `max_nnz 41203521491`, `max_cols 736148`, `wall_s 22598.3`. With `wall_s` at 26 percent of the 86,400 s
  budget, `timed_out true` here comes from `oversize` through `pkm_tower_pilot.rs:1506`; the deadline did not fire.
  Draw 1 is the same with `wall_s 21586.9` (line 35 symbolic phase 559.7 s, EPYC 9575F). `[pcc:unverified]`
- Shape of step 35 (derived): 1,359,662 rows, 30,304 nonzeros per row on average (twice the 14,000 to 18,700 of the
  large degree-6 steps), `cols − nq = 685,892` = the pivot count, `B' / (pivots · nq) = 0.985`. The 736,148 columns are
  44 percent of the 1,683,218 monomials of degree `≤ 7`. `[pcc:unverified]`

## 3. Memory model (derived, calibrated on the logs)

`peak ≈ RSS before the step + 4·B' + 4·fresh·nq + c₀`: the basis and earlier state, the `B'` block, the residue
echelon, and a remainder `c₀`. On steps 27 to 30 of the cap-7 log (where the peak rose) the model gives 9,277, 25,425,
24,939 and 65,073 MiB against logged peaks 9,844, 25,562, 25,562 and 64,823 MiB: within 0.7 GiB. `c₀ = 1 GiB` is used
below. The RSS before step 35 is 11,669 MiB (11.4 GiB), almost all of it the degree-≤6 basis (68,773 elements with
2,983,392,035 entries, line 34). `[pcc:unverified]`

Consequence for the registered formula (derived): Amendment 7 A sets `4·C ≤ request − 8 GiB`. At degree 7 the basis
alone is 11.4 GiB, so a step just under the `-m112` cap would have needed about 103.9 + 11.4 = 115.3 GiB, above 112 GiB:
at `t = 14` degree 7 the 8 GiB headroom is too small, and a run that passed the check would have been an OOM under owner
decision 15 rather than a clean `oversize` stop. The design below uses a 36 GiB headroom. `[pcc:unverified]`

## 4. Projection: what cap 7 at `t = 14` needs to finish degree 7 (derived)

**Firm lower bound (measured).** Step 35 is deterministic (identical on both draws and two CPU models), so any run
with `--max-dense < 33,961,233,968` stops at step 35 again. Finishing degree 7 needs at least that, about 144 to 148 GiB
of process memory with the model above. A 128 GB pod (119.2 GiB of cgroup limit) cannot hold it. `[pcc:unverified]`

**How large the later degree-7 steps get: an analogue ladder.** After step 35, the run may find elements of lower degree
(a fall), process them, and return to degree 7 with new pairs, as it did at degree 6. The best evidence for how much the
largest `B'` at the deciding degree exceeds the first step that takes all pairs of that degree (step 23 at degree 6,
step 35 at degree 7) is the degree-6 history of the refuting runs, where `t` moves along the plateau of `c* = 6`:
`[pcc:unverified]`

| analogue | measured run | largest degree-6 `B'` ÷ step-23 `B'` | applied to step 35: degree-7 `B'` | process peak (model) |
|:--|:--|--:|--:|--:|
| `t = 10` (first `t` with `c* = 6`) | `research/pkm_tower_round4_20260926/runs/T2-kummer-m2-p1-N20.log:23-25` (an earlier pilot build; its row equals the frozen build's `research/prime_fall_degree_t14_20261101/runs/R-t9-t10.jsonl:3`): step 23 is the largest, step 24 (degree 4) reaches `1` | 1.000 | 33.96e9 | 144-148 GiB |
| `t = 11` | `research/prime_fall_degree_t14_20261101/runs/R-t11-k0-c6.jsonl:1` (`max_dense_entries 305556904`; no trace; step 23 assumed equal to `t ≥ 12`'s 289,092,933) | 1.057 | 35.90e9 | 155-168 GiB |
| `t = 12` | `results/KUMMER/…/X-m2-t12-k0-c6.log:23,27` | 4.697 | 159.5e9 | about 640 GiB |
| `t = 13` | `results/RPX/pfd-x-t13-k0-c6-rp/runs/X-m2-t13-k0-c6.log:23,28` | 16.32 | 554e9 | about 2.1 TiB |
| `t = 14`, degree 6 (did **not** refute) | `SNAP:…k0-c7…log:23,30` | 52.6 | (1.79e12, above the column ceiling) | — |

Under C-step (Amendment 6), `t = 14` is the first `t` with `c* = 7`, the position `t = 10` held for `c* = 6`. That makes
the `t = 10` row the C-step-consistent analogue, not a prediction of it. The band: `[pcc:unverified]`

| band | assumption (stated, not established) | degree-7 `B'` to finish | memory (model; RSS before the step 11.4-30 GiB, echelon up to 4.7-9.4 GiB, `c₀` 1 GiB) |
|:--|:--|--:|--:|
| **low** | degree 7 behaves like degree 6 at `t = 10`: step 35 is the largest step; the falls it yields reach `1` at lower degree | 33.96·10⁹ (ratio 1.00) | 144-148 GiB |
| **central** | like `t = 11`: one more all-pairs degree-7 step about 6 percent larger | 35.9·10⁹ (1.06) | 155-168 GiB |
| **high** | like `t = 12` | 159.5·10⁹ (4.70) | about 640 GiB |
| beyond | like `t = 13`; or degree 7 does not refute | 554·10⁹ (16.3); up to the ceiling below | 2.1 TiB; up to about 2.6 TiB |

**Column ceiling (derived, exact arithmetic).** A degree-7 step has at most 1,683,218 columns, so
`B' ≤ (cols − nq)·nq ≤ 1,683,218² / 4 = 7.08·10¹¹` entries (2.58 TiB of `B'`). This bounds every degree-7 step whatever
happens. `[pcc:unverified]`

**If degree 7 does not refute (the `d_F ≥ 8` reading).** Completing a non-refuting degree has needed 52.6× (degree 6 at
`t = 14`) and 86× (degree 5) the first all-pairs step; at degree 7 both exceed the ceiling, so the requirement is
between the `t = 13`-like 5.5·10¹¹ and the ceiling 7.08·10¹¹ entries: 2.0 to 2.6 TiB of `B'`. Degree 8 would then start with
an all-pairs step whose `B'` grew 463× from degree 5 to 6 and 117.5× from 6 to 7; with a growth of 30× to 117× the first
degree-8 step alone needs 1.0·10¹² to 4.0·10¹² entries (3.6 to 15 TiB), against a ceiling of `4,791,323² / 4 = 5.74·10¹²`.
No RunPod CPU pod (largest 256 GB, `HOSTS.md`) comes within a factor of 8 of either. `[pcc:unverified]`

**Consequence for the reading.** With any host available on RunPod, the attempt can end in `d_F = 7` (refutes) or in a
censored stop; the outcome "completes degree 7 without refuting, `d_F ≥ 8`" is out of reach by a factor of about 9 or more in
memory. A censored outcome at the new size does not distinguish `d_F = 7` with `t = 12`-like growth from `d_F ≥ 8`.
`[pcc:unverified]`

## 5. The chosen size (derived)

Pod limit 256·10⁹ bytes = 238.42 GiB (the cgroup limit of a 256 GB pod; `status.json` of the 112Gi pods:
`memory_bytes 256000000000`). Formula: the largest multiple of 10⁹ with `4·C ≤ 238.42 GiB − 36 GiB`, the 36 GiB
covering a basis up to 24 GiB, a full residue echelon (4.7 GiB at `nq = 50,256`), `c₀` and about 5 GiB of margin:
**`--max-dense 54000000000`** (1.59× step 35). Worst case under it: 201.2 + 24 + 4.7 + 1 = 230.9 GiB. It covers the low
and central bands and not the high one. Amendment 7 A's own formula at this pod (8 GiB headroom) would give 61·10⁹;
section 3 is why it is not used. `[pcc:unverified]`

## 6. Time (derived; see `HOSTS.md` section 3 for the budget)

- **Reaching step 35** took 21,960 to 21,975 s (Threadripper 7960X) and about 21,030 s (EPYC 9575F) at 12 threads
  (sum of the step times, lines 1-34; row `wall_s` minus the step-35 time). A larger `--max-dense` does not change steps
  1 to 34. `[pcc:unverified]`
- **Step 35's `B'` formation (A phase).** Wall per unit of `nonzeros/row × nq × pivots` ranged 4.7·10⁻¹² to 1.06·10⁻¹¹
  s on steps 28-34; step 35 has 1.045·10¹⁵ units: **4,900 to 11,100 s**. `[pcc:unverified]`
- **Step 35's S-rows (B phase).** Wall per unit of `S-rows × nonzeros/row × nq` was 2.66·10⁻¹¹ to 3.14·10⁻¹¹ s on
  steps 28-32 (steady); step 35 has 1.026·10¹⁵ units: **27,300 to 32,200 s if every S-row is reduced**. With
  `full_rank_exit` (row `full_rank_exit true`) the rest are skipped once the residues have a pivot on every column of
  `Q`, which a refutation in this step would bring about; then B is shorter. `[pcc:unverified]`
- **End of step 35:** about 21,960 + 620 + A + B + 100 s = **27,600 s (fast exit) to 66,000 s (every S-row)** after
  the F4 start, on the 2026-11-16 host speed. Two pods of the same model (EPYC 9575F) differed by 1.27× (17,681 s and
  22,471 s for the same cap-6 run) and 1.56× (7,908 s and 12,372 s for `t = 13` cap 6), so host variation is of the
  same order as the slack. `[pcc:unverified]`
- **Later degree-7 steps** each need at least the symbolic phase (about 600 s) and an A phase of 5,000 to 11,000 s at
  `B'` near step 35's, plus the B phase of their own S-rows (unknown count; at degree 6 the second all-pairs step had
  2.1× the first one's S-rows). The low band (refutation through falls after step 35) fits the 86,400 s budget; the
  central band needs a second all-pairs degree-7 step and lands at about 90,000 to 140,000 s at 12 threads: the budget
  would then stop it (censored, `timed_out` without `oversize`). `[pcc:unverified]`

## 7. Assumptions, listed once

1. The degree-6 history of `t = 10..13` predicts the degree-7 history of `t = 14` through one ratio. Only four points,
   one of them (`t = 11`) without a trace. `[pcc:unverified]`
2. The memory model's remainder `c₀` stays near 1 GiB at degree 7 (the symbolic data of step 35 added 152 MiB, line 34
   to line 35). `[pcc:unverified]`
3. The basis RSS stays below 24 GiB during degree 7 (it is 11.4 GiB at step 35). If it grows past about 30 GiB, an OOM
   kill can come before a clean `oversize` stop; both are censored. `[pcc:unverified]`
4. Wall-time units measured on steps 28-34 carry over to step 35 (rows twice as long; same kernel). `[pcc:unverified]`
5. The 256 GB pod's cgroup limit is 256·10⁹ bytes, as the 128 GB smoke pods showed 128·10⁹ (`runpod_capped/SMOKE.md`);
   to be read before start (`DRAFT_AMENDMENT.md`). `[pcc:unverified]`
