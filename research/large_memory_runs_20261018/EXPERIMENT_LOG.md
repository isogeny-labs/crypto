# Large-memory index-calculus runs, 2026-10-18: experiment log

Every run launched on this host is listed here with its command, start time,
output paths and status. All artifacts live on the host's root volume, an
Amazon EBS volume (`vol0f3f8dc899a8f30c2`, 2 TB, mounted at `/`), inside the
checkout `/home/ec2-user/crypto`, and are committed to branch
`claude/large-memory-runs-20261018` every 10 minutes by
`/home/ec2-user/ops/autocommit.sh` and pushed to PR #2160. Times are UTC.

## Host

| item | value |
|:--|:--|
| instance | EC2 spot, `ip-172-31-35-131.us-east-2.compute.internal`, 32 vCPU, MemTotal 129,800,716 KiB (124 GB), no swap |
| disk | EBS root, 2 TB, `/` |
| toolchain | rustc 1.99.1; binaries built in release mode at commit `b7cafe044` (main) |
| binaries | `target/release/examples/pkm_tower_pilot` (sha256 `d25eadb1…c265d`), `pkm_tower_check`, `factor_base_shape_ladder`, `target/release/backlog` |

## Runs

| # | card / stage | command (from repo root) | started | ended | status | outputs |
|:--|:--|:--|:--|:--|:--|:--|
| 1 | `XP-20261015-pkm-m3-n21-m4-n20`, `run.sh host` | `research/pkm_lead1_m3n21_20261016/run.sh host` | 05:54:06 | 05:54:06 | done | `research/pkm_lead1_m3n21_20261016/runs/host-ip-172-31-35-131.us-east-2.compute.internal.txt` (A = 119,416,658 KiB, C = 2.03e10, SHARE = 1; host-size deviation recorded) |
| 2 | same, `run.sh identity` (N = 15, target 0) | `research/pkm_lead1_m3n21_20261016/run.sh identity` | 05:54:06 | 05:54:58 | done, exit 0, **identity passes** (all 7 compared fields equal round 6; `pkm_tower_check verify`: 1 row, 1 agrees) | `runs/identity-N15-t0-ip-172-31-35-131….{jsonl,log}` |
| 3 | same, `run.sh main 0` (N = 21, target 0, 32 threads, budget 345,600 s, `--max-dense 20300000000`) | `research/pkm_lead1_m3n21_20261016/run.sh main 0` | 06:11:43 | 07:59:09 | done, exit 0; **no verdict**: stopped in step 27 by the protocol's `--max-dense C` cap (B' would need 7.93e10 entries ≈ 317 GB against C = 2.03e10; the trace prints the generic `oversize: --max-nnz`) on entering degree 8 after every degree-7 pair was reduced without refuting (`last_productive_degree` 7, `degree_reached` 8, `inconsistent` false); peak resident 68.7 GB before the stop; 27 steps, 667,444 cols, 1.56e14 muladds, 6,445 s wall | `runs/M3-N21-t0.log`, `runs/M3-N21-t0.jsonl`, `progress.txt` |
| 5 | same, `run.sh main 1` (N = 21, target 1, same flags) | `research/pkm_lead1_m3n21_20261016/run.sh main 1` | 08:42:41 | 10:29:39 | done, exit 0; **identical outcome to target 0** (same 27 steps, same 667,444 cols, same B' demand 7.93e10 entries at the degree-8 step, `last_productive_degree` 7, `degree_reached` 8, not refuted); 6,416 s wall | `runs/M3-N21-t1.{log,jsonl}` |
| 8 | same, `run.sh verify` | `research/pkm_lead1_m3n21_20261016/run.sh verify` | 10:31 | 10:31 | done: 1 row (identity) checked by exhaustive search, agrees; the two N = 21 rows carry no verdict to check | `verify.txt` |
| — | same, `run.sh confirm` | — | — | — | not applicable: PROTOCOL.md §3.3 confirms only a target that finished with a D | — |
| 4 | `XP-20261015-shape-g1-frozen-cells`, sizing probe (13, 5), d_max = 7, random, 1 × 1, seed 1, 1 thread, 64 GB address cap, 4 h wall cap | `research/shape_search_g1_frozen_cells_20261018/probe/probe.sh 13:5:7 64 14400` | 06:11:00 | 10:11:05 | **killed at the 4 h wall cap** (exit 137), no draw finished, sampled peak RSS 7.7 GB: time, not memory, limits (13, 5) at d_max = 7 on one thread | `probe/probe-13-5-7.{jsonl,err,meta}` (empty jsonl), `probe/progress.txt` |
| 6 | same card, probe (13, 5) d_max = 7, 8 threads, 64 GB cap, 12 h wall | `probe/probe.sh 13:5:7 64 43200 8` | 10:12 | — | running (does threading help?) | `probe/probe-13-5-7-th8.*` |
| 7 | same card, probe (13, 5) d_max = 6, 1 thread, 64 GB cap, 4 h wall | `probe/probe.sh 13:5:6 64 14400 1` | 10:12 | 11:47 | done, exit 0: one random draw 5,732 s, peak 2.1 GB, outcome `at_least 7` | `probe/probe-13-5-6-th1.*` |
| 9 | same card, probes (17, 6) and (19, 7) at d_max = 6, 1 thread, 24 GB cap, 4 h wall | `probe/probe.sh 17:6:6 24 14400 1`, `probe/probe.sh 19:7:6 24 14400 1` | 10:33 | 14:33 | **both killed at the 4 h wall cap** (exit 137), no draw finished; sampled peak RSS 5.5 GB (17, 6) and 12.3 GB (19, 7). Under PROTOCOL.md §3, stage C ((17, 6) and (19, 7)) is **not sized on this host and is not run** (prediction 3 holds) | `probe/probe-17-6-6-th1.*`, `probe/probe-19-7-6-th1.*` |
| 10 | same card, probes (17, 6) and (19, 7) at d_max = 7, 1 thread, 24 GB cap, 12 h wall | `probe/probe.sh 17:6:7 24 43200 1`, `probe/probe.sh 19:7:7 24 43200 1` | 10:33 | — | running | `probe/probe-17-6-7-th1.*`, `probe/probe-19-7-7-th1.*` |

| 11 | `XP-20261015-shape-g1-frozen-cells`, **stage A** (PROTOCOL.md §3, frozen in `ad6c224f6`): (13, 5) d_max = 6, shapes random, random2, gp, fp, coord, 2 × 2, seed 20261015, one process per shape, 16 GB cap, 24 h wall | `research/shape_search_g1_frozen_cells_20261018/run.sh stage A` | 11:50:03 | ~18:50 | **lost to a host reboot** after ~7 h, no end lines; 11 draws committed by 17:18Z, all `at_least 7` (random 2, random2 2, gp 3, fp 2, coord 2); kept in `runs/attempt1-lost-host-reboot-20261018T1850Z/` |
| 12 | same, stage A **repeat** (same flags and binary, deterministic seeds) | `run.sh stage A` | 19:03:30 | — | running (5 processes); must reproduce attempt 1's draws line for line | `runs/g1-13-5-d6-<shape>.{jsonl,err,meta}` | `runs/g1-13-5-d6-<shape>.{jsonl,err,meta}`, `progress.txt` |

| 13 | `XP-20261015-kummer-p224-depth`, stage 0 (PROTOCOL.md frozen in `679acac3a`): identity `m = 4, N = 12` target 0 at `p_40` and P-224, 2 threads | `research/pkm_kummer_wide_field_depth_20261018/run.sh stage0` | 20:47:56 | 20:52:56 | done: **identity holds at both primes** (every field equals the study's row; checkers agree); trace peaks 315 MB and 1,138 MB give memory multiples 1.84 and 6.65 | `research/pkm_kummer_wide_field_depth_20261018/STAGE0.md`, `runs/S0-*` |
| 14 | same card, stage 1: `m = 2, N = 20`, `m = 3, N = 15` (targets 0, 1) and `m = 4, N = 12` (target 1) at `p_40` and P-224; 10 processes, 2 threads each, 32 GB cap, 48 h budget | `run.sh stage1` | 20:55 | — | 45-bit cells done 21:15: **H_same holds in all six rows** (`m = 2, N = 20`: 6/33446/24; `m = 3, N = 15`: 7/31397/24; `m = 4, N = 12`: 7/9452/29, all refute); both target-0 rows identical to the study's (multiply-adds included); `pkm_tower_check verify` 6/6 agree; P-224 `m = 4, N = 12` target 1 also 7/9452/29 with `--verify` ok. P-224 `m = 2, N = 20` both targets done 22:26: 6/33446/24, refute, `--verify` ok, target 0 identical to the study's row (5,549 s on 2 threads, trace peak 3.5 GB); P-224 `m = 3, N = 15` running | `runs/S1-*`, `progress.txt` |
| 15 | same card, stage 2 `m = 4, N = 16` at `p_40`, targets 0 and 1, 8 threads, 19 GB cap (admitted at MemAvailable 70.9 GB) | `run.sh stage2 m4N16 v40 8 19` | 21:15:24 | — | running; `m = 3, N = 18` at `p_40` deferred until it ends (host load) | `runs/S2-m4N16-v40-*` |

Planned, not started: P-224 card stage 2 (`m = 4, N = 16` and `m = 3, N = 18` at both primes, under the protocol's memory admission rule); shape stage B ((13, 5) d_max = 7, at most 2 processes at a time) after stage A; stage C is not run (its d_max = 6 probes did not finish within 4 h, PROTOCOL.md §3).

## Reports

- PKM `m = 3`, `N = 21`: `research/pkm_lead1_m3n21_20261016/RESULTS.md` and the run report `research/pkm_lead1_m3n21_20261016/reports/run-20261018T0611Z/report.pdf` (graph `steps.svg`, source `plot.py`).

## Deviations and incidents

- 2026-10-18 06:11: PKM PROTOCOL.md §2 requires ≥ 192 GB for SHARE = 1; this
  host has 124 GB. Rules for A and C applied unchanged; recorded in the host
  file before stage 2.
- 2026-10-18 06:05–06:13: the first autocommit loop failed silently on a
  missing pathspec; replaced. First commit `67e221167` at 06:13.
- 2026-10-18 07:59: target 0 stopped at the protocol's `--max-dense C` cap (engine source: `f4_fp_tower.rs` stops as oversize when `dense_entries > max_dense`; the trace's `oversize: --max-nnz` wording is generic). The degree-8 step needs a B' of 7.93e10 entries (≈ 317 GB, ≈ 460 GB at round 7's 1.45 peak ratio): the card's 133–146 GB projection assumed D stays at 7 and is too low, as PROTOCOL.md §4.4 warned. Under §5 the reading is for the coordinator: the degree-7 closure was exhausted without refuting, and the degree-8 step did no productive reduction before the cap (`muladds` 0). Target 1 runs with identical flags, as §5 requires.
- 2026-10-18 10:30: the 8-thread (13, 5) probe runs at one core (97% CPU): `factor_base_shape_ladder g1` does not parallelise a draw, so threads do not shorten a cell.
- 2026-10-18 ~17:19: a container restart killed the commit loop (last commit 17:18Z); it was not noticed until the reboot, so nothing was committed from 17:18Z to 19:04Z.
- 2026-10-18 ~18:50: the host rebooted. Lost without end lines: stage A attempt 1 (above) and the three degree-7 sizing probes `probe-13-5-7-th8`, `probe-17-6-7-th1`, `probe-19-7-7-th1` (sizing only; none had finished a draw in 6.5–8.5 h). The probes are not relaunched: stage C is already not sized under PROTOCOL.md §3, and stage B measures (13, 5) at degree 7 directly. Stage A was relaunched at 19:03:30Z; the relaunch truncated the on-disk files, which were restored from commit `20d85464f` into the attempt-1 directory.
- A swap file was not created (permission denied by the session's policy),
  so there is no cushion beyond the protocol's `ulimit -v`.
