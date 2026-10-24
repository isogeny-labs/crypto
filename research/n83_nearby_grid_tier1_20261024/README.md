# N83 K1102–K1120 same-point Tier 1 screen

This experiment screens all 19 exact current-source signed-Frobenius orbit
factor bases in the nearby K1102–K1120 grid. It runs each base as a fresh
one-target cold process on one newly sealed public point. Each three-base
block is bracketed by K1107 and same-point rho controls; rho repeats its seed
within the block. The second sweep reverses the challenger order. This
schedule measures the complete cold process and the separately defined
one-target online interval, while recording host noise and every failed
child. A passing screen nominates bases for a fresh multi-point confirmation;
it does not set `selected_best_total_runtime`.

The point has workload ID `052f3657ce24` and domain
`n83-nearby-grid-tier1-screen-20261024-v1`. Its scalar sidecar is outside
every IC and rho child input. `exclusion_audit.json` records zero overlap
with 35,155 earlier distinct public points, including the fixed31 holdout
and the preceding nearby-K Tier 1 point. The exact target, answer, generator,
exclusion audit, workload, source, rho configuration, and 19 candidate
manifests are pinned in `protocol.py`.

The frozen schedule has 84 children: 60 IC and 24 rho across 12 blocks.
Each IC child has a 900-second cap; each rho child has 120 seconds. The
whole target schedule has a 240-minute cap and the GitHub job has a
300-minute cap including compilation, independent replay, and artifact
upload. A 4 GiB child RSS cap, cgroup CPU/memory counters, per-event
host telemetry, affinity, same-seed rho controls, and K1107 online and cold A/A controls
are required for a Tier 1 result. Any incomplete, noisy, capped, or
unverified child leaves the screen `BOUNDED_UNKNOWN` and remains in the
uploaded raw artifact.

From the repository root, preflight and gate checks are:

```sh
python3 -m research.n83_nearby_grid_tier1_20261024.generate_fixture
python3 -m research.n83_nearby_grid_tier1_20261024.run --preflight
python3 -m unittest research.n83_nearby_grid_tier1_20261024.test_gates
```

The workflow `.github/workflows/n83-nearby-grid-tier1.yml` builds the
frozen native IC and rho producers, runs the schedule, independently replays
factor bases, rank, four-point relations, scalars, phase accounting, source
and binary hashes, and preserves raw receipts. The result is pending.
