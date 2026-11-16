# Native four-arm N83 selection replay

The native [selector](../../examples/n83_crossfamily_native_select.rs) places
K2816 first for supplementary cold-process cost among the frozen K2816,
K3072, K3450, and K3500 factor bases. Across 24 fresh public N83 points,
K2816 was faster in all 24 paired cold runs against each other arm. Each
two-sided exact sign-flip probability was `1.1920928955078125e-7`, below the
six-comparison threshold `0.05/6`. This selection is scoped to those four
candidate manifests and the archived cloud allocation.

| Factor base | Usable points | Folded columns | Mean cold process (s) | Geometric mean one-target online (ms) |
| --- | ---: | ---: | ---: | ---: |
| K2816 root11 | 467,456 | 2,816 | 334.291 | 5.937 |
| K3072 root11 | 509,952 | 3,072 | 393.983 | 6.370 |
| K3450 wide3 | 572,700 | 3,450 | 577.128 | 5.474 |
| K3500 root4 | 581,000 | 3,500 | 586.219 | 7.019 |

The primary online selector remains `BOUNDED_UNKNOWN`, with
`selected_candidate_id: null`: none of the six online contrasts crossed the
same frozen multiplicity gate. K3450 has the smallest descriptive geometric
mean here, while K3072 has the smallest online median. The result file keeps
every paired difference, uncertainty interval, exact probability, and
candidate ID.

The [result](native_selection_39075872898.json) is SHA-256
`d258e75e6b2dc6568b16dfe140c7ddc664ab63cdcb97222996700711a5028a77`.
The selector source is SHA-256
`f88d1f1207b37978285989810dc184119530968d1af907adc87252d940acced6`.
It binds the four raw archive ZIP hashes, the committed
[provider jobs record](provider_jobs_39075872898.json) (SHA-256
`fa4126b6ad2343f319af44429feea88c01e049abcbea568355eca536e2a7ce0b`),
four distinct overlapping provider runner IDs, exact source and stratum
identities, 24-point/four-arm coverage, and the separately passing
[native arithmetic replay](NATIVE_REPLAY.md) and the
[native raw resource replay](native_resource_replay_39075872898.json) (SHA-256
`3d3231161622e729db7f3d4330d1af644e647802c33424b217e9a7e35a8eed5a`).
Two independent exact sign-flip
implementations agree before a contrast is reported. Its online and cold
outputs match the earlier standalone native selector byte-for-byte after
removing the new provenance fields.

The [resource replayer](../../examples/n83_crossfamily_native_resource_replay.rs)
(source SHA-256 `7501c3a17812d6ac14f135d36505222597ee36728991322ced16978c739effe3`)
checks the 768 raw event receipts: 96 IC runs, 96 paired rho runs, and 576
calibration runs. It checks frozen launch caps and process RSS, cgroup CPU and
memory envelopes and per-event throttling/OOM deltas, steal-time deltas,
nonoverlapping intervals, and the 10% before/after median calibration gate.
It independently replays the calibration scalar against the frozen public
point. The largest observed calibration median spread was 9.2434%. The
selector still uses the archived stratum analyses for timing values and
execution order. The arithmetic replay independently checks all 24 public
points, 96 IC scalars and relations, 96 paired rho scalars, and all ordered
factor-base dumps. The new K2816 batched-index
measurement changes the code snapshot and only measures the index stage; its
stage time is not spliced into this older complete cold panel. The fresh
K1107/K2304 same-point bridge and a full batched-index candidate comparison
are the next selection gates.

Build with `cargo build --release --locked --example
n83_crossfamily_native_select`, then run the executable with an archive
directory containing `stratum0.zip` through `stratum3.zip`, this directory's
provider jobs JSON, and this directory's native arithmetic and resource
replay JSON files. The
four ZIPs are the preserved artifacts of
[run 39075872898](https://github.com/aburan28/crypto/actions/runs/39075872898).
