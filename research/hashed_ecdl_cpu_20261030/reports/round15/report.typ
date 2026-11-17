#set page(paper: "a4", margin: 15mm)
#set text(size: 9.5pt)
= CPU combined reduction snapshot
Shared-host observations only; no certified speedup or hypothesis promotion.
Job `hashed-ecdl-cpu-round15-20261030`, October 30, 2026 UTC.

*Unapplied snapshots:* combined candidate `9a05eb488405` plus frozen patch;
reference y is profiling-only `fb3cb55f518a` plus its patch. Original baseline
u/x: `2bd4588fba60`; combined v and batch1 w. Intel Xeon 6975P-C,
one software thread, 2 vCPUs / 4 GiB reserved. Within-run comparisons only.

#table(columns: (1.3fr,1fr,1fr,1fr,0.7fr), inset: 4pt,
 [Fixture], [Baseline u ms], [Profile-only y ms], [Combined v ms], [y/v],
 [8-bit / 3 slots], [0.397709], [0.147832], [0.143688], [1.029],
 [16-bit / 3 slots], [5.796725], [0.838992], [0.814772], [1.030],
 [16-bit / 4 slots], [6.060122], [1.032969], [1.007229], [1.026])
Medians from nine measured repetitions after one warmup. Baseline u/v ratios:
2.768, 7.115, 6.017. The further 10x target remains unmet.

#image("../../round15/results/latency.svg", width: 90%)

The combined snapshot disables unused internal clocks and reuses the collector's
generated target when checking tuples. Full point/membership/affine-sum checks
remain. The original relation checker and verifier are byte-identical; both
complete certificate replays still independently recompute every target.

All 150 executions completed (135 measured,15 warmups). Full certificate bytes,
scalar checks, replay and corrupted-scalar rejection pass. Setup, discarded
trials, elimination, both checks and serialization remain charged. Launch,
archive I/O and negative controls are excluded; external timers are unchanged.

All v/y ranges overlap. v ranges (ms): 0.140297–0.154517,0.801787–0.836947,
0.986422–1.020244; y: 0.143217–0.161480,0.824788–0.858150,1.014289–1.062496.
No certified improvement. A/A x medians: 0.389588,5.801267,6.070398ms.
Ratios are derived from measured medians. No cross-panel pooling.

The33-test snapshot passes strict Clippy and formatting. Export completeness and
file-integrity checks pass. Both returned materialization manifests match their
frozen copies. Effective source/patch hashes are recorded alongside the protocol.
Integration, strict isolation, independent review, external/rho comparisons and
larger-scale CPU recovery remain unresolved. Build layout/co-tenancy are nuisances.
Protocol: `6518e8d6d988c3800c9791e16566a4ad8c7fd64058ddab5b2ff8f6103bc8791e`.
Raw rows, certificates and checked export: `round15/results/`.
