# Current-source nearby-K same-point Tier 1 experiment

This experiment measures the complete one-target online interval for the
current-source K1104, K1107 and K1120 factor bases on the same sealed N83
public point. It also records each cold process interval and a same-point rho
reference. The input is a separate deterministic point, workload
`e88fa8e34255`, and its scalar sidecar stays outside every child input.
`generate_fixture.py` checked that point against 35,154 distinct earlier
public points, including the fixed31 screen and all 16 of its reserved
holdout points. `exclusion_audit.json` records zero overlap.

The frozen source SHA-256 is
`58e3ee492093de5f6b760b96e3aba819fa479cdecd22b1adee94f0ecb26026d8`.
`protocol.py` binds all three canonical candidate manifests, the independent fixture,
rho source and config, process caps, and exact event order before timing.
Five rounds alternate K1104 and K1120 around two K1107 A/A controls. A
same-point rho solve opens and closes each round. This is one target with
repeated empty-process runs; it does not estimate success over other targets.

The GitHub Actions job uses one `ubuntu-24.04` allocation. The host gate
requires at least two visible CPUs, 5 GiB cgroup memory, 4 GiB free disk and
readable cgroup telemetry. Each IC child has a 900-second cap, each rho child
a 120-second cap, the producer has a 120-minute cap, and a child is stopped
above 4 GiB RSS. Any missing, failed, timed-out or noisy child remains in the
raw artifact and leaves the Tier 1 comparison `BOUNDED_UNKNOWN`.

`verify.py` checks all 30 child launches, the exact ordered factor-base dump,
rank, each scalar, every four-point relation, exclusive target phase sums,
same-point rho receipts, source and binary hashes, RSS, OOM, throttling,
steal-time telemetry and the paired K1107/rho A/A spread. A Tier 1 comparison
passes only when every child replays and each within-round A/A spread is at
most 20%. Its median online and cold times describe this frozen point; the
larger high-K screen, K5500 full-process gate and cross-family selection
remain separate required evidence for the absolute factor-base objective.

| Requirement | Evidence after the hosted job |
| --- | --- |
| Exact N83 curve, source, three bases and independent point | `source.json`, manifests, `target.jsonl`, `exclusion_audit.json`, verifier |
| Full one-target online and cold IC intervals | Each `runs/*/stdout.jsonl`, `targets_result.jsonl`, `usage.json` |
| Same-point rho and A/A controls | Ten rho `receipt.json` files, `independent_replay.json` |
| Tier 1 host and noise envelope | `host.json`, each `host_telemetry.json`, cgroup snapshots |
| Absolute e2e factor-base selection | Pending high-K, K5500 and cross-family gates |

Run the frozen fixture check with:

```sh
python3 -m research.n83_nearby_k_github_tier1_20261021.run --preflight
```

The first prepared version reused fixed31 holdout point 0. It was cancelled
while queued, before any child started, because using that point to select a
nearby-K base would contaminate the later cross-family holdout. This version
uses the separate workload above; the fixed31 holdout remains reserved.

The workflow builds and tests the exact producers on Linux, runs the sealed
schedule, independently replays it, and uploads all raw receipts even if a
child or gate fails. The producer and verifier run in a GitHub Actions job;
the present macOS host is not used for a new timed comparison.


## Rho A/A control correction

Run [39695853504](https://github.com/aburan28/crypto/actions/runs/39695853504) remains `BOUNDED_UNKNOWN`. Its original source receipt and independent replay are retained in `prior_runs/39695853504/`, with the artifact digest and exact source commit. All 30 children replayed, but the 20% noise gate failed. The rho before/after pair used different seeds in each round, so its spread mixed stochastic search work with host noise. The K1107 pair also exceeded the gate in round 0 (21.316%); correcting rho does not validate that run.

The corrected schedule repeats the same rho seed within each round and keeps distinct seeds across the five rounds. Its protocol digest changes, and the verifier continues to require the exact schedule and protocol digest. The candidates, point, caps, five rounds and 20% gate stay fixed. Any new run is separate evidence; it must independently pass every existing gate. No historical timing is promoted, and the earlier failed run is never reclassified.
