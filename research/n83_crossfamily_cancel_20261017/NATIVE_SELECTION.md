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
`883b6a5469a963298d802271bda96ab4abea9b5a934e4e168fc7c10ff190022b`.
The selector source is SHA-256
`3034b1b8a20cf71608710f8169dffddb22426e954382a6ddc48267e18b29f861`.
It binds the four raw archive ZIP hashes, the committed
[provider jobs record](provider_jobs_39075872898.json) (SHA-256
`fa4126b6ad2343f319af44429feea88c01e049abcbea568355eca536e2a7ce0b`),
four distinct overlapping provider runner IDs, exact source and stratum
identities, 24-point/four-arm coverage, and the separately passing
[native arithmetic replay](NATIVE_REPLAY.md). Two independent exact sign-flip
implementations agree before a contrast is reported. Its online and cold
outputs match the earlier standalone native selector byte-for-byte after
removing the new provenance fields.

The resource, calibration, execution-order, and host admission checks in this
selection are taken from the archived stratum analyses; they have not been
replayed natively from every raw event. The arithmetic replay independently
checks all 24 public points, 96 IC scalars and relations, 96 paired rho
scalars, and all ordered factor-base dumps. The new K2816 batched-index
measurement changes the code snapshot and only measures the index stage; its
stage time is not spliced into this older complete cold panel. The fresh
K1107/K2304 same-point bridge and a full batched-index candidate comparison
are the next selection gates.

Build with `cargo build --release --locked --example
n83_crossfamily_native_select`, then run the executable with an archive
directory containing `stratum0.zip` through `stratum3.zip`, this directory's
provider jobs JSON, and this directory's native arithmetic replay JSON. The
four ZIPs are the preserved artifacts of
[run 39075872898](https://github.com/aburan28/crypto/actions/runs/39075872898).
