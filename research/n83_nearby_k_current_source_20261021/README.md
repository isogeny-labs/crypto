# N83 current-source nearby-base probe controls

The current N83 root11 producer reached full rank and independently recovered
the public-point scalar with **K1104, K1107, and K1120**. On that point,
K1120 found its four-point relation after 72,874 probes, K1104 after
161,733, and K1107 after 940,621. A separate 16-point public-screen check
verified every scalar and relation. K1104 had the lowest descriptive
geometric mean probe count, but neither of its paired sign tests met the
three-comparison threshold. The probe selector and the performance selector
therefore remain `BOUNDED_UNKNOWN`.

## Instance and execution

The curve is `EC1N83Ckb1a1haacb69ee9f37`: `y² + xy = x³ + x² + 1` over
`F₂[z]/(z⁸³ + z⁷ + z⁴ + z² + 1)`, with subgroup order
`8569786107849059`. The IC source SHA-256 is
`58e3ee492093de5f6b760b96e3aba819fa479cdecd22b1adee94f0ecb26026d8`;
the executed macOS ARM64 binary SHA-256 is
`07f7b5b424d0f1ecea1cd7e6564522c4d9c64557a820441c381e7474b13a388a`.
All three arms used the same root11/load75, sparse-canonical, symmetric-S3,
four-rank-thread and two-target-thread settings in their launch receipts.
Each process constructed its base and log system from empty process caches.

| Base | Current-source candidate ID |
| --- | --- |
| K1104 | `IC1N83Ckb1a1fb183264PDP4rootRCguidedLAgaussTDdirectISO0h2233626bba52` |
| K1107 | `IC1N83Ckb1a1fb183762PDP4rootRCguidedLAgaussTDdirectISO0hf1851768e85a` |
| K1120 | `IC1N83Ckb1a1fb185920PDP4rootRCguidedLAgaussTDdirectISO0hc21c54dc0a94` |

The one-target workload ID is `4dad9338d385`; the public-screen batch
workload ID is `e558cc41c783`. The stopped 192-point public-screen
extension has workload ID `9efc2cccff41`. K1107's earlier current-source one-target
control is repetition `R1`, so its new one-target row is `R2`. The other
rows are `R1`. The full `(candidate_id, workload_id, run_id)` keys and stage
costs are in [measurement_rows.json](identities/measurement_rows.json).
The [identity derivation](derive_identities.py) checks canonical hashes,
the enumerated-set digests, exact source, base, solver settings, and workload
inputs against the versioned artifacts.

The one-target control uses earlier public fixture 0 from
`n83_cold_factor_base_20261016`. The secondary screen uses the first sixteen
public `n83_highk_beyond4405_20261021` screen points; each point was checked
against its scalar sidecar outside the producer input. No sealed high-K
holdout point was used. The single-target children had 900-second caps;
the batch children had 1,200-second caps. All completed within their caps
on a shared macOS host. These are correctness and probe-count diagnostics;
the [Tier 1 contract](../../docs/IC_BENCHMARK_TIERS.md) remains the timing gate.

| Base | Usable points | Folded columns | Rank | One-target probes | Local online ms | Local cold process ms |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| K1104 | 183,264 | 1,104 | 1,104 | 161,733 | 418.554 | 721,464.812 |
| K1107 | 183,762 | 1,107 | 1,107 | 940,621 | 2,604.222 | 695,738.517 |
| K1120 | 185,920 | 1,120 | 1,120 | 72,874 | 64.134 | 723,986.066 |

The same K1107 source, base, relation, and probe count had previously taken
334.808 ms online and 128,364.716 ms cold on this host. Its new intervals
were 2,604.222 ms and 695,738.517 ms. This observed drift excludes the
local wall times from a speed ranking. The single target demonstrates that
first-relation probe count is not monotone in K.

| Base | Verified public-screen points | Geometric mean probes |
| --- | ---: | ---: |
| K1104 | 16 | 113,050.810 |
| K1107 | 16 | 134,762.705 |
| K1120 | 16 | 160,081.408 |

| Pair | First-arm wins | Second-arm wins | Ties | Exact two-sided sign p |
| --- | ---: | ---: | ---: | ---: |
| K1104 / K1107 | 11 | 5 | 0 | 0.210114 |
| K1104 / K1120 | 9 | 7 | 0 | 0.803619 |
| K1107 / K1120 | 7 | 9 | 0 | 0.803619 |

The three-pair threshold is `0.05/3`. The 16-point batch answers a secondary
question about deterministic relation-probe distribution after reusable
factor-log setup. Its process clocks include shared preparation and are not
substituted for individual one-target online intervals. The next performance
comparison must pair these bases on frozen public points, resource limits,
and same-point rho under an admitted Tier 1 envelope; high-K fixed31 and
K5500 remain part of the wider N83 factor-base search.

## Bounded 192-point extension

A larger paired public-screen probe check was frozen with K1107, K1120,
then K1104, at a 2,400-second cap per child. During the first child,
startup-volume free space fell to 619 MiB while swap use reached
20,303.44 MiB. The K1107 child was stopped with `SIGTERM` after
202.168 seconds, before a base dump or target row was emitted; K1120 and
K1104 were never launched. Its run row is
`STOPPED_HOST_STARTUP_DISK_GUARD`, with all incomplete stage costs null.
The [guard receipt](batch192_attempt/host_resource_guard.json) and the
[frozen input and process receipts](batch192_attempt) preserve this attempt.
The 16-point selector remains `BOUNDED_UNKNOWN` and the 192-point extension
contributes no ranking data.

## Independent replay and custody

`base_dump_k1104.jsonl.gz` and `base_dump_k1120.jsonl.gz` preserve the exact
ordered producer dumps. The canonical K1107 dump already lives in
`research/n83_highk_beyond4405_20261021/k1107_current_source_base/`.
Uncompressed SHA-256 values are
`aa2821e9c27222a94a0ba03bf1fcd7377ccabad3fdd4cff4046ccd1ca77f593f`
for K1104,
`08608f6ad2b8c66510246348933740c78d0937cae783d521514f467940edc3dc`
for K1107, and
`a1264aef4163f14f11738459fdd6620ca557c55c59e5f1e133fd3aa918b0ff34`
for K1120. The single-target and batch processes reproduced identical bytes
for each K. K1104 and K1107 are exact ordered prefixes of the already
independently replayed K1107 base. K1120 has that same prefix, and the
verifier checks the thirteen new representatives, all signed-Frobenius
labels, subgroup conditions, orbit closure, and distinctness against the
prefix. It independently checks all 51 target scalars and four-point
relations, source and binary custody, flags, phase sums, rank, and artifact
hashes.

Run the published verifier from the repository root:

```sh
python3 -m research.n83_nearby_k_current_source_20261021.verify_published
```

The frozen result is [published_replay.json](published_replay.json). Compact
launch, process, target, and replay receipts are under [single](single) and
[batch](batch). Full uncompressed source artifacts are retained on the
evidence volume at
`/Volumes/data/cryptanalysis/n83_nearby_k_current_source_20261021_run2`
and `/Volumes/data/cryptanalysis/n83_nearby_k_current_source_batch16_20261021`.
Their compressed ordered bases and all target rows needed for independent
replay are versioned here. The [artifact manifest](artifact_manifest.json)
hashes both the versioned and external raw files; [source snapshots](source_snapshots)
retain the executed batch runner, verifier, and selector.
