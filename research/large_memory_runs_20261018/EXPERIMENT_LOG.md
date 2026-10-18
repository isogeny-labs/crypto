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
| 7 | same card, probe (13, 5) d_max = 6, 1 thread, 64 GB cap, 4 h wall | `probe/probe.sh 13:5:6 64 14400 1` | 10:12 | — | running (fallback sizing) | `probe/probe-13-5-6-th1.*` |
| 9 | same card, probes (17, 6) and (19, 7) at d_max = 6, 1 thread, 24 GB cap, 4 h wall | `probe/probe.sh 17:6:6 24 14400 1`, `probe/probe.sh 19:7:6 24 14400 1` | 10:33 | — | running | `probe/probe-17-6-6-th1.*`, `probe/probe-19-7-6-th1.*` |
| 10 | same card, probes (17, 6) and (19, 7) at d_max = 7, 1 thread, 24 GB cap, 12 h wall | `probe/probe.sh 17:6:7 24 43200 1`, `probe/probe.sh 19:7:7 24 43200 1` | 10:33 | — | running | `probe/probe-17-6-7-th1.*`, `probe/probe-19-7-7-th1.*` |

Planned, not started: PKM `main 1`, `confirm T D` for each finished target,
`verify`; the shape card's PROTOCOL.md (sized from run 4), then its three
cells `13:5:7`, `17:6:7`, `19:7:7` as the card's shards.

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
- A swap file was not created (permission denied by the session's policy),
  so there is no cushion beyond the protocol's `ulimit -v`.
