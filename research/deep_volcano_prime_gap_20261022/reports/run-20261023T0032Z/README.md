# Run report: run-20261023T0032Z (sharded re-run of the study's sessions)

A descriptive record of the run. The results report is
[`../../speedups-20261023/README.md`](../../speedups-20261023/README.md).

## What ran

| item | value |
|:--|:--|
| build | `run_pipeline.sh build speedups-20261023/rerun-sharded 52 6 16384 20261022 2 6 4 2 3` (`rerun-sharded-20261023` when it ran; the directory was moved afterwards) |
| run | `run_pipeline.sh run`: five `ecbench run --cpus none --allow-busy` sessions, concurrently |
| start / end (UTC, `session.json`) | all started 00:32:17; ended 00:39:06 (BSGS), 00:44:12, 00:44:21, 00:44:24 (shards 0–2), 00:44:31 (A/A), 2026-10-23 |
| host | Intel Xeon Processor @ 2.10 GHz, 4 vCPU shared cloud VM, Linux; isolation L0 |
| ecbench binary | sha256 `4652d20005e16255…` (in each `session.json`); sessions' git `bec9df9e8`, dirty (the registry cache and `--shards`, committed as `0bf3db1a3`) |
| seeds | measurement seed 20261022; A/A ⊕ `0xAA0000000`; target law `planted_shared`, target seed 20261022 |

## Sessions

| session | spec | runs recorded | status | audit (`--replay 8`) |
|:--|:--|--:|:--|:--|
| `ECBS1h14d98bf2a3ec` | `spec-shard0` (`rho.negation`) | 1524 | 1524 verified | OK, 8 replays exact |
| `ECBS1h846ee50776a9` | `spec-shard1` | 1518 | 1518 verified | OK |
| `ECBS1hd4816b5f840b` | `spec-shard2` | 1518 | 1518 verified | OK |
| `ECBS1h0246a995e87e` | `spec-aa` | 1520 | 1520 verified | OK |
| `ECBS1hd719e0366969` | `spec-bsgs` | 760 | 760 verified | OK |

No run failed or timed out.

## Recorded values

| quantity | value |
|:--|:--|
| records, original run / this run | 6840 / 6840 |
| per-record identity | identical by (arm, workload, target, round, warmup, total GAE, recovered value, counters): `compare.sh`, both sides hash to `7bedc5cc055f…` (`compare.sha256`) |
| span, first start to last end | 734 s; the original run's three sessions spanned 2120 s (2026-10-22) |

![Pipeline stages](../../speedups-20261023/figures/pipeline-stages.svg)

## Raw records

- `../../speedups-20261023/rerun-sharded/sessions/*/records.jsonl.gz`, pinned
  uncompressed by `../../speedups-20261023/MANIFEST.sha256`
- `session.json`, `host.json`, `spec.json`, `plan.json.gz` in each session directory
- `../../speedups-20261023/rerun-sharded/specs/`, `pair.json`, build logs
