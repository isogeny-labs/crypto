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
| 3 | same, `run.sh main 0` (N = 21, target 0, 32 threads, budget 345,600 s, `--max-dense 20300000000`) | `research/pkm_lead1_m3n21_20261016/run.sh main 0` | 06:11:43 | — | running | `runs/M3-N21-t0.log` (step trace, live), `runs/M3-N21-t0.jsonl` (row, written at the end), `progress.txt` |
| 4 | `XP-20261015-shape-g1-frozen-cells`, sizing probe (13, 5), d_max = 7, random, 1 × 1, seed 1, 1 thread, 64 GB address cap, 4 h wall cap | `research/shape_search_g1_frozen_cells_20261018/probe/probe.sh 13:5:7 64 14400` | 06:11:00 | — | running (disclosed smoke run, not a result) | `probe/probe-13-5-7.{jsonl,err,meta}`, `probe/progress.txt` |

Planned, not started: PKM `main 1`, `confirm T D` for each finished target,
`verify`; the shape card's PROTOCOL.md (sized from run 4), then its three
cells `13:5:7`, `17:6:7`, `19:7:7` as the card's shards.

## Deviations and incidents

- 2026-10-18 06:11: PKM PROTOCOL.md §2 requires ≥ 192 GB for SHARE = 1; this
  host has 124 GB. Rules for A and C applied unchanged; recorded in the host
  file before stage 2.
- 2026-10-18 06:05–06:13: the first autocommit loop failed silently on a
  missing pathspec; replaced. First commit `67e221167` at 06:13.
- A swap file was not created (permission denied by the session's policy),
  so there is no cushion beyond the protocol's `ulimit -v`.
