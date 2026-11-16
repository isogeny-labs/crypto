# Current-source K3596 identity and capped full-path controls

The fixed31 source revision yields the exact K3596 root4 candidate
`IC1N83Ckb1a1fb596936PDP4rootRCguidedLAgaussTDdirectISO0h2a3353097e81`.
Its manifest hashes to
`c596ca9fb6a8f89a53b6d00100ae6303f14c912b98a5d641d68808dd3dfc70f6`.
The ordered base is the independently replayed set of 596,936 subgroup points
and 3,596 folded columns from
[the current-source base control](../n83_highk_beyond4405_20261021/K3596_CURRENT_SOURCE.md).
The candidate record updates the executed source and every source-bound stage
digest from the historical manifest, preserving its exact base and root4
configuration.

Two full-path controls used the earlier public point at
`research/n83_cold_factor_base_20261016/targets.jsonl` with its scalar
sidecar excluded from the child input. Both used the binary with SHA-256
`07f7b5b424d0f1ecea1cd7e6564522c4d9c64557a820441c381e7474b13a388a`
and source SHA-256
`58e3ee492093de5f6b760b96e3aba819fa479cdecd22b1adee94f0ecb26026d8`.
The root4 flags came from the archived K3596 pilot, with only the base-dump
path changed. Each attempt had a 1,200-second wall cap, 16 GiB sampled RSS
cap and 10% minimum host-free-memory gate. These were local correctness
controls on a busy 48 GiB macOS host, not Tier 1 benchmark pairs.

| Attempt | Producer status | Wall seconds | Peak sampled RSS | Replay |
| --- | --- | ---: | ---: | --- |
| priority 10 | operator SIGTERM after observed CPU starvation | 792.479 | 796,409,856 bytes | `BOUNDED_UNKNOWN` |
| normal priority | `TIME_CAP` | 1,200.952 | 9,580,888,064 bytes | `BOUNDED_UNKNOWN` |

Neither attempt emitted a base dump or one-target result. Their zero-byte
stdout, stderr, and result files, exact launches, receipts, and independent
bounded replays are versioned under `runs/`. The historical scripts that
produced and replayed them are under `executed/`; their absolute paths and
script hashes are retained as executed. The portable verifier for these
committed receipts is:

```sh
python3 -m research.n83_k3596_current_source_full_20261025.verify_evidence
```

The next discriminating experiment is a current-source K3596 full index,
rank, and one-target solve on a sufficiently provisioned Tier 1 Pod. Pair it
on the same fresh public point and resource envelope with the admitted
nearby-base finalist and rho, and independently replay the base, relation,
scalar, phases, source, and Pod receipts. The K3596 branch can enter a
cross-family online or cold selection only when those measured rows pass
their respective gates. The [sealed 16-point confirmation](../n83_nearby_grid_confirm16_20261024/README.md)
remains the nearby-base comparison.
