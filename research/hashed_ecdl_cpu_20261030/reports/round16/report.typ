#set page(paper: "a4", margin: 15mm)
#set text(size: 9.5pt)
= CPU bounded field-inverse cache
Shared-host observations only; no certified speedup or hypothesis promotion.
Job `hashed-ecdl-cpu-round16-20261030`, October 30, 2026 UTC.

*Integrated source:* candidate v/w `5d7dcb67616c`, pre-cache reference y
`4c03472f6bdd`, original baseline u/x `2bd4588fba60`. Intel Xeon 6975P-C,
one software thread, 2 vCPUs / 4 GiB reserved. No cross-panel pooling.

#table(columns: (1.3fr,1fr,1fr,1fr,0.7fr), inset: 4pt,
 [Fixture], [Baseline u ms], [Pre-cache y ms], [Candidate v ms], [y/v],
 [8-bit / 3 slots], [0.390351], [0.142364], [0.132843], [1.072],
 [16-bit / 3 slots], [5.732379], [0.802128], [0.793350], [1.011],
 [16-bit / 4 slots], [6.003614], [0.984624], [0.986796], [0.998])
Medians from nine measured repetitions after one warmup. Baseline u/v ratios:
2.938, 7.226, 6.084. Ratios are derived from measured medians.
The further 10x target remains unmet.

#image("../../round16/results/latency.svg", width: 90%)

The candidate builds field inverses for validated primes p <= 4096 and uses
them in affine additions, projective normalization and residual batch inversion.
Each Curve owns its table; construction remains charged to setup. Larger
fields retain extended Euclid. Added memory is 8*p bytes plus Vec overhead,
at most 32 KiB of entries; the retained sum-table metric excludes it.
Inversion counters count requests, including cache lookups, not Euclid calls.

All 150 executions completed (135 measured,15 warmups). Full certificate bytes,
scalar checks, replay and corrupted-scalar rejection pass. Setup, discarded
trials, elimination, both checks and serialization remain charged. Launch,
archive I/O and negative controls are excluded; external timers are unchanged.

Candidate v ranges (ms): 0.129832–0.137313,0.789557–0.817929,
0.980129–1.005459; reference y: 0.140528–0.147706,0.795359–0.810315,
0.980676–1.010528. The smallest fixture's ranges do not overlap; the two
larger ranges overlap. A/A x medians: 0.389038,5.727942,5.984068ms.
No certified timing improvement or equivalence follows from this L0 panel.

The34-test candidate passes strict Clippy and formatting. The export passes
completeness and file-integrity checks with zero findings. Returned build,
protocol and both source manifests match the frozen files. Full root/CUDA
build, strict isolation, independent review, external/rho comparisons and
larger-scale CPU recovery remain outstanding.
Protocol: `33e294d07d0ffc6f66fb696eac19d01a00c14add29e4532e48ea5e78c3d580c9`.
Raw rows, certificates and checked export: `round16/results/`.
