# Run report: run-20261023T0121Z (rho.negation against rho.negation_lanes; external rho)

A descriptive record of the run. The results report is
[`../../speedups-20261023/README.md`](../../speedups-20261023/README.md).

## What ran

| item | value |
|:--|:--|
| spec | `../../speedups-20261023/rho-lanes/spec.json` (spec `ECS1h17511a88c065`): every 16th curve model of `run/specs/spec.json` (48: both sides, every level), 4 `planted_shared` targets each |
| arms | `rho-neg` = `rho.negation` (reference); `rho-neg-lanes` = `rho.negation_lanes`, `lanes = 32` (candidate) |
| measurement | warmup 1, rounds 3, order shuffle, seed 20261022, timeout 600 s |
| run | `ecbench run --cpus auto --allow-busy`; session `ECBS1h45f937d2f95a`, 01:21:26–01:29:02 UTC, 2026-10-23 |
| host | Intel Xeon Processor @ 2.10 GHz, 4 vCPU shared cloud VM, Linux; isolation earned L1 / L2 per run |
| ecbench binary | sha256 `d9a6ca939982404c…`; sessions' git `bec9df9e8`, dirty (the lanes walk, committed as `d81da47d5`) |
| external | `rho-lanes/external_ca.sh`: `ca solve --alg rho --threads 1` (`aburan28/cryptanalysis` 1aa48b5, `ca` 0.1.0), `taskset -c 2`, seeds 1–3, on the session's 192 workload targets; run after the session (binary sha256 in `rho-lanes/external_ca.binary.sha256`, recorded afterwards) |

## Runs

| arm | executions | measured (non-warmup) | verified | failures |
|:--|--:|--:|--:|--:|
| `rho-neg` | 768 | 576 | 768 | 0 |
| `rho-neg-lanes` | 768 | 576 | 768 | 0 |
| external `ca` | 576 | 576 | 576 recovered the planted scalar | 0 |

The audit is `ecbench verify --replay 24`: `audit OK: 1536 records, 1536 verified, 24 replays reproduced, 0 problems` (`rho-lanes/session.audit.json`).

## Recorded values (from `summary.json`)

| quantity | `rho-neg` | `rho-neg-lanes` | external `ca` |
|:--|--:|--:|--:|
| median solve wall time | 362.1 ms | 154.6 ms | 58.4 ms (its own timer) |
| median wall time per counted operation | 293.9 ns | 119.4 ns | 45.4 ns (its own count) |
| mean S (GAE / √r) | 0.9303 | 0.9538 | 0.976 (ops / √r) |

The run is paired by `ecbench compare`:
- counted operations, lanes over serial: 1.0252 [1.0213, 1.0295], cluster interval over 576 pairs;
- median wall ratio: 0.4193 [0.4173, 0.4214], over all 576 pairs (306 of them at L2 on both sides);
- measurement quality: 58/100.

The §5a band is in `rho-lanes/band.json`.

![Rho: serial, lanes, external](../../speedups-20261023/figures/rho-speed.svg)

## Raw records

- `../../speedups-20261023/rho-lanes/session/records.jsonl.gz` (pinned by `../../speedups-20261023/MANIFEST.sha256`), plus `session.json`, `host.json`, `plan.json.gz`
- `../../speedups-20261023/rho-lanes/external_ca.jsonl`, `external_ca.inputs.tsv`
- `../../speedups-20261023/rho-lanes/session/comparisons/rho-neg__rho-neg-lanes.json`, `band.json`
