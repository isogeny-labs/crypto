# Current-source nearby-K same-point Tier 1 experiment

This experiment measures the complete one-target online interval for the
current-source K1104, K1107 and K1120 factor bases on the same sealed N83
public point. It also records each cold process interval and a same-point rho
reference. The input is holdout point 0 from the fixed31 fixture, workload
`f3f41cbb7544`, and its scalar sidecar stays outside every child input.

The frozen source SHA-256 is
`58e3ee492093de5f6b760b96e3aba819fa479cdecd22b1adee94f0ecb26026d8`.
`protocol.py` binds all three canonical candidate manifests, the fixture,
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
| Exact N83 curve, source, three bases and unseen point | `source.json`, manifests, `target.jsonl`, verifier |
| Full one-target online and cold IC intervals | Each `runs/*/stdout.jsonl`, `targets_result.jsonl`, `usage.json` |
| Same-point rho and A/A controls | Ten rho `receipt.json` files, `independent_replay.json` |
| Tier 1 host and noise envelope | `host.json`, each `host_telemetry.json`, cgroup snapshots |
| Absolute e2e factor-base selection | Pending high-K, K5500 and cross-family gates |

Run the frozen fixture check with:

```sh
python3 -m research.n83_nearby_k_github_tier1_20261021.run --preflight
```

The workflow builds and tests the exact producers on Linux, runs the sealed
schedule, independently replays it, and uploads all raw receipts even if a
child or gate fails. The producer and verifier run in a GitHub Actions job;
the present macOS host is not used for a new timed comparison.
